# Statistics & Probability Domain Primitives
> Cross-domain wiring: likelihood = project (data → parameter space); posterior = fold-update (prior × likelihood); MCMC = Markov chain over state space; EM = alternating maximization; mixture model = combine of components

## 1. Parameter Estimation

### [PRIM-001] maximum-likelihood-estimation
- **Atom/Composite:** Composite
- **Definition:** MLE: choose parameter θ to maximize likelihood L(θ; x) = P(x | θ). Equivalently maximize log-likelihood ℓ(θ; x) = Σ log P(xᵢ | θ). Consistent (as n → ∞) and asymptotically efficient (lowest variance among unbiased estimators).
- **Cost Model:** Optimization problem; closed form for exponential families; otherwise numerical optimization (Newton-Raphson, EM when applicable). O(n) per likelihood evaluation.
- **Real Wall:** Non-convex likelihoods → multiple local maxima; flat likelihood → high variance; regularization may be needed.
- **Cross-Domain Aliases:** likelihood-maximize (ml-training), parameter-estimate (control-numerical-opt).
- **Notes:** Fisher (1922); asymptotic normality: θ̂_MLE ~ N(θ, I(θ)⁻¹/n); Fisher information I(θ) = Var[∂ℓ/∂θ].

### [PRIM-002] bayesian-posterior-inference
- **Atom/Composite:** Primitive
- **Definition:** Bayesian inference: compute posterior p(θ | x) = p(x | θ)·p(θ) / p(x). Prior p(θ) encodes domain knowledge; posterior combines data + prior; predictive posterior p(x_new | x) = ∫ p(x_new | θ)·p(θ | x) dθ.
- **Cost Model:** Normalizing constant p(x) = ∫ p(x | θ)·p(θ) dθ: often intractable (high-dimensional θ); requires approximation (MCMC, variational, Laplace).
- **Real Wall:** Prior sensitivity: posterior can be prior-dominated if data is sparse; Jeffreys prior = uninformative; conjugate priors simplify.
- **Cross-Domain Aliases:** bayesian-update (ml-training), posterior-compute (information-theory-coding).
- **Notes:** Requires specifying prior; full posterior vs. MAP (maximum a posteriori) which discards posterior shape.

### [PRIM-003] maximum-a-posteriori
- **Atom/Composite:** Primitive
- **Definition:** MAP: θ_MAP = argmax_θ p(θ | x) = argmax_θ [p(x | θ)·p(θ)]. Equivalent to MLE with log-prior as regularizer. Not a proper Bayesian estimate (no uncertainty quantification).
- **Cost Model:** Same optimization as MLE plus log-prior term; if prior is L2-regularizer (Gaussian), MAP = ridge regression; L1 (Laplace prior) = LASSO.
- **Real Wall:** Ignores posterior variance (misleading confidence); not invariant to reparameterization; for proper uncertainty quantification, use full posterior.
- **Cross-Domain Aliases:** penalized-mle (ml-training), regularized-estimate (control-numerical-opt).
- **Notes:** Useful for regularized optimization; point estimate preferred when prior is informative and computation is constrained.

### [PRIM-004] method-of-moments
- **Atom/Composite:** Primitive
- **Definition:** Method of moments: equate sample moments E[X^k] to population moments; solve for parameters. Simpler than MLE but less efficient (higher variance).
- **Cost Model:** Solve system of equations (one per parameter); moments may not exist (heavy tails); computationally cheap.
- **Real Wall:** Can be inconsistent (moments don't converge to true values); fails for distributions without moments; less efficient than MLE.
- **Cross-Domain Aliases:** moment-matching (control-numerical-opt), empirical-moment-equate (information-theory-coding).
- **Notes:** Historical method (Pearson, 1894); used as initialization for MLE; connection to GMM (generalized method of moments).

### [PRIM-005] bias-variance-decomposition
- **Atom/Composite:** Primitive
- **Definition:** Expected prediction error = Bias² + Variance + Irreducible noise. Bias = E[θ̂] - θ_true; Variance = E[(θ̂ - E[θ̂])²]. Trade-off: complex models (low bias, high variance) vs. simple models (high bias, low variance).
- **Cost Model:** Estimate bias and variance via bootstrap or analytical computation; MSE = Bias² + Variance.
- **Real Wall:** Bias-variance decomposition holds for squared error; other losses have different decompositions; for classification, calibration matters.
- **Cross-Domain Aliases:** model-complexity-tradeoff (ml-training), mse-decomposition (control-numerical-opt).

### [PRIM-006] empirical-risk-minimization
- **Atom/Composite:** Composite
- **Definition:** ERM: minimize average loss over training data: θ̂ = argmin_θ (1/n) Σ L(f(xᵢ; θ), yᵢ) + λ·R(θ). Structural risk minimization adds complexity penalty; VC dimension controls generalization.
- **Cost Model:** Optimization over empirical distribution; SGD converges to global minimum for convex losses; non-convex losses = local minima.
- **Real Wall:** Overfitting (low train error, high test error) if model too expressive; regularization (λ) trades bias/variance; early stopping prevents overfitting in iterative methods.
- **Cross-Domain Aliases:** loss-minimization (ml-training), risk-empirical (control-numerical-opt).
- **Notes:** Vapnik-Chervonenkis theory: bounds generalization error via VC dimension; ERM principle = foundation of statistical learning theory.

### [PRIM-007] bootstrap-resampling
- **Atom/Composite:** Primitive
- **Definition:** Bootstrap: resample n observations with replacement B times; compute statistic on each resample; use distribution of bootstrap statistics to estimate standard error, confidence intervals, bias.
- **Cost Model:** B resamples × O(n) per statistic; parametric bootstrap (sample from fitted model) more efficient; bias-corrected accelerated (BCa) bootstrap.
- **Real Wall:** Small samples: bootstrap distributions are poor approximations; dependent data (time series) requires block bootstrap; pivot vs. percentile bootstrap.
- **Cross-Domain Aliases:** resampling-estimation (ml-training), uncertainty-resample (control-numerical-opt).

### [PRIM-008] jackknife-estimation
- **Atom/Composite:** Primitive
- **Definition:** Jackknife: leave-one-out resampling; n estimates θ̂_(-i) with each observation omitted; bias estimate = (n-1)(θ̄ - θ̂); variance estimate = ((n-1)/n) Σ(θ̂_(-i) - θ̄)².
- **Cost Model:** O(n) estimates, each computed from n-1 observations; computationally cheaper than bootstrap for small n.
- **Real Wall:** Jackknife fails for non-smooth statistics (median, mode); biased estimators can give unstable jackknife results.
- **Cross-Domain Aliases:** leave-one-out-estimation (ml-training), bias-correction (control-numerical-opt).

### [PRIM-009] cross-validation
- **Atom/Composite:** Composite
- **Definition:** CV: split data into k folds; train on k-1 folds, validate on remaining fold; repeat k times; average validation error. Variants: LOOCV (n folds), stratified CV, time-series CV (rolling window).
- **Cost Model:** k-fold CV: k training runs; computational cost = k × cost of single training; nested CV for model selection.
- **Real Wall:** Small datasets: k small (3–5); imbalanced classes: stratified sampling needed; time-series CV must respect temporal order (no random splits).
- **Cross-Domain Aliases:** holdout-validation (ml-training), hyperparameter-select (control-numerical-opt).

### [PRIM-010] bayesian-information-criterion
- **Atom/Composite:** Primitive
- **Definition:** BIC = -2·ℓ(θ̂_MLE) + k·log n. Lower BIC = better model. Penalizes complexity more heavily than AIC (which uses 2k). BIC is consistent (converges to true model as n → ∞ if true model is in candidate set).
- **Cost Model:** Requires MLE (log-likelihood); k = number of parameters; n = sample size.
- **Real Wall:** BIC assumes true model is in candidate set (not always); for model comparison, requires same data; DIC (Deviance Information Criterion) for hierarchical models.
- **Cross-Domain Aliases:** model-selection-criterion (ml-training), complexity-penalty (information-theory-coding).

## 2. Expectation Maximization

### [PRIM-011] expectation-maximization
- **Atom/Composite:** Composite
- **Definition:** EM algorithm: iterative optimization for latent variable models. E-step: compute Q(θ; θ^(t)) = E_{Z|X,θ^(t)}[log p(X, Z | θ)] (expected log-likelihood given current parameters). M-step: maximize Q → new θ^(t+1).
- **Cost Model:** Converges to (local) maximum of observed likelihood; monotonic increase of likelihood; convergence rate depends on proportion of observed vs. latent information.
- **Real Wall:** Only finds local maxima (run from multiple initializations); can be slow if latent space is large; initialization matters greatly.
- **Cross-Domain Aliases:** latent-variable-iteration (ml-training), hidden-state-estimate (control-numerical-opt).
- **Notes:** Dempster-Laird-Rubin (1977); used in Gaussian Mixture Models, Hidden Markov Models, factor analysis, k-means clustering.

### [PRIM-012] em-convergence-criterion
- **Atom/Composite:** Primitive
- **Definition:** EM convergence: stop when |ℓ^(t+1) - ℓ^(t)| / |ℓ^(t)| < ε, or when change in parameters ‖θ^(t+1) - θ^(t)‖ < ε, or after fixed max iterations.
- **Cost Model:** Checking convergence costs O(1) per iteration; compute observed log-likelihood requires summation over data (expensive); parameter change is cheaper proxy.
- **Real Wall:** Slow convergence near optimum (linear convergence rate); can get stuck on plateaus; Aitken acceleration predicts asymptotic limit and speeds convergence.

### [PRIM-013] gaussian-mixture-model
- **Atom/Composite:** Composite
- **Definition:** GMM: p(x) = Σ πₖ · N(x; μₖ, Σₖ). Fit via EM: E-step = compute posterior probability that observation xᵢ belongs to component k; M-step = re-estimate π, μ, Σ from responsibilities.
- **Cost Model:** E-step: O(n·k·d) for n observations, k components, d dimensions; M-step: O(n·k·d²) for covariance estimation.
- **Real Wall:** Number of components k must be chosen (BIC, AIC); singular covariances (preventable by regularization); non-convex optimization.
- **Cross-Domain Aliases:** mixture-density (ml-training), soft-clustering (retrieval-search).
- **Notes:** Used for density estimation, clustering, background subtraction; Variational Bayesian GMM (VBGMM) for automatic k selection.

### [PRIM-014] hidden-markov-model
- **Atom/Composite:** Composite
- **Definition:** HMM: sequence of hidden states Z₁:T with transition matrix A; observed emissions X_t | Z_t ~ B(z_t). Three problems: (1) likelihood (forward algorithm), (2) decoding (Viterbi), (3) learning (Baum-Welch = EM for HMM).
- **Cost Model:** Forward/backward: O(T·K²) where T = sequence length, K = states; Viterbi: O(T·K²); Baum-Welch: O(T·K²) per EM iteration.
- **Real Wall:** Number of states must be specified; large state spaces (K = 10⁴–10⁶) require approximate inference (particle filters, variational).
- **Cross-Domain Aliases:** latent-sequence-model (ml-training), state-sequence-decode (control-numerical-opt).
- **Notes:** Baum-Welch = special case of EM for HMMs; used in speech recognition, bioinformatics (CpG islands), finance.

### [PRIM-015] factor-analysis
- **Atom/Composite:** Composite
- **Definition:** Factor analysis: p = W·z + μ + ε, where z ∈ R^k (k << d) is latent factor, W ∈ R^{d×k} is factor loading matrix, ε ~ N(0, Ψ). Identified up to rotation (W·R for any orthogonal R).
- **Cost Model:** MLE for W, Ψ via EM; uniqueness problem (rotation invariance); typically use rotation (Varimax) to get interpretable factors.
- **Real Wall:** Number of factors k must be chosen; rotation is post-hoc (arbitrary); identifiability constraints needed for unique MLE.
- **Cross-Domain Aliases:** latent-factor-model (ml-training), dimensionality-reduction (linear-algebra-matrix).

### [PRIM-016] variational-em
- **Atom/Composite:** Composite
- **Definition:** Variational EM: when exact E-step is intractable, use variational inference to approximate Q(Z) with a simpler family (mean field: Q(Z) = Π qᵢ(Zᵢ)). Lower bound on log-likelihood; alternate between V-E and M.
- **Cost Model:** Tractability of Q-family determines cost; mean field: O(n·k) per iteration; factorized distributions may be poor approximations.
- **Real Wall:** Variational approximation introduces bias (lower bound, not exact); overpruning (approximation too simple); selection of Q-family is art.
- **Cross-Domain Aliases:** variational-inference (ml-training), approximate-em (control-numerical-opt).

### [PRIM-017] classification-em
- **Atom/Composite:** Primitive
- **Definition:** EM with partially observed class labels (semi-supervised): some xᵢ have observed labels yᵢ (supervised), others are unlabeled. E-step uses both labeled data and model-predicted labels for unlabeled data.
- **Cost Model:** Similar to standard EM; unlabeled data helps estimate cluster structure, especially when labels are expensive.
- **Real Wall:** Model misspecification: unlabeled data can hurt if mixture components don't align with true classes; self-training bias (confirmation of initial model).
- **Cross-Domain Aliases:** semi-supervised-em (ml-training), label-propagation (retrieval-search).

## 3. Bayesian Computation

### [PRIM-018] markov-chain-monte-carlo
- **Atom/Composite:** Composite
- **Definition:** MCMC: construct a Markov chain whose stationary distribution = target posterior p(θ | x); sample from chain after burn-in; ergodic theorem: time average → ensemble average.
- **Cost Model:** Autocorrelation time τ determines effective sample size (ESS = n / τ); mixing time governs burn-in length; convergence diagnosis: Geweke, Gelman-Rubin (R̂).
- **Real Wall:** Convergence diagnostics are necessary; chain can get stuck in modes (poor mixing); burn-in discarding wastes samples.
- **Cross-Domain Aliases:** posterior-sampling (ml-training), markov-chain-approx (control-numerical-opt).
- **Notes:** Metropolis-Hastings (1953); Gibbs sampling (Geman & Geman 1984); Hamiltonian Monte Carlo (Duane et al. 1987); MALA, NUTS.

### [PRIM-019] metropolis-hastings
- **Atom/Composite:** Composite
- **Definition:** MH algorithm: propose θ* ~ q(θ* | θ^(t)) (proposal), accept with probability α = min(1, p(θ*|x)·q(θ^(t)|θ*) / [p(θ^(t)|x)·q(θ*|θ^(t))]). Detailed balance ensures stationary = target.
- **Cost Model:** One likelihood evaluation per iteration; acceptance rate tuning (0.234 optimal for random-walk Gaussian proposal in high dimensions); expensive likelihood = slow chain.
- **Real Wall:** Random-walk proposals scale poorly with dimension; acceptance rate drops exponentially in high dimensions; adaptive proposals (tuned during burn-in) improve mixing.
- **Cross-Domain Aliases:** mcmc-acceptance (ml-training), proposal-acceptance (control-numerical-opt).

### [PRIM-020] gibbs-sampler
- **Atom/Composite:** Composite
- **Definition:** Gibbs sampling: sample each coordinate θᵢ from its full conditional p(θᵢ | θ_{-i}, x). Special case of MH where acceptance probability = 1. Requires ability to sample from all conditionals.
- **Cost Model:** O(k) coordinate samples per iteration (k = dimension); requires analytic form of all conditionals (conjugate priors make this easy); blocked Gibbs samples groups of coordinates.
- **Real Wall:** Highly correlated coordinates → slow mixing (Gibbs samples walk slowly); blocked Gibbs reduces autocorrelation; collapsed Gibbs integrates out some variables.
- **Cross-Domain Aliases:** coordinate-conditional-sample (ml-training), conditional-sampler (control-numerical-opt).
- **Notes:** Geman & Geman (1984) for image restoration; used in Bayesian mixture models, hierarchical models, LDA topic modeling.

### [PRIM-021] hamiltonian-monte-carlo
- **Atom/Composite:** Composite
- **Definition:** HMC: augment θ with momentum r; simulate Hamiltonian dynamics (dθ/dt = ∇_θ H, dr/dt = -∇_r H) for L steps of size ε; use Metropolis accept on final state. Uses gradient information for efficient exploration.
- **Cost Model:** Gradient evaluation per trajectory (O(gradient_cost) · L steps); leapfrog integrator; no U-turn termination (NUTS = No-U-Turn Sampler, Hoffman & Gelman 2014).
- **Real Wall:** Requires gradient of log-posterior (automatic differentiation); leapfrog step size ε and number of steps L must be tuned; NUTS automates L selection.
- **Cross-Domain Aliases:** gradient-mcmc (ml-training), hamiltonian-dynamics (control-numerical-opt).
- **Notes:** Neal (2011) review; Stan uses NUTS; HMC explores much faster than random-walk MH in high dimensions.

### [PRIM-022] slice-sampling
- **Atom/Composite:** Primitive
- **Definition:** Slice sampling: sample uniformly from region under unnormalized density f(θ) = p(θ|x); 1D: find interval around current θ, shrink via stepping out; higher dimensions: via conditional distributions.
- **Cost Model:** One log-density evaluation per sample; step size and stepping-out window are tuning parameters; automatic step size adaptation possible.
- **Real Wall:** Can be slow if density is narrow; stepping-out window too large → many rejected samples; shrinking interval.
- **Cross-Domain Aliases:** uniform-under-density (ml-training), region-sampling (control-numerical-opt).

### [PRIM-023] particle-filter
- **Atom/Composite:** Composite
- **Definition:** Sequential Monte Carlo: approximate posterior p(θ_{1:t} | x_{1:t}) with weighted particles; proposal = prior transition; resample when ESS drops. Bootstrap particle filter, auxiliary particle filter.
- **Cost Model:** O(n_particles · T) for T time steps; ESS = (Σ wᵢ)² / Σ wᵢ²; resampling when ESS < threshold.
- **Real Wall:** Particle degeneracy (most weight on few particles); proposal may be far from likelihood (use auxiliary PF); smoothing (full posterior over trajectory) is expensive.
- **Cross-Domain Aliases:** sequential-importance-resampling (ml-training), filter-state-approx (control-numerical-opt).
- **Notes:** Gordon et al. (1993); used in tracking, robotics (SLAM), time series; Kalman filter is the linear-Gaussian special case.

### [PRIM-024] variational-inference
- **Atom/Composite:** Composite
- **Definition:** VI: approximate intractable posterior p(θ | x) with a simpler distribution q(θ; φ). Minimize KL(q || p) = E_q[log(q/p] = ELBO (Evidence Lower Bound). ELBO = E_q[log p(x, θ)] + H(q).
- **Cost Model:** Optimization over φ (stochastic gradient VI uses reparameterization trick); O(1) gradient estimators per iteration; faster than MCMC but biased.
- **Real Wall:** VI underestimates posterior variance (mode-seeking); mean-field assumption restricts independence; normalizing flows can increase expressiveness.
- **Cross-Domain Aliases:** posterior-approx (ml-training), variational-lower-bound (information-theory-coding).
- **Notes:** Blei et al. (2017) VI review; used for large-scale Bayesian inference (topic models, VAEs, probabilistic programming).

### [PRIM-025] laplace-approximation
- **Atom/Composite:** Primitive
- **Definition:** Approximate p(θ | x) ≈ N(θ̂_MAP, I(θ̂_MAP)⁻¹) where θ̂_MAP is the MAP estimate and I is the Hessian of -log posterior. Fast (requires only MAP + Hessian).
- **Cost Model:** One optimization to find MAP + Hessian computation (O(d²) for d parameters); Laplace approximation of marginal likelihood (BIC approximation).
- **Real Wall:** Accurate only when posterior is approximately Gaussian (large n, smooth model); poor for multimodal or skewed posteriors.
- **Cross-Domain Aliases:** gaussian-approx (ml-training), normal-approx (control-numerical-opt).
- **Notes:** Tierney & Kadane (1986); used for Bayesian model selection, marginal likelihood approximation; Laplace for neural network priors (weights → Gaussian around MAP).

## 4. Hypothesis Testing & Decision Theory

### [PRIM-026] likelihood-ratio-test
- **Atom/Composite:** Composite
- **Definition:** LRT: reject H₀ when λ(x) = sup_θ∈Θ₀ L(θ;x) / sup_θ∈Θ₁ L(θ;x) < c. By Neyman-Pearson lemma, LRT is most powerful test for simple H₀ vs. H₁. Asymptotic χ² distribution: -2 log λ(x) ~ χ²_{df}.
- **Cost Model:** Two MLEs (under H₀ and H₁) + log-likelihood evaluation; degrees of freedom = difference in parameter counts.
- **Real Wall:** Large sample approximation may be poor for small n; multiple testing (adjust p-values via Bonferroni, FDR); nuisance parameters require profile likelihood.
- **Cross-Domain Aliases:** neyman-pearson-test (information-theory-coding), hypothesis-ratio (control-numerical-opt).

### [PRIM-027] wald-test
- **Atom/Composite:** Primitive
- **Definition:** Wald test: test restriction θ = θ₀ using estimate θ̂ and its variance: W = (θ̂ - θ₀)ᵀ · I(θ̂) · (θ̂ - θ₀) ~ χ²_1 asymptotically. Equivalent to t-test in 1D.
- **Cost Model:** O(1) after MLE + Fisher information estimate; works with any asymptotically normal estimator.
- **Real Wall:** Requires consistent estimator of information; Wald test is not invariant to parameterization (same test expressed in different params gives different results).
- **Cross-Domain Aliases:** asymptotic-normality-test (information-theory-coding), score-test (control-numerical-opt).

### [PRIM-028] score-test
- **Atom/Composite:** Primitive
- **Definition:** Score test: under H₀, U(θ₀) = ∂ℓ/∂θ|_{θ=θ₀} has mean 0; test statistic S = U(θ₀)ᵀ · I(θ₀)⁻¹ · U(θ₀) ~ χ²_df. Only requires MLE under H₀ (unlike Wald which needs unrestricted MLE).
- **Cost Model:** Score function + information evaluated at θ₀; useful when unrestricted MLE is hard but restricted MLE is easy.
- **Real Wall:** Requires regularity conditions (information matrix finite); less commonly used than Wald or LRT but sometimes computationally easier.
- **Cross-Domain Aliases:** efficient-score-test (information-theory-coding), constrained-mle (control-numerical-opt).

### [PRIM-029] permutation-test
- **Atom/Composite:** Primitive
- **Definition:** Nonparametric test: compute test statistic on observed data; compute distribution by permuting group labels (all permutations or Monte Carlo sample). Exact test if all permutations enumerated.
- **Cost Model:** O(n!) permutations → use Monte Carlo approximation (B permutations, B < n!); exact permutation test when feasible (small n).
- **Real Wall:** Exchangeability assumption must hold; test is exact under null; computationally infeasible for large n (use asymptotic approximation).
- **Cross-Domain Aliases:** randomization-test (ml-training), nonparametric-test (control-numerical-opt).

### [PRIM-030] false-discovery-rate
- **Atom/Composite:** Composite
- **Definition:** FDR = E[V/R] where V = false positives, R = total rejections. Benjamini-Hochberg: order p-values, find largest k such that p_(k) ≤ k·α/m; reject all p_(i) for i ≤ k. Less conservative than family-wise error rate (FWER).
- **Cost Model:** O(m log m) for m hypotheses (sorting p-values); BH controls FDR asymptotically but not FWER.
- **Real Wall:** Dependence between tests: BH assumes independence or positive regression dependence; more conservative corrections needed for arbitrary dependence (Benjamini-Yekutieli).
- **Cross-Domain Aliases:** multiple-testing-correction (ml-training), fdr-control (information-theory-coding).

## 5. Conjugate Priors & Distributions

### [PRIM-031] beta-bernoulli-conjugate
- **Atom/Composite:** Composite
- **Definition:** Beta prior on θ (success probability) + Bernoulli likelihood → Beta posterior. Prior: θ ~ Beta(α, β); Posterior: θ | x ~ Beta(α + Σxᵢ, β + n - Σxᵢ). Predictive: predictive probability of future data.
- **Cost Model:** Closed form: update hyperparameters by adding data counts; no numerical optimization; O(1) per update.
- **Real Wall:** Conjugate prior is not always appropriate (informative); for many parameters, exact conjugacy chains get complex.
- **Cross-Domain Aliases:** binomial-update (ml-training), posterior-analytic (control-numerical-opt).
- **Notes:** Laplace's rule of succession; used in A/B testing, spam filtering; Normal-Inverse-Gamma for Gaussian mean+variance.

### [PRIM-032] dirichlet-multinomial-conjugate
- **Atom/Composite:** Composite
- **Definition:** Dirichlet prior on category probabilities (θ₁:k) + multinomial likelihood → Dirichlet posterior. Prior: θ ~ Dir(α₁:k); Posterior: θ | data ~ Dir(α₁ + count₁, ..., α_k + count_k). Predictive: Dirichlet-multinomial.
- **Cost Model:** Closed form; O(k) update per observation; natural for categorical/discrete distributions.
- **Real Wall:** Dirichlet can't capture all dependencies; for overdispersed counts, use Dirichlet-process mixture.
- **Cross-Domain Aliases:** categorical-update (ml-training), topic-model-prior (retrieval-search).
- **Notes:** Latent Dirichlet Allocation (LDA): Dirichlet over topic distributions; Chinese Restaurant Process (CRP) is the infinite limit.

### [PRIM-033] normal-inverse-wishart
- **Atom/Composite:** Composite
- **Definition:** NIW prior for (μ, Σ) of multivariate Gaussian: μ|Σ ~ N(μ₀, Σ/κ₀), Σ ~ Inverse-Wishart(Λ₀, ν₀). Posterior: NIW with updated parameters; marginal likelihood = closed form.
- **Cost Model:** NIW posterior update: O(d³) for d-dimensional Gaussian (inverse-Wishart operations); closed-form predictive distribution.
- **Real Wall:** Inverse-Wishart parameterization varies by convention; wishart distribution only positive definite by construction.
- **Cross-Domain Aliases:** gaussian-covariance-prior (ml-training), multivariate-normal-posterior (linear-algebra-matrix).
- **Notes:** Conjugate prior for (mean, covariance) of Gaussian; used in Bayesian multivariate analysis, Gaussian process hyperparameter inference.

### [PRIM-034] exponential-family
- **Atom/Composite:** Composite
- **Definition:** Exponential family: p(x|θ) = h(x)·exp(η(θ)·T(x) - A(η)) where η = natural parameter, T = sufficient statistic, A = log-partition (cumulant generating function). MLE: T̄ = E_θ[T(X)].
- **Cost Model:** MLE has closed form for exponential families (solve T̄ = population sufficient statistic); conjugate prior exists for all exponential families.
- **Real Wall:** Not all distributions are exponential family; exponential family includes Gaussian, Poisson, Gamma, Beta, Dirichlet, multinomial, exponential, etc.
- **Cross-Domain Aliases:** natural-exponential-family (information-theory-coding), sufficient-statistic (control-numerical-opt).
- **Notes:** Koopman-Pitman-Darmois (KPD) theorem: exponential family is the only distribution family with finite-dimensional sufficient statistics for i.i.d. sampling.

## 6. Regression & Density Estimation

### [PRIM-035] kernel-density-estimation
- **Atom/Composite:** Primitive
- **Definition:** KDE: p̂(x) = (1/nh) Σ K((x - xᵢ)/h) where K = kernel, h = bandwidth. Silverman's rule: h ≈ 0.9·min(σ, IQR/1.34)·n^{-1/5}. Cross-validation bandwidth selection.
- **Cost Model:** O(n) to estimate density at one point; O(n·m) for m query points; bandwidth selection via LSCV (least squares cross-validation).
- **Real Wall:** Bandwidth too small = noisy; too large = oversmoothed; kernel choice less important than bandwidth; curse of dimensionality (h^d needed in d dimensions).
- **Cross-Domain Aliases:** nonparametric-density (ml-training), smoothing-estimator (control-numerical-opt).

### [PRIM-036] gaussian-process-regression
- **Atom/Composite:** Composite
- **Definition:** GP: prior over functions p(f) = GP(μ, K); posterior predictive: f* | x, y, x* ~ N(μ* + K(x*,x)·K(x,x)⁻¹·(y-μ), K(x*,x*) - K(x*,x)·K(x,x)⁻¹·K(x,x*)). Kernel K encodes covariance structure.
- **Cost Model:** O(n³) for Cholesky decomposition of K(x,x); sparse GP (inducing points) reduces to O(n·m²) with m inducing points; inference exact for Gaussian likelihood.
- **Real Wall:** Non-Gaussian likelihood (classification) requires approximation (Laplace, EP, variational); kernel design is key; O(n³) limits to n ≤ 10⁴.
- **Cross-Domain Aliases:** bayesian-nonparametric (ml-training), kernel-smoothing (control-numerical-opt).
- **Notes:** Rasmussen & Williams (GPML, 2006); used in Bayesian optimization (acquisition functions), spatial statistics, uncertainty quantification.

### [PRIM-037] quantile-regression
- **Atom/Composite:** Composite
- **Definition:** Quantile regression: estimate conditional quantile τ ∈ (0,1) by minimizing check loss: argmin_β Σ ρ_τ(yᵢ - xᵢᵀβ) where ρ_τ(u) = u(τ - I(u < 0)). Robust to outliers, reveals full conditional distribution.
- **Cost Model:** Linear programming formulation; can be solved via quantile matching or gradient methods;不像 OLS, quantile regression doesn't minimize MSE.
- **Real Wall:** Inference is harder (non-smooth objective); confidence intervals via bootstrap; tau must be specified (not estimated from data alone).
- **Cross-Domain Aliases:** conditional-quantile (ml-training), robust-regression (control-numerical-opt).

## 7. Statistical Decision Theory

### [PRIM-038] bayes-risk-minimization
- **Atom/Composite:** Primitive
- **Definition:** Decision theory: choose action a to minimize expected loss E[L(θ, a)] = ∫ L(θ, a)·p(θ | x) dθ. Bayes estimator = action minimizing Bayes risk. Point estimators: MAP (0-1 loss), posterior mean (squared error), posterior median (absolute error).
- **Cost Model:** Requires integration over posterior (often intractable); MCMC approximates; conjugate priors give closed form.
- **Real Wall:** Loss function choice is subjective; Bayes risk is relative to prior; if prior is wrong, Bayes risk is misleading.
- **Cross-Domain Aliases:** bayesian-decision (ml-training), expected-loss-minimize (control-numerical-opt).

### [PRIM-039] minimax-estimation
- **Atom/Composite:** Primitive
- **Definition:** Minimax: choose estimator minimizing worst-case risk: sup_θ E_θ[L(θ, θ̂)]. More robust than Bayes (doesn't depend on prior). Minimax estimator = Bayes against least favorable prior.
- **Cost Model:** Harder to compute than Bayes risk; often involves solving saddle-point equations; shrinkage estimators (James-Stein) are minimax for multivariate normal mean.
- **Real Wall:** Conservative (optimizes worst case, not typical case); least favorable prior may be improper; minimax is more robust for adversarial settings.
- **Cross-Domain Aliases:** worst-case-optimize (control-numerical-opt), robust-estimation (ml-training).

## Appendix: Primitive Count

Total primitives in statistics-probability domain: **39**
