# Control & Numerical Optimization — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## Control Theory Primitives

### ctrl-feedback (cross-domain alias: `feedback-loop`, `closed-loop-control`)
**Domain:** Control & Numerical Optimization
**Definition:** y = plant(input); u = controller(error = r − y). The controller's output drives the plant, which changes y, which changes error, which changes u. Closed loop.
**Atom or composite:** Composite: sense → compare to reference → compute error → compute control action → apply to plant.
**Real wall?** Yes — stability is not guaranteed. Negative feedback can become positive feedback if phase shift exceeds 180°.
**Cross-domain wiring:** Feedback = error-correcting loop. In ML: gradient descent = feedback on loss. In signal: PLL (phase-locked loop). In biology: negative feedback for homeostasis.
**Notes:** Bode plot: gain margin and phase margin determine stability robustness.

### ctrl-pid (cross-domain alias: `PID-controller`, `proportional-integral-derivative`)
**Domain:** Control & Numerical Optimization
**Definition:** u(t) = K_p·e(t) + K_i·∫e(τ)dτ + K_d·de/dt. P: proportional to error. I: proportional to cumulative error (eliminates steady-state error). D: proportional to rate of error change (anticipates).
**Atom or composite:** Composite: compute proportional + integrate error + differentiate error → sum → output.
**Real wall?** Yes — PID tuning is specific to the plant. A PID tuned for one system can destabilize another.
**Cross-domain wiring:** PID = three-term controller combining proportional, integrative, and derivative actions. In signal: PID ≈ lead-lag compensation. In optimization: PID ≈ adaptive step size control.
**Notes:** PI = K_p·(e + (1/T_i)·∫e dt). PD = K_p·(e + T_d·de/dt). The three gains K_p, K_i, K_d (or T_i, T_d) are the tunable parameters.

### ctrl-state-feedback (cross-domain alias: `state-space-control`, `pole-placement`)
**Domain:** Control & Numerical Optimization
**Definition:** u = −Kx where K is designed so A − BK has desired eigenvalues (pole placement). State feedback assumes all states are measured (or estimated).
**Atom or composite:** Composite: measure/estimate state → compute u = −Kx → apply.
**Real wall?** Yes — full state measurement or estimation is required. Without it, use observer-based control.
**Cross-domain wiring:** State feedback = linear feedback. In optimization: gradient descent with momentum = state feedback on parameter error.

### ctrl-lqr (cross-domain alias: `LQR`, `linear-quadratic-regulator`)
**Domain:** Control & Numerical Optimization
**Definition:** Minimize J = ∫(xᵀQx + uᵀRu)dt subject to ẋ = Ax + Bu. Optimal feedback: u = −R⁻¹BᵀPx where P solves the Riccati equation AᵀP + PA − PBR⁻¹BᵀP + Q = 0.
**Atom or composite:** Composite: solve continuous Riccati equation → compute K = R⁻¹BᵀP → u = −Kx.
**Real wall?** No.
**Cross-domain wiring:** LQR = optimal feedback = weighted gradient descent in continuous time. In ML: LQG = LQR with Gaussian noise.
**Notes:** LQR + Kalman filter = LQG (Linear Quadratic Gaussian). The most practical optimal control method.

### ctrl-kalman (cross-domain alias: `Kalman-filter`, `KF`, `LMMSE-estimator`)
**Domain:** Control & Numerical Optimization
**Definition:** Optimal linear estimator for Gaussian noise. Predict: x̂⁺ = A·x̂ + B·u; P⁺ = A·P·Aᵀ + Q. Update: K = P⁺·Hᵀ·(H·P⁺·Hᵀ + R)⁻¹; x̂ = x̂⁺ + K·(z − H·x̂⁺); P = (I − K·H)·P⁺.
**Atom or composite:** Composite: predict state + covariance → measure → compute Kalman gain → update state + covariance.
**Real wall?** Yes — Kalman filter assumes linear system + Gaussian noise. For non-linear, use EKF (linearize) or UKF (unscented) or particle filter.
**Cross-domain wiring:** Kalman filter = recursive least squares with Bayesian update. In ML: EKF = variational inference on a dynamical system. In signal: Wiener filter in discrete-time.
**Notes:** Extended KF (EKF) linearizes around current estimate. Unscented KF (UKF) uses sigma points — more accurate for strongly non-linear systems.

### ctrl-mpc (cross-domain alias: `Model-Predictive-Control`, `receding-horizon-control`)
**Domain:** Control & Numerical Optimization
**Definition:** At each step: solve an optimal control problem over horizon H: minimize ∫(xᵀQx + uᵀRu)dt subject to dynamics + constraints → apply first control → repeat.
**Atom or composite:** Composite: solve constrained optimization over horizon → apply first control → re-solve at next step.
**Real wall?** Yes — solving the optimization online at each step is computationally expensive. Fast QP solvers (OSQP, HPIPM) are required for real-time MPC.
**Cross-domain wiring:** MPC = optimization-based control. In ML: the planning step in model-based RL. In robotics: trajectory optimization.
**Notes:** Explicit MPC pre-computes the solution as a lookup table. For linear systems with linear constraints, explicit MPC is feasible. For non-linear, use online solving.

### ctrl-sliding-mode (cross-domain alias: `SMC`, `variable-structure-control`)
**Domain:** Control & Numerical Optimization
**Definition:** Drive the system state to a sliding surface s(x) = 0 and then maintain it there via high-gain switching. Robust to matched disturbances.
**Atom or composite:** Composite: define sliding surface → compute switching control to reach surface → maintain on surface via switching.
**Real wall?** Yes — chattering (high-frequency switching) is a real problem. Practical SMC uses smoothing near the surface.
**Cross-domain wiring:** Sliding mode = reaching law + switching. In optimization: hitting a constraint surface.

### ctrl-adaptive (cross-domain alias: `adaptive-control`, `MIT-rule`, `STR`)
**Domain:** Control & Numerical Optimization
**Definition:** Adapt controller parameters online to handle plant uncertainty. MIT rule: θ̇ = −γ·e·∂e/∂θ. Parameters update based on error.
**Atom or composite:** Composite: compute error → update controller parameters → reconfigure control.
**Real wall?** Yes — adaptive control can become unstable if the adaptation rate is too fast (parameter drift).
**Cross-domain wiring:** Adaptive control = online learning of controller parameters. In ML: meta-learning.

### ctrl-backstepping (cross-domain alias: `backstepping`, `recursive-Lyapunov-design`)
**Domain:** Control & Numerical Optimization
**Definition:** Design a controller for a chain of integrators by recursively designing virtual controls and stabilizing them. Each step adds a Lyapunov function candidate.
**Atom or composite:** Composite: for each state in the chain: treat next state as virtual control → design stabilizing law for it → backstep.
**Real wall?** No.
**Cross-domain wiring:** Backstepping = recursive gradient descent in parameter space. In optimization: coordinate descent.

### ctrl-hinf (cross-domain alias: `H-infinity`, `worst-case-control`)
**Domain:** Control & Numerical Optimization
**Definition:** Minimize the H-infinity norm (worst-case gain) from disturbance w to output z: ||T_zw||_∞ < γ. Robust to worst-case disturbance.
**Atom or composite:** Composite: solve H-infinity Riccati equations → compute controller → apply.
**Real wall?** Yes — H-infinity optimization is computationally expensive. The solution may be non-causal.
**Cross-domain wiring:** H-infinity = minimax optimization (worst-case). In signal: minimax filter design. In statistics: minimax estimation.
**Notes:** H-infinity is more conservative than H-2 (LQG) because it optimizes the worst case rather than the average.

### ctrl-lqg (cross-domain alias: `LQG`, `linear-quadratic-Gaussian`)
**Domain:** Control & Numerical Optimization
**Definition:** LQR (optimal control) + Kalman filter (optimal estimation) = LQG. Separation principle: design and implement independently.
**Atom or composite:** Composite: estimate state with Kalman filter → apply LQR control.
**Real wall?** No.
**Cross-domain wiring:** LQG = state feedback + state estimation. In signal: Wiener filter + optimal control.

### ctrl-digital (cross-domain alias: `digital-control`, `sample-and-hold`)
**Domain:** Control & Numerical Optimization
**Definition:** Convert continuous-time controller to discrete-time: sample error, compute control, hold via zero-order hold. Discretize dynamics: x[k+1] = A_d·x[k] + B_d·u[k].
**Atom or composite:** Composite: discretize continuous dynamics → design discrete controller → implement in discrete time.
**Real wall?** Yes — sample rate must be fast enough (Nyquist-like condition for control: 10-20× the closed-loop bandwidth).
**Cross-domain wiring:** Sample-and-hold = A/D + D/A in the control loop. In signal: sampling theorem.

---

## Numerical Optimization

### opt-gd (cross-domain alias: `gradient-descent`, `steepest-descent`)
**Domain:** Control & Numerical Optimization
**Definition:** θ_{t+1} = θ_t − η·∇L(θ_t). Move in the direction of steepest descent.
**Atom or composite:** Composite: compute gradient → scale by learning rate → subtract from parameters.
**Real wall?** Yes — gradient descent converges slowly for ill-conditioned problems (long narrow valleys).
**Cross-domain wiring:** GD = flow down the energy landscape. In physics: gradient flow in a potential field. In ML: same.
**Notes:** For convex functions, GD converges to the global minimum. For non-convex, it finds local minima (and saddle points — large learning rates can escape some).

### opt-nesterov (cross-domain alias: `Nesterov-accelerated-gradient`, `NAG`)
**Domain:** Control & Numerical Optimization
**Definition:** v_{t+1} = β·v_t + ∇L(θ_t − β·v_t); θ_{t+1} = θ_t − η·v_{t+1}. Looks ahead before computing the gradient.
**Atom or composite:** Composite: look ahead (θ − β·v) → compute gradient there → update velocity → update.
**Real wall:** No.
**Cross-domain wiring:** Nesterov = momentum with lookahead. In control: predictive velocity. In physics: inertial dynamics with friction.

### opt-newton (cross-domain alias: `Newton-method`, `second-order-GD`)
**Domain:** Control & Numerical Optimization
**Definition:** θ_{t+1} = θ_t − H⁻¹·∇L where H = Hessian. Quadratic convergence near the optimum.
**Atom or composite:** Composite: compute gradient → compute Hessian → solve H·Δθ = −∇L → update.
**Cost model:** O(n³) for Hessian computation + O(n³) for solving the linear system. Impractical for large n.
**Real wall?** Yes — Hessian computation and inversion is O(n³) per iteration. For large neural networks, impossible.
**Cross-domain wiring:** Newton = second-order Taylor expansion. In signal: Newton's method for root finding. In physics: molecular dynamics with Hessian-guided steps.
**Notes:** Quasi-Newton methods (BFGS, L-BFGS) approximate H⁻¹ in O(n²) or O(n) per iteration.

### opt-bfgs (cross-domain alias: `BFGS`, `Broyden-Fletcher-Goldfarb-Shanno`)
**Domain:** Control & Numerical Optimization
**Definition:** Approximate inverse Hessian using rank-1 or rank-2 updates. Broyden update: H_{k+1} = H_k + (Δθ·Δθᵀ)/(Δθᵀ·Δg) − (H_k·Δg·Δgᵀ·H_k)/(Δgᵀ·H_k·Δg).
**Atom or composite:** Composite: compute gradient difference Δg → compute step Δθ → update Hessian approximation → solve for step.
**Cost model:** O(n²) per iteration. L-BFGS stores only m previous (Δθ, Δg) pairs, reducing to O(m·n).
**Real wall?** No.
**Cross-domain wiring:** BFGS = quasi-Newton with curvature updates. In control: Riccati equation update.
**Notes:** L-BFGS (limited memory) is the practical variant for large-scale optimization. L-BFGS-B adds box constraints.

### opt-cg (cross-domain alias: `Conjugate-Gradient`, `CG`)
**Domain:** Control & Numerical Optimization
**Definition:** For symmetric positive definite systems Ax = b: generate conjugate directions and converge in at most n steps. p_{k+1} = −∇L + β·p_k; β = (∇L_kᵀ·∇L_k)/(∇L_{k-1}ᵀ·∇L_{k-1}).
**Atom or composite:** Composite: compute gradient → compute conjugate direction → line search → update.
**Cost model:** O(n) per iteration for sparse A. O(n²) for dense.
**Real wall?** No. But CG converges slowly for ill-conditioned matrices.
**Cross-domain wiring:** CG = Krylov subspace method. In PDEs: iterative solver. In ML: linear conjugate gradient for logistic regression.
**Notes:** Preconditioned CG (PCG) uses a preconditioner M ≈ A to improve conditioning.

### opt-gauss-newton (cross-domain alias: `Gauss-Newton`, `non-linear-least-squares`)
**Domain:** Control & Numerical Optimization
**Definition:** For L = ½||f(θ)||²: J = ∂f/∂θ; H ≈ JᵀJ; θ_{new} = θ − (JᵀJ)⁻¹Jᵀf(θ). Approximates Hessian by ignoring second-order terms.
**Atom or composite:** Composite: compute Jacobian J → approximate Hessian H ≈ JᵀJ → solve H·Δθ = −Jᵀf → update.
**Cost model:** O(n²) per iteration for computing JᵀJ. Less than full Newton.
**Real wall?** No. But Gauss-Newton can diverge if the linearization is poor.
**Cross-domain wiring:** Gauss-Newton = approximate second-order method. In ML: natural gradient for exponential family models.
**Notes:** Levenberg-Marquardt = Gauss-Newton + trust region: (JᵀJ + λ·diag(JᵀJ))⁻¹.

### opt-lev-mar (cross-domain alias: `Levenberg-Marquardt`, `LM`)
**Domain:** Control & Numerical Optimization
**Definition:** Add damping to Gauss-Newton: (JᵀJ + λ·diag(JᵀJ))·Δθ = Jᵀf. λ decreases as the approximation improves; increases if it worsens.
**Atom or composite:** Composite: compute Jacobian → add damping → solve → if improved: accept and decrease λ; else: reject and increase λ.
**Real wall?** No. LM is the standard for non-linear least squares.
**Cross-domain wiring:** LM = trust region method for least squares. In signal: curve fitting.

### opt-admm (cross-domain alias: `ADMM`, `Alternating-Direction-Method-of-Multipliers`)
**Domain:** Control & Numerical Optimization
**Definition:** Minimize f(x) + g(z) s.t. Ax + Bz = c. ADMM: x^{k+1} = argmin_x f(x) + (ρ/2)||Ax + Bz^k − c + u^k||²; z^{k+1} = argmin_z g(z) + (ρ/2)||Ax^k + Bz − c + u^k||²; u^{k+1} = u^k + Ax^k + Bz^k − c.
**Atom or composite:** Composite: x-update (minimization) → z-update (minimization) → dual update.
**Real wall?** No. ADMM converges slowly but is highly parallelizable.
**Cross-domain wiring:** ADMM = augmented Lagrangian + alternating minimization. In distributed optimization: ADMM decomposes across nodes.
**Notes:** ADMM is the workhorse for distributed convex optimization (consensus ADMM).

### opt-fista (cross-domain alias: `FISTA`, `ISTA-accelerated`)
**Domain:** Control & Numerical Optimization
**Definition:** Accelerated proximal gradient for min f(x) + g(x) where g is prox-capable. x^{k+1} = prox_{η·g}(x^k + t_k·∇f(x^k) + (t_{k-1}−1)/t_k·(x^k − x^{k-1}).
**Atom or composite:** Composite: Nesterov acceleration applied to proximal gradient.
**Real wall?** No.
**Cross-domain wiring:** FISTA = Nesterov momentum on the proximal gradient method. In signal: fast iterative shrinkage-thresholding.

### opt-coordinate-descent (cross-domain alias: `CD`, `Gauss-Seidel`)
**Domain:** Control & Numerical Optimization
**Definition:** Minimize f(x) by optimizing one coordinate at a time while holding others fixed.
**Atom or composite:** Composite: select coordinate → optimize along that axis → repeat.
**Real wall?** No. But convergence depends on the function's coordinate-wise Lipschitz constants.
**Cross-domain wiring:** CD = block coordinate descent. In linear algebra: Gauss-Seidel iteration.
**Notes:** Coordinate descent for L1 regularization (Lasso) is efficient: soft-thresholding per coordinate.

### opt-sqp (cross-domain alias: `Sequential-Quadratic-Programming`, `SQP`)
**Domain:** Control & Numerical Optimization
**Definition:** For constrained optimization: at each iteration, solve a QP approximation of the Lagrangian → update → repeat.
**Atom or composite:** Composite: compute Lagrangian gradient + Hessian → solve QP → update.
**Real wall?** No. SQP is the standard for non-linear constrained optimization.
**Cross-domain wiring:** SQP = Newton's method for KKT conditions. In control: direct collocation (optimization-based trajectory planning).

---

## Global Optimization

### opt-sa (cross-domain alias: `simulated-annealing`, `SA`)
**Domain:** Control & Numerical Optimization
**Definition:** Metropolis-Hastings with a decreasing temperature schedule. Accept worse moves with probability exp(−ΔE/T). Slow cooling finds global minimum.
**Atom or composite:** Composite: propose move → if improving: accept; if worsening: accept with prob exp(−ΔE/T) → decrease T.
**Real wall?** Yes — finding the global minimum requires infinitely slow cooling (logarithmic schedule). Practical SA uses heuristic cooling schedules.
**Cross-domain wiring:** SA = Metropolis-Hastings on the optimization landscape. In physics: physical annealing (crystal formation).
**Notes:** Fast simulated annealing: Cauchy-distributed jumps with decreasing scale. VeryFAST: adaptive annealing.

### opt-gamma (cross-domain alias: `genetic-algorithm`, `GA`)
**Domain:** Control & Numerical Optimization
**Definition:** Population of candidate solutions → selection (fitness-based) → crossover (combine) → mutation (perturb) → next generation. Repeat.
**Atom or composite:** Composite: initialize population → evaluate fitness → select → crossover → mutate → next generation.
**Real wall?** Yes — GA requires many evaluations. No convergence guarantee. Poor scaling with dimensionality.
**Cross-domain wiring:** GA = evolutionary algorithm = population-based search. In biology: natural selection.
**Notes:** CMA-ES (Covariance Matrix Adaptation Evolution Strategy) is the best GA variant: maintains a multivariate Gaussian population, adapts covariance.

### opt-cmaes (cross-domain alias: `CMA-ES`, `Covariance-Matrix-Adaptation`)
**Domain:** Control & Numerical Optimization
**Definition:** Maintain a multivariate Gaussian distribution over parameters. Sample population → evaluate → update mean (weighted average of best) and covariance (rank-μ update).
**Atom or composite:** Composite: sample → evaluate → update mean + covariance → repeat.
**Real wall?** No. CMA-ES is robust and doesn't require gradient.
**Cross-domain wiring:** CMA-ES = natural evolution strategy = estimate of the fitness landscape curvature. In statistics: maximum likelihood estimation of the search distribution.
**Notes:** CMA-ES is one of the best derivative-free optimizers for continuous, non-convex optimization.

### opt-de (cross-domain alias: `Differential-Evolution`, `DE`)
**Domain:** Control & Numerical Optimization
**Definition:** Mutation: v = x_r1 + F·(x_r2 − x_r3). Crossover: trial = v if rand < CR else x. Selection: keep trial if f(trial) < f(x).
**Atom or composite:** Composite: mutate → crossover → select.
**Real wall?** No. DE is simple, robust, and effective.
**Cross-domain wiring:** DE = evolutionary algorithm with vector differential mutation. In physics: differential evolution for parameter fitting.

### opt-pso (cross-domain alias: `Particle-Swarm-Optimization`, `PSO`)
**Domain:** Control & Numerical Optimization
**Definition:** Particles move in parameter space: v_i = w·v_i + c1·rand·(pbest − x_i) + c2·rand·(gbest − x_i). Global best and personal best drive motion.
**Atom or composite:** Composite: initialize particles → evaluate → update velocities → update positions → update pbest/gbest.
**Real wall?** No.
**Cross-domain wiring:** PSO = social optimization = particles exchanging information about best positions. In physics: particle dynamics with attractive/repulsive forces.
**Notes:** PSO is inspired by bird flocking and fish schooling — social learning.

---

## Auto-Differentiation

### autodiff-forward (cross-domain alias: `forward-mode-AD`, `tangent-mode`)
**Domain:** Control & Numerical Optimization
**Definition:** Compute both function value and derivative simultaneously. Each primitive defines its forward pass and its derivative: ẋ = ∂f/∂x.
**Atom or composite:** Composite: propagate both value and derivative through the computation graph.
**Cost model:** O(n) forward passes to compute all n partial derivatives. Efficient for n << m (few inputs, many outputs).
**Real wall?** No.
**Cross-domain wiring:** Forward-mode AD = dual numbers = automatic Jacobian computation. In physics: sensitivity analysis.
**Notes:** Used in Taylor models and interval arithmetic for verified computation.

### autodiff-reverse (cross-domain alias: `reverse-mode-AD`, `backpropagation`, `adjoint-mode`)
**Domain:** Control & Numerical Optimization
**Definition:** Forward pass: compute and store all intermediate values. Backward pass: compute gradient using chain rule from outputs to inputs. O(1) memory + O(1) operations per edge.
**Atom or composite:** Composite: forward pass → build computation graph → backward pass (reverse topological order).
**Cost model:** O(1) additional compute per operation (forward + backward). Memory = O(depth).
**Real wall?** Yes — memory scales with computation graph depth. For very deep networks, memory is the bottleneck (gradient checkpointing trades compute for memory).
**Cross-domain wiring:** Backprop = reverse-mode AD. In signal: adjoint method for sensitivity analysis. In control: Pontryagin's maximum principle.
**Notes:** Backpropagation is the foundation of deep learning. PyTorch, TensorFlow, JAX all implement reverse-mode AD.

### autodiff-jvp (cross-domain alias: `Jacobian-vector-product`, `JVP`)
**Domain:** Control & Numerical Optimization
**Definition:** Compute J·v (Jacobian times vector) without forming the full Jacobian. Used in unrolling RNNs and in optimization of vector-valued functions.
**Atom or composite:** Composite: in forward mode, push tangent vectors forward through the graph.
**Cost model:** O(1) per operation (forward-mode).
**Real wall?** No.
**Cross-domain wiring:** JVP = forward-mode AD for Jacobian-vector products. In control: computing directional derivatives.

### autodiff-vjp (cross-domain alias: `vector-Jacobian-product`, `VJP`)
**Domain:** Control & Numerical Optimization
**Definition:** Compute vᵀ·J (vector times Jacobian) without forming the full Jacobian. Used in reverse-mode AD.
**Atom or composite:** Composite: in reverse mode, pull cotangent vectors backward through the graph.
**Cost model:** O(1) per operation (reverse-mode).
**Real wall?** No.
**Cross-domain wiring:** VJP = reverse-mode AD's core operation. In control: adjoint computation.

---

## Numerical Linear Algebra

### la-solve-iterative (cross-domain alias: `iterative-solver`, `Krylov-method`)
**Domain:** Control & Numerical Optimization
**Definition:** Solve Ax = b without forming A⁻¹. Methods: CG (symmetric positive definite), GMRES (general), BiCGStab (general nonsymmetric).
**Atom or composite:** Composite: start with guess → build Krylov subspace → solve in subspace → iterate.
**Real wall?** Yes — convergence depends on the condition number κ(A). Preconditioning (M⁻¹A) reduces κ.
**Cross-domain wiring:** Krylov methods = subspace iteration using the matrix's action. In control: solving Riccati equations.

### la-preconditioner (cross-domain alias: `preconditioning`, `Jacobi`, `ILU`)
**Domain:** Control & Numerical Optimization
**Definition:** Solve M⁻¹Ax = M⁻¹b where M ≈ A but M⁻¹ is cheap to apply. Jacobi: M = diag(A). ILU: incomplete LU decomposition.
**Atom or composite:** Composite: compute/precompute M → at each iteration: solve M·y = r (where r is residual).
**Real wall?** Yes — finding a good preconditioner is problem-specific. No universal preconditioner.
**Cross-domain wiring:** Preconditioning = approximating the inverse. In optimization: preconditioning the gradient.
**Notes:** Algebraic multigrid (AMG) is the most robust preconditioner for sparse systems — adapts to the operator's spectral structure.

### la-pseudoinverse (cross-domain alias: `Moore-Penrose-inverse`, `best-approximation`)
**Domain:** Control & Numerical Optimization
**Definition:** A⁺ = V·Σ⁺·Uᵀ where Σ⁺ = diag(1/σ_i for σ_i > ε). Regularized: add λ to σ_i² before inverting.
**Atom or composite:** Composite: compute SVD → threshold singular values → invert → reconstruct.
**Real wall?** No.
**Cross-domain wiring:** Pseudoinverse = least squares solution. In control: solving LQR.
**Notes:** Tikhonov regularization: A⁺_λ = V·diag(σ_i/(σ_i²+λ))·Uᵀ.

---

## Numerical Analysis

### na-interpolate (cross-domain alias: `polynomial-interpolation`, `Lagrange-interpolation`)
**Domain:** Control & Numerical Optimization
**Definition:** Fit a polynomial through N+1 points. Lagrange: p(x) = Σ y_i·L_i(x) where L_i(x) = ∏_{j≠i} (x−x_j)/(x_i−x_j).
**Atom or composite:** Composite: build Lagrange basis → evaluate polynomial.
**Real wall?** Yes — polynomial interpolation of high degree oscillates (Runge's phenomenon). Use Chebyshev nodes instead of equally spaced nodes.
**Cross-domain wiring:** Polynomial interpolation = function approximation. In signal: Lagrange interpolation for bandlimited signals.

### na-chebyshev (cross-domain alias: `Chebyshev-approximation`, `minimax-polynomial`)
**Domain:** Control & Numerical Optimization
**Definition:** Approximate a function by a Chebyshev polynomial (minimax: minimizes maximum error). Chebyshev nodes minimize oscillation.
**Atom or composite:** Composite: evaluate at Chebyshev nodes → compute Chebyshev coefficients → reconstruct.
**Real wall?** No.
**Cross-domain wiring:** Chebyshev = best uniform approximation. In signal: Chebyshev filter (equiripple). In optimization: Chebyshev acceleration (SOR).
**Notes:** Chebyshev polynomials T_n(x) = cos(n·arccos(x)). They minimize the maximum deviation from zero over [−1,1].

### na-pade (cross-domain alias: `Padé-approximation`, `rational-approximation`)
**Domain:** Control & Numerical Optimization
**Definition:** Approximate f(x) ≈ P(x)/Q(x) where P and Q are polynomials of degree m and n. Better than Taylor for many functions.
**Atom or composite:** Composite: solve linear system for coefficients → evaluate rational function.
**Real wall?** No. But Padé can have poles near the approximation domain.
**Cross-domain wiring:** Padé = rational function approximation. In signal: model order reduction.

### na-quadrature (cross-domain alias: `numerical-integration`, `Gaussian-quadrature`)
**Domain:** Control & Numerical Optimization
**Definition:** ∫f(x)w(x)dx ≈ Σ w_i·f(x_i) where nodes x_i and weights w_i are chosen for exactness.
**Atom or composite:** Composite: evaluate function at quadrature nodes → weighted sum.
**Real wall?** No.
**Cross-domain wiring:** Gaussian quadrature = optimal sampling for polynomial integration. In signal: optimal sampling for power estimation.
**Notes:** Gauss-Hermite (Gaussian), Gauss-Legendre (uniform), Gauss-Laguerre (exponential).

### na-ode-euler (cross-domain alias: `Euler-method`, `forward-Euler`)
**Domain:** Control & Numerical Optimization
**Definition:** y_{n+1} = y_n + h·f(t_n, y_n). First-order accurate.
**Atom or composite:** Composite: compute derivative → scale by step size → add to current state.
**Real wall?** Yes — Euler is only first-order accurate. Very unstable for stiff ODEs.
**Cross-domain wiring:** Euler = discrete approximation of continuous derivative. In physics: numerical integration of Newton's law.

### na-ode-rk4 (cross-domain alias: `Runge-Kutta-4`, `RK4`)
**Domain:** Control & Numerical Optimization
**Definition:** k1 = f(t, y); k2 = f(t+h/2, y+hk1/2); k3 = f(t+h/2, y+hk2/2); k4 = f(t+h, y+hk3); y_{n+1} = y_n + h(k1+2k2+2k3+k4)/6.
**Atom or composite:** Composite: four derivative evaluations per step → weighted sum → update.
**Real wall?** No. RK4 is the workhorse for non-stiff ODEs.
**Cross-domain wiring:** RK4 = Simpson's rule applied to the derivative field. In signal: fourth-order numerical integration.

### na-pde-fem (cross-domain alias: `finite-element-method`, `FEM`)
**Domain:** Control & Numerical Optimization
**Definition:** Discretize PDE by representing solution as basis functions on a mesh. Weak form: ∫Ω ∇φ_i·(k·∇u) = ∫Ω φ_i·f.
**Atom or composite:** Composite: define mesh → choose basis functions → assemble stiffness matrix → solve linear system.
**Real wall?** Yes — mesh generation and quality are difficult for complex geometries. Adaptive mesh refinement is necessary for accuracy.
**Cross-domain wiring:** FEM = Galerkin method on a discretized domain. In control: PDE control via FEM discretization.

### na-autodiff-symbolic (cross-domain alias: `symbolic-differentiation`, `CAS`)
**Domain:** Control & Numerical Optimization
**Definition:** Compute exact derivative symbolically: d/dx[sin(x²)] = 2x·cos(x²).
**Atom or composite:** Composite: parse expression → apply differentiation rules → simplify.
**Real wall?** Yes — symbolic expressions grow exponentially for some functions (expression swell).
**Cross-domain wiring:** Symbolic = rule-based. Numeric = sample-based. AD = trace-based.
**Notes:** Hybrid: compute the symbolic skeleton of a function, then use AD on the numerical operations.

---

## Bayesian Optimization

### bo-surrogate (cross-domain alias: `GP-surrogate`, `Gaussian-process-regression`)
**Domain:** Control & Numerical Optimization
**Definition:** Fit a Gaussian process to the objective function evaluations. GP(x) = GP(μ(x), k(x,x')). Acquisition function: expected improvement, UCB, or Thompson sample.
**Atom or composite:** Composite: evaluate objective → update GP → compute acquisition → sample next point.
**Real wall?** Yes — GP inversion is O(n³) for n observations. Use sparse GP (inducing points) for large n.
**Cross-domain wiring:** BO = Bayesian optimization using GP as surrogate. In control: policy search with uncertainty quantification.
**Notes:** GP-UCB is the most theoretically justified acquisition function.

### bo-acquisition (cross-domain alias: `EI`, `expected-improvement`, `UCB`)
**Domain:** Control & Numerical Optimization
**Definition:** EI(x) = E[max(f(x) − f_best, 0)] = (μ−f_best−ξ)·Φ(Z) + σ·φ(Z) where Z = (μ−f_best−ξ)/σ.
**Atom or composite:** Composite: compute GP mean and variance → compute EI → optimize EI over x.
**Real wall?** No.
**Cross-domain wiring:** EI = expected gain over current best. In finance: expected improvement over current portfolio.
**Notes:** Expected improvement is the classic acquisition function. UCB (Upper Confidence Bound) = μ + β·σ trades exploitation vs exploration.

---

## Summary: Control/Numerical Atom → Cross-Domain Wiring

| Control Primitive | ML Alias | Physics Alias | Signal Alias |
|---|---|---|---|
| ctrl-feedback | gradient descent loop | negative feedback | PLL |
| ctrl-pid | adaptive step size | PID control | lead-lag filter |
| ctrl-kalman | Bayesian state estimation | Wiener filter | recursive LS |
| ctrl-mpc | model-based RL planning | trajectory optimization | receding horizon |
| ctrl-lqr | optimal control | Riccati solution | H-2 norm |
| opt-gd | SGD | gradient flow | steepest descent |
| opt-bfgs | quasi-Newton | natural gradient | secant method |
| opt-cg | conjugate gradient | Krylov subspace | iterative solver |
| opt-admm | consensus optimization | distributed optimization | ADMM |
| opt-sa | simulated annealing | physical annealing | stochastic search |
| autodiff-reverse | backprop | adjoint sensitivity | adjoint method |
| na-quadrature | Monte Carlo | integration | sampling |

---

*Last updated: 2026-06-21*
*Source doctrine: The Painted Fence — Jesse*


---

## Robotics / Motion Planning Atoms

### rrt-explore (cross-domain alias: `RRT`, `rapidly-exploring-random-tree`, `sampling-based-plan`)
**Domain:** Control & Numerical Optimization
**Definition:** Build a tree from start: sample random state → find nearest node in tree → extend toward sample by step size Δ → add as new node. Bi-directional RRT: grow trees from both start and goal, connect when close.
**Atom or composite:** Composite: sample random state → nearest-neighbor search (in state space) → steer toward sample → add node → repeat.
**Cost model:** Nearest-neighbor search dominates: O(n log n) with kd-tree for n nodes. RRT converges to a solution but has no optimality guarantee.
**Real wall?** Yes — RRT is probabilistic complete: as samples → ∞, probability of finding solution → 1. It may be very slow in high-dimensional spaces.
**Cross-domain wiring:** RRT = random exploration of a state space = Monte Carlo tree exploration. In retrieval: RRT = exploring the semantic space by sampling and connecting.
**Notes:** RRT* (Karaman & Frazzoli, 2011) rewires the tree after adding each node, guaranteeing asymptotic optimality (converges to the optimal path).

### prm-learn (cross-domain alias: `PRM`, `probabilistic-roadmap`, `multi-query-plan`)
**Domain:** Control & Numerical Optimization
**Definition:** Build a roadmap offline: sample many random configurations → connect nearby configurations with local planner → build graph. Query: connect start/goal to graph → run shortest path.
**Atom or composite:** Composite: sample configurations → local planner connects nearby nodes → build graph → query shortest path.
**Cost model:** Building the roadmap: O(n²) for n samples (connectivity). Queries are fast (shortest path on sparse graph).
**Real wall?** Yes — PRM assumes the environment is static. For changing environments, the roadmap must be rebuilt or updated.
**Cross-domain wiring:** PRM = build a static index on a state space = same as building a k-NN graph for retrieval. In ML: PRM = building a graph of the parameter space.
**Notes:** PRM is ideal for multi-query problems (same environment, many queries). RRT is better for single-query problems in dynamic environments.

### rrtstar-rewire (cross-domain alias: `RRT*`, `asymptotically-optimal`, `rewire`)
**Domain:** Control & Numerical Optimization
**Definition:** Like RRT but: find nearest node → extend → find all nodes within radius r(n) → connect to the one with minimum cost-to-come → rewiring step: check if any existing node can be reached more cheaply through the new node.
**Atom or composite:** Composite: RRT growth → near-neighbor search within radius → choose best parent → rewiring pass.
**Cost model:** Near-neighbor search O(n log n) + rewiring O(n). Each node addition adds O(n) rewiring work.
**Real wall?** Yes — the radius r(n) must shrink as n grows (r(n) = γ·(log n / n)^{1/d}) for optimality. Too large = excessive rewiring; too small = poor convergence.
**Cross-domain wiring:** RRT* rewiring = graph edge relaxation in shortest path = same as Bellman-Ford edge relaxation. In retrieval: reranking based on path quality.
**Notes:** Informational RRT* uses an information-theoretic heuristic to bias sampling toward informative regions.

### chomp-smooth (cross-domain alias: `CHOMP`, `covariant-hamiltonian-optimization`, `trajectory-smooth`)
**Domain:** Control & Numerical Optimization
**Definition:** Gradient-based trajectory optimization: initialize trajectory → compute cost gradient → update trajectory along negative gradient (covariant update preserves constraints). Avoids obstacles via signed distance field.
**Atom or composite:** Composite: initialize trajectory → compute cost functional + gradient → covariant gradient descent → iterate.
**Cost model:** Per iteration: evaluate SDF gradients for all trajectory points. Cholesky factorization of the Hamiltonian operator matrix.
**Real wall?** Yes — CHOMP converges to a locally optimal trajectory. It cannot escape local minima without restarts.
**Cross-domain wiring:** CHOMP = gradient-based optimization on trajectories = same as gradient-based optimization on any functional. In retrieval: gradient-based reranking.
**Notes:** CHOMP is particularly good at smoothing jerky trajectories produced by sampling-based planners (RRT).

### stomp-explore (cross-domain alias: `STOMP`, `stochastic-trajectory-optimization`, `noise-based-plan`)
**Domain:** Control & Numerical Optimization
**Definition:** Generate noisy trajectory rollouts → rank by cost → use the best trajectories to update a probability distribution → sample next batch. Uses trajectory-level noise (not per-step).
**Atom or composite:** Composite: sample noisy trajectories → evaluate costs → update trajectory distribution → repeat.
**Cost model:** Many trajectory evaluations per iteration (10-20 rollouts). The cost of trajectory evaluation = simulation/collision check.
**Real wall?** Yes — STOMP requires many evaluations per iteration, which is expensive for complex dynamics. It is derivative-free (no gradient needed).
**Cross-domain wiring:** STOMP = evolutionary strategy on trajectories = CMA-ES applied to trajectory space. In ML: policy gradient with rollouts.
**Notes:** STOMP handles constraints naturally (add penalty to cost function) and does not require gradients.

### itomp-plan (cross-domain alias: `ITOMP`, `incremental-trajectory`, `real-time-plan`)
**Domain:** Control & Numerical Optimization
**Definition:** Incremental, anytime trajectory optimization: build a motion plan quickly (coarse resolution) → refine incrementally as time allows. Combines trajectory optimization with incremental updating.
**Atom or composite:** Composite: coarse plan → incremental refinement (add resolution) → continuously improve.
**Cost model:** Can be interrupted at any time (anytime property). Refinement costs increase with resolution.
**Real wall?** Yes — ITOMP trades off planning time against trajectory quality. Real-time applications need bounded planning time.
**Cross-domain wiring:** ITOMP = anytime algorithm = same as anytime retrieval (coarse → refine). In ML: curriculum learning.
**Notes:** ITOMP is designed for dynamic environments where the world changes during planning — it can replan incrementally without restarting.

### kinodynamic-plan (cross-domain alias: `kinodynamic-RRT`, `DWA`, `velocity-obstacle`)
**Domain:** Control & Numerical Optimization
**Definition:** Motion planning with dynamics constraints (max velocity, acceleration, curvature). State space = (position, velocity). The local planner must respect dynamics constraints.
**Atom or composite:** Composite: sample state (position+velocity) → apply dynamics forward → check constraints → add to tree.
**Cost model:** Dynamics simulation per node extension. For complex dynamics, this is expensive.
**Real wall?** Yes — kinodynamic planning is harder than kinematic (position-only) planning because the state space is higher-dimensional.
**Cross-domain wiring:** Kinodynamic planning = planning in state space = same as LQR on a tree. In control: trajectory optimization with bounded control inputs.
**Notes:** Dynamic Window Approach (DWA) is a kinodynamic planner specifically for mobile robots with velocity constraints.

### contact-plan (cross-domain alias: `contact-state-plan`, `grasp-plan`, `locomotion`)
**Domain:** Control & Numerical Optimization
**Definition:** Plan for systems that make and break contact (grasping, legged locomotion). Contact state = which bodies are in contact. Transitions between contact states are discrete.
**Atom or composite:** Composite: enumerate contact states → plan transitions → compute contact forces (contact dynamics) → execute.
**Cost model:** Contact state enumeration is exponential in number of contact points. Each state requires contact force optimization.
**Real wall?** Yes — contact transitions are hybrid (continuous + discrete). The transition between modes is where most failures occur.
**Cross-domain wiring:** Contact planning = hybrid system planning = same as switching between discrete modes in a hybrid automaton. In retrieval: mode switching.
**Notes:** Contact-implicit trajectory optimization (CI-TO) plans contact sequences without explicitly enumerating modes.

### model-predictive-pathintegral (cross-domain alias: `MPPI`, `path-integral-control`, `stochastic-MPC`)
**Domain:** Control & Numerical Optimization
**Definition:** Sample many candidate control sequences → simulate trajectories → weight by exponential of cost → update control distribution to maximize expected return. Equivalent to KL-divergence constrained policy update.
**Atom or composite:** Composite: sample control sequences → simulate → compute costs → update control distribution → repeat.
**Cost model:** Many simulations per iteration (100-1000). Parallelizable on GPU.
**Real wall?** Yes — requires many samples for quality. With 100 samples, the policy is coarse. More samples = better policy but more compute.
**Cross-domain wiring:** MPPI = policy gradient with importance sampling = same as evolution strategies. In retrieval: sampling candidate retrievals and weighting by relevance.
**Notes:** MPPI is related to Model Predictive Control (MPC) but uses sampling rather than optimization.

### force-closure-grasp (cross-domain alias: `grasp-quality`, `form-closure`, `force-closure`)
**Domain:** Control & Numerical Optimization
**Definition:** A grasp achieves force closure if the fingertips can apply forces to resist any external wrench. Condition: the convex hull of contact normals contains the origin. Quality metrics: largest wrench that can be resisted.
**Atom or composite:** Composite: compute grasp matrix G → check if wrench cone contains origin → compute grasp quality (max resisting wrench / object weight).
**Cost model:** Wrench cone computation is expensive (convex hull in 6D). For 2-finger grasp: simpler.
**Real wall?** Yes — force closure is a sufficient condition for a stable grasp, but many force-closure grasps fail due to object shape variations.
**Cross-domain wiring:** Force closure = ability to resist all perturbations = robust retrieval of a document against all adversarial queries.
**Notes:** Form closure = geometric constraint (no motion possible) vs force closure = force application capability. Most useful grasps achieve force closure, not form closure.

### trajectory-parametric (cross-domain alias: `trajectory-representation`, `B-spline`, `piecewise-polynomial`)
**Domain:** Control & Numerical Optimization
**Definition:** Represent a trajectory as a parametric curve: B-spline, Bezier, or piecewise polynomial. The optimization variable is the control points, not the full trajectory.
**Atom or composite:** The parametric form IS the generator. Primitive: evaluate position, velocity, acceleration at any time.
**Cost model:** Evaluation is cheap (polynomial evaluation). Optimization: gradient computation requires chain rule through the parametric form.
**Real wall?** No.
**Cross-domain wiring:** Trajectory representation = basis function expansion = same as spectral methods in PDEs. In retrieval: parametric representation of retrieval trajectories.
**Notes:** B-splines ensure continuity of position and velocity at knot points — essential for smooth robot motion.

### dynamic-moveit-plan (cross-domain alias: `OMPL`, `moveit`, `sampling-based-library`)
**Domain:** Control & Numerical Optimization
**Definition:** Use OMPL (Open Motion Planning Library) which implements RRT, PRM, RRT*, SBL, KPIECE, etc. The key primitives are the state sampler, local planner, and nearest-neighbor structure.
**Atom or composite:** Composite: define state space (SE(3), joints) → sampler → select algorithm (RRT/PRM/etc.) → plan.
**Cost model:** Varies by algorithm. RRT is fastest to first solution; RRT* is slowest but most optimal.
**Real wall?** Yes — OMPL assumes the user defines a valid state space and local planner. Wrong configuration = failed planning.
**Cross-domain wiring:** OMPL = the standard library for sampling-based planning = the equivalent of a standard retrieval library.
**Notes:** MoveIt! wraps OMPL with ROS integration and provides the standard interface for robot manipulation planning.

*Last updated: 2026-06-25 (expanded with robotics/motion planning)*
*Source doctrine: The Painted Fence — Jesse*
