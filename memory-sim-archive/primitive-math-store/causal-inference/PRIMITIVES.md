# Causal Inference Domain Primitives
> Cross-domain wiring: do-calculus = intervention + counterfactual; SCM = structural equations + graphical models;
> causal discovery = search + conditional independence; causal bandits = online learning + treatment assignment.

## 1. Foundational Causal Framework

### [PRIM-001] structural-causal-model
- **Atom/Composite:** Composite
- **Definition:** SCM (Structural Causal Model): set of endogenous variables X, exogenous variables U, and structural equations f such that X = f(X, U). Each equation represents a causal mechanism.
- **Cost Model:** SCM evaluation = forward simulation through causal graph; counterfactual requires solving structural equations with new U values.
- **Real Wall:** SCMs assume acyclic graphs (except with cycles under equilibrium); cyclic SCMs require fixed-point solution.
- **Cross-Domain Aliases:** causal-dag (information-theory-coding), structural-equations (agentic-reasoning).
- **Notes:** Pearl (2009); SCMs formalize causation as mechanistic functions rather than statistical associations.

### [PRIM-002] do-operator
- **Atom/Composite:** Primitive
- **Definition:** do(X = x): intervention that fixes variable X to value x, removing incoming arrows from its causal parents. Post-intervention distribution P(Y | do(X=x)).
- **Cost Model:** do-operator requires graph surgery (remove incoming edges to X) + apply intervention; O(|E|) graph modification.
- **Real Wall:** do differs from conditioning: do(X=x) ≠ P(X=x | X=x) unless X has no confounders.
- **Cross-Domain Aliases:** intervention-operator (agentic-reasoning), experimental-manipulation (statistics-probability).
- **Notes:** Pearl (1995); do-calculus provides rules for manipulating do-expressions without simulation.

### [PRIM-003] causal-identification
- **Atom/Composite:** Composite
- **Definition:** Causal identification: determine if P(Y | do(X)) is identifiable from observational distribution P(X, Y, Z). Backdoor and front-door criteria.
- **Cost Model:** Identification algorithm: test for confounders, apply do-calculus rules; in worst case requires solving linear equations.
- **Real Wall:** Non-identifiable effects require auxiliary assumptions (instrumental variables, proxy variables) or bounds.
- **Cross-Domain Aliases:** identifiability (statistics-probability), causal-decomposition (logic-reasoning).
- **Notes:** Pearl (2009) causal identification theory; ID algorithm computes causal effect symbolically.

### [PRIM-004] backdoor-criterion
- **Atom/Composite:** Composite
- **Definition:** Backdoor criterion: set Z satisfies backdoor if Z blocks all backdoor paths from X to Y and contains no descendants of X.
- **Cost Model:** d-separation test for all paths; O(|paths|) path enumeration; d-separation check O(|V|·|E|).
- **Real Wall:** Minimum sufficient adjustment set = smallest set satisfying backdoor; different adjustment sets give same estimate.
- **Cross-Domain Aliases:** confounding-adjustment (statistics-probability), path-blocking (information-theory-coding).
- **Notes:** Pearl (1995); adjustment formula: P(Y | do(X)) = Σ_z P(Y | X, Z=z) P(Z=z).

### [PRIM-005] front-door-criterion
- **Atom/Composite:** Composite
- **Definition:** Front-door criterion: set Z mediates all causal effect of X on Y, with no unblocked backdoor path from X to Z and no confounder of Z and Y.
- **Cost Model:** Two-step identification: first P(Z | do(X)), then P(Y | do(Z)); both identifiable if front-door satisfied.
- **Real Wall:** Front-door is useful when X and Y share an unmeasured confounder; captures mediation through Z.
- **Cross-Domain Aliases:** mediation-path (agentic-reasoning), indirect-effect (statistics-probability).
- **Notes:** Pearl (1995); front-door formula: P(Y | do(X)) = Σ_z P(Z=z | do(X)) Σ_x' P(Y | Z=z, X=x') P(X=x').

### [PRIM-006] do-calculus
- **Atom/Composite:** Composite
- **Definition:** Do-calculus: three inference rules for manipulating do-expressions. (1) Insert/deletion: P(Y | do(X), Z, W) = P(Y | do(X), W) if Y ⊥ Z | X, W in G_δ; (2) Exchange; (3) Action/observation.
- **Cost Model:** Each rule application = check d-separation condition; recursive application until no more rules applicable.
- **Real Wall:** Complete algorithm exists for identification using do-calculus; ID algorithm is the constructive version.
- **Cross-Domain Aliases:** causal-manipulation (logic-reasoning), intervention-inference (agentic-reasoning).
- **Notes:** Pearl (1995, 2009); do-calculus is complete for identification; if no sequence of rules reduces do, then not identifiable.

### [PRIM-007] potential-outcomes
- **Atom/Composite:** Composite
- **Definition:** Potential outcomes framework (Rubin causal model): Y_i(1) = outcome for unit i if treated; Y_i(0) = outcome if control. Treatment effect = Y_i(1) - Y_i(0).
- **Cost Model:** Individual treatment effect (ITE) not directly observable (fundamental problem of causal inference); need SUTVA.
- **Real Wall:** Stable unit treatment value assumption (SUTVA): no interference, no hidden versions of treatment.
- **Cross-Domain Aliases:** treatment-effect (statistics-probability), counterfactual-outcome (logic-reasoning).
- **Notes:** Rubin (1974, 2005); fundamental tension: individual-level effects require untestable assumptions.

### [PRIM-008] causal-dag
- **Atom/Composite:** Primitive
- **Definition:** Causal DAG: directed acyclic graph where edges represent direct causal relationships. Markov property: each variable independent of non-descendants given parents.
- **Cost Model:** DAG evaluation: d-separation tests O(|V|·|E|); conditional independence queries via Bayes Ball algorithm.
- **Real Wall:** DAG representation requires causal sufficiency (no unmeasured confounders) or explicit latent variables.
- **Cross-Domain Aliases:** bayesian-network (information-theory-coding), directed-graph (linear-algebra-matrix).
- **Notes:** Pearl (2009); DAGs encode causal assumptions explicitly; d-separation = statistical independence condition.

### [PRIM-009] d-separation
- **Atom/Composite:** Composite
- **Definition:** D-separation (directed separation): path blocking condition. A path is blocked by set Z if: (a) chain or fork with intermediate in Z, or (b) collider without descendant in Z.
- **Cost Model:** Bayes Ball algorithm O(|V|·|E|) per query; moralization + graph traversal.
- **Real Wall:** Colliders (V-structures) unblock when conditioning on descendants; critical for distinguishing causation from correlation.
- **Cross-Domain Aliases:** path-blocking (information-theory-coding), conditional-independence (statistics-probability).
- **Notes:** Pearl (1988); d-separation ↔ conditional independence in Markov equivalence class.

### [PRIM-010] markov-equivalence-class
- **Atom/Composite:** Composite
- **Definition:** Markov equivalence class: set of DAGs with same d-separation relations. Characterized by essential graph (CPDAG): skeleton + v-structures.
- **Cost Model:** PC algorithm computes MEC from conditional independence tests; complexity O(2^d) for d variables.
- **Real Wall:** All DAGs in same MEC yield same observational distribution; MEC is all we can learn from observational data alone.
- **Cross-Domain Aliases:** equivalence-class (linear-algebra-matrix), skeleton-recovery (information-theory-coding).
- **Notes:** Chickering (2002); MEC size can be exponential in number of edges; essential graph uniquely represents MEC.

## 2. Causal Effect Estimation

### [PRIM-011] inverse-probability-weighting
- **Atom/Composite:** Composite
- **Definition:** IPW (Inverse Probability Weighting): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) P(Z=z) when Z satisfies backdoor. Empirically: weight observations by 1/P(X | Z).
- **Cost Model:** Estimate propensity scores e(Z) = P(X=1 | Z); weight = 1/e(Z) for treated, 1/(1-e(Z)) for control.
- **Real Wall:** IPW unstable when propensity scores near 0 or 1 (extreme weights); truncate weights or use stabilized weights.
- **Cross-Domain Aliases:** propensity-score-weighting (statistics-probability), ipw-estimation (ml-training).
- **Notes:** Robins et al. (2000); doubly robust IPW adds outcome regression to reduce variance.

### [PRIM-012] propensity-score
- **Atom/Composite:** Primitive
- **Definition:** Propensity score e(Z) = P(X=1 | Z): probability of treatment given covariates. Balancing score: treatment independent of confounders given e(Z).
- **Cost Model:** Propensity score estimation: logistic regression O(n·p), machine learning (CausalForest) O(n·p·trees).
- **Real Wall:** Propensity score estimation model must be correctly specified; overlap assumption (positivity) required.
- **Cross-Domain Aliases:** balancing-score (statistics-probability), treatment-probability (ml-training).
- **Notes:** Rosenbaum & Rubin (1983); propensity score enables dimension reduction from many covariates to scalar.

### [PRIM-013] propensity-score-matching
- **Atom/Composite:** Composite
- **Definition:** Propensity score matching: match treated units to control units with similar propensity scores. Nearest neighbor, radius, stratification matching.
- **Cost Model:** Matching O(n²) if naive; KD-tree indexing reduces to O(n log n); greedy matching is fast but may miss optimal matches.
- **Real Wall:** Matching without replacement can improve balance but reduces effective sample size; with replacement may reuse controls.
- **Cross-Domain Aliases:** nearest-neighbor-matching (statistics-probability), covariate-matching (ml-training).
- **Notes:** Rosenbaum & Rubin (1985); matching on propensity score = matching on all covariates (under ignorability).

### [PRIM-014] doubly-robust-estimation
- **Atom/Composite:** Composite
- **Definition:** Doubly robust (DR) estimator: combine outcome regression with IPW. Consistent if EITHER propensity model OR outcome model is correct.
- **Cost Model:** DR estimator: μ_1 = E[μ̂(X,W) + (X - ê(W)) (Y - μ̂(X,W)) / ê(W)]; two model fits + one weighting pass.
- **Real Wall:** DR estimators have lower variance than pure IPW; both models wrong → bias (not DR).
- **Cross-Domain Aliases:** augmented-ipw (statistics-probability), model-combination (ml-training).
- **Notes:** Robins & Rotnitzky (1995); AIPW = Augmented IPW = DR estimator; targeted learning uses DR as core primitive.

### [PRIM-015] g-computation
- **Atom/Composite:** Composite
- **Definition:** G-computation (g-formula): P(Y | do(X=x)) = Σ_z P(Y | X=x, Z=z) Π_j P(Z_j | Z_{<j}, X=x). Sequential backdoor adjustment.
- **Cost Model:** G-computation requires modeling sequential conditional distributions; parametric g-formula = regression-based.
- **Real Wall:** G-computation is sensitive to model misspecification; IPW is less sensitive but higher variance.
- **Cross-Domain Aliases:** sequential-adjustment (statistics-probability), marginal-structural-model (ml-training).
- **Notes:** Robins (1986); g-computation is the non-parametric identification formula; sequential regression estimate.

### [PRIM-016] marginal-structural-model
- **Atom/Composite:** Composite
- **Definition:** MSM (Marginal Structural Model): model causal effect as function of treatment history. E[Y(t)] = β_0 + β_1 t. IPTW used to fit MSM.
- **Cost Model:** MSM fitting: estimate IPTW weights, fit weighted regression; standard errors via influence function.
- **Real Wall:** MSMs correctly model treatment effect heterogeneity over time; time-varying confounding handled by IPTW.
- **Cross-Domain Aliases:** causal-regression (statistics-probability), weighted-estimation (ml-training).
- **Notes:** Robins et al. (2000); MSMs handle longitudinal treatments where g-computation requires sequential models.

### [PRIM-017] structural-nested-model
- **Atom/Composite:** Composite
- **Definition:** SNM (Structural Nested Model): model treatment effect at each time point conditional on history. g(μ(t, w)) = β(t) · ψ(t, w).
- **Cost Model:** SNM fitting via G-estimation: iterative optimization of β parameters; requires modeling E[Y | history, treatment].
- **Real Wall:** SNMs are more efficient than MSMs when correctly specified; sensitive to model specification.
- **Cross-Domain Aliases:** nested-effect-model (statistics-probability), sequential-causal-model (agentic-reasoning).
- **Notes:** Robins (1999); G-estimation solves estimating equations for β parameters.

### [PRIM-018] covariate-adjustment
- **Atom/Composite:** Primitive
- **Definition:** Covariate adjustment: control for confounders Z to estimate causal effect. Stratification, regression adjustment, ANCOVA.
- **Cost Model:** Adjustment formula Σ_z P(Y | X, Z=z) P(Z=z); stratification = weighted average across strata.
- **Real Wall:** Must only adjust for confounders, not colliders or mediators (over-adjustment); bad-controls rule applies.
- **Cross-Domain Aliases:** adjustment-formula (statistics-probability), stratification (ml-training).
- **Notes:** Pearl (2009); covariate adjustment is valid iff Z satisfies backdoor criterion.

## 3. Treatment Effects & Heterogeneity

### [PRIM-019] average-treatment-effect
- **Atom/Composite:** Primitive
- **Definition:** ATE (Average Treatment Effect): E[Y(1) - Y(0)] = E[Y | do(X=1)] - E[Y | do(X=0)]. Population-level causal contrast.
- **Cost Model:** ATE estimation requires either randomization or unconfoundedness + overlap assumptions.
- **Real Wall:** ATE differs from ATT (Average Treatment effect on Treated) when selection bias exists.
- **Cross-Domain Aliases:** causal-contrast (statistics-probability), treatment-mean-difference (ml-training).
- **Notes:** Randomized experiment: ATE = difference in means. Observational: ATE estimated via IPW, matching, or regression.

### [PRIM-020] conditional-average-treatment-effect
- **Atom/Composite:** Composite
- **Definition:** CATE (Conditional Average Treatment Effect): E[Y(1) - Y(0) | C=c]. Heterogeneous treatment effect across subpopulations.
- **Cost Model:** CATE estimation requires estimating E[Y | X, Z] for X=1 and X=0; CATE = difference of conditional expectations.
- **Real Wall:** CATE estimation is ill-posed without regularization; requires large sample for reliable subgroup estimates.
- **Cross-Domain Aliases:** hte (ml-training), subgroup-effect (statistics-probability).
- **Notes:** X-learner, T-learner, S-learner estimate CATE; causal forests provide nonparametric CATE estimates.

### [PRIM-021] heterogeneous-treatment-effects
- **Atom/Composite:** Composite
- **Definition:** HTE (Heterogeneous Treatment Effects): treatment effect varies across units based on covariates. Key for precision medicine, A/B testing.
- **Cost Model:** HTE estimation: predict treatment effect from features; X-learner = 2-stage with separate outcome models.
- **Real Wall:** HTE estimation requires overlap (positivity): treatment assignment depends on features but not deterministically.
- **Cross-Domain Aliases:** interaction-effect (statistics-probability), conditional-effect (ml-training).
- **Notes:** Athey & Imbens (2016); HTE framework underlies meta-learners and causal forests.

### [PRIM-022] instrumental-variable
- **Atom/Composite:** Composite
- **Definition:** IV (Instrumental Variable): variable Z that affects X, is unrelated to confounders U, and affects Y only through X. Enables causal effect estimation with unmeasured confounding.
- **Cost Model:** IV assumptions: (1) Z ⊥ U, (2) Z ⊥ Y | X, U, (3) Z → X. Estimand: LATE = E[Y(1) - Y(0) | compliers].
- **Real Wall:** IV only identifies LATE (Local Average Treatment Effect on compliers), not ATE; monotonicity needed for interpretation.
- **Cross-Domain Aliases:** causal-instrument (statistics-probability), two-stage-least-squares (control-numerical-opt).
- **Notes:** Angrist et al. (1996); 2SLS = two-stage least squares: (1) regress X on Z, (2) regress Y on predicted X.

### [PRIM-023] two-stage-least-squares
- **Atom/Composite:** Composite
- **Definition:** 2SLS (Two-Stage Least Squares): (1) First stage: regress X on instruments Z; (2) Second stage: regress Y on predicted X̂.
- **Cost Model:** First stage O(n·p_z); second stage O(n); total O(n·(p_z + p_x)). Standard errors from second stage.
- **Real Wall:** Weak instruments (low F-statistic) → biased estimates; rule of thumb: F > 10 for first-stage strength.
- **Cross-Domain Aliases:** iv-estimation (statistics-probability), reduced-form (control-numerical-opt).
- **Notes:** Wright (1928); 2SLS is the standard IV estimator; correct standard errors need correction for first-stage uncertainty.

### [PRIM-024] local-average-treatment-effect
- **Atom/Composite:** Composite
- **Definition:** LATE (Local Average Treatment Effect): E[Y(1) - Y(0) | compliers, Z=1] - E[Y(1) - Y(0) | compliers, Z=0]. Effect on units who comply with assignment.
- **Cost Model:** LATE = (E[Y | Z=1] - E[Y | Z=0]) / (E[X | Z=1] - E[X | Z=0]). Wald estimator.
- **Real Wall:** LATE excludes never-takers and always-takers; only identifies effect on compliers.
- **Cross-Domain Aliases:** complier-effect (statistics-probability), local-effect (control-numerical-opt).
- **Notes:** Imbens & Angrist (1994); monotonicity assumption: Z does not discourage treatment for anyone.

### [PRIM-025] difference-in-differences
- **Atom/Composite:** Composite
- **Definition:** DiD (Difference-in-Differences): compare treatment and control groups before and after treatment. ATT = (Ȳ_T,post - Ȳ_T,pre) - (Ȳ_C,post - Ȳ_C,pre).
- **Cost Model:** DiD estimation: O(n) to compute means; parallel trends assumption is the identifying assumption.
- **Real Wall:** Parallel trends violation is the main threat to validity; falsification tests, event studies check parallel trends.
- **Cross-Domain Aliases:** quasi-experiment (statistics-probability), before-after-comparison (ml-training).
- **Notes:** Card & Krueger (1994); DiD is robust to time-invariant group-level confounders; not to time-varying confounding.

### [PRIM-026] regression-discontinuity
- **Atom/Composite:** Composite
- **Definition:** RDD (Regression Discontinuity Design): treatment assigned at cutoff c. Units just above and just below c are comparable (continuity assumption).
- **Cost Model:** Local polynomial regression around cutoff c; bandwidth selection via MSE minimization (Imbens-Kalyanaraman).
- **Real Wall:** Fuzzy RDD: treatment probability jumps at cutoff but doesn't go to 1; use local IV around cutoff.
- **Cross-Domain Aliases:** cutoff-design (statistics-probability), local-comparison (ml-training).
- **Notes:** Thistlethwaite & Campbell (1960); RDD estimates local effect at the cutoff (LATE with compliance at cutoff).

### [PRIM-027] synthetic-control
- **Atom/Composite:** Composite
- **Definition:** Synthetic control method: construct weighted average of control units matching treated unit's pre-treatment outcomes. Estimate treatment effect as difference in post-period.
- **Cost Model:** Optimization to find weights w minimizing (Y_pretreated - Σ_j w_j Y_prej)²; convex optimization O(n²).
- **Real Wall:** Pre-treatment fit quality determines credibility; leave-one-out cross-validation for weight selection.
- **Cross-Domain Aliases:** counterfactual-control (statistics-probability), weighted-comparison (ml-training).
- **Notes:** Abadie et al. (2010); used for policy evaluation when few treated units; SCM generalizes to multiple outcomes.

### [PRIM-028] matching-estimators
- **Atom/Composite:** Composite
- **Definition:** Matching estimators: pair treated units with similar control units on covariates. Nearest neighbor, propensity score, covariate matching.
- **Cost Model:** Nearest neighbor matching O(n²) naive, O(n log n) with KD-tree; Mahalanobis distance accounts for covariate correlations.
- **Real Wall:** Common support restriction needed (overlap); matching on high-dimensional covariates suffers from curse of dimensionality.
- **Cross-Domain Aliases:** causal-matching (ml-training), covariate-pairing (statistics-probability).
- **Notes:** Rosenbaum & Rubin (1985); matching reduces overt bias from observable confounders; residual confounding remains.

### [PRIM-029] entropy-balancing
- **Atom/Composite:** Composite
- **Definition:** Entropy balancing: reweight control group to match moments (mean, variance, skewness) of treated group. Unlike matching, uses all controls.
- **Cost Model:** Entropy minimization with moment constraints: convex optimization; closed-form solution for mean constraints.
- **Real Wall:** Exact balance on specified moments; no variance reduction from partial matching; can fail if distributions don't overlap.
- **Cross-Domain Aliases:** covariate-balancing (statistics-probability), reweighting (ml-training).
- **Notes:** Hainmueller (2012); entropy balancing = covariate balancing propensity score (CBPS) generalization.

### [PRIM-030] targeted-maximum-likelihood-estimation
- **Atom/Composite:** Composite
- **Definition:** TMLE (Targeted Maximum Likelihood Estimation): estimate causal effect via iterative targeted learning. Combines super learner with bias reduction.
- **Cost Model:** TMLE steps: (1) initial estimate of treatment mechanism, (2) compute clever covariate, (3) update with least squares.
- **Real Wall:** TMLE is doubly robust and efficient; requires correct model for either treatment or outcome mechanism.
- **Cross-Domain Aliases:** efficient-influence-function (statistics-probability), targeted-learning (ml-training).
- **Notes:** van der Laan & Rubin (2006); TMLE achieves efficiency bound when both models are correct.

### [PRIM-031] causal-forest
- **Atom/Composite:** Composite
- **Definition:** Causal forest (Athey-Imbens): random forest adapted for CATE estimation. Grow trees that maximize treatment effect heterogeneity.
- **Cost Model:** Honest forests: separate sample for splitting and estimation; CATE variance from within-leaf variance; O(n·log n·trees).
- **Real Wall:** Causal forests estimate CATE without parametric assumptions; honest splitting improves coverage.
- **Cross-Domain Aliases:** heterogeneous-effect-forest (ml-training), adaptive-subgroup (statistics-probability).
- **Notes:** Wager & Athey (2018); local centering of outcomes removes confounding within leaves.

## 4. Causal Discovery

### [PRIM-032] causal-discovery
- **Atom/Composite:** Composite
- **Definition:** Causal discovery: learn causal structure from observational data. Score-based (BIC), constraint-based (PC), functional (LiNGAM).
- **Cost Model:** PC algorithm O(2^d) in worst case; BIC scoring O(n·d²) per parent set; structure search is NP-hard.
- **Real Wall:** Causal discovery from observational data alone can only identify Markov equivalence class, not full DAG.
- **Cross-Domain Aliases:** structure-learning (information-theory-coding), graph-recovery (ml-training).
- **Notes:** Spirtes et al. (2000); Shimizu et al. (2006) LiNGAM for linear non-Gaussian models.

### [PRIM-033] pc-algorithm
- **Atom/Composite:** Composite
- **Definition:** PC algorithm: constraint-based causal discovery. (1) Skeleton recovery via conditional independence tests, (2) edge orientation using v-structure detection.
- **Cost Model:** CI testing: conditional sets of size 0..d-1; exponential in conditioning set size; partial correlation tests O(n).
- **Real Wall:** PC assumes causal sufficiency (no hidden confounders) and faithfulness; sensitive to CI test accuracy.
- **Cross-Domain Aliases:** constraint-based-discovery (information-theory-coding), ci-testing (statistics-probability).
- **Notes:** Spirtes et al. (2000); PC algorithm outputs CPDAG (completed partially directed acyclic graph).

### [PRIM-034] fci-algorithm
- **Atom/Composite:** Composite
- **Definition:** FCI (Fast Causal Inference): handles hidden confounders. Augments PC with orientation rules and possible-ancestor relations.
- **Cost Model:** FCI complexity higher than PC; orientation rules more complex; may leave edges undirected in presence of latent variables.
- **Real Wall:** FCI is sound but not complete; PAG (Partial Ancestral Graph) represents output.
- **Cross-Domain Aliases:** latent-variable-discovery (information-theory-coding), hidden-confounder (agentic-reasoning).
- **Notes:** Spirtes et al. (1999); FCI recovers ancestral relations even with hidden confounders.

### [PRIM-035] greedy-equivalence-search
- **Atom/Composite:** Composite
- **Definition:** GES (Greedy Equivalence Search): score-based causal discovery. Start with empty graph; add, remove, or reverse edges to maximize BIC.
- **Cost Model:** BIC scoring: local modifications O(d²) per move; exponential search space but greedy is tractable.
- **Real Wall:** GES is consistent under faithfulness and score equivalence; can outperform PC when sample size is large.
- **Cross-Domain Aliases:** score-based-search (information-theory-coding), structure-optimization (ml-training).
- **Notes:** Chickering (2002); GES searches over equivalence classes of DAGs using BIC/MDL score.

### [PRIM-036] lingam
- **Atom/Composite:** Composite
- **Definition:** LiNGAM (Linear Non-Gaussian Acyclic Model): linear SCM where errors are non-Gaussian. Direct LiNGAM can identify full DAG orientation.
- **Cost Model:** LiNGAM estimation: ICA (Independent Component Analysis) or DirectLiNGAM O(n·d²); non-Gaussianity enables orientation.
- **Real Wall:** LiNGAM requires non-Gaussian errors; fails if errors are Gaussian (same as linear Gaussian SCM).
- **Cross-Domain Aliases:** non-gaussian-scm (information-theory-coding), linear-structural-model (control-numerical-opt).
- **Notes:** Shimizu et al. (2006); non-Gaussianity breaks the Markov equivalence class barrier.

### [PRIM-037] pcalg-rci
- **Atom/Composite:** Composite
- **Definition:** RCIT/RCD (Robust Conditional Independence Test / Discovery): kernel-based CI testing for causal discovery. More powerful than linear CI tests.
- **Cost Model:** HSIC (Hilbert-Schmidt Independence Criterion) test O(n²) per test; permutation testing O(n·B) for B permutations.
- **Real Wall:** Kernel CI tests detect non-linear dependencies that partial correlation misses; more robust to model violations.
- **Cross-Domain Aliases:** kernel-ci-test (statistics-probability), nonlinear-discovery (information-theory-coding).
- **Notes:** Zhang et al. (2011); kernel-based discovery (FCI, PC) is more powerful with sufficient sample size.

### [PRIM-038] samiam-bn-structure
- **Atom/Composite:** Composite
- **Definition:** Bayesian network structure learning: find DAG maximizing posterior probability P(G | D) ∝ P(D | G) P(G). Score-based or MCMC approaches.
- **Cost Model:** Bayesian scoring (BDe, BIC) O(n·d) per DAG evaluation; search over exponential DAG space.
- **Real Wall:** Score equivalence: BIC/BDe score is score-equivalent; different DAGs in same MEC get same score.
- **Cross-Domain Aliases:** bayesian-structure-learning (ml-training), dag-scoring (information-theory-coding).
- **Notes:** Cooper & Herskovits (1992) BDe score; Chickering (2002) GES optimal under score equivalence.

### [PRIM-039] mcmc-structure-learning
- **Atom/Composite:** Composite
- **Definition:** MCMC for DAG structures: Metropolis-Hastings over DAG space; reversible jump MCMC for varying dimension. Sample from posterior P(G | D).
- **Cost Model:** MCMC mixing can be slow due to high-dimensional DAG space; proposal distribution = edge addition/removal/reversal.
- **Real Wall:** DAG space is not continuous; reversible jump MCMC handles dimension changes between DAGs.
- **Cross-Domain Aliases:** posterior-sampling (statistics-probability), stochastic-structure-search (ml-training).
- **Notes:** Madigan & York (1995); Bayesian model averaging over DAGs uses MCMC posterior samples.

### [PRIM-040] dag-markov-property
- **Atom/Composite:** Primitive
- **Definition:** Markov property: in a DAG, each variable X_i is conditionally independent of its non-descendants given its parents. Faithfulness: all conditional independences arise from DAG structure.
- **Cost Model:** Markov property test: check CI in DAG vs. data; faithfulness violations are measure-zero but practically important.
- **Real Wall:** Unfaithful distributions exist but are measure-zero; detecting unfaithfulness is hard; assumes faithfulness holds.
- **Cross-Domain Aliases:** conditional-independence-markov (information-theory-coding), factorization-property (statistics-probability).
- **Notes:** Pearl (2009); Markov + faithfulness + causal sufficiency uniquely maps DAG to observational distribution.

### [PRIM-041] anc-search
- **Atom/Composite:** Composite
- **Definition:** ANC (Ancestral) search: recover ancestral relations from data. Extend PC/FCI with ancestral orientation rules.
- **Cost Model:** ANC rules add orientations not identifiable by PC alone; requires DAG cycles check.
- **Real Wall:** ANC can orient more edges than PC when causal structure is known to be ancestral.
- **Cross-Domain Aliases:** ancestral-orientation (information-theory-coding), orientation-rules (logic-reasoning).
- **Notes:** Spirtes et al. (2000); ANC search is part of FCI algorithm for ancestral relations.

### [PRIM-042] graphical-model-selection
- **Atom/Composite:** Composite
- **Definition:** Graphical model selection: recover sparsity structure of precision matrix or causal graph. neighborhood selection, graphical lasso, glasso.
- **Cost Model:** Graphical lasso O(d³) per iteration; neighborhood selection O(n·d²); alternating direction method O(d³).
- **Real Wall:** Graphical lasso assumes Gaussian distribution; non-Gaussian requires nonparametric methods.
- **Cross-Domain Aliases:** precision-matrix-estimation (linear-algebra-matrix), sparse-structure (ml-training).
- **Notes:** Meinshausen & Buhlmann (2006); neighborhood selection = lasso on each variable's regression.

### [PRIM-043] intervention-calculus
- **Atom/Composite:** Composite
- **Definition:** ID algorithm (Identifiable DAG): recursively identify causal effects in DAGs. Compute descendants, check for observable confounders.
- **Cost Model:** ID algorithm: O(d³) for d variables; recursive substitution of do-operators.
- **Real Wall:** ID algorithm is complete for identification in DAGs; outputs causal effect as linear combination of observational distributions.
- **Cross-Domain Aliases:** identification-algorithm (logic-reasoning), causal-decomposition (agentic-reasoning).
- **Notes:** Tian & Pearl (2002); ID algorithm generalizes backdoor/front-door criteria to arbitrary DAGs.

### [PRIM-044] zVI-identification
- **Atom/Composite:** Composite
- **Definition:** Z-identification (Z台风): identify causal effect using instrument Z. IV estimator generalizes to multi-instrument, multi-treatment settings.
- **Cost Model:** Optimal IV combination: 2SLS, J-test for overidentification; LIML (limited information ML) for weak instruments.
- **Real Wall:** Many instruments: aggregate information; weak instruments bias toward OLS; need relevance and exclusion restrictions.
- **Cross-Domain Aliases:** multi-instrument-IV (statistics-probability), iv-generalization (control-numerical-opt).
- **Notes:** Angrist & Pischke (2009); first-stage F-statistic rule of thumb: F > 10 for instrument strength.

## 5. Counterfactual Reasoning

### [PRIM-045] counterfactual-query
- **Atom/Composite:** Composite
- **Definition:** Counterfactual: \"Given that Y was observed to be y when X=x, what would Y be if X=x'?\". Three-step: abduction, action, prediction.
- **Cost Model:** Counterfactual = solve structural equations with observed evidence; requires Abductive inference to update U distribution.
- **Real Wall:** Counterfactuals are more demanding than interventional queries; require full SCM specification.
- **Cross-Domain Aliases:** what-if-reasoning (agentic-reasoning), hypothetical-reasoning (logic-reasoning).
- **Notes:** Pearl (2000); counterfactual = P(Y_x(u) | X=x, Y=y) for unit u; fundamental to causal explanation.

### [PRIM-046] counterfactual-attribution
- **Atom/Composite:** Composite
- **Definition:** Counterfactual attribution: decompose observed outcome into contribution of each factor. Causal inference for specific units.
- **Cost Model:** Attribution = counterfactual comparison: what would happen without each factor? Path-specific effects.
- **Real Wall:** Attribution requires assumptions about mechanism contributions; Shapley values provide model-agnostic attribution.
- **Cross-Domain Aliases:** causal-attribution (agentic-reasoning), factor-contribution (statistics-probability).
- **Notes:** Pearl (2001); causal mediation analysis decomposes total effect into direct and indirect effects.

### [PRIM-047] mediation-analysis
- **Atom/Composite:** Composite
- **Definition:** Causal mediation analysis: decompose total effect into natural direct effect (NDE) and natural indirect effect (NIE) through mediator M.
- **Cost Model:** NDE = E[Y(do(X=1, M=M(X=0))) - Y(do(X=0))]; NIE = E[Y(do(X=1)) - Y(do(X=1, M=M(X=0)))].
- **Real Wall:** Mediation identification requires no unmeasured confounding of exposure-mediator, mediator-outcome (sequential ignorability).
- **Cross-Domain Aliases:** indirect-effect (statistics-probability), path-decomposition (information-theory-coding).
- **Notes:** Robins & Greenland (1992), Pearl (2001); mediation formulas generalize Baron-Kenny approach.

### [PRIM-048] path-specific-effects
- **Atom/Composite:** Composite
- **Definition:** PSE (Path-Specific Effects): causal effect transmitted along specific causal paths. Generalizes mediation to multiple paths.
- **Cost Model:** PSE requires counterfactual decomposition; identified if no unmeasured confounding along each path.
- **Real Wall:** Identification of arbitrary PSE requires stronger assumptions than total effect; partial identification may be needed.
- **Cross-Domain Aliases:** multi-path-effect (agentic-reasoning), effect-decomposition (statistics-probability).
- **Notes:** Pearl (2001), Avin et al. (2005); PSE needed when direct and indirect effects are not sufficient.

### [PRIM-049] nested-counterfactuals
- **Atom/Composite:** Composite
- **Definition:** Nested counterfactuals: counterfactual within counterfactual. Y_x(x', u) = outcome under treatment x' for unit with counterfactual state u.
- **Cost Model:** Nested counterfactuals require second-level structural equations; computed by Abduction-ACTION-PREDICTION.
- **Real Wall:** Nested counterfactuals enable comparison of different treatment policies; key for policy evaluation.
- **Cross-Domain Aliases:** meta-counterfactual (logic-reasoning), policy-comparison (agentic-reasoning).
- **Notes:** Balke & Pearl (1994); nested counterfactuals underlie causal transportability and external validity.

### [PRIM-050] attribution-counterfactual
- **Atom/Composite:** Composite
- **Definition:** Attribution counterfactual: \"Did X cause Y?\" Compute probability that Y would not have occurred without X.
- **Cost Model:** Probability of causation P(NDE) = P(Y_0 = 0 | X=1, Y=1). Requires confounders Z for estimation.
- **Real Wall:** Probability of causation requires sensitivity to unmeasured confounding; bounds can be computed without assumptions.
- **Cross-Domain Aliases:** cause-probability (logic-reasoning), attribution-analysis (agentic-reasoning).
- **Notes:** Pearl (1999); probability of causation = E[NDE | observed outcome]; bounds via Shen et al.'s method.

## 6. Causal Inference with Machine Learning

### [PRIM-051] double-machine-learning
- **Atom/Composite:** Composite
- **Definition:** DML (Double/Debiased Machine Learning): estimate treatment effect by partialling out confounders using ML, then fitting linear model on residuals.
- **Cost Model:** Nuisance models: E[Y | X, W] and P(X | W); orthogonalized moment conditions O(n); ML for nuisance O(n·p·trees).
- **Real Wall:** DML requires orthogonal moments: nuisance estimation error is second-order; allows flexible ML for nuisance.
- **Cross-Domain Aliases:** orthogonal-learning (ml-training), nuisance-modeling (statistics-probability).
- **Notes:** Chernozhukov et al. (2018); DML enables valid inference with ML nuisance models.

### [PRIM-052] causal-inference-meta-learners
- **Atom/Composite:** Composite
- **Definition:** Meta-learners (S-learner, T-learner, X-learner): framework for applying ML to CATE estimation. S: single model on (X,W,Y); T: separate models per treatment.
- **Cost Model:** S-learner: O(n·d) for single model; T-learner: O(n·d·treatments); X-learner: 4 model fits.
- **Real Wall:** S-learner confounds treatment and covariate effects; T-learner may have imbalance across treatment groups.
- **Cross-Domain Aliases:** ml-causal-framework (ml-training), ensemble-causal (statistics-probability).
- **Notes:** Künzel et al. (2019); meta-learners provide plug-in framework for any base ML estimator.

### [PRIM-053] representation-learning-causal
- **Atom/Composite:** Composite
- **Definition:** Causal representation learning: learn representations that disentangle causal factors from data. CausalVAE, structural causal models in latent space.
- **Cost Model:** CausalVAE: encoder-decoder + SCM in latent space; adversarial training for disentanglement; O(n·d·epochs).
- **Real Wall:** Disentanglement requires inductive biases (sparsity, independence) not derivable from observational data alone.
- **Cross-Domain Aliases:** disentangled-representation (ml-training), latent-causal (information-theory-coding).
- **Notes:** Suter et al. (2019), Pfister et al. (2019); bridges causal inference and representation learning.

### [PRIM-054] causal-bandits
- **Atom/Composite:** Composite
- **Definition:** Causal bandits: multi-armed bandit with causal structure. Action = do-intervention; reward depends on causal effects.
- **Cost Model:** Causal UCB: incorporate causal structure into exploration bonus; information gain from interventions.
- **Real Wall:** Causal bandits exploit causal structure to reduce sample complexity vs. standard bandits.
- **Cross-Domain Aliases:** causal-exploration (ml-training), causal-rl (agentic-reasoning).
- **Notes:** Bareinboim & Pearl (2015); causal bandits exploit that actions affect different parts of the causal graph.

### [PRIM-055] transfer-learning-causal
- **Atom/Composite:** Composite
- **Definition:** Causal transfer learning: leverage causal structure from source to target domain. Identifiability conditions for transportability.
- **Cost Model:** Causal transportability: identify what can be transferred via do-calculus; z-transfer formula.
- **Real Wall:** Domain shift under non-stationarity; transfer only components that are invariant across domains.
- **Cross-Domain Aliases:** domain-adaptation (ml-training), causal-transportability (agentic-reasoning).
- **Notes:** Pearl & Bareinboim (2014); transportability formula: P(Y | do(X))_target = P(Y | do(X))_source if transportability conditions met.

### [PRIM-056] sensitivity-analysis
- **Atom/Composite:** Composite
- **Definition:** Causal sensitivity analysis: assess robustness of causal conclusions to unmeasured confounding. E-value, Rosenbaum bounds.
- **Cost Model:** E-value computation: HR = P(X=1 | U=1) / P(X=1 | U=0); E = HR + √(HR·(HR-1)); O(1) per analysis.
- **Real Wall:** Sensitivity analysis requires specifying plausible confounding strength; E-value translates to interpretable scale.
- **Cross-Domain Aliases:** unmeasured-confounding (statistics-probability), robustness-check (ml-training).
- **Notes:** VanderWeele & Ding (2017); E-value = minimum strength of association with both treatment and outcome needed to nullify.

### [PRIM-057] bounds-on-causal-effects
- **Atom/Composite:** Composite
- **Definition:** Partial identification: when causal effect is not point identifiable, derive bounds (e.g., for E[Y(1) - Y(0)] with binary outcome).
- **Cost Model:** Manski bounds: lower = P(Y=1 | X=1) - P(Y=1 | X=0); upper = similar with monotonicity. O(n) computation.
- **Real Wall:** Bounds can be very wide without additional assumptions; informative bounds require monotonicity or other restrictions.
- **Cross-Domain Aliases:** partial-identification (statistics-probability), interval-estimation (control-numerical-opt).
- **Notes:** Manski (1990); partial identification provides scientifically honest bounds when point identification is impossible.

### [PRIM-058] selection-bias
- **Atom/Composite:** Primitive
- **Definition:** Selection bias: systematic difference between treated and control groups not due to treatment. Berkson's paradox, survivorship bias.
- **Cost Model:** Selection bias correction: inverse probability weighting for selection, Heckman correction for sample selection.
- **Real Wall:** Selection on observables (ignorability) vs. selection on unobservables (requires stronger assumptions).
- **Cross-Domain Aliases:** confounding (statistics-probability), collider-bias (information-theory-coding).
- **Notes:** Heckman (1979); selection bias arises when treatment assignment depends on potential outcomes.

### [PRIM-059] confounding-adjustment-formula
- **Atom/Composite:** Composite
- **Definition:** G-computation formula: P(Y | do(X)) = Σ_z P(Y | X, Z=z) P(Z=z). The adjustment formula for backdoor criterion.
- **Cost Model:** O(|Z|) summation over all covariate values; for continuous Z, integrate.
- **Real Wall:** Only valid when Z satisfies backdoor; does not adjust for mediators or colliders.
- **Cross-Domain Aliases:** causal-adjustment (statistics-probability), stratification-formula (ml-training).
- **Notes:** Pearl (2009); the adjustment formula is the nonparametric generalization of ANCOVA.

### [PRIM-060] front-door-formula
- **Atom/Composite:** Composite
- **Definition:** Front-door formula: P(Y | do(X)) = Σ_z P(Z=z | do(X)) Σ_x P(Y | Z=z, X=x) P(X=x). Identifies effect via mediator Z.
- **Cost Model:** Two-step: estimate P(Z | X) and P(Y | Z, X); plug into front-door formula; O(n) estimation.
- **Real Wall:** Requires no unblocked backdoor from X to Z, and no confounder of Z and Y.
- **Cross-Domain Aliases:** mediation-formula (statistics-probability), indirect-effect-identification (agentic-reasoning).
- **Notes:** Pearl (1995); front-door criterion identifies causal effect even with unmeasured X-Y confounding.

## 7. Longitudinal & Complex Settings

### [PRIM-061] longitudinal-treatment
- **Atom/Composite:** Composite
- **Definition:** Longitudinal causal inference: treatment over time with time-varying confounders. G-methods: g-computation, IPTW, g-estimation.
- **Cost Model:** Time-varying IPTW: estimate propensity of each treatment at each time; product of propensity weights O(T·n·p).
- **Real Wall:** G-methods handle time-varying confounding that simple adjustment cannot; sequential randomization is ideal.
- **Cross-Domain Aliases:** dynamic-treatment (statistics-probability), sequential-causal (agentic-reasoning).
- **Notes:** Robins et al. (2000); g-computation, IPTW, and g-estimation are all valid under sequential ignorability.

### [PRIM-062] sequential-ignorability
- **Atom/Composite:** Composite
- **Definition:** Sequential ignorability: treatment at each time t is ignorable given history (past treatments, covariates). Enables identification in longitudinal settings.
- **Cost Model:** Sequential ignorability = {Y(t) ⊥ A_t | H_t} for all t; check overlap at each time point.
- **Real Wall:** Sequential ignorability is stronger than ignorability at baseline; requires no unmeasured time-varying confounding.
- **Cross-Domain Aliases:** sequential-randomization (statistics-probability), history-ignorability (agentic-reasoning).
- **Notes:** Robins (1999); sufficient condition for identification of causal effects of dynamic treatment regimens.

### [PRIM-063] optimal-treatment-policy
- **Atom/Composite:** Composite
- **Definition:** Optimal treatment policy: learn treatment rule a(H) maximizing expected outcome. Q-learning, A-learning, policy search.
- **Cost Model:** Q-learning: estimate Q(a, h) = E[Y | A=a, H=h]; optimal policy a*(h) = argmax_a Q(a, h). O(n·d·actions).
- **Real Wall:** Offline policy evaluation (OPE): estimate value of policy from historical data without randomization.
- **Cross-Domain Aliases:** policy-learning (ml-training), decision-theory (control-numerical-opt).
- **Notes:** Murphy et al. (2001); Q-learning is model-based RL for causal inference; regret minimization.

### [PRIM-064] g-estimation-snmm
- **Atom/Composite:** Composite
- **Definition:** G-estimation of structural nested models: estimate β in g(E[Y(t) | history, treatment]) = β(t) · ψ(t, history).
- **Cost Model:** G-estimation: solve estimating equations E[S_β(U_β(β))] = 0; iterative optimization O(n) per iteration.
- **Real Wall:** G-estimation is more efficient than IPTW when SNM is correctly specified; requires correct model for E[Y | history, treatment].
- **Cross-Domain Aliases:** structural-nested-estimation (statistics-probability), g-formula-estimation (ml-training).
- **Notes:** Robins (1999); G-estimation is the structural model counterpart to MSM for longitudinal treatments.

### [PRIM-065] structural-causal-graph
- **Atom/Composite:** Primitive
- **Definition:** Structural causal graph: DAG with explicit exogenous variable nodes U for each endogenous variable X. SCM representation.
- **Cost Model:** SCM ↔ DAG: each SCM induces a DAG; different SCMs can induce the same DAG.
- **Real Wall:** Cyclic SCMs require fixed-point semantics; causal Markov condition extends to cyclic graphs under equilibrium.
- **Cross-Domain Aliases:** causal-dag (information-theory-coding), functional-causal-model (agentic-reasoning).
- **Notes:** Pearl (2009); structural causal graph connects SCM to graphical representation.

### [PRIM-066] causal-effect-decomposition
- **Atom/Composite:** Composite
- **Definition:** Causal effect decomposition: total effect = direct effect + indirect effect (+ interaction if non-linear). Additive vs. multiplicative decomposition.
- **Cost Model:** Additive: TE = NDE + NIE; multiplicative: log(TE) = log(NDE) + log(NIE). Non-linear requires causal mediation formulas.
- **Real Wall:** Non-linear models: mediation decomposition is not straightforward; interaction terms complicate interpretation.
- **Cross-Domain Aliases:** effect-splitting (statistics-probability), mediation-decomposition (agentic-reasoning).
- **Notes:** VanderWeele (2015); natural direct and indirect effects are well-defined for stochastic interventions.

### [PRIM-067] causal-identification-id-algorithm
- **Atom/Composite:** Composite
- **Definition:** ID algorithm: identify causal effect in DAG by recursively applying do-calculus. Compute post-intervention distribution from observational.
- **Cost Model:** ID algorithm: recursively substitute do-operators; complexity O(d³) for d variables.
- **Real Wall:** ID outputs identification formula or declares non-identifiable; non-identifiable cases require bounds or additional assumptions.
- **Cross-Domain Aliases:** causal-decomposition-id (logic-reasoning), identification-algorithm (agentic-reasoning).
- **Notes:** Pearl (1995), Shpitser & Pearl (2006); ID algorithm is complete for DAG identification.

### [PRIM-068] transportability-formula
- **Atom/Composite:** Composite
- **Definition:** Causal transportability: transfer causal knowledge from source population to target with different distribution. do-calculus with selection variable S.
- **Cost Model:** Transportability formula: P(Y | do(X))_t = Σ_z P(Y | do(X), Z=z, S=1) P(Z=z | S=1). O(|Z|) summation.
- **Real Wall:** Transportability requires similarity of causal mechanisms; selection bias between populations complicates transfer.
- **Cross-Domain Aliases:** generalization-transfer (ml-training), population-transportability (agentic-reasoning).
- **Notes:** Pearl & Bareinboim (2011); z-transportability: transport across domains using z-factors.

### [PRIM-069] do-separation
- **Atom/Composite:** Primitive
- **Definition:** Do-separation: d-separation rule generalized to handle do-operators. X ⊥ Z | Y, G_δ means controlling for Y blocks path from X to Z in post-intervention graph.
- **Cost Model:** Do-separation tests require modified graph (remove incoming arrows to do(X)); then apply standard d-separation.
- **Real Wall:** Do-separation is used in do-calculus to determine if do-operator can be removed or simplified.
- **Cross-Domain Aliases:** intervention-separation (logic-reasoning), causal-independence (agentic-reasoning).
- **Notes:** Pearl (2009); do-separation generalizes d-separation to interventional settings.

### [PRIM-070] conditional-do-operator
- **Atom/Composite:** Primitive
- **Definition:** Conditional do (do(X | Z=z)): intervene fixing X=z within stratum Z=z. Generalizes standard do to conditional interventions.
- **Cost Model:** Conditional do = standard do after stratification; CATE identification uses conditional do formula.
- **Real Wall:** Conditional do enables identification of conditional causal effects; CATE = E[Y | do(X=1, Z)] - E[Y | do(X=0, Z)].
- **Cross-Domain Aliases:** stratified-intervention (statistics-probability), conditional-causal (agentic-reasoning).
- **Notes:** Pearl (2009); do(X | Z=z) = conditioning within intervention; not the same as P(Y | X, Z) in general.

## 8. Advanced Causal Inference

### [PRIM-071] instrumental-variable-strength
- **Atom/Composite:** Composite
- **Definition:** IV strength assessment: first-stage F-statistic. Weak instruments (F < 10) cause biased estimates and invalid inference.
- **Cost Model:** First-stage F = (β̂²_1S) / Var(β̂_1S); rule of thumb: F > 10 for relevant instruments.
- **Real Wall:** Weak instruments: bias of 2SLS toward OLS; robust inference requires LIML, Fuller, or Anderson-Rubin.
- **Cross-Domain Aliases:** instrument-relevance (statistics-probability), first-stage-strength (control-numerical-opt).
- **Notes:** Staiger & Stock (1997); weak instruments are common in practice; cluster-robust SEs needed.

### [PRIM-072] proxy-variable-causal
- **Atom/Composite:** Composite
- **Definition:** Proxy variables: use proxy W for unmeasured confounder U. MIDA (Model Instrumental/Directed Acyclic Graph) for proxy identification.
- **Cost Model:** MIDA model: if W ⊥ U | Z and W ⊥ Y | U, X and identification possible; O(n) estimation if model correct.
- **Real Wall:** Proxy variable methods require strong assumptions about relationship between proxy and confounder.
- **Cross-Domain Aliases:** confounder-proxy (statistics-probability), latent-variable-identification (agentic-reasoning).
- **Notes:** Kuroki & Pearl (2014); proxy variables enable causal identification in presence of unmeasured confounding.

### [PRIM-073] causal-bounds-intervals
- **Atom/Composite:** Composite
- **Definition:** Causal bounds with interval data: when Y is observed in intervals (censored), compute bounds on causal effect.
- **Cost Model:** Linear programming bounds: minimize/maximize causal effect subject to observed constraints; O(n) LP.
- **Real Wall:** Interval-censored outcomes produce wider bounds; partial identification is inherent in interval data.
- **Cross-Domain Aliases:** partial-identification-interval (statistics-probability), bounds-estimation (control-numerical-opt).
- **Notes:** Manski (1990); bounds are sharp if no additional assumptions; external information tightens bounds.

### [PRIM-074] causal-inference-with-networks
- **Atom/Composite:** Composite
- **Definition:** Causal inference with network interference: treatment spillover through network connections. Acyclic transfer, graph exposure mapping.
- **Cost Model:** Network interference modeling: exposure mapping = function from neighbor treatments to exposure level; O(n·d_neighbors).
- **Real Wall:** Interference violations bias standard causal estimates; SUTVA assumption fails; need design-based or model-based corrections.
- **Cross-Domain Aliases:** spillover-effects (statistics-probability), network-causality (distributed-systems).
- **Notes:** Aronow & Samii (2017); exposure mapping framework for interference; randomized block designs handle interference.

### [PRIM-075] experiment-design-causal
- **Atom/Composite:** Composite
- **Definition:** Randomized experiment design: randomization, blocking, matched pairs, factorial design. Reduces confounding through design.
- **Cost Model:** Power analysis: detect ATE of size δ with probability 1-β requires n = 2σ²/δ²; blocking reduces required n.
- **Real Wall:** Randomization balance on expectation but not always in realized sample; rerandomization improves balance.
- **Cross-Domain Aliases:** experimental-design (statistics-probability), causal-design (ml-training).
- **Notes:** Fisher (1935); randomization is the gold standard for causal identification.

### [PRIM-076] sharp-s玛dullah-bounds
- **Atom/Composite:** Composite
- **Definition:** Särndal-Lundqvist-Sjölander bounds: sharper bounds on causal effects using additional assumptions (monotonicity, no defiers).
- **Cost Model:** Monotonicity bound: ETE ≥ E[Y|X=1] - E[Y|X=0] under monotonicity; tighter than Manski bounds.
- **Real Wall:** Monotonicity is untestable; additional assumptions narrow bounds but risk misspecification.
- **Cross-Domain Aliases:** monotonicity-bounds (statistics-probability), assumption-dependent-bounds (agentic-reasoning).
- **Notes:** Särndal et al. (2012); assumptions like monotonicity of treatment effect tighten partial identification bounds.

### [PRIM-077] difference-in-differences-staggered
- **Atom/Composite:** Composite
- **Definition:** Staggered DiD (Callaway-Sant'Anna): treatment timing varies across units. ATT(g,t) = E[Y_t(g) - Y_t(0) | G=g] for units treated at time g.
- **Cost Model:** Callaway-Sant'Anna estimator: O(n·T·G) for groups × time × outcomes; doubly robust with outcome models.
- **Real Wall:** Staggered DiD with heterogeneous effects requires care: simple 2×2 DiD is biased with varying treatment timing.
- **Cross-Domain Aliases:** staggered-adoption (statistics-probability), event-study-design (ml-training).
- **Notes:** Callaway & Sant'Anna (2021); handles treatment effect heterogeneity across cohorts and time.

### [PRIM-078] synthetic-control-method
- **Atom/Composite:** Composite
- **Definition:** SCM (Synthetic Control Method): weighted combination of control units matching treated unit's pre-treatment trajectory.
- **Cost Model:** Convex optimization: minimize weighted MSE of pre-treatment outcomes; O(n_control²) optimization.
- **Real Wall:** SCM requires pre-treatment fit quality; placebo tests for robustness; donor pool selection matters.
- **Cross-Domain Aliases:** comparative-case-study (statistics-probability), causal-comparison (ml-training).
- **Notes:** Abadie et al. (2010); used for policy evaluation with single or few treated units.

### [PRIM-079] regression-discontinuity-fuzzy
- **Atom/Composite:** Composite
- **Definition:** Fuzzy RDD: treatment probability jumps discontinuously at cutoff but doesn't determine treatment deterministically. Local IV around cutoff.
- **Cost Model:** Fuzzy RDD: IV estimator using Z = 1(R ≥ c) as instrument for X; LATE = (E[Y | R=c⁺] - E[Y | R=c⁻]) / (E[X | R=c⁺] - E[X | R=c⁻]).
- **Real Wall:** Fuzzy RDD assumes monotonicity of treatment around cutoff; manipulation of running variable threatens validity.
- **Cross-Domain Aliases:** fuzzy-cutoff-design (statistics-probability), local-iv (control-numerical-opt).
- **Notes:** Imbens & Angrist (1994); fuzzy RDD = RDD + IV; estimates LATE at the cutoff.

### [PRIM-080] causal-inference-missing-data
- **Atom/Composite:** Composite
- **Definition:** Causal inference with missing data: missing at random (MAR) vs. missing not at random (MNAR). Inverse probability weighting for missingness.
- **Cost Model:** IPW for missing data: weight by P(observed | observed covariates); multiple imputation: O(m·n) for m imputations.
- **Real Wall:** MNAR requires sensitivity analysis; MCAR/MAR allow valid inference with proper missing data handling.
- **Cross-Domain Aliases:** missing-data-causal (statistics-probability), incomplete-observation (ml-training).
- **Notes:** Little & Rubin (2002); causal inference under missingness requires additional assumptions about missingness mechanism.

## 9. Semiparametric & Nonparametric Methods

### [PRIM-081] semiparametric-efficiency
- **Atom/Composite:** Composite
- **Definition:** Semiparametric efficiency bound: lowest variance achievable by any regular estimator. EIF (Efficient Influence Function) characterizes bound.
- **Cost Model:** EIF computation: derivative of nuisance parameter; influence function O(d) for d-dimensional nuisance.
- **Real Wall:** Efficient estimator achieves this bound; AIPW/TMLE achieve efficiency bound when nuisance models are correct.
- **Cross-Domain Aliases:** efficiency-bound (statistics-probability), influence-function (ml-training).
- **Notes:** Robins & Rotnitzky (1995); EIF = convolution of efficient estimator; DR estimators are asymptotically efficient.

### [PRIM-082] influence-function-estimation
- **Atom/Composite:** Composite
- **Definition:** Influence function (IF): first-order term in Von Mises expansion of estimator. Used for asymptotics and sensitivity analysis.
- **Cost Model:** IF computation O(d) for d parameters; plug into asymptotic variance formula.
- **Real Wall:** IF enables analytic standard errors for complex estimators; doubly robust estimators' IF has simple form.
- **Cross-Domain Aliases:** semiparametric-variance (statistics-probability), asy-distribution (ml-training).
- **Notes:** Hampel (1974); IF is the functional equivalent of leverage in regression.

### [PRIM-083] locally-efficient-estimation
- **Atom/Composite:** Composite
- **Definition:** Locally efficient estimator: achieves efficiency bound at correctly specified nuisance model, remains consistent otherwise (DR property).
- **Cost Model:** One-step correction: estimate θ̂, compute EIF, add correction term; O(n) beyond nuisance estimation.
- **Real Wall:** One-step estimators can overshoot; must use targeted estimators (TMLE) for bounded variance.
- **Cross-Domain Aliases:** one-step-correction (statistics-probability), dr-estimation (ml-training).
- **Notes:** Bickel et al. (1993); locally efficient = doubly robust + asymptotically optimal under correct nuisance.

### [PRIM-084] covariate-balance-check
- **Atom/Composite:** Primitive
- **Definition:** Covariate balance: treatment group and control group should have similar covariate distributions after adjustment. Standardized mean difference < 0.1.
- **Cost Model:** Balance check O(n·p) for p covariates; iterative proportional fitting if imbalance found.
- **Real Wall:** Perfect balance on observed covariates does not guarantee balance on unobserved; overlap assumption is key.
- **Cross-Domain Aliases:** balance-metric (statistics-probability), confounding-balance (ml-training).
- **Notes:** Imai & Ratkovic (2014); CBPS (Covariate Balancing Propensity Score) directly optimizes balance.

### [PRIM-085] overlap-assumption
- **Atom/Composite:** Primitive
- **Definition:** Overlap (positivity) assumption: P(A=1 | Z) ∈ (0, 1) for all Z in population. No deterministic treatment assignment by covariates.
- **Cost Model:** Overlap check: estimate propensity scores, check for extreme values near 0 or 1; O(n).
- **Real Wall:** Overlap violation = extrapolation; weights become extreme; trim or restrict analysis to overlap region.
- **Cross-Domain Aliases:** positivity-assumption (statistics-probability), common-support (ml-training).
- **Notes:** Crump et al. (2009); overlap region = units with non-zero probability of both treatment and control.

### [PRIM-086] unconfoundedness-assumption
- **Atom/Composite:** Primitive
- **Definition:** Unconfoundedness (ignorability): Y(1), Y(0) ⊥ A | X, W. Treatment assignment independent of potential outcomes given observed covariates.
- **Cost Model:** Unconfoundedness is untestable; sensitivity analysis (E-value) assesses robustness to violations.
- **Real Wall:** Unconfoundedness is the key assumption for observational causal inference; requires all confounders measured.
- **Cross-Domain Aliases:** ignorability (statistics-probability), conditional-independence (information-theory-coding).
- **Notes:** Rosenbaum & Rubin (1983); unconfoundedness + overlap enables causal identification via adjustment.

### [PRIM-087] causal-identification-complete
- **Atom/Composite:** Composite
- **Definition:** Complete identification: when is P(Y | do(X)) uniquely determined by P(X, Y, Z)? Causal identification theory studies these conditions.
- **Cost Model:** Complete identification: apply do-calculus until no do remains; ID algorithm does this systematically.
- **Real Wall:** Incomplete identification: multiple possible values consistent with observations; partial identification needed.
- **Cross-Domain Aliases:** identifiability-theory (logic-reasoning), do-calculus-completeness (agentic-reasoning).
- **Notes:** Pearl (2009); identification is the prerequisite for estimation; unidentifiable effects cannot be estimated.

### [PRIM-088] causal-discovery-functional
- **Atom/Composite:** Composite
- **Definition:** Functional causal models: X = f(Pa(X), N_X) where N_X is independent noise. Additive noise model (ANM), post-nonlinear (PNL) model.
- **Cost Model:** ANM: regress X on Pa(X), check independence of residual with Pa(X); O(n·d²) per pair.
- **Real Wall:** ANM identifies direction of causation for most pairs (except linear Gaussian case); requires non-Gaussianity for linear models.
- **Cross-Domain Aliases:** functional-causal-model (information-theory-coding), noise-independence (statistics-probability).
- **Notes:** Shimizu et al. (2006); functional causal models enable causal discovery without CI tests.

### [PRIM-089] additive-noise-model
- **Atom/Composite:** Composite
- **Definition:** ANM (Additive Noise Model): Y = f(X) + N where N ⊥ X. Independence of noise and cause identifies X → Y direction.
- **Cost Model:** ANM fitting: estimate f̂ via regression; check independence of N̂ = Y - f̂(X) with X; O(n) for regression + CI test.
- **Real Wall:** ANM fails if noise is not independent of cause; post-nonlinear model (PNL) relaxes this.
- **Cross-Domain Aliases:** independent-noise-causation (information-theory-coding), regression-noise-test (statistics-probability).
- **Notes:** Hoyer et al. (2009); ANM can distinguish cause from effect for many bivariate relationships.

### [PRIM-090] information-geometric-causality
- **Atom/Composite:** Composite
- **Definition:** IGCI (Information-Geometric Causal Inference): O-information criterion. Direction of causation from asymmetry in entropy production.
- **Cost Model:** IGCI: O(n·d²) for estimating entropy gradients; sign indicates causal direction.
- **Real Wall:** IGCI assumes functional relationship with additive noise; performance degrades with measurement noise.
- **Cross-Domain Aliases:** entropy-causation (information-theory-coding), geometric-causal (control-numerical-opt).
- **Notes:** Janzing et al. (2010); information-geometric approach uses entropy asymmetry to determine causal direction.

### [PRIM-091] causal-consistency-assumption
- **Atom/Composite:** Primitive
- **Definition:** Causal consistency: Y = Y(1)·A + Y(0)·(1-A). Observed outcome equals potential outcome under treatment actually received.
- **Cost Model:** Consistency is definitional; if violated (e.g., interference), must define potential outcomes differently.
- **Real Wall:** Violations of consistency: interference, multiple versions of treatment; requires extended causal framework.
- **Cross-Domain Aliases:** potential-outcome-consistency (statistics-probability), treatment-consistency (agentic-reasoning).
- **Notes:** Rubin (1990); consistency is the link between potential outcomes and observed data.

### [PRIM-092] sutva
- **Atom/Composite:** Primitive
- **Definition:** SUTVA (Stable Unit Treatment Value Assumption): (1) No interference: treatment of one unit doesn't affect another's outcome; (2) No hidden treatment versions.
- **Cost Model:** SUTVA is an assumption; violations require extending potential outcomes to include neighbor treatments.
- **Real Wall:** SUTVA is violated in network settings, general equilibrium, or multi-component treatments.
- **Cross-Domain Aliases:** interference-assumption (statistics-probability), unit-independence (agentic-reasoning).
- **Notes:** Rubin (1980); SUTVA is the foundational assumption connecting causal inference to potential outcomes.

### [PRIM-093] causal-effect-variance
- **Atom/Composite:** Composite
- **Definition:** Variance of treatment effects: heterogeneity of causal effects across units. Quantile treatment effects (QTE), ATT variance.
- **Cost Model:** QTE estimation: quantile regression on treatment × covariate interaction; O(n·log n) for quantile computation.
- **Real Wall:** Variance of ITE = variance of CATE + residual variance; treatment effect heterogeneity is key for personalization.
- **Cross-Domain Aliases:** heterogeneous-variance (statistics-probability), effect-dispersion (ml-training).
- **Notes:** Fan & Park (2010); quantile treatment effects reveal distribution of causal effects beyond the mean.

### [PRIM-094] causal-inference-with-text
- **Atom/Composite:** Composite
- **Definition:** Text as covariates or treatment in causal inference. Embeddings as high-dimensional covariates; supervised topic models for treatment.
- **Cost Model:** Text embedding: BERT/Oriented embeddings O(n·L) for L tokens; high-dimensional covariate adjustment O(n·d).
- **Real Wall:** Text features are high-dimensional and correlated; regularized causal inference needed.
- **Cross-Domain Aliases:** text-causal (ml-training), observational-text (information-theory-coding).
- **Notes:** Roberts et al. (2020); causal inference with text requires high-dimensional adjustment or deconfounding via text.

### [PRIM-095] instrumental-variable-pls
- **Atom/Composite:** Composite
- **Definition:** IV with聚类和学习者设计: optimal instrument construction from many weak predictors. Factor analysis for instruments.
- **Cost Model:** PCA/factor analysis on candidate instruments: O(n·p²); select top k factors as instruments.
- **Real Wall:** Many weak instruments: aggregate into few strong instruments; factor structure assumption needed.
- **Cross-Domain Aliases:** factor-iv (statistics-probability), instrument-aggregation (control-numerical-opt).
- **Notes:** Bai & Ng (2010); principal components of many instruments capture common variation for stronger first stage.

### [PRIM-096] causal-inference-multi-treatment
- **Atom/Composite:** Composite
- **Definition:** Multi-treatment causal inference: treatment A ∈ {1,...,K}. Inverse probability weighting generalizes: w_i = Σ_k 1/P(A=k | W) for each unit.
- **Cost Model:** K treatment IPW: O(K·n·p) for propensity estimation; K times more weights than binary case.
- **Real Wall:** Overlap deteriorates with more treatments; covariate balancing propensity score (CBPS) extends to multi-treatment.
- **Cross-Domain Aliases:** multi-valued-treatment (statistics-probability), polytomous-treatment (ml-training).
- **Notes:** Imbens (2000); multi-treatment generalization of propensity score; requires stronger overlap assumptions.

### [PRIM-097] causal-survival-analysis
- **Atom/Composite:** Composite
- **Definition:** Causal survival analysis: causal effect on time-to-event outcomes. Hazard ratio vs. survival causal contrast.IPCW (Inverse Probability Censoring Weighted).
- **Cost Model:** IPCW: weight by P(observed | covariates); Cox model for hazard; causal survival curves via g-formula.
- **Real Wall:** Censoring introduces selection bias; IPCW handles independent censoring; dependent censoring requires sensitivity analysis.
- **Cross-Domain Aliases:** causal-hazard (statistics-probability), survival-causal (ml-training).
- **Notes:** Robins & Finkelstein (2000); causal survival analysis combines survival analysis with IPTW/g-formula.

### [PRIM-098] marginal-effect-causal
- **Atom/Composite:** Composite
- **Definition:** Marginal causal effect: causal effect averaged over population distribution of confounders. ATT, ATC, ATE as marginal effects.
- **Cost Model:** Marginal effect = integral over covariate distribution; Monte Carlo approximation or IPW O(n).
- **Real Wall:** Marginal effects require proper standardization; inverse probability weighting provides direct marginal estimates.
- **Cross-Domain Aliases:** population-average-effect (statistics-probability), marginal-contrast (ml-training).
- **Notes:** Rubin (2005); marginal vs. conditional effects differ when model is misspecified; marginal effects are policy-relevant.

### [PRIM-099] causal-inference-confounders
- **Atom/Composite:** Primitive
- **Definition:** Confounder: common cause of treatment and outcome. Adjusting for confounders blocks backdoor paths and identifies causal effect.
- **Cost Model:** Confounder identification: prior knowledge + automated discovery (causal discovery); CI tests validate.
- **Real Wall:** Measuring all confounders is difficult; residual confounding bias persists even with many covariates.
- **Cross-Domain Aliases:** common-cause (statistics-probability), backdoor-path (information-theory-coding).
- **Notes:** Pearl (2009); confounders are the central concept linking causation to statistical association.

### [PRIM-100] bad-control-rule
- **Atom/Composite:** Composite
- **Definition:** Bad control rule: adjusting for a variable that is a descendant of the treatment biases the estimate (over-adjustment).
- **Cost Model:** DAG analysis: check if variable is descendant of X before adjusting; d-separation test.
- **Real Wall:** Adjusting for mediators removes part of the causal effect; adjusting for colliders induces selection bias.
- **Cross-Domain Aliases:** over-adjustment (statistics-probability), collider-bias (information-theory-coding).
- **Notes:** Pearl (2009); collider adjustment is a common mistake in observational causal inference.

## 10. Advanced Topics

### [PRIM-101] causal-discovery-score-based
- **Atom/Composite:** Composite
- **Definition:** Score-based causal discovery: maximize likelihood/BIC score over DAG structures. Decomposable scores enable local search.
- **Cost Model:** BIC score: log-likelihood - (d/2) log n; DAG enumeration intractable; greedy/local search O(n·d²).
- **Real Wall:** Score-based methods are consistent under correct score specification; may prefer wrong MEC under finite samples.
- **Cross-Domain Aliases:** dag-scoring (ml-training), structure-optimization (information-theory-coding).
- **Notes:** Chickering (2002); score equivalence: BIC is score-equivalent (same score for all DAGs in MEC).

### [PRIM-102] dynamic-treatment-regimen
- **Atom/Composite:** Composite
- **Definition:** Dynamic treatment regimen (DTR): treatment rule that adapts based on patient history. Personalized medicine via causal inference.
- **Cost Model:** Q-learning: estimate Q-function for each decision point; backward induction O(T·n·d·actions).
- **Real Wall:** DTR evaluation requires marginal structural model or g-computation; offline evaluation is challenging.
- **Cross-Domain Aliases:** adaptive-treatment (ml-training), sequential-decision (agentic-reasoning).
- **Notes:** Murphy et al. (2001); DTRs generalize fixed treatments to personalized, adaptive policies.

### [PRIM-103] causal-inference-with-networks-interference
- **Atom/Composite:** Composite
- **Definition:** Interference in networks: unit i's treatment affects unit j's outcome if j is connected to i. Graphical models for interference.
- **Cost Model:** Exposure mapping: define exposure level as function of neighbor treatments; O(n·degree) to compute.
- **Real Wall:** SUTVA violated; needs network-aware designs or models; clustered designs reduce interference.
- **Cross-Domain Aliases:** network-spillover (statistics-probability), social-interference (distributed-systems).
- **Notes:** Aronow & Samii (2017); exposure mappings formalize interference structure; partial interference is common.

### [PRIM-104] placebo-test-causal
- **Atom/Composite:** Composite
- **Definition:** Placebo tests for causal validity: test effect on pre-treatment outcomes (pre-trends), falsification endpoints, balance checks.
- **Cost Model:** Pre-treatment outcome test: DiD on pre-treatment periods should show zero effect; O(n) regression.
- **Real Wall:** Placebo tests are necessary but not sufficient; multiple placebo tests strengthen credibility.
- **Cross-Domain Aliases:** falsification-test (statistics-probability), robustness-check (ml-training).
- **Notes:** Angrist & Pischke (2009); placebo tests are key for credible quasi-experimental evidence.

### [PRIM-105] causal-inference-clustered
- **Atom/Composite:** Composite
- **Definition:** Clustered causal inference: grouped units (schools, hospitals) with intra-cluster correlation. Cluster-robust SEs, design effects.
- **Cost Model:** Clustered SEs: adjust variance for within-cluster correlation; design effect = 1 + (m-1)ρ for m cluster size, ρ ICC.
- **Real Wall:** Ignoring clustering leads to anti-conservative inference; cluster-level treatment assignment reduces bias.
- **Cross-Domain Aliases:** multi-level-causal (statistics-probability), hierarchical-causal (ml-training).
- **Notes:** Abadie et al. (2017); cluster-robust inference requires sufficient number of clusters (>50 for valid inference).

### [PRIM-106] instrumental-variable-heterogeneous
- **Atom/Composite:** Composite
- **Definition:** Heterogeneous IV effects: LATE varies across compliers. Local IV: instrument affects different subpopulations differently.
- **Cost Model:** LATE = weighted average of ITEs; weights = probability of being complier in each subgroup.
- **Real Wall:** IV identifies LATE, not ATE; heterogeneous LATEs across subgroups require stronger assumptions.
- **Cross-Domain Aliases:** local-iv-effect (statistics-probability), complier-heterogeneity (control-numerical-opt).
- **Notes:** Angrist et al. (1996); IV is a weighted average of treatment effects with weights proportional to compliance.

### [PRIM-107] causal-discovery-continuous
- **Atom/Composite:** Composite
- **Definition:** Causal discovery for continuous variables: NOTEARS (NO TEARS from structural equation models). Continuous optimization over DAG adjacency matrix.
- **Cost Model:** NOTEARS: minimize loss + λ||A||_1 subject to DAG constraint via augmented Lagrangian; O(n·d²·iterations).
- **Real Wall:** NOTEARS is a continuous relaxation; threshold needed to convert to DAG; may produce cyclic graphs.
- **Cross-Domain Aliases:** continuous-structure-learning (ml-training), dag-optimization (information-theory-coding).
- **Notes:** Zheng et al. (2018); NOTEARS enables gradient-based DAG learning without discrete search.

### [PRIM-108] causal-validity-testing
- **Atom/Composite:** Composite
- **Definition:** Causal validity testing: use experimental data to validate observational causal estimates. P-score alignment, concordance.
- **Cost Model:** Concordance: compare observational estimate distribution with experimental estimate distribution; O(n) comparison.
- **Real Wall:** Requires experimental validation data; concordance assessment guides trust in observational findings.
- **Cross-Domain Aliases:** validity-assessment (statistics-probability), estimate-validation (ml-training).
- **Notes:** Angrist & Pischke (2009); triangulation using multiple identification strategies strengthens causal conclusions.

### [PRIM-109] causal-ensemble-methods
- **Atom/Composite:** Composite
- **Definition:** Ensemble causal inference: combine multiple causal estimators (matching, IPW, regression) for robustness.
- **Cost Model:** Stacking: fit meta-learner on base estimators' predictions; O(n·m) for m base estimators.
- **Real Wall:** Ensemble of biased estimators can still be biased; need diversity and at least some unbiased components.
- **Cross-Domain Aliases:** causal-model-combination (ml-training), estimator-ensemble (statistics-probability).
- **Notes:** Künzel et al. (2019); ensemble meta-learners (R-learner) combine strengths of different base methods.

### [PRIM-110] causal-inference-uncertainty
- **Atom/Composite:** Composite
- **Definition:** Uncertainty quantification in causal inference: bootstrap, influence function, Bayesian posterior for causal effects.
- **Cost Model:** Bootstrap: O(B·n) for B bootstrap samples; Bayesian: MCMC for posterior O(n·iterations).
- **Real Wall:** Bootstrap may be invalid under non-regular conditions (e.g., near-zero propensity scores); use wild bootstrap.
- **Cross-Domain Aliases:** causal-variance-estimation (statistics-probability), uncertainty-quantification (ml-training).
- **Notes:** Efron (1979); uncertainty in causal estimates combines estimation uncertainty and assumption uncertainty.

### [PRIM-111] causal-inference-design-based
- **Atom/Composite:** Composite
- **Definition:** Design-based causal inference: randomization as the identification engine; design controls for confounders by construction.
- **Cost Model:** Randomized experiment: balance on all confounders in expectation; power analysis O(1) per design parameter.
- **Real Wall:** Design-based inference is robust to model misspecification; implementation threats (non-compliance, attrition) require attention.
- **Cross-Domain Aliases:** randomized-experiment (statistics-probability), design-control (agentic-reasoning).
- **Notes:** Fisher (1935); randomization is the gold standard; design-based inference makes assumptions transparent.

### [PRIM-112] causal-effect-measurement-error
- **Atom/Composite:** Composite
- **Definition:** Measurement error in causal inference: mismeasured treatment, outcome, or confounders. Attenuation bias, SIMEX correction.
- **Cost Model:** SIMEX: simulate additional measurement error, extrapolate to zero-error limit; O(B·n) for B simulations.
- **Real Wall:** Measurement error in treatment biases IPW; measurement error in outcome biases regression adjustment.
- **Cross-Domain Aliases:** error-in-variables (statistics-probability), mismeasurement-causal (ml-training).
- **Notes:** Carroll et al. (2006); measurement error generally biases causal estimates toward the null.

### [PRIM-113] causal-inference-selection
- **Atom/Composite:** Composite
- **Definition:** Sample selection bias: analysis restricted to subset of population; Heckman correction for selection on unobservables.
- **Cost Model:** Heckman correction: estimate selection equation (probit), compute inverse Mills ratio, include in outcome regression O(n).
- **Real Wall:** Heckman requires exclusion restriction (instrument affecting selection but not outcome); weak instrument = unreliable correction.
- **Cross-Domain Aliases:** selection-bias-correction (statistics-probability), sample-selection (ml-training).
- **Notes:** Heckman (1979); selection on observables (MAR) allows IPW; selection on unobservables needs stronger assumptions.

### [PRIM-114] causal-inference-longitudinal-g
- **Atom/Composite:** Composite
- **Definition:** G-formula for longitudinal treatments: sequential g-computation. E[Y^{do(a)}] = E_{Z_0}[ E_{Z_1}[ ... E_{Z_t}[ Y | do(a_t) ] ... ] ].
- **Cost Model:** Sequential regression: model E[Y | history, treatment] at each time step; g-formula = Monte Carlo integration O(n·T).
- **Real Wall:** G-formula is sensitive to model misspecification; doubly robust g-formula improves robustness.
- **Cross-Domain Aliases:** sequential-g-computation (statistics-probability), marginal-structural (ml-training).
- **Notes:** Robins (1986); g-formula generalizes standardization to time-varying treatments and confounders.

### [PRIM-115] causal-effect-heterogeneity-ml
- **Atom/Composite:** Composite
- **Definition:** ML for CATE: causal trees, causal forests, T-learner, X-learner, R-learner. Estimate treatment effect heterogeneity from data.
- **Cost Model:** Causal tree: honest splitting with treatment effect criterion O(n·d); R-learner: minimize oracle risk O(n·d).
- **Real Wall:** Overfitting: treatment effect signal is typically small relative to outcome noise; regularization is essential.
- **Cross-Domain Aliases:** machine-learning-causal (ml-training), adaptive-heterogeneity (statistics-probability).
- **Notes:** Athey & Imbens (2016); causal trees provide interpretable subgroup treatment effects.

### [PRIM-116] do-calculus-completeness
- **Atom/Composite:** Composite
- **Definition:** Completeness of do-calculus: if no sequence of do-calculus rules reduces expression, then causal effect is not identifiable.
- **Cost Model:** Do-calculus completeness: recursive algorithm; unidentifiable expressions trigger z-identification or partial identification.
- **Real Wall:** Completeness is for DAG identification; more general settings (counterfactuals) may have different completeness results.
- **Cross-Domain Aliases:** identification-completeness (logic-reasoning), causal-calculus-closure (agentic-reasoning).
- **Notes:** Shpitser & Pearl (2006); ID algorithm implements complete identification for DAG causal effects.

### [PRIM-117] counterfactual-surgery
- **Atom/Composite:** Composite
- **Definition:** Causal surgery (graph surgery): remove edges corresponding to do-operator. G_δ = graph with all incoming arrows to do(X) removed.
- **Cost Model:** Graph surgery O(|E|) to remove edges; post-intervention distribution computed on G_δ.
- **Real Wall:** Causal surgery is the operational meaning of do-operator; corresponds to physical intervention in system.
- **Cross-Domain Aliases:** intervention-graph (logic-reasoning), graph-modification (information-theory-coding).
- **Notes:** Pearl (2009); do-operator = perform surgery on causal graph, then compute conditional distribution.

### [PRIM-118] causal-effect-scaling
- **Atom/Composite:** Composite
- **Definition:** Causal effect scaling: standardize effect to population of interest. Inverse probability weighting achieves population standardization.
- **Cost Model:** Standardization: compute stratum-specific effects, weight by reference population distribution O(n).
- **Real Wall:** Different standardization populations yield different effect estimates; report which population is standardized to.
- **Cross-Domain Aliases:** effect-standardization (statistics-probability), population-standardization (ml-training).
- **Notes:** Hernan & Robins (2020); standardization is the nonparametric generalization of direct adjustment.

### [PRIM-119] causal-inference-multi-outcome
- **Atom/Composite:** Composite
- **Definition:** Multi-outcome causal inference: vector-valued outcomes Y ∈ ℝ^k. Joint vs. marginal effect analysis.
- **Cost Model:** Joint estimation: multivariate regression with IPW; O(n·k²) for k outcomes; multiple testing correction needed.
- **Real Wall:** Multiple outcomes inflate family-wise error rate; Bonferroni correction is conservative; FDR control preferred.
- **Cross-Domain Aliases:** multivariate-causal (statistics-probability), multi-response-causal (ml-training).
- **Notes:** Dickerman & Hernán (2020); simultaneous inference on multiple outcomes requires multiplicity adjustment.

### [PRIM-120] causal-inference-under-missingness
- **Atom/Composite:** Composite
- **Definition:** Missing not at random (MNAR) in causal inference: missingness depends on unobserved potential outcomes. Sensitivity analysis required.
- **Cost Model:** Pattern-mixture models: model outcome distribution by missingness pattern; O(n·patterns·d).
- **Real Wall:** MNAR is unfalsifiable from observed data alone; need external information or assumptions for identification.
- **Cross-Domain Aliases:** mnar-causal (statistics-probability), missing-not-at-random (ml-training).
- **Notes:** Little & Rubin (2002); causal inference with MNAR requires sensitivity analysis to unmeasured confounding + missingness.

### [PRIM-121] causal-inference-conceptual
- **Atom/Composite:** Composite
- **Definition:** Conceptual causal inference: define causal concepts (fairness, discrimination) precisely. Counterfactual fairness, disparate impact.
- **Cost Model:** Counterfactual fairness: P(Y_û(A=1) | C=c) = P(Y | C=c) for sensitive attribute A. O(n) computation.
- **Real Wall:** Different conceptual definitions of fairness (individual, group, counterfactual) give different conclusions.
- **Cross-Domain Aliases:** algorithmic-fairness (agentic-reasoning), discrimination-causal (statistics-probability).
- **Notes:** Kusner et al. (2017); counterfactual fairness operationalizes individual fairness causally.

### [PRIM-122] observational-study-design
- **Atom/Composite:** Composite
- **Definition:** Observational study design: emulate randomized experiment through design choices. P-score matching, stratification, weighting.
- **Cost Model:** Design emulation: balance checking O(n·p); iterative refinement until balance achieved.
- **Real Wall:** Good design does not guarantee valid inference; still need correct modeling of outcome for precision.
- **Cross-Domain Aliases:** quasi-experiment-design (statistics-probability), design-matching (ml-training).
- **Notes:** Rosenbaum (2002); observational study design parallels randomized experiment design.

### [PRIM-123] causal-effect-viz
- **Atom/Composite:** Primitive
- **Definition:** Causal effect visualization: DAG diagrams, causal effect plots, dose-response curves, spaghetti plots for heterogeneity.
- **Cost Model:** DAG rendering O(|V| + |E|); causal forest plots O(n); dose-response O(n·grid).
- **Real Wall:** Clear visualization communicates uncertainty and assumptions; DAG + outcome plots are complementary.
- **Cross-Domain Aliases:** causal-plot (statistics-probability), effect-visualization (ml-training).
- **Notes:** Pearl (2009); DAG visualization is essential for communicating causal assumptions to non-experts.

### [PRIM-124] mediation-decomposition-nonlinear
- **Atom/Composite:** Composite
- **Definition:** Nonlinear mediation: when outcome model is nonlinear, natural direct and indirect effects are not additive. Counterfactual definition needed.
- **Cost Model:** Nonlinear mediation: E[Y(1) - Y(0)] = NDE + NIE only if additive model; otherwise compute via simulation.
- **Real Wall:** Interaction between treatment and mediator complicates mediation decomposition; proportion mediated is scale-dependent.
- **Cross-Domain Aliases:** nonlinear-mediation (statistics-probability), causal-decomposition (agentic-reasoning).
- **Notes:** VanderWeele (2015); natural direct/indirect effects well-defined for any model via counterfactuals.

### [PRIM-125] do-calculus-action-observation
- **Atom/Composite:** Primitive
- **Definition:** Do-calculus rule 3 (action/observation exchange): P(Y | do(X), Z, W) = P(Y | do(X), Z, W, do(Z)) under condition. Enables absorption of observations into do.
- **Cost Model:** Apply when Z ⊥ Y | do(X), W in G_δX; check d-separation in modified graph.
- **Real Wall:** Rule 3 is less commonly applicable than rules 1 and 2; combines with other rules for complex expressions.
- **Cross-Domain Aliases:** do-calculus-rule3 (logic-reasoning), intervention-absorption (agentic-reasoning).
- **Notes:** Pearl (1995); three rules of do-calculus are complete for causal identification.

### [PRIM-126] causal-discovery-post-nonlinear
- **Atom/Composite:** Composite
- **Definition:** Post-nonlinear (PNL) causal model: Y = g(f(X) + N) where N ⊥ X. More general than ANM; can identify causal direction.
- **Cost Model:** PNL estimation: ICA for nonlinear transformation g, ANM for f; O(n·d²) per pair.
- **Real Wall:** PNL relaxes the independence-of-noise assumption of ANM; requires identifiable nonlinear functions.
- **Cross-Domain Aliases:** pnl-model (information-theory-coding), nonlinear-causal (control-numerical-opt).
- **Notes:** Zhang & Hyvärinen (2009); PNL captures post-nonlinearity in causal mechanisms.

### [PRIM-127] causal-effect-decomposition-interaction
- **Atom/Composite:** Composite
- **Definition:** Causal interaction: when treatment effect depends on another variable (moderator). Controlled direct effect, reference interaction.
- **Cost Model:** Interaction effect: β_3 in E[Y] = β_0 + β_1 X + β_2 M + β_3 X·M; decompose into pure direct and joint effects.
- **Real Wall:** Interaction makes mediation decomposition non-additive; path-specific effects needed for interpretation.
- **Cross-Domain Aliases:** causal-interaction (statistics-probability), moderation-effect (ml-training).
- **Notes:** VanderWeele (2015); interaction complicates causal decomposition; multiple decomposition formulas exist.

### [PRIM-128] causal-inference-spatial
- **Atom/Composite:** Composite
- **Definition:** Spatial causal inference: treatment effects spill over geographically. Spatial lag model, geographically weighted causal inference.
- **Cost Model:** Spatial weights matrix W: O(n²) to compute; spatial lag model: O(n³) for MLE.
- **Real Wall:** Spatial interference is common (disease spread, policy diffusion); requires spatial network structure.
- **Cross-Domain Aliases:** spatial-spillover (statistics-probability), geographic-causal (ml-training).
- **Notes:** Klitkou et al. (2015); spatial causal inference extends causal inference to geographic interference.

### [PRIM-129] causal-inference-synthetic
- **Atom/Composite:** Composite
- **Definition:** Synthetic cohort method: create synthetic control from weighted combination of untreated units matching treated unit's pre-treatment trajectory.
- **Cost Model:** Optimization: minimize MSE of pre-treatment outcomes with nonnegative weights summing to 1; convex O(n²).
- **Real Wall:** SCM requires good pre-treatment fit; multiple treated units: synthetic control method for each or hierarchical.
- **Cross-Domain Aliases:** synthetic-control-cohort (statistics-probability), causal-comparison (ml-training).
- **Notes:** Abadie et al. (2010); synthetic control is the leading method for comparative case studies.

### [PRIM-130] causal-external-validity
- **Atom/Composite:** Composite
- **Definition:** External validity: causal effect estimated in one population applies to another. Transportability, generalization, sampling.
- **Cost Model:** Transportability formula: P(Y | do(X))_target = f(P(Y | do(X))_source, selection bias). O(1) if parameters known.
- **Real Wall:** Extrapolation beyond support of source data is impossible without assumptions; overlap needed for valid transport.
- **Cross-Domain Aliases:** generalization-causal (ml-training), population-shift (agentic-reasoning).
- **Notes:** Pearl & Bareinboim (2011); external validity = causal transportability across domains.

### [PRIM-131] causal-benchmarking
- **Atom/Composite:** Composite
- **Definition:** Causal benchmarking: evaluate causal inference methods on synthetic data with known ground truth. TCEP, ACIC benchmarks.
- **Cost Model:** Benchmark generation: SCM + data simulation O(n·d·simulations); ground truth comparison O(n).
- **Real Wall:** Synthetic benchmarks may not reflect real-data complexity; semi-synthetic benchmarks (real covariates, simulated treatment) bridge gap.
- **Cross-Domain Aliases:** causal-evaluation (ml-training), benchmark-design (statistics-probability).
- **Notes:** Dorie et al. (2019) ACIC; Hahn et al. (2020) TCEP; benchmarking is essential for method development.

### [PRIM-132] causal-metrics
- **Atom/Composite:** Composite
- **Definition:** Causal inference metrics: PEHE (Precision in Estimation of Heterogeneous Effects), ATE error, MAIC, overlap weighting quality.
- **Cost Model:** PEHE = E[ (τ(X) - τ̂(X))² ]; computed on test set with known CATE; O(n) per evaluation.
- **Real Wall:** Different metrics evaluate different aspects (estimation vs. ranking vs. selection); no single metric suffices.
- **Cross-Domain Aliases:** causal-evaluation-metric (ml-training), effect-estimation-error (statistics-probability).
- **Notes:** Hill (2011); PEHE is the gold standard for CATE estimation quality.

### [PRIM-133] causal-inference-multiverse
- **Atom/Composite:** Composite
- **Definition:** Multiverse analysis for causal inference: report causal estimates across all reasonable analytical choices. Sensitivity to specification.
- **Cost Model:** Multiverse: O(m) analyses for m analytical choices; can quickly produce thousands of estimates.
- **Real Wall:** Multiverse reduces publication bias from specification searching; but must clearly define universe boundaries.
- **Cross-Domain Aliases:** specification-curve (statistics-probability), robustness-across-specs (ml-training).
- **Notes:** Steegen et al. (2016); multiverse analysis reveals how sensitive causal conclusions are to analyst choices.

### [PRIM-134] causal-decision-theory
- **Atom/Composite:** Composite
- **Definition:** Causal decision theory: choose actions to maximize expected causal utility. Causal utility vs. evidential decision theory.
- **Cost Model:** Causal DT: E_U[U(a, Y_{do(a)}(u))]; requires causal model for counterfactual outcomes.
- **Real Wall:** Causal DT differs from evidential DT when actions affect information (Newcomb-like problems).
- **Cross-Domain Aliases:** causal-choice (agentic-reasoning), decision-under-causation (logic-reasoning).
- **Notes:** Pearl (2000); causal decision theory uses do-operator to evaluate consequences of actions.

### [PRIM-135] causal-inference-econometrics
- **Atom/Composite:** Composite
- **Definition:** Econometrics meets causal inference: regression discontinuity (Hahn, Todd, van der Klaauw), synthetic control (Abadie), IV (Angrist).
- **Cost Model:** Each econometric method has specific estimation procedure; software (rdrobust, Synth, ivreg) O(n) to O(n²).
- **Real Wall:** Econometric causal methods emphasize identification strategy + estimation; credible inference requires both.
- **Cross-Domain Aliases:** econometric-causal (statistics-probability), quasi-experiment (control-numerical-opt).
- **Notes:** Angrist & Pischke (2009); \"credibility revolution\" in econometrics through quasi-experimental methods.

### [PRIM-136] causal-representation-learning-evaluation
- **Atom/Composite:** Composite
- **Definition:** Causal representation learning evaluation: downstream task performance, disentanglement metrics (MIG, DCI, Modularity).
- **Cost Model:** MIG (Mutual Information Gap): O(k²) for k latent factors; DCI: O(k²) for discoverability and complexity.
- **Real Wall:** Disentanglement metrics may not correlate with downstream causal performance; task-specific evaluation is key.
- **Cross-Domain Aliases:** representation-evaluation (ml-training), causal-disentanglement-metrics (information-theory-coding).
- **Notes:** Chen et al. (2018); disentanglement ≠ causality; need downstream causal tasks to validate representations.

### [PRIM-137] causal-inference-finite-sample
- **Atom/Composite:** Composite
- **Definition:** Finite sample properties of causal estimators: bias, variance, MSE. Finite sample vs. asymptotic inference.
- **Cost Model:** Finite sample MSE: analytic bias approximation for IPW O(1); simulation for complex estimators O(B·n).
- **Real Wall:** Asymptotic approximations fail in finite samples; bootstrap can be invalid near positivity violations.
- **Cross-Domain Aliases:** finite-sample-causal (statistics-probability), small-sample-inference (ml-training).
- **Notes:** Hernan & Robins (2020); finite sample bias of IPW is proportional to 1/n; doubly robust methods reduce this.

### [PRIM-138] causal-inference-publication-bias
- **Atom/Composite:** Composite
- **Definition:** Publication bias in causal studies: funnel asymmetry, p-curve, selection models. Assess veracity of published causal estimates.
- **Cost Model:** P-curve: compute distribution of p-values; full p-curve = evidential value; O(n) for study collection.
- **Real Wall:** Publication bias distorts literature; p-hacking and HARKing compound the problem.
- **Cross-Domain Aliases:** selection-bias-literature (statistics-probability), evidence-assessment (agentic-reasoning).
- **Notes:** Simonsohn et al. (2014); p-curve detects whether reported effects are genuine or fabricated.

### [PRIM-139] causal-inference-replication
- **Atom/Composite:** Composite
- **Definition:** Causal replication: design replication vs. conceptual replication. Large-N replication of quasi-experimental effects.
- **Cost Model:** Replication power: detect ATE of size δ with probability 1-β; n = 2σ²(Φ^{-1}(1-β/2) + Φ^{-1}(1-α/2))²/δ².
- **Real Wall:** Conceptual replication across different contexts is more valuable than exact replication for causal claims.
- **Cross-Domain Aliases:** replication-design (statistics-probability), causal-credibility (agentic-reasoning).
- **Notes:** Open Science Collaboration (2015); replication crisis in social science motivates stronger causal designs.

### [PRIM-140] causal-inference-causal-discovery
- **Atom/Composite:** Composite
- **Definition:** Integration of causal inference and discovery: use causal inference assumptions to guide discovery; use discovery to suggest inference strategies.
- **Cost Model:** Iterative: discovery → inference → validation → discovery; each cycle O(n·d²).
- **Real Wall:** Integration is iterative; discovery assumptions inform inference but must be validated empirically.
- **Cross-Domain Aliases:** causal-learning-cycle (information-theory-coding), integrated-causal-analysis (agentic-reasoning).
- **Notes:** Pearl (2009); causal inference (identification) and causal discovery (learning) are complementary disciplines.

### [PRIM-141] causal-inference-sensitivity-bounds
- **Atom/Composite:** Composite
- **Definition:** Sensitivity analysis bounds: E-value and partial R² for unmeasured confounding. Assess how strong confounding must be to nullify conclusion.
- **Cost Model:** E-value = RR + √(RR·(RR-1)); partial R² from regression of treatment on outcome controlling for covariates.
- **Real Wall:** Sensitivity analysis is qualitative guidance; quantitative bounds require specifying confounder-outcome relationship.
- **Cross-Domain Aliases:** confounding-sensitivity (statistics-probability), robustness-analysis (ml-training).
- **Notes:** VanderWeele & Ding (2017); E-value provides interpretable threshold for confounding strength.

### [PRIM-142] causal-inference-conceptual-fairness
- **Atom/Composite:** Composite
- **Definition:** Causal fairness definitions: counterfactual fairness (Kusner), parity fairness (Chouldechova), calibration fairness.
- **Cost Model:** Counterfactual fairness: compute E[Y_û(A=0) | S=s]; equal across s = counterfactual fair. O(n) for each sensitive group.
- **Real Wall:** Different fairness definitions are mutually incompatible (Impossibility theorem); must choose based on context.
- **Cross-Domain Aliases:** algorithmic-fairness-causal (agentic-reasoning), discrimination-measurement (statistics-probability).
- **Notes:** Kleinberg et al. (2016); causal framework enables precise definition of fairness criteria.

### [PRIM-143] causal-experiments-field
- **Atom/Composite:** Composite
- **Definition:** Field experiments: randomized trials in real-world settings. A/B testing, RCTs, stepped wedge designs.
- **Cost Model:** A/B test: O(n) per test; power analysis determines required n; sequential testing changes significance thresholds.
- **Real Wall:** Field experiments face external validity concerns; lab experiments have higher internal validity but less ecological validity.
- **Cross-Domain Aliases:** randomized-field-trial (statistics-probability), ab-testing-causal (ml-training).
- **Notes:** List et al. (2020); field experiments are gold standard but costly; digital platforms enable massive online experiments.

### [PRIM-144] causal-inference-hybrid
- **Atom/Composite:** Composite
- **Definition:** Hybrid designs: combine experimental and observational data. External validity via transport, instrumental variables from experiments.
- **Cost Model:** Transport formula: P(Y | do(X))_target = combination of source and target data; O(n_source + n_target).
- **Real Wall:** Hybrid approaches leverage strengths of both designs; need assumptions about transportability of causal effects.
- **Cross-Domain Aliases:** combined-design (statistics-probability), mixed-causal (ml-training).
- **Notes:** Egami & Pearl (2016); hybrid designs address external validity by integrating experimental and observational evidence.

### [PRIM-145] causal-inference-causal-prior
- **Atom/Composite:** Composite
- **Definition:** Bayesian causal inference: prior over causal effects, DAG structures, or potential outcomes. Posterior causal effect distribution.
- **Cost Model:** Bayesian causal model: prior P(G) × P(D | G); posterior MCMC over DAG space O(n·iterations).
- **Real Wall:** Prior specification is subjective; Bayesian causal inference quantifies uncertainty but doesn't eliminate assumption dependence.
- **Cross-Domain Aliases:** bayesian-causal (statistics-probability), posterior-causal (ml-training).
- **Notes:** Madigan et al. (1995); Bayesian approach to causal inference naturally handles uncertainty in DAG structure.

### [PRIM-146] causal-inference-decomposition-total
- **Atom/Composite:** Composite
- **Definition:** Total effect decomposition: TE = DE + IE + interaction. Four-way decomposition for binary/continuous outcomes.
- **Cost Model:** Four-way decomposition: TE = (pure direct) + (mediation) + (interaction) + (both); each component computable via formulas.
- **Real Wall:** Decomposition depends on scale (additive vs multiplicative) and model specification; no single correct decomposition.
- **Cross-Domain Aliases:** effect-decomposition (statistics-probability), causal-attribution (agentic-reasoning).
- **Notes:** VanderWeele (2013); four-way decomposition generalizes mediation analysis to include interaction.

### [PRIM-147] causal-inference-sequencing
- **Atom/Composite:** Composite
- **Definition:** Causal sequencing: order of discovery → identification → estimation → validation. Each step has its own methods and assumptions.
- **Cost Model:** Sequential pipeline: discovery O(n·d²), identification O(d³), estimation O(n), validation O(n).
- **Real Wall:** Mistakes in early stages propagate; DAG specification errors are not fixed by sophisticated estimation.
- **Cross-Domain Aliases:** causal-pipeline (agentic-reasoning), workflow-causal (ml-training).
- **Notes:** Pearl (2009); causal inference is a sequential process from assumptions to conclusions.

### [PRIM-148] causal-discovery-continuous-optimization
- **Atom/Composite:** Composite
- **Definition:** Continuous DAG optimization: NOTEARS, DAG-GNN. Smooth DAG constraint via acyclicity constraint (h(A) = tr(e^{A∘A}) - d = 0).
- **Cost Model:** NOTEARS: augmented Lagrangian O(d³) per iteration; DAG-GNN: neural network O(n·d·epochs).
- **Real Wall:** NOTEARS may produce near-cyclic graphs; threshold and threshold-free assessment needed.
- **Cross-Domain Aliases:** neural-causal-discovery (ml-training), differentiable-dag (information-theory-coding).
- **Notes:** Zheng et al. (2018); continuous optimization avoids combinatorial search over DAG space.

### [PRIM-149] causal-inference-dose-response
- **Atom/Composite:** Composite
- **Definition:** Dose-response curve: causal effect as function of treatment dose. Causal dose-response, instrumental dose-response.
- **Cost Model:** Dose-response estimation: local polynomial regression at each dose level; O(n·doses) for grid.
- **Real Wall:** Monotonicity assumption (higher dose → higher effect) can improve estimation; shape restrictions add information.
- **Cross-Domain Aliases:** causal-dose-response (statistics-probability), continuous-treatment (ml-training).
- **Notes:** Hirano & Imbens (2005); generalized propensity score for continuous treatment dose-response.

### [PRIM-150] causal-inference-causal-triangulation
- **Atom/Composite:** Primitive
- **Definition:** Causal triangulation: use multiple identification strategies (IV, DiD, RDD) for same causal question. Concordance as evidence.
- **Cost Model:** Multiple estimates: O(m·n) for m methods; concordance check = correlation across estimates.
- **Real Wall:** Concordant estimates across different designs strengthen causal inference; discordant estimates reveal hidden assumptions.
- **Cross-Domain Aliases:** multiple-evidence (statistics-probability), triangulation-causal (agentic-reasoning).
- **Notes:** Angrist & Pischke (2009); triangulation from multiple identification strategies is the gold standard for credibility.

### [PRIM-151] causal-inference-causal-mechanism
- **Atom/Composite:** Composite
- **Definition:** Causal mechanism analysis: why does treatment work? Mediator analysis, mechanism comparison, pathway decomposition.
- **Cost Model:** Mediation analysis: E[Y(1) - Y(0)] = NDE + NIE; sequential ignorability assumptions O(n).
- **Real Wall:** Mechanisms are probabilistic; mechanism decomposition is sensitive to model assumptions.
- **Cross-Domain Aliases:** mechanism-analysis (agentic-reasoning), causal-pathway (statistics-probability).
- **Notes:** Pearl (2001); understanding mechanisms enables intervention improvement beyond just predicting effects.

### [PRIM-152] causal-inference-causal-generalization
- **Atom/Composite:** Composite
- **Definition:** Causal generalization: from sample to population, from experiment to policy. Sampling weighting, transportability.
- **Cost Model:** Generalization bounds: bias = discrepancy × treatment effect heterogeneity; O(n) to estimate discrepancy.
- **Real Wall:** Generalization requires transportability assumptions; unmeasured effect heterogeneity limits generalization.
- **Cross-Domain Aliases:** external-validity-causal (ml-training), population-generalization (agentic-reasoning).
- **Notes:** Pearl & Bareinboim (2011); generalization formulas quantify external validity bias.

### [PRIM-153] causal-inference-complex-interventions
- **Atom/Composite:** Composite
- **Definition:** Complex interventions: multi-component treatments, factorial designs. ANOVA decomposition, sufficient cause interaction.
- **Cost Model:** Factorial ANOVA: decompose total effect into main effects and interactions; O(2^k·n) for k factors.
- **Real Wall:** Interaction identification requires larger samples; sufficient cause framework for mechanistic interactions.
- **Cross-Domain Aliases:** factorial-design-causal (statistics-probability), multi-component-effect (ml-training).
- **Notes:** VanderWeele (2015); sufficient cause framework provides mechanistic interpretation of interactions.

### [PRIM-154] causal-inference-survey
- **Atom/Composite:** Composite
- **Definition:** Causal inference with survey data: complex survey designs (stratification, clustering, weighting). Design-based vs. model-based inference.
- **Cost Model:** Survey weights: inverse probability of selection; incorporate into IPW estimator O(n) weighting.
- **Real Wall:** Survey design effects inflate variance; design-based inference is robust to model misspecification.
- **Cross-Domain Aliases:** survey-causal (statistics-probability), complex-survey (ml-training).
- **Notes:** Lumley (2010); causal inference from surveys requires accounting for survey design.

### [PRIM-155] causal-inference-conjoint
- **Atom/Composite:** Composite
- **Definition:** Conjoint analysis for causal effects: factorial experiments measuring attribute importance. Average marginal component effect (AMCE).
- **Cost Model:** AMCE: regression of outcome on all attributes with interactions; O(2^a·n) for a attributes.
- **Real Wall:** Conjoint assumes no carryover effects between profiles; balance checks needed across attribute levels.
- **Cross-Domain Aliases:** conjoint-analysis-causal (statistics-probability), factorial-ame (ml-training).
- **Notes:** Hainmueller et al. (2014); conjoint experiments enable causal measurement of attribute preferences.

### [PRIM-156] causal-inference-agglomerative
- **Atom/Composite:** Composite
- **Definition:** Causal inference for network data: network autocorrelation, peer effects. Exposure mapping, spillover estimation.
- **Cost Model:** Network exposure: O(n·degree) to compute exposure mapping; spatial autocorrelation models O(n²).
- **Real Wall:** Network interference complicates causal estimates; must model interference structure explicitly.
- **Cross-Domain Aliases:** network-interference-causal (statistics-probability), social-spillover (distributed-systems).
- **Notes:** Angrist (2014); network causal inference is an active area; natural experiments in networks provide identification.

### [PRIM-157] causal-inference-time-varying
- **Atom/Composite:** Composite
- **Definition:** Time-varying treatments: exposure accumulated over time. Cumulative treatment effects, dose-response curves.
- **Cost Model:** Cumulative effect: integrate over time-specific treatment effects; g-formula O(n·T).
- **Real Wall:** Time-varying treatments have dynamic confounding; static treatment policies require g-methods.
- **Cross-Domain Aliases:** dynamic-treatment-causal (statistics-probability), cumulative-exposure (ml-training).
- **Notes:** Robins et al. (2000); g-methods are needed when treatment is administered over time.

### [PRIM-158] causal-inference-proxy-outcomes
- **Atom/Composite:** Composite
- **Definition:** Proxy outcomes: use surrogate outcomes for causal inference. Principal stratification, surrogate index.
- **Cost Model:** Principal surrogate evaluation: estimate proportion of treatment effect explained by surrogate; O(n) regression.
- **Real Wall:** Surrogate outcomes can mislead; requires strong biological or mechanistic rationale.
- **Cross-Domain Aliases:** surrogate-outcome (statistics-probability), surrogate-marker (ml-training).
- **Notes:** Gilbert & Hudgens (2008); surrogate index approach uses many weak proxies to construct strong surrogate.

### [PRIM-159] causal-inference-sensitivity-continuous
- **Atom/Composite:** Composite
- **Definition:** Continuous sensitivity analysis: Rosenbaum bounds extended to continuous outcomes, matching. Sensitivity parameter Γ.
- **Cost Model:** Rosenbaum bound: P(max(Y) ≥ max(Y') | Γ) bounds; O(n) for bounds computation.
- **Real Wall:** Sensitivity parameter interpretation is key; must translate Γ to meaningful confounding magnitude.
- **Cross-Domain Aliases:** continuous-rosenbaum (statistics-probability), bounds-sensitivity (ml-training).
- **Notes:** Rosenbaum (2002); sensitivity analysis reveals how strong unmeasured confounding must be to nullify findings.

### [PRIM-160] causal-inference-meta-analysis
- **Atom/Composite:** Composite
- **Definition:** Meta-analysis of causal studies: random effects model, heterogeneity assessment (I², Q-statistic). Publication bias corrections.
- **Cost Model:** Random effects meta-analysis: REML estimation of between-study variance O(B) for B studies.
- **Real Wall:** Meta-analysis of observational studies is more biased than RCT meta-analysis; quality assessment needed.
- **Cross-Domain Aliases:** meta-analysis-causal (statistics-probability), study-synthesis (agentic-reasoning).
- **Notes:** DerSimonian & Laird (1986); heterogeneity in causal estimates across studies reveals effect modifiers.

### [PRIM-161] causal-inference-conceptual-baseline
- **Atom/Composite:** Composite
- **Definition:** Baseline balance: after adjustment, treatment and control groups should have similar covariate distributions. Standardized mean difference.
- **Cost Model:** SMD = (mean_treat - mean_control) / pooled_sd; O(n·p) to compute; threshold < 0.1 for good balance.
- **Real Wall:** Balance on observed covariates does not guarantee balance on unobserved; overlap assumption is untestable.
- **Cross-Domain Aliases:** covariate-balance-check (statistics-probability), balance-metric (ml-training).
- **Notes:** Imbens & Rubin (2015); achieving baseline balance is necessary (but not sufficient) for valid causal inference.

### [PRIM-162] causal-inference-fuzzy-matching
- **Atom/Composite:** Composite
- **Definition:** Fuzzy matching: match treated units to controls with similar propensity scores, allowing imperfect matches. OLS on matched sample.
- **Cost Model:** Fuzzy matching O(n·log n) with KD-tree; matching with replacement O(n²) worst-case.
- **Real Wall:** Fuzzy matching allows units to contribute fractionally to multiple matches; reduces variance but may increase bias.
- **Cross-Domain Aliases:** propensity-score-fuzzy (statistics-probability), soft-matching (ml-training).
- **Notes:** Rosenbaum (2002); fuzzy matching preserves more units than exact matching; balance-precision trade-off.

### [PRIM-163] causal-inference-matching-imbalance
- **Atom/Composite:** Composite
- **Definition:** Imbalance metrics: L¹ statistic, kernel distance, energy distance between treatment and control distributions.
- **Cost Model:** L¹ = Σ_z |P(Z=z | X=1) - P(Z=z | X=0)|; O(n) computation; L¹ = 0 for perfect balance.
- **Real Wall:** L¹ of 1 means no overlap; L¹ = 0 means perfect balance; metric guides matching refinement.
- **Cross-Domain Aliases:** distribution-imbalance (statistics-probability), overlap-metric (ml-training).
- **Notes:** Iacus et al. (2012); entropy balancing achieves exact balance on specified moments.

### [PRIM-164] causal-inference-continuous-treatment
- **Atom/Composite:** Composite
- **Definition:** Continuous treatment causal inference: treatment dose is continuous. Generalized propensity score, dose-response function.
- **Cost Model:** GPS estimation: O(n·p) for logistic/cubic spline regression; dose-response O(n·grid) for interpolation.
- **Real Wall:** Continuous treatment requires stronger overlap; GPS balancing must hold at all dose levels.
- **Cross-Domain Aliases:** continuous-dose-causal (statistics-probability), dose-response (ml-training).
- **Notes:** Hirano & Imbens (2005); GPS enables dose-response estimation via covariate adjustment.

### [PRIM-165] causal-inference-hypothesis-testing
- **Atom/Composite:** Composite
- **Definition:** Causal hypothesis testing: sharp null hypothesis (no unit-level effect), Fisher's exact test, permutation inference.
- **Cost Model:** Fisher's exact test: enumerate all treatment assignments (2^n); Monte Carlo approximation O(B·n) for B permutations.
- **Real Wall:** Sharp null tests are conservative; weak null (ATE=0) is more common and more testable.
- **Cross-Domain Aliases:** causal-null-test (statistics-probability), fisher-exact-causal (ml-training).
- **Notes:** Fisher (1935); randomization inference provides exact p-values without asymptotic approximations.

### [PRIM-166] causal-inference-causal-criticism
- **Atom/Composite:** Composite
- **Definition:** Causal criticism: evaluate causal claims on internal validity, external validity, construct validity, reliability.
- **Cost Model:** Validity assessment: qualitative rubric; each dimension O(1) to evaluate but requires domain expertise.
- **Real Wall:** Criticisms may reveal assumptions researchers didn't consider; triangulation helps address criticisms.
- **Cross-Domain Aliases:** causal-quality-assessment (agentic-reasoning), validity-evaluation (statistics-probability).
- **Notes:** Shadish et al. (2002); Campbell's validity framework applied to causal inference.

### [PRIM-167] causal-inference-causal-story
- **Atom/Composite:** Primitive
- **Definition:** Causal story: narrative explanation linking causal mechanisms to observed outcomes. Theory of change, causal narrative.
- **Cost Model:** Qualitative: narrative construction O(1); quantitative: test mechanism predictions via mediation analysis.
- **Real Wall:** Causal stories guide hypothesis generation and interpretation; must be validated empirically, not just asserted.
- **Cross-Domain Aliases:** causal-narrative (agentic-reasoning), theory-of-change (statistics-probability).
- **Notes:** Weiss (1997); causal stories provide theoretical grounding for statistical causal estimates.

### [PRIM-168] causal-inference-regression-discontinuity-design
- **Atom/Composite:** Composite
- **Definition:** RDD elements: running variable R, cutoff c, treatment A = 1(R ≥ c). Sharp vs. fuzzy RDD; bandwidth selection.
- **Cost Model:** Bandwidth selection: MSE-optimal bandwidth h = c n^{-1/5}; Imbens-Kalyanaraman optimal O(n).
- **Real Wall:** Bandwidth choice is critical; different bandwidths give different estimates; falsification tests needed.
- **Cross-Domain Aliases:** rdd-elements (statistics-probability), cutoff-design (ml-training).
- **Notes:** Imbens & Lemieux (2008); RDD is credible when running variable cannot be perfectly manipulated.

### [PRIM-169] causal-inference-multilevel-design
- **Atom/Composite:** Composite
- **Definition:** Multilevel causal inference: hierarchical data (students in schools). Mixed effects models, cluster-randomized designs.
- **Cost Model:** Mixed model: ICC = σ²_cluster / (σ²_cluster + σ²_individual); design effect = 1 + (m-1)ICC.
- **Real Wall:** Ignoring clustering underestimates SEs; cluster-level treatment requires many clusters for valid inference.
- **Cross-Domain Aliases:** hierarchical-causal (statistics-probability), multilevel-design (ml-training).
- **Notes:** Raudenbush & Bryk (2002); multilevel designs require multilevel analysis for proper inference.

### [PRIM-170] causal-inference-causal-consistency
- **Atom/Composite:** Composite
- **Definition:** Causal consistency verification: observed outcome matches potential outcome under treatment received. SUTVA holds.
- **Cost Model:** Consistency check: if SUTVA violated, define multiple potential outcomes per version; O(1) conceptual.
- **Real Wall:** Consistency violation indicates interference or multiple treatment versions; must extend causal model.
- **Cross-Domain Aliases:** consistency-check (statistics-probability), potential-outcome-match (agentic-reasoning).
- **Notes:** Rubin (1990); consistency is definitional but critical; violations require causal model revision.

## 11. Bayesian Causal Methods

### [PRIM-171] causal-inference-bayesian-causal-forest
- **Atom/Composite:** Composite
- **Definition:** Bayesian Causal Forest (BCF): separately parameterize prognostic μ(x) and treatment effect τ(x) functions via BART priors with regularization toward homogeneous effects.
- **Cost Model:** Two BART ensembles fit jointly via Gibbs sampling; O(M·n·log n) per MCMC iteration with M trees; thousands of iterations.
- **Real Wall:** Strong regularization on τ avoids spurious heterogeneity; sensitive to prior on tree depth; targeted selection bias if propensity not included.
- **Cross-Domain Aliases:** bayesian-cate (statistics-probability), bcf-prior (ml-training).
- **Notes:** Hahn, Murray, Carvalho (2020); BCF outperforms vanilla BART for CATE by parameterizing the treatment effect separately.

### [PRIM-172] causal-inference-bart-for-causal
- **Atom/Composite:** Composite
- **Definition:** BART (Bayesian Additive Regression Trees) for outcome regression in causal inference: posterior over E[Y|A,X] yields posterior over ATE via g-formula.
- **Cost Model:** Sum-of-trees ensemble; backfitting MCMC; O(M·n) per iteration; default M=200 trees.
- **Real Wall:** BART can underperform on highly nonlinear treatment-covariate interactions without BCF reparameterization; uncertainty quantification depends on prior calibration.
- **Cross-Domain Aliases:** bart-causal (statistics-probability), tree-ensemble-causal (ml-training).
- **Notes:** Hill (2011); BART provides natural uncertainty quantification for causal estimands via posterior draws.

### [PRIM-173] causal-inference-posterior-treatment-effect
- **Atom/Composite:** Primitive
- **Definition:** Posterior distribution over treatment effect τ given data: p(τ|D) ∝ p(D|τ,θ)p(τ)p(θ) marginalizing nuisance parameters.
- **Cost Model:** MCMC: O(K·iterations) per parameter K; variational: O(K) per gradient step.
- **Real Wall:** Posterior credible intervals are not frequentist confidence intervals; calibration requires careful prior specification.
- **Cross-Domain Aliases:** posterior-ate (statistics-probability), bayesian-effect (ml-training).
- **Notes:** Bayesian inference enables direct probability statements about causal effects given priors.

### [PRIM-174] causal-inference-bayesian-iv
- **Atom/Composite:** Composite
- **Definition:** Bayesian instrumental variables: joint posterior over structural and reduced-form parameters with prior on identification strength.
- **Cost Model:** Gibbs or HMC sampling over (β, π, Σ) for outcome/first-stage; O(d²·iterations) for d covariates.
- **Real Wall:** Weak instruments yield diffuse posteriors; Jeffreys-style priors can dominate likelihood in weak-IV regime.
- **Cross-Domain Aliases:** bayesian-iv (statistics-probability), structural-bayesian (ml-training).
- **Notes:** Conley et al. (2012); Bayesian IV gracefully handles weak instruments via priors on partial identification.

### [PRIM-175] causal-inference-bayesian-bootstrap-causal
- **Atom/Composite:** Composite
- **Definition:** Bayesian bootstrap for causal estimands: Dirichlet weights on observations induce posterior over empirical distribution and resulting causal functionals.
- **Cost Model:** Per draw: sample w ~ Dir(1,...,1), compute weighted estimator O(n); B draws yield posterior O(Bn).
- **Real Wall:** Bayesian bootstrap equals exchangeability prior; not valid under interference; ignores model uncertainty.
- **Cross-Domain Aliases:** dirichlet-bootstrap (statistics-probability), bayes-resample (ml-training).
- **Notes:** Rubin (1981); useful for posterior on functionals like ATE without parametric outcome model.

### [PRIM-176] causal-inference-dirichlet-process-causal
- **Atom/Composite:** Composite
- **Definition:** Nonparametric Bayes via Dirichlet process mixture for outcome distribution within strata; flexible posterior over response surface.
- **Cost Model:** DP mixture MCMC: Chinese restaurant process sampling O(n·K) per iteration with K active components.
- **Real Wall:** Sensitive to base measure choice; clustering can collapse if concentration parameter mis-specified.
- **Cross-Domain Aliases:** dp-mixture-causal (statistics-probability), nonparametric-bayes (ml-training).
- **Notes:** Roy, Lum, Daniels (2017); DP mixtures provide flexible nonparametric priors for response surfaces.

### [PRIM-177] causal-inference-bayesian-model-averaging-causal
- **Atom/Composite:** Composite
- **Definition:** BMA over alternative causal adjustment sets or models: τ̂_BMA = Σ_m p(m|D)·τ̂_m weighted by posterior model probability.
- **Cost Model:** Per model m: fit cost; aggregation O(M) over M models; marginal likelihoods may need bridge sampling.
- **Real Wall:** Requires careful prior on model space; ignores models outside the candidate set; cannot recover from misspecification of all candidates.
- **Cross-Domain Aliases:** bma-causal (statistics-probability), model-averaging (ml-training).
- **Notes:** Wang, Parmigiani, Dominici (2012); BMA for confounder selection acknowledges model uncertainty in causal estimates.

### [PRIM-178] causal-inference-gaussian-process-causal
- **Atom/Composite:** Composite
- **Definition:** Gaussian process priors on outcome surfaces under treatment and control; posterior CATE via difference of GP posteriors.
- **Cost Model:** Exact GP: O(n³) inverse + O(n²) prediction; sparse approximations O(nm²) with m inducing points.
- **Real Wall:** Kernel choice encodes smoothness assumptions; treatment effect inherits kernel smoothness, possibly biasing toward homogeneity.
- **Cross-Domain Aliases:** gp-cate (statistics-probability), gaussian-process-regression-causal (ml-training).
- **Notes:** Alaa & van der Schaar (2017); multi-task GPs for CATE estimation share information across treatment arms.

## 12. Continuous Treatments

### [PRIM-179] causal-inference-generalized-propensity-score
- **Atom/Composite:** Composite
- **Definition:** GPS r(a,x) = f_{A|X}(a|x): conditional density of treatment given covariates; balancing score for continuous A.
- **Cost Model:** Estimate via parametric model (Gaussian, gamma) or kernel density; O(n) for parametric, O(n²) for kernel.
- **Real Wall:** Requires correct density model; tails of treatment distribution have sparse data and unstable weights.
- **Cross-Domain Aliases:** continuous-propensity (statistics-probability), gps (ml-training).
- **Notes:** Hirano & Imbens (2004); GPS extends propensity score logic to continuous treatments.

### [PRIM-180] causal-inference-dose-response-curve
- **Atom/Composite:** Primitive
- **Definition:** μ(a) = E[Y(a)]: expected potential outcome as function of continuous treatment dose a.
- **Cost Model:** Estimation: kernel smoothing O(n²) or local polynomial; bias O(h²), variance O(1/nh).
- **Real Wall:** Requires positivity across continuum; extrapolation near treatment boundaries is unreliable.
- **Cross-Domain Aliases:** dose-response (statistics-probability), continuous-ate (ml-training).
- **Notes:** Imbens (2000); central estimand for continuous treatment causal inference.

### [PRIM-181] causal-inference-msm-continuous-treatment
- **Atom/Composite:** Composite
- **Definition:** Marginal structural model for continuous A: E[Y(a)] = g(a;β); fit via IPW with stabilized weights w(a,x) = f(a)/f(a|x).
- **Cost Model:** Density ratio estimation O(n); weighted regression O(n·d).
- **Real Wall:** Extreme weight values when density ratio is large; requires correct density specification for both numerator and denominator.
- **Cross-Domain Aliases:** continuous-msm (statistics-probability), continuous-ipw (ml-training).
- **Notes:** Robins, Hernán, Brumback (2000); stabilized weights are essential for variance control with continuous A.

### [PRIM-182] causal-inference-covariate-balancing-gps
- **Atom/Composite:** Composite
- **Definition:** CB-GPS: estimate GPS by directly optimizing covariate balance moments (E[A·X·w] = E[A]·E[X]) rather than density fit.
- **Cost Model:** Empirical likelihood or method-of-moments optimization O(d·n) per iteration.
- **Real Wall:** Balance on observed moments does not imply balance on unobserved transformations; nonlinear functions of X may remain imbalanced.
- **Cross-Domain Aliases:** cbps-continuous (statistics-probability), balance-weighting (ml-training).
- **Notes:** Fong, Hazlett, Imai (2018); direct balancing avoids modeling treatment density.

### [PRIM-183] causal-inference-entropy-balancing-continuous
- **Atom/Composite:** Composite
- **Definition:** Entropy balancing weights for continuous treatment: minimize KL divergence subject to moment constraints on (A,X) covariance.
- **Cost Model:** Convex dual: O(d) parameters; Newton iterations O(d²·n).
- **Real Wall:** Convergence requires feasible balance constraints; high-dimensional moments may not have feasible weights.
- **Cross-Domain Aliases:** ebal-continuous (statistics-probability), max-entropy-weights (ml-training).
- **Notes:** Tübbicke (2022); extends Hainmueller's entropy balancing to continuous treatments.

### [PRIM-184] causal-inference-incremental-effect
- **Atom/Composite:** Primitive
- **Definition:** Incremental propensity score intervention: shift propensity by odds ratio δ rather than setting treatment value; defines smooth alternative estimand.
- **Cost Model:** Estimation via efficient influence function; O(n) for cross-fitted nuisances.
- **Real Wall:** Departs from standard hypothetical interventions; interpretation is policy-relative shift, not setting A to fixed value.
- **Cross-Domain Aliases:** ipsi (statistics-probability), incremental-intervention (ml-training).
- **Notes:** Kennedy (2019); avoids positivity violations because no treatment value is fixed.

### [PRIM-185] causal-inference-modified-treatment-policies
- **Atom/Composite:** Composite
- **Definition:** MTP: stochastic intervention defined as A → d(A,X) (e.g., A+1 if feasible); estimand E[Y(d)] = E[Y under modified policy].
- **Cost Model:** TMLE or one-step estimator with O(n) per cross-fit fold; nuisances Q̄ and density ratio.
- **Real Wall:** Requires positivity for the modified policy distribution, not the original; choice of modification is policy-dependent.
- **Cross-Domain Aliases:** mtp (statistics-probability), stochastic-intervention (ml-training).
- **Notes:** Díaz, van der Laan (2012); useful when atomic interventions are infeasible.

## 13. Mediation Extensions

### [PRIM-186] causal-inference-interventional-direct-effect
- **Atom/Composite:** Primitive
- **Definition:** Interventional direct effect: E[Y(a, G_{M|a*})] - E[Y(a*, G_{M|a*})] where G is a random draw from M's distribution under a*.
- **Cost Model:** Estimation via Monte Carlo over mediator draws + outcome regression O(B·n).
- **Real Wall:** Does not require cross-world independence; weaker assumptions than NDE/NIE but different interpretation.
- **Cross-Domain Aliases:** ide (statistics-probability), interventional-mediation (ml-training).
- **Notes:** VanderWeele, Vansteelandt, Robins (2014); identifiable without cross-world assumptions.

### [PRIM-187] causal-inference-interventional-indirect-effect
- **Atom/Composite:** Primitive
- **Definition:** Interventional indirect effect: E[Y(a, G_{M|a})] - E[Y(a, G_{M|a*})] using stochastic draws of mediator distributions under different treatments.
- **Cost Model:** Same as IDE: Monte Carlo over mediator distributions; O(B·n).
- **Real Wall:** Sum of IDE + IIE does not equal total effect in general (unlike NDE+NIE under standard assumptions).
- **Cross-Domain Aliases:** iie (statistics-probability), interventional-indirect (ml-training).
- **Notes:** VanderWeele (2015); identifiable with treatment-induced confounder L of M-Y relationship.

### [PRIM-188] causal-inference-organic-direct-effect
- **Atom/Composite:** Primitive
- **Definition:** Organic direct effect (Lok 2016): random draw from natural distribution of M without intervention on covariates; defines path-specific effects under unverifiable assumptions.
- **Cost Model:** Posterior over mediator draws under natural mechanism; O(B·n).
- **Real Wall:** Definition tied to "natural" stochastic process; identification requires structural assumptions about mediator generation.
- **Cross-Domain Aliases:** organic-mediation (statistics-probability), natural-stochastic-mediator (ml-training).
- **Notes:** Lok (2016); alternative to interventional effects with different conceptual basis.

### [PRIM-189] causal-inference-randomized-analog-nde
- **Atom/Composite:** Composite
- **Definition:** Randomized analog of NDE: identifiable substitute for natural direct effect using randomly drawn mediator value from M|A=a* distribution.
- **Cost Model:** IPW or g-computation estimator O(n); random mediator draws O(B).
- **Real Wall:** Coincides with NDE only when mediator has no treatment-induced confounders; otherwise differs.
- **Cross-Domain Aliases:** rde (statistics-probability), randomized-direct (ml-training).
- **Notes:** Vansteelandt, Daniel (2017); avoids cross-world counterfactual independence.

### [PRIM-190] causal-inference-randomized-analog-nie
- **Atom/Composite:** Composite
- **Definition:** Randomized analog of NIE: substitute estimand using random mediator draw, identifiable under sequential ignorability without cross-world assumptions.
- **Cost Model:** Two-stage Monte Carlo; O(B·n) per estimand.
- **Real Wall:** Decomposition of TE into RDE+RNIE differs from NDE+NIE when treatment-induced confounders exist.
- **Cross-Domain Aliases:** rnie (statistics-probability), randomized-indirect (ml-training).
- **Notes:** Vansteelandt, Daniel (2017); coupled with RDE for full TE decomposition.

### [PRIM-191] causal-inference-path-specific-effect-multi-mediator
- **Atom/Composite:** Composite
- **Definition:** Path-specific effects with multiple mediators M₁, M₂: identify effect along specific causal paths via repeated counterfactual contrasts.
- **Cost Model:** Identification: path-formula via mediation chain; estimation: nested g-computation O(n·K) for K mediators.
- **Real Wall:** Non-identifiable in general due to recanting witness; requires structural restrictions on mediator ordering.
- **Cross-Domain Aliases:** pse-multi (statistics-probability), multi-mediator-pse (ml-training).
- **Notes:** Avin, Shpitser, Pearl (2005); recanting witness criterion characterizes non-identifiability.

## 14. Sensitivity Analysis Extensions

### [PRIM-192] causal-inference-omitted-variable-bias-bounds
- **Atom/Composite:** Composite
- **Definition:** OVB bounds (Cinelli-Hazlett): bound bias from omitted confounder using R² strength of unobserved on treatment and outcome.
- **Cost Model:** Closed-form bias formula given R²_{A~U|X} and R²_{Y~U|X,A}; computation O(1) given regression outputs.
- **Real Wall:** Requires reasoning about plausible R² for unobserved confounder; benchmarking against observed covariates helps.
- **Cross-Domain Aliases:** ovb (statistics-probability), cinelli-hazlett (ml-training).
- **Notes:** Cinelli & Hazlett (2020); robustness value and partial R² quantify minimal confounding strength to nullify result.

### [PRIM-193] causal-inference-robustness-value
- **Atom/Composite:** Primitive
- **Definition:** Robustness value RV_q: minimum strength of association between unobserved confounder and (A,Y) needed to reduce estimate by factor (1-q).
- **Cost Model:** Closed-form from regression t-statistic and degrees of freedom; O(1).
- **Real Wall:** Single scalar summary; reality may have many confounders each with small effect that aggregate.
- **Cross-Domain Aliases:** rv (statistics-probability), causal-robustness (ml-training).
- **Notes:** Cinelli, Hazlett (2020); single-number sensitivity summary for OLS estimands.

### [PRIM-194] causal-inference-benchmarking-sensitivity
- **Atom/Composite:** Composite
- **Definition:** Benchmark sensitivity parameters against observed covariates: "an unobserved confounder k× as strong as covariate X would bias estimate by Δ".
- **Cost Model:** Repeated leave-one-out covariate analysis O(d) for d benchmark variables.
- **Real Wall:** Assumes unobserved confounder is similar in nature to observed; may understate strength of qualitatively different confounders.
- **Cross-Domain Aliases:** sensitivity-benchmark (statistics-probability), confounder-comparison (ml-training).
- **Notes:** Imbens (2003); benchmarking grounds sensitivity in plausible scales.

### [PRIM-195] causal-inference-manski-bounds
- **Atom/Composite:** Composite
- **Definition:** Manski bounds: no-assumption bounds on ATE using only Y range; tighten with monotone treatment response or selection assumptions.
- **Cost Model:** Compute empirical bounds: O(n) for ATE bounds = [E[Y|A=1,K=1]p+y_min(1-p), E[Y|A=1,K=1]p+y_max(1-p)] - similar for control.
- **Real Wall:** No-assumption bounds often too wide to be useful; tightening requires assumptions like MTR or MTS.
- **Cross-Domain Aliases:** partial-identification-bounds (statistics-probability), manski (logic-reasoning).
- **Notes:** Manski (1990); foundational partial identification approach without untestable assumptions.

### [PRIM-196] causal-inference-monotone-treatment-response
- **Atom/Composite:** Primitive
- **Definition:** MTR assumption: Y(1) ≥ Y(0) (or reverse) for all units; tightens Manski bounds substantially.
- **Cost Model:** Apply MTR-constrained bounds: O(n) per stratum.
- **Real Wall:** MTR is strong substantive assumption; violated if treatment harms some subjects.
- **Cross-Domain Aliases:** mtr (statistics-probability), monotone-response (ml-training).
- **Notes:** Manski, Pepper (2000); MTR is testable on observed quantiles but cannot rule out individual reversals.

### [PRIM-197] causal-inference-monotone-treatment-selection
- **Atom/Composite:** Primitive
- **Definition:** MTS assumption: those with A=1 have weakly larger Y(a) than those with A=0 (selection into treatment is positive).
- **Cost Model:** Apply MTS bound: ATE ≤ observed difference (under MTS+MTR).
- **Real Wall:** MTS bounds observed difference; assumes self-selection is informative about outcomes.
- **Cross-Domain Aliases:** mts (statistics-probability), positive-selection (ml-training).
- **Notes:** Manski, Pepper (2000); combined with MTR yields useful one-sided bounds.

### [PRIM-198] causal-inference-tipping-point-analysis
- **Atom/Composite:** Composite
- **Definition:** Tipping-point sensitivity: search for unmeasured confounder parameters (Γ, prevalence) that "tip" significance or sign of estimate.
- **Cost Model:** Grid search over (Γ, π) space; O(K²) evaluations.
- **Real Wall:** Two-dimensional sensitivity surface complex to summarize; tipping point may require implausibly strong confounding.
- **Cross-Domain Aliases:** tipping-sensitivity (statistics-probability), sensitivity-grid (ml-training).
- **Notes:** Rosenbaum, Silber (2009); informative for whether plausible confounding overturns conclusions.

## 15. Causal Reinforcement Learning & Off-Policy Evaluation

### [PRIM-199] causal-inference-off-policy-evaluation
- **Atom/Composite:** Composite
- **Definition:** OPE: estimate value V(π_e) of evaluation policy π_e using data from behavior policy π_b without on-policy rollouts.
- **Cost Model:** IS estimator: Σ ρ_t r_t with ρ = Π π_e/π_b; variance grows exponentially in horizon.
- **Real Wall:** Exponential variance in trajectory length (curse of horizon); requires support overlap π_b > 0 where π_e > 0.
- **Cross-Domain Aliases:** ope (ml-training), counterfactual-policy-value (statistics-probability).
- **Notes:** Precup, Sutton, Singh (2000); foundational for offline RL evaluation.

### [PRIM-200] causal-inference-doubly-robust-ope
- **Atom/Composite:** Composite
- **Definition:** DR-OPE: V̂_DR = Σ_t γ^t [ρ_t (r_t - Q̂(s,a)) + V̂(s)] combining IS and model-based estimates.
- **Cost Model:** O(T·n) per trajectory with fitted Q-function; cross-fit O(K·T·n).
- **Real Wall:** Consistent if either Q̂ or behavior policy correct; doubly robust in trajectory not per-step.
- **Cross-Domain Aliases:** dr-ope (ml-training), doubly-robust-rl (statistics-probability).
- **Notes:** Jiang, Li (2016); Thomas, Brunskill (2016); reduces variance vs. pure IS.

### [PRIM-201] causal-inference-magic-estimator
- **Atom/Composite:** Composite
- **Definition:** MAGIC: model and guided importance sampling combination; weighted combination of n-step DR estimators minimizing MSE.
- **Cost Model:** Convex optimization over weights of length-n returns; O(T²) per trajectory.
- **Real Wall:** Weight optimization requires estimating bias-variance per horizon; biased if model is wrong.
- **Cross-Domain Aliases:** magic-ope (ml-training), weighted-dr (statistics-probability).
- **Notes:** Thomas, Brunskill (2016); empirically state-of-the-art OPE for finite horizon.

### [PRIM-202] causal-inference-weighted-importance-sampling
- **Atom/Composite:** Composite
- **Definition:** WIS: normalize importance weights by sum across trajectories; biased but consistent and lower variance than ordinary IS.
- **Cost Model:** O(N·T) per evaluation across N trajectories.
- **Real Wall:** Bias O(1/N); finite-sample bias can be substantial; not unbiased even asymptotically without correction.
- **Cross-Domain Aliases:** wis (ml-training), self-normalized-is (statistics-probability).
- **Notes:** Precup et al. (2000); standard variance reduction at cost of bias.

### [PRIM-203] causal-inference-fitted-q-evaluation
- **Atom/Composite:** Composite
- **Definition:** FQE: fit Q̂_π via Bellman iteration on offline data; V̂(π) = E_{s~d_0}[Q̂_π(s,π(s))].
- **Cost Model:** Iterative regression: K iterations of O(n·d²) for d-dim Q-function.
- **Real Wall:** Model-based; biased if function class misspecified; requires distributional overlap for stable Q-iteration.
- **Cross-Domain Aliases:** fqe (ml-training), direct-method-ope (statistics-probability).
- **Notes:** Le, Voloshin, Yue (2019); model-based OPE complementary to IS methods.

### [PRIM-204] causal-inference-marginalized-importance-sampling
- **Atom/Composite:** Composite
- **Definition:** Marginalized IS / state distribution correction: estimate ratio d_πe(s)/d_πb(s) directly, avoiding per-step product.
- **Cost Model:** Density ratio estimation via DICE methods O(n·d).
- **Real Wall:** Breaks curse of horizon; but density ratio estimation in high-dim state spaces is hard.
- **Cross-Domain Aliases:** mis (ml-training), state-ratio (statistics-probability).
- **Notes:** Liu et al. (2018); breaks variance scaling with horizon.

### [PRIM-205] causal-inference-v-trace
- **Atom/Composite:** Composite
- **Definition:** V-trace: clipped importance sampling for off-policy actor-critic; truncates ρ̄ = min(ρ̄, ρ) to bound variance with controlled bias.
- **Cost Model:** O(T) per trajectory with truncation thresholds.
- **Real Wall:** Bias from truncation; designed for IMPALA-style distributed RL not pure OPE.
- **Cross-Domain Aliases:** v-trace (ml-training), clipped-is (statistics-probability).
- **Notes:** Espeholt et al. (2018); IMPALA's off-policy correction.

### [PRIM-206] causal-inference-retrace
- **Atom/Composite:** Composite
- **Definition:** Retrace(λ): low-variance off-policy return estimator with λ-mixing of corrected n-step returns; safe and efficient.
- **Cost Model:** O(T) recursive computation along trajectory.
- **Real Wall:** Convergence guarantees under arbitrary behavior policy; tradeoff via λ between bias and variance.
- **Cross-Domain Aliases:** retrace-lambda (ml-training), safe-off-policy (statistics-probability).
- **Notes:** Munos et al. (2016); guarantees convergence under arbitrary behavior policies.

### [PRIM-207] causal-inference-behavior-cloning-confounded
- **Atom/Composite:** Composite
- **Definition:** Behavior cloning under hidden confounders: imitation of demonstrator policy conditional on observed state may not recover demonstrator's true latent-conditional policy.
- **Cost Model:** Supervised learning O(n·d) for policy class; bias persists if confounder unobserved.
- **Real Wall:** Confounded BC suffers compounding bias; cannot extrapolate to deployment distributions where confounder shifts.
- **Cross-Domain Aliases:** confounded-imitation (ml-training), bc-bias (statistics-probability).
- **Notes:** de Haan, Jayaraman, Levine (2019); "causal confusion in imitation learning".

### [PRIM-208] causal-inference-causal-bandits
- **Atom/Composite:** Composite
- **Definition:** Causal bandits: leverage known causal graph to improve regret in contextual or structural bandit setting via do-interventions on parent nodes.
- **Cost Model:** Regret O(√(nK_eff)) where K_eff < K via graph structure.
- **Real Wall:** Requires known causal structure; mis-specified graph can worsen regret vs. structure-blind.
- **Cross-Domain Aliases:** causal-mab (ml-training), structural-bandit (statistics-probability).
- **Notes:** Lattimore, Lattimore, Reid (2016); leveraging causal knowledge accelerates exploration.

## 16. Survival & Continuous-Time Causal Inference

### [PRIM-209] causal-inference-counting-process-msm
- **Atom/Composite:** Composite
- **Definition:** Counting-process MSM: time-varying treatment hazard ratio λ(t|A,X) = λ_0(t)exp(βA(t)+γX); IPW weights for time-varying confounding.
- **Cost Model:** Cox regression with time-varying weights; partial likelihood O(n²) per event.
- **Real Wall:** Censoring weights for informative censoring; positivity at each event time.
- **Cross-Domain Aliases:** cox-msm (statistics-probability), survival-msm (ml-training).
- **Notes:** Robins, Hernán, Brumback (2000); MSM for survival outcomes.

### [PRIM-210] causal-inference-causal-cox-model
- **Atom/Composite:** Composite
- **Definition:** Causal interpretation of Cox PH model: hazard ratio is not collapsible; conditional ≠ marginal HR even with no confounding.
- **Cost Model:** Standard Cox O(n²); marginalization via g-computation O(B·n).
- **Real Wall:** Non-collapsibility creates interpretive challenges; PH assumption needs verification.
- **Cross-Domain Aliases:** cox-causal (statistics-probability), hazard-ratio (ml-training).
- **Notes:** Hernán (2010); "hazards of hazard ratios" cautions about causal interpretation.

### [PRIM-211] causal-inference-ipcw
- **Atom/Composite:** Composite
- **Definition:** Inverse probability of censoring weights: weight uncensored subjects by 1/P(C>t|history) to correct for informative censoring.
- **Cost Model:** Censoring model estimation O(n·T); weighted analysis O(n).
- **Real Wall:** Requires correct censoring model; extreme weights from censoring near end of follow-up.
- **Cross-Domain Aliases:** ipcw (statistics-probability), censoring-weights (ml-training).
- **Notes:** Robins, Finkelstein (2000); core technique for survival analysis under dropout.

### [PRIM-212] causal-inference-structural-failure-time-model
- **Atom/Composite:** Composite
- **Definition:** SFTM: T(0) = ∫_0^T exp(ψA(u))du; relates observed T to counterfactual under no treatment via accelerated failure-time structure.
- **Cost Model:** G-estimation: search over ψ for which counterfactual T(0) ⊥ A|X; O(K·n) for K candidate values.
- **Real Wall:** Restrictive functional form; sensitive to specification of acceleration factor.
- **Cross-Domain Aliases:** sftm (statistics-probability), accelerated-causal (ml-training).
- **Notes:** Robins (1992); time-varying treatment effect on survival via AFT model.

### [PRIM-213] causal-inference-competing-risks-causal
- **Atom/Composite:** Composite
- **Definition:** Causal effects in competing risks: cause-specific cumulative incidence F_k(t|do(A=a)); requires care with non-cause-specific censoring.
- **Cost Model:** Fine-Gray or cause-specific Cox O(n²); g-formula across competing events O(K·n).
- **Real Wall:** Cause-specific hazard ≠ subdistribution hazard; interpretation differs; conditional on competing event status.
- **Cross-Domain Aliases:** competing-risks (statistics-probability), multi-state-causal (ml-training).
- **Notes:** Young et al. (2020); separable effects for competing risks.

### [PRIM-214] causal-inference-restricted-mean-survival
- **Atom/Composite:** Primitive
- **Definition:** RMST(τ) = E[min(T,τ)] = ∫_0^τ S(t)dt: causal contrast in restricted mean survival; collapsible unlike HR.
- **Cost Model:** Kaplan-Meier integration O(n log n) per arm; difference is causal contrast.
- **Real Wall:** Choice of truncation τ affects estimand; less power than log-rank under PH.
- **Cross-Domain Aliases:** rmst (statistics-probability), restricted-mean (ml-training).
- **Notes:** Royston, Parmar (2013); collapsibility makes RMST attractive causal estimand.

## 17. Spatial & Network Causal Inference

### [PRIM-215] causal-inference-spatial-confounding
- **Atom/Composite:** Composite
- **Definition:** Spatial confounding: unobserved spatially-structured covariate creates bias when exposure has spatial pattern; spatial random effects partially absorb treatment.
- **Cost Model:** Spatial+ models, geo-additive regression O(n²) for kriging; sparse approximations O(n log n).
- **Real Wall:** Including spatial smooth can amplify or reduce bias depending on confounding structure.
- **Cross-Domain Aliases:** spatial-confounding (statistics-probability), geo-causal (ml-training).
- **Notes:** Reich et al. (2006); Paciorek (2010); active research area without consensus solution.

### [PRIM-216] causal-inference-geographic-rdd
- **Atom/Composite:** Composite
- **Definition:** Geographic RDD: boundary between treated and untreated regions is "running variable"; identification at boundary under continuity of potential outcomes.
- **Cost Model:** Bandwidth selection in geographic distance; O(n) local linear regression near boundary.
- **Real Wall:** Boundary may correlate with other geographic discontinuities; requires assumption of smooth potential outcomes across boundary.
- **Cross-Domain Aliases:** geo-rdd (statistics-probability), boundary-discontinuity (ml-training).
- **Notes:** Keele, Titiunik (2015); useful for evaluating policies along administrative borders.

### [PRIM-217] causal-inference-spatial-spillover
- **Atom/Composite:** Composite
- **Definition:** Spatial spillover: outcome of unit i depends on treatment at nearby units j via weight matrix W; total effect = direct + indirect spatial.
- **Cost Model:** Spatial autoregressive model O(n³) for direct, O(n) for sparse W.
- **Real Wall:** Identification requires W known a priori; misspecified neighborhood biases all effects.
- **Cross-Domain Aliases:** spatial-spillover (statistics-probability), spatial-interference (ml-training).
- **Notes:** LeSage, Pace (2009); spatial econometrics meets causal inference.

### [PRIM-218] causal-inference-exposure-mapping
- **Atom/Composite:** Composite
- **Definition:** Exposure mapping f(A,i): summarize neighbor treatments into low-dimensional exposure type for unit i; reduces SUTVA violation.
- **Cost Model:** Compute exposure per unit O(d_i) for degree d_i; analysis at exposure-type level.
- **Real Wall:** Misspecified exposure mapping ignores relevant interference dimensions; sensitivity analysis needed.
- **Cross-Domain Aliases:** exposure-summary (statistics-probability), interference-pooling (ml-training).
- **Notes:** Aronow, Samii (2017); pragmatic dimensionality reduction for interference.

### [PRIM-219] causal-inference-peer-effects-identification
- **Atom/Composite:** Composite
- **Definition:** Peer effects (Manski reflection problem): endogenous, exogenous, and correlated effects are not separately identified in linear-in-means model without exclusion restrictions.
- **Cost Model:** 2SLS with network-based instruments O(n²·d).
- **Real Wall:** Linear-in-means is fundamentally under-identified; nonlinear models or instruments needed.
- **Cross-Domain Aliases:** reflection-problem (statistics-probability), peer-effects (ml-training).
- **Notes:** Manski (1993); foundational impossibility result for peer effects.

### [PRIM-220] causal-inference-randomized-saturation
- **Atom/Composite:** Composite
- **Definition:** Randomized saturation: vary treatment fraction across clusters to identify direct and spillover effects; saturation level π_c randomized.
- **Cost Model:** Two-stage randomization design; analysis O(n·C) for C clusters.
- **Real Wall:** Power depends on cluster count and saturation range; spillovers may be nonlinear in saturation.
- **Cross-Domain Aliases:** saturation-design (statistics-probability), two-stage-randomization (ml-training).
- **Notes:** Baird, Bohren, McIntosh, Özler (2018); identifies both ITT and spillover effects.

### [PRIM-221] causal-inference-design-based-interference
- **Atom/Composite:** Composite
- **Definition:** Design-based inference under interference: Horvitz-Thompson estimators for exposure effects using known randomization probabilities, no outcome model.
- **Cost Model:** HT estimator O(n) given exposure probabilities; complex variance with interference.
- **Real Wall:** Variance estimation under arbitrary interference is hard; conservative bounds often necessary.
- **Cross-Domain Aliases:** design-based-spillover (statistics-probability), horvitz-thompson-interference (ml-training).
- **Notes:** Aronow, Samii (2017); randomization-based inference avoids parametric assumptions.

### [PRIM-222] causal-inference-cluster-randomized-interference
- **Atom/Composite:** Composite
- **Definition:** Cluster-randomized design exploits within-cluster interference: randomize at cluster level, all interference contained within clusters (partial interference).
- **Cost Model:** Cluster-level analysis O(C) for C clusters; design effect from intraclass correlation.
- **Real Wall:** Between-cluster interference (spillovers across clusters) violates assumption; requires sufficient cluster separation.
- **Cross-Domain Aliases:** partial-interference (statistics-probability), cluster-randomization (ml-training).
- **Notes:** Hudgens, Halloran (2008); formalized partial-interference framework.

## 18. Advanced Identification

### [PRIM-223] causal-inference-napkin-problem
- **Atom/Composite:** Composite
- **Definition:** Napkin problem: identifying P(Y|do(X)) when no instrument or back-door, but conditional independence Y ⊥ X | W via intermediate W via specific graph structure.
- **Cost Model:** ID algorithm checks graph structure O(|V|²); estimation via two regressions.
- **Real Wall:** Identification depends on subtle graph structure; small misspecification breaks identification.
- **Cross-Domain Aliases:** napkin-id (logic-reasoning), pearl-napkin (statistics-probability).
- **Notes:** Pearl (2018); illustrates non-back-door identification beyond front-door.

### [PRIM-224] causal-inference-id-star-algorithm
- **Atom/Composite:** Composite
- **Definition:** ID* algorithm: extends ID to counterfactual queries with multiple worlds; checks identifiability of joint counterfactuals.
- **Cost Model:** Recursive c-component decomposition over twin/multi-network O(|V|^k) for k worlds.
- **Real Wall:** Counterfactual queries are often non-identifiable from observational data alone.
- **Cross-Domain Aliases:** id-star (logic-reasoning), counterfactual-id (statistics-probability).
- **Notes:** Shpitser, Pearl (2008); complete algorithm for counterfactual identification.

### [PRIM-225] causal-inference-recursive-c-components
- **Atom/Composite:** Primitive
- **Definition:** C-component (confounded component): maximal set connected by bidirected edges in ADMG; identification decomposes along c-components.
- **Cost Model:** Graph decomposition O(|V|+|E|) via union-find on bidirected edges.
- **Real Wall:** c-component structure dictates identifiability; large c-components often non-identifiable.
- **Cross-Domain Aliases:** c-component (logic-reasoning), district (statistics-probability).
- **Notes:** Tian, Pearl (2002); structural basis for ID algorithm.

### [PRIM-226] causal-inference-ancestral-identification
- **Atom/Composite:** Composite
- **Definition:** Ancestral identification: restrict to ancestors of query nodes; reduces graph and may simplify identification.
- **Cost Model:** Ancestor search O(|V|+|E|); subgraph identification on reduced graph.
- **Real Wall:** Only necessary step toward ID; full identification may still fail on ancestral subgraph.
- **Cross-Domain Aliases:** ancestor-projection (logic-reasoning), graph-restriction (statistics-probability).
- **Notes:** Used as preprocessing step in ID algorithm.

### [PRIM-227] causal-inference-conditional-identification
- **Atom/Composite:** Composite
- **Definition:** Conditional ID: identify P(Y|do(X), Z=z) where conditioning on Z occurs after intervention; uses Z-specific c-component analysis.
- **Cost Model:** Modified ID algorithm O(|V|²) with conditional subgraph.
- **Real Wall:** Conditioning sets cannot include descendants of intervened nodes without special handling.
- **Cross-Domain Aliases:** conditional-id (logic-reasoning), stratified-id (statistics-probability).
- **Notes:** Shpitser, Pearl (2006); extension to conditional interventional queries.

### [PRIM-228] causal-inference-mediation-identification
- **Atom/Composite:** Composite
- **Definition:** Identification of natural direct/indirect effects requires no treatment-induced confounder of M-Y path; otherwise non-identifiable.
- **Cost Model:** Graph check O(|V|+|E|) for treatment-induced confounders.
- **Real Wall:** Recanting witness L (descendant of A, ancestor of M and Y) blocks NDE/NIE identification.
- **Cross-Domain Aliases:** nde-id (logic-reasoning), mediation-id (statistics-probability).
- **Notes:** Pearl (2001); Avin, Shpitser, Pearl (2005); structural conditions for mediation ID.

### [PRIM-229] causal-inference-transportability-id
- **Atom/Composite:** Composite
- **Definition:** Transportability: identify causal effect in target population from experiments in source population + observational data in target via selection diagrams.
- **Cost Model:** TR algorithm extends ID with selection nodes; O(|V|²).
- **Real Wall:** Requires correct selection diagram identifying which variables differ across populations.
- **Cross-Domain Aliases:** transportability (logic-reasoning), generalizability-id (statistics-probability).
- **Notes:** Bareinboim, Pearl (2013); generalization across populations.

## 19. Causal Discovery Extensions

### [PRIM-230] causal-inference-ica-lingam
- **Atom/Composite:** Composite
- **Definition:** ICA-LiNGAM: linear non-Gaussian acyclic model identified via independent component analysis on residuals; recovers causal ordering.
- **Cost Model:** ICA O(n·d²); ordering search O(d!) brute, O(d²) greedy variants.
- **Real Wall:** Requires non-Gaussianity of errors; sensitive to outliers.
- **Cross-Domain Aliases:** lingam (statistics-probability), ica-causal (ml-training).
- **Notes:** Shimizu et al. (2006); LiNGAM gives full causal ordering when assumptions hold.

### [PRIM-231] causal-inference-direct-lingam
- **Atom/Composite:** Composite
- **Definition:** DirectLiNGAM: greedy algorithm finding sink/source by regressing each variable on others and testing residual independence.
- **Cost Model:** O(d³·n) per ordering iteration; total O(d⁴·n) for full ordering.
- **Real Wall:** Greedy nature may miss optimal ordering; sensitive to independence test choice.
- **Cross-Domain Aliases:** direct-lingam (statistics-probability), greedy-lingam (ml-training).
- **Notes:** Shimizu et al. (2011); avoids ICA's local-minima issues.

### [PRIM-232] causal-inference-resit
- **Atom/Composite:** Composite
- **Definition:** RESIT (Regression with Subsequent Independence Test): for nonlinear ANM, regress effect on cause, test residual ⊥ cause; if independent, ANM holds.
- **Cost Model:** Nonparametric regression O(n²) + HSIC independence test O(n²).
- **Real Wall:** Computationally expensive; power of independence test critical.
- **Cross-Domain Aliases:** resit (statistics-probability), anm-test (ml-training).
- **Notes:** Peters et al. (2014); discovers nonlinear ANM structure.

### [PRIM-233] causal-inference-additive-noise-model
- **Atom/Composite:** Primitive
- **Definition:** ANM: Y = f(X) + ε with ε ⊥ X; asymmetry allows identification of cause-effect from observational bivariate data.
- **Cost Model:** Nonparametric regression + independence test O(n²).
- **Real Wall:** Identifiability requires f nonlinear and ε non-Gaussian; bivariate Gaussian case unidentifiable.
- **Cross-Domain Aliases:** anm (statistics-probability), additive-noise (ml-training).
- **Notes:** Hoyer et al. (2009); functional asymmetry yields causal direction.

### [PRIM-234] causal-inference-post-nonlinear-model
- **Atom/Composite:** Composite
- **Definition:** PNL: Y = g(f(X) + ε) with invertible g; generalization of ANM allowing post-transformation.
- **Cost Model:** Two-stage estimation: invert g, then ANM test; O(n²).
- **Real Wall:** Identification fails in 5 specific cases (Gaussian linear, etc.); requires invertibility of g.
- **Cross-Domain Aliases:** pnl (statistics-probability), post-nonlinear (ml-training).
- **Notes:** Zhang, Hyvärinen (2009); broadest identifiable functional model.

### [PRIM-235] causal-inference-notears
- **Atom/Composite:** Composite
- **Definition:** NOTEARS: continuous optimization for DAG structure via smooth acyclicity constraint h(W) = tr(e^(W◦W)) - d = 0.
- **Cost Model:** Gradient-based optimization O(d²·n) per step; total iterations depend on dual ascent.
- **Real Wall:** Score function (least squares) is variance-sensitive; recent critiques show varsortability bias.
- **Cross-Domain Aliases:** notears (ml-training), continuous-dag-learning (statistics-probability).
- **Notes:** Zheng et al. (2018); converts combinatorial DAG search to continuous optimization.

### [PRIM-236] causal-inference-dagma
- **Atom/Composite:** Composite
- **Definition:** DAGMA: smooth acyclicity via log-det h(W) = -log det(sI - W◦W); more efficient gradients than NOTEARS exponential.
- **Cost Model:** Per iteration O(d³) for log-det; faster convergence than NOTEARS.
- **Real Wall:** Still subject to varsortability and score-scale issues; nonconvex landscape.
- **Cross-Domain Aliases:** dagma (ml-training), log-det-dag (statistics-probability).
- **Notes:** Bello, Aragam, Ravikumar (2022); improved continuous DAG learning.

### [PRIM-237] causal-inference-golem
- **Atom/Composite:** Composite
- **Definition:** GOLEM: NOTEARS variant with likelihood-based score and L1 regularization; addresses var-sortability via standardization.
- **Cost Model:** O(d²·n) per gradient step; competitive with NOTEARS.
- **Real Wall:** Still affected by sample variance heterogeneity across nodes.
- **Cross-Domain Aliases:** golem (ml-training), likelihood-dag (statistics-probability).
- **Notes:** Ng, Ghassami, Zhang (2020); likelihood improves over LS score.

### [PRIM-238] causal-inference-mmhc
- **Atom/Composite:** Composite
- **Definition:** MMHC (Max-Min Hill Climbing): hybrid constraint+score discovery; constraint-based skeleton from MMPC + greedy hill-climbing orientation.
- **Cost Model:** MMPC O(d²·n) + HC O(d³) per iteration.
- **Real Wall:** Hybrid methods inherit weaknesses of both; constraint phase determines search space for score phase.
- **Cross-Domain Aliases:** mmhc (ml-training), hybrid-discovery (statistics-probability).
- **Notes:** Tsamardinos, Brown, Aliferis (2006); influential hybrid algorithm.

### [PRIM-239] causal-inference-causal-discovery-interventional
- **Atom/Composite:** Composite
- **Definition:** Discovery from interventional data: known interventions narrow Markov equivalence class to interventional equivalence class, eventually identifying DAG.
- **Cost Model:** GIES (Greedy Interventional Equivalence Search) O(d³) per iteration.
- **Real Wall:** Number of interventions to fully identify DAG can be log d in best case, much more in adversarial structures.
- **Cross-Domain Aliases:** gies (statistics-probability), interventional-discovery (ml-training).
- **Notes:** Hauser, Bühlmann (2012); interventions break equivalence class symmetries.

## 20. Causal Machine Learning

### [PRIM-240] causal-inference-r-learner
- **Atom/Composite:** Composite
- **Definition:** R-learner: two-stage CATE estimation; first stage estimates m(x)=E[Y|X], e(x)=P(A|X); second stage regresses (Y-m)/(A-e) on X with weights (A-e)².
- **Cost Model:** Two nuisance fits + final regression: O(n·d) per stage with cross-fitting.
- **Real Wall:** Sensitive to nuisance quality but Neyman-orthogonal; second stage requires regularization for stability.
- **Cross-Domain Aliases:** r-learner (ml-training), residualized-cate (statistics-probability).
- **Notes:** Nie, Wager (2021); orthogonalized learning for heterogeneous effects.

### [PRIM-241] causal-inference-dr-learner
- **Atom/Composite:** Composite
- **Definition:** DR-learner: regresses doubly-robust pseudo-outcome ψ(O) = μ₁(X) - μ₀(X) + (A-e)/[e(1-e)](Y-μ_A) on X to estimate CATE.
- **Cost Model:** Three nuisance models (μ₀, μ₁, e) + final regression; cross-fitting for orthogonality.
- **Real Wall:** Highest-order errors from nuisance products; finite-sample variance can be large.
- **Cross-Domain Aliases:** dr-learner (ml-training), dr-cate (statistics-probability).
- **Notes:** Kennedy (2020); flexible CATE estimation with double robustness.

### [PRIM-242] causal-inference-x-learner
- **Atom/Composite:** Composite
- **Definition:** X-learner: uses information from larger arm to inform CATE in smaller arm via imputation of counterfactuals plus weighting.
- **Cost Model:** Three regressions + weighted combination O(n·d).
- **Real Wall:** Excels when treatment arms have very different sizes; weight choice (propensity vs constant) matters.
- **Cross-Domain Aliases:** x-learner (ml-training), meta-learner-cate (statistics-probability).
- **Notes:** Künzel et al. (2019); meta-learner especially good for imbalanced trials.

### [PRIM-243] causal-inference-t-learner
- **Atom/Composite:** Composite
- **Definition:** T-learner: separate outcome models for each treatment arm μ_a(X); CATE = μ_1(X) - μ_0(X).
- **Cost Model:** Two regressions O(n·d) each.
- **Real Wall:** Doesn't share information across arms; high variance with small arms; biased by separate regularization.
- **Cross-Domain Aliases:** t-learner (ml-training), two-model-cate (statistics-probability).
- **Notes:** Simple baseline meta-learner; outperformed by R, DR, X for most settings.

### [PRIM-244] causal-inference-s-learner
- **Atom/Composite:** Composite
- **Definition:** S-learner: single outcome model μ(X,A) with treatment as feature; CATE = μ(X,1)-μ(X,0).
- **Cost Model:** Single regression O(n·d).
- **Real Wall:** Treatment indicator may be regularized away; biased toward zero CATE.
- **Cross-Domain Aliases:** s-learner (ml-training), single-model-cate (statistics-probability).
- **Notes:** Künzel et al. (2019); naive but common baseline.

### [PRIM-245] causal-inference-tarnet
- **Atom/Composite:** Composite
- **Definition:** TARNet: shared representation encoder + two outcome heads (treated/control); avoids covariate shift between arms via shared representation.
- **Cost Model:** Neural network O(n·H·E) for H hidden units, E epochs.
- **Real Wall:** Representation may collapse treatment-relevant features; balance penalty often added.
- **Cross-Domain Aliases:** tarnet (ml-training), shared-rep-cate (statistics-probability).
- **Notes:** Shalit, Johansson, Sontag (2017); foundation for neural CATE methods.

### [PRIM-246] causal-inference-cfrnet
- **Atom/Composite:** Composite
- **Definition:** CFR (Counterfactual Regression): TARNet + integral probability metric (IPM) penalty on representation between treated/control distributions.
- **Cost Model:** Add Wasserstein or MMD term O(n²) per batch.
- **Real Wall:** IPM regularization strength is hyperparameter; can over-balance and lose predictive information.
- **Cross-Domain Aliases:** cfr (ml-training), wasserstein-cate (statistics-probability).
- **Notes:** Shalit, Johansson, Sontag (2017); balance via distribution matching.

### [PRIM-247] causal-inference-dragonnet
- **Atom/Composite:** Composite
- **Definition:** Dragonnet: TARNet extended with propensity head g(X); end-to-end learning with targeted regularization for ATE.
- **Cost Model:** Three heads + targeted reg; O(n·H·E).
- **Real Wall:** Targeted regularization improves ATE not CATE; joint training may degrade individual heads.
- **Cross-Domain Aliases:** dragonnet (ml-training), e2e-causal (statistics-probability).
- **Notes:** Shi, Blei, Veitch (2019); incorporates propensity for efficient ATE.

### [PRIM-248] causal-inference-ganite
- **Atom/Composite:** Composite
- **Definition:** GANITE: adversarial network generates counterfactual outcomes; outputs CATE estimates with uncertainty.
- **Cost Model:** GAN training O(n·E·H); typically unstable.
- **Real Wall:** GAN training instability; counterfactual fidelity hard to validate.
- **Cross-Domain Aliases:** ganite (ml-training), gan-causal (statistics-probability).
- **Notes:** Yoon, Jordon, van der Schaar (2018); generative approach to CATE.

### [PRIM-249] causal-inference-orthogonal-random-forest
- **Atom/Composite:** Composite
- **Definition:** Orthogonal Random Forest: forest weights × orthogonalized estimating equation; valid CIs under high-dim nuisances.
- **Cost Model:** Forest fitting O(B·n log n) + orthogonal moment per leaf O(d).
- **Real Wall:** Computationally heavier than vanilla causal forest; requires Neyman-orthogonal moment.
- **Cross-Domain Aliases:** orf (ml-training), orthogonal-forest (statistics-probability).
- **Notes:** Oprescu, Syrgkanis, Wu (2019); flexible CATE with confidence intervals.

## 21. Robust & Semiparametric Causal Inference

### [PRIM-250] causal-inference-neyman-orthogonality
- **Atom/Composite:** Primitive
- **Definition:** Neyman orthogonality: estimating equation ψ(O;θ,η) satisfies ∂E[ψ]/∂η|_{η_0} = 0; first-order insensitive to nuisance error.
- **Cost Model:** Construct via influence function: O(1) once derived; estimation O(n).
- **Real Wall:** Required for ML-based nuisances; without orthogonality, plug-in bias dominates.
- **Cross-Domain Aliases:** neyman-orthogonal (statistics-probability), orthogonal-moment (ml-training).
- **Notes:** Chernozhukov et al. (2018); cornerstone of double/debiased ML.

### [PRIM-251] causal-inference-double-debiased-ml
- **Atom/Composite:** Composite
- **Definition:** DML: cross-fitted Neyman-orthogonal estimating equation; achieves √n-consistency with arbitrary ML nuisance estimators.
- **Cost Model:** K-fold cross-fitting: K nuisance fits + aggregation O(K·n·complexity).
- **Real Wall:** Requires nuisance convergence rate n^(-1/4); sample splitting halves effective sample.
- **Cross-Domain Aliases:** dml (ml-training), debiased-ml (statistics-probability).
- **Notes:** Chernozhukov et al. (2018); enables black-box ML for causal inference.

### [PRIM-252] causal-inference-cross-fitting
- **Atom/Composite:** Composite
- **Definition:** Cross-fitting: train nuisance on K-1 folds, evaluate on held-out fold; prevents own-observation bias from overfit nuisances.
- **Cost Model:** K parallel fits; O(K·n·d) total.
- **Real Wall:** Variance reduction by averaging across fold splits; computational overhead vs single fit.
- **Cross-Domain Aliases:** cross-fitting (ml-training), sample-splitting (statistics-probability).
- **Notes:** Schick (1986); Chernozhukov et al. (2018); central trick of DML.

### [PRIM-253] causal-inference-sample-splitting
- **Atom/Composite:** Composite
- **Definition:** Sample splitting: divide data into independent parts for nuisance estimation and target estimation; ensures independence between nuisance and inference samples.
- **Cost Model:** Halves data; O(n/2) per phase.
- **Real Wall:** Loss of efficiency; cross-fitting recovers this via averaging.
- **Cross-Domain Aliases:** sample-split (statistics-probability), train-test-causal (ml-training).
- **Notes:** Bickel (1982); foundational technique made practical by cross-fitting.

### [PRIM-254] causal-inference-bias-corrected-estimator
- **Atom/Composite:** Composite
- **Definition:** One-step bias correction: θ̂ + (1/n)Σ ψ(O_i; θ̂, η̂) where ψ is the influence function; debiases plug-in.
- **Cost Model:** Compute influence function values O(n); add to plug-in.
- **Real Wall:** Requires consistent influence function estimate; second-order remainder must vanish.
- **Cross-Domain Aliases:** one-step (statistics-probability), bias-correction (ml-training).
- **Notes:** Pfanzagl (1982); precursor to modern semiparametric theory.

### [PRIM-255] causal-inference-targeted-mle-step
- **Atom/Composite:** Composite
- **Definition:** TMLE targeting step: solve efficient influence equation by updating initial fit via parametric submodel; iterates until score equation = 0.
- **Cost Model:** Logistic regression update O(n) per iteration; few iterations to convergence.
- **Real Wall:** Choice of submodel affects finite-sample performance; converges differently than one-step.
- **Cross-Domain Aliases:** tmle-update (statistics-probability), targeted-fluctuation (ml-training).
- **Notes:** van der Laan, Rubin (2006); TMLE characteristic update step.

### [PRIM-256] causal-inference-efficient-influence-function
- **Atom/Composite:** Primitive
- **Definition:** EIF: derivative of estimand along scores in nonparametric model; characterizes semiparametric efficiency bound.
- **Cost Model:** Derivation O(1) per estimand; evaluation O(n).
- **Real Wall:** Derivation requires functional analysis; specific to estimand and statistical model.
- **Cross-Domain Aliases:** eif (statistics-probability), influence-function (ml-training).
- **Notes:** Hampel (1974); van der Vaart (1998); core object in semiparametric inference.

### [PRIM-257] causal-inference-semiparametric-efficiency-bound
- **Atom/Composite:** Primitive
- **Definition:** Lower bound on asymptotic variance among regular estimators in semiparametric model: variance of efficient influence function.
- **Cost Model:** Computed from EIF: Var(ψ_eff); O(n) sample estimate.
- **Real Wall:** Bound may not be achievable under high-dim or sparse settings; assumes regular asymptotics.
- **Cross-Domain Aliases:** semiparametric-bound (statistics-probability), efficiency-limit (control-numerical-opt).
- **Notes:** Bickel et al. (1993); analog of Cramér-Rao for semiparametric models.

## 22. Balancing & Weighting Methods

### [PRIM-258] causal-inference-cbps
- **Atom/Composite:** Composite
- **Definition:** Covariate Balancing Propensity Score: estimate propensity by directly optimizing covariate balance via GMM, jointly with likelihood.
- **Cost Model:** GMM with d moment conditions; O(d²·n) per iteration.
- **Real Wall:** Trade-off between balance and propensity fit; over-identified GMM can be unstable.
- **Cross-Domain Aliases:** cbps (statistics-probability), balance-propensity (ml-training).
- **Notes:** Imai, Ratkovic (2014); propensity score that directly targets balance.

### [PRIM-259] causal-inference-stable-balancing-weights
- **Atom/Composite:** Composite
- **Definition:** SBW: minimize variance of weights subject to balance constraints up to tolerance δ; quadratic program with linear constraints.
- **Cost Model:** QP solver O(n²) for dense or O(n·d) for sparse constraints.
- **Real Wall:** Tolerance δ governs bias-variance tradeoff; tight δ may infeasible.
- **Cross-Domain Aliases:** sbw (statistics-probability), constrained-weights (control-numerical-opt).
- **Notes:** Zubizarreta (2015); minimum-variance weights satisfying explicit balance.

### [PRIM-260] causal-inference-entropy-balancing
- **Atom/Composite:** Composite
- **Definition:** Entropy balancing: weights w_i minimizing Σw_i log w_i s.t. moment constraints E_w[X] = E[X|A=1]; convex dual problem.
- **Cost Model:** Dual: d parameters via Newton; O(d² + d·n) per iteration.
- **Real Wall:** Balance only on chosen moments; can't extrapolate; requires feasibility of constraints.
- **Cross-Domain Aliases:** ebal (statistics-probability), maxent-balance (information-theory-coding).
- **Notes:** Hainmueller (2012); efficient exact balance for low-dim moments.

### [PRIM-261] causal-inference-empirical-likelihood-causal
- **Atom/Composite:** Composite
- **Definition:** Empirical likelihood ATE: maximize Π w_i subject to weight balance constraints; nonparametric inference with χ² ratio.
- **Cost Model:** Dual formulation O(d²·n) per Newton step.
- **Real Wall:** EL can have convex-hull issues with sparse data; alternative versions (ETEL) help.
- **Cross-Domain Aliases:** el-causal (statistics-probability), nonparametric-likelihood (ml-training).
- **Notes:** Owen (2001); nonparametric inference framework adaptable to causal.

### [PRIM-262] causal-inference-optimal-transport-weighting
- **Atom/Composite:** Composite
- **Definition:** Optimal transport-based weighting: map treated distribution to control via OT plan; weights = marginals of transport plan.
- **Cost Model:** Sinkhorn OT O(n²) per iteration; Frank-Wolfe O(n² log n).
- **Real Wall:** Cost matrix specification critical; OT plan may not equal IPW under correct model.
- **Cross-Domain Aliases:** ot-weighting (statistics-probability), wasserstein-balance (ml-training).
- **Notes:** Connection to entropy balancing through entropy-regularized OT.

## 23. Off-Policy & Bandit Causal Inference

### [PRIM-263] causal-inference-doubly-robust-policy-learning
- **Atom/Composite:** Composite
- **Definition:** Doubly-robust policy learning: optimize policy via empirical risk over DR-pseudo outcomes; consistent if μ̂ or ê correct.
- **Cost Model:** Per policy class size; O(n·|Π|) for tabular, gradient O(n·d) for parametric.
- **Real Wall:** Policy class capacity must match identifiability; regret bounds require concentration of DR estimator.
- **Cross-Domain Aliases:** dr-policy (ml-training), robust-policy (statistics-probability).
- **Notes:** Dudík, Langford, Li (2011); Athey, Wager (2021).

### [PRIM-264] causal-inference-switch-estimator
- **Atom/Composite:** Composite
- **Definition:** Switch estimator: combines IS estimator with truncation threshold; uses model-based estimate when IS weights exceed threshold τ.
- **Cost Model:** O(T) per trajectory.
- **Real Wall:** Threshold τ trades bias (model error) vs variance (IS variance); optimal τ depends on horizon.
- **Cross-Domain Aliases:** switch-estimator (ml-training), threshold-is (statistics-probability).
- **Notes:** Wang, Agarwal, Dudík (2017); per-step adaptive method selection.

### [PRIM-265] causal-inference-self-normalized-is
- **Atom/Composite:** Composite
- **Definition:** Self-normalized IS: V̂ = Σ ρ_i r_i / Σ ρ_i; biased but variance reduced; matches WIS in single-trajectory case.
- **Cost Model:** O(n) per estimator.
- **Real Wall:** Bias O(1/n); cannot directly use for unbiased CI without correction.
- **Cross-Domain Aliases:** snips (ml-training), self-norm-is (statistics-probability).
- **Notes:** Swaminathan, Joachims (2015); SNIS for contextual bandits.

### [PRIM-266] causal-inference-contextual-bandit-evaluation
- **Atom/Composite:** Composite
- **Definition:** Contextual bandit OPE: estimate V(π) for new policy from logged (x,a,r,p) tuples; IPS, DR, slate estimators.
- **Cost Model:** O(n) per IPS or DR estimator.
- **Real Wall:** Support overlap π(a|x) requires p(a|x) > 0 wherever π > 0; high-action-space bandits suffer.
- **Cross-Domain Aliases:** cb-ope (ml-training), bandit-counterfactual (statistics-probability).
- **Notes:** Dudík et al. (2011); core technique for personalization.

### [PRIM-267] causal-inference-slate-recommendation-evaluation
- **Atom/Composite:** Composite
- **Definition:** Slate OPE: counterfactual evaluation of ordered lists; pseudoinverse estimator decomposes slate value across positions.
- **Cost Model:** O(L²·n) for slate length L; pseudo-inverse O(L³).
- **Real Wall:** Position interactions hard to capture; assumes linearity across positions.
- **Cross-Domain Aliases:** slate-ope (ml-training), list-counterfactual (statistics-probability).
- **Notes:** Swaminathan et al. (2017); slate-aware off-policy evaluation.

### [PRIM-268] causal-inference-causal-thompson-sampling
- **Atom/Composite:** Composite
- **Definition:** Causal Thompson sampling: use posterior over causal effects (not just rewards) to balance exploration with confounding-aware uncertainty.
- **Cost Model:** Posterior sampling O(d) per round; cumulative regret bounds depend on graph structure.
- **Real Wall:** Confounding in observational arm requires identification; may add bias if structure unknown.
- **Cross-Domain Aliases:** causal-ts (ml-training), confounded-ts (statistics-probability).
- **Notes:** Bareinboim, Forney, Pearl (2015); exploits causal structure for sample efficiency.

## 24. Time Series Causal Inference

### [PRIM-269] causal-inference-granger-causality
- **Atom/Composite:** Primitive
- **Definition:** Granger causality: X Granger-causes Y if past X helps predict Y beyond past Y; not true causation absent additional assumptions.
- **Cost Model:** F-test on VAR coefficients O(p²·T) for lag p.
- **Real Wall:** Confused with causation in popular use; doesn't handle confounders or contemporaneous causation.
- **Cross-Domain Aliases:** granger (statistics-probability), predictive-causality (ml-training).
- **Notes:** Granger (1969); predictive not causal; standard in econometrics.

### [PRIM-270] causal-inference-transfer-entropy
- **Atom/Composite:** Primitive
- **Definition:** Transfer entropy T_{X→Y} = I(Y_{t+1}; X_t^k | Y_t^l): conditional mutual information measuring directed information flow.
- **Cost Model:** Density estimation in (X^k, Y^l) space; O(n) parametric, O(n^(k+l)) nonparametric.
- **Real Wall:** Curse of dimensionality with embedding lags; biased small-sample estimates.
- **Cross-Domain Aliases:** transfer-entropy (information-theory-coding), schreiber-te (statistics-probability).
- **Notes:** Schreiber (2000); model-free generalization of Granger.

### [PRIM-271] causal-inference-liang-information-flow
- **Atom/Composite:** Composite
- **Definition:** Liang information flow: derive analytic expression for information flow rate dT/dt from dynamical systems; closed-form for linear stochastic systems.
- **Cost Model:** O(d²) for d-variable system once dynamics estimated.
- **Real Wall:** Requires dynamics model; sensitive to parameter estimation noise.
- **Cross-Domain Aliases:** liang (statistics-probability), causal-flow (information-theory-coding).
- **Notes:** Liang (2014); rigorous foundation for information flow.

### [PRIM-272] causal-inference-convergent-cross-mapping
- **Atom/Composite:** Composite
- **Definition:** CCM: dynamical systems causality test using shadow manifolds in delay embedding; X causes Y if reconstruction of X from Y improves with library length.
- **Cost Model:** Nearest-neighbor reconstruction O(n² log n).
- **Real Wall:** Designed for deterministic nonlinear systems; spurious results with strong coupling or shared driver.
- **Cross-Domain Aliases:** ccm (statistics-probability), sugihara-ccm (ml-training).
- **Notes:** Sugihara et al. (2012); takens-embedding-based causality.

### [PRIM-273] causal-inference-pcmci
- **Atom/Composite:** Composite
- **Definition:** PCMCI: time-series causal discovery via PC-style skeleton on time-lagged variables + momentary conditional independence test.
- **Cost Model:** O(T·d²·max_lag) for conditional independence testing.
- **Real Wall:** Conditional independence in high-dim time series is hard; lag selection affects results.
- **Cross-Domain Aliases:** pcmci (statistics-probability), tigramite (ml-training).
- **Notes:** Runge et al. (2019); state-of-art time-series causal discovery.

### [PRIM-274] causal-inference-dynamic-causal-model
- **Atom/Composite:** Composite
- **Definition:** DCM: state-space ODE models with parameter inference via variational Bayes; causal interpretation via intervention on coupling parameters.
- **Cost Model:** Bayesian inversion O(d³) per integration step; computationally heavy.
- **Real Wall:** Strong parametric assumptions; results sensitive to network structure prior.
- **Cross-Domain Aliases:** dcm (statistics-probability), variational-causal (ml-training).
- **Notes:** Friston et al. (2003); used in neuroimaging.

## 25. Difference-in-Differences Extensions

### [PRIM-275] causal-inference-staggered-adoption-did
- **Atom/Composite:** Composite
- **Definition:** Staggered DiD: units adopt treatment at different times; two-way fixed effects estimator biased by negative weighting of already-treated as controls.
- **Cost Model:** TWFE OLS O(n·T) but biased; alternative estimators O(n·T²) for proper cohort-time decomposition.
- **Real Wall:** Goodman-Bacon decomposition shows when TWFE biased; cohort-time aggregation preferred.
- **Cross-Domain Aliases:** staggered-did (statistics-probability), event-staggered (ml-training).
- **Notes:** Goodman-Bacon (2021); identified TWFE pathology.

### [PRIM-276] causal-inference-event-study
- **Atom/Composite:** Composite
- **Definition:** Event study: dynamic DiD with leads and lags relative to treatment timing; tests parallel trends and effect dynamics.
- **Cost Model:** OLS with event-time dummies O(n·T·E) for E event-time bins.
- **Real Wall:** Multicollinearity at extreme leads/lags; pre-trends test is necessary but not sufficient.
- **Cross-Domain Aliases:** event-study (statistics-probability), dynamic-did (ml-training).
- **Notes:** Borusyak, Jaravel (2017); standard tool with known TWFE issues.

### [PRIM-277] causal-inference-twfe-bias
- **Atom/Composite:** Primitive
- **Definition:** Two-way fixed effects bias: in staggered designs, TWFE coefficient is weighted sum of cohort-time ATTs with potentially negative weights.
- **Cost Model:** Decomposition O(C²) for C cohorts.
- **Real Wall:** Even with parallel trends, TWFE can have "wrong sign" weights; avoid in staggered designs.
- **Cross-Domain Aliases:** twfe-issue (statistics-probability), goodman-bacon (ml-training).
- **Notes:** de Chaisemartin, D'Haultfœuille (2020); Goodman-Bacon (2021).

### [PRIM-278] causal-inference-sun-abraham
- **Atom/Composite:** Composite
- **Definition:** Sun-Abraham estimator: weighted average of cohort-specific event-study coefficients; robust to staggered treatment timing.
- **Cost Model:** Cohort-specific regressions O(C·n·T); weighted aggregation.
- **Real Wall:** Requires sufficient observations per cohort; loses power with many small cohorts.
- **Cross-Domain Aliases:** sun-abraham (statistics-probability), cohort-event-study (ml-training).
- **Notes:** Sun, Abraham (2021); corrects event-study estimation under heterogeneity.

### [PRIM-279] causal-inference-callaway-santanna
- **Atom/Composite:** Composite
- **Definition:** Callaway-Sant'Anna estimator: ATT(g,t) for cohort g at time t; aggregate flexibly across cohorts and times.
- **Cost Model:** Per-(g,t) DR estimator O(n_g); aggregation O(C·T).
- **Real Wall:** Pre-treatment baseline choice (never-treated vs not-yet-treated) affects identification.
- **Cross-Domain Aliases:** cs-did (statistics-probability), did-cohort (ml-training).
- **Notes:** Callaway, Sant'Anna (2021); modern DiD with heterogeneous effects.

### [PRIM-280] causal-inference-dechaisemartin-dhaultfoeuille
- **Atom/Composite:** Composite
- **Definition:** dCDH estimator: weighted DID estimator across switching cells; robust to heterogeneity in panel data with multiple treatment changes.
- **Cost Model:** Per cell ATE O(n_cell); aggregation O(cells).
- **Real Wall:** Requires sharp design (one treatment change per unit) for cleanest interpretation.
- **Cross-Domain Aliases:** did-multiple (statistics-probability), dcdh (ml-training).
- **Notes:** de Chaisemartin, D'Haultfœuille (2020); multi-period DiD without TWFE bias.

### [PRIM-281] causal-inference-synthetic-did
- **Atom/Composite:** Composite
- **Definition:** Synthetic DID: combines synthetic control unit weights with DiD time weights; double-weighted estimator with improved properties.
- **Cost Model:** Two QP optimizations O(n² + T²); inference via jackknife or placebo.
- **Real Wall:** Time-weighting partially relaxes parallel trends; computational cost of double optimization.
- **Cross-Domain Aliases:** sdid (statistics-probability), synth-did (ml-training).
- **Notes:** Arkhangelsky, Athey, Hirshberg, Imbens, Wager (2021).

### [PRIM-282] causal-inference-borusyak-jaravel-spiess
- **Atom/Composite:** Composite
- **Definition:** Imputation estimator: fit untreated potential outcome to never-treated/pre-treatment data; impute counterfactuals; average treatment effects.
- **Cost Model:** Single regression on untreated cells O(n·T·d) + imputation O(n·T).
- **Real Wall:** Requires sufficient never-treated or pre-treatment data; misspecification affects all imputations.
- **Cross-Domain Aliases:** bjs-imputation (statistics-probability), did-imputation (ml-training).
- **Notes:** Borusyak, Jaravel, Spiess (2024); efficient estimator under parallel trends.

## 26. Specialized Designs

### [PRIM-283] causal-inference-regression-kink-design
- **Atom/Composite:** Composite
- **Definition:** RKD: identification at point where treatment is kinked function of running variable; estimand is derivative of outcome at kink.
- **Cost Model:** Local polynomial with kink O(n·h) bandwidth.
- **Real Wall:** Requires sharp kink in policy; weaker than discontinuity; sensitive to functional form near kink.
- **Cross-Domain Aliases:** rkd (statistics-probability), kink-design (ml-training).
- **Notes:** Card, Lee, Pei, Weber (2015); generalizes RDD to slope changes.

### [PRIM-284] causal-inference-bunching-estimator
- **Atom/Composite:** Composite
- **Definition:** Bunching: estimate elasticity from excess mass at kink in budget constraint; uses missing mass on one side and pile-up on the other.
- **Cost Model:** Density estimation around kink O(n) + integration of excess mass.
- **Real Wall:** Counterfactual density estimation is hard; assumptions about smooth counterfactual distribution.
- **Cross-Domain Aliases:** bunching (statistics-probability), excess-mass (ml-training).
- **Notes:** Saez (2010); Chetty et al. (2011); structural elasticity from bunching.

### [PRIM-285] causal-inference-marginal-treatment-effect
- **Atom/Composite:** Primitive
- **Definition:** MTE(u,x) = E[Y(1)-Y(0)|X=x, U_D=u]: treatment effect for unit at quantile u of latent resistance to treatment.
- **Cost Model:** Local IV: derivative of E[Y|P=p,X=x] w.r.t. p; O(n) for local linear.
- **Real Wall:** Requires continuous instrument support across [0,1]; identification at quantiles with low instrument density unstable.
- **Cross-Domain Aliases:** mte (statistics-probability), marginal-effect (ml-training).
- **Notes:** Heckman, Vytlacil (2005); generalizes LATE across all quantiles.

### [PRIM-286] causal-inference-policy-relevant-treatment-effect
- **Atom/Composite:** Composite
- **Definition:** PRTE: weighted average of MTE corresponding to specific policy counterfactual; estimand for counterfactual policy expansion.
- **Cost Model:** Integration over MTE × policy weight function O(n).
- **Real Wall:** Policy-specific weights require careful policy modeling; identified only over MTE support.
- **Cross-Domain Aliases:** prte (statistics-probability), policy-effect (ml-training).
- **Notes:** Heckman, Vytlacil (2001); policy-targeted treatment effect.

### [PRIM-287] causal-inference-mendelian-randomization
- **Atom/Composite:** Composite
- **Definition:** Mendelian randomization: use genetic variants as instruments for exposures based on Mendel's law (random allocation at conception).
- **Cost Model:** 2SLS O(n·G) for G genetic variants; weighted methods O(G).
- **Real Wall:** Pleiotropy (variant affects outcome through paths other than exposure) violates IV assumption.
- **Cross-Domain Aliases:** mr (statistics-probability), genetic-iv (ml-training).
- **Notes:** Davey Smith, Ebrahim (2003); growing use in epidemiology.

### [PRIM-288] causal-inference-mr-egger
- **Atom/Composite:** Composite
- **Definition:** MR-Egger: weighted regression of variant-outcome on variant-exposure with intercept; detects directional pleiotropy.
- **Cost Model:** Weighted regression O(G).
- **Real Wall:** Less powerful than IVW; requires InSIDE assumption.
- **Cross-Domain Aliases:** mr-egger (statistics-probability), pleiotropy-test (ml-training).
- **Notes:** Bowden, Davey Smith, Burgess (2015); robust MR under pleiotropy.

### [PRIM-289] causal-inference-surrogate-index
- **Atom/Composite:** Composite
- **Definition:** Surrogate index (Athey-Chetty-Imbens): combine short-term outcomes into index predictive of long-term outcome; impute long-term outcomes.
- **Cost Model:** ML model from short→long outcomes; O(n_old·d) for old data, O(n_new·d) for new.
- **Real Wall:** Surrogate must capture full treatment effect on long-term outcome (surrogacy condition).
- **Cross-Domain Aliases:** surrogate-index (statistics-probability), proxy-outcome (ml-training).
- **Notes:** Athey, Chetty, Imbens, Kang (2019); combines experiments + observational for long-term effects.

### [PRIM-290] causal-inference-prentice-criterion
- **Atom/Composite:** Primitive
- **Definition:** Prentice criterion: surrogate S is valid replacement for outcome Y if treatment effect on S captures all treatment effect on Y; tested via independence of A and Y given S.
- **Cost Model:** Conditional independence test O(n).
- **Real Wall:** Strong; rarely holds in practice; weaker surrogacy conditions used in practice.
- **Cross-Domain Aliases:** prentice (statistics-probability), surrogacy-test (ml-training).
- **Notes:** Prentice (1989); foundational definition of valid surrogate.

## 27. Counterfactual Representation Learning

### [PRIM-291] causal-inference-causal-vae
- **Atom/Composite:** Composite
- **Definition:** Causal VAE: VAE with latent variables corresponding to causal factors; structural prior enforces causal graph in latent space.
- **Cost Model:** ELBO optimization O(n·d·E) with E epochs.
- **Real Wall:** Without supervision or interventions, factor identifiability fails (Locatello et al.).
- **Cross-Domain Aliases:** causal-vae (ml-training), structural-latent (statistics-probability).
- **Notes:** Yang et al. (2021); links to disentanglement literature.

### [PRIM-292] causal-inference-identifiable-vae
- **Atom/Composite:** Composite
- **Definition:** iVAE: identifiable VAE using auxiliary variable (label/time/domain) as conditioning; provably identifies latent factors up to permutation+scaling.
- **Cost Model:** Conditional ELBO O(n·d·E).
- **Real Wall:** Requires sufficient diversity in auxiliary variable; identifiability is up to permutation.
- **Cross-Domain Aliases:** ivae (ml-training), identifiable-latent (statistics-probability).
- **Notes:** Khemakhem et al. (2020); identifies disentanglement using exponential-family conditional priors.

### [PRIM-293] causal-inference-anchor-variable
- **Atom/Composite:** Primitive
- **Definition:** Anchor variable: observed variable A used to identify or stabilize latent structure; anchor regression uses A to robustify against shifts.
- **Cost Model:** Anchor regression: penalize prediction-residual correlation with A; O(n·d).
- **Real Wall:** Requires A actually causes shifts in variables of interest; misuse if A is post-treatment.
- **Cross-Domain Aliases:** anchor (statistics-probability), anchor-regression (ml-training).
- **Notes:** Rothenhäusler, Meinshausen, Bühlmann, Peters (2021).

### [PRIM-294] causal-inference-invariant-causal-prediction
- **Atom/Composite:** Composite
- **Definition:** ICP: find feature set S such that P(Y|X_S, E=e) is invariant across environments e; causal predictors are subset of S.
- **Cost Model:** Test all subsets O(2^d) or greedy O(d²·E).
- **Real Wall:** Strong assumption of invariant conditional; may return empty set if no invariant subset exists.
- **Cross-Domain Aliases:** icp (statistics-probability), invariant-prediction (ml-training).
- **Notes:** Peters, Bühlmann, Meinshausen (2016); links invariance to causality.

### [PRIM-295] causal-inference-invariant-risk-minimization
- **Atom/Composite:** Composite
- **Definition:** IRM: learn representation Φ(X) and classifier w such that w is simultaneously optimal across environments; penalty on gradient of risk w.r.t. w within each env.
- **Cost Model:** Per-env gradient + penalty term O(n·d·E·|envs|).
- **Real Wall:** Optimization landscape difficult; can collapse to ERM if penalty too small or fail if too large.
- **Cross-Domain Aliases:** irm (ml-training), invariant-risk (statistics-probability).
- **Notes:** Arjovsky, Bottou, Gulrajani, Lopez-Paz (2019); causally-motivated domain generalization.

### [PRIM-296] causal-inference-domain-invariant-representation
- **Atom/Composite:** Composite
- **Definition:** Domain-invariant representation: Φ(X) such that Φ(X)|E=e has same distribution across environments; supports OOD generalization under invariant mechanism assumption.
- **Cost Model:** Adversarial domain classifier O(n·d·E).
- **Real Wall:** Distributional invariance ≠ causal invariance; can lose discriminative info.
- **Cross-Domain Aliases:** dann (ml-training), domain-invariant (statistics-probability).
- **Notes:** Ganin, Lempitsky (2015); domain-adversarial training.

## 28. Causal Fairness

### [PRIM-297] causal-inference-counterfactual-fairness
- **Atom/Composite:** Composite
- **Definition:** Counterfactual fairness: prediction Ŷ is counterfactually fair if Ŷ(A=a) = Ŷ(A=a') for all sensitive attribute values a,a' in counterfactuals.
- **Cost Model:** Counterfactual evaluation via causal model O(n·d).
- **Real Wall:** Requires causal model of how sensitive attribute affects features; sensitive to model specification.
- **Cross-Domain Aliases:** cf-fairness (statistics-probability), counterfactual-equity (logic-reasoning).
- **Notes:** Kusner, Loftus, Russell, Silva (2017); stronger than statistical parity.

### [PRIM-298] causal-inference-path-specific-fairness
- **Atom/Composite:** Composite
- **Definition:** Path-specific fairness: distinguish admissible from inadmissible causal paths from sensitive attribute to outcome; require zero effect along inadmissible paths.
- **Cost Model:** Path-specific effect estimation O(n·|paths|).
- **Real Wall:** Categorization of paths is normative; requires consensus on which paths are "fair".
- **Cross-Domain Aliases:** path-fairness (statistics-probability), nabbi-fairness (logic-reasoning).
- **Notes:** Nabi, Shpitser (2018); operationalizes fairness as path-specific causal effect.

### [PRIM-299] causal-inference-equalized-odds-counterfactual
- **Atom/Composite:** Composite
- **Definition:** Counterfactual equalized odds: P(Ŷ=1|Y(a)=y, A=a) = P(Ŷ=1|Y(a')=y, A=a') for all y, a, a'.
- **Cost Model:** Conditional probability comparison across counterfactual worlds.
- **Real Wall:** Counterfactual estimands require causal model; observable equalized odds is weaker proxy.
- **Cross-Domain Aliases:** counterfactual-eqodds (statistics-probability), causal-fairness-constraint (logic-reasoning).
- **Notes:** Coston et al. (2020); links observable fairness to counterfactual notions.

### [PRIM-300] causal-inference-fair-causal-data-augmentation
- **Atom/Composite:** Composite
- **Definition:** Fairness via counterfactual data augmentation: generate counterfactual samples flipping sensitive attribute via causal model; train on augmented data.
- **Cost Model:** Counterfactual generation O(n) + retrain O(n·d·E).
- **Real Wall:** Augmented samples are model-based; biased augmentation can degrade fairness.
- **Cross-Domain Aliases:** cf-augmentation (ml-training), causal-data-aug (statistics-probability).
- **Notes:** Garg et al. (2019); generates counterfactual training pairs.

## 29. High-Dimensional & Modern Causal Methods

### [PRIM-301] causal-inference-text-as-treatment
- **Atom/Composite:** Composite
- **Definition:** Text-as-treatment: treatment is text feature; topic models or LLM embeddings define low-dim treatment representation; estimate effects on outcome.
- **Cost Model:** Text embedding O(n·L·d) + downstream causal O(n·d).
- **Real Wall:** Embedding may carry unintended structure; treatment definition is researcher-degree-of-freedom.
- **Cross-Domain Aliases:** text-treatment (ml-training), topic-effect (statistics-probability).
- **Notes:** Egami, Fong, Grimmer, Roberts, Stewart (2018); causal inference with text.

### [PRIM-302] causal-inference-image-as-treatment
- **Atom/Composite:** Composite
- **Definition:** Image-as-treatment: treatment is visual content; CNN features as low-dim treatment; identification challenges from confounded image content.
- **Cost Model:** Feature extraction O(n·image_size); downstream causal O(n·d).
- **Real Wall:** Visual features highly entangled; isolation of causal vs correlational visual elements is difficult.
- **Cross-Domain Aliases:** image-treatment (ml-training), vision-causal (statistics-probability).
- **Notes:** Jerzak et al. (2023); image-based treatments in social sciences.

### [PRIM-303] causal-inference-embedding-treatment
- **Atom/Composite:** Composite
- **Definition:** Embedding-as-treatment: continuous or structured treatment in embedding space; estimate dose-response over embedding manifold.
- **Cost Model:** Embedding propensity (density) + outcome model; O(n·d²).
- **Real Wall:** Positivity in embedding space is hard to verify; smoothness assumption on dose-response.
- **Cross-Domain Aliases:** embedding-causal (ml-training), continuous-vector-treatment (statistics-probability).
- **Notes:** Growing area: text/image embeddings as continuous treatments.

### [PRIM-304] causal-inference-llm-as-causal-engine
- **Atom/Composite:** Composite
- **Definition:** LLM as causal reasoner: use language model to propose causal graphs, suggest adjustment sets, or interpret causal estimates given domain context.
- **Cost Model:** LLM inference per query O(L) tokens.
- **Real Wall:** LLM hallucinates causal claims not grounded in data; should be hypothesis generator not validator.
- **Cross-Domain Aliases:** llm-causal (agentic-reasoning), language-model-causal (ml-training).
- **Notes:** Kıcıman et al. (2023); LLMs propose, data validates.

### [PRIM-305] causal-inference-causal-graph-llm-elicit
- **Atom/Composite:** Composite
- **Definition:** Elicit causal graph from LLM: query LLM for pairwise causal relations, aggregate into graph, validate against data via conditional independence tests.
- **Cost Model:** O(d²) pairwise queries × LLM cost; + CI tests O(d²·n).
- **Real Wall:** LLM proposes plausible but unverified relations; combined with data tests, can fail consistency checks.
- **Cross-Domain Aliases:** llm-graph-elicit (agentic-reasoning), prompted-causal-discovery (ml-training).
- **Notes:** Long et al. (2023); LLM-aided causal discovery.

## 30. Optimal Transport & Distributional Causal Inference

### [PRIM-306] causal-inference-quantile-treatment-effect
- **Atom/Composite:** Primitive
- **Definition:** QTE(τ) = F_{Y(1)}^{-1}(τ) - F_{Y(0)}^{-1}(τ): difference in τ-quantile of potential outcomes; distributional treatment effect.
- **Cost Model:** Quantile estimation O(n log n) per quantile; QR-style O(n) per quantile.
- **Real Wall:** Quantile effects require quantile-specific assumptions; tails noisy with finite n.
- **Cross-Domain Aliases:** qte (statistics-probability), quantile-effect (ml-training).
- **Notes:** Firpo (2007); distributional effect at quantile level.

### [PRIM-307] causal-inference-conditional-quantile-treatment-effect
- **Atom/Composite:** Composite
- **Definition:** CQTE(τ|X=x) = Q_{Y(1)|X}(τ|x) - Q_{Y(0)|X}(τ|x): heterogeneous quantile effects by covariate.
- **Cost Model:** Quantile regression on each arm O(n·d) per quantile.
- **Real Wall:** Quantile crossing across τ; requires quantile-specific overlap.
- **Cross-Domain Aliases:** cqte (statistics-probability), conditional-quantile (ml-training).
- **Notes:** Chernozhukov, Hansen (2005); IV quantile regression for endogenous treatment.

### [PRIM-308] causal-inference-distributional-treatment-effect
- **Atom/Composite:** Primitive
- **Definition:** DTE: any functional contrast between F_{Y(1)} and F_{Y(0)} (variance, Gini, entropy, Wasserstein distance).
- **Cost Model:** Functional evaluation depends on choice; O(n log n) for empirical CDF + functional.
- **Real Wall:** Identification via marginal distributions; joint distribution of (Y(1),Y(0)) not identified.
- **Cross-Domain Aliases:** dte (statistics-probability), distributional-causal (ml-training).
- **Notes:** Generalizes ATE to distributional functionals.

### [PRIM-309] causal-inference-counterfactual-optimal-transport
- **Atom/Composite:** Composite
- **Definition:** OT-based counterfactual: transport observed distribution to counterfactual via optimal transport plan with cost reflecting "minimal change" principle.
- **Cost Model:** OT solver Sinkhorn O(n²) or exact O(n³).
- **Real Wall:** Cost function specification is normative; OT plan does not coincide with structural counterfactual in general.
- **Cross-Domain Aliases:** ot-counterfactual (statistics-probability), wasserstein-causal (ml-training).
- **Notes:** Black, Yeom, Fredrikson (2020); OT for algorithmic recourse.

### [PRIM-310] causal-inference-rank-preservation
- **Atom/Composite:** Primitive
- **Definition:** Rank preservation: if unit i has higher Y(0) than unit j, also has higher Y(1); strong assumption enabling joint distribution identification.
- **Cost Model:** Comonotonic coupling: rank-match O(n log n).
- **Real Wall:** Rarely holds; testable via observed ranks under specific designs.
- **Cross-Domain Aliases:** comonotonicity (statistics-probability), rank-invariance (ml-training).
- **Notes:** Heckman, Smith, Clements (1997); rank invariance for individual-level effects.

## 31. Negative Controls & Proximal Causal Inference

### [PRIM-311] causal-inference-negative-outcome-control
- **Atom/Composite:** Composite
- **Definition:** Negative outcome control: outcome Y_neg known not to be affected by treatment; non-null effect on Y_neg signals confounding bias.
- **Cost Model:** Standard estimation on Y_neg O(n); compare to null.
- **Real Wall:** Choice of Y_neg requires substantive knowledge of independence; quantitative correction not straightforward.
- **Cross-Domain Aliases:** negative-control (statistics-probability), nco (ml-training).
- **Notes:** Lipsitch et al. (2010); diagnostic for unmeasured confounding.

### [PRIM-312] causal-inference-negative-exposure-control
- **Atom/Composite:** Composite
- **Definition:** Negative exposure control: exposure A_neg known not to cause Y but shares confounders with A; significant Y~A_neg suggests confounding of A~Y.
- **Cost Model:** Standard estimation on A_neg O(n).
- **Real Wall:** Requires substantive identification of valid negative exposure; shared confounder assumption critical.
- **Cross-Domain Aliases:** negative-exposure (statistics-probability), nec (ml-training).
- **Notes:** Tchetgen Tchetgen et al. (2014); diagnostic tool.

### [PRIM-313] causal-inference-proximal-causal-inference
- **Atom/Composite:** Composite
- **Definition:** Proximal causal inference: use treatment and outcome proxies (W, Z) of unmeasured confounder U to identify effects via "bridge function" h(W,A,X).
- **Cost Model:** Bridge function via integral equation; nonparametric O(n²) or sieve O(n·k).
- **Real Wall:** Bridge function may not exist or not be unique; requires double-proxy structure.
- **Cross-Domain Aliases:** proximal-causal (statistics-probability), proxy-id (ml-training).
- **Notes:** Tchetgen Tchetgen, Ying, Cui, Shi, Miao (2020); modern identification under unmeasured confounding.

### [PRIM-314] causal-inference-proxy-confounder-control
- **Atom/Composite:** Composite
- **Definition:** Proxy confounder control: imperfect proxy of unmeasured confounder; standard adjustment yields residual bias; matrix-error methods correct.
- **Cost Model:** Errors-in-variables correction O(n·d).
- **Real Wall:** Requires knowledge of measurement error structure; can amplify bias if model wrong.
- **Cross-Domain Aliases:** proxy-control (statistics-probability), measurement-error-causal (ml-training).
- **Notes:** Kuroki, Pearl (2014); using proxies as adjustment with error correction.

### [PRIM-315] causal-inference-bridge-function
- **Atom/Composite:** Primitive
- **Definition:** Outcome bridge function h(W,A,X) satisfying E[Y|A,Z,X] = E[h(W,A,X)|A,Z,X]; central object in proximal identification.
- **Cost Model:** Integral equation solving O(n²) sieve or O(n) sieve with regularization.
- **Real Wall:** Identification requires completeness conditions; ill-posed inverse problem in general.
- **Cross-Domain Aliases:** bridge (statistics-probability), proxy-bridge (ml-training).
- **Notes:** Miao, Geng, Tchetgen Tchetgen (2018); analogous to confounding-bridge.

## 32. Federated, Distributed & Privacy-Preserving Causal Inference

### [PRIM-316] causal-inference-federated-causal
- **Atom/Composite:** Composite
- **Definition:** Federated causal inference: estimate causal effects across multiple data silos without sharing raw data; aggregate summary statistics or models.
- **Cost Model:** Local fit O(n_k·d) per site; aggregation O(K·d) for K sites.
- **Real Wall:** Heterogeneity across sites; identifying common estimand requires meta-analysis framework.
- **Cross-Domain Aliases:** federated-causal (ml-training), distributed-causal (statistics-probability).
- **Notes:** Vo et al. (2022); federated learning meets causal inference.

### [PRIM-317] causal-inference-secure-aggregation-causal
- **Atom/Composite:** Composite
- **Definition:** Secure aggregation of causal estimates: cryptographic protocols (MPC, homomorphic encryption) for combining estimators without revealing site data.
- **Cost Model:** MPC overhead O(K²·d) communication; HE multiplication O(n·log n) per ciphertext.
- **Real Wall:** Cryptographic overhead orders of magnitude vs plaintext; limited operations under HE.
- **Cross-Domain Aliases:** secure-causal (statistics-probability), mpc-causal (information-theory-coding).
- **Notes:** Combines secure multi-party computation with causal estimation.

### [PRIM-318] causal-inference-differential-privacy-causal
- **Atom/Composite:** Composite
- **Definition:** DP-causal: causal estimators with differential privacy guarantees; add calibrated noise to estimator or use private mechanisms.
- **Cost Model:** Noise calibration O(1); sensitivity analysis of estimator O(d).
- **Real Wall:** Privacy-utility tradeoff: small ε noisy estimates; large ε weak privacy.
- **Cross-Domain Aliases:** dp-causal (statistics-probability), private-ate (ml-training).
- **Notes:** Niu et al. (2022); DP-ATE estimation with privacy budget.

### [PRIM-319] causal-inference-meta-analysis-causal
- **Atom/Composite:** Composite
- **Definition:** Meta-analysis of causal effects: combine effect estimates from multiple studies via fixed or random effects models; check heterogeneity (I²).
- **Cost Model:** Weighted average O(K) for K studies; random effects iteration O(K) per step.
- **Real Wall:** Study-level confounding; differing definitions of treatment/outcome; publication bias.
- **Cross-Domain Aliases:** meta-causal (statistics-probability), pooled-estimate (ml-training).
- **Notes:** DerSimonian, Laird (1986); standard tool for pooling causal evidence.

### [PRIM-320] causal-inference-data-fusion-causal
- **Atom/Composite:** Composite
- **Definition:** Data fusion: combine experimental and observational data sources for richer identification; bound or sharpen effects via cross-source moments.
- **Cost Model:** Fusion estimator combining EIFs across sources; O(Σn_k·d).
- **Real Wall:** Selection mechanism between sources must be modeled; otherwise selection bias contaminates fusion.
- **Cross-Domain Aliases:** data-fusion (statistics-probability), multi-source-causal (ml-training).
- **Notes:** Bareinboim, Pearl (2016); structural framework for combining data sources.
