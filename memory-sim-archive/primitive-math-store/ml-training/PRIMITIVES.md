# ML / Training — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## Loss Functions

### loss-mse (cross-domain alias: `L2-loss`, `quadratic-loss`, `mean-squared-error`)
**Domain:** ML / Training
**Definition:** L = (1/n)·Σ(y_true − y_pred)². Measures average squared deviation.
**Atom or composite:** Composite: scan → fold(mean of squared errors).
**Cost model:** One subtraction + one square + one addition per sample.
**Real wall?** No. But MSE penalizes large errors quadratically — outliers dominate the gradient.
**Cross-domain wiring:** MSE = squared L2 distance. In physics: potential energy = m·v²/2. In signal: mean squared error = power of the error signal.

### loss-cross-entropy (cross-domain alias: `log-loss`, `softmax-loss`, `NLL-loss`)
**Domain:** ML / Training
**Definition:** L = −Σ y_true·log(y_pred). For multi-class classification with softmax.
**Atom or composite:** Composite: scan → fold(sum of −y_true·log(y_pred)).
**Cost model:** One log + one multiplication per class per sample.
**Real wall?** No. But cross-entropy is unbounded — predictions near 0 for true classes → log(0) → −∞.
**Cross-domain wiring:** Cross-entropy = D(P_true||P_pred) (KL divergence) + H(P_true). In information theory: log-loss is the cross-entropy between true and predicted distributions. In signal: log-likelihood in AWGN = cross-entropy.

### loss-bce (cross-domain alias: `binary-cross-entropy`, `logistic-loss`, `sigmoid-loss`)
**Domain:** ML / Training
**Definition:** L = −[y·log(p) + (1−y)·log(1−p)] for each binary classification.
**Atom or composite:** Composite: scan → fold(sum of BCE per sample).
**Cost model:** One sigmoid (or logit) + one log + one addition per sample.
**Real wall?** No. But BCE gradient saturates when predictions are very wrong — the model stops learning from confident wrong predictions.

### loss-hinge (cross-domain alias: `SVM-loss`, `max-margin-loss`, `感知器-loss`)
**Domain:** ML / Training
**Definition:** L = max(0, 1 − y·f(x)). Margin-based loss — zero if correctly classified beyond margin.
**Atom or composite:** Composite: compare margin to threshold → fold(max(0, threshold − margin)).
**Real wall?** No.
**Cross-domain wiring:** Hinge loss = margin violation penalty. In SVM: the margin constraint. In control: dead-zone nonlinearity. In signal: hard clipping.

### loss-huber (cross-domain alias: `smooth-L1`, `soft-hinge`, `Charbonnier-loss`)
**Domain:** ML / Training
**Definition:** L = 0.5·(y − f)² if |y−f| ≤ δ; else δ·|y−f| − 0.5·δ². Combines L2 for small errors, L1 for large.
**Atom or composite:** Composite: compare |error| to δ → if small: quadratic; if large: linear.
**Real wall?** No.
**Cross-domain wiring:** Huber loss = combination of L2 and L1. In statistics: Tukey's biweight function. In physics:弾簧 with linear friction at large displacement.

### loss-focal (cross-domain alias: `retina-loss`, `class-balanced-loss`)
**Domain:** ML / Training
**Definition:** L = −α·(1−p)^γ · log(p). Down-weights easy examples, focuses on hard ones. γ=2 typical.
**Atom or composite:** Composite: BCE loss × (1−p)^γ.
**Cost model:** One extra multiply + one power per sample vs BCE.
**Real wall?** No.
**Cross-domain wiring:** Focal loss = importance-weighted BCE where the importance decays as (1−p)^γ. In retrieval: weighted loss that down-weights frequent terms (like IDF).
**Notes:** γ controls how much to down-weight: γ=0 → standard BCE. γ=1 → mild focus. γ=2 → strong focus.

### loss-triplet-margin (cross-domain alias: `siamese-loss`, `contrastive-embedding-loss`)
**Domain:** ML / Training
**Definition:** L = max(0, ||a−p||² − ||a−n||² + margin). Anchor a, positive p (same class), negative n (different class). Pushes same-class closer, different-class apart.
**Atom or composite:** Composite: compare(dist(a,p), dist(a,n)) → fold(max(0, diff + margin)).
**Real wall?** No.
**Cross-domain wiring:** In retrieval: triplet loss = learning a distance metric where relevant docs are closer than irrelevant ones. In metric learning: same as LMNN, contrastive loss.
**Notes:** Hard negative mining (selecting the closest negative) is critical — easy negatives teach the model nothing.

### loss-contrastive (cross-domain alias: `NT-Xent`, `SimCLR-loss`, `infoNCE-loss`)
**Domain:** ML / Training
**Definition:** L = −log(exp(sim(z_i,z_j)/τ) / Σ_k exp(sim(z_i,z_k)/τ)). Positive pair i,j; negatives k≠i. τ is temperature.
**Atom or composite:** Composite: compute similarity matrix → compare positive pair score to all pair scores → fold(log-sum-exp).
**Cost model:** O(N²) similarity computations per batch (N = batch size).
**Real wall?** No. But O(N²) is expensive — contrastive learning doesn't scale to very large N without tricks.
**Cross-domain wiring:** infoNCE = importance-sampled estimate of mutual information. In information theory: maximizing I(z_i; z_j) is exactly the infoNCE objective. In retrieval: contrastive learning = learning an embedding where similar documents have high mutual information.
**Notes:** Temperature τ controls how sharp the distribution is: low τ → sharper, harder negatives matter more.

### loss-GAN-generator (cross-domain alias: `adversarial-loss`, `minimax-loss`, `Wasserstein-loss`)
**Domain:** ML / Training
**Definition:** L_G = −log(D(G(z))) (standard). WGAN: L = −D(G(z)). GAN training = minimax between generator and discriminator.
**Atom or composite:** Composite: generate fake → discriminate → compare score to real → fold gradients.
**Real wall?** Yes — mode collapse, training instability, and non-convergence are endemic. The minimax objective doesn't have a unique equilibrium.
**Cross-domain wiring:** GAN training = game theory Nash equilibrium. In signal: spoofing detection = discriminator discriminating real from fake. In control: min-max control (H-infinity).
**Notes:** Wasserstein GAN (WGAN-GP) replaces the Jensen-Shannon divergence with Earth-Mover distance, which has better gradient properties.

---

## Optimizers

### SGD (cross-domain alias: `stochastic-gradient-descent`, `SGD`)
**Domain:** ML / Training
**Definition:** θ_{t+1} = θ_t − η·∇L(θ_t; x_t). Update by negative gradient of the loss on one sample (or minibatch).
**Atom or composite:** Composite: compute gradient → scale by learning rate → subtract from parameters.
**Real wall?** No.
**Cross-domain wiring:** Gradient descent = gradient flow in parameter space. In physics: steepest descent on an energy landscape = same mathematical structure. In control: gradient-based policy optimization.
**Notes:** For batch SGD (full dataset), this is O(N) per step. For minibatch, it's O(batch_size). The noise from stochastic sampling is the "temperature" of the optimization.

### SGD-momentum (cross-domain alias: `momentum`, `Nesterov-momentum`)
**Domain:** ML / Training
**Definition:** v_{t+1} = β·v_t + ∇L(θ_t); θ_{t+1} = θ_t − η·v_{t+1}. Accumulated velocity dampens oscillation and accelerates convergence.
**Atom or composite:** Composite: velocity update (scale gradient × β) + parameter update (scale velocity × η).
**Real wall?** No. But momentum hyperparameter β must be tuned — too high = overshoot; too low = slow.
**Cross-domain wiring:** Momentum in optimization = inertia in physics. Nesterov momentum = lookahead gradient = same as "anticipation" in control theory.
**Notes:** Nesterov accelerated gradient (NAG): compute gradient at θ_t − β·v_t (lookahead) instead of θ_t. Faster convergence, particularly for convex functions.

### adam (cross-domain alias: `adaptive-moment-estimation`, `Adam`)
**Domain:** ML / Training
**Definition:** m_t = β₁·m_{t-1} + (1−β₁)·g_t; v_t = β₂·v_{t-1} + (1−β₂)·g_t²; θ_{t+1} = θ_t − η·m_t/(√v_t + ε). Adaptive per-parameter learning rates from running estimates of first and second moments.
**Atom or composite:** Composite: update first moment (momentum-like) + update second moment (RMSProp-like) + bias correction + parameter update.
**Real wall?** No. But bias correction is necessary for early steps when moments are biased toward zero.
**Cross-domain wiring:** Adam = momentum + RMSProp combined. RMSProp adapts per-parameter learning rate by dividing by running average of gradient squared — same as AGC in control.

### adamW (cross-domain alias: `Adam-weight-decay`, `decoupled-weight-decay`)
**Domain:** ML / Training
**Definition:** AdamW = Adam + explicit L2 regularization decoupled from the adaptive learning rate. θ_{t+1} = θ_t − η·(m_t/(√v_t + ε) + λ·θ_t).
**Atom or composite:** Composite: Adam update + explicit weight decay step.
**Real wall?** No.
**Cross-domain wiring:** In standard Adam, L2 regularization interacts with the adaptive learning rates — effectively applying different weight decay per parameter. AdamW decouples this, giving cleaner regularization.
**Notes:** AdamW produces better regularization for transformer models. The explicit λ·θ_t term is the L2 penalty, independent of the adaptive gradient scaling.

### rmsprop (cross-domain alias: `root-mean-square-prop`, `adaptive-lr`)
**Domain:** ML / Training
**Definition:** v_t = β·v_{t-1} + (1−β)·g_t²; θ_{t+1} = θ_t − η·g_t/√(v_t + ε). Per-parameter adaptive learning rate based on running average of squared gradients.
**Atom or composite:** Composite: update running second moment + divide gradient by RMS + update.
**Real wall?** No.
**Cross-domain wiring:** RMSProp = per-parameter gain scheduling in control theory. The denominator is an adaptive gain that reduces the step size for parameters with large gradients.

### ftrl (cross-domain alias: `Follow-The-Regularized-Leader`, `online-learning`)
**Domain:** ML / Training
**Definition:** Online convex optimization: maintain cumulative loss + regularization. Per-coordinate learning rates based on sum of squared gradients.
**Atom or composite:** Composite: accumulate per-coordinate squared gradients → solve for optimal update analytically → apply.
**Real wall?** No.
**Cross-domain wiring:** FTRL = online convex optimization. The "leader" is the best fixed strategy in hindsight; FTRL tracks it. In game theory: no-regret learning.
**Notes:** FTRL with L1 regularization produces sparse models — many parameters go to exactly zero. Used in large-scale click-through rate prediction.

### lars (cross-domain alias: `layer-wise-adaptive-rate`, `LARS`)
**Domain:** ML / Training
**Definition:** Compute local gradient norm / weight norm ratio → scale global LR by this ratio. Per-layer adaptive learning rate based on the ratio of gradient to weight norms.
**Atom or composite:** Composite: compute ratio per layer → scale global LR → apply SGD.
**Real wall?** No.
**Cross-domain wiring:** LARS = per-layer adaptive scaling. In physics: the ratio of driving force to resistance determines local acceleration.

---

## Schedulers

### sched-step (cross-domain alias: `step-decay`, `piecewise-decay`)
**Domain:** ML / Training
**Definition:** η_t = η_0 · γ^{⌊t/p⌋}. Drop learning rate by factor γ every p steps.
**Atom or composite:** Composite: compare(step mod period) → scale learning rate.
**Real wall?** No.
**Cross-domain wiring:** Step decay = quantized exponential decay. In control: gain scheduling with discrete steps.

### sched-cosine (cross-domain alias: `cosine-annealing`, `SGDR`)
**Domain:** ML / Training
**Definition:** η_t = η_min + 0.5·(η_max − η_min)·(1 + cos(t·π/T)). Smooth cosine annealing from η_max to η_min over T steps.
**Atom or composite:** Composite: compute cosine(t·π/T) → scale LR.
**Real wall?** No.
**Cross-domain wiring:** Cosine annealing = simulated cooling schedule in simulated annealing. The "temperature" of the optimization decreases according to cos(t).

### sched-warmup (cross-domain alias: `linear-warmup`, `gradual-warmup`)
**Domain:** ML / Training
**Definition:** Linearly increase LR from η_min to η_max over W warmup steps, then switch to main schedule.
**Atom or composite:** Composite: for t < W: η = η_min + (η_max−η_min)·(t/W). After W: use main scheduler.
**Real wall?** No. But warmup is necessary for large models — without it, early gradient updates are unstable.
**Cross-domain wiring:** In control: ramping up control authority gradually prevents actuator saturation and transient instability.

### sched-poly (cross-domain alias: `polynomial-decay`, `inverse-sqrt-decay`)
**Domain:** ML / Training
**Definition:** η_t = η_0·(1 − t/T)^p. Polynomial decay to near-zero over T steps.
**Atom or composite:** Composite: compute (1 − t/T)^p → scale LR.
**Real wall?** No.
**Cross-domain wiring:** Polynomial decay is slower than exponential early, then faster near the end. In physics: polynomial cooling = slower quench = better annealing results.

---

## Activations

### relu (cross-domain alias: `rectified-linear`, `ReLU`)
**Domain:** ML / Training
**Definition:** f(x) = max(0, x). Zeroes negative inputs, passes positive unchanged.
**Atom or composite:** Atom
**Real wall?** No. But dying ReLU: neurons that output zero stop learning (gradient = 0 for negative inputs).
**Cross-domain wiring:** ReLU = half-wave rectification. In signal: half-wave rectifier. In electronics: diode clipper.

### leaky-relu (cross-domain alias: `PReLU`, `ELU`)
**Domain:** ML / Training
**Definition:** f(x) = max(α·x, x) where α is small (e.g., 0.01 for Leaky, learned for PReLU).
**Atom or composite:** Composite: compare(x, 0) → scale and pass if negative.
**Real wall?** No.
**Cross-domain wiring:** Leaky ReLU = half-wave rectifier with small negative pass-through. In signal: biased detector with small offset.

### gelu (cross-domain alias: `Gaussian-Error-Linear-Unit`, `Gaussian-ERF`)
**Domain:** ML / Training
**Definition:** f(x) = x·Φ(x) where Φ is the standard Gaussian CDF. Approximated as: 0.5·x·(1 + tanh(√(2/π)·(x + 0.044715·x³))).
**Atom or composite:** Composite: compute CDF approximation → multiply by input.
**Real wall?** No.
**Cross-domain wiring:** GELU is a smoothed, probabilistic version of ReLU — it weights inputs by their probability under a Gaussian. In signal: weighted half-wave rectification.
**Notes:** GELU is the activation in BERT and most modern transformers. Slightly more expensive than ReLU but empirically superior.

### softmax (cross-domain alias: `normalized-exponential`, `soft-argmax`)
**Domain:** ML / Training
**Definition:** softmax(x)_i = exp(x_i) / Σ_j exp(x_j). Maps arbitrary real scores to a probability distribution.
**Atom or composite:** Composite: compute exp(x) → fold(sum) → divide each exp(x) by sum.
**Real wall?** No. But exp(x) can overflow for large x — subtract max(x) before exponentiating (numerically stable version).
**Cross-domain wiring:** Softmax = Gibbs distribution = Boltzmann distribution = e^{−E/kT} in physics. Temperature T controls sharpness. In information theory: softmax with temperature T is the maximum entropy distribution constrained to match the mean.

### swish (cross-domain alias: `self-gated-activation`, `SiLU`)
**Domain:** ML / Training
**Definition:** f(x) = x·sigmoid(x). Self-gated: the input gates itself via a sigmoid.
**Atom or composite:** Composite: compute sigmoid(x) → multiply by x.
**Real wall?** No.
**Cross-domain wiring:** Swish = multiplicative interaction between input and its sigmoid. In signal: product detector = multiply by gating signal.

---

## Attention

### attention-scaled-dot-product (cross-domain alias: `QKV-attention`, `self-attention`)
**Domain:** ML / Training
**Definition:** Attention(Q,K,V) = softmax(Q·Kᵀ/√d_k)·V. Scaled dot product attention: divide Q·Kᵀ by √d_k to prevent softmax saturation.
**Atom or composite:** Composite: project(Q) + project(K) + project(V) → Q·Kᵀ → scale → softmax → matmul(V) → scale output.
**Real wall?** Yes — O(n²·d) for sequence length n. The quadratic complexity in sequence length is the fundamental bottleneck.
**Cross-domain wiring:** Attention = soft lookup in key-value memory. In retrieval: attention weights = soft relevance scores for each key. In signal: matched filtering (Q·Kᵀ/√d = normalized correlation).

### attention-multi-head (cross-domain alias: `MHA`, `transformer-attention`)
**Domain:** ML / Training
**Definition:** Split Q, K, V into h heads, run attention in each head, concatenate outputs, project. MHA(Q,K,V) = Concat(head_1,...,head_h)·W^O where head_i = Attention(QW_i^Q, KW_i^K, VW_i^V).
**Atom or composite:** Composite: split Q,K,V into h heads → h × scaled-dot-product-attention → concat → project.
**Real wall:** O(h·n²·d) total. Same quadratic in n as single-head.
**Cross-domain wiring:** Multi-head = ensemble of attention mechanisms, each looking for different types of relationships. In retrieval: multi-head = multiple "query types" simultaneously.

### attention-flash (cross-domain alias: `FlashAttention`, `IO-aware-attention`)
**Domain:** ML / Training
**Definition:** Attention computed in tiles that fit in SRAM, avoiding materializing the full N×N attention matrix in HBM. Achieves O(N²·d) compute with O(N·d) memory.
**Atom or composite:** Composite: tile Q,K,V → compute attention per tile in SRAM → accumulate in HBM.
**Real wall:** No — but requires hardware support (SRAM bandwidth). The IO-bound is the real constraint.
**Cross-domain wiring:** FlashAttention = blocking + fusion to reduce memory bandwidth. In linear algebra: blocked matrix multiplication — same IO optimization principle.
**Notes:** This is the most important practical primitive for training transformers. Without it, long sequences don't fit in GPU memory.

### attention-cross (cross-domain alias: `encoder-decoder-attention`, `cross-attention`)
**Domain:** ML / Training
**Definition:** Q from decoder, K and V from encoder. Allows decoder to attend to the encoder's representation.
**Atom or composite:** Composite: Q from current layer → K,V from encoder output → scaled-dot-product-attention.
**Real wall:** No.
**Cross-domain wiring:** Cross-attention = retrieval of encoder context conditioned on decoder query. In retrieval: query attending to documents = cross-attention.

### attention-xpos (cross-domain alias: `exponential-decay-position`, `RoPE`, `ALiBi`)
**Domain:** ML / Training
**Definition:** Rotary Position Embedding: encode position by rotating Q and K in 2D subspaces: R(θ,m)·q = rotation by angle θ·m applied to q. Alternatively: ALiBi = linear bias on attention weights proportional to distance.
**Atom or composite:** Composite: for RoPE: rotate each head's Q and K vectors by their position encoding → then compute dot product.
**Real wall:** No.
**Cross-domain wiring:** RoPE = encoding position as a rotation = Fourier features for position. In signal: phasor representation = complex exponential for position/phase.

---

## Pooling

### pool-max (cross-domain alias: `max-pool`, `MAX-pool`)
**Domain:** ML / Training
**Definition:** Output the maximum value in each pooling window. Preserves the strongest activation.
**Atom or composite:** Composite: partition into windows → fold(max) per window.
**Real wall:** No. But max pooling is non-differentiable — use straight-through estimator or learnable pooling.
**Cross-domain wiring:** Max pooling = morphological dilation followed by threshold. In signal: peak detector = max over a sliding window.

### pool-avg (cross-domain alias: `mean-pool`, `average-pool`)
**Domain:** ML / Training
**Definition:** Output the average value in each pooling window. Smooths the representation.
**Atom or composite:** Composite: partition into windows → fold(mean) per window.
**Real wall:** No.
**Cross-domain wiring:** Average pooling = low-pass filter in the spatial domain. In signal: moving average filter.

### pool-global (cross-domain alias: `global-average-pooling`, `GAP`)
**Domain:** ML / Training
**Definition:** Pool over the entire spatial extent, outputting one value per channel. Replaces fully-connected layers in image classification.
**Atom or composite:** Composite: scan entire spatial extent → fold(mean).
**Real wall:** No.
**Cross-domain wiring:** Global average pooling = integral over the domain. In signal: DC component of a Fourier transform. In physics: spatial average over the entire field.

### pool-spp (cross-domain alias: `spatial-pyramid-pooling`, `SPP`)
**Domain:** ML / Training
**Definition:** Pool at multiple window sizes (e.g., 1×1, 2×2, 4×4) and concatenate. Produces fixed-length output regardless of input size.
**Atom or composite:** Composite: for each bin size: partition → pool → concat outputs.
**Real wall:** No.
**Cross-domain wiring:** Spatial pyramid = multi-resolution analysis = Gaussian/Laplacian pyramid. In signal: multi-rate signal processing.

---

## Embeddings

### embed-lookup (cross-domain alias: `token-embedding`, `embedding-table`)
**Domain:** ML / Training
**Definition:** Table lookup: given token ID i, return the i-th row of the embedding matrix E ∈ ℝ^{V×d}.
**Atom or composite:** Composite: index(E, token_id) → return row.
**Real wall:** No. But embedding table size = V·d — memory scales with vocabulary × dimension.
**Cross-domain wiring:** Embedding lookup = one-hot encode → matmul(Eᵀ, one_hot) = E[i]. In signal: LUT (look-up table) = discrete embedding.

### embed-positional (cross-domain alias: `position-embedding`, `sinusoidal-positional-encoding`)
**Domain:** ML / Training
**Definition:** Encode position as sinusoidal features: PE(pos,2i) = sin(pos/10000^{2i/d}); PE(pos,2i+1) = cos(...). Or learned.
**Atom or composite:** Composite: compute positional features (sin/cos or learned) → add to token embeddings.
**Real wall:** No. But sinusoidal generalize to longer sequences than seen during training; learned do not.
**Cross-domain wiring:** Sinusoidal PE = Fourier features for position = expressing position in the Fourier basis. In signal: Fourier series encodes a function as a sum of sinusoids.

### embed-entity (cross-domain alias: `entity-embedding`, `categorical-embedding`)
**Domain:** ML / Training
**Definition:** Embed categorical variables (user ID, product ID, city) into continuous space. Same mechanism as word embeddings.
**Atom or composite:** Composite: index → lookup → return embedding.
**Real wall:** No. But large cardinality categories need many embedding dimensions to avoid excessive collisions.

### embed-graph-node2vec (cross-domain alias: `node2vec`, `DeepWalk`, `LINE`)
**Domain:** ML / Training
**Definition:** Random walk on graph → treat walk as sentence → skip-gram on "sentence" of node IDs. node2vec adds bias toward BFS (structural) or DFS (homophily) walks.
**Atom or composite:** Composite: random walk generator → scan walk → skip-gram training.
**Real wall:** No.
**Cross-domain wiring:** Random walk on graph = diffusion on graph. Skip-gram on walk = language model on the diffusion path. In physics: random walk sampling = Metropolis algorithm.
**Notes:** node2vec produces node embeddings that capture both structural equivalence (BFS) and homophily (DFS).

### embed-kg (cross-domain alias: `TransE`, `DistMult`, `ComplEx`, `RotatE`)
**Domain:** ML / Training
**Definition:** Embed knowledge graph entities and relations. TransE: h + r ≈ t. DistMult: score = h·diag(r)·t. ComplEx: complex embeddings enabling asymmetric relations. RotatE: h·r = t (rotation in complex space).
**Atom or composite:** Composite: embed entities + relations → compute score via the scoring function.
**Real wall:** No. But knowledge graphs are incomplete — embeddings trained on observed facts may not generalize to unseen combinations.
**Cross-domain wiring:** KG embeddings = factorization of the adjacency tensor. ComplEx = complex SVD. TransE = translating in embedding space = additive relation.
**Notes:** ComplEx is currently the strongest baseline for KG completion.

---

## Normalization

### norm-batch (cross-domain alias: `BN`, `batch-normalization`)
**Domain:** ML / Training
**Definition:** y = γ·(x − μ_B)/√(σ_B² + ε) + β. Normalize activations to zero mean, unit variance within each minibatch.
**Atom or composite:** Composite: compute mean and variance over batch → normalize → scale by γ and shift by β.
**Real wall:** Yes — BN fails for very small batch sizes (batch size 1 = undefined variance). Its performance degrades significantly below ~16.
**Cross-domain wiring:** BN = whitening in statistics (decorrelating + scaling). In signal: AGC + DC removal. In physics: subtracting the mean and normalizing variance.

### norm-layer (cross-domain alias: `LN`, `layer-normalization`)
**Domain:** ML / Training
**Definition:** y = γ·(x − μ_H)/√(σ_H² + ε) + β. Normalize over the feature dimension (not the batch) for each individual sample.
**Atom or composite:** Composite: compute mean and variance over hidden dimension → normalize → scale and shift.
**Real wall:** No. But LN doesn't use batch statistics — no dependence on batch size.
**Cross-domain wiring:** LN = per-sample normalization. In signal: normalizing a signal vector to zero mean and unit variance within the signal itself.
**Notes:** LN is the standard for autoregressive models (transformer decoders) because it doesn't couple samples across the batch dimension.

### norm-group (cross-domain alias: `GN`, `group-normalization`)
**Domain:** ML / Training
**Definition:** Divide channels into G groups, normalize within each group. GN = between BN and LN: G=1 → LN, G=C → BN.
**Atom or composite:** Composite: partition channels into groups → fold over each group → normalize → scale and shift.
**Real wall:** No. GN is independent of batch size — works for any batch size.
**Cross-domain wiring:** Group normalization = partial whitening within subgroups of the channel dimension. In signal: subband normalization.

### norm-weight (cross-domain alias: `weight-standardization`, `WS`)
**Domain:** ML / Training
**Definition:** Normalize weights per filter: w_norm = (w − μ_w)/σ_w. Used before activation to improve training stability.
**Atom or composite:** Composite: compute weight mean and variance → normalize weights.
**Real wall:** No.
**Cross-domain wiring:** Weight standardization = whitening the weight distribution. In signal: normalizing filter coefficients to unit L2 norm.

---

## Architecture Primitives

### arch-residual (cross-domain alias: `skip-connection`, `resnet`, `shortcut`)
**Domain:** ML / Training
**Definition:** y = F(x) + x. Add the input directly to the transformed output. Gradient flows directly through the shortcut.
**Atom or composite:** Composite: compute transform F(x) → combine(F(x), x) = F(x) + x.
**Real wall:** No.
**Cross-domain wiring:** Residual connection = identity mapping + correction. In numerical methods: Newton's method with identity as the preconditioner. In physics: perturbation theory = exact solution + small correction.

### arch-dense (cross-domain alias: `dense-connect`, `DenseNet`)
**Domain:** ML / Training
**Definition:** Each layer receives feature maps from all preceding layers as input: x_l = H([x_0, x_1, ..., x_{l-1}]).
**Atom or composite:** Composite: concat all previous feature maps → project → output.
**Real wall:** Yes — memory grows with depth because all feature maps must be stored for the concatenations.
**Cross-domain wiring:** Dense connections = all-to-all coupling. In signal: multi-rate filter bank with all paths. In physics: mean-field approximation in spin systems.

### arch-se (cross-domain alias: `Squeeze-Excitation`, `channel-attention`)
**Domain:** ML / Training
**Definition:** Squeeze: global average pooling → Excite: two FC layers (bottleneck) → reweight channels: y = x · σ(W₂·ReLU(W₁·GAP(x))).
**Atom or composite:** Composite: GAP → project down → ReLU → project up → sigmoid → scale channels.
**Real wall:** No. Small computational overhead (2-3% of total).
**Cross-domain wiring:** SE block = channel-wise gating = attention over channels. In signal: channel selection = adaptive filter bank gain.

### arch-attention-feedforward (cross-domain alias: `transformer-feedforward`, `FFN`)
**Domain:** ML / Training
**Definition:** FFN(x) = W₂·σ(W₁·x). Two linear layers with a non-linearity. Typically expansion ratio 4 (d_model → 4d → d_model).
**Atom or composite:** Composite: project up → activate → project down.
**Real wall:** No. But FFN uses ~2/3 of transformer parameters — the most parameter-heavy component.
**Cross-domain wiring:** FFN = two-layer perceptron = universal function approximator. In signal: two-stage filter bank.

---

## Regularization

### reg-dropout (cross-domain alias: `dropout`, `variational-dropout`, `drophead`)
**Domain:** ML / Training
**Definition:** During training: randomly zero a fraction p of activations: y = (x · mask) / (1−p). During inference: no masking (equivalent to scaling by 1−p at train time).
**Atom or composite:** Composite: generate binary mask → multiply activations → scale.
**Real wall:** No. But p must be tuned — too high = underfitting, too low = insufficient regularization.
**Cross-domain wiring:** Dropout = ensemble of sub-networks. In signal: stochastic sampling in Monte Carlo. In physics: random pruning of a network.
**Notes:** Dropout = bagging over exponentially many sub-networks. The scaling (1/(1-p)) ensures the expected value of the output matches the expected value without dropout.

### reg-mixup (cross-domain alias: `mixup`, `cutmix`)
**Domain:** ML / Training
**Definition:** Mixup: x_mixed = λ·x_i + (1−λ)·x_j; y_mixed = λ·y_i + (1−λ)·y_j where λ ~ Beta(α,α). CutMix: copy a patch from x_j into x_i.
**Atom or composite:** Composite: sample pair → blend → mix labels proportionally.
**Real wall:** No. But mixup destroys the label structure for multi-class problems — interpolating "dog and cat" is not a valid class.
**Cross-domain wiring:** Mixup = linear interpolation in input space = data augmentation via convex combinations. In signal: cross-fade between two signals. In physics: mixture distribution.

### reg-label-smooth (cross-domain alias: `label-smoothing`, `LSR`)
**Domain:** ML / Training
**Definition:** Instead of hard labels (0 or 1), use soft labels: y_smooth = (1−ε)·y_hard + ε/K where K is number of classes.
**Atom or composite:** Composite: blend hard label with uniform distribution → compute loss against smoothed target.
**Real wall:** No.
**Cross-domain wiring:** Label smoothing = regularization by adding entropy to the target distribution. In information theory: adding uniform noise = maximum entropy regularization.

### reg-weight-decay (cross-domain alias: `L2-regularization`, `ridge`)
**Domain:** ML / Training
**Definition:** Add λ·||θ||² to the loss. Equivalent to updating: θ_{t+1} = (1−2λη)·θ_t − η·∇L.
**Atom or composite:** Composite: add penalty to loss → gradient includes 2λ·θ.
**Real wall:** No.
**Cross-domain wiring:** L2 regularization = Gaussian prior on weights (MAP with Gaussian prior). In physics: potential energy well = restoring force proportional to displacement.
**Notes:** In AdamW, L2 regularization is decoupled from adaptive learning rates — weight decay acts directly on the parameters.

### reg-spectral-norm (cross-domain alias: `spectral-normalization`, `SN`)
**Domain:** ML / Training
**Definition:** Normalize the spectral norm (largest singular value) of each weight matrix to 1: W_SN = W / σ_max(W). This constrains the Lipschitz constant of the network.
**Atom or composite:** Composite: compute σ_max(W) via power iteration → divide W by σ_max.
**Real wall:** No. But computing σ_max via power iteration is expensive — used per iteration.
**Cross-domain wiring:** Spectral norm = operator norm = maximum amplification of a vector. In control: H-infinity norm = worst-case gain of a system.

---

## Augmentation

### aug-random-crop (cross-domain alias: `random-crop`, `jitter`)
**Domain:** ML / Training
**Definition:** Randomly crop a subregion of the input and resize to original size.
**Atom or composite:** Composite: sample crop coordinates → extract → resize.
**Real wall:** No.
**Cross-domain wiring:** Random crop = adding spatial noise to the input. In signal: random time-windowing.

### aug-random-flip (cross-domain alias: `horizontal-flip`, `mirror`)
**Domain:** ML / Training
**Definition:** Randomly flip along an axis. For images: horizontal flip is typically label-preserving.
**Atom or composite:** Composite: sample flip → apply (or not).
**Real wall:** No.

### aug-cutout (cross-domain alias: `random-erasing`, `cutout`)
**Domain:** ML / Training
**Definition:** Randomly mask a rectangular region with zero (or random values). Forces the model to be robust to partial occlusion.
**Atom or composite:** Composite: sample region → zero out → pass.
**Real wall:** No.
**Cross-domain wiring:** Cutout = random ablation of input signal. In physics: knockout experiments in biological systems.

### aug-mixup (cross-domain alias: `mixup-augmentation`, `input-mixup`)
**Domain:** ML / Training
**Definition:** Generate synthetic training examples by linearly interpolating between pairs of examples.
**Atom or composite:** Composite: sample pair → blend → mix labels.
**See:** reg-mixup.

### aug-autoaugment (cross-domain alias: `RandAugment`, `AutoAugment`, `TrivialAugment`)
**Domain:** ML / Training
**Definition:** Apply a sequence of augmentations sampled from a learned or random policy. RandAugment: pick N operations, each with magnitude M.
**Atom or composite:** Composite: sample augmentation sequence → apply in order.
**Real wall:** No. But the augmentation policy can change the label semantics (e.g., color inversion destroys the label for traffic signs).
**Cross-domain wiring:** AutoAugment = learning an augmentation policy = hyperparameter optimization over the augmentation space.

---

## Training Dynamics

### train-gradient-clip (cross-domain alias: `gradient-clipping`, `grad-clip`)
**Domain:** ML / Training
**Definition:** After backprop, clip gradients to a maximum norm: if ||g|| > max_norm: g = g·(max_norm/||g||).
**Atom or composite:** Composite: compute ||g|| → compare to max_norm → scale if exceeded.
**Real wall:** No. But clip threshold must be tuned — too small = kills learning, too large = no effect.
**Cross-domain wiring:** Gradient clipping = bounded step size in parameter space. In optimization: projected gradient descent with an L2 ball constraint.
**Notes:** Critical for RNNs (gradient explosion) and transformers (especially with warmup issues).

### train-mixed-precision (cross-domain alias: `FP16-training`, `BF16-training`, `AMP`)
**Domain:** ML / Training
**Definition:** Store weights in FP32 for updates, but compute forward/backward in FP16 (or BF16) for speed. Use loss scaling to prevent underflow in FP16.
**Atom or composite:** Composite: cast to FP16 for forward/backward → cast gradients to FP32 for optimizer update.
**Real wall:** Yes — FP16 has limited dynamic range (65504 max). Loss scaling prevents small gradients from becoming zeros.
**Cross-domain wiring:** Mixed precision = trading dynamic range for throughput. In signal: fixed-point vs floating-point DSP.
**Notes:** BF16 (brain float) has the same exponent range as FP32 (8 bits) with only 7 mantissa bits. Much better dynamic range than FP16 — preferred for training.

### train-gradient-accumulate (cross-domain alias: `gradient-accumulation`, `virtual-batch`)
**Domain:** ML / Training
**Definition:** Accumulate gradients over multiple micro-batches before applying an optimizer step. Effective batch size = micro_batch × accumulation_steps.
**Atom or composite:** Composite: compute gradient on micro-batch → add to accumulator → when full: optimizer step + reset.
**Real wall:** No. But gradient accumulation changes the effective batch size — learning rate may need scaling.
**Cross-domain wiring:** Gradient accumulation = running average of gradients over time. In signal: integration over a longer time window.

### train-loss-scale (cross-domain alias: `dynamic-loss-scaling`, `DLS`)
**Domain:** ML / Training
**Definition:** Dynamically adjust the loss scaling factor to keep gradients in the representable range for FP16. If gradients are finite and not underflowing, increase scale; if NaN/Inf, decrease.
**Atom or composite:** Composite: scale loss → backprop → check for NaN/Inf → adjust scale.
**Real wall:** No.
**Cross-domain wiring:** Loss scaling = adaptive gain in AGC. In signal: automatic gain control.

---

## AutoML / Architecture Search

### nas-controller (cross-domain alias: `RNN-controller`, `DARTS`, `ENAS`)
**Domain:** ML / Training
**Definition:** A controller RNN that generates candidate architectures. Trained via policy gradient (REINFORCE) or via bilevel optimization (DARTS).
**Atom or composite:** Composite: controller samples architecture → train child network → update controller → repeat.
**Real wall:** Yes — NAS is computationally expensive (thousands of child network trainings). OnceNAS and ZeroNAS reduce cost.
**Cross-domain wiring:** Architecture search = meta-learning over the architecture space. In optimization: hyperparameter optimization.
**Notes:** DARTS: relax discrete architecture choice to continuous weights → joint optimize architecture weights and network weights.

### prune-magnitude (cross-domain alias: `magnitude-pruning`, `weight-pruning`)
**Domain:** ML / Training
**Definition:** Zero out weights with magnitude below a threshold: w = w if |w| > τ, else 0.
**Atom or composite:** Composite: compare |w| to τ → zero if below.
**Real wall:** No. But pruning changes the architecture — requires retraining or iterative pruning-finetuning.
**Cross-domain wiring:** Magnitude pruning = hard thresholding = step function on weight magnitude. In signal: noise gate = threshold on signal power.

### prune-lottery-ticket (cross-domain alias: `lottery-ticket-hypothesis`, `LTH`)
**Domain:** ML / Training
**Definition:** A randomly initialized network contains a subnetwork (the winning ticket) that, when trained in isolation, reaches the original's accuracy. Find it by: train → prune lowest-magnitude weights → reset remaining to original init → repeat.
**Atom or composite:** Composite: train → prune → reset to original init → retrain.
**Real wall:** No. But requires iterative training-pruning cycles — computationally expensive to find the ticket.

---

## Model Components

### model-lstm (cross-domain alias: `LSTM`, `long-short-term-memory`)
**Domain:** ML / Training
**Definition:** f = σ(W_f·[h_{t-1}, x_t] + b_f); i = σ(W_i·[h_{t-1}, x_t] + b_i); C̃ = tanh(W_C·[h_{t-1}, x_t] + b_C); C_t = f·C_{t-1} + i·C̃; o = σ(W_o·[h_{t-1}, x_t] + b_o); h_t = o·tanh(C_t). Gates control what to forget (f), what to add (i), and what to output (o).
**Atom or composite:** Composite: four matmuls (W_f,W_i,W_C,W_o) + four activations + combine (forget gate + input gate + output gate).
**Real wall:** No. But 4× the matmul cost of a vanilla RNN.
**Cross-domain wiring:** LSTM = gated linear recurrence. The forget gate = decay factor. In signal: IIR filter with adaptive coefficients.

### model-gru (cross-domain alias: `GRU`, `gated-recurrent-unit`)
**Domain:** ML / Training
**Definition:** z = σ(W_z·[h_{t-1}, x_t]); r = σ(W_r·[h_{t-1}, x_t]); h̃ = tanh(W·[r·h_{t-1}, x_t]); h_t = (1−z)·h_{t-1} + z·h̃.
**Atom or composite:** Composite: three matmuls + three activations + two combines (update gate + candidate hidden state).
**Real wall:** No. Fewer parameters than LSTM (3 vs 4 matmuls), often comparable performance.

### model-attention-pool (cross-domain alias: `self-attention-pooling`, `BiEncoder`)
**Domain:** ML / Training
**Definition:** For each pair of inputs: attend. For a single input: self-attention pooling: attention over positions, output = weighted sum of input representations.
**Atom or composite:** Composite: compute self-attention → weighted sum of input features.

### model-autoencoder (cross-domain alias: `AE`, `autoencoder`)
**Domain:** ML / Training
**Definition:** Encoder: x → z ∈ ℝ^d (bottleneck). Decoder: z → x̂. Train to minimize reconstruction error: L = ||x − D(E(x))||².
**Atom or composite:** Composite: encode → decode → compare → fold(loss).
**Real wall:** No.
**Cross-domain wiring:** Autoencoder = dimensionality reduction = PCA (linear) or nonlinear manifold learning. In signal: source coding with side information (z is the compressed representation).
**Notes:** Variational AE (VAE): encode as distribution (μ, σ), sample z ~ N(μ, σ), decode. Adds KL divergence regularization: L = reconstruction + D(N(μ,σ²)||N(0,1)).

### model-cnn-2d (cross-domain alias: `Conv2D`, `conv-layer`)
**Domain:** ML / Training
**Definition:** y = Conv2D(x, W) + b where y_{b,c,h,w} = Σ_{k_h,k_w,c'} W_{c,k_h,k_w,c'}·x_{b,c',h+k_h,w+k_w}.
**Atom or composite:** Composite: im2col (reorganize input patches) → matmul(W_col) → col2im (reorganize back).
**Real wall:** No. But the number of parameters = C_in × C_out × K_h × K_w — can be very large.
**Cross-domain wiring:** Conv2D = Toeplitz matrix multiplication = linear shift-invariant filtering. In signal: 2D convolution = cross-correlation with kernel.

### model-transpose-conv (cross-domain alias: `deconv`, `ConvTranspose2D`, `upconv`)
**Domain:** ML / Training
**Definition:** Upsampling via transposed convolution (also called deconvolution, incorrectly). Represents the backward pass of a convolution as a forward pass.
**Atom or composite:** Composite: can be implemented as: insert zeros between pixels → convolve with strided kernel. Or: pad → convolve → remove padding.
**Real wall:** No. But transpose convolution produces checkerboard artifacts — prefer bilinear upsampling + conv.

### model-depthwise-conv (cross-domain alias: `DWConv`, `depthwise-separable`)
**Domain:** ML / Training
**Definition:** Apply one filter per input channel: y_{b,c,h,w} = Σ_{k_h,k_w} W_{c,k_h,k_w}·x_{b,c,h+k_h,w+k_w}. Then 1×1 pointwise conv to mix channels.
**Atom or composite:** Composite: depthwise conv (one filter per channel) → pointwise conv (1×1 to mix channels).
**Real wall:** No. MobileNet uses this — ~8-9× fewer parameters than standard Conv2D.
**Cross-domain wiring:** Depthwise separable = factorized convolution = spatial filtering + channel mixing as separate steps.

### model-cgan (cross-domain alias: `CGAN`, `conditional-GAN`)
**Domain:** ML / Training
**Definition:** GAN conditioned on a label y: G(z, y) → x̂; D(x, y) → real/fake. The conditioning is injected into both G and D (concat, AdaIN, etc.).
**Atom or composite:** Composite: G(z, y) → generate → D(x̂, y) → discriminate → update.
**Real wall:** Yes — mode collapse, training instability. CGAN partially mitigates by giving the model a target distribution.

### model-diffusion-ddpm (cross-domain alias: `DDPM`, `denoising-diffusion-probabilistic-model`)
**Domain:** ML / Training
**Definition:** Forward: x_t = √(1−β_t)·x_{t-1} + √β_t·ε. Reverse: predict ε_θ(x_t, t) or μ_θ(x_t, t) → denoise one step. Trained by predicting the noise ε_θ.
**Atom or composite:** Composite: T forward diffusion steps (fixed) + T reverse denoising steps (learned) → at inference: T reverse steps.
**Real wall:** Yes — T can be 1000 steps for high-quality images. Each step requires a full model forward pass. Distillation reduces this (DDIM, consistency models).
**Cross-domain wiring:** Diffusion = annealing in a Gaussian noise space. In physics: diffusion equation describes the same stochastic process. In signal: progressive refinement = successive approximation.

### model-lm-autoregressive (cross-domain alias: `autoregressive-LM`, `next-token-prediction`)
**Domain:** ML / Training
**Definition:** Predict the next token given all previous tokens: L = −Σ log P(x_t | x_{<t}). Autoregressive factorization of the joint probability.
**Atom or composite:** Composite: encode context → predict distribution over next token → compute log likelihood.
**Real wall:** No. But AR generation is sequential (can't parallelize) — slow at inference.
**Cross-domain wiring:** Autoregressive LM = causal convolution (1D) or causal attention. In signal: autoregressive model (AR(p)) — predict next sample from past p samples.

### model-vae (cross-domain alias: `VAE`, `variational-autoencoder`)
**Domain:** ML / Training
**Definition:** Encoder outputs μ and log σ² → sample z ~ N(μ, σ) → decoder reconstructs. Loss: L = L_recon + β·D_KL(N(μ,σ²)||N(0,I)).
**Atom or composite:** Composite: encode → sample → decode → L_recon + β·L_KL.
**Real wall:** No.
**Cross-domain wiring:** VAE = probabilistic latent variable model = continuous generalization of discrete clustering. KL term = regularization toward the prior.
**Notes:** β-VAE: β > 1 forces more disentangled representations. β = 1 → standard VAE.

---

## Reinforcement Learning

### rl-q-learning (cross-domain alias: `Q-learning`, `DQN`)
**Domain:** ML / Training
**Definition:** Q(s,a) ← Q(s,a) + α·[r + γ·max_{a'} Q(s',a') − Q(s,a)]. Learn Q-values from experience.
**Atom or composite:** Composite: compute TD error → update Q-value estimate.
**Real wall:** No. But off-policy learning can diverge without sufficient exploration or function approximation instability.
**Cross-domain wiring:** Q-learning = Bellman optimality equation. In control: dynamic programming. In physics: value iteration on a Markov decision process.
**Notes:** DQN: Q-network + target network + experience replay + gradient clipping. These four tricks stabilize training enough to work.

### rl-policy-gradient (cross-domain alias: `REINFORCE`, `policy-gradient`)
**Domain:** ML / Training
**Definition:** ∇_θ J(θ) = E[∇_θ log π_θ(a|s) · Q^{π}(s,a)]. Gradient of expected return w.r.t. policy parameters.
**Atom or composite:** Composite: sample trajectory → compute return → compute log probability gradient → weight by return → update.
**Real wall:** Yes — high variance in policy gradient estimates. Baseline subtraction (Advantage) reduces variance but doesn't eliminate it.
**Cross-domain wiring:** Policy gradient = likelihood ratio method = score function estimator. In statistics: importance sampling estimator.

### rl-ppo (cross-domain alias: `PPO`, `proximal-policy-optimization`)
**Domain:** ML / Training
**Definition:** L^{CLIP}(θ) = E[min(r_t(θ)·A_t, clip(r_t(θ), 1−ε, 1+ε)·A_t)] where r_t = π_θ(a|s)/π_{θ_old}(a|s). Clips the policy ratio to prevent too-large updates.
**Atom or composite:** Composite: compute policy ratio r_t → compare to clip bounds → take minimum → compute weighted loss.
**Real wall:** No.
**Cross-domain wiring:** PPO clipping = trust region constraint. In optimization: projected gradient descent. In control: constrained MPC.
**Notes:** PPO with clipping is the most widely used RL algorithm for continuous control. Simpler than TRPO, more stable than vanilla policy gradient.

### rl-dqn (cross-domain alias: `Deep-Q-Network`, `target-network`)
**Domain:** ML / Training
**Definition:** DQN: Q-network + target Q-network (periodically copied) + experience replay + gradient clipping. Stabilizes Q-learning with function approximation.
**Atom or composite:** Composite: forward pass → compute TD target using target network → compute loss → update online network → periodically sync target.
**Real wall:** No.
**Cross-domain wiring:** Target network = slow update of the reference policy. In control: certainty equivalence = use current estimate as if it were correct.

---

## Summary: ML/Training Atom → Cross-Domain Wiring

| ML Primitive | Signal Alias | Optimization Alias | Physics Alias |
|---|---|---|---|
| loss-mse | mean squared error (power) | L2 objective | spring potential |
| loss-cross-entropy | log-likelihood | KL divergence | free energy |
| loss-hinge | margin loss | SVM objective | dead-zone |
| SGD | gradient flow | steepest descent | particle dynamics |
| adam | RMS + momentum | adaptive gain | adaptive damping |
| softmax | Boltzmann/Gibbs distribution | maximum entropy | thermal equilibrium |
| attention-QKV | matched filtering | soft lookup | correlation |
| attention-flash | blocked convolution | IO-optimized GEMM | tiled computation |
| embed-lookup | LUT | table lookup | lookup table |
| norm-batch | AGC + whitening | batch whitening | moment normalization |
| dropout | stochastic sampling | random ensemble | random pruning |
| gradient-clip | bounded step | projected gradient | bounded displacement |
| LSTM | gated IIR filter | adaptive memory | memory with decay |
| Q-learning | value iteration | dynamic programming | MDP solving |

---

*Last updated: 2026-06-21*
*Source doctrine: The Painted Fence — Jesse*


---

## Training Infrastructure Atoms

### distributed-data-parallel (cross-domain alias: `DDP`, `allreduce`, `gradient-sync`)
**Domain:** ML / Training
**Definition:** Replicate the model on N GPUs; each processes a different mini-batch; gradients are all-reduced across all replicas (summed or averaged). Synchronous SGD: wait for all replicas, then update.
**Atom or composite:** Composite: forward → backward on each GPU → allreduce(gradients) → step(optimizer) → all-reduce again if using sync batch norm.
**Cost model:** Gradient communication: O(model_size) per step. With NVLink (~600 GB/s), this is much faster than compute for large models.
**Real wall?** Yes — the all-reduce is a synchronization barrier. With N GPUs, the effective throughput is limited by the slowest GPU. Stragglers kill the efficiency.
**Cross-domain wiring:** DDP all-reduce = distributed consensus on the gradient. In signal: averaging signals from multiple sensors = all-reduce.
**Notes:** PyTorch DDP uses gradient bucketing to overlap communication with computation. Gradient accumulation enables large effective batch sizes with small memory per GPU.

### gradient-accumulate (cross-domain alias: `virtual-batch`, `accumulate-steps`, `effective-batch`)
**Domain:** ML / Training
**Definition:** Before optimizer step, accumulate gradients over multiple micro-batches (forward + backward on each). Only call optimizer step every N steps. Enables large effective batch with limited GPU memory.
**Atom or composite:** Composite: for each micro-batch: forward → backward → accumulate_grad() → every N batches: step(optimizer) → zero_grad().
**Cost model:** Zero additional compute; only extra memory for the accumulated gradients.
**Real wall?** No. But gradient accumulation changes the effective batch normalization statistics — ensure BN is computed over the true effective batch.
**Cross-domain wiring:** Gradient accumulation = temporal averaging of gradients = the same as EMA of the model weights. In signal: accumulating samples = integration over a longer window.
**Notes:** For large language model training (LLaMA, GPT-4 scale), gradient accumulation with DDP is the standard setup. The effective batch size = micro_batch × gradient_accumulation_steps × num_gpus.

### mixed-precision-train (cross-domain alias: `fp16-BF16`, `autocast`, `loss-scale`)
**Domain:** ML / Training
**Definition:** Use FP16 or BF16 for forward/backward passes (faster, less memory); maintain master weights in FP32 for the optimizer step (prevents gradient underflow).
**Atom or composite:** Composite: forward in FP16 → backward in FP16 → cast gradients to FP32 → loss scale (to avoid underflow) → optimizer step in FP32 → cast weights back to FP16.
**Cost model:** FP16 matmul is 2× faster than FP32 on Tensor Cores (NVIDIA). Memory reduction is ~2× (half-precision weights).
**Real wall?** Yes — gradient scaling is needed to avoid underflow (tiny gradients rounding to zero in FP16). Wrong loss scale causes training instability.
**Cross-domain wiring:** Mixed precision = trading numerical precision for speed/memory. In signal: fixed-point vs floating-point DSP — same trade-off.
**Notes:** BF16 (brain float) has the same exponent range as FP32 but only 8 fraction bits. It is more stable than FP16 for training because it rarely overflows.

### checkpoint-recover (cross-domain alias: `save-load`, `snapshot`, `fault-tolerance`)
**Domain:** ML / Training
**Definition:** Save model state (weights, optimizer state, gradients) to disk for recovery. Checkpoint every N steps. Recovery: load checkpoint → resume training.
**Atom or composite:** Composite: save(model_state_dict) + save(optimizer_state_dict) + save(step, lr, etc.) → write to disk.
**Cost model:** Model checkpoint = model_size (e.g., 7B params = ~14GB in FP16). Write time is the bottleneck.
**Real wall?** Yes — checkpoint writes can take minutes for large models. Async checkpointing (write in background thread) is essential.
**Cross-domain wiring:** Checkpoint = save state in a state machine = checkpoint in scientific simulation. In databases: checkpoint = flush state to disk for recovery.
**Notes:** Optimizer state checkpointing (saving optimizer state) is the dominant cost for large models — saves can be several GB. LLM training frameworks use overlapping checkpoint writes with training.

### tensor-parallel (cross-domain alias: `Megatron-LM`, `col-row-shard`, `all-gather`)
**Domain:** ML / Training
**Definition:** Partition a large tensor (e.g., weight matrix) across N GPUs. For matrix multiplication: partition along the hidden dimension; each GPU computes a partial result; all-gather to reconstruct the full output.
**Atom or composite:** Composite: shard weights → local matmul → all-gather(output) → optionally reduce.
**Cost model:** Communication = size of the output tensor per layer. For large N (8-64 GPUs), tensor parallelism requires high-bandwidth interconnects (NVLink).
**Real wall?** Yes — the all-gather communication is a bottleneck. Tensor parallelism requires NVLink (or similar) and doesn't scale beyond ~16 GPUs without model parallelism pipelining.
**Cross-domain wiring:** Tensor parallelism = distributed matrix multiplication = the same as the sharded matrix multiply in distributed linear algebra.
**Notes:** Megatron-LM shards the weight matrix by columns and rows. The attention and MLP layers are partitioned differently. This is the standard approach for training trillion-parameter models.

### pipeline-parallel (cross-domain alias: `pipe-async`, `micro-batch-schedule`, `GPipe`)
**Domain:** ML / Training
**Definition:** Partition layers across GPUs: GPU0 → GPU1 → GPU2 → ... Each GPU processes micro-batches in a pipeline. GPipe: forward pass fills the pipeline; backward pass drains it.
**Atom or composite:** Composite: for each micro-batch: forward through stage i → pass to stage i+1 → backward after fill → accumulate gradients.
**Cost model:** Pipeline bubble (idle GPUs waiting for the pipeline to fill/drain) reduces efficiency. With enough micro-batches, bubble overhead is small.
**Real wall?** Yes — the pipeline bubble is proportional to depth / num_microbatches. Too few micro-batches = large bubble. Too many = memory pressure.
**Cross-domain wiring:** Pipeline parallelism = assembly line for neural network computation. In distributed systems: pipeline parallelism = data pipeline with multiple stages.
**Notes:** PipeDream (2019) uses weight stashing (separate model replicas for different micro-batches) to avoid gradient staleness. This doubles the memory usage.

---

## Training Stability Atoms

### gradient-clip (cross-domain alias: `clip-norm`, `gradient-cap`, `explosion-stop`)
**Domain:** ML / Training
**Definition:** After computing gradients, clip them to a maximum norm: if ||g|| > clip_norm: g = g · (clip_norm / ||g||). Prevents gradient explosion.
**Atom or composite:** Composite: compute g → g_norm = ||g|| → if g_norm > clip_norm: g *= clip_norm / g_norm → optimizer.step().
**Cost model:** One norm computation + one comparison + one conditional scale. Negligible.
**Real wall?** No. But the optimal clip norm is data-dependent and must be tuned.
**Cross-domain wiring:** Gradient clipping = bounded step size in optimization = velocity clamping in physics. In control: bounded control input.
**Notes:** Gradient clipping is essential for recurrent networks (RNNs, LSTMs) where gradients can explode in long sequences. For Transformers, gradient clipping is less critical but still used.

### warmup-schedule (cross-domain alias: `lr-warmup`, `gradual-increase`, `precondition`)
**Domain:** ML / Training
**Definition:** Linearly increase the learning rate from a small value to the target over the first N steps. Addresses the problem that early gradients are large and poorly conditioned.
**Atom or composite:** Composite: if step < warmup_steps: lr = base_lr * (step / warmup_steps) → else: use main schedule.
**Cost model:** Negligible — just a conditional and a multiplication.
**Real wall?** Yes — warmup is necessary for large batch training and for training from scratch without pretrained weights. Without warmup, early updates can destabilize training.
**Cross-domain wiring:** Warmup = ramp-up period = spool-up time in physical systems. In control: gradual command to avoid overshoot.
**Notes:** The "deep learning warmup" (Goyal et al., 2018) showed that warmup is critical for large batch training — it allows the optimizer to adapt to the large batch statistics before scaling up the learning rate.

### early-stop (cross-domain alias: `patience`, `overfit-stop`, `validation-gate`)
**Domain:** ML / Training
**Definition:** Monitor validation loss; stop training when it stops improving for N consecutive epochs (patience). This prevents overfitting.
**Atom or composite:** Composite: at end of each epoch: compute val_loss → if val_loss > best + min_delta: patience -= 1 → if patience == 0: stop → else: best = val_loss, patience = patience.
**Cost model:** One extra forward pass over the validation set per epoch.
**Real wall?** No. But the patience parameter must be tuned — too small = premature stopping, too large = wasted compute.
**Cross-domain wiring:** Early stopping = the same as the patience parameter in optimization convergence criteria.
**Notes:** Learning rate scheduling (cosine decay, one-cycle) often achieves better results than early stopping because the learning rate schedule itself acts as regularization.

### batch-norm-adapt (cross-domain alias: `eval-mode`, `running-stats`, `frozen-statistics`)
**Domain:** ML / Training
**Definition:** During training, batch norm computes running mean and variance of each batch. During inference, it uses the running statistics (frozen). This makes BN work in both training and inference modes.
**Atom or composite:** Composite: train: μ_batch = mean(x) → σ²_batch = var(x) → BN = γ·(x − μ_batch)/√(σ²_batch + ε) + β → update running stats. Eval: BN = γ·(x − μ_running)/√(σ²_running + ε) + β.
**Cost model:** Batch norm adds negligible compute overhead. But it breaks the independence of samples — it couples samples within a batch.
**Real wall?** Yes — batch norm degrades for small batch sizes (the batch statistics are noisy) and for distributed training (different batches on different GPUs). GroupNorm and LayerNorm are alternatives.
**Cross-domain wiring:** Batch norm = whitening with learned scale/shift = normalizing flow. In signal: AGC (automatic gain control) = adaptive normalization.
**Notes:** SyncBatchNorm synchronizes batch norm statistics across GPUs before computing the running mean and variance — needed for tensor/pipeline parallelism where different GPUs see different batches.

---

## Data Augmentation Atoms

### mixup (cross-domain alias: `interpolate-mix`, `convex-mixup`, `manifold-mix`)
**Domain:** ML / Training
**Definition:** Mix two samples: x̃ = λ·x_i + (1−λ)·x_j, ỹ = λ·y_i + (1−λ)·y_j where λ ~ Beta(α, α). Simple and effective regularization.
**Atom or composite:** Composite: draw λ ~ Beta(α,α) → x_mix = λ·x₁ + (1−λ)·x₂ → y_mix = λ·y₁ + (1−λ)·y₂.
**Cost model:** Two forward passes for the mixed sample. Negligible.
**Real wall?** No. But mixup requires label smoothing — the mixed labels must be one-hot mixed with λ.
**Cross-domain wiring:** Mixup = interpolation in data space = convex combination = same as the lerp primitive in graphics.
**Notes:** CutMix cuts a patch from one image and pastes it into another, mixing spatial content rather than interpolating pixels.

### cutout (cross-domain alias: `random-erase`, `patch-remove`, `hole-fill`)
**Domain:** ML / Training
**Definition:** Randomly erase a rectangular region of the input image (set to zero or random values). Forces the model to not rely on any single region.
**Atom or composite:** Composite: sample a random rectangle (x, y, w, h) → set image[x:x+w, y:y+h] = 0/random.
**Cost model:** Negligible.
**Real wall?** No.
**Cross-domain wiring:** Cutout = dropout in the spatial domain. In retrieval: hiding parts of the query = similar augmentation.
**Notes:** Cutout is particularly effective for object detection (where dropout is hard to apply) because it doesn't require bounding boxes.

### augment-compose (cross-domain alias: `RandAugment`, `augmentation-pipeline`, `stochastic-transform`)
**Domain:** ML / Training
**Definition:** Apply a sequence of random augmentations: translate, rotate, flip, color jitter, cutout, etc. RandAugment: randomly select N augmentations, each with magnitude M.
**Atom or composite:** Composite: for each augmentation in pipeline: sample parameter → apply transform → return augmented sample.
**Cost model:** The augmentation pipeline can be the dominant cost in data loading for image models. GPU augmentation (DALI) is used to overlap augmentation with computation.
**Real wall?** No.
**Cross-domain wiring:** Augmentation = data augmentation = regularization via noise. In retrieval: query augmentation = expanding the query with related terms.
**Notes:** TrivialAugment (Müller et al., 2021) applies one random augmentation per sample — simpler than RandAugment but equally effective.

---

## Model Compression Atoms

### quantize-post (cross-domain alias: `PTQ`, `dynamic-quantize`, `int8-inference`)
**Domain:** ML / Training
**Definition:** Quantize weights to lower precision after training. Symmetric quantization: round(x / scale) where scale = max(|x|) / (2^{bits−1}−1). Asymmetric: round(x/scale) + zero_point.
**Atom or composite:** Composite: compute scale → quantize weights → store quantized weights + scale.
**Cost model:** Negligible. Weight-only quantization (for GEMM) can be done in a single pass.
**Real wall?** Yes — quantization degrades accuracy if the weight distribution is not well-matched to the quantization grid. Per-channel quantization helps.
**Cross-domain wiring:** Post-training quantization = rounding to a grid = the same as the quantization primitive in signal processing and information theory.
**Notes:** GPTQ and AWQ (activation-aware weight quantization) are quantization methods for large language models that achieve near-FP16 accuracy with INT4 weights.

### quantize-aware (cross-domain alias: `QAT`, `fake-quantize`, `straight-through-estimator`)
**Domain:** ML / Training
**Definition:** During training, simulate quantization effects with fake quantization (straight-through estimator passes gradients through the rounding). This trains the model to be robust to quantization.
**Atom or composite:** Composite: forward: fake_quant(x) → backward: STE gradient (pass through rounding) → update weights → repeat.
**Cost model:** Slightly more expensive than FP training (fake quant is cheap).
**Real wall?** No. But QAT requires careful tuning of the learning rate and quantization parameters.
**Cross-domain wiring:** QAT = training with rounding noise = same as dithering in signal processing.
**Notes:** The straight-through estimator (STE): backward pass ignores the rounding and computes gradients as if the quantizer were the identity function. This is correct when the rounding is a small perturbation.

### prune-weight (cross-domain alias: `magnitude-prune`, `sparse-train`, `weight-zero`)
**Domain:** ML / Training
**Definition:** Set weights below a threshold to zero. Magnitude pruning: sort |w|, keep top-k, zero the rest. Iterative magnitude pruning: prune a fraction → retrain → repeat.
**Atom or composite:** Composite: compute |w| → threshold → set below-threshold to 0 → optionally mask gradients during backprop.
**Cost model:** One pass over weights. The sparsity pattern must be tracked (indices of non-zero weights).
**Real wall?** Yes — sparse models require sparse kernels for speedup. General sparse matmul is not much faster than dense on most hardware. Structured sparsity (N:M) helps.
**Cross-domain wiring:** Weight pruning = removing low-importance weights = same as feature selection in statistics.
**Notes:** The "Lottery Ticket Hypothesis" (Frankle & Carbin, 2019): dense, randomly-initialized networks contain sparse subnetworks ("winning tickets") that can train to similar accuracy with fewer parameters.

### distill-soft (cross-domain alias: `teacher-student`, `knowledge-distill`, `soft-targets`)
**Domain:** ML / Training
**Definition:** Train a small student model using the soft targets (probability distribution over classes) from a large teacher model, not just hard labels. Loss: L = α·CE(y, student) + (1−α)·KL(teacher_soft, student_soft)·T².
**Atom or composite:** Composite: forward pass with teacher (large, frozen) → get soft targets → forward pass with student → compute distillation loss.
**Cost model:** One extra forward pass per batch (the teacher). The teacher is typically frozen, so only its forward pass costs.
**Real wall?** No.
**Cross-domain wiring:** Knowledge distillation = model compression = compression of the teacher into the student. In information theory: distilling = compressing the teacher's knowledge distribution into the student's parameters.
**Notes:** The temperature T > 1 softens the teacher distribution, making it richer. The student learns not just the hard labels but the relative probabilities of incorrect classes.

---

*Last updated: 2026-06-21 (expanded with training infrastructure, stability, data augmentation, model compression)*
*Source doctrine: The Painted Fence — Jesse*
