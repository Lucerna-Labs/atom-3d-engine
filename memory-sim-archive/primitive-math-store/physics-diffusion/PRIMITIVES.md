# Physics / Diffusion — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## Diffusion Equation Atoms

### diffuse (cross-domain alias: `spread`, `heat-equation`, `Laplacian`)
**Domain:** Physics / Diffusion
**Definition:** Fick's second law: ∂C/∂t = D·∇²C. The concentration C spreads from high to low over time, governed by the diffusion coefficient D. The Laplacian ∇² captures the curvature driving the flow.
**Atom or composite:** Composite: compute Laplacian over a neighborhood → multiply by diffusion coefficient → update concentration. Discrete form: C_new = C + D·Δt·∇²C.
**Cost model:** O(N) per time step where N is the number of sites. Alternating direction implicit (ADI) methods solve tridiagonal systems in O(N) per step for arbitrary boundary conditions.
**Real wall?** No. But the diffusion coefficient D is conserved — a higher D means faster spread but less spatial resolution. There's a CFL (Courant-Friedrichs-Lewy) stability condition: D·Δt/Δx² ≤ 0.5 for explicit methods.
**Cross-domain wiring:** Label propagation on a graph = discrete heat equation on a graph. Random walk on a graph = diffusion. Spreading activation in neural networks = same discrete diffusion. In retrieval: relevance diffusion from seed nodes = heat spreading from query anchors.
**Notes:** The heat kernel on a graph = the continuous-time random walk. Its spectral decomposition gives the graph's diffusion modes — same as the Fourier modes for signals.

### diffuse-decay (cross-domain alias: `decay`, `exponential-dampen`, `distance-discount`)
**Domain:** Physics / Diffusion
**Definition:** Diffusion with exponential decay: ∂C/∂t = D·∇²C − λ·C. The decay term λ removes long-distance signals, preventing infinite spread.
**Atom or composite:** Composite: diffuse(step) + decay(fold(multiply by exp(−λ·Δt))).
**Cost model:** O(N) per step — same as diffuse plus one multiply per site.
**Real wall?** No.
**Cross-domain wiring:** Exponential decay = discounting future rewards in RL. In retrieval: relevance decay with distance = exponential decay on graph walks. In signal: RC circuit discharge = exponential decay.
**Notes:** The decay rate λ controls how far information travels before it vanishes. This is the percolation threshold — above a critical decay, the diffusion front dies out; below it, it reaches the whole graph.

### laplacian (cross-domain alias: `curvature`, `smooth`, `second-derivative`)
**Domain:** Physics / Diffusion
**Definition:** The discrete Laplacian at a node: ∇²C = Σ_{neighbors}(C_neighbor − C) / degree(node). Captures the average deviation from the local mean — positive at peaks, negative at troughs.
**Atom or composite:** Atom (for the discrete graph form). Composite for continuous: second spatial derivatives in each dimension.
**Cost model:** O(N·degree) for a full graph step. For grid graphs: one addition + one division per edge per step.
**Real wall?** No.
**Cross-domain wiring:** Graph Laplacian is the discrete analogue of the continuous Laplacian. Its eigenvectors are the graph's Fourier basis. In signal: discrete Laplacian filter = [1, −2, 1] in 1D. In linear algebra: L = D − A (degree matrix minus adjacency matrix).
**Notes:** The graph Laplacian's smallest eigenvector is the all-ones vector (for connected graphs) — the corresponding eigenvalue is zero. This is the DC mode, the uniform solution of the heat equation.

---

## Spreading Activation Atoms

### spread-activate (cross-domain alias: `activate`, `fan-out`, `signal-distribute`)
**Domain:** Physics / Diffusion
**Definition:** From an activated seed node, distribute activation to neighbors weighted by edge strength. Each node accumulates activation from all neighbors, optionally subtracts its own threshold.
**Atom or composite:** Composite: for each edge (u,v): activation_v += activation_u × weight(u,v).
**Cost model:** O(E) per propagation step — one multiply-add per edge.
**Real wall?** No.
**Cross-domain wiring:** Message passing on factor graphs = spreading activation with learned message functions. Belief propagation = spreading activation with max-sum or sum-product message passing. In graphics: glow/bloom = spreading activation in the pixel graph.
**Notes:** The key choice is the threshold function: hard threshold (step), sigmoid (soft threshold), or linear (no threshold). This is the same choice as in neural network activations.

### percolate (cross-domain alias: `phase-transition`, `giant-component`, `threshold-reach`)
**Domain:** Physics / Diffusion
**Definition:** The formation of a giant connected component in a random graph as edge probability p exceeds the percolation threshold p_c ≈ 1/N. Above p_c, a macroscopic fraction of the graph becomes connected.
**Atom or composite:** Composite: generate random graph → find connected components → check if largest component contains O(N) nodes.
**Cost model:** O(N + E) for component detection via BFS/union-find. Percolation simulation is Monte Carlo.
**Real wall?** Yes — the percolation threshold is a real phase transition. Below it, no giant component exists. Above it, the giant component grows smoothly with p. The transition is sharp at large N.
**Cross-domain wiring:** Information cascade = percolation in social networks. Search in a graph = percolation until the target is reached. In retrieval: relevance front reaching a document = percolation event.
**Notes:** The critical threshold for a d-regular random graph is p_c = 1/(d−1). For Erdős–Rényi with average degree ⟨k⟩, p_c = 1/⟨k⟩. This gives you the threshold at which your relevance front goes from sparse to dense.

### diffuse-with-restart (cross-domain alias: `random-walk-with-restart`, `teleport`, `personalized-pagerank`)
**Domain:** Physics / Diffusion
**Definition:** A random walk that at each step has probability α of teleporting back to a seed node (or uniformly at random). The stationary distribution is the Personalized PageRank.
**Atom or composite:** Composite: from current node, with probability (1−α): move to random neighbor; with probability α: teleport to seed. Iterate until convergence.
**Cost model:** O(1) per step. Converges in ~50 iterations for typical graphs.
**Real wall?** No.
**Cross-domain wiring:** Personalized PageRank = random walk with restart. In information retrieval: relevance propagation from seed documents = PPR. In signal: damped harmonic oscillator with periodic forcing = random walk with periodic restart.
**Notes:** The restart probability α controls the locality of the walk. High α = stays near seeds. Low α = explores broadly. This is the distance/specificity knob.

---

## Wave / Propagation Atoms

### propagate (cross-domain alias: `wavefront`, `time-step`, `displace`)
**Domain:** Physics / Diffusion
**Definition:** Wave equation: ∂²u/∂t² = c²·∇²u. The wavefront advances at speed c — information travels at speed c. Each point's state depends on its neighbors at time t−Δt.
**Atom or composite:** Composite: for each point: u_new = 2·u_current − u_prev + (c·Δt/Δx)²·∇²u.
**Cost model:** O(N) per time step. Leapfrog / Verlet integration is the standard method.
**Real wall?** Yes — the speed of information propagation c is a real physical constant. Nothing can propagate faster than c (in special relativity). In the wave equation: the CFL condition c·Δt/Δx ≤ 1.
**Cross-domain wiring:** Signal propagation in transmission lines = wave equation. Shock wave formation = non-linear wave equation. In retrieval: information cascade = wavefront on a social graph.
**Notes:** When the wave hits a boundary, it reflects (Dirichlet or Neumann boundary conditions). The reflection rule is the boundary condition — different conditions give different reflection behaviors.

### reflect (cross-domain alias: `bounce`, `scatter`, `mirror`)
**Domain:** Physics / Diffusion
**Definition:** When a wave hits a boundary, it reflects. Dirichlet: u=0 at boundary (fixed end). Neumann: ∂u/∂n=0 (free end). Mixed: Robin boundary = a·u + b·∂u/∂n = 0.
**Atom or composite:** Composite: at boundary: compute incoming signal → apply reflection rule → inject reflected signal.
**Cost model:** One extra computation per boundary point per step.
**Real wall?** No.
**Cross-domain wiring:** Signal reflection at impedance mismatch = same as wave reflection. Light reflection = electromagnetic wave reflection. In retrieval: document citing itself = self-reflection.
**Notes:** The reflection coefficient R = (Z_2 − Z_1)/(Z_2 + Z_1) where Z is the impedance. R=0 (no reflection) when impedances match — this is matched in matched filter.

### diffract (cross-domain alias: `bend`, `scatter-around`, `knife-edge`)
**Domain:** Physics / Diffusion
**Definition:** Wave bending around obstacles and through apertures, described by Huygens' principle: every point on a wavefront is a source of secondary spherical wavelets. The Fresnel and Fraunhofer regimes describe the near and far diffraction patterns.
**Atom or composite:** Composite: for each point on aperture/obstacle edge: compute contribution to field at observation point using the Huygens integral.
**Cost model:** O(N·M) where N is the number of edge points and M is the number of observation points. Fast multipole methods reduce this to O(N log N).
**Real wall?** Yes — the wavelength λ sets the diffraction scale. Objects much larger than λ cast sharp shadows; objects comparable to λ produce significant diffraction. λ is the conserved quantity.
**Cross-domain wiring:** Diffraction limit in optics = resolution limit = wavelength-governed. In retrieval: query expansion around a semantic boundary = semantic diffraction.
**Notes:** The Fraunhofer diffraction pattern of a slit = sinc function. The diffraction pattern IS the Fourier transform of the aperture. This connects diffraction to the FFT primitive.

---

## Monte Carlo / Sampling Atoms

### sample-uniform (cross-domain alias: `rand`, `uniform-sample`, `random-pick`)
**Domain:** Physics / Diffusion
**Definition:** Generate a uniformly distributed random number in [0,1). All values equally likely. Maps to uniform sampling in any range via: x = a + (b−a)·U(0,1).
**Atom or composite:** Atom (the primitive RNG). The generator is the PRNG algorithm (Mersenne Twister, PCG, xorshift).
**Cost model:** One RNG call. Modern CPUs have hardware random number generation.
**Real wall?** No.
**Cross-domain wiring:** All other sampling methods start with uniform sampling. Inverse transform sampling: U(0,1) → CDF⁻¹(U) → arbitrary distribution. Rejection sampling: accept/reject uniform samples. In signal: dithering = uniform noise added to quantize.
**Notes:** The distinction between PRNG (deterministic, fast) and TRNG (hardware, slow, truly random) is the generator/primitive split.

### sample-importance (cross-domain alias: `weighted-sample`, `importance-resample`, `SIR`)
**Domain:** Physics / Diffusion
**Definition:** Sample from a target distribution π(x) using a proposal distribution q(x) that we can sample from. Weight each sample by w(x) = π(x)/q(x) and resample proportional to weights.
**Atom or composite:** Composite: sample from q(x) → compute weight w = π(x)/q(x) → resample with probabilities ∝ w.
**Cost model:** O(N) for N samples per round. Importance weights can have high variance.
**Real wall?** Yes — high variance in importance weights causes instability. The gap between π and q (the mismatch) is the real cost. A bad proposal can make the estimator useless.
**Cross-domain wiring:** Particle filter = importance sampling + resampling. In retrieval: weighted fusion of candidate lists = importance-weighted sampling of candidates.
**Notes:** The effective sample size (ESS) = (Σw_i)²/Σw_i² measures how many effective samples you have. Low ESS means the weights are very unequal — most samples are wasted.

### metropolis-sample (cross-domain alias: `MCMC`, `markov-chain-sample`, `monte-carlo-markov-chain`)
**Domain:** Physics / Diffusion
**Definition:** Generate samples from π(x) by building a Markov chain whose stationary distribution is π. Proposal q(x'|x) → accept with probability min(1, π(x')/π(x)). Uses detailed balance to guarantee convergence to π.
**Atom or composite:** Composite: propose new state → compute acceptance ratio → accept/reject → iterate.
**Cost model:** O(1) per sample (propose + accept/reject). Burn-in period of ~1000 steps is typically needed.
**Real wall?** Yes — autocorrelation time can be very large for multi-modal distributions. The chain can get stuck in local modes. This is the fundamental limitation of MCMC — the mixing time is the conserved quantity.
**Cross-domain wiring:** Gibbs sampling = Metropolis-Hastings with full conditional proposals. Simulated annealing = Metropolis with decreasing temperature → ground state. In retrieval: beam search = MCMC with deterministic proposals.
**Notes:** Simulated annealing is Metropolis-Hastings with a temperature schedule: high T explores broadly, low T converges to the mode. The cooling schedule is the key hyperparameter.

### gibbs-sample (cross-domain alias: `conditional-sample`, `blocked-gibbs`, `coordinate-descent-MCMC`)
**Domain:** Physics / Diffusion
**Definition:** Specialized Metropolis where proposals are drawn from the full conditional distribution P(x_i | x_{-i}). No rejection needed — always accept.
**Atom or composite:** Composite: for each variable: sample from its conditional distribution given all others → iterate.
**Cost model:** O(1) per variable per iteration. Requires being able to sample from all full conditionals.
**Real wall?** No.
**Cross-domain wiring:** Latent Dirichlet Allocation = Gibbs sampling over topic assignments. In linear algebra: coordinate descent = same structure.
**Notes:** Gibbs is the workhorse of probabilistic graphical models because the full conditionals are often tractable even when the joint is not.

---

## Force-Directed / Particle Atoms

### attract (cross-domain alias: `pull`, `spring-force`, `inverse-square`)
**Domain:** Physics / Diffusion
**Definition:** Force between two particles: F = G·m₁·m₂/r² (gravity) or F = k·x (Hooke's law spring). Directed toward the other particle.
**Atom or composite:** Atom (for the mathematical form)
**Cost model:** One inverse-square computation per pair per step. For N particles: O(N²) naive. Barnes-Hut tree reduces to O(N log N).
**Real wall?** No. But long-range forces (gravity) create O(N²) interactions — the tree is necessary, not optional.
**Cross-domain wiring:** Spring force in physics = attractive potential gradient = same as kernel density estimation with a Gaussian kernel. In retrieval: attraction between semantically similar concepts = spring-like force in a semantic embedding space.
**Notes:** The t-SNE / UMAP layout algorithm uses attractive forces between nearby points + repulsive forces between all points — a force-directed graph layout.

### repel (cross-domain alias: `push`, `coulomb-force`, `short-range-repulsion`)
**Domain:** Physics / Diffusion
**Definition:** Force directed away from another particle: F = k/r² (Coulomb-like). Prevents overlap and causes uniform spacing.
**Atom or composite:** Atom
**Cost model:** Same as attract — O(N²) or O(N log N) with tree.
**Real wall?** No.
**Cross-domain wiring:** In potential fields: repulsive potential = safety zone. In retrieval: diversity penalty = repulsion between similar documents to avoid redundancy.
**Notes:** In t-SNE, repulsion between all pairs is implemented via a tree approximation — Barnes-Hut or fast multipole. The approximation is necessary.

### damp (cross-domain alias: `friction`, `decay`, `velocity-smooth`)
**Domain:** Physics / Diffusion
**Definition:** Velocity damping: v_new = v · (1 − γ·Δt). Reduces velocity each step, causing particles to settle.
**Atom or composite:** Composite: multiply velocity by decay factor each step.
**Cost model:** One multiplication per particle per step.
**Real wall?** No.
**Cross-domain wiring:** Exponential decay in signal processing. Momentum in gradient descent (friction on parameter updates). In retrieval: score decay over time.
**Notes:** Critical damping: γ·Δt = 2 (overdamped: settles slowly; underdamped: oscillates). The optimal damping ratio for convergence speed depends on the landscape curvature.

---

## Reaction-Diffusion Atoms

### react (cross-domain alias: `interact`, `transform`, `convert`)
**Domain:** Physics / Diffusion
**Definition:** Two or more species interact and change form: A + B → C. Governed by the reaction rate k. Rate: dC/dt = k·A·B.
**Atom or composite:** Composite: for each pair: compute reaction rate → update concentrations.
**Cost model:** O(N²) per step for pairwise reactions in well-mixed systems. Spatial reactions require tracking local concentrations.
**Real wall?** No.
**Cross-domain wiring:** Chemical reaction = logistic growth. A + B → A (catalysis) = autocatalysis. In retrieval: concept fusion = reaction between query and document features.
**Notes:** Turing patterns (reaction-diffusion morphogenesis) emerge from the interaction of an activator (slow diffusion) and an inhibitor (fast diffusion). The diffusion rate difference is the key — it's the parameter that breaks the symmetry.

### turing-instability (cross-domain alias: `pattern-form`, `morphogen`, `self-organize`)
**Domain:** Physics / Diffusion
**Definition:** A reaction-diffusion system becomes unstable to spatial perturbations when the inhibitor diffuses faster than the activator (D_inhibitor >> D_activator). This produces stable spatial patterns from uniform initial conditions.
**Atom or composite:** Composite: react(A, B) + diffuse(A, D_A) + diffuse(B, D_B). Requires D_B/D_A > threshold (Turing condition).
**Cost model:** O(N) per step for spatial simulation.
**Real wall?** Yes — the Turing condition is a real constraint. If diffusion rates are too similar, no pattern forms. The pattern wavelength is set by the reaction kinetics, not by external instructions.
**Cross-domain wiring:** Pattern formation in biology = Turing patterns in chemistry. In retrieval: topic clustering = same kind of spontaneous symmetry breaking on a document graph.
**Notes:** This is the most conceptually interesting primitive: structure emerges from the interaction of two processes with different diffusion rates. No central controller specifies where the spots go — the pattern is intrinsic.

---

## Summary: Physics Atom → Cross-Domain Wiring

| Physics Primitive | Retrieval Alias | Signal Alias | Graphics Alias |
|---|---|---|---|
| diffuse | label propagation | smoothing filter | Gaussian blur |
| laplacian | graph smoothing | edge detection filter | Laplacian-of-Gaussian |
| diffuse-decay | distance-discounted relevance | exponential decay | vignette |
| spread-activate | relevance propagation | message passing | bloom/glow |
| percolate | giant-component cascade | phase transition | flood fill |
| random-walk-Restart | personalized pagerank | damped oscillation | importance sampling |
| propagate | information cascade | wavefront | scan line |
| reflect | self-citation echo | echo/reflection | mirror/bounce |
| diffract | semantic boundary bending | diffraction | soft shadow |
| sample-importance | weighted candidate fusion | importance sampling | weighted blending |
| metropolis | beam search | simulated annealing | stochastic sampling |
| attract | semantic clustering | spring | positive feedback |
| repel | diversity penalty | repulsion | anti-aliasing |
| damp | score decay | friction | easing |

---

*Last updated: 2026-06-21*
*Source doctrine: The Painted Fence — Jesse*


---

## Stochastic / Fokker-Planck Atoms

### fokker-planck (cross-domain alias: `FP-equation`, `Kramers-Moyal`, `FPE`)
**Domain:** Physics / Diffusion
**Definition:** The Fokker-Planck equation: ∂P/∂t = −∂/∂x(D₁(x)P) + ∂²/∂x²(D₂(x)P). Describes the evolution of the probability density under drift (D₁) and diffusion (D₂). The Kramers-Moyal expansion truncates after the first two terms.
**Atom or composite:** Composite: compute drift and diffusion coefficients → update probability density via the FP equation.
**Cost model:** O(N) per time step for 1D. Higher dimensions are much more expensive (curse of dimensionality).
**Real wall?** Yes — the FP equation is valid only when the underlying noise is approximately delta-correlated (Markovian). Colored noise requires generalized FP equations with memory terms.
**Cross-domain wiring:** FP equation = the master equation for stochastic processes. In ML: the FP equation describes the evolution of the probability distribution under a diffusion process (score matching, diffusion models).
**Notes:** The Ornstein-Uhlenbeck process (linear drift, constant diffusion) has a Gaussian stationary distribution — the FP equation has an exact solution.

### langevin-dynamics (cross-domain alias: `stochastic-heat`, `Langevin-eq`, `Brownian-motion`)
**Domain:** Physics / Diffusion
**Definition:** Stochastic differential equation: dx = −∇U(x)dt + √(2D)dW where dW is the Wiener process (Brownian motion). The noise term models thermal fluctuations; the drift term is the deterministic force.
**Atom or composite:** Composite: x_{n+1} = x_n − ∇U(x_n)·Δt + √(2D·Δt)·N(0,1).
**Cost model:** O(N) per step where N = number of particles/dimensions. Noise generation is the main cost.
**Real wall?** No. But the time step Δt must be small enough to resolve the fastest dynamics.
**Cross-domain wiring:** Langevin dynamics = Hamiltonian dynamics + heat bath. In ML: Langevin dynamics is used for sampling from posterior distributions (stochastic gradient Langevin dynamics, SGLD).
**Notes:** Overdamped Langevin (no inertia) is appropriate when inertia timescales are much faster than the dynamics of interest. Underdamped Langevin includes momentum.

### brownian-motion (cross-domain alias: `Wiener-process`, `random-walk-diffusion`, `diffusive-scaling`)
**Domain:** Physics / Diffusion
**Definition:** The Wiener process W(t): W(0)=0, increments are independent and N(0,Δt). A particle undergoing Brownian motion has displacement distributed as N(0, 2D·t) in d dimensions.
**Atom or composite:** Atom
**Cost model:** One random number per step per dimension.
**Real wall?** No. But the mean squared displacement = 2d·D·t is the defining signature. Deviations from this scaling indicate non-diffusive transport.
**Cross-domain wiring:** Brownian motion = random walk with continuous time = the limiting case of all stochastic processes. In retrieval: random walk on a document graph = Brownian motion on the document space.
**Notes:** The Lévy flight is a generalization where step sizes follow a heavy-tailed distribution — this is super-diffusive (faster than Brownian). Animal foraging patterns and financial returns exhibit Lévy flight statistics.

---

## Statistical Mechanics Atoms

### partition-function (cross-domain alias: `Z`, `statistical-sum`, `Gibbs-functional`)
**Domain:** Physics / Diffusion
**Definition:** Z = Σ_{states} e^{−βE(state)}. The partition function encodes all thermodynamic properties: F = −kT·log Z, S = −∂F/∂T, C = −T·∂²F/∂T².
**Atom or composite:** Composite: for each state: compute energy → add exp(−βE) → sum.
**Cost model:** Exponential in system size — intractable for large systems. Mean field, transfer matrix, and Monte Carlo methods are approximations.
**Real wall?** Yes — Z cannot be computed exactly for most interesting systems. The computational cost of computing Z is exponential in the number of degrees of freedom.
**Cross-domain wiring:** Z = generating function = moment generating function = Σ p(x)·e^{β·x}. In ML: the partition function of a Boltzmann machine = the normalization constant that makes it a proper distribution.
**Notes:** The free energy F is the central quantity — all thermodynamic observables are derivatives of F. The entropy S = k·log Z + E/T is the information-theoretic entropy of the system.

### mean-field (cross-domain alias: `self-consistent-field`, `variational-approx`, `Weiss-field`)
**Domain:** Physics / Diffusion
**Definition:** Replace the interaction of a spin with all others by its average (the mean field). The mean field h_MF = J·⟨s⟩ acts on each spin, giving a self-consistent equation: ⟨s⟩ = tanh(β·h_MF).
**Atom or composite:** Composite: guess ⟨s⟩ → compute mean field → compute new ⟨s⟩ = tanh(β·h_MF) → iterate until convergence.
**Cost model:** One self-consistency update per iteration — very cheap. Converges quickly for most systems.
**Real wall?** Yes — mean field ignores fluctuations. It overestimates order in low-dimensional systems and near critical points.
**Cross-domain wiring:** Mean field = variational approximation = the simplest form of variational inference. In ML: mean field = mean field approximation in variational autoencoders.
**Notes:** The Bethe-Peierls approximation extends mean field by including correlations with immediate neighbors — it is more accurate near phase transitions.

### metropolis-hastings (cross-domain alias: `MCMC-sampler`, `proposal-accept`, `Markov-chain`)
**Domain:** Physics / Diffusion
**Definition:** Markov chain Monte Carlo: propose a new state x' from proposal q(x'|x) → accept with probability min(1, π(x')·q(x|x')/(π(x)·q(x'|x))). For symmetric q, accept with min(1, π(x')/π(x)).
**Atom or composite:** Composite: propose → compute acceptance ratio → accept/reject → update state.
**Cost model:** O(1) per sample (proposal + accept/reject). Effective sample size is much lower than 1 due to autocorrelation.
**Real wall?** Yes — the mixing time (time to converge to π) can be very long for multi-modal distributions. The proposal distribution q must be tuned for good mixing.
**Cross-domain wiring:** MH = the fundamental MCMC algorithm. Gibbs sampling = MH with full conditional proposals. Simulated annealing = MH with a temperature schedule. In retrieval: beam search = MH with deterministic proposals.
**Notes:** The acceptance rate of a symmetric random-walk proposal (q(x'|x) = N(x, σ²)) is optimal around 0.234 in high dimensions — this is the Gelman rule of thumb for tuning σ.

---

## Ising / Lattice Atoms

### ising-model (cross-domain alias: `spin-lattice`, `ferromagnet`, `Heisenberg-approx`)
**Domain:** Physics / Diffusion
**Definition:** A lattice of spins s_i ∈ {+1,−1} with Hamiltonian H = −J·Σ_{⟨i,j⟩} s_i·s_j − h·Σ_i s_i. At T > T_c (critical temperature), the system is disordered; at T < T_c, long-range order appears.
**Atom or composite:** Composite: for each spin: compute local field → flip with Metropolis/HB acceptance → update.
**Cost model:** O(N) per Monte Carlo sweep. N = number of spins. Critical slowing down (autocorrelation time diverges) near T_c.
**Real wall?** Yes — the correlation length ξ diverges at T_c, making it impossible to sample large systems efficiently near the critical point without cluster algorithms.
**Cross-domain wiring:** Ising model = binary Hopfield network = Boltzmann machine with pairwise interactions. In retrieval: the Ising model of document relevance = spin glass model of retrieval.
**Notes:** The critical temperature for the 2D Ising model on a square lattice is exactly T_c = 2.269J/k. This is one of the few exactly solvable models with a phase transition.

### cluster-flip (cross-domain alias: `Wolff-algorithm`, `Swendsen-Wang`, `non-local-update`)
**Domain:** Physics / Diffusion
**Definition:** Cluster algorithms flip entire connected clusters of spins at once, avoiding the critical slowing down of local Metropolis updates near T_c. Wolff: grow a cluster by adding neighbors with probability p = 1 − e^{−2βJ}.
**Atom or composite:** Composite: start from random spin → for each neighbor with same spin: add to cluster with probability p → repeat until no more to add → flip entire cluster.
**Cost model:** One cluster flip vs O(N) local flips — much faster in correlated regimes. Cluster size grows near T_c, making the algorithm more efficient when it's needed most.
**Real wall?** No.
**Cross-domain wiring:** Swendsen-Wang flips two clusters (one for +1, one for −1), avoiding bias. In ML: cluster updates = simultaneous parameter changes in Hopfield networks.
**Notes:** The Wolff algorithm has near-zero critical slowing down for the Ising model, making it the standard for high-precision Monte Carlo studies of the Ising universality class.

---

## Fluid / Advection Atoms

### advect (cross-domain alias: `transport`, `follow-field`, `conservative-move`)
**Domain:** Physics / Diffusion
**Definition:** Move a quantity along a velocity field: dq/dt = −v·∇q. The conservative form: ∂q/∂t + ∇·(v·q) = 0. Discretized via semi-Lagrangian advection: trace particle backward → interpolate value.
**Atom or composite:** Composite: for each grid cell: trace backwards along velocity → interpolate q from the backward position → update q.
**Cost model:** One backward trace + interpolation per cell. Semi-Lagrangian advection is unconditionally stable (can use large time steps).
**Real wall?** No. But semi-Lagrangian advection is diffusive (numerical diffusion along characteristics). Conservative (finite volume) advection is more accurate for conserved quantities.
**Cross-domain wiring:** Advection = transport along a field. In retrieval: following a gradient = advecting in the relevance gradient field.
**Notes:** The semi-Lagrangian method (S利amard, 1968) was originally developed for atmospheric modeling. It is unconditionally stable but first-order accurate.

### conserve (cross-domain alias: `finite-volume`, `divergence-theorem`, `integral-conservation`)
**Domain:** Physics / Diffusion
**Definition:** A conserved quantity satisfies ∫_V q dV = const. In a discretized system: the change in q inside a cell = flux through faces. The finite volume method ensures conservation cell-by-cell.
**Atom or composite:** Composite: for each face: compute flux(q) → sum divergences → update cell average.
**Cost model:** One flux computation per face per time step.
**Real wall?** No. But the accuracy of the flux computation determines the overall accuracy.
**Cross-domain wiring:** Conservation = the discrete version of the divergence theorem. In ML: weight decay preserves the norm (conserves parameter magnitude).
**Notes:** The Godunov method uses the exact Riemann solver for flux computation. The HLL (Harten-Lax-van Leer) approximate Riemann solver is cheaper and widely used.

### incompressible-solve (cross-domain alias: `Poisson-pressure`, `divergence-free`, `projection-method`)
**Domain:** Physics / Diffusion
**Definition:** Enforce incompressibility (∇·v = 0) via a pressure Poisson equation: ∇²p = ∇·v (divergence-free constraint). The pressure gradient subtracts out the divergence.
**Atom or composite:** Composite: compute velocity divergence → solve Poisson equation for pressure (FFT in periodic domain, multigrid otherwise) → subtract pressure gradient from velocity.
**Cost model:** Solving the Poisson equation dominates — O(N log N) with FFT, O(N) with multigrid.
**Real wall?** Yes — the Poisson solver is the bottleneck of incompressible flow simulations. Multigrid is the standard approach for large-scale problems.
**Cross-domain wiring:** Projection method = Helmholtz decomposition = separating a vector field into divergence-free and curl-free components. In signal: projection onto the orthogonal complement.
**Notes:** The Chorin (1968) projection method is the standard algorithm: compute velocity → project to divergence-free → repeat. The pressure enforces the incompressibility constraint.

---

## Numerical Methods Atoms

### forward-euler (cross-domain alias: `explicit-scheme`, `first-order-taylor`, `ODE-step`)
**Domain:** Physics / Diffusion
**Definition:** y_{n+1} = y_n + Δt·f(t_n, y_n). Simplest time-stepping scheme for ODEs.
**Atom or composite:** Atom
**Cost model:** One function evaluation per step.
**Real wall?** Yes — conditionally stable: Δt must satisfy the CFL condition. For stiff systems, forward Euler is unstable even for very small Δt.
**Cross-domain wiring:** Forward Euler = gradient descent with step size Δt. In optimization: the learning rate = time step.
**Notes:** Second-order Runge-Kutta (RK2 / midpoint method): k₁ = f(t, y); k₂ = f(t+Δt, y+Δt·k₁); y_{n+1} = y + Δt·k₂. Much better stability than forward Euler.

### backward-euler (cross-domain alias: `implicit-scheme`, `A-stable`, `backward-diff`)
**Domain:** Physics / Diffusion
**Definition:** y_{n+1} = y_n + Δt·f(t_{n+1}, y_{n+1}). Implicit — requires solving a non-linear equation at each step.
**Atom or composite:** Composite: for stiff systems: use Newton's method to solve y_{n+1} − Δt·f(y_{n+1}) = y_n.
**Cost model:** Requires solving an implicit equation (linear system for linear f, Newton's method for non-linear). Much more expensive per step than explicit methods.
**Real wall?** No. But the implicit solve adds complexity and cost per step.
**Cross-domain wiring:** Backward Euler = implicit gradient descent. In optimization: implicit regularization = the scheme that prevents overfitting.
**Notes:** Backward Euler is A-stable (stable for all complex Δt in the left half-plane) — this is the strongest stability property any linear multistep method can have.

### crank-nicolson (cross-domain alias: `second-order-implicit`, `unconditionally-stable`, `average-scheme`)
**Domain:** Physics / Diffusion
**Definition:** y_{n+1} = y_n + (Δt/2)·[f(t_n, y_n) + f(t_{n+1}, y_{n+1})]. The average of forward and backward Euler. Second-order accurate and A-stable.
**Atom or composite:** Composite: form the tridiagonal system (A + (Δt/2)·J)·y_{n+1} = ... → solve via Thomas algorithm (O(n)).
**Cost model:** One tridiagonal solve per step. The Thomas algorithm is O(n) for n unknowns — cheap.
**Real wall?** No.
**Cross-domain wiring:** CN for the heat equation = the same as the exponential Euler method. In ML: CN is equivalent to the trapezoidal rule for ODE integration.
**Notes:** CN is the workhorse for diffusive problems: heat equation, diffusion equation, advection-diffusion. It is second-order accurate in both time and space (when combined with central differences).

### runge-kutta4 (cross-domain alias: `RK4`, `fourth-order-classical`, `classic-scheme`)
**Domain:** Physics / Diffusion
**Definition:** y_{n+1} = y_n + (Δt/6)·(k₁ + 2k₂ + 2k₃ + k₄) where k₁ = f(t,y), k₂ = f(t+Δt/2, y+Δt·k₁/2), etc.
**Atom or composite:** Composite: four function evaluations per step.
**Cost model:** Four f evaluations per step. Fourth-order accurate — the most accurate method per function evaluation among one-step methods.
**Real wall?** No. But for very stiff systems, RK4 is still explicit and requires small Δt.
**Cross-domain wiring:** RK4 = the Simpson's rule of ODE integration. In ML: RK4 is rarely used — adaptive step size methods (DOPRI5, RK45) are preferred.
**Notes:** The Butcher tableau of RK4 encodes the coefficients. Adaptive step size (RK45) uses embedded RK methods to estimate the error and adjust Δt.

---

## Classical Mechanics & Hamilton-Lagrange Formalism

### lagrangian-formulation (cross-domain alias: `L=T-V`, `lagrangian-mechanics`, `variational-mech`)
**Domain:** Physics / Diffusion
**Definition:** L(q, q̇, t) = T − V; equations of motion from δ∫L dt = 0. Coordinates q_i are generalized.
**Atom or composite:** Atom — scalar function on tangent bundle TQ.
**Cost model:** Symbolic derivation O(n) in DoF; numerical integration O(n³) per step for dense Jacobians.
**Real wall?** No.
**Cross-domain wiring:** linear-algebra-matrix (mass matrix), control-numerical-opt (optimal control via Pontryagin), quantum-computing (path integral lift).
**Notes:** Lagrange (1788, Mécanique Analytique). Coordinate-free reformulation of Newton.

### hamiltonian-formulation (cross-domain alias: `H=T+V`, `phase-space-energy`, `legendre-of-L`)
**Domain:** Physics / Diffusion
**Definition:** H(q,p,t) = p·q̇ − L; Hamilton's eqs q̇ = ∂H/∂p, ṗ = −∂H/∂q.
**Atom or composite:** Atom — scalar on cotangent bundle T*Q.
**Cost model:** Same as Lagrangian; symplectic integrators preserve volume.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (Ĥ operator), statistics-probability (canonical ensemble), control-numerical-opt (Hamilton-Jacobi-Bellman).
**Notes:** Hamilton (1833). Legendre transform of L in q̇ → p.

### action-principle (cross-domain alias: `least-action`, `stationary-action`, `δS=0`)
**Domain:** Physics / Diffusion
**Definition:** S = ∫_{t₁}^{t₂} L dt; physical trajectory makes δS = 0 with fixed endpoints.
**Atom or composite:** Atom — defines the variational kernel.
**Cost model:** Analytic; discretized in path-integral MC as O(N_path).
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (Feynman path integral e^{iS/ℏ}), control-numerical-opt (action = cost functional).
**Notes:** Maupertuis, Euler, Lagrange, Hamilton. Generalizes Fermat's principle from optics.

### euler-lagrange-equation (cross-domain alias: `EL-eq`, `lagrange-equation`, `variational-eom`)
**Domain:** Physics / Diffusion
**Definition:** d/dt (∂L/∂q̇_i) − ∂L/∂q_i = 0 — necessary condition for stationary action.
**Atom or composite:** Atom — local PDE in time.
**Cost model:** Symbolic O(n); numeric integration depends on stiffness.
**Real wall?** No.
**Cross-domain wiring:** control-numerical-opt (calculus of variations), photonics-optics (eikonal/Fermat).
**Notes:** Derived from δS = 0 via integration by parts. Foundation of classical field theory.

### noethers-theorem (cross-domain alias: `symmetry→conservation`, `noether-current`, `continuous-symmetry-law`)
**Domain:** Physics / Diffusion
**Definition:** Every continuous symmetry of S yields a conserved current ∂_μ j^μ = 0 (or conserved charge Q = ∫j⁰ d³x).
**Atom or composite:** Atom — meta-theorem.
**Cost model:** Symbolic; no numerical cost.
**Real wall?** No.
**Cross-domain wiring:** electromagnetics-antennas (charge conservation), quantum-computing (angular momentum), astrophysics-cosmology (energy-momentum).
**Notes:** Emmy Noether (1918). Time translation → energy; spatial → momentum; rotation → angular momentum.

### poisson-brackets (cross-domain alias: `{f,g}`, `classical-commutator`, `phase-space-bracket`)
**Domain:** Physics / Diffusion
**Definition:** {f,g} = Σ_i (∂f/∂q_i)(∂g/∂p_i) − (∂f/∂p_i)(∂g/∂q_i); df/dt = {f,H} + ∂f/∂t.
**Atom or composite:** Atom — bilinear antisymmetric.
**Cost model:** O(n) per evaluation.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing ({,}→(1/iℏ)[,] correspondence), linear-algebra-matrix (Lie bracket structure).
**Notes:** Poisson (1809). Lie algebra of smooth functions on phase space.

### canonical-transformation (cross-domain alias: `symplectomorphism`, `phase-space-coord-change`, `(q,p)→(Q,P)`)
**Domain:** Physics / Diffusion
**Definition:** Transform (q,p) → (Q,P) preserving the symplectic form dp∧dq = dP∧dQ; equivalently preserves Poisson brackets.
**Atom or composite:** Composite — generated by generating function F.
**Cost model:** O(n²) Jacobian check.
**Real wall?** No.
**Cross-domain wiring:** control-numerical-opt (Hamilton-Jacobi), linear-algebra-matrix (Sp(2n,ℝ) group).
**Notes:** Four standard types F₁(q,Q), F₂(q,P), F₃(p,Q), F₄(p,P). Identity = F₂ = q·P.

### hamilton-jacobi-equation (cross-domain alias: `HJ-eq`, `action-as-generating-fn`, `∂S/∂t+H=0`)
**Domain:** Physics / Diffusion
**Definition:** ∂S/∂t + H(q, ∂S/∂q, t) = 0 — PDE for Hamilton's principal function S(q,t).
**Atom or composite:** Atom — first-order nonlinear PDE.
**Cost model:** Method of characteristics O(N_char); separation of variables when integrable.
**Real wall?** Yes — generally not analytically separable (KAM).
**Cross-domain wiring:** control-numerical-opt (HJB = HJ + Bellman), photonics-optics (eikonal limit).
**Notes:** Hamilton-Jacobi is the classical limit of Schrödinger via S = −iℏ log ψ.

### symplectic-geometry (cross-domain alias: `(M,ω)`, `phase-space-geometry`, `even-dim-manifold`)
**Domain:** Physics / Diffusion
**Definition:** Even-dim manifold M with closed non-degenerate 2-form ω; dω = 0, ω^n ≠ 0.
**Atom or composite:** Atom — geometric kernel of Hamiltonian mechanics.
**Cost model:** Symbolic; affects integrator design (symplectic schemes).
**Real wall?** No.
**Cross-domain wiring:** linear-algebra-matrix (Sp(2n) group), control-numerical-opt (symplectic Runge-Kutta).
**Notes:** Darboux theorem: locally ω = Σ dp_i ∧ dq_i. Volume is preserved.

### liouvilles-theorem (cross-domain alias: `phase-volume-conservation`, `incompressible-flow-phase`, `df/dt=0`)
**Domain:** Physics / Diffusion
**Definition:** Hamiltonian flow preserves phase-space volume: ∇·(q̇,ṗ) = 0; for density ρ(q,p,t): ∂ρ/∂t + {ρ,H} = 0.
**Atom or composite:** Atom — corollary of symplecticity.
**Cost model:** Constraint on integrators; symplectic schemes enforce it exactly.
**Real wall?** No (in Hamiltonian systems).
**Cross-domain wiring:** statistics-probability (microcanonical ensemble), condensed-matter (Vlasov eq).
**Notes:** Liouville (1838). Underpins ergodic hypothesis and statistical mechanics.

### phase-space (cross-domain alias: `(q,p)-space`, `cotangent-bundle`, `T*Q`)
**Domain:** Physics / Diffusion
**Definition:** 2n-dimensional manifold parameterized by generalized coordinates q_i and conjugate momenta p_i.
**Atom or composite:** Atom — substrate of classical dynamics.
**Cost model:** State vector size 2n; integration cost depends on H.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (Wigner function lives on phase space), statistics-probability (ensemble density).
**Notes:** Distinct from configuration space Q. Trajectories never cross in phase space (uniqueness).

### action-angle-variables (cross-domain alias: `(I,θ)`, `integrable-coords`, `tori-coordinates`)
**Domain:** Physics / Diffusion
**Definition:** Canonical coords (I_i, θ_i) for integrable systems where H = H(I) only; θ_i = ω_i(I)·t + θ_i(0).
**Atom or composite:** Composite — exists only when ≥n independent conserved quantities.
**Cost model:** Solve H = H(I): hard in general.
**Real wall?** Yes — most systems are non-integrable.
**Cross-domain wiring:** signal-processing-rf (frequency analysis), astrophysics-cosmology (orbital mechanics).
**Notes:** Liouville-Arnold theorem: integrable ⇒ invariant tori. Foundation of perturbation theory.

### KAM-theorem (cross-domain alias: `Kolmogorov-Arnold-Moser`, `tori-survival`, `near-integrable`)
**Domain:** Physics / Diffusion
**Definition:** Under small perturbations of an integrable H, most invariant tori (with Diophantine frequencies) survive.
**Atom or composite:** Atom — theorem.
**Cost model:** Numerical detection of surviving tori is delicate; Lindstedt series.
**Real wall?** Yes — resonant tori are destroyed.
**Cross-domain wiring:** astrophysics-cosmology (solar system stability), control-numerical-opt (chaos onset).
**Notes:** Kolmogorov (1954), Arnold (1963), Moser (1962). Resolved the small-divisor problem.

### lyapunov-exponents (cross-domain alias: `λ_max`, `chaos-rate`, `exponential-divergence`)
**Domain:** Physics / Diffusion
**Definition:** λ_i = lim_{t→∞} (1/t) log |δx_i(t)|/|δx_i(0)| — exponential rate of trajectory separation.
**Atom or composite:** Atom — averaged quantity.
**Cost model:** O(n² T) for n DoF, T steps with Gram-Schmidt re-orthogonalization.
**Real wall?** No (numerically); convergence may be slow.
**Cross-domain wiring:** statistics-probability (KS entropy), quantum-computing (OTOC), signal-processing-rf (spectral analysis).
**Notes:** Positive max λ ⇒ chaos. Sum of λ_i = average divergence of flow.

### integrable-systems (cross-domain alias: `Liouville-integrable`, `n-conserved-quantities`, `solvable-by-quadrature`)
**Domain:** Physics / Diffusion
**Definition:** Hamiltonian with n independent Poisson-commuting conserved quantities {F_i, F_j} = 0 for n DoF.
**Atom or composite:** Composite — strong constraint.
**Cost model:** Action-angle reduction makes integration trivial.
**Real wall?** Yes — measure-zero class in generic Hamiltonians.
**Cross-domain wiring:** quantum-computing (Bethe ansatz), condensed-matter (XXX chain), control-numerical-opt (Lax pairs).
**Notes:** Examples: Kepler, harmonic oscillator, Toda lattice, Calogero-Moser.

### central-force-problem (cross-domain alias: `Kepler-problem`, `radial-potential`, `V(r)`)
**Domain:** Physics / Diffusion
**Definition:** H = p²/(2m) + V(r); conserves E, L; effective 1D radial problem with V_eff = V(r) + L²/(2mr²).
**Atom or composite:** Composite — reduction via symmetry.
**Cost model:** Quadrature for r(t); Kepler's equation E − e sin E = M for orbits.
**Real wall?** No (analytic for V ∝ 1/r and V ∝ r²).
**Cross-domain wiring:** astrophysics-cosmology (planetary motion), quantum-computing (hydrogen atom).
**Notes:** Bertrand's theorem: only V∝1/r and V∝r² give closed bound orbits.

### rigid-body-euler-equations (cross-domain alias: `euler-eqs-rotation`, `body-frame-rotation`, `Iω̇=τ`)
**Domain:** Physics / Diffusion
**Definition:** I₁ω̇₁ = (I₂−I₃)ω₂ω₃ + τ₁ (and cyclic) — angular momentum in body frame.
**Atom or composite:** Atom — coupled ODE system.
**Cost model:** O(1) per step for 3 DoF; quaternions for orientation.
**Real wall?** No.
**Cross-domain wiring:** control-numerical-opt (spacecraft attitude), linear-algebra-matrix (SO(3)).
**Notes:** Euler (1758). Free body shows intermediate-axis (Dzhanibekov) instability.

### inertia-tensor (cross-domain alias: `I_ij`, `moment-of-inertia-tensor`, `rotational-mass`)
**Domain:** Physics / Diffusion
**Definition:** I_ij = ∫ρ(r)(δ_ij r² − r_i r_j) dV — symmetric 3×3 tensor; diagonalizable to principal axes.
**Atom or composite:** Atom — geometric integral.
**Cost model:** O(N) for N particles; eigendecomposition O(1) for 3×3.
**Real wall?** No.
**Cross-domain wiring:** linear-algebra-matrix (symmetric eigenproblem), condensed-matter (rotational spectra).
**Notes:** Parallel-axis theorem: I = I_cm + Md². Principal axes = eigenvectors.

### virial-theorem (cross-domain alias: `⟨2T⟩=⟨r·∇V⟩`, `time-average-energy`, `clausius-virial`)
**Domain:** Physics / Diffusion
**Definition:** For bound system with V ∝ r^n: 2⟨T⟩ = n⟨V⟩. Special case (Kepler n=−1): 2⟨T⟩ = −⟨V⟩.
**Atom or composite:** Atom — time-averaged identity.
**Cost model:** Estimation from MD trajectories O(N·T).
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (galaxy masses), condensed-matter (cohesive energy), statistics-probability (equipartition).
**Notes:** Clausius (1870). Used to estimate dark matter from velocity dispersions.

### normal-modes (cross-domain alias: `eigenmodes`, `principal-vibrations`, `decoupled-oscillators`)
**Domain:** Physics / Diffusion
**Definition:** Solve det(K − ω²M) = 0 for n-DoF coupled linear system; eigenvectors give mode shapes.
**Atom or composite:** Composite — uses generalized eigenproblem.
**Cost model:** O(n³) generalized eigendecomposition.
**Real wall?** No.
**Cross-domain wiring:** linear-algebra-matrix (gen. eigenproblem), signal-processing-rf (modal analysis), condensed-matter (phonons).
**Notes:** Coordinate transform diagonalizes M and K simultaneously. Foundation of phonons and molecular vibrations.

### holonomic-constraints (cross-domain alias: `f(q,t)=0-constraints`, `integrable-constraints`, `lagrange-multipliers`)
**Domain:** Physics / Diffusion
**Definition:** Constraints expressible as f_k(q,t) = 0; reduce 3N coordinates to 3N−k generalized coords.
**Atom or composite:** Atom — geometric restriction.
**Cost model:** Symbolic elimination, or λ_k method with O((n+k)³) DAE solve.
**Real wall?** No.
**Cross-domain wiring:** control-numerical-opt (DAE solvers), linear-algebra-matrix (constraint Jacobian).
**Notes:** Distinction matters: holonomic ⇒ reducible to fewer coords.

### nonholonomic-constraints (cross-domain alias: `velocity-constraints`, `rolling-constraints`, `pfaffian-form`)
**Domain:** Physics / Diffusion
**Definition:** Constraints of form Σ a_ki(q,t) q̇_i + a_k0 = 0 not integrable to f(q)=0; e.g., rolling without slipping.
**Atom or composite:** Composite — anholonomic.
**Cost model:** Use d'Alembert with multipliers; harder than holonomic.
**Real wall?** Yes — cannot eliminate via coords.
**Cross-domain wiring:** control-numerical-opt (path planning for wheeled robots).
**Notes:** Classic example: rolling coin, ice skate, car kinematics.

### generating-functions (cross-domain alias: `F₁,F₂,F₃,F₄`, `canonical-generator`, `legendre-transform-pairs`)
**Domain:** Physics / Diffusion
**Definition:** Functions F(old,new,t) that generate canonical transformations via implicit equations.
**Atom or composite:** Composite — four standard types.
**Cost model:** Symbolic; O(n) for solution.
**Real wall?** No.
**Cross-domain wiring:** control-numerical-opt (HJ method), quantum-computing (semiclassical propagator).
**Notes:** F₂ = q·P generates identity; ∂F₂/∂q = p, ∂F₂/∂P = Q. Legendre transform structure.

### geodesic-flow (cross-domain alias: `free-particle-on-manifold`, `kinetic-only-flow`, `levi-civita-transport`)
**Domain:** Physics / Diffusion
**Definition:** Hamiltonian flow with H = (1/2) g^{ij}(q) p_i p_j; trajectories are geodesics of metric g.
**Atom or composite:** Composite — H built from metric.
**Cost model:** Integrate geodesic eq d²q/dτ² + Γ q̇q̇ = 0; O(n²) per step.
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (GR free-fall), photonics-optics (Fermat in inhomogeneous medium).
**Notes:** Maupertuis principle: classical trajectories at fixed E are geodesics of Jacobi metric.

### principle-of-least-action (cross-domain alias: `δS=0`, `hamilton-principle`, `variational-physics`)
**Domain:** Physics / Diffusion
**Definition:** Physical trajectory between fixed (q₁,t₁) and (q₂,t₂) extremizes S = ∫L dt. Most paths are saddles.
**Atom or composite:** Atom — foundational variational statement.
**Cost model:** Discretized as O(N_steps) optimization.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (path integral sum over all paths), photonics-optics (Fermat's principle for light).
**Notes:** Hamilton (1834). Underpins classical and quantum theory; lifts to QFT as effective action.

---

## Electrodynamics & Maxwell

### maxwell-eq-differential (cross-domain alias: `∇·E,∇·B,∇×E,∇×B`, `local-maxwell`, `field-PDEs`)
**Domain:** Physics / Diffusion
**Definition:** ∇·E = ρ/ε₀; ∇·B = 0; ∇×E = −∂B/∂t; ∇×B = μ₀J + μ₀ε₀∂E/∂t.
**Atom or composite:** Composite — four coupled PDEs.
**Cost model:** FDTD O(N_cells) per step; FEM O(N^1.x).
**Real wall?** Yes — CFL c·Δt ≤ Δx/√d.
**Cross-domain wiring:** electromagnetics-antennas (full EM solver), photonics-optics (in dielectrics).
**Notes:** Maxwell (1865); unified by Heaviside into 4-vector form (1884).

### maxwell-eq-integral (cross-domain alias: `flux-form-maxwell`, `gauss-ampere-faraday-integrals`, `loop-and-surface`)
**Domain:** Physics / Diffusion
**Definition:** ∮E·dA = Q/ε₀; ∮B·dA = 0; ∮E·dl = −dΦ_B/dt; ∮B·dl = μ₀I + μ₀ε₀ dΦ_E/dt.
**Atom or composite:** Atom — Stokes-dual of differential form.
**Cost model:** Closed-form for high-symmetry; method of moments O(N²) for arbitrary.
**Real wall?** No.
**Cross-domain wiring:** electromagnetics-antennas (BEM/MoM), control-numerical-opt (conservation form).
**Notes:** Pedagogically primary; converts to differential via Stokes/Gauss theorems.

### gauss-law (cross-domain alias: `∇·E=ρ/ε₀`, `electric-flux-law`, `coulomb-integrated`)
**Domain:** Physics / Diffusion
**Definition:** ∇·E = ρ/ε₀ — local conservation of electric flux from charges.
**Atom or composite:** Atom — first Maxwell equation.
**Cost model:** Poisson solve O(N log N) FFT or multigrid.
**Real wall?** No.
**Cross-domain wiring:** statistics-probability (Poisson sampling), linear-algebra-matrix (Laplacian).
**Notes:** Equivalent to inverse-square Coulomb law; gives ϕ via ∇²ϕ = −ρ/ε₀.

### faraday-law (cross-domain alias: `∇×E=-∂B/∂t`, `induction-law`, `EMF-from-dΦ`)
**Domain:** Physics / Diffusion
**Definition:** ∇×E = −∂B/∂t; EMF = −dΦ_B/dt around closed loop.
**Atom or composite:** Atom — Maxwell #3.
**Cost model:** O(N_cells) curl in FDTD; cheap.
**Real wall?** No.
**Cross-domain wiring:** electromagnetics-antennas (transformers, motors), signal-processing-rf (inductive coupling).
**Notes:** Faraday (1831). Lenz's law from sign: induced current opposes flux change.

### ampere-maxwell-law (cross-domain alias: `∇×B=μ₀J+μ₀ε₀∂E/∂t`, `displacement-current-law`, `maxwell-correction`)
**Domain:** Physics / Diffusion
**Definition:** ∇×B = μ₀J + μ₀ε₀ ∂E/∂t. The ∂E/∂t term is Maxwell's displacement current.
**Atom or composite:** Composite — Ampère + Maxwell's addition.
**Cost model:** O(N_cells) curl.
**Real wall?** No.
**Cross-domain wiring:** photonics-optics (light is propagating ∂E/∂t coupling), electromagnetics-antennas.
**Notes:** Maxwell's displacement current closed the equations and predicted light c = 1/√(μ₀ε₀).

### displacement-current (cross-domain alias: `ε₀∂E/∂t`, `J_D`, `vacuum-current`)
**Domain:** Physics / Diffusion
**Definition:** J_D = ε₀ ∂E/∂t — current density that flows through vacuum from time-varying E.
**Atom or composite:** Atom — Maxwell's correction term.
**Cost model:** O(N_cells).
**Real wall?** No.
**Cross-domain wiring:** electromagnetics-antennas (capacitor between plates), photonics-optics (EM wave generation).
**Notes:** Maxwell (1861). Restores current continuity ∇·J + ∂ρ/∂t = 0.

### vector-potential (cross-domain alias: `A`, `magnetic-potential`, `B=∇×A`)
**Domain:** Physics / Diffusion
**Definition:** B = ∇×A; A is a 3-vector field; ∇·B = 0 automatically.
**Atom or composite:** Atom — gauge field.
**Cost model:** Vector Laplacian O(N log N).
**Real wall?** Yes — gauge ambiguity; need gauge fix.
**Cross-domain wiring:** quantum-computing (canonical momentum p−qA), photonics-optics (vector beams).
**Notes:** Lifts to 4-potential A^μ in covariant form. Source of Aharonov-Bohm phase.

### scalar-potential (cross-domain alias: `φ`, `electric-potential`, `voltage`)
**Domain:** Physics / Diffusion
**Definition:** E = −∇φ − ∂A/∂t; in statics E = −∇φ; ∇²φ = −ρ/ε₀ in Coulomb gauge.
**Atom or composite:** Atom — gauge field.
**Cost model:** Poisson solver O(N log N).
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (potential in Schrödinger), linear-algebra-matrix (discrete Laplacian).
**Notes:** Combined with A into 4-potential A^μ = (φ/c, A).

### gauge-transforms (cross-domain alias: `A→A+∇χ,φ→φ-∂χ/∂t`, `gauge-freedom`, `redundancy`)
**Domain:** Physics / Diffusion
**Definition:** A → A + ∇χ, φ → φ − ∂χ/∂t leaves E, B invariant for any χ(x,t).
**Atom or composite:** Atom — gauge symmetry.
**Cost model:** Choice affects solver conditioning.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (gauge invariance of wavefunction phase), field theory (Yang-Mills generalization).
**Notes:** Coulomb gauge ∇·A = 0; Lorenz gauge ∂_μA^μ = 0 (relativistic).

### lienard-wiechert-potentials (cross-domain alias: `LW-potentials`, `point-charge-potentials`, `retarded-charge-fields`)
**Domain:** Physics / Diffusion
**Definition:** φ = q / [4πε₀(R − v·R/c)]_ret; A = (v/c²) φ — fields of moving point charge at retarded time.
**Atom or composite:** Composite — built from retarded Green's function.
**Cost model:** O(1) per source, but O(N²) for N charges in time domain.
**Real wall?** Yes — retarded-time root-finding.
**Cross-domain wiring:** astrophysics-cosmology (pulsar radiation), electromagnetics-antennas (radiation).
**Notes:** Liénard (1898), Wiechert (1900). Foundation of classical radiation theory.

### retarded-potentials (cross-domain alias: `causality-potentials`, `t-r/c`, `green-fn-d-alembert`)
**Domain:** Physics / Diffusion
**Definition:** φ(r,t) = (1/4πε₀)∫ ρ(r',t_ret)/|r−r'| d³r' with t_ret = t − |r−r'|/c. Same for A.
**Atom or composite:** Composite — convolution with retarded Green's function.
**Cost model:** O(N²) direct; O(N log N) with multipole methods.
**Real wall?** Yes — speed-of-light causality.
**Cross-domain wiring:** signal-processing-rf (causal filters), control-numerical-opt (delay equations).
**Notes:** Solutions of □φ = −ρ/ε₀ in Lorenz gauge. Distinguished from advanced potentials by causality.

### em-waves (cross-domain alias: `light`, `radiation-EM`, `transverse-waves`)
**Domain:** Physics / Diffusion
**Definition:** In vacuum: □E = 0, □B = 0; plane wave E = E₀ exp(i(k·r − ωt)) with ω = c|k|, E⊥B⊥k.
**Atom or composite:** Atom — solution of source-free Maxwell.
**Cost model:** Plane wave free; FDTD O(N_cells).
**Real wall?** Yes — c is fundamental.
**Cross-domain wiring:** photonics-optics (light), signal-processing-rf (RF), astrophysics-cosmology (CMB).
**Notes:** Maxwell (1865) predicted c = 1/√(μ₀ε₀) ≈ speed of light, unifying optics + EM.

### poynting-vector (cross-domain alias: `S=E×B/μ₀`, `EM-energy-flux`, `radiation-flux`)
**Domain:** Physics / Diffusion
**Definition:** S = (1/μ₀) E × B — energy flux per unit area; dimension W/m².
**Atom or composite:** Atom — energy current.
**Cost model:** O(N_cells) pointwise.
**Real wall?** No.
**Cross-domain wiring:** photonics-optics (irradiance), electromagnetics-antennas (far-field power).
**Notes:** Poynting (1884). Poynting theorem: ∂u/∂t + ∇·S = −J·E.

### em-energy-density (cross-domain alias: `u=ε₀E²/2+B²/(2μ₀)`, `field-energy`, `EM-Hamiltonian-density`)
**Domain:** Physics / Diffusion
**Definition:** u = (ε₀/2)|E|² + (1/2μ₀)|B|² — local EM energy per volume.
**Atom or composite:** Atom — quadratic in fields.
**Cost model:** O(N_cells) pointwise.
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (cavity QED), photonics-optics (mode energy), statistics-probability (blackbody integration).
**Notes:** Total EM energy U = ∫u dV. Combined with S yields conservation law.

### multipole-expansion (cross-domain alias: `1/r,1/r²,1/r³`, `monopole-dipole-quadrupole`, `far-field-series`)
**Domain:** Physics / Diffusion
**Definition:** φ(r) = (1/4πε₀)[q/r + p·r̂/r² + (1/2)Q_ij r̂_i r̂_j/r³ + ...] for sources within radius R<r.
**Atom or composite:** Composite — series in 1/r.
**Cost model:** FMM accelerates many-body to O(N log N).
**Real wall?** Yes — series fails near sources.
**Cross-domain wiring:** astrophysics-cosmology (gravitational multipoles), photonics-optics (Mie expansion).
**Notes:** Spherical harmonics Y_l^m form natural basis. Lowest non-vanishing moment dominates far field.

### electric-dipole-radiation (cross-domain alias: `Larmor-dipole`, `p̈-radiation`, `lowest-order-radiation`)
**Domain:** Physics / Diffusion
**Definition:** Radiated power P = (μ₀ |p̈|²)/(6πc) where p̈ = second time derivative of dipole moment.
**Atom or composite:** Composite — multipole expansion lowest order.
**Cost model:** O(1) given p(t).
**Real wall?** No.
**Cross-domain wiring:** photonics-optics (atomic transitions), electromagnetics-antennas (Hertzian dipole).
**Notes:** Most atomic transitions are electric dipole (E1).

### magnetic-dipole-radiation (cross-domain alias: `M1-radiation`, `m̈-radiation`, `forbidden-dipole`)
**Domain:** Physics / Diffusion
**Definition:** P = (μ₀ |m̈|²)/(6πc³) — typically (v/c)² weaker than E1 for atoms.
**Atom or composite:** Composite — higher-order multipole.
**Cost model:** O(1).
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (M1 atomic transitions), astrophysics-cosmology (21cm hydrogen line).
**Notes:** Allowed when E1 is forbidden by selection rules. 21cm line is M1.

### quadrupole-radiation (cross-domain alias: `E2-radiation`, `Q-radiation`, `second-multipole`)
**Domain:** Physics / Diffusion
**Definition:** P = (μ₀ ⃛Q²)/(180πc⁵) using Q_ij quadrupole moment third derivative.
**Atom or composite:** Composite.
**Cost model:** O(1).
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (gravitational wave analog), nuclear physics.
**Notes:** Lowest order for gravitational radiation (no mass dipole due to momentum conservation).

### radiation-reaction (cross-domain alias: `abraham-lorentz`, `self-force`, `r-r-friction`)
**Domain:** Physics / Diffusion
**Definition:** Self-force on radiating charge: F_rr = (μ₀q²/6πc) v⃛ — Abraham-Lorentz formula.
**Atom or composite:** Composite — third time derivative.
**Cost model:** Pre-acceleration / runaway solutions complicate integration.
**Real wall?** Yes — non-causal pathologies; need LAD or LL reduction.
**Cross-domain wiring:** quantum-computing (QED resolves classical pathologies), astrophysics-cosmology (synchrotron cooling).
**Notes:** Dirac (1938), Landau-Lifshitz reduction yields well-behaved equation.

### larmor-formula (cross-domain alias: `P=q²a²/6πε₀c³`, `accelerated-charge-power`, `nonrelativistic-radiation`)
**Domain:** Physics / Diffusion
**Definition:** Nonrel. radiated power P = q²a²/(6πε₀c³); rel. version multiplies by γ^6.
**Atom or composite:** Atom — classic radiation formula.
**Cost model:** O(1).
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (cyclotron/synchrotron), electromagnetics-antennas (radiation resistance).
**Notes:** Larmor (1897). Underpins instability of Rutherford atom (resolved by QM).

### cherenkov-radiation (cross-domain alias: `cone-radiation`, `v>c/n-radiation`, `superluminal-medium`)
**Domain:** Physics / Diffusion
**Definition:** Charge moving with v > c/n in medium emits cone with cos θ = c/(nv).
**Atom or composite:** Composite — coherent shock-front radiation.
**Cost model:** Frank-Tamm spectrum: dN/dx dω ∝ sin²θ.
**Real wall?** Yes — requires v > c/n.
**Cross-domain wiring:** astrophysics-cosmology (neutrino detection), photonics-optics (water Cherenkov).
**Notes:** Cherenkov (1934), theory Frank-Tamm (1937). Used in particle detectors.

### transition-radiation (cross-domain alias: `interface-radiation`, `Ginzburg-Frank-radiation`, `n-change-radiation`)
**Domain:** Physics / Diffusion
**Definition:** Charge crossing boundary between media of different n radiates; intensity ∝ γ at relativistic speeds.
**Atom or composite:** Composite — boundary mode matching.
**Cost model:** Analytic for planar interface; numerical for arbitrary geometry.
**Real wall?** No.
**Cross-domain wiring:** photonics-optics (TR detectors), astrophysics-cosmology (cosmic rays).
**Notes:** Ginzburg-Frank (1946). γ-factor identification of particle energy.

### synchrotron-radiation (cross-domain alias: `magnetobremsstrahlung`, `B-field-radiation`, `power-law-spectrum`)
**Domain:** Physics / Diffusion
**Definition:** Relativistic charge in B-field radiates at ω_c = (3γ²eB)/(2mc); power-law spectrum.
**Atom or composite:** Composite — relativistic Larmor in B.
**Cost model:** Schott formulas; tabulated K_{1/3}, K_{2/3}.
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (pulsars, AGN jets), photonics-optics (synchrotron light sources).
**Notes:** Schwinger (1949). Underlies most cosmic non-thermal emission.

### em-duality (cross-domain alias: `E↔B-duality`, `electric-magnetic-symmetry`, `S-duality-EM`)
**Domain:** Physics / Diffusion
**Definition:** Source-free Maxwell invariant under (E, cB) → (cB, −E); broken by presence of electric charges only.
**Atom or composite:** Atom — symmetry.
**Cost model:** Symbolic.
**Real wall?** Yes — magnetic monopoles not observed.
**Cross-domain wiring:** quantum-computing (Dirac quantization), photonics-optics (electric-magnetic vectors).
**Notes:** Restored if magnetic monopoles exist. Foundation of strong-weak duality in field theory.

### aharonov-bohm-effect (cross-domain alias: `AB-phase`, `vector-potential-phase`, `topological-EM`)
**Domain:** Physics / Diffusion
**Definition:** Charged particle picks up phase exp(iq/ℏ ∮A·dl) around region of B even if B=0 on its path.
**Atom or composite:** Composite — quantum + EM gauge.
**Cost model:** Path integration of A; topological.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (geometric phases), condensed-matter (mesoscopic rings).
**Notes:** Aharonov-Bohm (1959). Proves A is physical, not just calculational tool.

---

## Optics & Wave Phenomena

### snells-law (cross-domain alias: `n₁sinθ₁=n₂sinθ₂`, `refraction-law`, `descartes-law`)
**Domain:** Physics / Diffusion
**Definition:** n₁ sin θ₁ = n₂ sin θ₂ for ray crossing interface between media with refractive indices n_i.
**Atom or composite:** Atom — phase-matching at interface.
**Cost model:** O(1) per interface; tracing N rays through M surfaces O(NM).
**Real wall?** Yes — TIR when n₂ < n₁ and sin θ₁ > n₂/n₁.
**Cross-domain wiring:** photonics-optics (lens design), electromagnetics-antennas (radome refraction).
**Notes:** Ibn Sahl (984), Snell (1621), Descartes (1637). From Fermat's principle.

### fresnel-coefficients (cross-domain alias: `r_s,r_p,t_s,t_p`, `reflection-transmission-amps`, `polarized-reflection`)
**Domain:** Physics / Diffusion
**Definition:** Amplitude reflection/transmission ratios for s and p polarizations at dielectric interface.
**Atom or composite:** Composite — 4 coefficients from boundary conditions.
**Cost model:** O(1) per interface; stack: transfer matrix O(N_layers).
**Real wall?** No.
**Cross-domain wiring:** photonics-optics (AR coatings), electromagnetics-antennas (radomes).
**Notes:** Fresnel (1823). At normal incidence: r = (n₁−n₂)/(n₁+n₂).

### brewster-angle (cross-domain alias: `θ_B`, `polarizing-angle`, `p-zero-reflection`)
**Domain:** Physics / Diffusion
**Definition:** tan θ_B = n₂/n₁ — p-polarized reflection vanishes; reflected light pure s-polarized.
**Atom or composite:** Atom — zero of r_p.
**Cost model:** O(1).
**Real wall?** No.
**Cross-domain wiring:** photonics-optics (laser windows), signal-processing-rf (polarized antennas).
**Notes:** Brewster (1815). Used in laser cavities to eliminate reflection loss for one polarization.

### total-internal-reflection (cross-domain alias: `TIR`, `θ>θ_c`, `evanescent-coupling`)
**Domain:** Physics / Diffusion
**Definition:** When n₁ > n₂ and θ_i > θ_c = arcsin(n₂/n₁), reflection coefficient |r| = 1; transmitted wave is evanescent.
**Atom or composite:** Atom — Fresnel limit.
**Cost model:** O(1).
**Real wall?** Yes — produces evanescent decay length 1/κ ~ λ.
**Cross-domain wiring:** photonics-optics (fiber optics, SPR), quantum-computing (frustrated TIR ~ tunneling).
**Notes:** Foundation of optical fiber light guiding.

### huygens-fresnel-principle (cross-domain alias: `wavelet-superposition`, `secondary-sources`, `wavefront-construction`)
**Domain:** Physics / Diffusion
**Definition:** Each point on a wavefront is a source of secondary spherical wavelets; superposition gives next wavefront.
**Atom or composite:** Composite — basis for diffraction theory.
**Cost model:** O(N²) direct summation; FFT methods O(N log N).
**Real wall?** No.
**Cross-domain wiring:** signal-processing-rf (array antenna patterns), photonics-optics (diffraction).
**Notes:** Huygens (1678), Fresnel (1818) added interference. Foundation of Kirchhoff theory.

### kirchhoff-diffraction (cross-domain alias: `Kirchhoff-integral`, `scalar-diffraction-theory`, `boundary-integral-optics`)
**Domain:** Physics / Diffusion
**Definition:** U(P) = (1/4π) ∮[U∂G/∂n − G∂U/∂n] dS over closed surface using Green's function G.
**Atom or composite:** Composite — Green's theorem applied to wave eq.
**Cost model:** O(N²) surface integration.
**Real wall?** Yes — scalar approximation breaks at λ ~ aperture.
**Cross-domain wiring:** signal-processing-rf (Stratton-Chu), photonics-optics (lens design).
**Notes:** Kirchhoff (1882). Rigorous scalar diffraction; Rayleigh-Sommerfeld improves boundary conditions.

### fresnel-diffraction (cross-domain alias: `near-field-diffraction`, `quadratic-phase-approx`, `paraxial-diffraction`)
**Domain:** Physics / Diffusion
**Definition:** Approximation valid for kz ≫ 1, paraxial: U(x,y,z) = (e^{ikz}/iλz) ∫∫ U(x',y',0) e^{ik((x−x')²+(y−y')²)/2z} dx'dy'.
**Atom or composite:** Composite — paraxial Kirchhoff.
**Cost model:** FFT-based O(N log N).
**Real wall?** Yes — paraxial only.
**Cross-domain wiring:** signal-processing-rf (chirp Z-transform), photonics-optics (free-space propagation).
**Notes:** Used in beam propagation, Fresnel zone plates.

### fraunhofer-diffraction (cross-domain alias: `far-field-diffraction`, `fourier-transform-aperture`, `kz>>D²/λ`)
**Domain:** Physics / Diffusion
**Definition:** Far-field limit of Fresnel: U(x,y,z) ∝ FT[U(x',y',0)] evaluated at (kx/z, ky/z).
**Atom or composite:** Composite — Fourier optics.
**Cost model:** Single FFT O(N log N).
**Real wall?** Yes — needs z ≫ D²/λ.
**Cross-domain wiring:** signal-processing-rf (antenna pattern = FT of aperture), photonics-optics.
**Notes:** Fraunhofer (1814-23). Foundation of Fourier optics.

### geometric-optics-limit (cross-domain alias: `ray-optics`, `λ→0-limit`, `eikonal-limit`)
**Domain:** Physics / Diffusion
**Definition:** Limit of wave optics where wavelength is negligible; rays travel along characteristics of eikonal eq.
**Atom or composite:** Composite — WKB-style asymptotic.
**Cost model:** Ray tracing O(N_rays).
**Real wall?** Yes — fails near caustics, edges, focii.
**Cross-domain wiring:** quantum-computing (semiclassical), astrophysics-cosmology (gravitational lensing).
**Notes:** Equivalent to Hamilton-Jacobi with H = c|p|/n.

### eikonal-equation (cross-domain alias: `|∇S|²=n²`, `optical-path`, `geometric-optics-PDE`)
**Domain:** Physics / Diffusion
**Definition:** |∇S(r)|² = n²(r) where S is optical phase / path length.
**Atom or composite:** Atom — first-order nonlinear PDE.
**Cost model:** Fast marching O(N log N); fast sweeping O(N).
**Real wall?** Yes — multi-valued solutions at caustics.
**Cross-domain wiring:** control-numerical-opt (HJ-eq family), photonics-optics (ray tracing).
**Notes:** From WKB ansatz ψ = A e^{ikS}; equivalent to Fermat's principle.

### ray-equation (cross-domain alias: `d/ds(n dr/ds)=∇n`, `ray-trajectory-ODE`, `geodesic-of-optics`)
**Domain:** Physics / Diffusion
**Definition:** d/ds (n dr/ds) = ∇n — ODE for ray path in graded-index medium.
**Atom or composite:** Atom — geodesic in Fermat metric.
**Cost model:** RK4 integration O(N_rays · steps).
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (atmospheric refraction), photonics-optics (GRIN lenses).
**Notes:** Reduces to straight lines in uniform n. Foundation of computational ray tracing.

### abcd-matrices (cross-domain alias: `ray-transfer-matrix`, `paraxial-matrices`, `2×2-optical-matrix`)
**Domain:** Physics / Diffusion
**Definition:** [y',θ'] = [[A,B],[C,D]] [y,θ] — paraxial transformation of ray height y and angle θ.
**Atom or composite:** Composite — composition of elementary matrices.
**Cost model:** O(N_elements) matrix multiplications.
**Real wall?** Yes — paraxial only.
**Cross-domain wiring:** linear-algebra-matrix (2×2 algebra), photonics-optics (resonator design).
**Notes:** Gaussian beams transform with same matrices via q-parameter.

### jones-matrices (cross-domain alias: `polarization-matrices`, `2×2-complex-polarization`, `coherent-polarization-calculus`)
**Domain:** Physics / Diffusion
**Definition:** Pure polarization state [E_x; E_y] transforms via 2×2 complex Jones matrix; coherent only.
**Atom or composite:** Composite — algebra of polarization elements.
**Cost model:** O(N) matrix multiplications.
**Real wall?** Yes — fails for partial polarization.
**Cross-domain wiring:** quantum-computing (qubit ↔ polarization), linear-algebra-matrix (SU(2) action).
**Notes:** Jones (1941). Maps to spinors; circular polarization = eigenstates of S_z.

### mueller-matrices (cross-domain alias: `4×4-stokes-transform`, `partial-polarization`, `incoherent-polarization`)
**Domain:** Physics / Diffusion
**Definition:** 4×4 real matrix transforming Stokes vector S = (I, Q, U, V); handles partial polarization and depolarization.
**Atom or composite:** Composite — generalizes Jones to incoherent.
**Cost model:** O(N) matrix ops.
**Real wall?** No (broader applicability than Jones).
**Cross-domain wiring:** signal-processing-rf (radar polarimetry), photonics-optics (LCD characterization).
**Notes:** Mueller (1943). Required for depolarizing scatterers; 16 vs Jones's 4 complex = 8 real.

### stokes-parameters (cross-domain alias: `S=(I,Q,U,V)`, `polarization-state-vector`, `4-real-parameters`)
**Domain:** Physics / Diffusion
**Definition:** S₀ = I, S₁ = I_H − I_V, S₂ = I_+45 − I_−45, S₃ = I_R − I_L; intensities through polarizers.
**Atom or composite:** Composite — measurable polarization vector.
**Cost model:** Six intensity measurements.
**Real wall?** Yes — only S²₀ ≥ S²₁ + S²₂ + S²₃ (DOP ≤ 1).
**Cross-domain wiring:** astrophysics-cosmology (polarized cosmological signals), signal-processing-rf (polarimetric radar).
**Notes:** Stokes (1852). Map to Poincaré sphere; pure states on surface, mixed inside.

### polarization-ellipse (cross-domain alias: `E-field-trace`, `ellipse-parameters`, `azimuth-ellipticity`)
**Domain:** Physics / Diffusion
**Definition:** Tip of E vector traces ellipse with azimuth ψ and ellipticity angle χ; parameterizes pure polarization.
**Atom or composite:** Composite — 2 parameters.
**Cost model:** O(1).
**Real wall?** No.
**Cross-domain wiring:** signal-processing-rf (antenna polarization), quantum-computing (qubit Bloch).
**Notes:** Linear: χ=0; Circular: χ=±π/4. Maps to Poincaré sphere with (2ψ,2χ).

### birefringence (cross-domain alias: `double-refraction`, `n_o-vs-n_e`, `anisotropic-optics`)
**Domain:** Physics / Diffusion
**Definition:** Anisotropic medium has different n for orthogonal polarizations; rays split into ordinary and extraordinary.
**Atom or composite:** Composite — tensor permittivity.
**Cost model:** O(1) per element; design via Δn·d = retardance.
**Real wall?** Yes — fundamental to crystal optics.
**Cross-domain wiring:** photonics-optics (waveplates), condensed-matter (liquid crystals).
**Notes:** Bartholin (1669) discovered in calcite. Used in waveplates, polarizers, LCDs.

### dispersion (cross-domain alias: `n(ω)`, `chromatic-dispersion`, `frequency-dependent-index`)
**Domain:** Physics / Diffusion
**Definition:** n = n(ω) — refractive index varies with frequency; Sellmeier equation for transparent media.
**Atom or composite:** Atom — material property.
**Cost model:** O(1) per ω; broadband simulation O(N_ω).
**Real wall?** Yes — Kramers-Kronig: anomalous dispersion near absorption.
**Cross-domain wiring:** signal-processing-rf (pulse spreading in waveguides), photonics-optics (prisms).
**Notes:** Newton (1666). Group-velocity dispersion d²k/dω² causes pulse broadening.

### group-velocity (cross-domain alias: `v_g=dω/dk`, `envelope-velocity`, `energy-velocity-typically`)
**Domain:** Physics / Diffusion
**Definition:** v_g = dω/dk — velocity of envelope of a wave packet.
**Atom or composite:** Atom — defined by dispersion relation.
**Cost model:** O(1) given ω(k).
**Real wall?** Yes — in anomalous regions v_g may exceed c (but no info travel).
**Cross-domain wiring:** signal-processing-rf (group delay), photonics-optics (slow light).
**Notes:** Hamilton (1839), Rayleigh (1877). Quantum: velocity of probability wave packet.

### phase-velocity (cross-domain alias: `v_p=ω/k`, `wavefront-velocity`, `crest-velocity`)
**Domain:** Physics / Diffusion
**Definition:** v_p = ω/k — velocity of points of constant phase.
**Atom or composite:** Atom — defined by dispersion relation.
**Cost model:** O(1).
**Real wall?** No — v_p > c is fine (carries no info).
**Cross-domain wiring:** signal-processing-rf (waveguide modes), photonics-optics (negative index materials).
**Notes:** In waveguides v_p > c always; in plasmas v_p > c above plasma frequency.

### evanescent-waves (cross-domain alias: `tunneling-EM`, `exponential-decay-wave`, `imaginary-k`)
**Domain:** Physics / Diffusion
**Definition:** Wave with imaginary wavevector component: E ∝ e^{−κz} e^{i(kx−ωt)}; carries no net energy normal to interface.
**Atom or composite:** Atom — solution past TIR or in bandgap.
**Cost model:** O(1) per mode.
**Real wall?** Yes — decay length ~λ.
**Cross-domain wiring:** quantum-computing (tunneling analogy), condensed-matter (surface plasmons).
**Notes:** Foundation of near-field optics, SNOM, prism couplers, plasmonic devices.

### goos-hanchen-shift (cross-domain alias: `lateral-shift-TIR`, `beam-shift-on-reflection`, `Δ-shift`)
**Domain:** Physics / Diffusion
**Definition:** Lateral shift of finite beam at TIR by Δ = −∂φ/∂k_∥ where φ is reflection phase.
**Atom or composite:** Composite — phase-derivative effect.
**Cost model:** Numerical for arbitrary geometry.
**Real wall?** Subwavelength magnitude.
**Cross-domain wiring:** quantum-computing (analog in tunneling), photonics-optics (SPR sensors).
**Notes:** Goos-Hänchen (1947). Imai-Federov shift is the perpendicular analog.

### optical-theorem (cross-domain alias: `forward-scattering-theorem`, `Im[f(0)]∝σ_tot`, `unitarity-scattering`)
**Domain:** Physics / Diffusion
**Definition:** σ_tot = (4π/k) Im f(0) — total cross-section from imaginary part of forward scattering amplitude.
**Atom or composite:** Atom — consequence of unitarity.
**Cost model:** O(1) given f(0).
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (S-matrix unitarity), signal-processing-rf (RCS).
**Notes:** Bohr-Peierls-Placzek (1939). Holds in QM and classical scattering.

### partial-wave-expansion (cross-domain alias: `spherical-harmonic-decomp-scattering`, `phase-shifts-δ_l`, `Legendre-expansion`)
**Domain:** Physics / Diffusion
**Definition:** f(θ) = (1/k) Σ_l (2l+1) e^{iδ_l} sin δ_l P_l(cos θ) — scattering amplitude in terms of phase shifts δ_l.
**Atom or composite:** Composite — series in l.
**Cost model:** Truncate at l_max ~ kR; O(l_max).
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (nuclear scattering), photonics-optics (Mie scattering).
**Notes:** Each l-channel is independent due to angular momentum conservation.

### scattering-amplitude (cross-domain alias: `f(θ,φ)`, `outgoing-spherical-amp`, `differential-cross-section-amp`)
**Domain:** Physics / Diffusion
**Definition:** Asymptotic wavefunction ψ → e^{ikz} + f(θ,φ) e^{ikr}/r; differential cross section dσ/dΩ = |f|².
**Atom or composite:** Atom — complex function on sphere.
**Cost model:** Born: O(N) FFT; exact: O(N³) or worse.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (Born series), signal-processing-rf (radar cross section).
**Notes:** Born approximation: f ∝ FT of potential. Foundation of scattering theory.

---

## Special Relativity

### lorentz-transform (cross-domain alias: `Λ^μ_ν`, `boost-transformation`, `inertial-frame-transform`)
**Domain:** Physics / Diffusion
**Definition:** t' = γ(t − vx/c²), x' = γ(x − vt), y'=y, z'=z with γ = 1/√(1−β²), β = v/c.
**Atom or composite:** Atom — linear coordinate transformation preserving Minkowski metric.
**Cost model:** O(1) per event.
**Real wall?** Yes — c is universal speed limit.
**Cross-domain wiring:** linear-algebra-matrix (Lorentz group SO(3,1)), electromagnetics-antennas (field transforms).
**Notes:** Lorentz (1904), Einstein (1905). Forms a group with rotations.

### four-vector (cross-domain alias: `A^μ`, `contravariant-vector`, `spacetime-4-tuple`)
**Domain:** Physics / Diffusion
**Definition:** A^μ = (A⁰, A¹, A², A³) transforms as A'^μ = Λ^μ_ν A^ν under Lorentz.
**Atom or composite:** Atom — irreducible rep of Lorentz group.
**Cost model:** O(1).
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (Dirac spinors are double-cover), linear-algebra-matrix (tensors).
**Notes:** Index placement: contravariant up, covariant down with η_μν = diag(+,−,−,−).

### four-momentum (cross-domain alias: `p^μ=(E/c,p)`, `energy-momentum-4-vector`, `relativistic-momentum`)
**Domain:** Physics / Diffusion
**Definition:** p^μ = (E/c, p) = m u^μ; p^μ p_μ = m²c² — invariant mass-shell condition.
**Atom or composite:** Atom — conserved 4-vector.
**Cost model:** O(1).
**Real wall?** Yes — m²c² ≥ 0 for physical particles.
**Cross-domain wiring:** quantum-computing (Klein-Gordon p²=m²), astrophysics-cosmology (collisions).
**Notes:** E² = (pc)² + (mc²)² — Pythagorean structure of energy-momentum.

### four-velocity (cross-domain alias: `u^μ=dx^μ/dτ`, `proper-velocity`, `tangent-to-worldline`)
**Domain:** Physics / Diffusion
**Definition:** u^μ = dx^μ/dτ = γ(c, v); u^μ u_μ = c² (normalization).
**Atom or composite:** Atom — unit timelike 4-vector.
**Cost model:** O(1).
**Real wall?** Yes — always timelike for massive particles.
**Cross-domain wiring:** astrophysics-cosmology (worldlines), control-numerical-opt (proper-time integration).
**Notes:** τ is proper time. u·a = 0 (acceleration orthogonal to velocity in 4D).

### four-acceleration (cross-domain alias: `a^μ=du^μ/dτ`, `proper-acceleration-4-vector`, `spacelike-vector`)
**Domain:** Physics / Diffusion
**Definition:** a^μ = du^μ/dτ; always spacelike, orthogonal to u^μ.
**Atom or composite:** Atom — derivative of 4-velocity.
**Cost model:** O(1).
**Real wall?** No.
**Cross-domain wiring:** electromagnetics-antennas (Larmor radiation from a^μ a_μ), astrophysics-cosmology.
**Notes:** Magnitude |a^μa_μ| = proper acceleration squared, measured by comoving accelerometer.

### proper-time (cross-domain alias: `τ`, `eigenzeit`, `clock-time`)
**Domain:** Physics / Diffusion
**Definition:** dτ² = ds²/c² = dt²(1 − v²/c²); time measured by clock comoving with particle.
**Atom or composite:** Atom — invariant.
**Cost model:** O(1) integration along worldline.
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (GPS clock corrections), quantum-computing (Dirac equation parameter).
**Notes:** Maximum proper time = inertial worldline (twin paradox: traveler ages less).

### invariant-interval (cross-domain alias: `ds²`, `spacetime-interval`, `Minkowski-norm`)
**Domain:** Physics / Diffusion
**Definition:** ds² = c²dt² − dx² − dy² − dz²; positive (timelike), zero (null), negative (spacelike).
**Atom or composite:** Atom — Minkowski quadratic form.
**Cost model:** O(1).
**Real wall?** Yes — signed signature is physical.
**Cross-domain wiring:** astrophysics-cosmology (lifts to ds² = g_μν dx^μ dx^ν in GR).
**Notes:** Frame-invariant; replaces Galilean separate time and distance.

### minkowski-metric (cross-domain alias: `η_μν`, `flat-spacetime-metric`, `signature-(+---)`)
**Domain:** Physics / Diffusion
**Definition:** η_μν = diag(+1, −1, −1, −1) [or opposite]; ds² = η_μν dx^μ dx^ν.
**Atom or composite:** Atom — geometry of flat spacetime.
**Cost model:** Trivial.
**Real wall?** Yes — defines causal structure.
**Cross-domain wiring:** quantum-computing (Dirac eq uses γ^μγ_μ = η^μμ structure), astrophysics-cosmology.
**Notes:** Symmetric, non-degenerate, signature (1,3). Lorentz group preserves it.

### light-cone (cross-domain alias: `null-cone`, `causal-cone`, `c·t=±r-cone`)
**Domain:** Physics / Diffusion
**Definition:** Set of null rays through an event; ds² = 0. Future/past cones partition spacetime causally.
**Atom or composite:** Atom — invariant under Lorentz.
**Cost model:** Topological/geometric.
**Real wall?** Yes — fundamental causal structure.
**Cross-domain wiring:** astrophysics-cosmology (Penrose diagrams), quantum-computing (microcausality).
**Notes:** Events outside light cone causally disconnected. Foundation of QFT microcausality.

### simultaneity (cross-domain alias: `frame-dependent-simul`, `t'=const-hyperplane`, `relativity-of-simul`)
**Domain:** Physics / Diffusion
**Definition:** Events simultaneous in one frame need not be in another; t' = γ(t − vx/c²).
**Atom or composite:** Atom — kinematic consequence.
**Cost model:** O(1).
**Real wall?** Yes — no absolute "now."
**Cross-domain wiring:** astrophysics-cosmology (cosmic time slicing), control-numerical-opt (clock sync).
**Notes:** Einstein (1905). Foundation of relativity; explains length contraction operationally.

### time-dilation (cross-domain alias: `γt₀`, `moving-clocks-slow`, `proper-vs-coordinate-time`)
**Domain:** Physics / Diffusion
**Definition:** Δt = γΔτ; a clock moving with v in S frame ticks slower by factor γ.
**Atom or composite:** Atom — kinematic effect.
**Cost model:** O(1).
**Real wall?** Yes — γ → ∞ as v → c.
**Cross-domain wiring:** astrophysics-cosmology (cosmic ray muons, GPS), quantum-computing (lifetime extension).
**Notes:** Confirmed by Hafele-Keating (1971), GPS (~38 μs/day correction).

### length-contraction (cross-domain alias: `L=L₀/γ`, `lorentz-contraction`, `fitzgerald-contraction`)
**Domain:** Physics / Diffusion
**Definition:** Length of object in motion along direction of motion: L = L₀/γ where L₀ is proper length.
**Atom or composite:** Atom — kinematic effect.
**Cost model:** O(1).
**Real wall?** No (kinematic only).
**Cross-domain wiring:** condensed-matter (charge density transforms), electromagnetics-antennas.
**Notes:** Fitzgerald (1889), Lorentz (1892). Resolves the ladder paradox.

### relativistic-energy-momentum (cross-domain alias: `E²=(pc)²+(mc²)²`, `mass-shell`, `relativistic-Pythagoras`)
**Domain:** Physics / Diffusion
**Definition:** E² = (pc)² + (mc²)²; E = γmc², p = γmv, |p|/E = v/c².
**Atom or composite:** Atom — invariant of p^μ.
**Cost model:** O(1).
**Real wall?** Yes — m² ≥ 0 for physical particles.
**Cross-domain wiring:** quantum-computing (Dirac, Klein-Gordon), astrophysics-cosmology (cosmic rays).
**Notes:** Einstein (1905). E = mc² as rest energy when p=0.

### relativistic-doppler (cross-domain alias: `f'=f√((1±β)/(1∓β))`, `doppler-with-time-dilation`, `transverse-doppler`)
**Domain:** Physics / Diffusion
**Definition:** Longitudinal: f' = f √((1−β)/(1+β)) for receding; transverse: f' = f/γ.
**Atom or composite:** Composite — combines classical + γ.
**Cost model:** O(1).
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (redshift), photonics-optics (laser cooling), signal-processing-rf.
**Notes:** Predicts transverse Doppler effect (purely relativistic), confirmed by Ives-Stilwell (1938).

### aberration (cross-domain alias: `angle-transformation`, `headlight-effect`, `relativistic-aberration`)
**Domain:** Physics / Diffusion
**Definition:** cos θ' = (cos θ − β)/(1 − β cos θ); photons emitted isotropically in rest frame beam forward.
**Atom or composite:** Atom — angle Lorentz transform.
**Cost model:** O(1).
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (relativistic beaming), photonics-optics (relativistic optics).
**Notes:** Bradley (1729) discovered stellar aberration. Relativistic refinement adds γ.

### relativistic-mechanics (cross-domain alias: `F=dp/dt-relativistic`, `4-force-equation`, `m-dependent-mechanics`)
**Domain:** Physics / Diffusion
**Definition:** dp/dt = F (3-form) or dp^μ/dτ = F^μ (4-form); F^μ u_μ = 0 for proper-force.
**Atom or composite:** Atom — Newton II generalized.
**Cost model:** O(1) per step.
**Real wall?** Yes — v < c maintained.
**Cross-domain wiring:** electromagnetics-antennas (Lorentz force), astrophysics-cosmology (cyclotron motion).
**Notes:** F = ma fails; correct form F = dp/dt with relativistic p.

### stress-energy-tensor (cross-domain alias: `T^μν`, `energy-momentum-tensor`, `source-of-gravity`)
**Domain:** Physics / Diffusion
**Definition:** T^μν: T⁰⁰ = energy density, T^0i = momentum density, T^ij = stress.
**Atom or composite:** Composite — symmetric rank-2 tensor.
**Cost model:** O(N_cells) field evaluation.
**Real wall?** Yes — conservation ∂_μ T^μν = 0.
**Cross-domain wiring:** astrophysics-cosmology (RHS of Einstein eq), fluid dynamics (perfect fluid form).
**Notes:** Belinfante-Rosenfeld procedure symmetrizes canonical T^μν for spin fields.

### em-field-tensor (cross-domain alias: `F_μν`, `faraday-tensor`, `electromagnetic-2-form`)
**Domain:** Physics / Diffusion
**Definition:** F_μν = ∂_μ A_ν − ∂_ν A_μ; F^{0i} = E_i/c, F^{ij} = −ε^{ijk} B_k.
**Atom or composite:** Composite — antisymmetric rank-2 tensor.
**Cost model:** O(1) per point.
**Real wall?** No.
**Cross-domain wiring:** electromagnetics-antennas (covariant Maxwell), quantum-computing (gauge field strength).
**Notes:** Covariant Maxwell: ∂_μ F^μν = μ₀ J^ν and ∂_[α F_{βγ]} = 0.

### dual-tensor (cross-domain alias: `*F_μν=½ε_μναβF^αβ`, `hodge-dual-em`, `dual-of-F`)
**Domain:** Physics / Diffusion
**Definition:** *F_μν = (1/2) ε_μναβ F^αβ — Hodge dual of F; swaps E and B (up to signs/factors of c).
**Atom or composite:** Atom — derived antisymmetric tensor.
**Cost model:** O(1) contraction.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (chiral anomaly involves F*F), electromagnetics-antennas (EM duality).
**Notes:** Bianchi identity ∂_μ *F^{μν} = 0 ↔ ∇·B = 0 and Faraday's law.

### thomas-precession (cross-domain alias: `relativistic-gyro-precession`, `Wigner-rotation`, `spin-orbit-prefactor`)
**Domain:** Physics / Diffusion
**Definition:** Spin precesses at ω_T = (γ²/(γ+1c²)) (a × v) due to non-commutativity of Lorentz boosts.
**Atom or composite:** Composite — kinematic, no force needed.
**Cost model:** O(1).
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (spin-orbit coupling factor of 2), condensed-matter (g-factor anomaly).
**Notes:** Thomas (1926) — explains factor of 1/2 in spin-orbit coupling.

### twin-paradox-formalism (cross-domain alias: `twin-paradox-resolution`, `accelerated-twin`, `worldline-proper-time`)
**Domain:** Physics / Diffusion
**Definition:** τ_traveler = ∫√(1−v²/c²) dt < τ_stationary; asymmetry due to acceleration breaking inertial frame.
**Atom or composite:** Atom — application of proper time.
**Cost model:** O(N_steps) line integral.
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (GPS), control-numerical-opt (clock comparison).
**Notes:** Resolved by recognizing only one twin remains inertial; the other accelerates.

### particle-decay-kinematics (cross-domain alias: `2-body-decay`, `M→p₁+p₂`, `invariant-mass-spectra`)
**Domain:** Physics / Diffusion
**Definition:** In rest frame of M: |p*| = √((M²−(m₁+m₂)²)(M²−(m₁−m₂)²))/(2M); decay angles isotropic for scalar.
**Atom or composite:** Composite — 4-momentum conservation.
**Cost model:** O(1) closed-form.
**Real wall?** Yes — M ≥ m₁+m₂.
**Cross-domain wiring:** quantum-computing (decay rates), astrophysics-cosmology (particle physics).
**Notes:** Lab-frame distributions via Lorentz boost of rest-frame distributions.

### mandelstam-variables (cross-domain alias: `s,t,u`, `2→2-invariants`, `scattering-invariants`)
**Domain:** Physics / Diffusion
**Definition:** s=(p₁+p₂)², t=(p₁−p₃)², u=(p₁−p₄)²; s+t+u = Σm_i².
**Atom or composite:** Composite — three Lorentz invariants from 4 momenta.
**Cost model:** O(1) per event.
**Real wall?** Yes — physical regions constrained.
**Cross-domain wiring:** quantum-computing (Feynman amplitudes), astrophysics-cosmology (cross sections).
**Notes:** Mandelstam (1958). √s = center-of-mass energy. Crossing symmetry permutes channels.

### rapidity (cross-domain alias: `ϕ=arctanh(β)`, `additive-velocity-param`, `hyperbolic-angle`)
**Domain:** Physics / Diffusion
**Definition:** ϕ = arctanh(β); rapidities add for collinear boosts: ϕ_tot = ϕ₁ + ϕ₂.
**Atom or composite:** Atom — natural boost parameter.
**Cost model:** O(1).
**Real wall?** No — ϕ ∈ (−∞, +∞).
**Cross-domain wiring:** astrophysics-cosmology (jet kinematics), quantum-computing (SU(1,1) parameter).
**Notes:** Pseudorapidity η = −ln tan(θ/2) approximates rapidity for m ≪ p (LHC convention).

### relativistic-velocity-addition (cross-domain alias: `u=(u'+v)/(1+u'v/c²)`, `velocity-composition`, `Einstein-addition`)
**Domain:** Physics / Diffusion
**Definition:** u = (u' + v)/(1 + u'v/c²) (collinear); never exceeds c if both speeds ≤ c.
**Atom or composite:** Atom — Lorentz boost composition.
**Cost model:** O(1).
**Real wall?** Yes — c is invariant under this operation.
**Cross-domain wiring:** astrophysics-cosmology (cosmological recession ≠ velocity addition naively), photonics-optics.
**Notes:** Reduces to Galilean for v ≪ c. Generalizes to 3D via Wigner rotation.

---

## General Relativity

### equivalence-principle (cross-domain alias: `EEP`, `gravity=acceleration`, `weak-strong-EP`)
**Domain:** Physics / Diffusion
**Definition:** Gravitational and inertial mass are equal; local physics in free-fall frame is special-relativistic.
**Atom or composite:** Atom — foundational postulate.
**Cost model:** Conceptual.
**Real wall?** Yes — no local experiment distinguishes uniform gravity from acceleration.
**Cross-domain wiring:** astrophysics-cosmology (foundation of GR), control-numerical-opt (inertial nav).
**Notes:** Galileo, Newton, Einstein (1907 — "happiest thought of my life"). Tested to 10^-15.

### metric-tensor (cross-domain alias: `g_μν`, `spacetime-metric`, `pseudo-riemannian-metric`)
**Domain:** Physics / Diffusion
**Definition:** Symmetric rank-2 tensor with signature (−,+,+,+) [or (+,−,−,−)]; ds² = g_μν dx^μ dx^ν.
**Atom or composite:** Atom — fundamental geometric object.
**Cost model:** 10 components in 4D; symbolic algebra heavy.
**Real wall?** Yes — must satisfy Einstein eq.
**Cross-domain wiring:** linear-algebra-matrix (4×4 symmetric), astrophysics-cosmology (cosmological metric).
**Notes:** g^μν is the inverse. Raises/lowers indices.

### christoffel-symbols (cross-domain alias: `Γ^λ_μν`, `connection-coefficients`, `levi-civita-connection`)
**Domain:** Physics / Diffusion
**Definition:** Γ^λ_μν = (1/2) g^{λσ}(∂_μ g_{σν} + ∂_ν g_{σμ} − ∂_σ g_{μν}).
**Atom or composite:** Composite — built from g and ∂g.
**Cost model:** 40 independent components in 4D; tensor algebra software needed.
**Real wall?** No.
**Cross-domain wiring:** control-numerical-opt (manifold optimization), photonics-optics (curved space ray tracing).
**Notes:** Not a tensor; transforms with inhomogeneous term. Defines parallel transport.

### geodesic-equation (cross-domain alias: `d²x^μ/dτ²+Γx'x'=0`, `free-fall-eq`, `parallel-velocity`)
**Domain:** Physics / Diffusion
**Definition:** d²x^μ/dτ² + Γ^μ_αβ (dx^α/dτ)(dx^β/dτ) = 0 — equation of free-falling particle.
**Atom or composite:** Atom — straightest path in curved spacetime.
**Cost model:** RK4 O(N_steps); needs Γ at each point.
**Real wall?** Yes — singularities can stop geodesics.
**Cross-domain wiring:** astrophysics-cosmology (orbits, lensing), photonics-optics (eikonal analog).
**Notes:** Generalization of straight lines. Variational form: extremize ∫ds.

### riemann-tensor (cross-domain alias: `R^ρ_σμν`, `curvature-tensor`, `commutator-of-covariant-derivs`)
**Domain:** Physics / Diffusion
**Definition:** R^ρ_σμν = ∂_μ Γ^ρ_νσ − ∂_ν Γ^ρ_μσ + Γ^ρ_μλ Γ^λ_νσ − Γ^ρ_νλ Γ^λ_μσ.
**Atom or composite:** Composite — built from Γ and ∂Γ.
**Cost model:** 20 independent components in 4D.
**Real wall?** Yes — characterizes curvature; vanishes iff spacetime is flat.
**Cross-domain wiring:** linear-algebra-matrix (tensor algebra), condensed-matter (Berry curvature analog).
**Notes:** Symmetries: R_(μν)(ρσ) = R_(ρσ)(μν), R_μ[νρσ] = 0, ∇_[λ R_μν]ρσ = 0 (Bianchi).

### ricci-tensor (cross-domain alias: `R_μν`, `traced-riemann`, `ricci-curvature`)
**Domain:** Physics / Diffusion
**Definition:** R_μν = R^λ_μλν — single contraction of Riemann tensor.
**Atom or composite:** Composite — trace of Riemann.
**Cost model:** O(N²) contractions per point.
**Real wall?** No.
**Cross-domain wiring:** linear-algebra-matrix (tensor traces), astrophysics-cosmology (Einstein eq).
**Notes:** Symmetric tensor; 10 independent components in 4D. Appears in Einstein eq.

### ricci-scalar (cross-domain alias: `R`, `scalar-curvature`, `traced-ricci`)
**Domain:** Physics / Diffusion
**Definition:** R = g^μν R_μν — trace of Ricci tensor; scalar curvature.
**Atom or composite:** Composite — double trace of Riemann.
**Cost model:** O(N²) per point.
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (action: S = ∫R√−g d⁴x), condensed-matter.
**Notes:** Hilbert (1915) action for GR uses R. Constant R characterizes maximally symmetric spaces.

### einstein-field-equations (cross-domain alias: `R_μν-½g_μνR+Λg_μν=8πGT_μν`, `EFE`, `gravity-from-matter`)
**Domain:** Physics / Diffusion
**Definition:** G_μν + Λg_μν = (8πG/c⁴) T_μν where G_μν = R_μν − (1/2)g_μν R.
**Atom or composite:** Atom — 10 coupled nonlinear PDEs.
**Cost model:** Numerical relativity O(N⁴) per timestep on 3D grid.
**Real wall?** Yes — singularities, no global Cauchy problem in general.
**Cross-domain wiring:** astrophysics-cosmology (cosmology, BHs), linear-algebra-matrix (tensor PDE).
**Notes:** Einstein (1915), Hilbert (1915). Hilbert derived from variational principle.

### schwarzschild-metric (cross-domain alias: `static-spherical-vacuum`, `BH-metric`, `r_s=2GM/c²`)
**Domain:** Physics / Diffusion
**Definition:** ds² = −(1−r_s/r)c²dt² + (1−r_s/r)^{-1}dr² + r²dΩ² where r_s = 2GM/c².
**Atom or composite:** Composite — vacuum spherical solution.
**Cost model:** Analytic.
**Real wall?** Yes — coordinate singularity at r=r_s; true singularity at r=0.
**Cross-domain wiring:** astrophysics-cosmology (BHs, Mercury precession), photonics-optics (lensing).
**Notes:** Schwarzschild (1916). First exact solution; Birkhoff's theorem: only spherical vacuum.

### kerr-metric (cross-domain alias: `rotating-BH-metric`, `Kerr-Newman`, `axially-symmetric-vacuum`)
**Domain:** Physics / Diffusion
**Definition:** Rotating BH metric with mass M and angular momentum J; in Boyer-Lindquist coords with Δ, Σ functions.
**Atom or composite:** Composite — vacuum rotating solution.
**Cost model:** Analytic but algebraically heavy.
**Real wall?** Yes — Cauchy horizon, ring singularity, ergosphere.
**Cross-domain wiring:** astrophysics-cosmology (astrophysical BHs), control-numerical-opt (orbit integration).
**Notes:** Kerr (1963). Frame dragging; ISCO depends on spin parameter a=J/Mc.

### reissner-nordstrom-metric (cross-domain alias: `charged-BH`, `RN-metric`, `electrovacuum-spherical`)
**Domain:** Physics / Diffusion
**Definition:** Schwarzschild generalized for charge Q: g_tt = −(1 − r_s/r + r_Q²/r²) with r_Q² = GQ²/(4πε₀c⁴).
**Atom or composite:** Composite — electrovacuum.
**Cost model:** Analytic.
**Real wall?** Yes — inner and outer horizons; naked singularity if Q²>M² (cosmic censorship).
**Cross-domain wiring:** electromagnetics-antennas (EM stress-energy as source), astrophysics-cosmology.
**Notes:** Reissner (1916), Nordström (1918). Pedagogical: simplest charged BH.

### flrw-metric (cross-domain alias: `friedmann-lemaitre-robertson-walker`, `cosmological-metric`, `homogeneous-isotropic`)
**Domain:** Physics / Diffusion
**Definition:** ds² = −dt² + a(t)²[dr²/(1−kr²) + r²dΩ²]; k = curvature parameter.
**Atom or composite:** Composite — maximally symmetric spatial slices.
**Cost model:** Cosmological evolution via Friedmann eq O(N).
**Real wall?** Yes — Big Bang singularity at a→0.
**Cross-domain wiring:** astrophysics-cosmology (cosmology backbone), statistics-probability (CMB).
**Notes:** Friedmann (1922), Lemaître (1927), Robertson-Walker (1935). Universe metric.

### friedmann-equation (cross-domain alias: `(ȧ/a)²=...`, `cosmic-expansion-eq`, `H²-equation`)
**Domain:** Physics / Diffusion
**Definition:** H² = (ȧ/a)² = (8πG/3)ρ − kc²/a² + Λ/3; second eq: ä/a = −(4πG/3)(ρ+3p/c²) + Λ/3.
**Atom or composite:** Composite — derived from EFE for FLRW.
**Cost model:** ODE in a(t).
**Real wall?** Yes — singularity at a=0.
**Cross-domain wiring:** astrophysics-cosmology (ΛCDM), statistics-probability (BBN).
**Notes:** Foundation of modern cosmology. H_0 ≈ 70 km/s/Mpc; tension between methods.

### cosmological-constant (cross-domain alias: `Λ`, `dark-energy-density`, `vacuum-energy`)
**Domain:** Physics / Diffusion
**Definition:** Constant term Λg_μν in EFE; corresponds to constant vacuum energy density ρ_Λ = Λc²/(8πG).
**Atom or composite:** Atom — extra term in EFE.
**Cost model:** O(1).
**Real wall?** Yes — 120 orders of magnitude smaller than QFT prediction.
**Cross-domain wiring:** quantum-computing (vacuum energy puzzle), astrophysics-cosmology (accelerated expansion).
**Notes:** Einstein (1917, retracted); resurrected by 1998 supernova observations.

### gravitational-waves-linearized (cross-domain alias: `h_μν`, `weak-field-GW`, `tensor-radiation`)
**Domain:** Physics / Diffusion
**Definition:** g_μν = η_μν + h_μν with |h|≪1; in TT gauge: □h̄_μν = −16πG T_μν/c⁴.
**Atom or composite:** Composite — linearization of EFE.
**Cost model:** FFT/template matching O(N log N) for detection.
**Real wall?** Yes — propagates at c; quadrupole radiation only.
**Cross-domain wiring:** electromagnetics-antennas (analog to EM), signal-processing-rf (matched filtering).
**Notes:** Detected by LIGO (2015, GW150914). Two polarizations + and ×.

### gw-polarizations (cross-domain alias: `h_+,h_×`, `plus-cross-modes`, `transverse-traceless-modes`)
**Domain:** Physics / Diffusion
**Definition:** Two physical polarization modes h_+ (stretch x, compress y) and h_× (45° rotated); spin-2 graviton.
**Atom or composite:** Atom — TT-gauge polarization basis.
**Cost model:** O(1).
**Real wall?** Yes — only 2 modes due to gauge.
**Cross-domain wiring:** signal-processing-rf (polarized signal), astrophysics-cosmology (BBO).
**Notes:** Massless spin-2 ⇒ 2 helicity states ±2. Distinguishes from scalar/vector alternatives.

### event-horizon (cross-domain alias: `r=r_s`, `point-of-no-return`, `null-surface`)
**Domain:** Physics / Diffusion
**Definition:** Null surface beyond which no causal curves escape; for Schwarzschild at r = 2GM/c².
**Atom or composite:** Atom — global causal structure.
**Cost model:** Determined by global metric; numerical relativity computes via apparent horizons.
**Real wall?** Yes — fundamental.
**Cross-domain wiring:** statistics-probability (BH entropy A/4), quantum-computing (Hawking radiation).
**Notes:** Schwarzschild coordinate singularity is regular; true singularity is at r=0.

### ergosphere (cross-domain alias: `static-limit-region`, `frame-dragging-zone`, `r<r_static`)
**Domain:** Physics / Diffusion
**Definition:** Region outside event horizon of rotating BH where g_tt > 0; static observers impossible; energy extraction possible.
**Atom or composite:** Composite — feature of Kerr metric.
**Cost model:** Analytic from Kerr.
**Real wall?** Yes — rotation is bounded by a ≤ M.
**Cross-domain wiring:** astrophysics-cosmology (BH jets, Penrose), control-numerical-opt (orbit stability).
**Notes:** Negative-energy orbits possible — Penrose process extracts rotational energy.

### penrose-process (cross-domain alias: `BH-energy-extraction`, `negative-energy-orbit`, `superradiance-analog`)
**Domain:** Physics / Diffusion
**Definition:** Particle splits in ergosphere; one falls in with negative E, other escapes with E > original.
**Atom or composite:** Composite — process within ergosphere.
**Cost model:** Geodesic integration.
**Real wall?** Yes — bounded by irreducible mass.
**Cross-domain wiring:** astrophysics-cosmology (AGN jets, Blandford-Znajek), quantum-computing (superradiance).
**Notes:** Penrose (1969). Up to ~29% of M can be extracted from maximal Kerr.

### geodetic-precession (cross-domain alias: `de-sitter-precession`, `gyroscope-precession`, `space-curvature-spin-rotation`)
**Domain:** Physics / Diffusion
**Definition:** Gyroscope in orbit precesses at ω_geo = (3GM/(2c²r²))v due to space curvature.
**Atom or composite:** Composite — kinematic GR effect.
**Cost model:** O(1) per orbit.
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (Gravity Probe B), quantum-computing (Berry phase analog).
**Notes:** De Sitter (1916). Confirmed by Gravity Probe B to ~1% (6.6 arcsec/yr).

### frame-dragging (cross-domain alias: `lense-thirring`, `gravitomagnetism`, `rotating-mass-drag`)
**Domain:** Physics / Diffusion
**Definition:** Rotating mass drags local inertial frames; in weak field ω = (G/c²)(J − 3(J·r̂)r̂)/r³.
**Atom or composite:** Composite — gravitomagnetic effect.
**Cost model:** O(1) per point.
**Real wall?** No.
**Cross-domain wiring:** electromagnetics-antennas (analog to B-field), astrophysics-cosmology.
**Notes:** Lense-Thirring (1918). Measured by Gravity Probe B (~0.039 arcsec/yr).

### gravitational-lensing (cross-domain alias: `light-bending`, `einstein-ring`, `microlensing`)
**Domain:** Physics / Diffusion
**Definition:** Light deflected by mass; weak field: α = 4GM/(c²b) for impact parameter b. Einstein ring when source-lens-observer collinear.
**Atom or composite:** Composite — geodesic of light.
**Cost model:** Ray tracing through metric.
**Real wall?** No.
**Cross-domain wiring:** photonics-optics (lens analogy), astrophysics-cosmology (dark matter mapping).
**Notes:** Einstein (1915), Eddington (1919). Strong lensing produces multiple images.

### komar-mass (cross-domain alias: `stationary-mass-integral`, `killing-vector-mass`, `surface-integral-mass`)
**Domain:** Physics / Diffusion
**Definition:** M_K = (1/8πG) ∮ ∇^μ k^ν dS_{μν} where k^μ is timelike Killing vector; surface integral over closed 2-surface.
**Atom or composite:** Composite — integral over surface.
**Cost model:** Surface integration.
**Real wall?** Yes — requires stationarity.
**Cross-domain wiring:** astrophysics-cosmology (BH mass), linear-algebra-matrix (tensor algebra).
**Notes:** Komar (1959). For Schwarzschild M_K = M_∞; agrees with ADM in asymptotically flat case.

### adm-mass (cross-domain alias: `asymptotic-mass`, `ADM-energy`, `bondi-arnowitt-deser-misner`)
**Domain:** Physics / Diffusion
**Definition:** Total energy measured at spatial infinity in asymptotically flat spacetime; computed from metric falloff.
**Atom or composite:** Composite — surface integral at infinity.
**Cost model:** Asymptotic analysis.
**Real wall?** Yes — only meaningful asymptotically flat.
**Cross-domain wiring:** astrophysics-cosmology (binary BH inspiral masses), control-numerical-opt (numerical GR).
**Notes:** Arnowitt-Deser-Misner (1962). Distinguished from Bondi mass (at null infinity).

### penrose-diagram (cross-domain alias: `conformal-diagram`, `compactified-spacetime`, `causal-diagram`)
**Domain:** Physics / Diffusion
**Definition:** Conformally compactified spacetime diagram with light cones at ±45°; reveals causal structure globally.
**Atom or composite:** Composite — diffeomorphism + conformal rescaling.
**Cost model:** Symbolic; visualizes topology.
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (BH topology), quantum-computing (Hawking radiation derivation).
**Notes:** Penrose (1963). Brings infinity to finite location; reveals horizons, singularities, regions.

---

## Wave Equations & Nonlinear Waves

### wave-equation-1d (cross-domain alias: `∂²u/∂t²=c²∂²u/∂x²`, `1d-d-alembertian`, `string-equation`)
**Domain:** Physics / Diffusion
**Definition:** ∂²u/∂t² = c² ∂²u/∂x² — hyperbolic PDE; general solution u = f(x−ct) + g(x+ct).
**Atom or composite:** Atom — canonical hyperbolic PDE.
**Cost model:** O(N) per step explicit; CFL c·Δt ≤ Δx.
**Real wall?** Yes — CFL.
**Cross-domain wiring:** signal-processing-rf (transmission lines), photonics-optics (1D waves).
**Notes:** d'Alembert (1747). Foundation of wave theory.

### wave-equation-3d (cross-domain alias: `□u=0`, `3d-d-alembertian`, `vacuum-wave-eq`)
**Domain:** Physics / Diffusion
**Definition:** ∂²u/∂t² = c²∇²u; spherical solutions u(r,t) = f(t−r/c)/r + g(t+r/c)/r.
**Atom or composite:** Atom — relativistic wave eq.
**Cost model:** FDTD O(N³) per step on 3D grid.
**Real wall?** Yes — CFL.
**Cross-domain wiring:** electromagnetics-antennas (EM in vacuum), astrophysics-cosmology (gravitational waves linearized).
**Notes:** Sharp Huygens' principle holds in odd spatial dim ≥ 3.

### dalembert-solution (cross-domain alias: `f(x-ct)+g(x+ct)`, `1d-wave-general-soln`, `traveling-wave-decomp`)
**Domain:** Physics / Diffusion
**Definition:** u(x,t) = (1/2)[u₀(x−ct) + u₀(x+ct)] + (1/2c)∫_{x−ct}^{x+ct} v₀(s) ds for ICs (u₀, v₀).
**Atom or composite:** Composite — superposition of left/right movers.
**Cost model:** O(1) per evaluation.
**Real wall?** No.
**Cross-domain wiring:** signal-processing-rf (forward/backward decomp), photonics-optics (mode decomposition).
**Notes:** d'Alembert (1747). Domain of dependence: interval [x−ct, x+ct].

### waves-green-function (cross-domain alias: `retarded-green`, `G_R`, `wave-eq-propagator`)
**Domain:** Physics / Diffusion
**Definition:** □G_R = δ⁴(x−x'); in 3D G_R = δ(t−|r−r'|/c)/(4π|r−r'|).
**Atom or composite:** Composite — fundamental solution.
**Cost model:** Direct convolution O(N²); FFT O(N log N).
**Real wall?** Yes — causality enforces retardation.
**Cross-domain wiring:** electromagnetics-antennas (LW potentials), photonics-optics (Kirchhoff).
**Notes:** Advanced solution G_A by t → −t. Used to build retarded potentials.

### helmholtz-equation (cross-domain alias: `∇²u+k²u=0`, `time-harmonic-wave`, `frequency-domain-wave`)
**Domain:** Physics / Diffusion
**Definition:** ∇²u + k²u = 0 — time-harmonic reduction of wave eq with u(x,t) = u(x)e^{-iωt}.
**Atom or composite:** Atom — elliptic PDE.
**Cost model:** Indefinite linear system O(N^1.5–N^3); preconditioning hard.
**Real wall?** Yes — high frequency ill-conditioning ("pollution effect").
**Cross-domain wiring:** signal-processing-rf (waveguides), photonics-optics (scattering), electromagnetics-antennas.
**Notes:** Boundary integral methods (BEM) common. Sommerfeld radiation condition for outgoing solutions.

### paraxial-wave-equation (cross-domain alias: `∂A/∂z=(i/2k)∇_T²A`, `slowly-varying-envelope`, `schrodinger-like-optics`)
**Domain:** Physics / Diffusion
**Definition:** For slowly-varying envelope A: 2ik ∂A/∂z + ∇_T²A = 0; same form as 2D Schrödinger.
**Atom or composite:** Composite — approximation of Helmholtz.
**Cost model:** Split-step Fourier O(N log N) per step.
**Real wall?** Yes — fails for large angles >~10°.
**Cross-domain wiring:** quantum-computing (formal Schrödinger analogy), photonics-optics (beam propagation).
**Notes:** Underlies Gaussian beam optics, fiber simulations.

### wave-packet (cross-domain alias: `localized-wave`, `gaussian-packet`, `superposition-of-modes`)
**Domain:** Physics / Diffusion
**Definition:** ψ(x,t) = ∫A(k) e^{i(kx−ω(k)t)} dk — superposition of plane waves localized in space.
**Atom or composite:** Composite — Fourier superposition.
**Cost model:** FFT O(N log N).
**Real wall?** Yes — uncertainty Δx Δk ≥ 1/2.
**Cross-domain wiring:** quantum-computing (Heisenberg), signal-processing-rf (pulse), photonics-optics.
**Notes:** Group velocity = dω/dk gives packet motion; dispersion causes spreading.

### group-vs-phase-velocity (cross-domain alias: `v_g≠v_p`, `dispersion-pair`, `envelope-vs-crest`)
**Domain:** Physics / Diffusion
**Definition:** v_p = ω/k; v_g = dω/dk. Identical only in non-dispersive media (ω = ck).
**Atom or composite:** Composite — pair of derived quantities.
**Cost model:** O(1).
**Real wall?** No.
**Cross-domain wiring:** signal-processing-rf (group delay), photonics-optics (dispersion management).
**Notes:** v_g typically carries energy; v_p > c possible in anomalous regions without info violation.

### dispersion-relation (cross-domain alias: `ω(k)`, `mode-relation`, `band-structure-1D`)
**Domain:** Physics / Diffusion
**Definition:** Relation ω = ω(k) between frequency and wavevector for linear modes; from substituting plane wave.
**Atom or composite:** Atom — characteristic of medium.
**Cost model:** Analytic for linear PDE; eigenvalue problem for inhomogeneous.
**Real wall?** Yes — determines whether waves propagate (real k) or decay (imag k).
**Cross-domain wiring:** condensed-matter (phonons, electrons), photonics-optics (photonic crystals).
**Notes:** Universal description of linear wave propagation; nonlinear generalizations via NLS.

### kdv-soliton (cross-domain alias: `sech²-soliton`, `korteweg-de-vries-eq`, `shallow-water-soliton`)
**Domain:** Physics / Diffusion
**Definition:** KdV: ∂u/∂t + 6u ∂u/∂x + ∂³u/∂x³ = 0. Soliton: u(x,t) = (c/2) sech²(√c (x−ct)/2).
**Atom or composite:** Composite — exact nonlinear traveling wave.
**Cost model:** Inverse scattering exact; pseudospectral O(N log N).
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (Lax pair structure), control-numerical-opt (integrable systems).
**Notes:** Russell (1834), Korteweg-de Vries (1895), Zabusky-Kruskal (1965) — birth of soliton concept.

### nls-soliton (cross-domain alias: `nonlinear-schrodinger-soliton`, `bright-soliton`, `cubic-NLS`)
**Domain:** Physics / Diffusion
**Definition:** iψ_t + (1/2)ψ_xx + |ψ|²ψ = 0; bright soliton: ψ = η sech(η(x−vt)) e^{i(vx−(v²−η²)t/2)}.
**Atom or composite:** Composite — exact via IST.
**Cost model:** Split-step Fourier O(N log N) per step.
**Real wall?** No.
**Cross-domain wiring:** photonics-optics (fiber solitons), condensed-matter (BEC bright solitons).
**Notes:** Zakharov-Shabat (1972) showed integrability. Mollenauer demonstrated fiber transmission.

### breather (cross-domain alias: `oscillating-soliton`, `breathing-soliton`, `localized-oscillation`)
**Domain:** Physics / Diffusion
**Definition:** Localized nonlinear solution oscillating periodically in time; e.g., sine-Gordon breather, NLS Akhmediev breather.
**Atom or composite:** Composite — bound state of two solitons.
**Cost model:** Analytic or pseudospectral.
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (Josephson junctions), photonics-optics (rogue wave precursors).
**Notes:** Foundation of Peregrine rogue-wave model; observed in optical fibers.

### dark-soliton (cross-domain alias: `defocusing-NLS-soliton`, `intensity-dip`, `phase-jump-soliton`)
**Domain:** Physics / Diffusion
**Definition:** Defocusing NLS iψ_t + ψ_xx/2 − |ψ|²ψ = 0; ψ = ψ_0 tanh(ψ_0 x) is stationary dark soliton.
**Atom or composite:** Composite — phase-discontinuity solution.
**Cost model:** Pseudospectral.
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (BEC dark solitons), photonics-optics (defocusing fibers).
**Notes:** Dip in background intensity; carries π phase shift across center.

### sine-gordon-equation (cross-domain alias: `□u+sinu=0`, `nonlinear-klein-gordon-sine`, `josephson-eq`)
**Domain:** Physics / Diffusion
**Definition:** ∂²u/∂t² − ∂²u/∂x² + sin u = 0; kink: u = 4 arctan(exp((x−vt)/√(1−v²))).
**Atom or composite:** Composite — integrable nonlinear PDE.
**Cost model:** IST exact; pseudospectral O(N log N).
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (Josephson junctions), quantum-computing (Mass-equivalent excitations).
**Notes:** Topological soliton (kink/antikink); has breathers and multi-soliton solutions.

### davey-stewartson-equation (cross-domain alias: `2D-NLS-generalization`, `DS-system`, `coupled-NLS-mean-field`)
**Domain:** Physics / Diffusion
**Definition:** Coupled 2D PDE: iψ_t + ψ_{xx} ± ψ_{yy} + |ψ|²ψ = ψφ; ∇²φ = ∂²|ψ|²/∂x².
**Atom or composite:** Composite — 2D integrable extension.
**Cost model:** Pseudospectral O(N² log N).
**Real wall?** No.
**Cross-domain wiring:** photonics-optics (2D beam propagation), condensed-matter (water waves).
**Notes:** Davey-Stewartson (1974). Two types: DSI (elliptic-hyperbolic), DSII (elliptic-elliptic).

### shock-wave (cross-domain alias: `discontinuous-solution`, `nonlinear-wave-steepening`, `entropy-condition`)
**Domain:** Physics / Diffusion
**Definition:** Discontinuous solution of conservation laws; jump from breakdown of smoothness via nonlinear steepening.
**Atom or composite:** Composite — weak solution of hyperbolic PDE.
**Cost model:** Godunov, Lax-Friedrichs schemes; needs limiters for non-oscillation.
**Real wall?** Yes — shock formation finite-time.
**Cross-domain wiring:** astrophysics-cosmology (SNR shocks), control-numerical-opt (TVD schemes).
**Notes:** Solved with Rankine-Hugoniot + entropy/Lax conditions.

### riemann-problem (cross-domain alias: `IVP-with-two-constant-states`, `riemann-solver`, `1D-hyperbolic-IVP`)
**Domain:** Physics / Diffusion
**Definition:** IC u(x,0) = u_L for x<0, u_R for x>0; solution self-similar in x/t for hyperbolic systems.
**Atom or composite:** Composite — canonical IVP.
**Cost model:** Analytic for linear; iterative for Euler (Roe, HLL, HLLC).
**Real wall?** Yes — discontinuity propagation.
**Cross-domain wiring:** control-numerical-opt (Godunov method), astrophysics-cosmology (SN shock breakout).
**Notes:** Riemann (1860). Building block of finite-volume schemes via Godunov approach.

### method-of-characteristics (cross-domain alias: `MOC`, `characteristic-curves`, `transport-along-rays`)
**Domain:** Physics / Diffusion
**Definition:** Reduce first-order PDE to ODEs along characteristics dx/dt = c(u); solution constant along them until they cross.
**Atom or composite:** Composite — solution technique.
**Cost model:** O(N_char) for N characteristics.
**Real wall?** Yes — characteristics can cross ⇒ shocks.
**Cross-domain wiring:** photonics-optics (eikonal), control-numerical-opt (Hamilton-Jacobi).
**Notes:** Foundation for understanding nonlinear hyperbolic PDE.

### rankine-hugoniot (cross-domain alias: `jump-conditions`, `shock-relations`, `[F]=s[U]`)
**Domain:** Physics / Diffusion
**Definition:** Across discontinuity moving at speed s: s·[ρ] = [ρu]; s·[ρu] = [ρu²+p]; energy similarly.
**Atom or composite:** Composite — algebraic from conservation.
**Cost model:** Solve nonlinear system at interface.
**Real wall?** Yes — entropy condition selects physical shocks.
**Cross-domain wiring:** control-numerical-opt (FV solvers), astrophysics-cosmology (SN shocks).
**Notes:** Rankine (1870), Hugoniot (1887). Connect upstream/downstream of shock.

### weak-vs-strong-shocks (cross-domain alias: `M~1-vs-M>>1`, `weak-strong-shock-classification`, `mach-number-regimes`)
**Domain:** Physics / Diffusion
**Definition:** Weak: M-1 ≪ 1, density jump small; strong: M ≫ 1, density jump → (γ+1)/(γ−1) for ideal gas.
**Atom or composite:** Composite — limiting regimes.
**Cost model:** O(1) per shock.
**Real wall?** Yes — strong limit is asymptotic.
**Cross-domain wiring:** astrophysics-cosmology (SN remnants), control-numerical-opt (Euler solvers).
**Notes:** Strong-shock limit gives compression ratio 4 for γ=5/3; useful in cosmic-ray acceleration.

### dispersive-shock (cross-domain alias: `Whitham-shock`, `oscillatory-shock`, `KdV-shock`)
**Domain:** Physics / Diffusion
**Definition:** Trains of nonlinear oscillations replacing classical shocks in dispersive systems (e.g., KdV).
**Atom or composite:** Composite — modulation theory.
**Cost model:** Whitham-Gurevich-Pitaevskii equations.
**Real wall?** No (dispersion regularizes).
**Cross-domain wiring:** photonics-optics (fiber DSWs), condensed-matter (BEC).
**Notes:** Gurevich-Pitaevskii (1974). Foundation: modulated multiphase solutions.

### rogue-waves (cross-domain alias: `freak-waves`, `peregrine-soliton`, `extreme-events-waves`)
**Domain:** Physics / Diffusion
**Definition:** Rare large-amplitude waves; Peregrine soliton ψ = e^{2it}(1 − 4(1+4it)/(1+4x²+16t²)).
**Atom or composite:** Composite — rational NLS solution.
**Cost model:** Analytic; pseudospectral verification.
**Real wall?** No.
**Cross-domain wiring:** photonics-optics (optical rogue waves), statistics-probability (heavy-tail).
**Notes:** Peregrine (1983), observed in optical fibers (Solli 2007). Modulationally unstable origin.

### modulational-instability (cross-domain alias: `Benjamin-Feir`, `MI`, `sideband-growth`)
**Domain:** Physics / Diffusion
**Definition:** Plane wave background unstable to small modulations in focusing NLS; sidebands grow exponentially.
**Atom or composite:** Composite — linear stability of NLS background.
**Cost model:** Eigenvalue analysis O(1).
**Real wall?** Yes — sets pulse breakup threshold.
**Cross-domain wiring:** photonics-optics (supercontinuum generation), condensed-matter (BEC collapse).
**Notes:** Benjamin-Feir (1967). Maximum growth at Ω_max = √2 |ψ₀|; mechanism for rogue waves.

### solitary-wave-stability (cross-domain alias: `vakhitov-kolokolov`, `VK-criterion`, `soliton-stability`)
**Domain:** Physics / Diffusion
**Definition:** Soliton stable if dN/dλ > 0 where N = ∫|ψ|² and λ is propagation constant (VK criterion).
**Atom or composite:** Composite — derived stability criterion.
**Cost model:** Compute N(λ); often analytic.
**Real wall?** Yes — determines existence of stable solitons.
**Cross-domain wiring:** photonics-optics (fiber soliton design), condensed-matter (BEC).
**Notes:** Vakhitov-Kolokolov (1973). Foundational stability theorem for NLS-like equations.

### inverse-scattering-transform (cross-domain alias: `IST`, `nonlinear-fourier`, `lax-pair-method`)
**Domain:** Physics / Diffusion
**Definition:** Maps integrable PDE to linear scattering problem; evolve scattering data; reconstruct via Gelfand-Levitan-Marchenko.
**Atom or composite:** Composite — method for integrable PDEs.
**Cost model:** Scattering problem O(N²) for N-soliton.
**Real wall?** Yes — only works for integrable systems.
**Cross-domain wiring:** quantum-computing (integrable spin chains), signal-processing-rf (nonlinear Fourier transform).
**Notes:** GGKM (1967) for KdV. Foundation of soliton theory.

---

## Fluid Dynamics

### navier-stokes-equation (cross-domain alias: `NS`, `viscous-momentum-eq`, `incompressible-NS`)
**Domain:** Physics / Diffusion
**Definition:** ρ(∂u/∂t + u·∇u) = −∇p + μ∇²u + f; ∇·u = 0 (incompressible).
**Atom or composite:** Composite — momentum + continuity.
**Cost model:** Pressure Poisson O(N log N); explicit advection O(N).
**Real wall?** Yes — existence/uniqueness in 3D unproven (Clay Millennium Problem).
**Cross-domain wiring:** control-numerical-opt (DNS/LES), astrophysics-cosmology (cosmic fluid).
**Notes:** Navier (1822), Stokes (1845). Turbulence is unresolved analytically.

### euler-equations-inviscid (cross-domain alias: `inviscid-NS`, `μ=0-flow`, `compressible-euler`)
**Domain:** Physics / Diffusion
**Definition:** ∂ρ/∂t + ∇·(ρu) = 0; ∂(ρu)/∂t + ∇·(ρuu+pI) = 0; energy eq for compressible.
**Atom or composite:** Composite — Euler system.
**Cost model:** Godunov / WENO finite-volume O(N).
**Real wall?** Yes — shocks form even from smooth data.
**Cross-domain wiring:** astrophysics-cosmology (gasdynamics), control-numerical-opt (shock capturing).
**Notes:** Euler (1757). Hyperbolic system; foundation of compressible flow.

### reynolds-number (cross-domain alias: `Re=UL/ν`, `inertial-vs-viscous`, `re`)
**Domain:** Physics / Diffusion
**Definition:** Re = UL/ν — ratio of inertial to viscous forces; transition to turbulence ~Re_c.
**Atom or composite:** Atom — dimensionless group.
**Cost model:** O(1).
**Real wall?** Yes — high-Re flows are turbulent and unresolvable by DNS in many cases.
**Cross-domain wiring:** condensed-matter (transport), control-numerical-opt (resolution scaling).
**Notes:** Reynolds (1883). Re_c ~ 2300 for pipe flow.

### mach-number (cross-domain alias: `Ma=u/c_s`, `compressibility-parameter`, `sound-speed-ratio`)
**Domain:** Physics / Diffusion
**Definition:** Ma = u/c_s where c_s = sound speed; Ma<1 subsonic, =1 transonic, >1 supersonic.
**Atom or composite:** Atom — dimensionless number.
**Cost model:** O(1).
**Real wall?** Yes — shock formation at Ma>1.
**Cross-domain wiring:** astrophysics-cosmology (cosmic shocks), control-numerical-opt (compressible flow).
**Notes:** Mach (1887). Compressibility effects significant at Ma > 0.3.

### froude-number (cross-domain alias: `Fr=u/√(gL)`, `gravity-wave-ratio`, `surface-wave-number`)
**Domain:** Physics / Diffusion
**Definition:** Fr = u/√(gL) — ratio of inertia to gravity; analog of Mach for surface waves.
**Atom or composite:** Atom — dimensionless number.
**Cost model:** O(1).
**Real wall?** Yes — Fr=1 critical flow; hydraulic jumps at Fr>1.
**Cross-domain wiring:** astrophysics-cosmology (atmospheric flows), control-numerical-opt (ship hydrodynamics).
**Notes:** Froude (1870). Determines ship wave-making resistance.

### vorticity-equation (cross-domain alias: `Dω/Dt=(ω·∇)u+ν∇²ω`, `curl-of-NS`, `vorticity-transport`)
**Domain:** Physics / Diffusion
**Definition:** Dω/Dt = (ω·∇)u + ν∇²ω + (1/ρ²)∇ρ×∇p; pressure absent in incompressible barotropic flow.
**Atom or composite:** Composite — curl of NS.
**Cost model:** Same as NS.
**Real wall?** Yes — vortex stretching is heart of 3D turbulence.
**Cross-domain wiring:** astrophysics-cosmology (cosmic vorticity), control-numerical-opt (vortex methods).
**Notes:** Helmholtz (1858). 2D: ω is scalar conserved along streamlines (barotropic).

### helmholtz-decomposition (cross-domain alias: `solenoidal+irrotational`, `hodge-decomp-3D`, `vector-field-split`)
**Domain:** Physics / Diffusion
**Definition:** Any smooth vector field decomposes u = ∇φ + ∇×A with ∇·A=0 (gauge); unique with appropriate decay.
**Atom or composite:** Composite — vector calculus decomposition.
**Cost model:** Two Poisson solves O(N log N).
**Real wall?** No.
**Cross-domain wiring:** electromagnetics-antennas (E from φ + A), linear-algebra-matrix (orthogonal subspaces).
**Notes:** Helmholtz (1858). Foundation of projection methods in incompressible NS.

### stream-function (cross-domain alias: `ψ`, `2D-incompressible-potential`, `flow-line-function`)
**Domain:** Physics / Diffusion
**Definition:** In 2D incompressible: u = ∂ψ/∂y, v = −∂ψ/∂x; level sets = streamlines.
**Atom or composite:** Atom — scalar potential for 2D flow.
**Cost model:** Poisson solve O(N log N) when ω=−∇²ψ.
**Real wall?** Yes — restricted to 2D / axisymmetric.
**Cross-domain wiring:** photonics-optics (analog: phase contours), control-numerical-opt (vortex methods).
**Notes:** Lagrange (1781). 3D generalization: vector potential A.

### velocity-potential (cross-domain alias: `φ`, `irrotational-flow-potential`, `Laplacian-flow`)
**Domain:** Physics / Diffusion
**Definition:** For irrotational flow u = ∇φ; incompressible adds ∇²φ = 0 (Laplace).
**Atom or composite:** Atom — scalar potential for ω=0.
**Cost model:** Laplace solver O(N log N); BIE methods.
**Real wall?** Yes — valid only for irrotational flow.
**Cross-domain wiring:** electromagnetics-antennas (electrostatic analog), photonics-optics (paraxial).
**Notes:** Euler (1755). Foundation of classical airfoil theory.

### bernoulli-equation (cross-domain alias: `p+ρgh+ρu²/2=const`, `streamline-energy-conservation`, `bernoulli-principle`)
**Domain:** Physics / Diffusion
**Definition:** Along streamline (steady, incompressible, inviscid): p + ρgh + (1/2)ρu² = const.
**Atom or composite:** Atom — energy conservation.
**Cost model:** O(1).
**Real wall?** Yes — requires inviscid steady flow.
**Cross-domain wiring:** control-numerical-opt (Pitot tube), astrophysics-cosmology (wind dynamics).
**Notes:** Bernoulli (1738). Lift on airfoil via difference in u over/under wing.

### kelvin-circulation-theorem (cross-domain alias: `dΓ/dt=0`, `circulation-conservation`, `barotropic-circulation`)
**Domain:** Physics / Diffusion
**Definition:** dΓ/dt = 0 for material loop in inviscid barotropic flow; Γ = ∮u·dl.
**Atom or composite:** Atom — conservation theorem.
**Cost model:** O(N_loop).
**Real wall?** Yes — fails with viscosity, baroclinity, or non-conservative forces.
**Cross-domain wiring:** astrophysics-cosmology (galactic rotation), control-numerical-opt (vortex methods).
**Notes:** Kelvin (1869). Underlies persistence of vortices in ideal fluid.

### kelvin-helmholtz-instability (cross-domain alias: `shear-instability`, `KH-billows`, `velocity-shear-instability`)
**Domain:** Physics / Diffusion
**Definition:** Instability of velocity-shear layer; growth rate σ = k|Δu|/2 for vortex sheet.
**Atom or composite:** Composite — linear stability eigenvalue.
**Cost model:** Orr-Sommerfeld eigenproblem O(N³).
**Real wall?** Yes — leads to roll-up and mixing.
**Cross-domain wiring:** astrophysics-cosmology (jets), photonics-optics (analogy in nonlinear optics).
**Notes:** Helmholtz (1868), Kelvin (1871). Forms KH billows in clouds.

### rayleigh-taylor-instability (cross-domain alias: `RT`, `density-stratification-instability`, `heavier-on-top`)
**Domain:** Physics / Diffusion
**Definition:** Heavy fluid over light fluid in gravity; growth rate σ = √(Atkg) where A = (ρ₁−ρ₂)/(ρ₁+ρ₂).
**Atom or composite:** Composite — buoyancy-driven instability.
**Cost model:** Linear: analytic; nonlinear: DNS.
**Real wall?** Yes — leads to turbulent mixing.
**Cross-domain wiring:** astrophysics-cosmology (SN ejecta), condensed-matter (ICF implosions).
**Notes:** Rayleigh (1883), Taylor (1950). Key in SNRs, ICF, atmospheric convection.

### boundary-layer-prandtl (cross-domain alias: `prandtl-BL`, `viscous-layer`, `δ~√(νx/U)`)
**Domain:** Physics / Diffusion
**Definition:** Thin viscous layer near wall: δ ~ √(νx/U); equations reduce via Prandtl scaling.
**Atom or composite:** Composite — asymptotic regime.
**Cost model:** BL eqs cheaper than full NS; parabolic in x.
**Real wall?** Yes — separation at adverse pressure gradient.
**Cross-domain wiring:** control-numerical-opt (aerodynamic design), condensed-matter (transport BL).
**Notes:** Prandtl (1904) — revolutionized aerodynamics.

### blasius-solution (cross-domain alias: `flat-plate-BL`, `similarity-soln-BL`, `f'''+ff''/2=0`)
**Domain:** Physics / Diffusion
**Definition:** Similarity ODE f''' + (1/2)ff'' = 0 with f(0)=f'(0)=0, f'(∞)=1 for flat-plate BL.
**Atom or composite:** Composite — similarity reduction.
**Cost model:** Shooting method O(N).
**Real wall?** No.
**Cross-domain wiring:** control-numerical-opt (BL solvers), photonics-optics (analog Tollmien-Schlichting).
**Notes:** Blasius (1908). Skin friction C_f = 0.664/√Re_x.

### kolmogorov-cascade (cross-domain alias: `energy-cascade`, `inertial-range`, `forward-cascade`)
**Domain:** Physics / Diffusion
**Definition:** Energy cascades from large eddies to dissipative scales η = (ν³/ε)^{1/4}; energy flux ε is constant in inertial range.
**Atom or composite:** Composite — turbulence theory.
**Cost model:** DNS resolves up to η: O(Re^{9/4}) grid points.
**Real wall?** Yes — DNS cost grows quickly with Re.
**Cross-domain wiring:** astrophysics-cosmology (cosmic turbulence), signal-processing-rf (1/f-like spectra).
**Notes:** Kolmogorov (1941, K41). Foundation of statistical theory of turbulence.

### five-thirds-law (cross-domain alias: `E(k)~ε^{2/3}k^{-5/3}`, `K41-spectrum`, `inertial-spectrum`)
**Domain:** Physics / Diffusion
**Definition:** Energy spectrum E(k) = C_K ε^{2/3} k^{-5/3} in inertial range; C_K ≈ 1.5 Kolmogorov constant.
**Atom or composite:** Composite — dimensional/scaling argument.
**Cost model:** Spectral analysis O(N log N).
**Real wall?** Yes — observed but intermittency corrections at high orders.
**Cross-domain wiring:** astrophysics-cosmology (ISM turbulence), signal-processing-rf (spectrum estimation).
**Notes:** Kolmogorov (1941). Verified experimentally across many flows.

### reynolds-stress (cross-domain alias: `-ρ⟨u'u'⟩`, `turbulent-momentum-flux`, `closure-tensor`)
**Domain:** Physics / Diffusion
**Definition:** −ρ⟨u'_i u'_j⟩ — average of fluctuation products; appears as additional stress in averaged equations.
**Atom or composite:** Atom — turbulence statistic.
**Cost model:** Closure models add equations; large eddy simulation costs.
**Real wall?** Yes — closure problem (no exact closed equation).
**Cross-domain wiring:** statistics-probability (correlation tensor), control-numerical-opt (turbulence modeling).
**Notes:** Reynolds (1895). 6 independent components in 3D.

### rans (cross-domain alias: `reynolds-averaged-NS`, `time-averaged-NS`, `engineering-CFD`)
**Domain:** Physics / Diffusion
**Definition:** Reynolds-averaged Navier-Stokes equations: NS for ⟨u⟩ with closure for Reynolds stresses.
**Atom or composite:** Composite — time-averaging + closure.
**Cost model:** Much cheaper than DNS/LES; O(N) per step.
**Real wall?** Yes — closure (k-ε, k-ω, RSM) accuracy varies.
**Cross-domain wiring:** control-numerical-opt (CFD design), astrophysics-cosmology (mean flows).
**Notes:** Engineering workhorse; sacrifices unsteady info for tractability.

### les (cross-domain alias: `large-eddy-simulation`, `filtered-NS`, `SGS-modeling`)
**Domain:** Physics / Diffusion
**Definition:** Filter NS over scale Δ; resolve large eddies, model subgrid stresses (e.g., Smagorinsky ν_t = (C_s Δ)²|S|).
**Atom or composite:** Composite — spatial filtering.
**Cost model:** O(Re^{9/4}/(filter_ratio)) vs DNS; cheaper at high Re.
**Real wall?** Yes — SGS modeling near walls is hard.
**Cross-domain wiring:** astrophysics-cosmology (cosmological hydro), control-numerical-opt (CFD).
**Notes:** Smagorinsky (1963), Lilly (1967). Compromise between DNS and RANS.

### dns (cross-domain alias: `direct-numerical-simulation`, `no-model-NS`, `fully-resolved-turbulence`)
**Domain:** Physics / Diffusion
**Definition:** Solve NS directly with grid resolving Kolmogorov scale η; no turbulence model.
**Atom or composite:** Atom — gold-standard simulation.
**Cost model:** O(Re^{9/4} log Re) memory and ops; constrains achievable Re.
**Real wall?** Yes — limited to moderate Re even on largest supercomputers.
**Cross-domain wiring:** statistics-probability (statistics of turbulence), control-numerical-opt (benchmark).
**Notes:** Orszag-Patterson (1972) seminal DNS. Validation tool for turbulence theory.

### lattice-boltzmann (cross-domain alias: `LBM`, `kinetic-CFD`, `BGK-on-lattice`)
**Domain:** Physics / Diffusion
**Definition:** Discrete velocity Boltzmann eq: f_i(x+c_iΔt, t+Δt) = f_i + ω(f_i^{eq} − f_i) with BGK collision.
**Atom or composite:** Composite — discrete kinetic scheme.
**Cost model:** O(N_cells × N_vels) per step; embarrassingly parallel.
**Real wall?** Yes — low-Mach incompressible limit; stability limits Δt.
**Cross-domain wiring:** condensed-matter (microflows), control-numerical-opt (CFD).
**Notes:** McNamara-Zanetti (1988). Recovers NS in low-Mach limit via Chapman-Enskog.

### sph (cross-domain alias: `smoothed-particle-hydrodynamics`, `meshless-fluid`, `kernel-particle-method`)
**Domain:** Physics / Diffusion
**Definition:** Lagrangian: ρ(r) = Σ_j m_j W(|r−r_j|, h) with smoothing kernel W; equations for particles.
**Atom or composite:** Composite — kernel-based discretization.
**Cost model:** O(N log N) with tree-based neighbor search.
**Real wall?** Yes — shock capturing requires artificial viscosity.
**Cross-domain wiring:** astrophysics-cosmology (galaxy formation), control-numerical-opt (free surface flows).
**Notes:** Gingold-Monaghan (1977), Lucy (1977). Lagrangian alternative to grid methods.

### compressible-flow (cross-domain alias: `gasdynamics`, `Ma>0.3-flow`, `density-variable-flow`)
**Domain:** Physics / Diffusion
**Definition:** Flow with significant density changes from pressure/temperature: requires energy equation and EoS.
**Atom or composite:** Composite — Euler/NS with ρ(p,T).
**Cost model:** Riemann-solver-based FV O(N) per step.
**Real wall?** Yes — shocks, choked flow at Ma=1.
**Cross-domain wiring:** astrophysics-cosmology (SN ejecta), control-numerical-opt (aerodynamics).
**Notes:** Important for Ma>0.3, supersonic regimes, and any flow with thermodynamics.

### sound-waves (cross-domain alias: `acoustic-waves`, `compressional-waves`, `pressure-perturbations`)
**Domain:** Physics / Diffusion
**Definition:** Linearized small perturbations of compressible flow at rest: ∂²p'/∂t² = c_s²∇²p' with c_s = √(∂p/∂ρ)_s.
**Atom or composite:** Composite — linearization of compressible flow.
**Cost model:** O(N log N) per step.
**Real wall?** Yes — nonlinear steepening leads to shocks.
**Cross-domain wiring:** astrophysics-cosmology (cosmological perturbations), signal-processing-rf (acoustics).
**Notes:** Newton, Laplace (corrected for adiabatic). Foundation of acoustics.

---

## Statistical Mechanics

### canonical-ensemble (cross-domain alias: `NVT-ensemble`, `boltzmann-ensemble`, `fixed-T-ensemble`)
**Domain:** Physics / Diffusion
**Definition:** Probability of state s: p_s = e^{−E_s/kT}/Z with Z = Σ_s e^{−E_s/kT}.
**Atom or composite:** Atom — fixed N, V, T.
**Cost model:** Z combinatorial; sampling via Metropolis O(N×steps).
**Real wall?** No.
**Cross-domain wiring:** statistics-probability (Gibbs measure), quantum-computing (Boltzmann machines).
**Notes:** Gibbs (1902). Mostly used because experimentally most relevant.

### microcanonical-ensemble (cross-domain alias: `NVE-ensemble`, `fixed-E-ensemble`, `equal-a-priori-probabilities`)
**Domain:** Physics / Diffusion
**Definition:** All microstates with energy in [E, E+dE] equally probable: p_s = 1/Ω(E).
**Atom or composite:** Atom — fixed N, V, E.
**Cost model:** Ω(E) counting via density-of-states methods.
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (isolated systems), control-numerical-opt (Liouville flow).
**Notes:** Boltzmann (1877). S = k ln Ω; foundation of all ensembles.

### grand-canonical-ensemble (cross-domain alias: `μVT-ensemble`, `open-system-ensemble`, `chemical-potential-ensemble`)
**Domain:** Physics / Diffusion
**Definition:** Probability of state s with N_s particles: p_s = e^{−(E_s−μN_s)/kT}/Ξ.
**Atom or composite:** Atom — fixed μ, V, T.
**Cost model:** Ξ = Σ_N e^{βμN} Z(N); GC Monte Carlo O(N×steps).
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (Fermi/Bose), statistics-probability (point processes).
**Notes:** Gibbs. Used for variable-N (open) systems.

### equipartition-theorem (cross-domain alias: `(1/2)kT-per-DoF`, `quadratic-DoF`, `classical-energy-partition`)
**Domain:** Physics / Diffusion
**Definition:** Each quadratic DoF in H contributes (1/2)kT to ⟨E⟩ at thermal equilibrium.
**Atom or composite:** Atom — consequence of canonical ensemble.
**Cost model:** O(1).
**Real wall?** Yes — fails when ℏω ≳ kT (quantum freezing).
**Cross-domain wiring:** quantum-computing (UV catastrophe resolution), condensed-matter (Dulong-Petit).
**Notes:** Maxwell (1860). Foundation of ideal gas: U = (3/2)NkT.

### boltzmann-distribution (cross-domain alias: `e^{-E/kT}`, `gibbs-measure`, `thermal-occupation`)
**Domain:** Physics / Diffusion
**Definition:** Probability of state ∝ e^{−E/kT}; ratio of occupancies p₁/p₂ = e^{−(E₁−E₂)/kT}.
**Atom or composite:** Atom — fundamental thermal probability.
**Cost model:** O(1) per state.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (thermal states), photonics-optics (Boltzmann factor in lasers).
**Notes:** Boltzmann (1877). Cornerstone of statistical mechanics.

### maxwell-boltzmann (cross-domain alias: `MB-distribution`, `classical-velocity-distribution`, `ideal-gas-velocities`)
**Domain:** Physics / Diffusion
**Definition:** f(v) = (m/2πkT)^{3/2} 4πv² e^{−mv²/2kT} — speed distribution of ideal gas.
**Atom or composite:** Composite — Boltzmann × density of velocity states.
**Cost model:** Sampling via Box-Muller O(N).
**Real wall?** Yes — fails at quantum degeneracy.
**Cross-domain wiring:** statistics-probability (Gaussian moments), quantum-computing (semiclassical limit).
**Notes:** Maxwell (1860), Boltzmann (1872). Most probable v = √(2kT/m), mean v = √(8kT/πm).

### partition-function-derivatives (cross-domain alias: `∂lnZ/∂β`, `Z-derived-quantities`, `cumulant-generators`)
**Domain:** Physics / Diffusion
**Definition:** ⟨E⟩ = −∂ ln Z/∂β; F = −kT ln Z; S = −∂F/∂T; ⟨E²⟩−⟨E⟩² = ∂² ln Z/∂β².
**Atom or composite:** Composite — derived from Z.
**Cost model:** Analytical or O(1) given Z.
**Real wall?** No.
**Cross-domain wiring:** statistics-probability (cumulant gen. function), quantum-computing (operator means).
**Notes:** Ln Z generates moments and cumulants of E. Free energy is convex in β.

### free-energy (cross-domain alias: `F=U-TS`, `helmholtz-energy`, `available-work`)
**Domain:** Physics / Diffusion
**Definition:** F = U − TS = −kT ln Z — minimized in equilibrium at fixed (T,V,N).
**Atom or composite:** Atom — thermodynamic potential.
**Cost model:** Compute Z then F; or thermodynamic integration.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (variational free energy), statistics-probability (KL divergence link).
**Notes:** Helmholtz (1882). Minimum F = equilibrium criterion at fixed T.

### helmholtz-free-energy (cross-domain alias: `F`, `A`, `work-function`)
**Domain:** Physics / Diffusion
**Definition:** F(T,V,N) = U − TS; dF = −SdT − pdV + μdN.
**Atom or composite:** Atom — thermodynamic potential at constant V.
**Cost model:** Same as free energy.
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (phase transitions), statistics-probability (variational bound).
**Notes:** Maxwell relation: (∂S/∂V)_T = (∂p/∂T)_V.

### gibbs-free-energy (cross-domain alias: `G=H-TS`, `gibbs-function`, `constant-p-potential`)
**Domain:** Physics / Diffusion
**Definition:** G(T,p,N) = H − TS = U + pV − TS = μN; dG = −SdT + Vdp + μdN.
**Atom or composite:** Atom — thermodynamic potential at constant p.
**Cost model:** Same as free energy.
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (Clausius-Clapeyron), control-numerical-opt (process design).
**Notes:** Gibbs (1875). Minimum G = equilibrium at fixed (T,p).

### chemical-potential (cross-domain alias: `μ`, `partial-G/∂N`, `fugacity-driver`)
**Domain:** Physics / Diffusion
**Definition:** μ = (∂U/∂N)_{S,V} = (∂F/∂N)_{T,V} = (∂G/∂N)_{T,p} — energy to add a particle.
**Atom or composite:** Atom — conjugate to N.
**Cost model:** Numerical via GC ensemble or Widom insertion.
**Real wall?** Yes — for fermions, μ→E_F at T=0.
**Cross-domain wiring:** condensed-matter (Fermi level), astrophysics-cosmology (chemistry).
**Notes:** Equal in equilibrium across phases — Gibbs phase rule.

### fluctuation-dissipation-theorem (cross-domain alias: `FDT`, `response-vs-noise`, `χ(ω)↔S(ω)`)
**Domain:** Physics / Diffusion
**Definition:** Im χ(ω) = (1/2kT) S(ω) (classical); links linear response χ to equilibrium correlations S.
**Atom or composite:** Atom — equilibrium theorem.
**Cost model:** Direct measurement of S or χ; both informative.
**Real wall?** No.
**Cross-domain wiring:** signal-processing-rf (Johnson-Nyquist noise), control-numerical-opt (stochastic methods).
**Notes:** Callen-Welton (1951). Foundation of linear response theory.

### onsager-reciprocity (cross-domain alias: `L_ij=L_ji`, `cross-coefficient-symmetry`, `microscopic-reversibility`)
**Domain:** Physics / Diffusion
**Definition:** In linear non-equilibrium thermodynamics, kinetic coefficients L_ij = L_ji (with sign flip for time-odd quantities).
**Atom or composite:** Atom — symmetry from microscopic reversibility.
**Cost model:** Constrains models.
**Real wall?** Yes — fails far from equilibrium.
**Cross-domain wiring:** condensed-matter (thermoelectrics), control-numerical-opt (transport models).
**Notes:** Onsager (1931, Nobel 1968). Foundation of irreversible thermodynamics.

### kubo-formula (cross-domain alias: `χ(ω)=⟨[A,B(t)]⟩/iℏ`, `response-formula`, `green-kubo`)
**Domain:** Physics / Diffusion
**Definition:** Linear response χ_AB(ω) = (i/ℏ) ∫₀^∞ ⟨[A(t), B(0)]⟩ e^{iωt} dt.
**Atom or composite:** Composite — built from correlation function.
**Cost model:** Real-time evolution + commutator: O(N²) MD/QMC.
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (conductivity σ_xx), quantum-computing (response functions).
**Notes:** Kubo (1957). Quantum generalization of FDT.

### linear-response (cross-domain alias: `χ(ω)`, `linear-susceptibility`, `first-order-perturbation`)
**Domain:** Physics / Diffusion
**Definition:** Response ⟨A⟩(ω) = χ(ω) F(ω) for small force F; valid to first order.
**Atom or composite:** Atom — perturbative framework.
**Cost model:** O(1) once χ known; eigenmode decomposition.
**Real wall?** Yes — fails for large perturbations.
**Cross-domain wiring:** signal-processing-rf (transfer functions), electromagnetics-antennas (susceptibility).
**Notes:** Foundation of linear electromagnetism, RPA, etc.

### bbgky-hierarchy (cross-domain alias: `bogoliubov-born-green-kirkwood-yvon`, `n-particle-distribution-hierarchy`, `closure-hierarchy`)
**Domain:** Physics / Diffusion
**Definition:** Hierarchy of equations for n-particle distributions f_n; coupled to f_{n+1} via interactions.
**Atom or composite:** Composite — exact but needs closure.
**Cost model:** Truncation needed; otherwise intractable.
**Real wall?** Yes — closure assumptions essential.
**Cross-domain wiring:** astrophysics-cosmology (Vlasov from f_2≈f_1f_1), condensed-matter (kinetics).
**Notes:** Bogoliubov-Born-Green-Kirkwood-Yvon. Closures yield Boltzmann, Vlasov, etc.

### vlasov-equation (cross-domain alias: `collisionless-boltzmann`, `mean-field-kinetic-eq`, `f_1-eq`)
**Domain:** Physics / Diffusion
**Definition:** ∂f/∂t + v·∇_x f + (F/m)·∇_v f = 0 — collisionless evolution of f(x,v,t) in mean force F.
**Atom or composite:** Composite — BBGKY closure.
**Cost model:** PIC O(N_particles) or grid O(N⁶) in 6D phase space.
**Real wall?** Yes — 6D phase space is expensive.
**Cross-domain wiring:** condensed-matter (plasma physics), astrophysics-cosmology (dark matter).
**Notes:** Vlasov (1938). Foundation of plasma physics; Landau damping arises here.

### boltzmann-transport-equation (cross-domain alias: `BTE`, `collisional-kinetic-eq`, `f_1-with-collisions`)
**Domain:** Physics / Diffusion
**Definition:** ∂f/∂t + v·∇f + (F/m)·∇_v f = C[f] — Boltzmann collision integral C couples particles.
**Atom or composite:** Composite — Vlasov + collisions.
**Cost model:** DSMC O(N_particles); deterministic grid O(N⁶+) very expensive.
**Real wall?** Yes — collision term complex.
**Cross-domain wiring:** condensed-matter (electron transport), control-numerical-opt (rarefied gas).
**Notes:** Boltzmann (1872). H-theorem and transport coefficients derived from BTE.

### h-theorem (cross-domain alias: `dH/dt≤0`, `entropy-production`, `boltzmann-irreversibility`)
**Domain:** Physics / Diffusion
**Definition:** H = ∫f ln f d³v decreases monotonically under Boltzmann collisions; reaches minimum at MB distribution.
**Atom or composite:** Atom — theorem.
**Cost model:** Conceptual.
**Real wall?** Yes — apparent paradox with reversibility (Loschmidt).
**Cross-domain wiring:** statistics-probability (KL divergence analog), control-numerical-opt (irreversibility).
**Notes:** Boltzmann (1872). Foundation of second law from microscopic mechanics.

### ergodic-hypothesis (cross-domain alias: `time-avg=ensemble-avg`, `mixing-ergodicity`, `phase-space-equidistribution`)
**Domain:** Physics / Diffusion
**Definition:** Time average of observable along trajectory equals ensemble average over phase space (for almost every IC).
**Atom or composite:** Atom — equivalence postulate.
**Cost model:** Conceptual; tested via MD.
**Real wall?** Yes — KAM tori violate ergodicity in integrable systems.
**Cross-domain wiring:** quantum-computing (eigenstate thermalization), statistics-probability (mixing).
**Notes:** Boltzmann's hypothesis. Justifies replacing time averages with ensemble averages.

### liouville-equation (cross-domain alias: `∂ρ/∂t+{ρ,H}=0`, `phase-space-density-evolution`, `quantum-liouville-von-neumann`)
**Domain:** Physics / Diffusion
**Definition:** ∂ρ(q,p,t)/∂t = −{ρ, H} — phase-space density conserved along Hamiltonian flow.
**Atom or composite:** Atom — fundamental kinetic equation.
**Cost model:** PIC or grid in 6D phase space — expensive.
**Real wall?** Yes — high-dim discretization.
**Cross-domain wiring:** quantum-computing (von Neumann eq: iℏ∂ρ/∂t = [H,ρ]), control-numerical-opt.
**Notes:** Liouville (1838). Foundation of statistical mechanics from classical dynamics.

### virial-expansion (cross-domain alias: `pV/NkT=1+B₂(T)n+B₃n²+...`, `density-expansion`, `imperfect-gas`)
**Domain:** Physics / Diffusion
**Definition:** EoS expanded in density n: p/kT = n + B₂(T)n² + B₃(T)n³ + ...
**Atom or composite:** Composite — perturbative in n.
**Cost model:** B_n integrals over n-body configurations: high-n expensive.
**Real wall?** Yes — series convergence limited near phase transitions.
**Cross-domain wiring:** condensed-matter (real gas EoS), statistics-probability (cluster).
**Notes:** Kamerlingh Onnes (1901). B₂(T) = -2π∫(e^{-βU(r)}-1)r²dr for pair interactions.

### cluster-expansion (cross-domain alias: `mayer-cluster`, `linked-cluster-expansion`, `linked-graph-expansion`)
**Domain:** Physics / Diffusion
**Definition:** ln Z = Σ_cluster (cluster diagrams); B_n from sums over n-clusters of Mayer f-functions f_ij = e^{−βU_ij}−1.
**Atom or composite:** Composite — graph expansion.
**Cost model:** Combinatorial; high-order clusters exponentially expensive.
**Real wall?** Yes — exponential growth in diagrams.
**Cross-domain wiring:** quantum-computing (Feynman diagrams analog), condensed-matter.
**Notes:** Mayer (1937). Linked cluster theorem: only connected diagrams contribute to ln Z.

### mayer-expansion (cross-domain alias: `Mayer-f-expansion`, `f-bond-expansion`, `imperfect-gas-expansion`)
**Domain:** Physics / Diffusion
**Definition:** Expand interaction part of Z in products of f_ij = e^{−βU(r_ij)} − 1; suitable for short-ranged potentials.
**Atom or composite:** Composite — diagrammatic.
**Cost model:** Same as cluster expansion.
**Real wall?** Yes — convergence at high density.
**Cross-domain wiring:** quantum-computing (vertex factors), condensed-matter (liquid theory).
**Notes:** Maps complex many-body integrals to diagram sums; backbone of liquid-state theory.

### second-law-statement (cross-domain alias: `dS≥0`, `entropy-increase`, `thermodynamic-arrow`)
**Domain:** Physics / Diffusion
**Definition:** Total entropy of isolated system never decreases: dS_total ≥ 0; equality for reversible processes.
**Atom or composite:** Atom — fundamental law.
**Cost model:** Conceptual.
**Real wall?** Yes — distinguishes past from future.
**Cross-domain wiring:** quantum-computing (decoherence direction), control-numerical-opt (irreversibility).
**Notes:** Clausius (1865). Statistical foundation from H-theorem and Boltzmann's S = k ln Ω.

---

## Phase Transitions & RG

### landau-theory (cross-domain alias: `phenomenological-mean-field`, `F(η)-expansion`, `order-parameter-expansion`)
**Domain:** Physics / Diffusion
**Definition:** F(η) = a(T)η² + bη⁴ + ... with a(T) = a₀(T−T_c); symmetry dictates allowed terms.
**Atom or composite:** Composite — phenomenological expansion.
**Cost model:** Minimization O(1).
**Real wall?** Yes — fails near T_c due to fluctuations (Ginzburg criterion).
**Cross-domain wiring:** quantum-computing (variational ansatz), condensed-matter.
**Notes:** Landau (1937). Foundation of phase transition theory; needs Ginzburg-Landau for fluctuations.

### ginzburg-landau (cross-domain alias: `GL-theory`, `field-theoretic-Landau`, `superconductivity-GL`)
**Domain:** Physics / Diffusion
**Definition:** F[ψ] = ∫(|∇ψ|² + a|ψ|² + b|ψ|⁴) d^d x — Landau theory upgraded to field theory.
**Atom or composite:** Composite — field-theoretic action.
**Cost model:** Mean field analytic; fluctuation corrections via RG.
**Real wall?** Yes — mean field invalid below upper critical dimension d_c = 4 (for n=1).
**Cross-domain wiring:** condensed-matter (superconductivity), quantum-computing (Higgs analog).
**Notes:** Ginzburg-Landau (1950). Foundation of order-parameter field theory.

### order-parameter (cross-domain alias: `η`, `phase-distinguishing-variable`, `broken-symmetry-OP`)
**Domain:** Physics / Diffusion
**Definition:** Field η whose value distinguishes ordered (η≠0) from disordered (η=0) phases.
**Atom or composite:** Atom — symmetry-broken variable.
**Cost model:** Direct measurement.
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (magnetization, density wave), quantum-computing (mean-field ansatz).
**Notes:** Landau. Distinct OPs for distinct phases (sometimes more than one).

### broken-symmetry (cross-domain alias: `SSB`, `spontaneous-symmetry-breaking`, `ground-state-asymmetry`)
**Domain:** Physics / Diffusion
**Definition:** Hamiltonian symmetric but ground state breaks symmetry; e.g., ⟨ψ⟩ ≠ 0 with H invariant under ψ → −ψ.
**Atom or composite:** Atom — phenomenon.
**Cost model:** Conceptual.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (Higgs), condensed-matter (ferromagnets), astrophysics-cosmology.
**Notes:** Continuous symmetry: massless Goldstones; discrete: domain walls.

### mean-field-theory (cross-domain alias: `MFT`, `factorized-state`, `Hartree-approximation`)
**Domain:** Physics / Diffusion
**Definition:** Replace interactions with effective single-particle mean field; e.g., Ising H_MF = −Js⟨s⟩.
**Atom or composite:** Composite — approximation.
**Cost model:** Self-consistent O(N_iter) per scale.
**Real wall?** Yes — fails near T_c and in low dim.
**Cross-domain wiring:** quantum-computing (Hartree-Fock), condensed-matter (BCS).
**Notes:** Curie-Weiss for magnetism. Becomes exact in d→∞ or for infinite-range interactions.

### critical-exponents (cross-domain alias: `α,β,γ,δ,ν,η`, `critical-power-laws`, `universality-exponents`)
**Domain:** Physics / Diffusion
**Definition:** C ∝ |t|^{-α}, M ∝ |t|^β, χ ∝ |t|^{-γ}, M ∝ H^{1/δ}, ξ ∝ |t|^{-ν}, G ∝ 1/r^{d-2+η} at criticality.
**Atom or composite:** Composite — set of exponents.
**Cost model:** Numerical via FSS or RG.
**Real wall?** Yes — universal but exact values hard to compute (ε-expansion, MC, CFT bootstrap).
**Cross-domain wiring:** statistics-probability (heavy-tail), condensed-matter (universality classes).
**Notes:** Scaling relations: α + 2β + γ = 2, γ = β(δ−1), etc.

### scaling-hypothesis (cross-domain alias: `widom-scaling`, `homogeneous-F`, `scaling-form`)
**Domain:** Physics / Diffusion
**Definition:** Near T_c, free energy is homogeneous: F(t, h) = b^{-d} F(b^{y_t} t, b^{y_h} h). All exponents reduce to y_t, y_h.
**Atom or composite:** Composite — homogeneity ansatz.
**Cost model:** Conceptual.
**Real wall?** No.
**Cross-domain wiring:** statistics-probability (self-similar), condensed-matter (critical phenomena).
**Notes:** Widom (1965), Kadanoff (1966). Foundation of modern critical phenomena.

### universality (cross-domain alias: `universality-classes`, `dimension+symmetry-only`, `microscopic-detail-irrelevance`)
**Domain:** Physics / Diffusion
**Definition:** Critical exponents depend only on (d, symmetry of OP, range of interactions), not on microscopic details.
**Atom or composite:** Atom — RG principle.
**Cost model:** Conceptual.
**Real wall?** No — robust.
**Cross-domain wiring:** condensed-matter (Ising, XY, Heisenberg classes), quantum-computing (Q-bit physics universality).
**Notes:** Emerges from RG: irrelevant operators flow to zero at fixed point.

### wilson-rg (cross-domain alias: `Kadanoff-Wilson-RG`, `block-spin-renormalization`, `Wilson-flow`)
**Domain:** Physics / Diffusion
**Definition:** Iterative coarse-graining: integrate out high-k modes, rescale to get effective theory at larger scale.
**Atom or composite:** Composite — RG procedure.
**Cost model:** Numerical via Wilsonian momentum-shell integration.
**Real wall?** Yes — full nonperturbative RG hard.
**Cross-domain wiring:** quantum-computing (entanglement RG), control-numerical-opt (multiscale).
**Notes:** Wilson (1971, Nobel 1982). Won Nobel for explaining universality.

### epsilon-expansion (cross-domain alias: `ε=4-d-expansion`, `Wilson-Fisher-expansion`, `d=4−ε-RG`)
**Domain:** Physics / Diffusion
**Definition:** Expand around upper critical dimension d=4: critical exponents as series in ε = 4−d.
**Atom or composite:** Composite — perturbative RG.
**Cost model:** Higher orders: many-loop diagrams.
**Real wall?** Yes — asymptotic, not convergent.
**Cross-domain wiring:** quantum-computing (loop integrals), condensed-matter.
**Notes:** Wilson-Fisher (1972). Predicts Ising exponents at d=3 with ε=1.

### block-spin-transformation (cross-domain alias: `Kadanoff-blocks`, `coarse-graining-spins`, `majority-rule-blocking`)
**Domain:** Physics / Diffusion
**Definition:** Group b^d spins into blocks; new spin = majority (or appropriate average); iterate.
**Atom or composite:** Composite — real-space RG.
**Cost model:** O(N) per RG step; storage O(N).
**Real wall?** Yes — keeping all operators infeasible.
**Cross-domain wiring:** quantum-computing (MERA tensor networks), control-numerical-opt (multigrid).
**Notes:** Kadanoff (1966). Real-space alternative to momentum-shell RG.

### fixed-points-rg (cross-domain alias: `RG-fixed-points`, `flow-attractors`, `critical-vs-trivial-FPs`)
**Domain:** Physics / Diffusion
**Definition:** Couplings unchanged under RG: K_n+1 = K_n; stable fixed points correspond to phases, unstable to critical points.
**Atom or composite:** Atom — fixed point of RG flow.
**Cost model:** Find roots of RG eqs.
**Real wall?** No.
**Cross-domain wiring:** control-numerical-opt (attractor analysis), quantum-computing (gapless theories).
**Notes:** Critical exponents = eigenvalues of linearized RG at fixed point.

### relevant-irrelevant-operators (cross-domain alias: `relevant-vs-irrelevant`, `RG-scaling-dimensions`, `marginal-operators`)
**Domain:** Physics / Diffusion
**Definition:** Operator O_i is relevant if its coupling grows under RG (y_i > 0), irrelevant if shrinks (y_i < 0), marginal y_i = 0.
**Atom or composite:** Composite — classification by RG eigenvalues.
**Cost model:** Compute scaling dimensions.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (asymptotic freedom), condensed-matter (universality).
**Notes:** Explains universality: irrelevant operators drop out at large scales.

### percolation-thresholds (cross-domain alias: `p_c`, `connectivity-transition`, `cluster-formation-threshold`)
**Domain:** Physics / Diffusion
**Definition:** Critical occupation probability p_c at which infinite cluster forms; lattice-dependent.
**Atom or composite:** Atom — critical probability.
**Cost model:** Hoshen-Kopelman O(N) cluster labeling.
**Real wall?** Yes — exact p_c known only in some lattices.
**Cross-domain wiring:** statistics-probability (geometric phase transition), condensed-matter (disordered systems).
**Notes:** Broadbent-Hammersley (1957). 2D bond percolation on square lattice: p_c = 1/2.

### curie-temperature (cross-domain alias: `T_C`, `ferromagnetic-transition`, `magnetic-T_c`)
**Domain:** Physics / Diffusion
**Definition:** Temperature above which ferromagnet loses spontaneous magnetization; second-order phase transition.
**Atom or composite:** Atom — material-specific T_c.
**Cost model:** Measured or computed via Heisenberg/Ising models.
**Real wall?** Yes — sets working temperatures for magnets.
**Cross-domain wiring:** condensed-matter (magnetism), control-numerical-opt (Monte Carlo).
**Notes:** Curie (1895). Fe: T_C ≈ 1043 K; Ni: 627 K; Gd: 292 K.

### bose-einstein-condensation (cross-domain alias: `BEC`, `macroscopic-Bose-occupation`, `quantum-condensation`)
**Domain:** Physics / Diffusion
**Definition:** Below T_c, finite fraction of bosons occupy single ground state; T_c = (2πℏ²/m)(n/ζ(3/2))^{2/3}/k.
**Atom or composite:** Composite — quantum statistical effect.
**Cost model:** Analytic for ideal gas; QMC for interacting.
**Real wall?** Yes — requires T < T_c; interactions complicate.
**Cross-domain wiring:** quantum-computing (BEC qubits), photonics-optics (photon condensation).
**Notes:** Bose-Einstein (1924), realized in cold atoms (Cornell, Wieman, Ketterle 1995).

### mott-transition (cross-domain alias: `Mott-Hubbard-MIT`, `metal-insulator-transition`, `U-driven-transition`)
**Domain:** Physics / Diffusion
**Definition:** Transition from metal to insulator driven by electron-electron repulsion U; insulator with one electron per site when U≫t.
**Atom or composite:** Composite — strong-correlation phenomenon.
**Cost model:** DMFT or QMC; expensive.
**Real wall?** Yes — strong correlations defeat mean field.
**Cross-domain wiring:** quantum-computing (Hubbard model), condensed-matter.
**Notes:** Mott (1949). Resolves "why some half-filled bands are insulators."

### bcs-bec-crossover (cross-domain alias: `weak-strong-pairing`, `superfluid-crossover`, `Feshbach-tuning`)
**Domain:** Physics / Diffusion
**Definition:** Smooth crossover from BCS-pairing (weakly bound Cooper pairs) to BEC (tightly bound molecules) as attraction increases.
**Atom or composite:** Composite — interpolation.
**Cost model:** Mean-field crossover analytic; beyond MF expensive.
**Real wall?** Yes — strong-coupling regime requires nonperturbative methods.
**Cross-domain wiring:** condensed-matter (high-Tc?), quantum-computing (BEC-BCS in cold atoms).
**Notes:** Leggett (1980), Eagles (1969). Cold-atom realization via Feshbach resonances.

### bkt-transition (cross-domain alias: `Berezinskii-Kosterlitz-Thouless`, `topological-phase-transition`, `vortex-unbinding`)
**Domain:** Physics / Diffusion
**Definition:** 2D XY model: low-T phase has quasi-long-range order; vortex unbinding at T_BKT destroys it.
**Atom or composite:** Composite — topological transition.
**Cost model:** MC study O(N²).
**Real wall?** Yes — no symmetry breaking (Mermin-Wagner) but transition exists.
**Cross-domain wiring:** condensed-matter (2D superfluid), photonics-optics (2D lasers).
**Notes:** Berezinskii (1971), Kosterlitz-Thouless (1973, Nobel 2016). Infinite-order transition.

### vortex-unbinding (cross-domain alias: `topological-unbinding`, `BKT-mechanism`, `pair-creation-transition`)
**Domain:** Physics / Diffusion
**Definition:** At T_BKT, vortex-antivortex pairs unbind into free vortices; correlation length diverges essential-singularly.
**Atom or composite:** Composite — mechanism of BKT.
**Cost model:** Topological charge counting in MC.
**Real wall?** Yes.
**Cross-domain wiring:** condensed-matter (Helium films), quantum-computing (topological order).
**Notes:** Kosterlitz-Thouless mechanism. Entropy of free vortex matches energy at T_BKT.

### first-vs-second-order-transitions (cross-domain alias: `discontinuous-vs-continuous`, `latent-heat-vs-singular`, `Ehrenfest-class`)
**Domain:** Physics / Diffusion
**Definition:** First order: discontinuous OP, latent heat; second order: continuous OP, divergent susceptibility.
**Atom or composite:** Composite — classification.
**Cost model:** Equation of state analysis.
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (water-ice vs Curie), control-numerical-opt (nucleation barriers).
**Notes:** Ehrenfest (1933) classification by derivative discontinuity.

### hysteresis (cross-domain alias: `path-dependence`, `metastability-driven-loop`, `B-H-loop`)
**Domain:** Physics / Diffusion
**Definition:** System response depends on history due to metastability; e.g., ferromagnet M(H) traces loop.
**Atom or composite:** Composite — metastable dynamics.
**Cost model:** Time-dependent simulation O(N×steps).
**Real wall?** Yes — irreversibility.
**Cross-domain wiring:** condensed-matter (magnetic recording), control-numerical-opt (Preisach model).
**Notes:** Ewing (1882). Area under loop = energy dissipated per cycle.

### lattice-models (cross-domain alias: `discrete-spin-models`, `lattice-spins`, `statistical-mechanics-on-lattice`)
**Domain:** Physics / Diffusion
**Definition:** Spins on lattice with H = −Σ J_ij f(s_i, s_j); examples: Ising, Potts, XY, Heisenberg, clock.
**Atom or composite:** Composite — family.
**Cost model:** MC O(N×sweeps); exact for some 2D models.
**Real wall?** Yes — exact solutions rare beyond 2D Ising.
**Cross-domain wiring:** quantum-computing (lattice gauge theory), condensed-matter (magnetism).
**Notes:** Workhorses of statistical mechanics; Onsager's 2D Ising exact solution (1944).

### potts-model (cross-domain alias: `q-state-Potts`, `general-discrete-spin-model`, `Potts-Ising-generalization`)
**Domain:** Physics / Diffusion
**Definition:** H = −J Σ_{⟨ij⟩} δ_{s_i, s_j} with s_i ∈ {1, ..., q}; reduces to Ising at q=2.
**Atom or composite:** Composite — generalizes Ising.
**Cost model:** Wolff/Swendsen-Wang cluster algorithm O(N) per sweep.
**Real wall?** Yes — transition becomes first-order at q≥5 in 2D.
**Cross-domain wiring:** condensed-matter (FePt alloys), statistics-probability (community detection analog).
**Notes:** Potts (1952). q-coloring problem in CS for q discrete.

---

## Plasma & MHD

### vlasov-equation-plasma (cross-domain alias: `collisionless-plasma-kinetics`, `vlasov-poisson`, `vlasov-maxwell`)
**Domain:** Physics / Diffusion
**Definition:** ∂f_s/∂t + v·∇f_s + (q_s/m_s)(E + v×B)·∇_v f_s = 0; coupled to Maxwell for self-consistent fields.
**Atom or composite:** Composite — kinetic equation.
**Cost model:** PIC O(N_particles); Eulerian grid O(N^6) in phase space.
**Real wall?** Yes — 6D phase space + Maxwell.
**Cross-domain wiring:** astrophysics-cosmology (cosmic plasmas), electromagnetics-antennas (RF discharge).
**Notes:** Vlasov (1938). Collisionless limit valid when ν_ei → 0.

### ideal-mhd (cross-domain alias: `infinite-conductivity-MHD`, `ideal-magnetohydrodynamics`, `frozen-flux-MHD`)
**Domain:** Physics / Diffusion
**Definition:** Continuity + Euler + ∂B/∂t = ∇×(u×B) + ∇·B = 0; valid at large scales, high conductivity.
**Atom or composite:** Composite — fluid + frozen B.
**Cost model:** Godunov-MHD O(N×timestep).
**Real wall?** Yes — fails at small scales (need resistivity).
**Cross-domain wiring:** astrophysics-cosmology (solar wind, accretion disks), control-numerical-opt (fusion).
**Notes:** Alfvén (1942, Nobel 1970). Foundation of large-scale plasma dynamics.

### resistive-mhd (cross-domain alias: `MHD-with-η`, `resistive-magnetohydrodynamics`, `finite-conductivity-MHD`)
**Domain:** Physics / Diffusion
**Definition:** Ohm's law E + u×B = ηJ; induction eq ∂B/∂t = ∇×(u×B) − ∇×(η∇×B/μ₀).
**Atom or composite:** Composite — diffusive correction.
**Cost model:** Adds diffusion term: implicit or sub-cycled.
**Real wall?** Yes — resistive timescale τ_R = μ₀L²/η.
**Cross-domain wiring:** astrophysics-cosmology (reconnection), control-numerical-opt (fusion plasma).
**Notes:** Sweet-Parker reconnection rate ~ S^{-1/2} where S = τ_R/τ_A.

### alfven-waves (cross-domain alias: `transverse-MHD-waves`, `magnetic-tension-waves`, `v_A=B/√(μ₀ρ)`)
**Domain:** Physics / Diffusion
**Definition:** Transverse waves propagating along B with v_A = B/√(μ₀ρ); incompressible.
**Atom or composite:** Atom — MHD mode.
**Cost model:** Linear MHD eigenmode O(1).
**Real wall?** Yes — Alfvén speed limits causality in MHD.
**Cross-domain wiring:** astrophysics-cosmology (solar corona heating), signal-processing-rf (RF in plasmas).
**Notes:** Alfvén (1942). Magnetic tension restores; analog of stringed-wave on field line.

### magnetosonic-waves (cross-domain alias: `fast-slow-MHD`, `compressible-MHD-modes`, `magnetosonic-modes`)
**Domain:** Physics / Diffusion
**Definition:** Compressible MHD modes propagating across B with speeds v_± = √((c_s² + v_A² ± Δ)/2).
**Atom or composite:** Composite — coupled magnetic-acoustic.
**Cost model:** Linear MHD eigenanalysis.
**Real wall?** Yes — characteristic speeds limit timestep.
**Cross-domain wiring:** astrophysics-cosmology (solar wind, ICME shocks), control-numerical-opt.
**Notes:** Fast: both magnetic and gas pressure aligned. Slow: opposed.

### frozen-flux-theorem (cross-domain alias: `Alfven-theorem`, `field-line-tying`, `flux-conservation`)
**Domain:** Physics / Diffusion
**Definition:** In ideal MHD, magnetic flux through any co-moving surface is conserved; field lines move with fluid.
**Atom or composite:** Atom — ideal MHD theorem.
**Cost model:** Constraint on numerical schemes (CT, divergence-free).
**Real wall?** Yes — fails when η > 0 (reconnection).
**Cross-domain wiring:** astrophysics-cosmology (galactic dynamos), control-numerical-opt (CT schemes).
**Notes:** Alfvén theorem; analogous to Kelvin's circulation theorem in fluid mechanics.

### magnetic-reconnection (cross-domain alias: `field-line-reconnection`, `topology-change`, `reconnection-rate`)
**Domain:** Physics / Diffusion
**Definition:** Topology change of B field lines in localized resistive/collisionless region; releases magnetic energy.
**Atom or composite:** Composite — multiscale process.
**Cost model:** Extended MHD or PIC; expensive multiscale.
**Real wall?** Yes — bridges MHD and kinetic scales.
**Cross-domain wiring:** astrophysics-cosmology (solar flares, magnetotail), control-numerical-opt (fusion disruptions).
**Notes:** Sweet-Parker, Petschek, Hall reconnection models give different rates.

### sweet-parker-reconnection (cross-domain alias: `SP-reconnection`, `resistive-current-sheet`, `M_A∝S^-1/2`)
**Domain:** Physics / Diffusion
**Definition:** Steady-state resistive reconnection with elongated current sheet: M_A = v_in/v_A ~ S^{-1/2}.
**Atom or composite:** Composite — analytic resistive MHD model.
**Cost model:** Analytical estimate; numerical verification expensive.
**Real wall?** Yes — too slow to explain observed solar/space events.
**Cross-domain wiring:** astrophysics-cosmology (reconnection physics), control-numerical-opt.
**Notes:** Sweet (1958), Parker (1957). Slow due to long thin current sheet.

### petschek-reconnection (cross-domain alias: `fast-reconnection`, `slow-shock-reconnection`, `M_A~1/log-S`)
**Domain:** Physics / Diffusion
**Definition:** Reconnection with X-line geometry and slow-mode shocks; rate ~1/log S, much faster than SP.
**Atom or composite:** Composite — alternative geometry.
**Cost model:** Requires localized resistivity or anomalous mechanisms.
**Real wall?** Yes — only realized with non-uniform η or kinetic effects.
**Cross-domain wiring:** astrophysics-cosmology (solar flares), control-numerical-opt.
**Notes:** Petschek (1964). Not stable in uniform η; appears in collisionless reconnection.

### drift-waves (cross-domain alias: `density-gradient-waves`, `drift-instability`, `universal-mode`)
**Domain:** Physics / Diffusion
**Definition:** Low-frequency waves in inhomogeneous plasma driven by density gradient; ω ~ ω* = k_y T/(eB L_n).
**Atom or composite:** Composite — kinetic gradient-driven mode.
**Cost model:** Gyrokinetic O(N⁵).
**Real wall?** Yes — turbulence dominates fusion confinement.
**Cross-domain wiring:** condensed-matter (gradient flows), control-numerical-opt (fusion plasma).
**Notes:** Ubiquitous in magnetized plasmas; foundation of tokamak transport models.

### landau-damping (cross-domain alias: `collisionless-damping`, `kinetic-damping`, `particle-wave-resonance`)
**Domain:** Physics / Diffusion
**Definition:** Linear damping of plasma waves due to wave-particle resonance at v = ω/k, despite no collisions.
**Atom or composite:** Atom — kinetic effect.
**Cost model:** Linear analysis O(1).
**Real wall?** Yes — fundamental kinetic effect.
**Cross-domain wiring:** astrophysics-cosmology (CMB Silk damping analog), signal-processing-rf (filter analogy).
**Notes:** Landau (1946). First explicit collisionless dissipation.

### debye-length (cross-domain alias: `λ_D=√(ε₀kT/ne²)`, `screening-length`, `debye-radius`)
**Domain:** Physics / Diffusion
**Definition:** λ_D = √(ε₀kT_e/(ne²)) — scale over which charge perturbations are screened.
**Atom or composite:** Atom — characteristic plasma length.
**Cost model:** O(1).
**Real wall?** Yes — sets scale below which plasma collective behavior breaks down.
**Cross-domain wiring:** condensed-matter (Thomas-Fermi screening), astrophysics-cosmology (ISM).
**Notes:** Debye-Hückel (1923). Plasma defined by λ_D ≪ system size.

### plasma-frequency (cross-domain alias: `ω_p=√(ne²/ε₀m)`, `Langmuir-frequency`, `electron-oscillation-freq`)
**Domain:** Physics / Diffusion
**Definition:** ω_p = √(ne²/(ε₀m)) — natural electron oscillation frequency in cold plasma.
**Atom or composite:** Atom — characteristic frequency.
**Cost model:** O(1).
**Real wall?** Yes — EM waves with ω < ω_p reflect (radio reflection from ionosphere).
**Cross-domain wiring:** photonics-optics (Drude metals), electromagnetics-antennas (ionosphere).
**Notes:** Tonks-Langmuir (1929). Used for plasma density diagnostics.

### plasma-oscillations (cross-domain alias: `Langmuir-waves`, `electron-plasma-waves`, `ω≈ω_p`)
**Domain:** Physics / Diffusion
**Definition:** Electrostatic oscillations of electrons at ω ≈ ω_p; ion timescales much slower.
**Atom or composite:** Composite — collective oscillation.
**Cost model:** Linear dispersion; PIC for nonlinear.
**Real wall?** Yes — Landau damping at finite k.
**Cross-domain wiring:** signal-processing-rf (plasma diagnostic), photonics-optics (laser-plasma).
**Notes:** Tonks-Langmuir (1929). Foundation of plasma kinetic theory.

### bohm-gross-dispersion (cross-domain alias: `BG-relation`, `langmuir-dispersion`, `ω²=ω_p²+3k²v_te²`)
**Domain:** Physics / Diffusion
**Definition:** Warm-plasma Langmuir dispersion: ω² = ω_p² + 3k²v_te² where v_te = √(kT_e/m_e).
**Atom or composite:** Composite — extension to warm plasma.
**Cost model:** O(1).
**Real wall?** Yes — breaks at kλ_D ~ 1.
**Cross-domain wiring:** signal-processing-rf (plasma diagnostics), condensed-matter (thermal corrections).
**Notes:** Bohm-Gross (1949). Warm-fluid Langmuir wave dispersion.

### ion-acoustic-waves (cross-domain alias: `IAW`, `ion-sound-waves`, `low-frequency-plasma-waves`)
**Domain:** Physics / Diffusion
**Definition:** Sound-like waves with c_s = √((γ_e T_e + γ_i T_i)/m_i); ω = c_s k for kλ_D ≪ 1.
**Atom or composite:** Composite — fluid mode.
**Cost model:** Linear dispersion.
**Real wall?** Yes — strong Landau damping when T_e ~ T_i.
**Cross-domain wiring:** astrophysics-cosmology (solar wind), control-numerical-opt (laser-plasma).
**Notes:** Tonks-Langmuir (1929). Dominate low-frequency response of plasma.

### dust-acoustic-waves (cross-domain alias: `DAW`, `dusty-plasma-waves`, `complex-plasma-modes`)
**Domain:** Physics / Diffusion
**Definition:** Very low-frequency waves in dusty plasma; c_DAW = √(Z_d² n_d T_e/(n_e m_d)) ≪ c_s.
**Atom or composite:** Composite — mode in 3-component plasma.
**Cost model:** Linear theory; large mass ratio makes timestep long.
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (ISM dust), condensed-matter (complex plasmas).
**Notes:** Rao-Shukla-Yu (1990). Observable due to slow speed.

### buneman-instability (cross-domain alias: `electron-ion-streaming`, `current-driven-instability`, `relative-drift-instability`)
**Domain:** Physics / Diffusion
**Definition:** Instability when electrons drift faster than ion thermal speed; growth rate γ ~ (m_e/m_i)^{1/3} ω_pe.
**Atom or composite:** Composite — current-driven kinetic instability.
**Cost model:** PIC simulations O(N×T).
**Real wall?** Yes — anomalous resistivity from instability.
**Cross-domain wiring:** astrophysics-cosmology (reconnection inflow), control-numerical-opt (current sheets).
**Notes:** Buneman (1958). Saturates by trapping; generates anomalous resistivity.

### weibel-instability (cross-domain alias: `temperature-anisotropy-instability`, `EM-Weibel`, `magnetic-field-generation`)
**Domain:** Physics / Diffusion
**Definition:** Anisotropy T_⊥ > T_∥ drives growth of transverse B fields; γ_max from kinetic theory.
**Atom or composite:** Composite — EM kinetic instability.
**Cost model:** PIC.
**Real wall?** Yes — saturates by field strength.
**Cross-domain wiring:** astrophysics-cosmology (GRB afterglows), condensed-matter (laser-plasma).
**Notes:** Weibel (1959). Important seed for cosmic magnetic fields.

### two-stream-instability (cross-domain alias: `electron-electron-stream`, `electrostatic-streaming`, `bump-on-tail`)
**Domain:** Physics / Diffusion
**Definition:** Two counterstreaming electron beams unstable; γ_max ~ ω_pe/2 at k = ω_pe/v_0.
**Atom or composite:** Composite — kinetic instability.
**Cost model:** PIC simulations.
**Real wall?** Yes — saturates via particle trapping (BGK modes).
**Cross-domain wiring:** astrophysics-cosmology (solar Type III bursts), condensed-matter (electron beams).
**Notes:** Pierce, Bohm-Gross. Prototype of velocity-space instability.

### kinetic-vs-fluid-description (cross-domain alias: `f(x,v)-vs-moments`, `BBGKY-closure`, `regime-of-validity`)
**Domain:** Physics / Diffusion
**Definition:** Kinetic: full f(x,v,t); fluid: moments ⟨1, v, vv, ...⟩. Fluid valid when collisions equilibrate locally (ω, k·v_t much less than ν).
**Atom or composite:** Composite — modeling choice.
**Cost model:** Kinetic O(N⁶), fluid O(N³).
**Real wall?** Yes — fluid fails for collisionless plasmas.
**Cross-domain wiring:** condensed-matter (electron transport), control-numerical-opt (multiscale).
**Notes:** Most plasmas are intermediate: gyrokinetic captures both.

### gyrokinetic (cross-domain alias: `5D-gyroaveraged-kinetic`, `magnetized-plasma-kinetic`, `removed-gyrophase`)
**Domain:** Physics / Diffusion
**Definition:** Average over rapid gyromotion; reduces 6D Vlasov to 5D in (X, v_∥, μ).
**Atom or composite:** Composite — reduction.
**Cost model:** O(N⁵) per timestep; still expensive but tractable.
**Real wall?** Yes — assumes ω ≪ Ω_ci, k_⊥ρ_i ≲ 1.
**Cross-domain wiring:** control-numerical-opt (fusion plasma), astrophysics-cosmology (turbulence).
**Notes:** Frieman-Chen, Brizard-Hahm. Workhorse of fusion turbulence simulation.

### hall-mhd (cross-domain alias: `extended-MHD`, `Hall-current-MHD`, `whistler-MHD`)
**Domain:** Physics / Diffusion
**Definition:** Generalized Ohm's law E = -u×B + (1/ne)J×B + ηJ; whistler waves enter dispersion.
**Atom or composite:** Composite — extended ideal MHD.
**Cost model:** Adds whistler timestep constraint.
**Real wall?** Yes — fast whistlers force small Δt.
**Cross-domain wiring:** astrophysics-cosmology (reconnection), control-numerical-opt (magnetosheath).
**Notes:** Important when L ~ ion skin depth d_i = c/ω_pi.

### ambipolar-diffusion (cross-domain alias: `neutral-ion-diffusion`, `partially-ionized-MHD`, `slip-diffusion`)
**Domain:** Physics / Diffusion
**Definition:** Slip between ions (frozen to B) and neutrals; effective magnetic diffusivity η_AD = B²/(γ_AD ρ_i ρ_n).
**Atom or composite:** Composite — multifluid effect.
**Cost model:** Adds diffusion term with strong T, ρ dependence.
**Real wall?** Yes — limits flux freezing in weakly ionized plasmas.
**Cross-domain wiring:** astrophysics-cosmology (molecular clouds, protostars), condensed-matter.
**Notes:** Key for star formation: ambipolar diffusion sets timescale for collapse.

### spitzer-resistivity (cross-domain alias: `classical-η`, `coulomb-resistivity`, `η_S~T^{-3/2}`)
**Domain:** Physics / Diffusion
**Definition:** η_S = (m_e ν_ei)/(n_e e²) ∝ T_e^{-3/2}; from Coulomb collisions.
**Atom or composite:** Composite — collisional transport coefficient.
**Cost model:** O(1).
**Real wall?** Yes — hot plasmas are nearly collisionless.
**Cross-domain wiring:** condensed-matter (collisional transport), control-numerical-opt (fusion modeling).
**Notes:** Spitzer-Härm (1953). Anomalous resistivity often dominates in turbulent plasma.

---

## Quantum Mechanics Formalism

### schrodinger-time-dep (cross-domain alias: `iℏ∂ψ/∂t=Hψ`, `TDSE`, `time-dependent-Schrodinger`)
**Domain:** Physics / Diffusion
**Definition:** iℏ ∂ψ/∂t = Ĥψ — first-order in t, linear, deterministic evolution of state.
**Atom or composite:** Atom — fundamental QM equation.
**Cost model:** Split-step Fourier O(N log N); Crank-Nicolson O(N^1.5).
**Real wall?** Yes — Hilbert space dimension grows exponentially with particles.
**Cross-domain wiring:** quantum-computing (unitary evolution), photonics-optics (paraxial analog).
**Notes:** Schrödinger (1926). U(t) = exp(−iĤt/ℏ) is unitary.

### schrodinger-time-indep (cross-domain alias: `Hψ=Eψ`, `TISE`, `stationary-states`)
**Domain:** Physics / Diffusion
**Definition:** Ĥψ_n = E_n ψ_n — eigenvalue problem for stationary states.
**Atom or composite:** Atom — separable in t.
**Cost model:** Lanczos/Arnoldi O(N·k) for k eigenvalues; full diag O(N³).
**Real wall?** Yes — curse of dimensionality.
**Cross-domain wiring:** linear-algebra-matrix (hermitian eigenproblem), condensed-matter (band structure).
**Notes:** Foundation of atomic, molecular, condensed-matter spectra.

### heisenberg-picture (cross-domain alias: `time-dep-operators`, `dÔ/dt=[Ô,Ĥ]/iℏ`, `Heisenberg-eq`)
**Domain:** Physics / Diffusion
**Definition:** Operators evolve, states fixed: dÔ/dt = (i/ℏ)[Ĥ, Ô] + ∂Ô/∂t.
**Atom or composite:** Atom — equivalent picture.
**Cost model:** Evolve operator matrix O(N²) per step.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (Heisenberg evolution), control-numerical-opt (quantum control).
**Notes:** Heisenberg (1925). Equivalent to Schrödinger via U†ÔU.

### interaction-picture (cross-domain alias: `Dirac-picture`, `int-pic`, `H=H_0+V-splitting`)
**Domain:** Physics / Diffusion
**Definition:** ψ_I = e^{iH₀t/ℏ}ψ; operators evolve with H₀, states with V. Useful for time-dependent perturbation theory.
**Atom or composite:** Composite — third picture.
**Cost model:** Dyson series O(N²) per order.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (Trotter splitting), photonics-optics (rotating wave).
**Notes:** Dirac (1927). Foundation of S-matrix, scattering theory.

### ladder-operators (cross-domain alias: `creation-annihilation`, `a†,a`, `raising-lowering-ops`)
**Domain:** Physics / Diffusion
**Definition:** [a, a†] = 1; a|n⟩ = √n |n−1⟩, a†|n⟩ = √(n+1)|n+1⟩.
**Atom or composite:** Atom — algebraic structure.
**Cost model:** O(1) on number basis.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (qudit ladder), photonics-optics (Fock states).
**Notes:** Dirac. Foundation of harmonic oscillator and QFT.

### harmonic-oscillator (cross-domain alias: `QHO`, `H=p²/2m+½mω²x²`, `bosonic-mode`)
**Domain:** Physics / Diffusion
**Definition:** Ĥ = ℏω(a†a + 1/2); E_n = ℏω(n+1/2); eigenstates |n⟩ are number states.
**Atom or composite:** Composite — exactly solvable.
**Cost model:** Algebraic O(1) per matrix element.
**Real wall?** No.
**Cross-domain wiring:** photonics-optics (cavity modes), condensed-matter (phonons), quantum-computing.
**Notes:** Pauli, Dirac. Most important model in QM; coherent states minimize uncertainty.

### hydrogen-atom (cross-domain alias: `Coulomb-bound-states`, `-Ry/n²`, `analytic-atom`)
**Domain:** Physics / Diffusion
**Definition:** H = p²/2m_e − e²/(4πε₀r); E_n = −13.6 eV/n²; eigenstates ψ_{nlm}(r,θ,φ).
**Atom or composite:** Composite — exactly solvable.
**Cost model:** Closed-form via Laguerre × spherical harmonics.
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (hydrogen-like impurities), photonics-optics (Rydberg atoms).
**Notes:** Bohr (1913), full QM Pauli-Schrödinger (1926). Accidental SO(4) degeneracy.

### angular-momentum (cross-domain alias: `L²,L_z`, `commutator-[L_i,L_j]=iℏε_ijk L_k`, `SU(2)-algebra`)
**Domain:** Physics / Diffusion
**Definition:** [L_i, L_j] = iℏε_ijk L_k; L²|l,m⟩ = ℏ²l(l+1)|l,m⟩; L_z|l,m⟩ = ℏm|l,m⟩.
**Atom or composite:** Atom — Lie algebra.
**Cost model:** O(N²) matrix elements.
**Real wall?** No.
**Cross-domain wiring:** linear-algebra-matrix (su(2) algebra), quantum-computing (qubit rotations).
**Notes:** Heisenberg, Wigner. Pauli matrices σ_i = 2L_i/ℏ for spin-1/2.

### spin (cross-domain alias: `intrinsic-angular-momentum`, `S=ℏ/2`, `qubit-spin`)
**Domain:** Physics / Diffusion
**Definition:** Intrinsic angular momentum with no classical analog; for electron S=1/2 ⇒ two basis states.
**Atom or composite:** Atom — quantum number.
**Cost model:** 2×2 matrices for spin-1/2.
**Real wall?** Yes — no classical limit of half-integer spin.
**Cross-domain wiring:** quantum-computing (qubit physics), condensed-matter (magnetism).
**Notes:** Goudsmit-Uhlenbeck (1925). Stern-Gerlach demonstrated spin quantization.

### pauli-matrices (cross-domain alias: `σ_x,σ_y,σ_z`, `2×2-Hermitian-traceless`, `spin-1/2-operators`)
**Domain:** Physics / Diffusion
**Definition:** σ_x = [[0,1],[1,0]], σ_y = [[0,-i],[i,0]], σ_z = [[1,0],[0,-1]]; satisfy σ_iσ_j = δ_ij I + iε_ijk σ_k.
**Atom or composite:** Atom — basis for 2×2 traceless Hermitian.
**Cost model:** O(1) per matrix element.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (Pauli gates X,Y,Z), linear-algebra-matrix (Lie algebra).
**Notes:** Pauli (1927). Generators of SU(2).

### stern-gerlach (cross-domain alias: `SG-experiment`, `spin-quantization-measurement`, `inhomogeneous-B-field`)
**Domain:** Physics / Diffusion
**Definition:** Beam of atoms through inhomogeneous B-field splits into discrete spots — direct evidence of spin quantization.
**Atom or composite:** Composite — experimental concept.
**Cost model:** N/A.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (qubit measurement), condensed-matter (magnetism).
**Notes:** Stern-Gerlach (1922). Demonstrated angular momentum quantization.

### density-matrix (cross-domain alias: `ρ`, `statistical-operator`, `mixed-state-formalism`)
**Domain:** Physics / Diffusion
**Definition:** ρ = Σ_i p_i |ψ_i⟩⟨ψ_i|; ⟨A⟩ = Tr(ρA); evolution iℏ∂ρ/∂t = [H,ρ].
**Atom or composite:** Composite — sum over pure states.
**Cost model:** O(N²) storage; O(N³) operations.
**Real wall?** Yes — N² scaling vs N for pure states.
**Cross-domain wiring:** statistics-probability (probability distribution), quantum-computing (mixed states).
**Notes:** von Neumann (1927). Tr(ρ²) = 1 for pure, <1 for mixed.

### mixed-state (cross-domain alias: `incoherent-superposition`, `statistical-mixture`, `ρ-not-pure`)
**Domain:** Physics / Diffusion
**Definition:** Statistical mixture: ρ = Σ p_i |ψ_i⟩⟨ψ_i| with Σp_i = 1; Tr(ρ²) < 1.
**Atom or composite:** Composite — non-coherent ensemble.
**Cost model:** O(N²) storage.
**Real wall?** No.
**Cross-domain wiring:** statistics-probability (ensemble average), quantum-computing (noise modeling).
**Notes:** Decoherence converts pure → mixed via environment entanglement.

### dirac-equation (cross-domain alias: `(iγ^μ∂_μ-m)ψ=0`, `relativistic-electron-eq`, `spinor-eq`)
**Domain:** Physics / Diffusion
**Definition:** (iℏγ^μ∂_μ − mc)ψ = 0 — relativistic wave equation for spin-1/2 fermions.
**Atom or composite:** Composite — Lorentz-covariant first-order PDE.
**Cost model:** Larger Hilbert space (4-spinors); FDTD/lattice O(N).
**Real wall?** Yes — predicts antiparticles, requires QFT for stability.
**Cross-domain wiring:** condensed-matter (graphene, topological insulators), quantum-computing.
**Notes:** Dirac (1928). Predicted positron, intrinsic spin emerges naturally.

### klein-gordon-equation (cross-domain alias: `(□+m²)ψ=0`, `relativistic-scalar-eq`, `spin-0-eq`)
**Domain:** Physics / Diffusion
**Definition:** (□ + m²c²/ℏ²)φ = 0 — relativistic wave equation for spin-0 fields.
**Atom or composite:** Atom — Lorentz-invariant.
**Cost model:** FDTD O(N) per step.
**Real wall?** Yes — negative-energy states require second quantization.
**Cross-domain wiring:** field theory (scalar fields), quantum-computing (Higgs analog).
**Notes:** Klein-Gordon (1926). Single-particle interpretation fails; resolved in QFT.

### perturbation-theory-time-indep (cross-domain alias: `Rayleigh-Schrodinger`, `RSPT`, `E_n^(k)-expansion`)
**Domain:** Physics / Diffusion
**Definition:** Expand E_n, |n⟩ in λ: E_n = E_n^(0) + λE_n^(1) + λ²E_n^(2) + ...; E_n^(1) = ⟨n^(0)|V|n^(0)⟩.
**Atom or composite:** Composite — series in V.
**Cost model:** Each order requires O(N²) sums.
**Real wall?** Yes — asymptotic, not convergent.
**Cross-domain wiring:** condensed-matter (Stark, Zeeman), quantum-computing (variational).
**Notes:** Rayleigh-Schrödinger (1926). Foundation of analytic spectroscopy.

### perturbation-theory-time-dep (cross-domain alias: `TDPT`, `Dyson-series`, `time-dep-V`)
**Domain:** Physics / Diffusion
**Definition:** Expand U(t) = T exp(-i/ℏ ∫V_I(t')dt') in V; first-order: P(i→f) = |⟨f|V|i⟩|² · (transition probability).
**Atom or composite:** Composite — Dyson series.
**Cost model:** O(N²) per order.
**Real wall?** Yes — convergence depends on |V|/E.
**Cross-domain wiring:** photonics-optics (atom-light), quantum-computing (driven gates).
**Notes:** Dirac (1927). Yields Fermi's golden rule for continuous spectra.

### degenerate-perturbation-theory (cross-domain alias: `DPT`, `secular-matrix-diagonalization`, `splitting-degeneracy`)
**Domain:** Physics / Diffusion
**Definition:** For degenerate E_n^(0), diagonalize V in degenerate subspace before applying RSPT.
**Atom or composite:** Composite — RSPT modification.
**Cost model:** Diagonalize d×d matrix in degenerate space.
**Real wall?** No (resolves the degeneracy issue).
**Cross-domain wiring:** linear-algebra-matrix (block diag), condensed-matter (Stark splitting).
**Notes:** Resolves vanishing-denominator issue. Example: Zeeman splitting of hydrogen.

### variational-principle-qm (cross-domain alias: `Rayleigh-Ritz`, `⟨ψ|H|ψ⟩≥E_0`, `upper-bound-on-E_0`)
**Domain:** Physics / Diffusion
**Definition:** For any trial |ψ⟩, ⟨ψ|H|ψ⟩/⟨ψ|ψ⟩ ≥ E_0 (ground-state energy).
**Atom or composite:** Atom — variational bound.
**Cost model:** Minimize over parameters; O(N²) per evaluation.
**Real wall?** No — bound is always valid.
**Cross-domain wiring:** quantum-computing (VQE algorithms), condensed-matter (Hartree-Fock).
**Notes:** Foundation of variational methods (HF, DFT, DMRG, VMC, VQE).

### wkb-approximation (cross-domain alias: `WKB`, `semiclassical-ansatz`, `ψ~exp(iS/ℏ)`)
**Domain:** Physics / Diffusion
**Definition:** Ansatz ψ(x) ~ A(x) exp(±i ∫p(x)dx/ℏ); valid when ℏ|dp/dx|/p² ≪ 1.
**Atom or composite:** Composite — semiclassical.
**Cost model:** O(N) integration.
**Real wall?** Yes — fails at classical turning points (Airy patching).
**Cross-domain wiring:** photonics-optics (eikonal analog), control-numerical-opt (semiclassical methods).
**Notes:** Wentzel-Kramers-Brillouin (1926). Bohr-Sommerfeld quantization arises here.

### born-approximation (cross-domain alias: `first-Born`, `weak-scattering`, `FT-of-potential`)
**Domain:** Physics / Diffusion
**Definition:** f^(1)(θ) = −(m/2πℏ²) ∫ V(r) e^{iq·r} d³r — scattering amplitude as FT of potential.
**Atom or composite:** Composite — first-order T-matrix.
**Cost model:** Single FFT O(N log N).
**Real wall?** Yes — fails for strong potentials, low energy.
**Cross-domain wiring:** signal-processing-rf (FT relation), photonics-optics (Rayleigh scattering).
**Notes:** Born (1926). Foundation of weak-scattering theory.

### partial-wave-analysis-qm (cross-domain alias: `PWA`, `phase-shifts-δ_l-QM`, `spherical-wave-decomp`)
**Domain:** Physics / Diffusion
**Definition:** ψ → e^{ikz} + f(θ) e^{ikr}/r expanded in Legendre P_l; f(θ) = (1/k) Σ (2l+1) e^{iδ_l} sin δ_l P_l(cos θ).
**Atom or composite:** Composite — angular momentum decomposition.
**Cost model:** Truncate at l_max ~ kR; O(l_max).
**Real wall?** No.
**Cross-domain wiring:** signal-processing-rf (multipole), photonics-optics (Mie).
**Notes:** Foundation of low-energy scattering analysis.

### optical-theorem-qm (cross-domain alias: `σ_tot=(4π/k)Imf(0)`, `unitarity-of-S`, `forward-amp-relation`)
**Domain:** Physics / Diffusion
**Definition:** σ_tot = (4π/k) Im f(0) — total cross-section from imaginary part of forward amplitude; follows from unitarity.
**Atom or composite:** Atom — consequence of S†S=I.
**Cost model:** O(1) given f(0).
**Real wall?** No.
**Cross-domain wiring:** signal-processing-rf (RCS), photonics-optics.
**Notes:** Foundation of consistency check in scattering calculations.

### fermis-golden-rule (cross-domain alias: `2π|V_fi|²/ℏ·ρ(E_f)`, `transition-rate-formula`, `FGR`)
**Domain:** Physics / Diffusion
**Definition:** Γ_{i→f} = (2π/ℏ) |⟨f|V|i⟩|² ρ(E_f) — transition rate to continuum of final states.
**Atom or composite:** Composite — TDPT first order with continuum.
**Cost model:** O(N) matrix element + density of states.
**Real wall?** Yes — fails for strong V or short times.
**Cross-domain wiring:** condensed-matter (electron-phonon), photonics-optics (spontaneous emission).
**Notes:** Dirac (1927), Fermi popularized. Foundation of decay rates and absorption.

### bohr-sommerfeld-quantization (cross-domain alias: `∮p dq=2πℏ(n+½)`, `EBK-quantization`, `semiclassical-quantization`)
**Domain:** Physics / Diffusion
**Definition:** ∮p dq = 2πℏ(n + ν/4) where ν = Maslov index counts turning points (½ for soft).
**Atom or composite:** Composite — quantization condition.
**Cost model:** O(1) per level given trajectory.
**Real wall?** Yes — only for integrable systems.
**Cross-domain wiring:** quantum-computing (semiclassical analog), photonics-optics (whispering gallery).
**Notes:** Bohr (1913), Sommerfeld (1916). Refined by Einstein-Brillouin-Keller for multi-D integrable.

---

## Field Theory & Other

### classical-field-theory (cross-domain alias: `field-as-DoF`, `infinite-DoF-mechanics`, `continuum-mechanics`)
**Domain:** Physics / Diffusion
**Definition:** Generalize Lagrangian mechanics to fields φ(x,t); EoM from δS = δ∫L d⁴x = 0 with Lagrangian density L.
**Atom or composite:** Composite — generalization of mechanics.
**Cost model:** PDE discretization O(N^d) per timestep.
**Real wall?** No.
**Cross-domain wiring:** electromagnetics-antennas (EM Lagrangian), astrophysics-cosmology.
**Notes:** Generalizes mechanics to ∞ DoF. Lifts to QFT after second quantization.

### lagrangian-density (cross-domain alias: `L(φ,∂_μφ)`, `field-Lagrangian`, `local-Lagrangian-density`)
**Domain:** Physics / Diffusion
**Definition:** L(φ,∂_μφ): scalar density such that S = ∫L d⁴x; EoM via δS=0.
**Atom or composite:** Atom — scalar function of fields and derivatives.
**Cost model:** Symbolic.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (path integral), electromagnetics-antennas.
**Notes:** Must be Lorentz scalar for relativistic theories.

### action-functional (cross-domain alias: `S[φ]`, `field-action`, `integral-of-L-d4x`)
**Domain:** Physics / Diffusion
**Definition:** S[φ] = ∫L d⁴x; physical fields extremize S; quantum mechanics path-integrate e^{iS/ℏ}.
**Atom or composite:** Atom — functional of field configurations.
**Cost model:** Single integral over spacetime.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (path integral weight), control-numerical-opt (action principle).
**Notes:** Foundation of quantum and classical field theory.

### noether-current (cross-domain alias: `j^μ`, `conserved-current`, `∂_μ j^μ=0`)
**Domain:** Physics / Diffusion
**Definition:** j^μ = (∂L/∂(∂_μφ)) δφ − F^μ for continuous symmetry; ∂_μj^μ = 0 on-shell.
**Atom or composite:** Composite — built from L and symmetry.
**Cost model:** Symbolic.
**Real wall?** No.
**Cross-domain wiring:** electromagnetics-antennas (charge current), astrophysics-cosmology (energy current).
**Notes:** Noether (1918). Charge Q = ∫j⁰ d³x conserved.

### canonical-energy-momentum-tensor (cross-domain alias: `T^μν-canonical`, `Noether-from-translations`, `canonical-Hilbert-tensor`)
**Domain:** Physics / Diffusion
**Definition:** T^μν = (∂L/∂(∂_μφ)) ∂^νφ − η^μν L — Noether current of spacetime translations.
**Atom or composite:** Composite — built from L.
**Cost model:** Symbolic.
**Real wall?** Yes — not always symmetric (spin fields).
**Cross-domain wiring:** astrophysics-cosmology (GR source), control-numerical-opt.
**Notes:** Symmetrized via Belinfante-Rosenfeld for spinors.

### belinfante-rosenfeld-tensor (cross-domain alias: `symmetric-T^μν`, `BR-improvement`, `symmetrized-stress-tensor`)
**Domain:** Physics / Diffusion
**Definition:** T^μν_BR = T^μν_canonical + ∂_λ S^[λμ]ν where S contains spin density; symmetric and conserved.
**Atom or composite:** Composite — improvement term.
**Cost model:** Symbolic.
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (GR coupling), condensed-matter (spin currents).
**Notes:** Belinfante (1939), Rosenfeld (1940). Couples to gravity properly.

### gauge-invariance (cross-domain alias: `local-symmetry`, `U(1),SU(N)-gauge`, `gauge-redundancy`)
**Domain:** Physics / Diffusion
**Definition:** Theory invariant under local symmetry ψ → e^{iα(x)}ψ; requires gauge field A_μ → A_μ + ∂_μα.
**Atom or composite:** Atom — local symmetry.
**Cost model:** Imposes constraints; lattice gauge theory O(N⁴) for QCD.
**Real wall?** Yes — gauge fixing needed in QFT.
**Cross-domain wiring:** quantum-computing (lattice QCD), electromagnetics-antennas (EM as U(1)).
**Notes:** Weyl (1929). Foundation of standard model.

### yang-mills (cross-domain alias: `non-abelian-gauge-theory`, `SU(N)-YM`, `chromodynamics-like`)
**Domain:** Physics / Diffusion
**Definition:** L_YM = −(1/4)F^a_μν F^{aμν} where F^a_μν = ∂_μA^a_ν − ∂_νA^a_μ + g f^{abc} A^b_μ A^c_ν.
**Atom or composite:** Composite — non-abelian generalization.
**Cost model:** Lattice QCD O(N⁴) very expensive.
**Real wall?** Yes — confinement nonperturbative.
**Cross-domain wiring:** quantum-computing (lattice gauge), astrophysics-cosmology (early universe).
**Notes:** Yang-Mills (1954). Backbone of standard model (QCD: SU(3); electroweak: SU(2)×U(1)).

### abelian-vs-non-abelian-gauge (cross-domain alias: `commutative-vs-non`, `U(1)-vs-SU(N)`, `linear-vs-nonlinear-gauge`)
**Domain:** Physics / Diffusion
**Definition:** Abelian (U(1)): gauge fields don't self-interact. Non-abelian (SU(N)): [T^a,T^b] = if^{abc}T^c, gauge fields self-interact via f^{abc}.
**Atom or composite:** Composite — classification.
**Cost model:** Non-abelian costs more in both perturbative and lattice calcs.
**Real wall?** Yes — non-abelian causes asymptotic freedom and confinement.
**Cross-domain wiring:** quantum-computing (gauge models), photonics-optics (analog photonic gauge).
**Notes:** Abelian: QED; Non-abelian: QCD, weak.

### spontaneous-symmetry-breaking (cross-domain alias: `SSB`, `mexican-hat-potential`, `vacuum-asymmetry`)
**Domain:** Physics / Diffusion
**Definition:** Hamiltonian symmetric, ground state breaks symmetry; ⟨φ⟩ ≠ 0 picks direction.
**Atom or composite:** Atom — mechanism.
**Cost model:** Conceptual; minimum of V(φ).
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (ferromagnetism, superconductivity), astrophysics-cosmology.
**Notes:** Continuous symmetry SSB → Goldstone bosons.

### goldstone-bosons (cross-domain alias: `nambu-goldstone-mode`, `massless-modes-from-SSB`, `phonons-analog`)
**Domain:** Physics / Diffusion
**Definition:** Massless excitations from spontaneous breaking of continuous global symmetry; one per broken generator.
**Atom or composite:** Composite — consequence of SSB.
**Cost model:** Linear dispersion ω = c_s k at low k.
**Real wall?** Yes — exact masslessness; pseudo-Goldstones get small mass from explicit breaking.
**Cross-domain wiring:** condensed-matter (phonons, magnons), quantum-computing.
**Notes:** Goldstone (1961). Pions are pseudo-Goldstones of chiral symmetry breaking.

### higgs-mechanism (cross-domain alias: `gauge-boson-mass`, `BEH-mechanism`, `mass-generation`)
**Domain:** Physics / Diffusion
**Definition:** Local gauge SSB: Goldstone "eaten" by gauge boson, which acquires mass m_A = gv where v is VEV.
**Atom or composite:** Composite — gauge + SSB.
**Cost model:** Conceptual; tree-level mass generation.
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (EW phase transition), quantum-computing (Higgs analog).
**Notes:** Brout-Englert, Higgs, Guralnik-Hagen-Kibble (1964). Discovered at LHC (2012).

### chiral-anomaly (cross-domain alias: `ABJ-anomaly`, `axial-current-non-conservation`, `triangle-diagram-anomaly`)
**Domain:** Physics / Diffusion
**Definition:** Classically conserved axial current j^5_μ violates conservation at quantum level: ∂_μj^{5μ} = (e²/16π²) ε^μναβ F_μν F_αβ.
**Atom or composite:** Composite — quantum effect.
**Cost model:** One-loop triangle diagram.
**Real wall?** Yes — anomalies must cancel in consistent gauge theories.
**Cross-domain wiring:** condensed-matter (Weyl semimetals), astrophysics-cosmology (baryogenesis).
**Notes:** Adler-Bell-Jackiw (1969). Explains π⁰ → γγ decay.

### spinor-fields (cross-domain alias: `ψ`, `dirac-spinor`, `weyl-spinor`)
**Domain:** Physics / Diffusion
**Definition:** Fields transforming in spinor representation of Lorentz group; 4-component Dirac, 2-component Weyl, Majorana.
**Atom or composite:** Composite — Lorentz rep.
**Cost model:** 4N or 2N values per spacetime point.
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (Weyl semimetals), quantum-computing (qubit analog).
**Notes:** Dirac (1928). Half-integer spin lifts via SL(2,ℂ) covering of Lorentz.

### dirac-lagrangian (cross-domain alias: `L_D=ψ̄(iγ^μ∂_μ-m)ψ`, `fermion-Lagrangian`, `Dirac-field-action`)
**Domain:** Physics / Diffusion
**Definition:** L = ψ̄(iγ^μ∂_μ − m)ψ; ψ̄ = ψ†γ⁰; gauge-covariantize via ∂_μ → D_μ = ∂_μ + ieA_μ.
**Atom or composite:** Composite — built from Dirac field.
**Cost model:** Symbolic.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (QED), condensed-matter (graphene effective theory).
**Notes:** Foundation of QED when coupled to A_μ.

### yukawa-coupling (cross-domain alias: `gψ̄ψφ`, `scalar-fermion-coupling`, `Yukawa-interaction`)
**Domain:** Physics / Diffusion
**Definition:** Interaction L_Y = −y ψ̄ψ φ between scalar φ and fermion ψ; gives mass m_ψ = yv after Higgs SSB.
**Atom or composite:** Atom — coupling term.
**Cost model:** Tree-level evaluation O(1).
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (electron-phonon analog), quantum-computing (atomic physics).
**Notes:** Yukawa (1935) for nuclear forces. Source of fermion masses in standard model.

### feynman-propagator (cross-domain alias: `G_F(x-y)`, `causal-propagator`, `time-ordered-propagator`)
**Domain:** Physics / Diffusion
**Definition:** G_F(x−y) = ⟨0|T φ(x)φ(y)|0⟩; in momentum space G̃_F(p) = i/(p²−m²+iε).
**Atom or composite:** Atom — building block of perturbation theory.
**Cost model:** Closed form; convolutions in QFT diagrams.
**Real wall?** No.
**Cross-domain wiring:** signal-processing-rf (Green's function), quantum-computing (Lehmann representation).
**Notes:** Feynman (1948). Causal: positive freq forward, negative freq backward.

### fine-structure (cross-domain alias: `α=e²/4πε₀ℏc`, `spin-orbit-corrections`, `relativistic-corrections`)
**Domain:** Physics / Diffusion
**Definition:** Splittings of hydrogen levels of order α² from relativistic + spin-orbit + Darwin terms.
**Atom or composite:** Composite — corrections to Schrödinger.
**Cost model:** Perturbation theory.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (atomic precision), photonics-optics (spectroscopy).
**Notes:** Sommerfeld (1916), full QED derivation later. α ≈ 1/137.036.

### hyperfine-structure (cross-domain alias: `nuclear-electron-coupling`, `21cm-line`, `Δm_I-splitting`)
**Domain:** Physics / Diffusion
**Definition:** Splittings from coupling of nuclear magnetic moment to electron magnetic moment; ~α⁴ corrections.
**Atom or composite:** Composite — hyperfine corrections.
**Cost model:** Perturbation theory.
**Real wall?** No.
**Cross-domain wiring:** astrophysics-cosmology (21cm cosmology), quantum-computing (atomic clocks).
**Notes:** Fermi contact term, dipolar coupling. 21cm hydrogen line is hyperfine.

### zeeman-effect (cross-domain alias: `magnetic-splitting`, `B-induced-splitting`, `Δm-splitting`)
**Domain:** Physics / Diffusion
**Definition:** Splitting of atomic levels in external B; weak field: ΔE = g_J m_J μ_B B; strong field: Paschen-Back.
**Atom or composite:** Composite — perturbation by B.
**Cost model:** Perturbation theory or direct diag.
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (NMR), quantum-computing (qubit splitting).
**Notes:** Zeeman (1896, Nobel 1902). Diagnostic for stellar magnetic fields.

### stark-effect (cross-domain alias: `electric-field-splitting`, `linear-quadratic-Stark`, `E-field-perturbation`)
**Domain:** Physics / Diffusion
**Definition:** Atomic level shifts in external E-field; linear for degenerate states (hydrogen), quadratic for non-degenerate.
**Atom or composite:** Composite — perturbation by E.
**Cost model:** DPT for hydrogen; RSPT otherwise.
**Real wall?** No.
**Cross-domain wiring:** photonics-optics (electro-optics), quantum-computing (charge-qubit tuning).
**Notes:** Stark (1913, Nobel 1919). Limits Rydberg atom storage time.

### lamb-shift (cross-domain alias: `2S_{1/2}-2P_{1/2}-split`, `radiative-correction`, `QED-shift`)
**Domain:** Physics / Diffusion
**Definition:** Splitting of hydrogen 2S_{1/2} and 2P_{1/2} (degenerate in Dirac eq) by ~1057 MHz from QED corrections.
**Atom or composite:** Composite — radiative correction.
**Cost model:** QED loop calculation.
**Real wall?** No.
**Cross-domain wiring:** quantum-computing (precision tests), photonics-optics (vacuum effects).
**Notes:** Lamb-Retherford (1947). Critical evidence for QED renormalization.

### lorenz-attractor (cross-domain alias: `Lorenz-system`, `butterfly-attractor`, `strange-attractor-prototype`)
**Domain:** Physics / Diffusion
**Definition:** dx/dt = σ(y−x), dy/dt = x(ρ−z)−y, dz/dt = xy−βz; chaotic for σ=10, ρ=28, β=8/3.
**Atom or composite:** Composite — 3D nonlinear ODE.
**Cost model:** RK4 integration O(N_steps).
**Real wall?** Yes — exponential sensitivity to ICs.
**Cross-domain wiring:** statistics-probability (chaotic dynamics), control-numerical-opt.
**Notes:** Lorenz (1963). Birth of chaos theory; "butterfly effect" coined here.

### period-doubling (cross-domain alias: `Feigenbaum-route`, `bifurcation-cascade`, `δ=4.669`)
**Domain:** Physics / Diffusion
**Definition:** Period of limit cycle doubles in series 2,4,8,... at parameter values converging geometrically with ratio δ ≈ 4.669.
**Atom or composite:** Composite — universal route to chaos.
**Cost model:** Bifurcation analysis O(N_params).
**Real wall?** No.
**Cross-domain wiring:** statistics-probability (chaos onset), photonics-optics (laser instabilities).
**Notes:** Feigenbaum (1978). δ is universal for unimodal maps.

### fractal-dimension (cross-domain alias: `Hausdorff-D`, `box-counting-D`, `correlation-D`)
**Domain:** Physics / Diffusion
**Definition:** D such that N(ε) ~ ε^{-D} for cover by ε-boxes; non-integer for fractal sets.
**Atom or composite:** Atom — geometric measure.
**Cost model:** Box-counting O(N log N).
**Real wall?** Yes — definition depends on chosen measure.
**Cross-domain wiring:** statistics-probability (random fractals), signal-processing-rf (1/f spectra).
**Notes:** Mandelbrot (1975). D > topological dimension for fractals.

### maxwell-thermodynamic-relations (cross-domain alias: `Maxwell-relations`, `mixed-partials-equal`, `thermo-cross-derivatives`)
**Domain:** Physics / Diffusion
**Definition:** From dU = TdS − pdV: (∂T/∂V)_S = −(∂p/∂S)_V; similarly from F, G, H.
**Atom or composite:** Composite — derived from potentials.
**Cost model:** O(1).
**Real wall?** No.
**Cross-domain wiring:** condensed-matter (specific heats), control-numerical-opt (process design).
**Notes:** Maxwell (1871). Four basic relations; experimentally relate hard-to-measure quantities to easy ones.
*Source doctrine: The Painted Fence — Jesse*
