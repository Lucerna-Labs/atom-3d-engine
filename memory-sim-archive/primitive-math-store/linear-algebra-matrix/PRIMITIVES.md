# Linear Algebra / Matrix — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## Core Vector Atoms

### dot (cross-domain alias: `inner-product`, `scalar-product`, `cosine-prep`)
**Domain:** Linear Algebra / Matrix
**Definition:** ⟨a,b⟩ = Σ a_i·b_i. Sum of element-wise products. The fundamental bilinear form.
**Atom or composite:** Atom
**Cost model:** O(n) multiplications + O(n) additions for vectors of length n. SIMD parallelism reduces this significantly in practice.
**Real wall?** No.
**Cross-domain wiring:** Matched filter in signal processing. Cosine similarity (after normalization). Correlation coefficient (after mean-centering). Matrix-vector multiply = sequence of dot products.
**Notes:** The dot product is the most frequently executed operation in all of numerical computing. Every matrix-vector multiply is a sequence of dot products.

### outer (cross-domain alias: `rank-1-form`, `tensor-product`, `matrix-construct`)
**Domain:** Linear Algebra / Matrix
**Definition:** Given vectors a ∈ ℝᵐ and b ∈ ℝⁿ, produce matrix A = a·bᵀ with elements A_ij = a_i·b_j. A rank-1 matrix.
**Atom or composite:** Composite: for i in [1..m]: for j in [1..n]: A[i,j] = a[i]·b[j].
**Cost model:** O(m·n) operations. Produces a dense m×n matrix from two vectors.
**Real wall?** No.
**Cross-domain wiring:** In signal: outer product of two signals creates a 2D response surface (the convolution matrix). In probability: outer product of marginals produces the joint under independence.
**Notes:** Any matrix of rank r can be expressed as a sum of r rank-1 outer products — this is the foundation of CUR and skeletal decompositions.

### project-vec (cross-domain alias: `component`, `decompose`, `subtract-component`)
**Domain:** Linear Algebra / Matrix
**Definition:** Given vectors u and v, compute the projection of u onto v: proj_v(u) = (⟨u,v⟩/⟨v,v⟩)·v. Also: the rejection: u − proj_v(u) = component perpendicular to v.
**Atom or composite:** Composite: dot(u,v) / dot(v,v) → scale v by the scalar ratio.
**Cost model:** Two dot products + one scale.
**Real wall?** No.
**Cross-domain wiring:** In signal: matched filter output IS the projection onto the signal subspace. In graphics: projecting a point onto a plane = rejection from the plane normal.
**Notes:** The Gram-Schmidt orthogonalization process is a sequence of project-vec operations: each new basis vector is the rejection of the raw vector from all previous basis vectors.

### normalize-vec (cross-domain alias: `unitize`, `L2-scale`, `whiten-vec`)
**Domain:** Linear Algebra / Matrix
**Definition:** Scale a vector to unit L2 norm: v_norm = v / ||v||_2. Often implemented via rsqrt (reciprocal square root approximation) for speed.
**Atom or composite:** Composite: compute ||v|| = sqrt(dot(v,v)) → scale each element by 1/||v||.
**Cost model:** One dot product + one sqrt + N divisions. rsqrt is a single hardware instruction on most GPUs.
**Real wall?** No. But vectors near zero (||v|| ≈ 0) cause division by near-zero — numerical instability.
**Cross-domain wiring:** L2 normalization in retrieval enables cosine similarity via dot product. In signal: power normalization. In statistics: z-score = (x − μ)/σ = normalize by L2-like statistic.
**Notes:** After normalization, dot(u_norm, v_norm) = cosine similarity. This is why normalized embeddings are the preferred representation in retrieval.

### scale-vec (cross-domain alias: `multiply-scalar`, `amplify`, `attenuate`)
**Domain:** Linear Algebra / Matrix
**Definition:** Multiply a vector by a scalar: v_scaled = α·v. Element-wise: v_scaled[i] = α·v[i].
**Atom or composite:** Atom
**Cost model:** N multiplications.
**Real wall?** No.
**Cross-domain wiring:** In signal: amplification/attenuation of signal amplitude. In graphics: color scaling/brightness. In retrieval: score scaling.
**Notes:** Scaling is free in terms of information — it only changes magnitude, not direction. After normalization, direction is all that remains.

---

## Decomposition Atoms

### svd (cross-domain alias: `factorize`, `spectral-decompose`, `diagonalize`)
**Domain:** Linear Algebra / Matrix
**Definition:** A = U·Σ·Vᵀ where U, V are orthonormal (UᵀU = I, VᵀV = I) and Σ is diagonal with singular values σ₁ ≥ σ₂ ≥ ... ≥ σ_r ≥ 0. Truncated SVD keeps top k singular values.
**Atom or composite:** Composite: iterative power method → extract largest singular vector/value → deflate → repeat. Full SVD: Golub-Reinsch algorithm O(n²m + m·n²).
**Cost model:** Full SVD is expensive: O(min(n²m, m²n)). Randomized SVD: O(n·k + k²·n) for rank k. The randomized version is the practical primitive.
**Real wall?** No.
**Cross-domain wiring:** SVD = Fourier transform for arbitrary matrices. The singular vectors are the optimal basis for low-rank approximation. Truncated SVD = PCA. In signal: SVD of covariance matrix = PCA = spectral analysis. In retrieval: LSA = truncated SVD of term-document matrix.
**Notes:** The Eckart-Young theorem: the best rank-k approximation of A (in Frobenius norm) is U_k·Σ_k·V_kᵀ. This is the mathematical foundation of LSA, word2vec SVD preprocessing, and all low-rank approximation methods.

### eigen-decompose (cross-domain alias: `spectrum`, `diagonalize-symmetric`, `modal-decompose`)
**Domain:** Linear Algebra / Matrix
**Definition:** For symmetric matrix A: A = Q·Λ·Qᵀ where Q's columns are orthonormal eigenvectors and Λ is diagonal with eigenvalues λ₁ ≥ ... ≥ λ_n.
**Atom or composite:** Composite: same as SVD for symmetric matrices. For symmetric A, eigenvalues = singular values, eigenvectors are related.
**Cost model:** O(n³) for dense symmetric matrices. Sparse Lanczos method: O(n·k) for k eigenvalues.
**Real wall?** No.
**Cross-domain wiring:** PageRank = dominant eigenvector of the transition matrix. Spectral graph theory: eigenvalues of the Laplacian give graph structural information. In physics: normal modes = eigenvectors of the system matrix.
**Notes:** For non-symmetric matrices, SVD (which gives left and right singular vectors) is the stable alternative. The eigenvalues of non-symmetric matrices may be complex.

### qr-decompose (cross-domain alias: `orthogonalize`, `Gram-Schmidt`, `QR-factor`)
**Domain:** Linear Algebra / Matrix
**Definition:** A = Q·R where Q is orthonormal (columns) and R is upper triangular. The standard algorithm: Gram-Schmidt, Modified Gram-Schmidt (MGS), or Householder reflections.
**Atom or composite:** Composite: Modified Gram-Schmidt: for j in columns: v_j = a_j − Σ_{i<j} ⟨a_j, q_i⟩·q_i → normalize.
**Cost model:** O(m·n²) for m×n matrix.
**Real wall?** No.
**Cross-domain wiring:** QR decomposition solves least squares: R·x = Qᵀ·b (back-substitution in R). In signal: QR is used in adaptive filtering (QR-RLS algorithm). In graphics: QR gives the orthonormal basis of a set of directions.
**Notes:** MGS is numerically stable; classical Gram-Schmidt is not. Householder QR is the standard for dense matrices (LAPACK).

### cholesky-decompose (cross-domain alias: `factorize-positive-definite`, `square-root`, `LDLᵀ`)
**Domain:** Linear Algebra / Matrix
**Definition:** For symmetric positive definite (SPD) matrix A: A = L·Lᵀ where L is lower triangular with positive diagonal. Equivalent to A = LDLᵀ where D is diagonal.
**Atom or composite:** Composite: for k in [1..n]: L[k,k] = sqrt(A[k,k] − Σ_{j<k} L[k,j]²) → for i in [k+1..n]: L[i,k] = (A[i,k] − Σ_{j<k} L[i,j]·L[k,j]) / L[k,k].
**Cost model:** O(n³/6) — half the cost of LU decomposition.
**Real wall?** No. But A must be SPD; if not, Cholesky fails or produces complex numbers.
**Cross-domain wiring:** Cholesky decomposition is the basis of Gaussian process inference (kernel matrix inversion). In random number generation: Cholesky of covariance matrix produces correlated Gaussians from uncorrelated ones.
**Notes:** The inverse of a SPD matrix via Cholesky is faster and more stable than general inverse. This is why Gaussian processes use Cholesky of the kernel matrix.

---

## Randomized Linear Algebra Atoms

### random-project (cross-domain alias: `JL-transform`, `dimension-reduce`, `hash-to-dim`)
**Domain:** Linear Algebra / Matrix
**Definition:** Project N-dimensional vectors to d << N dimensions using a random matrix R ∈ ℝ^{d×N}. Preserves distances up to (1±ε) with high probability if d = O(log N / ε²).
**Atom or composite:** Composite: v_reduced = R·v. The random matrix R can be: Gaussian entries, ±1 entries (sign matrix), or sparse entries (Achlioptas/Li-Sparse JL).
**Cost model:** O(d·N) for the matrix multiply. But R is structured (sparse, fast JL) to reduce the cost.
**Real wall?** Yes — the Johnson-Lindenstrauss lemma guarantees projection but requires d = O(log N / ε²). For very high N, this can still be large. The lemma is a real wall with a concrete cost.
**Cross-domain wiring:** In signal: spread spectrum = random projection onto spreading codes. In cryptography: random projection for secure computation. In retrieval: random projection = the first rung of the project reinforcement ladder in SCG.
**Notes:** The Li-Sparse JL matrix (with ±1, 0 entries in proportion 1/6, 2/3, 1/6) achieves the same guarantee with 3× fewer non-zero entries → faster multiply.

### randomized-svd (cross-domain alias: `fast-svd`, `sketched-svd`, `Nyström`)
**Domain:** Linear Algebra / Matrix
**Definition:** Compute a rank-k SVD using random projections to reduce the problem size: sketch A → power iteration to improve accuracy → compute SVD of the sketch → project back.
**Atom or composite:** Composite: A_sampled = R·A → power_iterate(A_sampled) → compute SVD of A_sampled → project back to get U_k, Σ_k, V_k.
**Cost model:** O(N·k + k²·N) — linear in the matrix dimension for fixed rank k.
**Real wall?** No. But the error is probabilistic — there is a small chance the approximation is bad. Power iteration reduces this probability.
**Cross-domain wiring:** Nyström method for kernel matrices = randomized SVD on a subsampled kernel. In retrieval: the randomized SVD is the practical way to compute LSA without full matrix decomposition.
**Notes:** The Halko-Tropp-Tyrtyshnik (HMT) randomized SVD is the standard algorithm. It achieves O(N·k) time for rank-k approximation of an N×N matrix, vs O(N³) for full SVD.

---

## Matrix Multiplication Atoms

### matmul (cross-domain alias: `multiply`, `gemm`, `dot-product-blocks`)
**Domain:** Linear Algebra / Matrix
**Definition:** C = A·B where A ∈ ℝ^{m×k}, B ∈ ℝ^{k×n}, C ∈ ℝ^{m×n}. C_ij = Σ_k A_ik·B_kj.
**Atom or composite:** Composite: three nested loops over i,j,k. Optimized by blocking (cache efficiency), SIMD (instruction parallelism), and multithreading.
**Cost model:** O(m·n·k). Strassen reduces the constant: O(n^{2.807}) for square matrices (theoretical). Coppersmith-Winograd: O(n^{2.373}) but the constant is too large to be practical.
**Real wall?** Yes — matrix multiplication is the computational core of deep learning, scientific computing, and graph algorithms. The cost is fundamental — improving it has been a central goal of computing since Strassen (1969).
**Cross-domain wiring:** Every linear transform is matmul. Neural network forward pass = sequence of matmuls. Convolution = im2col + matmul. Graph convolution = adjacency matrix · feature matrix.
**Notes:** The "algebra of matmul" — associativity (A(BC) = (AB)C), distributivity — is what makes chaining transforms possible. The lack of commutativity (AB ≠ BA in general) is why order matters.

### hadamard (cross-domain alias: `element-wise-multiply`, `Schur-product`, `pointwise-multiply`)
**Domain:** Linear Algebra / Matrix
**Definition:** Element-wise multiplication of two matrices of the same shape: C_ij = A_ij·B_ij. No summation.
**Atom or composite:** Atom
**Cost model:** O(m·n) multiplications. Very cheap compared to matmul.
**Real wall?** No.
**Cross-domain wiring:** In signal: element-wise multiply in the frequency domain = convolution in time domain. In graphics: blend operation. In signal: modulation.
**Notes:** Hadamard is the tensor product (⊗) for diagonal matrices but for general matrices it is element-wise. Schur product theorem: the Hadamard product of two positive semi-definite matrices is also positive semi-definite.

### kronecker (cross-domain alias: `tensor-product`, `block-expand`, `super-operators`)
**Domain:** Linear Algebra / Matrix
**Definition:** Given A ∈ ℝ^{m×n} and B ∈ ℝ^{p×q}, A ⊗ B is an (m·p)×(n·q) block matrix where each block is A_ij·B.
**Atom or composite:** Composite: for i in [1..m], j in [1..n]: block(i,j) = A_ij·B.
**Cost model:** O(m·n·p·q) — grows with the product of dimensions.
**Real wall?** No. But storage grows as product of dimensions — Kronecker products of large matrices are infeasible to materialize.
**Cross-domain wiring:** In signal processing: Kronecker product of signals = tensor product = creates a higher-dimensional signal space. In quantum computing: tensor product of qubits = Kronecker product.
**Notes:** The Kronecker product is the natural way to express operations on multi-dimensional data (tensors). It is the foundation of tensor decomposition methods (Tucker, CP decomposition).

---

## Special Matrices and Operations

### sparse-dot (cross-domain alias: `CSR-multiply`, `efficient-multiply`)
**Domain:** Linear Algebra / Matrix
**Definition:** Matrix-vector multiply where the matrix is stored in sparse format (CSR/CSC). Only store non-zero elements and their positions.
**Atom or composite:** Composite: for each row: scan the column indices → accumulate dot product of row's non-zeros with the vector.
**Cost model:** O(nnz) where nnz = number of non-zeros. vs O(m·n) for dense.
**Real wall?** No.
**Cross-domain wiring:** Sparse matmul underlies graph convolution (adjacency matrix · feature matrix). In retrieval: sparse embedding = dot product over only non-zero dimensions.
**Notes:** The CSR (Compressed Sparse Row) format stores: values[], col_indices[], row_ptr[]. This makes row access O(1) and column access O(nnz_per_row).

### trace (cross-domain alias: `sum-diagonal`, `rotation-invariant`)
**Domain:** Linear Algebra / Matrix
**Definition:** Tr(A) = Σ_i A_ii. The sum of diagonal elements.
**Atom or composite:** Atom
**Cost model:** O(n) for n×n matrix.
**Real wall?** No.
**Cross-domain wiring:** Trace(A) = Σ singular_values(A)² = sum of squared singular values = Frobenius norm squared. In physics: trace of the density matrix = 1. In statistics: trace of the hat matrix = degrees of freedom.
**Notes:** Trace is invariant under cyclic permutations: Tr(ABC) = Tr(BCA) = Tr(CAB). This cyclic invariance is useful for rewriting matrix expressions.

### frobenius-norm (cross-domain alias: `matrix-L2`, `F-norm`, `RMS-matrix`)
**Domain:** Linear Algebra / Matrix
**Definition:** ||A||_F = sqrt(Σ_ij A_ij²) = sqrt(Tr(AᵀA)) = sqrt(Σ_k σ_k²). The Euclidean norm of the flattened matrix.
**Atom or composite:** Composite: compute Aᵀ·A (or Σ_k σ_k² directly if SVD is available).
**Cost model:** O(m·n) to compute directly.
**Real wall?** No.
**Cross-domain wiring:** Frobenius norm is the natural matrix distance metric (minimized by PCA). ||A − B||_F = sqrt(Σ_ij (A_ij − B_ij)²). In retrieval: the difference between two term-document matrices = Frobenius norm distance.
**Notes:** The Frobenius norm is the ℓ₂ norm of the vectorized matrix. It is the natural choice for matrix approximation quality because Tr(AᵀA) = Σ σ² = ||A||_F².

### spectral-norm (cross-domain alias: `operator-norm`, `largest-sv`, `matrix-gain`)
**Domain:** Linear Algebra / Matrix
**Definition:** ||A||₂ = σ_max(A) (the largest singular value). The maximum factor by which A stretches a vector: ||Ax||_2 / ||x||_2.
**Atom or composite:** Composite: compute largest singular value via power iteration on A·Aᵀ or Aᵀ·A.
**Cost model:** O(n·iterations) via power iteration.
**Real wall?** No.
**Cross-domain wiring:** Condition number κ(A) = σ_max / σ_min = ||A||₂ · ||A⁻¹||₂. Large κ → numerical instability. In physics: the condition number determines the amplification of measurement errors.
**Notes:** The condition number governs the sensitivity of linear solves to input perturbations. κ(A) >> 1 → ill-conditioned → use QR or SVD-based solve instead of direct inversion.

### pseudoinverse (cross-domain alias: `Moore-Penrose-inverse`, `least-squares`, `best-approximation`)
**Domain:** Linear Algebra / Matrix
**Definition:** A⁺ = V·Σ⁺·Uᵀ where Σ⁺ = diag(1/σ₁,...,1/σ_r,0,...,0). Gives the minimum-norm least-squares solution to Ax = b.
**Atom or composite:** Composite: compute truncated SVD → invert non-zero singular values → reconstruct.
**Cost model:** Same as randomized SVD for large matrices.
**Real wall?** No. But very small singular values (near zero) → very large inverse singular values → numerical instability. Regularized pseudo-inverse (Tikhonov) adds λ to σ before inversion.
**Cross-domain wiring:** Ridge regression = (AᵀA + λI)⁻¹·Aᵀ = pseudoinverse with regularization. In retrieval: pseudo-inverse of the term-document matrix = latent semantic analysis (LSA).
**Notes:** For full rank m×n with m ≥ n: A⁺·A = I_n. For m < n: A·A⁺ is the projection onto the column space of A.

---

## Low-Rank Approximation Atoms

### CUR-decompose (cross-domain alias: `skeletal-decomposition`, `column-row-decomposition`)
**Domain:** Linear Algebra / Matrix
**Definition:** A ≈ C·U·R where C is a matrix of sampled columns, R is a matrix of sampled rows, and U is the intersection matrix at sampled columns and rows.
**Atom or composite:** Composite: sample columns (proportional to leverage scores) → sample rows → form C, R → U = C⁺·A·R⁺.
**Cost model:** O(N·k) for column/row sampling + O(k³) for U computation.
**Real wall?** No.
**Cross-domain wiring:** CUR vs SVD: CUR uses actual columns and rows (interpretable) vs SVD's abstract singular vectors (not interpretable). In retrieval: selecting actual documents and terms vs selecting latent dimensions.
**Notes:** The key advantage of CUR over SVD: the decomposition uses actual rows and columns of the original matrix, making it interpretable. SVD's U and V are abstract basis vectors.

### nmf (cross-domain alias: `non-negative-factorize`, `parts-decompose`, `topic-decompose`)
**Domain:** Linear Algebra / Matrix
**Definition:** A ≈ W·H where W ∈ ℝ^{m×k}_+ and H ∈ ℝ^{k×n}_+ are both non-negative. Solved via multiplicative updates or alternating least squares.
**Atom or composite:** Composite: initialize W, H non-negative → repeat: H ∝ H ⊙ (Wᵀ·A) / (Wᵀ·W·H) → W ∝ W ⊙ (A·Hᵀ) / (W·H·Hᵀ).
**Cost model:** O(n·k) per iteration. Requires many iterations to converge.
**Real wall?** No.
**Cross-domain wiring:** NMF on term-document matrix = topic modeling (Latent Dirichlet Allocation uses a related probabilistic model). In physics: non-negative matrix factorization = parts-based decomposition (each part is additive, not subtractive).
**Notes:** NMF gives a parts-based representation because negative factors are prohibited — parts add together to reconstruct, they don't cancel. SVD allows cancellation (subtractive reconstruction).

---

## Summary: Linear Algebra Atom → Cross-Domain Wiring

| LA Primitive | Signal Alias | Retrieval Alias | Graphics Alias |
|---|---|---|---|
| dot | matched filter | cosine similarity | N·L (lighting) |
| outer | correlation matrix | co-occurrence matrix | tensor product |
| project-vec | signal projection | feature projection | point-on-plane |
| normalize-vec | unit-normalize signal | L2-normalize embedding | normalize direction |
| svd | Fourier transform | LSA, PCA | eigendecomposition |
| eigen-decompose | spectral analysis | PageRank | normal modes |
| qr-decompose | orthogonal filtering | Gram-Schmidt orthogonalize | orthonormal basis |
| random-project | JL projection | random embed | dimension reduce |
| matmul | all linear transforms | matrix retrieval | affine transform |
| hadamard | pointwise multiply | element-wise combine | blend |
| sparse-dot | sparse filter | sparse retrieval | sparse render |
| pseudoinverse | Wiener filter | LSA inverse | least-squares solve |

---

*Last updated: 2026-06-21*
*Source doctrine: The Painted Fence — Jesse*


---

## Tensor Operations Atoms

### tensor-contract (cross-domain alias: `einsum`, `mode-n-product`, `n-mode-product`)
**Domain:** Linear Algebra / Matrix
**Definition:** Generalize matrix multiplication to tensors. n-mode product: (X ×_n A)[i₁,...,i_{n−1},j,i_{n+1},...] = Σ_j A_{j,i_n}·X[i₁,...,j,...,i_N].
**Atom or composite:** Composite: einsum("ij,jk->ik", X, A) executes tensor contraction with specified index contraction pattern.
**Cost model:** O(∏ dim_i) for contraction along matching indices. Mode-n product costs O(dim_n·∏ dim_i).
**Real wall?** Yes — tensor contraction scales exponentially in the number of modes. This is the "curse of dimensionality" in tensor methods.
**Cross-domain wiring:** Neural network layers = tensor contractions (Conv2D = batch × h × w × c contracted with kernel). In physics: any multi-index quantity (stress tensor, Riemann tensor) is a tensor.
**Notes:** einsum (Einstein summation) is the most general and efficient tensor operation notation. Most tensor libraries implement it as the core primitive.

### tensor-decompose-tucker (cross-domain alias: `HOSVD`, `Tucker-factor`, `core-compress`)
**Domain:** Linear Algebra / Matrix
**Definition:** X ≈ G ×₁ A¹ ×₂ A² ×₃ ... ×_N A^N where G is the core tensor and A^n are factor matrices. HOSVD computes them via SVD of each mode's unfolding.
**Atom or composite:** Composite: for each mode n: unfold(X, n) → SVD → take top-k singular vectors → mode-n factor A^n. Core = contracted original with inverse factors.
**Cost model:** O(N·(∏ d_i)·Σ d_i) for an N-mode tensor. Expensive but structured.
**Real wall?** Yes — Tucker rank selection is a hyperparameter. Too small = approximation error; too large = no compression.
**Cross-domain wiring:** Tucker decomposition = PCA for tensors. In physics: the Tucker rank = the number of retained modes in each dimension. In ML: tensor factorization for recommendation systems.
**Notes:** The core tensor G captures the "interaction" between the factors. A small core × large factors ≠ large core × small factors.

### tensor-decompose-cp (cross-domain alias: `PARAFAC`, `CANDECOMP`, `rank-R-factor`)
**Domain:** Linear Algebra / Matrix
**Definition:** X ≈ Σ_{r=1}^R λ_r · u¹_r ⊗ u²_r ⊗ ... ⊗ u^N_r. R rank-one tensors. Computed via alternating least squares (ALS).
**Atom or composite:** Composite: initialize factors → repeat: fix all but one factor, solve least squares for it → rotate through all factors → until convergence.
**Cost model:** O(R·N·∏ d_i) per ALS iteration. Fast for sparse tensors.
**Real wall?** Yes — the CP rank R must be known or estimated. Finding the exact rank of a tensor is NP-hard.
**Cross-domain wiring:** CP = the probabilistic latent semantic analysis (PLSA) model. In physics: quantum state decomposition = CP decomposition in the quantum state space.
**Notes:** CP-ALS is the workhorse algorithm for tensor decomposition. The tensor train (TT) format is an alternative that stores factors sequentially rather than all at once.

### tensor-train (cross-domain alias: `TT-format`, `matrix-product-state`, `sequential-factor`)
**Domain:** Linear Algebra / Matrix
**Definition:** A tensor decomposed as a chain of matrices: X(i₁,...,i_N) = G¹_{i₁} · G²_{i₂} · ... · G^N_{i_N}. Each G^k is an r_{k-1} × r_k matrix for modes k=1...N.
**Atom or composite:** Composite: initialize ranks → TT-SVD (fold-unfold sequentially) → extract cores.
**Cost model:** O(N·d²·r²) where d = max dimension, r = TT rank. Much smaller than dense storage for many tensors.
**Real wall?** Yes — the TT ranks determine the storage. If ranks must be large for accuracy, the compression advantage disappears.
**Cross-domain wiring:** Tensor train = matrix product state (MPS) in quantum physics = same mathematical structure. In ML: tensor train networks are the basis of tensor ring networks.
**Notes:** TT-SVD is the stable way to compute a TT decomposition: sequential SVD of unfoldings, truncating ranks.

---

## Solver Atoms

### lu-decompose (cross-domain alias: `factor-into-triangular`, `forward-back-sub`, `pivoting`)
**Domain:** Linear Algebra / Matrix
**Definition:** A = L·U where L is lower triangular and U is upper triangular. With partial pivoting: PA = LU where P is a permutation matrix.
**Atom or composite:** Composite: for k in [1..n]: find pivot row → swap → compute multipliers → eliminate below.
**Cost model:** O(n³/3) without pivoting. Partial pivoting adds O(n²) overhead.
**Real wall?** No. But LU without pivoting fails for singular matrices. Partial pivoting is standard and adds negligible cost.
**Cross-domain wiring:** LU = Gaussian elimination = the standard algorithm for solving Ax = b. In signal: LU factorization of the covariance matrix is the basis of linear prediction (LPC).
**Notes:** Cholesky (for SPD matrices) is LU with L = Uᵀ — no pivoting needed and half the cost.

### solve-linear (cross-domain alias: `x=A\\b`, `direct-solve`, `forward-back`)
**Domain:** Linear Algebra / Matrix
**Definition:** Solve Ax = b for x. Use LU or Cholesky factorization: forward substitution (Ly = b) then back substitution (Ux = y).
**Atom or composite:** Composite: factor A (LU or Cholesky) → forward-sub(b) → back-sub(result).
**Cost model:** O(n³) for factorization + O(n²) for the two solves. Factorization dominates.
**Real wall?** Yes — the condition number κ(A) determines numerical stability. Large κ → amplified errors. Preconditioning is needed for ill-conditioned systems.
**Cross-domain wiring:** Linear solve = inverting a linear system = finding the fixed point of a linear operator. In graphics: solving lighting equations = linear solves.
**Notes:** For sparse A, sparse Cholesky is used — fill-in (non-zeros appearing in the factors) must be managed via ordering (minimum degree, nested dissection).

### conjugate-gradient (cross-domain alias: `CG`, `iterative-solve`, `Krylov-solve`)
**Domain:** Linear Algebra / Matrix
**Definition:** For symmetric positive definite A: iteratively refine the solution. x_{k+1} = x_k + α_k·p_k where α_k = r_kᵀr_k / p_kᵀAp_k and p_{k+1} = r_{k+1} + β_k·p_k.
**Atom or composite:** Composite: initialize r = b − Ax, p = r → repeat: α = (rᵀr)/(pᵀAp) → x += α·p → r_new = r − α·Ap → β = (r_newᵀr_new)/(rᵀr) → p = r_new + β·p.
**Cost model:** O(n²) per iteration for dense A (matrix-vector multiply dominates). Converges in at most n iterations for exact arithmetic.
**Real wall?** Yes — the convergence rate depends on the condition number: converges faster for well-conditioned A. Preconditioning is essential for bad κ(A).
**Cross-domain wiring:** CG = the optimal iterative method for SPD systems. In optimization: CG = the nonlinear conjugate gradient method applied to the quadratic function xᵀAx − 2bᵀx. In physics: CG solves the heat equation iteratively.
**Notes:** Preconditioning (left-multiplying by M⁻¹) accelerates convergence: M⁻¹Ax = M⁻¹b. Common preconditioners: Jacobi, incomplete Cholesky, AMG.

### gmres (cross-domain alias: `generalized-minres`, `Krylov-GMRES`, `non-symmetric-solve`)
**Domain:** Linear Algebra / Matrix
**Definition:** Generalized Minimum Residual — iterative solver for non-symmetric A. Builds a Krylov subspace from which the minimum residual solution is selected.
**Atom or composite:** Composite: build Krylov basis (Arnoldi process) → at each step: solve least squares over Krylov basis for minimum residual.
**Cost model:** O(n²·k) for k restarts. GMRES(k) restarts periodically to limit memory.
**Real wall?** Yes — GMRES can stagnate or converge very slowly for systems with a poor condition number or irregular spectra. Flexible GMRES (FGMRES) allows variable preconditioning.
**Cross-domain wiring:** GMRES = the non-symmetric equivalent of CG. In physics: any non-symmetric linear system (advection-diffusion) is solved with GMRES.
**Notes:** BiCGSTAB is an alternative that requires less memory (two matrix-vector multiplies per iteration vs Arnoldi's one × basis dimension). For very ill-conditioned systems, IDR(s) (Induced Dimension Reduction) often outperforms both.

---

## Eigenvalue / SVD Variants Atoms

### power-iterate (cross-domain alias: `dominant-eigen`, `Rayleigh-quotient-iterate`, `eigen-power`)
**Domain:** Linear Algebra / Matrix
**Definition:** Repeatedly multiply a random vector by A and normalize: v_{k+1} = A·v_k / ||A·v_k||. Converges to the dominant eigenvector.
**Atom or composite:** Composite: v = random → repeat: v = A·v → v = v/||v|| → until convergence.
**Cost model:** O(n²·iterations) for dense A. Each iteration = one matrix-vector multiply.
**Real wall?** Yes — convergence rate = |λ₁/λ₂| (gap to second eigenvalue). Small gap → very slow convergence. Shifted power iteration accelerates by shifting the spectrum.
**Cross-domain wiring:** Power iteration = random walk in a graph = the algorithm behind PageRank (with damping). In physics: power iteration = finding the ground state energy.
**Notes:** Shifted power iteration (SPI): v_{k+1} = (A−σI)⁻¹·v_k → converges to eigenvector of closest eigenvalue to σ. Rayleigh quotient iteration refines σ at each step.

### lanczos-iterate (cross-domain alias: `tridiagonalize`, `Krylov-eigensolver`, `Lanczos-decomposition`)
**Domain:** Linear Algebra / Matrix
**Definition:** Build a tridiagonal matrix T = Qᵀ·A·Q where Q contains orthonormal Lanczos vectors. The eigenvalues of T approximate the extreme eigenvalues of A. For sparse A, this is the standard eigensolver.
**Atom or composite:** Composite: q₀ = random → β₀ = 0 → for j = 1..m: r_j = A·q_{j−1} − β_{j−1}·q_{j−2} → α_j = q_{j−1}ᵀr_j → r_j -= α_j·q_{j−1} + β_{j−1}·q_{j−2} → β_j = ||r_j|| → q_j = r_j/β_j.
**Cost model:** O(n·m·iterations) for m Lanczos steps. Much cheaper than full eigendecomposition for sparse matrices.
**Real wall?** Yes — Lanczos is sensitive to loss of orthogonality among Lanczos vectors (round-off errors). Reorthogonalization (full or selective) is needed for accuracy.
**Cross-domain wiring:** Lanczos = the tridiagonalization of the adjacency matrix in graph spectral analysis. In physics: Lanczos is the method of choice for large quantum systems.
**Notes:** The tridiagonal T has eigenvalues that interlace the eigenvalues of A. After m Lanczos steps, m Ritz values approximate the m largest (or smallest) eigenvalues.

### arnoldi-iterate (cross-domain alias: `non-symmetric-Krylov`, `Hessenberg-reduce`, `generalized-eigen`)
**Domain:** Linear Algebra / Matrix
**Definition:** The non-symmetric equivalent of Lanczos. Builds a Hessenberg matrix H = Qᵀ·A·Q. Used with GMRES for non-symmetric eigensolvers and for solving non-symmetric systems.
**Atom or composite:** Composite: similar to Lanczos but with non-symmetric Gram-Schmidt orthogonalization.
**Cost model:** O(n²·m) for m Arnoldi steps. Gram-Schmidt orthogonalization costs dominate.
**Real wall?** No. But the Arnoldi process is more expensive than Lanczos due to the need for full reorthogonalization.
**Cross-domain wiring:** Arnoldi = the foundation of implicitly restarted Arnoldi methods (IRA, IRAM) for non-symmetric eigenvalue problems.
**Notes:** The implicitly restarted Arnoldi method (IRAM) is the standard algorithm for computing a few eigenvalues of large non-symmetric matrices (e.g., in fluid dynamics).

---

## Matrix Functions / Special Structures Atoms

### matrix-exp (cross-domain alias: `Frechet-derivative`, `exponential-map`, `dynamical-system`)
**Domain:** Linear Algebra / Matrix
**Definition:** e^A = Σ_{k=0}^∞ A^k/k!. For sparse A, use Krylov subspace approximation: e^{tA}b ≈ β·V·e^{tT}·e₁ where V, T are from the Lanczos process.
**Atom or composite:** Composite: Lanczos on (A, b) → compute e^{tT} → reconstruct.
**Cost model:** O(n²·m) for m-step Krylov approximation. The dominant cost is computing e^{tT} (small m×m matrix).
**Real wall?** Yes — the convergence of Krylov approximation depends on the spectral radius. For large t, the approximation can be poor.
**Cross-domain wiring:** Matrix exponential = solution of the ODE system ẋ = A·x. In physics: the time evolution operator U = e^{−iHt/ℏ} = matrix exponential of the Hamiltonian.
**Notes:** For very large A, the action of e^{A}b (without forming e^{A}) is all that is needed — the Krylov approximation is the standard approach.

### matrix-log (cross-domain alias: `inverse-exponential`, `log-det`, `principal-log`)
**Domain:** Linear Algebra / Matrix
**Definition:** Given a PD matrix M, compute log(M) via eigendecomposition: log(M) = V·diag(log(λ_i))·Vᵀ.
**Atom or composite:** Composite: eigendecompose M → take log of eigenvalues → reconstruct.
**Cost model:** O(n³) for eigendecomposition. Same as log-det = Σ log(λ_i).
**Real wall?** No. But M must be PD and the principal logarithm (with eigenvalues in (−π, π)) must be selected.
**Cross-domain wiring:** Matrix log = integral of M⁻¹·dM = the continuous version of multiplicative updates. In information theory: log-det divergence = the natural metric for positive definite matrices.
**Notes:** The log-det divergence D(M||N) = Tr(M·N⁻¹) − log(Tr(M·N⁻¹)) − n is the natural distance between PD matrices.

### hadamard-product (cross-domain alias: `elementwise-mult`, `Schur-product`, `pointwise-apply`)
**Domain:** Linear Algebra / Matrix
**Definition:** Element-wise multiplication A ∘ B where (A ∘ B)_{ij} = A_{ij}·B_{ij}.
**Atom or composite:** Atom
**Cost model:** O(n·m) element-wise multiplications.
**Real wall?** No.
**Cross-domain wiring:** In signal processing: hadamard = element-wise multiply in frequency domain = convolution in time domain. In neural networks: the Hadamard product is used in gates (LSTM input/forget gates multiply by the same element-wise value).
**Notes:** Schur product theorem: if A and B are PSD, then A ∘ B is also PSD. This is not true for general matrices.

---

## Low-Rank Approximation Variants Atoms

### random-svd-alt (cross-domain alias: `matrix-sketch`, `row-sampling-SVD`, `leverage-skeleton`)
**Domain:** Linear Algebra / Matrix
**Definition:** Sample rows/columns proportionally to their leverage scores (the diagonal of the projection matrix). The sampled rows/columns form a "sketch" that captures the row space.
**Atom or composite:** Composite: compute leverage scores of A (diagonal of A·pinv(A)) → sample rows proportionally → SVD of sampled matrix → project.
**Cost model:** O(n·k) to compute leverage scores (via a QR decomposition). Sampling is O(n·k) for k samples.
**Real wall?** Yes — leverage scores measure how well each row contributes to the row space. Rows with low leverage can be sampled more aggressively.
**Cross-domain wiring:** Row sampling = importance sampling in statistics. The leverage score = the "importance" of each row in the spectral decomposition.
**Notes:** The matrix sketch of (S Liberty, 2007) is a single-pass row sampling algorithm that achieves relative-error low-rank approximation.

### nystrom-approx (cross-domain alias: `kernel-approximate`, `column-sampling`, `landmark-approx`)
**Domain:** Linear Algebra / Matrix
**Definition:** For PSD kernel matrix K, sample columns to form K̃. K ≈ K̃ · K̃̃⁻¹ · K̃ᵀ where K̃̃ is the intersection of sampled columns. Nyström extension uses this to approximate out-of-sample points.
**Atom or composite:** Composite: sample k columns → form K̃ → compute K̃̃⁻¹ → Nyström extension = K_new,new ≈ K_new,samples · K̃̃⁻¹ · K_s,samplesᵀ.
**Cost model:** O(n·k²) for k samples. Dominated by computing the inverse of the k×k matrix K̃̃.
**Real wall?** Yes — the approximation quality depends on the column sampling. Uniform sampling works poorly if columns are heterogeneous.
**Cross-domain wiring:** Nyström = kernel approximation = the same as the Nyström method for solving integral equations. In ML: Nyström method for Gaussian process regression.
**Notes:** The Nyström approximation is equivalent to a specific form of the column sampling SVD — it's a structured low-rank approximation.

---

## Matrix Reordering Atoms

### cuthill-mckee (cross-domain alias: `bandwidth-reduce`, `reverse-Cuthill-McKee`, `profile-minimize`)
**Domain:** Linear Algebra / Matrix
**Definition:** Reorder rows and columns to reduce the bandwidth of a sparse matrix. Start from a peripheral vertex and BFS outward, numbering nodes in order. Reverse Cuthill-McKee (RCM) produces a symmetric permutation with minimum profile.
**Atom or composite:** Composite: find pseudo-peripheral vertex → BFS from that vertex → order vertices by level → reverse the ordering.
**Cost model:** O(E) for the BFS.
**Real wall?** No.
**Cross-domain wiring:** Cuthill-McKee = graph bandwidth minimization = reordering for cache-efficient matrix-vector multiply. In databases: the same as reordering rows for sequential access patterns.
**Notes:** RCM reduces the bandwidth and profile of a sparse matrix, making sparse Cholesky faster and more cache-friendly.

### minimum-degree (cross-domain alias: `MD-ordering`, `fill-in-reduce`, `approximate-minimum-degree`)
**Domain:** Linear Algebra / Matrix
**Definition:** Order variables to minimize fill-in during Gaussian elimination. At each step, eliminate the variable with the minimum degree (fewest non-zeros in its row/column).
**Atom or composite:** Composite: while vars remain: compute degree of each var → eliminate var with minimum degree → update adjacency.
**Cost model:** O(n²·nz) naive, O(n log n) with heaps. AMD (approximate minimum degree) gives nearly optimal orderings much faster.
**Real wall?** No.
**Cross-domain wiring:** Minimum degree = the elimination ordering that minimizes the fill-in graph. In graphs: it's equivalent to finding a good graph coloring or treewidth approximation.
**Notes:** AMD (Davis et al.) is the standard ordering for sparse Cholesky in most sparse matrix libraries (CHOLMOD, SuiteSparse).

---

*Last updated: 2026-06-21 (expanded with tensor ops, solvers, eigenvalue variants, matrix functions, reordering)*
*Source doctrine: The Painted Fence — Jesse*
