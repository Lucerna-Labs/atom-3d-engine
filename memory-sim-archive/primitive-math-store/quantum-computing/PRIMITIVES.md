# Quantum Computing — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## Qubit Primitives

### qubit-state (cross-domain alias: `qubit`, `two-level-system`, `spin-1/2`)
**Domain:** Quantum Computing
**Definition:** A quantum bit: |ψ⟩ = α|0⟩ + β|1⟩ where |α|² + |β|² = 1. The state is a unit vector in ℂ².
**Atom or composite:** Atom (the fundamental unit)
**Cost model:** Qubit coherence time is the real cost — T1 (relaxation) and T2 (dephasing) degrade the state over time.
**Real wall?** Yes — decoherence (loss of quantum coherence) is the fundamental real wall. The qubit's quantum state must be preserved longer than gate times.
**Cross-domain wiring:** Qubit = two-level system. In physics: any system with two distinct energy levels. In signal: binary amplitude modulation.
**Notes:** Physical implementations: superconducting transmon, trapped ion, photonic, spin qubit, topological qubit. Each has different T1/T2 times and gate fidelities.

### qubit-measure (cross-domain alias: `measurement`, `Z-basis-measure`, `projective-measure`)
**Domain:** Quantum Computing
**Definition:** Project |ψ⟩ onto computational basis: |0⟩ with probability |α|², |1⟩ with probability |β|². Measurement collapses the state irreversibly.
**Atom or composite:** Composite: apply measurement operator → sample random outcome → collapse state.
**Cost model:** Measurement is destructive and probabilistic. In near-term hardware, measurement fidelity is a key constraint.
**Real wall?** Yes — measurement is irreversible. Once measured, the quantum information is gone (it becomes classical).
**Cross-domain wiring:** Measurement = projection onto a basis = threshold decision in signal processing. In physics: wave function collapse.
**Notes:** Weak measurement: partial information extraction without full collapse. Tomography: measuring many copies to reconstruct the state.

### qubit-entangle (cross-domain alias: `entanglement`, `Bell-state`, `GHZ-state`)
**Domain:** Quantum Computing
**Definition:** Create an entangled state between qubits: |Φ+⟩ = (|00⟩ + |11⟩)/√2. Entangled qubits have correlations stronger than any classical state.
**Atom or composite:** Composite: apply entangling gate (CNOT, CZ) to separable qubits → entangled state.
**Cost model:** Entanglement creation requires an entangling gate (CNOT). Gate fidelity and qubit connectivity determine feasibility.
**Real wall?** No. But entangled states are fragile — decoherence destroys them. Entanglement distribution over distance requires quantum repeaters.
**Cross-domain wiring:** Entanglement = non-local correlation = violating Bell inequalities. In signal: perfect correlation across channels. In physics: EPR correlation.
**Notes:** GHZ state: (|000⟩ + |111⟩)/√2 — generalization to N qubits. GHZ is maximally non-classical for N qubits.

---

## Single-Qubit Gates

### gate-H (cross-domain alias: `Hadamard-gate`, `superposition-gate`)
**Domain:** Quantum Computing
**Definition:** H|0⟩ = (|0⟩+|1⟩)/√2, H|1⟩ = (|0⟩−|1⟩)/√2. Creates superposition from a basis state.
**Atom or composite:** Atom (single-qubit unitary)
**Real wall?** No.
**Cross-domain wiring:** Hadamard = Fourier transform on ℤ₂. In signal: 90° hybrid coupler (creates equal superposition of I/Q). In linear algebra: Householder reflection.
**Notes:** H² = I: applying Hadamard twice returns the original state.

### gate-P (cross-domain alias: `phase-gate`, `S-gate`, `T-gate`)
**Domain:** Quantum Computing
**Definition:** P(φ)|0⟩ = |0⟩, P(φ)|1⟩ = e^{iφ}|1⟩. Adds a phase to the |1⟩ component.
**Atom or composite:** Atom
**Real wall?** No. But T gate (φ=π/8) is the expensive non-Clifford gate — fault-tolerant implementation requires magic state distillation.
**Cross-domain wiring:** Phase gate = rotation around Z axis on the Bloch sphere. In signal: phase shift = i·quadrature component.
**Notes:** T gate = P(π/4). The set {H, T, CNOT} is universal for quantum computation.

### gate-X (cross-domain alias: `NOT-gate`, `bit-flip`, `Pauli-X`)
**Domain:** Quantum Computing
**Definition:** X|0⟩ = |1⟩, X|1⟩ = |0⟩. Bit flip on computational basis.
**Atom or composite:** Atom
**Real wall?** No.
**Cross-domain wiring:** Pauli-X = NOT = classical XOR. In signal: polarity inversion.

### gate-Y (cross-domain alias: `Pauli-Y`, `bit-phase-flip`)
**Domain:** Quantum Computing
**Definition:** Y = i·|0⟩⟨1| − i·|1⟩⟨0|. Bit flip + phase flip.
**Atom or composite:** Atom
**Real wall?** No.

### gate-Z (cross-domain alias: `Pauli-Z`, `phase-flip`)
**Domain:** Quantum Computing
**Definition:** Z|0⟩ = |0⟩, Z|1⟩ = −|1⟩. Phase flip.
**Atom or composite:** Atom
**Real wall?** No.
**Cross-domain wiring:** Z = phase shift by π. In signal: 180° phase shift.

### gate-RX-RY-RZ (cross-domain alias: `rotation-gate`, `Bloch-sphere-rotation`)
**Domain:** Quantum Computing
**Definition:** R_x(θ) = e^{−iθX/2}, R_y(θ) = e^{−iθY/2}, R_z(θ) = e^{−iθZ/2}. Rotations around axes on the Bloch sphere.
**Atom or composite:** Composite: rotation operators = unitary matrices built from Pauli generators.
**Real wall?** No.
**Cross-domain wiring:** Rotation gates = unitary evolution = Schrödinger equation. In signal: phase rotation in IQ plane.

### gate-U (cross-domain alias: `universal-single-qubit-gate`, `SU(2)-gate`)
**Domain:** Quantum Computing
**Definition:** U(θ, φ, λ) = e^{i(α+β)/2}·RZ(φ)·RY(θ)·RZ(λ). The most general single-qubit unitary (up to global phase).
**Atom or composite:** Composite: sequence of rotations on Bloch sphere.
**Real wall?** No.

---

## Two-Qubit Gates

### gate-CNOT (cross-domain alias: `controlled-NOT`, `CX`, `Toffoli-seed`)
**Domain:** Quantum Computing
**Definition:** CNOT|00⟩→|00⟩, |01⟩→|01⟩, |10⟩→|11⟩, |11⟩→|10⟩. Control qubit flips target if control is |1⟩.
**Atom or composite:** Atom (two-qubit gate)
**Real wall?** No. But CNOT has limited connectivity in most hardware (only adjacent qubits).
**Cross-domain wiring:** CNOT = controlled-X = IF-THEN operation. In classical logic: the Toffoli gate is the reversible version of AND. In signal: gate signal based on control input.
**Notes:** CNOT + single-qubit gates = universal set.

### gate-CZ (cross-domain alias: `controlled-Z`, `controlled-phase`)
**Domain:** Quantum Computing
**Definition:** CZ|00⟩→|00⟩, |01⟩→|01⟩, |10⟩→|10⟩, |11⟩→−|11⟩. Adds a ZZ interaction.
**Atom or composite:** Atom
**Real wall?** No.
**Cross-domain wiring:** CZ = ZZ interaction term = Heisenberg coupling. In physics: Ising model interaction.

### gate-SWAP (cross-domain alias: `SWAP-gate`, `quantum-swap`)
**Domain:** Quantum Computing
**Definition:** SWAP|01⟩→|10⟩, |10⟩→|01⟩. Exchanges qubit states.
**Atom or composite:** Composite: SWAP = CNOT₁₂·CNOT₂₁·CNOT₁₂.
**Real wall?** No.

### gate-iSWAP (cross-domain alias: `iSWAP-gate`, `XY-interaction`)
**Domain:** Quantum Computing
**Definition:** iSWAP = (|01⟩⟨10| + |10⟩⟨01|)/√2 (plus phases). Creates XY-type interaction.
**Atom or composite:** Composite: iSWAP = RZ(π/2)·(H⊗H)·CNOT·(H⊗H).
**Real wall?** No.

### gate-Toffoli (cross-domain alias: `CCNOT`, `Toffoli-gate`, `AND-gate`)
**Domain:** Quantum Computing
**Definition:** CCNOT|abc⟩ → |ab(c⊕ab)⟩. Flips target iff both controls are |1⟩. Universal for reversible classical computation.
**Atom or composite:** Composite: typically implemented with 6 CNOTs + 6 single-qubit gates (standard decomposition).
**Real wall?** Yes — Toffoli gate cost is the main cost in quantum arithmetic. Large Toffoli networks are expensive.
**Cross-domain wiring:** Toffoli = AND + CNOT. In classical logic: universal for reversible computing. In signal: gated gate with two control inputs.

### gate-Fredkin (cross-domain alias: `controlled-SWAP`, `Fredkin-gate`)
**Domain:** Quantum Computing
**Definition:** CSWAP|abc⟩ → |a, b⊕a·c, c⊕a·b⟩. Swaps target qubits iff control is |1⟩.
**Atom or composite:** Composite: Fredkin = 5 CNOTs + 1 Toffoli.
**Real wall?** No.

---

## Multi-Qubit & Specialty Gates

### gate-Measurement-adaptive (cross-domain alias: `feed-forward-measurement`, `conditional-gate`)
**Domain:** Quantum Computing
**Definition:** Measure some qubits → use measurement outcomes to decide which gates to apply to remaining qubits.
**Atom or composite:** Composite: measure → classically compute → apply gate conditionally.
**Real wall?** Yes — real-time classical computation between measurement and subsequent gates requires fast classical control.
**Cross-domain wiring:** Adaptive measurement = closed-loop control. In control theory: feedback based on measurement.

### gate-CCCNOT (cross-domain alias: `multi-control-Toffoli`, `3-Control-Toffoli`)
**Domain:** Quantum Computing
**Definition:** Generalized Toffoli: flip target iff all N controls are |1⟩. Cost scales ~4N+1 CNOTs via Gray code decomposition.
**Atom or composite:** Composite: decompose into Toffoli + CNOT.
**Real wall?** Yes — CNOT count grows linearly with the number of controls. Large multi-control gates are expensive.
**Cross-domain wiring:** Multi-control = AND over N inputs. In logic: generalized AND gate.
**Notes:** For N controls: use ancilla qubits and intermediate Toffoli decomposition. Phase gradient adders use similar techniques.

---

## Quantum Error Correction

### qec-stabilizer (cross-domain alias: `stabilizer-formalism`, `Pauli-frames`)
**Domain:** Quantum Computing
**Definition:** Represent an N-qubit state by its stabilizers: {P₁, P₂, ..., P_{N−k}} where each P_i is a Pauli string and the state is the +1 eigenstate of all stabilizers.
**Atom or composite:** Composite: track stabilizer generators (2N bits) instead of full 2^N complex amplitudes.
**Real wall:** No.
**Cross-domain wiring:** Stabilizer formalism = classical simulation of a restricted class of quantum circuits. In coding theory: linear codes over GF(4) are equivalent to stabilizer codes.
**Notes:** The Clifford group (generated by H, S, CNOT) preserves the stabilizer form — efficient classical simulation is possible.

### qec-surface-code (cross-domain alias: `surface-code`, `planar-code`, `topological-QEC`)
**Domain:** Quantum Computing
**Definition:** 2D grid of data qubits and syndrome (measurement) qubits. Errors create pairs of syndrome excitations (e and m) that can be detected. Logical qubit = any topologically non-trivial loop of operations.
**Atom or composite:** Composite: prepare grid → measure stabilizers → detect syndrome → decode → correct.
**Cost model:** Current implementations require ~1000 physical qubits per logical qubit (surface code overhead).
**Real wall?** Yes — the physical qubit overhead (thousands per logical qubit) is a real wall. Fault-tolerant quantum computing requires many more physical qubits than logical ones.
**Cross-domain wiring:** Surface code = topological quantum field theory. In physics: anyons and braiding. In coding theory: a quantum LDPC code.

### qec-magic-state-distillation (cross-domain alias: `magic-state-factory`, `T-state-distillation`)
**Domain:** Quantum Computing
**Definition:** The T gate (non-Clifford) requires "magic states" — special resource states that can't be produced by Clifford operations alone. Distillation: encode → do Clifford operations → measure → keep states that pass.
**Atom or composite:** Composite: prepare magic state → distill → use T gate.
**Cost model:** T gate cost = magic state cost × distillation overhead. The distillation protocol consumes many magic states to produce one high-fidelity T.
**Real wall?** Yes — magic state distillation overhead is a dominant cost in fault-tolerant quantum computing. T gate is ~100-1000× more expensive than Clifford gates.
**Cross-domain wiring:** Magic state distillation = error correction + error filtering. In coding theory: concatenated codes = hierarchical error correction.

### qec-decoherence (cross-domain alias: `T1-T2`, `dephasing`, `depolarizing-channel`)
**Domain:** Quantum Computing
**Definition:** T1 (energy relaxation): |1⟩ → |0⟩ with rate 1/T1. T2 (dephasing): coherence decays with rate 1/T2. Depolarizing: with probability p, apply random Pauli.
**Atom or composite:** Atom (channel)
**Real wall?** Yes — T1 and T2 times are fundamental hardware limitations. Gate time / T1 must be << 1 for fault tolerance.
**Cross-domain wiring:** Depolarizing channel = uniform noise. In signal: additive white noise (but quantum).
**Notes:** Superconducting qubits: T1 ~ 50-200 μs, T2 ~ 50-300 μs. Gate times ~ 20-50 ns. So far so good.

---

## Quantum Algorithms

### algo-grover (cross-domain alias: `Grover-search`, `quantum-search`)
**Domain:** Quantum Computing
**Definition:** Search N items in O(√N) queries using amplitude amplification. Oracle marks target state; repeated Grover iterations (H, Oracle, H, Z) amplify target amplitude.
**Atom or composite:** Composite: H^{⊗n} → repeat: Oracle → Grover diffusion (H^{⊗n}·Z·H^{⊗n}·H^{⊗n}) → measure.
**Real wall?** Yes — O(√N) is quadratically faster than classical O(N), but requires a good oracle. For N=1000, √N = 31 queries. No speedup if N is small.
**Cross-domain wiring:** Amplitude amplification = importance sampling in quantum form. In optimization: quantum annealing.
**Notes:** Multi-target Grover: search for one of M targets → O(√(N/M)) queries.

### algo-shor (cross-domain alias: `Shor-factoring`, `period-finding`)
**Domain:** Quantum Computing
**Definition:** Factor N in poly(log N) time using quantum period finding. Compute period r of f(x) = a^x mod N using QFT. Period → factors via gcd.
**Atom or composite:** Composite: quantum period finding (modular exponentiation + QFT) → classical post-processing (gcd).
**Real wall?** Yes — Shor's algorithm threatens RSA/ECC. But requires ~10N logical qubits for N-bit numbers. No known classical cryptanalysis threat yet.
**Cross-domain wiring:** Shor = period finding = Fourier transform on ℤ_N. In physics: spectral analysis = finding eigenfrequencies.
**Notes:** RSA-2048 would need ~4000 logical qubits → ~4 million physical qubits. Current hardware: ~100 physical qubits.

### algo-QPE (cross-domain alias: `Quantum-Phase-Estimation`, `eigenvalue-estimation`)
**Domain:** Quantum Computing
**Definition:** Estimate eigenvalue λ of unitary U: |ψ⟩ is eigenstate. QPE: controlled-U^j operations + inverse QFT → estimate λ to t bits.
**Atom or composite:** Composite: prepare eigenstate → controlled powers of U → inverse QFT → measure phase.
**Real wall?** No. QPE is the quantum primitive for most quantum algorithms.
**Cross-domain wiring:** QPE = Fourier transform for eigenvalues = spectral analysis. In linear algebra: computing eigenvalues via phase estimation.
**Notes:** QPE is the basis of HHL (quantum linear systems), quantum chemistry algorithms, and Shor's algorithm.

### algo-HHL (cross-domain alias: `quantum-linear-solver`, `HHL-algorithm`)
**Domain:** Quantum Computing
**Definition:** Solve linear system Ax = b exponentially faster than classical. Requires preparing b as quantum state, QPE to condition on eigenvalues, rotation to encode solution, inverse QPE.
**Atom or composite:** Composite: prepare |b⟩ → QPE (condition on eigenvalues) → rotate → QPE† (uncompute).
**Real wall?** Yes — exponential speedup is conditional on sparse/A-conditioned matrix. State preparation and readout are costly.
**Cross-domain wiring:** HHL = quantum linear algebra. In linear algebra: solving sparse linear systems.
**Notes:** HHL gives exponential speedup for the specific task of computing expectation values of the solution vector, not for retrieving the full solution.

### algo-VQE (cross-domain alias: `variational-quantum-eigensolver`, `hybrid-QC`)
**Domain:** Quantum Computing
**Definition:** Hybrid quantum-classical optimization: quantum computer evaluates energy (Hamiltonian expectation) for a parameterized circuit; classical optimizer updates parameters.
**Atom or composite:** Composite: prepare ansatz state → measure Hamiltonian terms → compute energy → classical optimizer → update params.
**Real wall?** Yes — barren plateaus (gradient vanishing) and classical simulation cost of the classical optimizer limit scalability.
**Cross-domain wiring:** VQE = variational quantum circuit + classical optimizer. In ML: same structure as hybrid neural network training.
**Notes:** VQE is the primary algorithm for near-term (NISQ) quantum chemistry.

### algo-QAOA (cross-domain alias: `Quantum-Approximate-Optimization-Algorithm`)
**Domain:** Quantum Computing
**Definition:** Variational algorithm for combinatorial optimization: parameterize as alternating layers of problem Hamiltonian H_C and mixer Hamiltonian H_M. Vary parameters to minimize expectation of H_C.
**Atom or composite:** Composite: initial state → repeat: apply H_C(γ) → apply H_M(β) → measure.
**Real wall?** Yes — QAOA does not provably outperform classical optimization for general problems. For specific structured problems (MAX-CUT on graphs), it can show advantage.
**Cross-domain wiring:** QAOA = quantum annealing in discrete time. In physics: quantum adiabatic evolution discretized.

---

## Quantum Communication

### qc-teleport (cross-domain alias: `quantum-teleportation`, `ENT-state-transfer`)
**Domain:** Quantum Computing
**Definition:** Transfer an unknown quantum state from A to B using: (1) Bell pair shared between A and B; (2) Bell measurement at A; (3) classical communication of measurement outcomes; (4) local corrections at B.
**Atom or composite:** Composite: prepare Bell pair → Bell measurement (A) → classical communication → local correction (B).
**Real wall?** Yes — classical communication of measurement outcomes is required (speed of light limit). No faster-than-light communication.
**Cross-domain wiring:** Teleportation = moving quantum information without moving the physical qubit. In signal: forwarding a signal by re-transmitting.

### qc-entanglement-swap (cross-domain alias: `entanglement-swapping`, `purification`)
**Domain:** Quantum Computing
**Definition:** Given two entangled pairs (A-B, C-D), perform Bell measurement on B-C. Result: A becomes entangled with D (without direct A-D entanglement).
**Atom or composite:** Composite: Bell measurement on B-C → classical announcement → A-D are now entangled.
**Real wall:** No. But each entanglement swap consumes a pair of entangled qubits.
**Cross-domain wiring:** Entanglement swapping = projective measurement that projects distant systems onto an entangled state. In physics: Bell state measurement.

### qc-QKD (cross-domain alias: `BB84`, `E91`, `quantum-key-distribution`)
**Domain:** Quantum Computing
**Definition:** BB84: sender encodes random bits in qubit basis (Z or X). Receiver measures in random basis. Sifted key: keep bits where bases matched. Eavesdropper causes detectable error rate.
**Atom or composite:** Composite: prepare qubit (basis + bit) → send → receive (random basis) → sift → error check → privacy amplification.
**Real wall?** No. But QKD requires quantum channels (fiber or free-space), not standard internet. Quantum repeaters needed for long distances.
**Cross-domain wiring:** QKD = information-theoretic security based on quantum mechanics. In cryptography: the only provably secure key exchange mechanism (under quantum mechanics).
**Notes:** Device-independent QKD removes the need to trust the hardware — security based on Bell inequality violation.

---

## Quantum Fourier Transform

### qft (cross-domain alias: `Quantum-Fourier-Transform`, `QFT`)
**Domain:** Quantum Computing
**Definition:** QFT|x⟩ = (1/√N) Σ_y e^{2πixy/N}|y⟩. The quantum version of the discrete Fourier transform.
**Atom or composite:** Composite: for each qubit i: apply Hadamard → apply controlled rotations R_k with k = N/2^i → swap pairs.
**Real wall:** No. But QFT requires O(n²) gates for n qubits. The inverse QFT requires the same.
**Cross-domain wiring:** QFT = Fourier transform on ℤ_{2^n}. In signal: FFT on quantum superposition. In linear algebra: DFT matrix applied to quantum state amplitudes.
**Notes:** QFT is exponentially faster than classical FFT on quantum hardware because the Fourier transform is applied to 2^n amplitudes simultaneously.

### qft-inverse (cross-domain alias: `QFT-dagger`, `QFT†`)
**Domain:** Quantum Computing
**Definition:** The inverse of QFT: QFT†|x⟩ = (1/√N) Σ_y e^{−2πixy/N}|y⟩. Reverses the phase relationships.
**Atom or composite:** Composite: reverse the QFT circuit (swap pairs, apply inverse rotations, apply H).
**Real wall:** No.

---

## Summary: Quantum Atom → Cross-Domain Wiring

| Quantum Primitive | Linear Algebra Alias | Signal Alias | Physics Alias |
|---|---|---|---|
| qubit-state | normalized complex vector | binary state | spin-1/2 particle |
| gate-H | Hadamard matrix | superposition creation | 90° hybrid coupler |
| gate-CNOT | controlled-X matrix | gated switching | IF-THEN operation |
| gate-CZ | ZZ interaction | phase flip | Ising coupling |
| qubit-measure | projection onto basis | threshold detection | wave function collapse |
| qubit-entangle | Bell state generation | non-local correlation | EPR correlation |
| algo-grover | amplitude amplification | quantum search | quantum annealing |
| algo-shor | period finding | Fourier on ℤ_N | spectral analysis |
| algo-QPE | eigenvalue estimation | spectral analysis | resonance detection |
| qft | Fourier transform | FFT | wave decomposition |
| qc-QKD | basis encoding | quantum encoding | photon polarization |
| qc-teleport | state transfer | signal forwarding | entanglement transfer |

---

*Last updated: 2026-06-21*
*Source doctrine: The Painted Fence — Jesse*

---

## Oracle-Based Algorithms

### deutsch-jozsa (cross-domain alias: `DJ-algorithm`, `constant-vs-balanced`, `oracle-distinguisher`)
**Domain:** Quantum Computing
**Definition:** Given oracle Uf for f:{0,1}ⁿ→{0,1} promised constant or balanced, decide which with ONE query. Apply H⊗ⁿ, Uf, H⊗ⁿ; measure all-zero ⇒ constant.
**Atom or composite:** Composite: H⊗ⁿ → oracle → H⊗ⁿ → measure.
**Cost model:** 1 oracle query (quantum) vs 2ⁿ⁻¹+1 classical worst-case. O(n) gates.
**Real wall?** No — but the promise structure is artificial; rarely matches real problems.
**Cross-domain wiring:** linear-algebra-matrix: Hadamard basis change reveals global structure of f. signal-processing-rf: Walsh-Hadamard transform of indicator function. cryptography-advanced: prototype for hidden subgroup problems.
**Notes:** Deutsch (1985), Deutsch-Jozsa (1992) — first exponential quantum speedup, foundational template for Simon/Shor.

### bernstein-vazirani (cross-domain alias: `BV-algorithm`, `hidden-bitstring`, `inner-product-oracle`)
**Domain:** Quantum Computing
**Definition:** Oracle Uf encodes f(x)=s·x mod 2 for hidden s∈{0,1}ⁿ. Recover s with ONE query: H⊗ⁿ|0⟩ → Uf → H⊗ⁿ → measure = s.
**Atom or composite:** Composite: identical circuit to Deutsch-Jozsa, different oracle.
**Cost model:** 1 query vs n classical queries. O(n) gates.
**Real wall?** No — assumes phase-kickback oracle access.
**Cross-domain wiring:** linear-algebra-matrix: inner-product extraction via Hadamard. cryptography-advanced: learning-with-parity (LPN) without noise. ml-training: prototype quantum learning advantage.
**Notes:** Bernstein-Vazirani (1993). Shows query separation O(1) vs O(n) — modest but cleaner than DJ.

### simon-algorithm (cross-domain alias: `simons-problem`, `hidden-XOR-period`, `Z2n-period-finding`)
**Domain:** Quantum Computing
**Definition:** Oracle f:{0,1}ⁿ→{0,1}ⁿ with f(x)=f(x⊕s) for hidden s≠0. Find s with O(n) queries. Repeatedly measure post-Hadamard register to gather y·s=0 equations; solve linearly.
**Atom or composite:** Composite: H⊗ⁿ → Uf → H⊗ⁿ → measure → repeat → Gaussian elimination.
**Cost model:** O(n) quantum queries vs Ω(2^{n/2}) classical. Linear post-processing.
**Real wall?** No — exponential separation in query complexity.
**Cross-domain wiring:** linear-algebra-matrix: solving over 𝔽₂. cryptography-advanced: Simon-style attacks on Even-Mansour, FX-construction. number-theory: precursor to abelian hidden subgroup problem.
**Notes:** Simon (1994) — direct inspiration for Shor's algorithm.

### amplitude-amplification (cross-domain alias: `generalized-grover`, `AA`, `Brassard-Hoyer-Mosca-Tapp`)
**Domain:** Quantum Computing
**Definition:** Given algorithm A producing |ψ⟩=sin(θ)|good⟩+cos(θ)|bad⟩, iterate Q=−A·S₀·A†·Sχ to rotate amplitude. After k≈π/(4θ) iterations, |good⟩ dominates.
**Atom or composite:** Composite generalization of Grover; A replaces uniform superposition.
**Cost model:** O(1/√p) calls to A where p=|⟨good|ψ⟩|². Quadratic speedup over O(1/p) classical.
**Real wall?** No — but requires reflection S₀ around initial state.
**Cross-domain wiring:** linear-algebra-matrix: rotation in 2D invariant subspace. statistics-probability: quantum analogue of importance sampling. ml-training: speedup for rejection-based sampling.
**Notes:** Brassard, Hoyer, Mosca, Tapp (2000). Foundation for QAE, quantum mean estimation, dozens of derivative algorithms.

### amplitude-estimation (cross-domain alias: `QAE`, `quantum-counting`, `phase-estimation-on-Q`)
**Domain:** Quantum Computing
**Definition:** Estimate p=|⟨good|ψ⟩|² with additive error ε using O(1/ε) queries to A. Apply QPE to Grover operator Q whose eigenphases ±2θ encode p=sin²(θ).
**Atom or composite:** Composite: A → controlled-Q ladder → inverse QFT → measure.
**Cost model:** O(1/ε) queries vs O(1/ε²) classical Monte Carlo. Quadratic Monte Carlo speedup.
**Real wall?** Depth-heavy due to QPE — challenging on NISQ hardware.
**Cross-domain wiring:** finance-pricing: option pricing speedup. statistics-probability: Monte Carlo integration. signal-processing-rf: phase estimation of Grover operator.
**Notes:** Brassard et al. (2000). Backbone for quantum-accelerated finance, integration, ML expectation values.

### iterative-amplitude-estimation (cross-domain alias: `IQAE`, `Grinko-Gacon-Zoufal-Woerner`, `QPE-free-QAE`)
**Domain:** Quantum Computing
**Definition:** QAE without QFT/QPE: iteratively refine confidence interval [θ_lo,θ_hi] using only Grover power applications and basis measurements with Chernoff bounds.
**Atom or composite:** Composite of A·Qᵏ·measurement with adaptive k schedule.
**Cost model:** O((1/ε)·log(1/δ)) queries; NO ancillas, NO QFT.
**Real wall?** No — practical on near-term hardware.
**Cross-domain wiring:** statistics-probability: confidence interval refinement. signal-processing-rf: power-of-Q without spectral readout. ml-training: estimator of expectations under quantum prior.
**Notes:** Grinko et al. (2021). Currently the preferred QAE variant for NISQ era.

### MLAE (cross-domain alias: `maximum-likelihood-amplitude-estimation`, `Suzuki-MLAE`, `QPE-free-likelihood-QAE`)
**Domain:** Quantum Computing
**Definition:** Run A·Qᵐ for several m, count |good⟩ outcomes, then MLE the angle θ from likelihood ∏ P(k_m|θ).
**Atom or composite:** Composite + classical MLE post-processing.
**Cost model:** O(1/ε) queries with explicit schedule (e.g., m_k = 2^k −1).
**Real wall?** No — bounded by likelihood-flat regions; needs careful schedule.
**Cross-domain wiring:** statistics-probability: maximum likelihood inference. ml-training: classical likelihood fit on quantum-generated samples.
**Notes:** Suzuki et al. (2020). Simpler implementation, used widely in finance benchmarks.

### robust-amplitude-estimation (cross-domain alias: `RAE`, `noise-aware-QAE`, `Wang-Higgott-Brierley`)
**Domain:** Quantum Computing
**Definition:** QAE variant that models depolarizing noise as a fidelity F per Grover iterate; jointly estimates p and F via Bayesian inference.
**Atom or composite:** Composite + Bayesian filter / Hamiltonian Monte Carlo.
**Cost model:** Same scaling as IQAE but with noise-robust error bars.
**Real wall?** Effective up to coherence-limited depth.
**Cross-domain wiring:** statistics-probability: Bayesian inference under nuisance parameter. quantum-noise-mitigation: built-in error model. signal-processing-rf: SNR-aware estimation.
**Notes:** Wang, Higgott, Brierley (2021). Bridges QAE with realistic NISQ noise.

### grover-fixed-point (cross-domain alias: `fixed-point-AA`, `pi/3-algorithm`, `Yoder-Low-Chuang`)
**Domain:** Quantum Computing
**Definition:** Modified amplitude amplification that monotonically converges (no over-rotation) even with imperfect angle estimate. Uses generalized π/3 rotations.
**Atom or composite:** Composite of phase-modulated Grover iterations.
**Cost model:** O((1/√p)·log(1/δ)) — slight log overhead, but tolerant.
**Real wall?** No — fixes the "soufflé problem" of classical Grover.
**Cross-domain wiring:** signal-processing-rf: phase-corrected matched filtering. ml-training: robust search without prior probability knowledge. statistics-probability: monotone convergence.
**Notes:** Yoder, Low, Chuang (2014). Essential when p is unknown a priori.

### quantum-mean-estimation (cross-domain alias: `Montanaro-mean`, `quantum-Monte-Carlo`, `QMC-speedup`)
**Domain:** Quantum Computing
**Definition:** Given quantum sampler for random variable X with mean μ and variance σ², estimate μ to additive error ε with O(σ/ε) queries (vs O(σ²/ε²) classical).
**Atom or composite:** Composite: amplitude estimation on indicator of X-thresholds + median-of-means.
**Cost model:** Õ(σ/ε) quantum queries. Quadratic Monte Carlo speedup.
**Real wall?** Requires quantum sample access (QRAM-like).
**Cross-domain wiring:** statistics-probability: quantum-accelerated estimation. finance-pricing: derivative pricing. ml-training: gradient estimation in expectation form.
**Notes:** Montanaro (2015). Generic Monte-Carlo speedup engine.

---

## Quantum Walks

### dtqw (cross-domain alias: `discrete-time-quantum-walk`, `coined-quantum-walk`, `Aharonov-walk`)
**Domain:** Quantum Computing
**Definition:** State |position⟩⊗|coin⟩ evolves by U=S·(I⊗C). Coin flip C (e.g. Hadamard) followed by shift S that moves position conditioned on coin. Variance grows ∝t² (ballistic) vs ∝t classical.
**Atom or composite:** Composite of coin gate + conditional shift, iterated.
**Cost model:** O(t·log N) gates for t steps on graph of N nodes.
**Real wall?** No — diffusion replaced by interference.
**Cross-domain wiring:** signal-processing-rf: tight-binding model on lattice. statistics-probability: non-Markovian walk. ml-training: backbone for QML walk-based algorithms.
**Notes:** Aharonov, Davidovich, Zagury (1993). Source of quadratic speedup in many graph problems.

### ctqw (cross-domain alias: `continuous-time-quantum-walk`, `Farhi-Gutmann-walk`, `Hamiltonian-walk`)
**Domain:** Quantum Computing
**Definition:** Hamiltonian H = adjacency matrix (or Laplacian) of graph G. Evolution |ψ(t)⟩ = e^{−iHt}|ψ(0)⟩. Spreads coherently on graph.
**Atom or composite:** Atom (Hamiltonian evolution on graph).
**Cost model:** Hamiltonian simulation cost — O(t·d·polylog) for sparse graphs.
**Real wall?** Locality of H — must implement e^{−iHt}.
**Cross-domain wiring:** linear-algebra-matrix: matrix exponential of adjacency. graph-theory: spectral graph walks. ml-training: kernel on graphs (graph diffusion).
**Notes:** Farhi & Gutmann (1998). Glued-trees exponential speedup uses CTQW.

### szegedy-walk (cross-domain alias: `quantum-walk-on-MC`, `Szegedy-quantization`, `walk-search`)
**Domain:** Quantum Computing
**Definition:** Quantization of classical Markov chain P. Walk operator W=S·(2|π⟩⟨π|−I) on edge space. Spectral gap of W is Θ(√Δ) where Δ is classical gap.
**Atom or composite:** Composite: reflection through stationary edge-state + swap.
**Cost model:** Hitting time O(√(HT_classical)) — quadratic walk speedup.
**Real wall?** Requires implementable transition oracle.
**Cross-domain wiring:** statistics-probability: MCMC quadratic speedup. linear-algebra-matrix: discriminant matrix spectral mapping. ml-training: sampling-based learning.
**Notes:** Szegedy (2004). Quantum hitting time framework — basis for many quantum algorithms.

### glued-trees (cross-domain alias: `Childs-glued-trees`, `exponential-walk-speedup`, `welded-trees`)
**Domain:** Quantum Computing
**Definition:** Two binary trees of depth n joined by random matching at leaves. CTQW finds opposite root in poly(n) time; classical needs exp(n).
**Atom or composite:** Composite: CTQW on specific graph oracle.
**Cost model:** Quantum O(n⁵) vs classical Ω(2^{n/2}).
**Real wall?** Requires graph oracle.
**Cross-domain wiring:** graph-theory: oracle traversal. ml-training: shows superpolynomial separation. cryptography-advanced: black-box separation.
**Notes:** Childs, Cleve, Deotto, Farhi, Gutmann, Spielman (2003). One of few known exponential walk separations.

### element-distinctness (cross-domain alias: `Ambainis-walk`, `quantum-collision-finding`, `walk-on-Johnson-graph`)
**Domain:** Quantum Computing
**Definition:** Given f:[N]→[M], decide if all f(i) distinct using O(N^{2/3}) queries via quantum walk on Johnson graph J(N,r), r=N^{2/3}.
**Atom or composite:** Composite: setup + walk + check phases.
**Cost model:** O(N^{2/3}) queries, optimal.
**Real wall?** No — but uses substantial memory (r values stored).
**Cross-domain wiring:** cryptography-advanced: collision-resistance bounds. statistics-probability: birthday paradox quantum analogue. linear-algebra-matrix: Johnson-scheme spectra.
**Notes:** Ambainis (2007). Resolved long-standing open problem; tight lower bound by polynomial method.

### quantum-hitting-time (cross-domain alias: `QHT`, `Szegedy-hitting`, `walk-detection`)
**Domain:** Quantum Computing
**Definition:** Time for quantum walk to detect marked vertex with constant probability. For ergodic chain with gap δ and marked fraction ε, QHT = O(1/√(δ·ε)).
**Atom or composite:** Composite of Szegedy walk + amplitude amplification.
**Cost model:** Quadratic speedup over classical hitting time.
**Real wall?** No — generic over reversible Markov chains.
**Cross-domain wiring:** statistics-probability: MCMC mixing speedup. ml-training: quantum-accelerated reinforcement-learning exploration. graph-theory: spectral hitting bounds.
**Notes:** Magniez, Nayak, Roland, Santha (2007). Unified walk-search framework.

---

## Hamiltonian Simulation

### trotter-1 (cross-domain alias: `Trotter-Lie-formula`, `first-order-product-formula`, `Lie-splitting`)
**Domain:** Quantum Computing
**Definition:** For H=A+B, e^{−i(A+B)t} ≈ (e^{−iA t/r}·e^{−iB t/r})^r with error O(t²/r·‖[A,B]‖).
**Atom or composite:** Composite of single-term exponentials.
**Cost model:** r=O(t²·‖[A,B]‖/ε) Trotter steps for error ε.
**Real wall?** No — but exponentially expensive in t for small ε.
**Cross-domain wiring:** linear-algebra-matrix: matrix-exponential splitting. signal-processing-rf: leapfrog integrators. classical-numerics: operator splitting methods.
**Notes:** Lie-Trotter formula (1875/1959). Foundation of Hamiltonian simulation, Lloyd's 1996 result.

### trotter-2 (cross-domain alias: `Strang-splitting`, `second-order-Suzuki`, `symmetric-Trotter`)
**Domain:** Quantum Computing
**Definition:** S₂(t) = e^{−iA t/2}·e^{−iB t}·e^{−iA t/2}. Error O(t³·‖[A,[A,B]]‖+‖[B,[A,B]]‖).
**Atom or composite:** Composite — symmetric BCH structure.
**Cost model:** r = O((t³/ε)^{1/2}) Trotter steps.
**Real wall?** No — better commutator scaling than first-order.
**Cross-domain wiring:** signal-processing-rf: Strang splitting in PDE solvers. linear-algebra-matrix: symmetric product formula. statistics-probability: SDE integrators.
**Notes:** Strang (1968). Workhorse method for chemistry simulation.

### suzuki-higher (cross-domain alias: `Suzuki-fractal-formula`, `Suzuki-2k`, `recursive-Trotter`)
**Domain:** Quantum Computing
**Definition:** Recursive construction S_{2k}(t) = S_{2k-2}(s_k t)²·S_{2k-2}((1-4s_k)t)·S_{2k-2}(s_k t)², s_k=1/(4−4^{1/(2k−1)}). Order 2k product formula.
**Atom or composite:** Composite — exponential fractal.
**Cost model:** Step count r = O((t^{1+1/2k}/ε^{1/2k})·5^{k}).
**Real wall?** Diminishing returns past k≈4 due to 5^k blow-up.
**Cross-domain wiring:** linear-algebra-matrix: high-order operator splitting. signal-processing-rf: symplectic integrators. statistics-probability: high-order SDE methods.
**Notes:** Suzuki (1991). Tight error analysis by Childs, Su, Tran, Wiebe, Zhu (2021).

### qdrift (cross-domain alias: `random-Trotter`, `Campbell-qDRIFT`, `stochastic-Hamiltonian-sim`)
**Domain:** Quantum Computing
**Definition:** Sample term H_j with prob ‖h_j‖/λ, apply e^{−iH_j·t·λ/N}; repeat N=2λ²t²/ε times. Error independent of number of Hamiltonian terms.
**Atom or composite:** Composite — randomized product formula.
**Cost model:** O(λ²t²/ε) where λ=Σ‖h_j‖. Beats Trotter when many small terms.
**Real wall?** No — but worse t-scaling than QSP-based methods.
**Cross-domain wiring:** statistics-probability: Monte Carlo on operators. ml-training: stochastic gradient analogue. signal-processing-rf: randomized sampling.
**Notes:** Campbell (2019). Especially good for chemistry Hamiltonians with O(N⁴) terms.

### lcu (cross-domain alias: `linear-combination-of-unitaries`, `Berry-Childs-Kothari`, `unitary-decomposition`)
**Domain:** Quantum Computing
**Definition:** Implement H=Σ_j α_j U_j via PREP|0⟩=Σ√(α_j/λ)|j⟩ ancilla, controlled SELECT applies U_j conditioned on j, then PREP† and post-select |0⟩. Success prob (‖H‖/λ)².
**Atom or composite:** Composite: PREP-SELECT-PREP† sandwich.
**Cost model:** O(λ/ε) given α_j, U_j oracles. Backbone of qubitization.
**Real wall?** Subnormalization factor λ — λ=Σα_j ≥ ‖H‖.
**Cross-domain wiring:** linear-algebra-matrix: sparse decomposition into unitaries. signal-processing-rf: weighted-sum filters. statistics-probability: importance-weighted operators.
**Notes:** Childs & Wiebe (2012); refined by Berry-Childs-Kothari. Foundational primitive for modern quantum algorithms.

### block-encoding (cross-domain alias: `unitary-dilation`, `BE`, `matrix-as-corner-of-unitary`)
**Domain:** Quantum Computing
**Definition:** Unitary U is (α,a,ε)-block-encoding of A if ‖A − α·(⟨0|⊗I)U(|0⟩⊗I)‖ ≤ ε. The matrix A appears as the top-left block of α·U.
**Atom or composite:** Composite of LCU, qubitization, oracle access.
**Cost model:** Construction cost depends on access model (sparse, QRAM, LCU).
**Real wall?** Subnormalization α inflates simulation cost linearly.
**Cross-domain wiring:** linear-algebra-matrix: matrix-as-submatrix-of-unitary. signal-processing-rf: dilation theorem analogue. ml-training: encoding kernels into circuits.
**Notes:** Low & Chuang (2017); Gilyén-Su-Low-Wiebe (2019). The unifying object of modern quantum algorithms.

### qubitization (cross-domain alias: `Low-Chuang-qubitization`, `walk-operator-from-BE`, `reflection-of-block-encoding`)
**Domain:** Quantum Computing
**Definition:** Given block encoding W of A, the walk operator U_W = (2|0⟩⟨0|−I)⊗I · W has eigenvalues e^{±i·arccos(λ_k/α)} embedding spectrum of A on a qubit-sized invariant subspace.
**Atom or composite:** Composite of block encoding + reflection.
**Cost model:** O(α·t) queries to W for Hamiltonian sim; optimal in α,t,ε.
**Real wall?** No — provably optimal Hamiltonian simulation method.
**Cross-domain wiring:** linear-algebra-matrix: SVD spectral lifting. signal-processing-rf: phase-estimation-friendly encoding. ml-training: SVD-based feature transforms.
**Notes:** Low & Chuang (2017). Unifies QSP, QSVT, QPE, AA into one framework.

### qsp (cross-domain alias: `quantum-signal-processing`, `Low-Yoder-Chuang`, `Chebyshev-on-qubit`)
**Domain:** Quantum Computing
**Definition:** For signal unitary W=e^{iθZ} (signal=cos θ), interleave with parameterized X-rotations: ∏_k R_x(φ_k)·W gives U(φ) whose top-left entry is polynomial P(cos θ) of degree d=len(φ).
**Atom or composite:** Composite — phased product of W and rotations.
**Cost model:** O(d) gates for polynomial of degree d; classical preprocessing computes φ.
**Real wall?** Polynomial must satisfy parity + bound conditions.
**Cross-domain wiring:** signal-processing-rf: Chebyshev FIR filter design. linear-algebra-matrix: polynomial transformations of operators. ml-training: polynomial feature maps.
**Notes:** Low & Chuang (2016). Implements ANY bounded poly P(x) on a qubit via O(d) gates.

### qsvt (cross-domain alias: `quantum-singular-value-transformation`, `Gilyén-Su-Low-Wiebe`, `polynomial-of-singular-values`)
**Domain:** Quantum Computing
**Definition:** Given block-encoding of A=UΣV†, QSVT implements f(Σ) for any bounded polynomial f. Generalizes QSP to non-square / non-unitary block-encoded matrices.
**Atom or composite:** Composite of qubitized W and phase rotations on signal register.
**Cost model:** O(d) block-encoding queries; classical Remez to find φ.
**Real wall?** Subnormalization, parity constraints.
**Cross-domain wiring:** linear-algebra-matrix: f(SVD)=Uf(Σ)V†. signal-processing-rf: spectral filtering. ml-training: kernel transforms on data matrices.
**Notes:** Gilyén, Su, Low, Wiebe (2019). The "grand unifier" of quantum algorithms; recovers Grover, HHL, QPE, AA, simulation.

### taylor-series-sim (cross-domain alias: `Berry-truncated-Taylor`, `BCCKS-simulation`, `LCU-Taylor`)
**Domain:** Quantum Computing
**Definition:** Approximate e^{−iHt} ≈ Σ_{k=0}^K (−iHt)^k/k! via LCU on each term. Robust amplification recovers unitary.
**Atom or composite:** Composite: LCU of polynomial of H + oblivious amplification.
**Cost model:** O(t·log(t/ε)/log log(t/ε)) queries — nearly linear in t.
**Real wall?** Subnormalization of LCU; ancilla overhead.
**Cross-domain wiring:** linear-algebra-matrix: Taylor expansion of matrix exponential. signal-processing-rf: polynomial approximation. ml-training: truncated kernel expansions.
**Notes:** Berry, Childs, Cleve, Kothari, Somma (2015). Exponential improvement over Trotter in ε.

### sparse-hamiltonian-sim (cross-domain alias: `sparse-H-oracle`, `row-column-access-sim`, `Berry-Childs-sparse`)
**Domain:** Quantum Computing
**Definition:** H is d-sparse with oracles (i,j)→H_{ij}. Achieve query complexity O(τ·polylog(τ/ε)/log log(τ/ε)) where τ=d²‖H‖_max·t.
**Atom or composite:** Composite: graph coloring → 1-sparse pieces → simulated via QSVT/LCU.
**Cost model:** Optimal in d, ‖H‖_max, t, ε.
**Real wall?** No — but requires structured oracle access.
**Cross-domain wiring:** graph-theory: edge coloring of H-graph. linear-algebra-matrix: sparse matrix exponential. signal-processing-rf: sparse FIR analogue.
**Notes:** Berry, Childs (2012); Low-Chuang (2017). Standard for k-local Hamiltonians.

### multiproduct-formula (cross-domain alias: `Richardson-extrapolation-sim`, `MPF`, `Childs-Wiebe-multiproduct`)
**Domain:** Quantum Computing
**Definition:** Linear combination of low-order Trotter formulas e.g., Σ a_k S_2(t/k)^k tuned to cancel low-order errors via Richardson extrapolation.
**Atom or composite:** Composite — LCU of Trotter products with classical coefficients.
**Cost model:** Better commutator scaling than fixed-order Trotter; comparable to QSP.
**Real wall?** Requires LCU overhead (ancilla, post-selection).
**Cross-domain wiring:** numerics: Richardson extrapolation. linear-algebra-matrix: high-order approximation by combination. signal-processing-rf: filter combination.
**Notes:** Childs & Wiebe (2012); refined Watson, Aharonov, Childs (2023).

### interaction-picture-sim (cross-domain alias: `IPS`, `Low-Wiebe-interaction-picture`, `rotating-frame-sim`)
**Domain:** Quantum Computing
**Definition:** Split H=H₀+H_I where ‖H₀‖≫‖H_I‖. Simulate in rotating frame using e^{iH₀t}H_I e^{−iH₀t}; reduces simulation cost to scale with ‖H_I‖, not ‖H‖.
**Atom or composite:** Composite — frame transformation + Dyson series.
**Cost model:** O(t·‖H_I‖·polylog(1/ε)) — independent of ‖H₀‖.
**Real wall?** Requires fast-forwardable H₀ (e.g., diagonal).
**Cross-domain wiring:** signal-processing-rf: rotating-frame transforms in NMR/MRI. linear-algebra-matrix: similarity transform diagonalization. physics: Dirac interaction picture.
**Notes:** Low & Wiebe (2018). Crucial for plane-wave chemistry, lattice models with large kinetic term.

---

## Quantum Linear Algebra

### hhl-eigenvalue-inversion (cross-domain alias: `HHL-inner-routine`, `eigenvalue-rotation-step`, `1/λ-conditional-rotation`)
**Domain:** Quantum Computing
**Definition:** Within HHL: after QPE writes eigenvalue λ_k to ancilla, apply controlled R_y with angle θ=2 arcsin(C/λ_k) to ancilla qubit, then uncompute QPE.
**Atom or composite:** Composite: QPE → controlled rotation → inverse QPE.
**Cost model:** Dominated by QPE; condition number κ enters via failure probability.
**Real wall?** Conditional rotation needs accurate λ_k estimate; small λ blow up.
**Cross-domain wiring:** linear-algebra-matrix: reciprocal of eigenvalue → A⁻¹. signal-processing-rf: spectral inversion. ml-training: ridge regression-style inversion.
**Notes:** Harrow-Hassidim-Lloyd (2009) inner step. Replaced by QSVT-based inversion (Chakraborty-Gilyén-Jeffery).

### qlsa-qsvt (cross-domain alias: `QSVT-linear-solver`, `block-encoded-Ax=b`, `Chakraborty-Gilyén-Jeffery`)
**Domain:** Quantum Computing
**Definition:** Given block encoding of A and unitary preparing |b⟩, prepare |A⁻¹b⟩/‖A⁻¹b‖ via QSVT polynomial approximating 1/x on [1/κ,1].
**Atom or composite:** Composite of block encoding + QSVT polynomial filter.
**Cost model:** O(κ·polylog(κ/ε)) — exponential improvement in ε over HHL.
**Real wall?** Condition number κ; output state normalization issues remain.
**Cross-domain wiring:** linear-algebra-matrix: matrix inverse via Chebyshev approximation of 1/x. signal-processing-rf: inverse filter. ml-training: kernel ridge regression speedup.
**Notes:** Chakraborty, Gilyén, Jeffery (2018). Current SOTA quantum linear solver.

### block-encoding-arithmetic (cross-domain alias: `BE-sum-product-tensor`, `matrix-algebra-on-BE`, `Gilyén-arithmetic`)
**Domain:** Quantum Computing
**Definition:** Given BE(A), BE(B): construct BE(A+B) via LCU, BE(A·B) via composition, BE(A⊗B) trivially via ancilla rearrangement.
**Atom or composite:** Composite — algebraic primitives on block encodings.
**Cost model:** Multiplicative: BE(A·B) costs ≈ BE(A)+BE(B) queries; sum doubles ancilla.
**Real wall?** Subnormalization compounds: α_{A+B} ≤ α_A + α_B.
**Cross-domain wiring:** linear-algebra-matrix: closure of matrix operations. signal-processing-rf: filter composition. ml-training: composing kernels.
**Notes:** Gilyén-Su-Low-Wiebe (2019). Allows constructing complex matrix functions from primitive block encodings.

### quantum-svd (cross-domain alias: `qSVD`, `block-encoded-SVD`, `singular-value-projection`)
**Domain:** Quantum Computing
**Definition:** Given block encoding of A, use QSVT to project onto singular values above threshold, extract via QPE-like routine on qubitized walk operator.
**Atom or composite:** Composite of qubitization + QSVT filter + measurement.
**Cost model:** O(polylog/ε) per singular value; total scales with rank or threshold.
**Real wall?** Output is quantum state encoding, not classical SVD.
**Cross-domain wiring:** linear-algebra-matrix: SVD = U·diag(σ)·V†. signal-processing-rf: subspace methods. ml-training: PCA, low-rank decomposition.
**Notes:** Foundation for quantum PCA, recommendation systems (dequantized by Tang).

### matrix-exp-qsp (cross-domain alias: `e^A-via-QSP`, `qubitized-exponential`, `Hamiltonian-sim-as-QSP`)
**Domain:** Quantum Computing
**Definition:** Use QSVT polynomial approximation of e^{ix} (Jacobi-Anger) to apply e^{iAt} given block encoding of A.
**Atom or composite:** Composite: block encoding + QSVT with Jacobi-Anger coefficients.
**Cost model:** Optimal O(αt + log(1/ε)) queries — matches lower bound.
**Real wall?** No — but α inflates query cost.
**Cross-domain wiring:** linear-algebra-matrix: matrix exponential. signal-processing-rf: frequency response of polynomial filter. ml-training: continuous-time dynamics for neural ODEs.
**Notes:** Low-Chuang (2017). The reason qubitization "wins" Hamiltonian simulation.

### dense-hamiltonian-sim (cross-domain alias: `Childs-Kothari-Somma`, `dense-Hsim`, `LCU-for-dense-H`)
**Domain:** Quantum Computing
**Definition:** For dense H specified by membership oracle, simulate via LCU+block encoding with cost O(N^{1/2}·polylog) where N is dimension.
**Atom or composite:** Composite — generic LCU/QSVT.
**Cost model:** O(N^{1/2}·t·polylog(1/ε)) — quadratic speedup over O(N) reads.
**Real wall?** Requires oracle access to entries.
**Cross-domain wiring:** linear-algebra-matrix: dense matrix simulation. signal-processing-rf: dense filter realization. ml-training: dense-kernel sim.
**Notes:** Wang (2017); Berry-Childs-Kothari (2017). Dense regime complement to sparse simulation.

---

## Quantum Information Theory

### von-neumann-entropy (cross-domain alias: `S(ρ)`, `quantum-entropy`, `vN-entropy`)
**Domain:** Quantum Computing
**Definition:** S(ρ) = −Tr(ρ log ρ) = −Σ λ_i log λ_i where λ_i are eigenvalues of ρ. Quantifies mixedness; 0 for pure states.
**Atom or composite:** Atom — fundamental information quantity.
**Cost model:** Classical: O(d³) eigendecomp. Quantum estimation: O(1/ε²) via swap test or shadows.
**Real wall?** No — but computing exactly requires full ρ.
**Cross-domain wiring:** information-theory-coding: quantum analogue of Shannon entropy. statistics-probability: entropy of eigenvalue distribution. linear-algebra-matrix: spectral functional.
**Notes:** von Neumann (1932). Operational meaning: optimal compression rate (Schumacher).

### quantum-relative-entropy (cross-domain alias: `S(ρ‖σ)`, `quantum-KL`, `Kullback-Leibler-quantum`)
**Domain:** Quantum Computing
**Definition:** S(ρ‖σ) = Tr(ρ log ρ) − Tr(ρ log σ), if supp(ρ)⊆supp(σ); ∞ otherwise. Monotone under CPTP maps (data-processing inequality).
**Atom or composite:** Atom — divergence measure.
**Cost model:** Quantum hypothesis testing achieves error exp(−n·S) (Quantum Stein's Lemma).
**Real wall?** Not symmetric; not a metric.
**Cross-domain wiring:** information-theory-coding: KL-divergence quantum analogue. statistics-probability: hypothesis testing rate. ml-training: variational bounds on free energy.
**Notes:** Umegaki (1962); Hiai-Petz. Foundation of quantum hypothesis testing.

### quantum-conditional-entropy (cross-domain alias: `S(A|B)`, `conditional-vN-entropy`, `negative-entropy`)
**Domain:** Quantum Computing
**Definition:** S(A|B) = S(AB) − S(B). Can be NEGATIVE for entangled states (no classical analogue!).
**Atom or composite:** Composite — combination of joint and marginal entropies.
**Cost model:** Same as von Neumann entropy computation per term.
**Real wall?** No — negativity is operational (state merging).
**Cross-domain wiring:** information-theory-coding: cf. classical H(X|Y)≥0. statistics-probability: conditional uncertainty. linear-algebra-matrix: subsystem trace.
**Notes:** Horodecki-Oppenheim-Winter (2005): negative conditional entropy = entanglement currency for state merging.

### quantum-mutual-information (cross-domain alias: `I(A:B)`, `qMI`, `total-correlations`)
**Domain:** Quantum Computing
**Definition:** I(A:B) = S(A) + S(B) − S(AB). Captures total (classical+quantum) correlations between A and B.
**Atom or composite:** Composite — entropy combination.
**Cost model:** Three entropy computations.
**Real wall?** No — upper-bounded by 2·log(min(d_A,d_B)).
**Cross-domain wiring:** information-theory-coding: cf. I(X:Y) classical. statistics-probability: mutual dependence. ml-training: InfoMax learning.
**Notes:** Equals classical MI for separable states; 2× classical for maximally entangled.

### coherent-information (cross-domain alias: `Ic(A⟩B)`, `−S(A|B)`, `Schumacher-Nielsen`)
**Domain:** Quantum Computing
**Definition:** I_c(A⟩B) = S(B) − S(AB) = −S(A|B). Quantum analogue of mutual information; gives quantum channel capacity for some channels.
**Atom or composite:** Composite.
**Cost model:** Entropy estimation.
**Real wall?** Not additive in general — superadditivity gives nonadditive channel capacities.
**Cross-domain wiring:** information-theory-coding: hashing bound for entanglement distillation. statistics-probability: signed information measure. linear-algebra-matrix: spectral functional difference.
**Notes:** Schumacher-Nielsen (1996). LSD theorem: quantum capacity = regularized coherent information.

### entanglement-of-formation (cross-domain alias: `EoF`, `formation-entropy`, `Bennett-DiVincenzo-Smolin-Wootters`)
**Domain:** Quantum Computing
**Definition:** E_F(ρ) = min Σ p_i S(Tr_B|ψ_i⟩⟨ψ_i|) over decompositions ρ=Σ p_i|ψ_i⟩⟨ψ_i|. Quantifies cost in EPR pairs.
**Atom or composite:** Composite — convex roof over ensemble decompositions.
**Cost model:** NP-hard in general; closed form for two qubits (concurrence).
**Real wall?** Hard convex optimization for general states.
**Cross-domain wiring:** linear-algebra-matrix: convex roof construction. information-theory-coding: resource cost. ml-training: optimization-over-decompositions.
**Notes:** Bennett, DiVincenzo, Smolin, Wootters (1996). Wootters formula gives closed-form 2-qubit case.

### distillable-entanglement (cross-domain alias: `E_D`, `distillation-rate`, `LOCC-EPR-yield`)
**Domain:** Quantum Computing
**Definition:** Asymptotic rate of EPR pairs extractable from copies of ρ under LOCC. Lower bound: 1-way distillation rate ≥ coherent information.
**Atom or composite:** Composite — asymptotic resource quantity.
**Cost model:** Open-problem to compute generally.
**Real wall?** Bound entanglement: ρ entangled but E_D=0.
**Cross-domain wiring:** information-theory-coding: operational resource theory. statistics-probability: asymptotic rate. cryptography-advanced: usable entanglement for QKD.
**Notes:** Bennett, Bernstein, Popescu, Schumacher (1996). Defines resource theory of entanglement.

### squashed-entanglement (cross-domain alias: `E_sq`, `Christandl-Winter`, `intrinsic-entanglement`)
**Domain:** Quantum Computing
**Definition:** E_sq(ρ_{AB}) = (1/2) inf I(A:B|E) over extensions ρ_{ABE} with Tr_E = ρ_{AB}. Faithful, monogamous entanglement measure.
**Atom or composite:** Composite — infimum over purifications.
**Cost model:** Generally uncomputable; satisfies many natural axioms.
**Real wall?** Infimum is hard.
**Cross-domain wiring:** information-theory-coding: intrinsic info analogue. statistics-probability: conditional MI minimization. cryptography-advanced: secret-key rate bound.
**Notes:** Christandl, Winter (2004). Monogamy = useful upper bound on distillable entanglement.

### negativity (cross-domain alias: `N(ρ)`, `partial-transpose-negativity`, `Vidal-Werner`)
**Domain:** Quantum Computing
**Definition:** N(ρ) = (‖ρ^{T_B}‖₁ − 1)/2 = sum of absolute values of negative eigenvalues of partial transpose. Witnesses NPT entanglement.
**Atom or composite:** Composite — spectral functional on partial transpose.
**Cost model:** O(d³) eigendecomposition.
**Real wall?** Misses PPT entanglement (bound entanglement undetected).
**Cross-domain wiring:** linear-algebra-matrix: partial transpose spectrum. information-theory-coding: entanglement monotone. statistics-probability: tail of negative spectrum.
**Notes:** Vidal-Werner (2002). Easy-to-compute monotone; widely used in many-body physics.

### log-negativity (cross-domain alias: `E_N`, `logarithmic-negativity`, `Plenio`)
**Domain:** Quantum Computing
**Definition:** E_N(ρ) = log₂‖ρ^{T_B}‖₁. Upper bound on distillable entanglement; additive, computable.
**Atom or composite:** Composite — log of trace norm of partial transpose.
**Cost model:** Same as negativity + log.
**Real wall?** Not faithful — log-negativity zero ⇏ separable for PPT bound entanglement.
**Cross-domain wiring:** information-theory-coding: distillable-entanglement bound. linear-algebra-matrix: matrix log/norm. statistics-probability: log-Schatten functional.
**Notes:** Plenio (2005). De-facto standard entanglement measure in numerical many-body simulations.

### concurrence (cross-domain alias: `C(ρ)`, `Wootters-concurrence`, `2-qubit-entanglement`)
**Domain:** Quantum Computing
**Definition:** For 2-qubit ρ: C = max(0, λ_1 − λ_2 − λ_3 − λ_4), λ_i eigenvalues (decreasing) of √(√ρ·ρ̃·√ρ), ρ̃=(σ_y⊗σ_y)ρ*(σ_y⊗σ_y).
**Atom or composite:** Atom — closed form for 2-qubit entanglement.
**Cost model:** O(1) eigendecomp of 4×4 matrix.
**Real wall?** Only 2-qubit; generalizations are partial.
**Cross-domain wiring:** linear-algebra-matrix: spin-flip operator. information-theory-coding: monotone for 2-qubit case. statistics-probability: rank-1 to rank-4 transition.
**Notes:** Wootters (1998). Gives E_F via h(½+½√(1−C²)).

### schmidt-decomposition (cross-domain alias: `Schmidt-form`, `bipartite-SVD`, `canonical-bipartite-form`)
**Domain:** Quantum Computing
**Definition:** |ψ⟩_{AB} = Σ_k √(p_k)|a_k⟩|b_k⟩ with p_k ≥ 0, Σp_k=1, ⟨a_k|a_l⟩=δ_{kl}.
**Atom or composite:** Composite — SVD of coefficient matrix.
**Cost model:** O(d_A·d_B·min(d_A,d_B)) classical SVD.
**Real wall?** Bipartite only — multipartite analogue (HOSVD) is not unique.
**Cross-domain wiring:** linear-algebra-matrix: SVD of bipartite state. signal-processing-rf: rank-revealing decomposition. ml-training: low-rank state factorization.
**Notes:** Schmidt (1907) — Schmidt rank = entanglement rank. Backbone of MPS/tensor networks.

### schmidt-rank (cross-domain alias: `entanglement-rank`, `bipartite-rank`, `χ-bond-dimension`)
**Domain:** Quantum Computing
**Definition:** Number of nonzero p_k in Schmidt decomposition. Rank-1 = separable; max rank = maximally entangled.
**Atom or composite:** Atom — integer invariant.
**Cost model:** Counting nonzero singular values.
**Real wall?** Robust generalization: ε-Schmidt rank.
**Cross-domain wiring:** linear-algebra-matrix: matrix rank. ml-training: low-rank model. signal-processing-rf: number of independent modes.
**Notes:** Foundational for matrix product states (MPS) — bond dimension = Schmidt rank across each cut.

### entanglement-spectrum (cross-domain alias: `Li-Haldane-spectrum`, `pseudo-Hamiltonian-spectrum`, `−log-eigenvalues`)
**Domain:** Quantum Computing
**Definition:** {ξ_k = −log p_k} where p_k are Schmidt coefficients squared. Spectrum of "entanglement Hamiltonian" H_E = −log ρ_A.
**Atom or composite:** Composite — spectrum of reduced density matrix.
**Cost model:** Eigendecomp of ρ_A.
**Real wall?** Captures topological phases of matter (Li-Haldane).
**Cross-domain wiring:** physics-condensed-matter: topological phase fingerprint. linear-algebra-matrix: spectrum of partial trace. information-theory-coding: full distribution vs scalar entropy.
**Notes:** Li, Haldane (2008). More info than entanglement entropy alone.

### purity (cross-domain alias: `Tr(ρ²)`, `state-purity`, `inverse-participation-ratio`)
**Domain:** Quantum Computing
**Definition:** γ(ρ) = Tr(ρ²) ∈ [1/d, 1]. =1 for pure states, =1/d for maximally mixed.
**Atom or composite:** Atom — Hilbert-Schmidt norm squared.
**Cost model:** O(d²) classical; O(1/ε²) via swap-test or shadows.
**Real wall?** Insensitive to which mixed state.
**Cross-domain wiring:** linear-algebra-matrix: Frobenius norm squared. statistics-probability: collision probability. signal-processing-rf: signal energy.
**Notes:** Trivial via two-copy swap test ⟨SWAP⟩ = Tr(ρ²).

### renyi-entropy (cross-domain alias: `S_α`, `α-Renyi-entropy`, `quantum-Rényi`)
**Domain:** Quantum Computing
**Definition:** S_α(ρ) = (1/(1−α)) log Tr(ρ^α) for α>0,≠1. Limits: α→1 gives von Neumann; α=2 gives −log purity.
**Atom or composite:** Atom — one-parameter family.
**Cost model:** α-th moment via α copies (swap-network).
**Real wall?** Different α probe different aspects of spectrum.
**Cross-domain wiring:** statistics-probability: Rényi divergences. information-theory-coding: smooth entropy framework. ml-training: regularization parameters.
**Notes:** Rényi (1961) classical, quantum extension via Petz / sandwiched variants.

### holevo-information (cross-domain alias: `χ-quantity`, `Holevo-bound`, `accessible-information-bound`)
**Domain:** Quantum Computing
**Definition:** χ({p_i, ρ_i}) = S(Σp_i ρ_i) − Σp_i S(ρ_i). Upper bounds the classical information extractable from quantum ensemble.
**Atom or composite:** Composite — entropy of mixture minus average entropy.
**Cost model:** Entropy of ensemble.
**Real wall?** Achievable asymptotically (Holevo-Schumacher-Westmoreland).
**Cross-domain wiring:** information-theory-coding: classical capacity of quantum channels. statistics-probability: maximum mutual info via measurement. ml-training: information bottleneck-like bound.
**Notes:** Holevo (1973). Foundation of classical-over-quantum communication.

### fidelity (cross-domain alias: `F(ρ,σ)`, `Bures-fidelity`, `Uhlmann-fidelity`)
**Domain:** Quantum Computing
**Definition:** F(ρ,σ) = (Tr√(√ρ·σ·√ρ))². For pure states: F = |⟨ψ|φ⟩|². Range [0,1].
**Atom or composite:** Atom — closeness measure.
**Cost model:** O(d³); swap-test estimates pure-state fidelity O(1/ε²).
**Real wall?** No.
**Cross-domain wiring:** linear-algebra-matrix: matrix geometric mean. statistics-probability: Bhattacharyya coefficient analogue. signal-processing-rf: correlation coefficient.
**Notes:** Uhlmann (1976). Operational: max overlap of purifications.

### trace-distance (cross-domain alias: `T(ρ,σ)`, `quantum-TV-distance`, `Schatten-1-distance`)
**Domain:** Quantum Computing
**Definition:** D(ρ,σ) = ½‖ρ−σ‖₁ = ½ Σ|λ_i| of (ρ−σ). Operational: max distinguishability with single-shot measurement.
**Atom or composite:** Atom — metric on states.
**Cost model:** O(d³) eigendecomp.
**Real wall?** Sublinear estimation requires shadows or special structure.
**Cross-domain wiring:** statistics-probability: total variation distance analogue. information-theory-coding: hypothesis-testing error. linear-algebra-matrix: trace norm.
**Notes:** Fuchs-van de Graaf inequalities link D and F.

---

## Quantum Channels

### kraus-representation (cross-domain alias: `operator-sum`, `Kraus-operators`, `OSR`)
**Domain:** Quantum Computing
**Definition:** Any CPTP map ℰ has form ℰ(ρ) = Σ_k K_k ρ K_k† with Σ K_k†K_k = I. Kraus operators not unique (unitary freedom).
**Atom or composite:** Atom — representation of channels.
**Cost model:** ≤ d² Kraus operators needed.
**Real wall?** No — universal representation.
**Cross-domain wiring:** linear-algebra-matrix: operator-sum decomposition. signal-processing-rf: linear systems with multiple paths. statistics-probability: mixture of Hilbert-space maps.
**Notes:** Kraus (1971), Choi (1975). Operational meaning: outcomes of environment measurement.

### stinespring-dilation (cross-domain alias: `unitary-dilation-of-channel`, `purification-of-CPTP`, `system-environment-form`)
**Domain:** Quantum Computing
**Definition:** Any CPTP ℰ: B(H_S) → B(H_S) can be written ℰ(ρ) = Tr_E[U(ρ⊗|0⟩⟨0|_E)U†] with isometry U.
**Atom or composite:** Composite — channel as partial trace of unitary evolution.
**Cost model:** Environment dim ≤ d² (Kraus rank).
**Real wall?** Implementing on hardware requires ancilla qubits.
**Cross-domain wiring:** linear-algebra-matrix: Naimark-style dilation. signal-processing-rf: open system embedded in closed system. physics: thermodynamic bath model.
**Notes:** Stinespring (1955). Foundational theorem of completely positive maps.

### complete-positivity (cross-domain alias: `CP-map`, `tensor-positive-preserving`, `CP-condition`)
**Domain:** Quantum Computing
**Definition:** Map ℰ is CP if ℰ⊗I_n is positive for all n. Captures requirement that ℰ on subsystem preserves positivity globally.
**Atom or composite:** Atom — property of linear maps.
**Cost model:** Verified via Choi matrix positivity.
**Real wall?** Positive-but-not-CP maps (e.g., transposition) are unphysical as channels.
**Cross-domain wiring:** linear-algebra-matrix: positive semidefinite preservation. statistics-probability: stochastic-map analogue. signal-processing-rf: passive system constraint.
**Notes:** Stinespring (1955), Choi (1975). Cornerstone of quantum channels.

### depolarizing-channel (cross-domain alias: `D_p`, `white-noise-channel`, `random-Pauli-channel`)
**Domain:** Quantum Computing
**Definition:** ℰ(ρ) = (1−p)ρ + p·(I/d). For qubits: equivalent to applying X,Y,Z each with prob p/3, identity with prob 1−p.
**Atom or composite:** Composite — Pauli channel.
**Cost model:** Single-qubit Kraus rank 4.
**Real wall?** Worst-case noise — useful baseline.
**Cross-domain wiring:** information-theory-coding: erasure-like channel. signal-processing-rf: AWGN analogue. statistics-probability: uniform mixing.
**Notes:** Standard noise model in fault-tolerance threshold analyses.

### amplitude-damping (cross-domain alias: `T1-channel`, `spontaneous-emission`, `relaxation-channel`)
**Domain:** Quantum Computing
**Definition:** ℰ(ρ) = K_0 ρ K_0† + K_1 ρ K_1†, K_0=diag(1,√(1−γ)), K_1=√γ·|0⟩⟨1|. Models T1 decay.
**Atom or composite:** Composite — non-unital channel.
**Cost model:** Two Kraus operators.
**Real wall?** Physical — energy relaxation in real hardware.
**Cross-domain wiring:** physics: spontaneous emission. signal-processing-rf: lossy resonator. information-theory-coding: erasure-like.
**Notes:** Models energy relaxation. Generalized amplitude damping extends to finite-T environment.

### phase-damping (cross-domain alias: `T2-channel`, `pure-dephasing`, `Z-noise`)
**Domain:** Quantum Computing
**Definition:** ℰ(ρ) = (1−λ/2)ρ + (λ/2)·ZρZ. Equivalent to applying Z with prob λ/2 — loss of off-diagonal coherence.
**Atom or composite:** Atom — Pauli-Z subchannel.
**Cost model:** Single-qubit Kraus rank 2.
**Real wall?** Physical — environmental dephasing.
**Cross-domain wiring:** physics: random phase noise. signal-processing-rf: phase noise / jitter. statistics-probability: Bernoulli noise on phases.
**Notes:** Often dominant noise in superconducting qubits.

### generalized-amplitude-damping (cross-domain alias: `GAD`, `finite-T-damping`, `thermal-channel`)
**Domain:** Quantum Computing
**Definition:** Amplitude damping with thermal background: equilibrates qubit to thermal state with population p_∞ rather than ground state.
**Atom or composite:** Composite — 4 Kraus operators.
**Real wall?** Models qubits at finite temperature.
**Cross-domain wiring:** physics: thermal bath coupling. statistics-probability: detailed balance. signal-processing-rf: thermal noise model.
**Notes:** Important for superconducting qubits at 10–30 mK with non-negligible thermal photons.

### pauli-channel (cross-domain alias: `random-Pauli`, `Pauli-twirling-channel`, `weighted-Pauli-mix`)
**Domain:** Quantum Computing
**Definition:** ℰ(ρ) = Σ_{P∈{I,X,Y,Z}} p_P · P ρ P. Parametrized by probability simplex.
**Atom or composite:** Composite — convex combination of Paulis.
**Cost model:** Diagonal in Pauli basis → easy to analyze.
**Real wall?** Pauli twirling reduces general noise to Pauli channel.
**Cross-domain wiring:** quantum-error-correction: standard noise model. statistics-probability: discrete probability on group. information-theory-coding: classical Pauli error correction.
**Notes:** Twirling under Clifford group makes any channel Pauli; key tool in benchmarking.

### erasure-channel (cross-domain alias: `erasure-flag-channel`, `loss-channel`, `Knill-Laflamme-Zurek`)
**Domain:** Quantum Computing
**Definition:** With prob 1−p: identity; with prob p: output is flag |e⟩ orthogonal to qubit Hilbert space (erasure detected).
**Atom or composite:** Composite — detected loss.
**Real wall?** Photon loss in linear-optical QC is erasure-like.
**Cross-domain wiring:** information-theory-coding: erasure code recovery. signal-processing-rf: known dropouts. statistics-probability: censored data.
**Notes:** Easier to correct than depolarizing — error location known.

### twirling (cross-domain alias: `Pauli-twirling`, `Clifford-twirling`, `Haar-twirling`)
**Domain:** Quantum Computing
**Definition:** Symmetrize channel via averaging: ℰ_twirl(ρ) = ∫dU U†·ℰ(UρU†)·U. Pauli twirl → Pauli channel; Clifford twirl → depolarizing.
**Atom or composite:** Composite — group-average over noise.
**Cost model:** O(|G|) overhead, or random sampling from G.
**Real wall?** No — but reduces noise structure (loses coherent errors).
**Cross-domain wiring:** statistics-probability: group averaging / Haar measure. signal-processing-rf: noise whitening. quantum-error-correction: simplifies decoder analysis.
**Notes:** Foundation of randomized compiling, randomized benchmarking.

### choi-jamiolkowski-iso (cross-domain alias: `CJ-iso`, `channel-state-duality`, `Choi-matrix`)
**Domain:** Quantum Computing
**Definition:** Map ℰ ↔ state C(ℰ) = (I⊗ℰ)(|Φ⁺⟩⟨Φ⁺|) where |Φ⁺⟩ is maximally entangled. CP iff C(ℰ)≥0, TP iff Tr_B(C)=I/d.
**Atom or composite:** Composite — bijection between channels and bipartite states.
**Cost model:** d²×d² Choi matrix encodes whole channel.
**Real wall?** No.
**Cross-domain wiring:** linear-algebra-matrix: vectorization isomorphism. information-theory-coding: state-based channel analysis. ml-training: matrix-form processing.
**Notes:** Jamiolkowski (1972), Choi (1975). Used to test channel properties via state properties.

### holevo-capacity (cross-domain alias: `χ-capacity`, `product-state-classical-capacity`, `C₁`)
**Domain:** Quantum Computing
**Definition:** C₁(ℰ) = max_{ensemble} χ({p_i, ℰ(ρ_i)}). Single-letter classical capacity using product-state encodings.
**Atom or composite:** Composite — optimization of Holevo info.
**Cost model:** Non-convex; HSW theorem gives operational meaning.
**Real wall?** Not equal to full classical capacity due to superadditivity (Hastings).
**Cross-domain wiring:** information-theory-coding: Shannon-capacity analogue. statistics-probability: optimal ensemble distribution. ml-training: representation capacity bound.
**Notes:** Holevo-Schumacher-Westmoreland (1996-98). Hastings (2009) showed C = lim (1/n) C₁(ℰ⊗ⁿ) > C₁ in general.

### lsd-quantum-capacity (cross-domain alias: `Q(ℰ)`, `Lloyd-Shor-Devetak`, `regularized-coherent-info`)
**Domain:** Quantum Computing
**Definition:** Q(ℰ) = lim_{n→∞} (1/n) max_ρ I_c(ρ; ℰ⊗ⁿ). Regularized coherent information = quantum capacity.
**Atom or composite:** Composite — limit of coherent information.
**Cost model:** Regularization makes computation intractable in general.
**Real wall?** Superadditive — Q can be positive when single-letter Q₁ is zero.
**Cross-domain wiring:** information-theory-coding: quantum analogue of Shannon capacity. statistics-probability: asymptotic rate. linear-algebra-matrix: spectral optimization.
**Notes:** Lloyd (1997), Shor (2002), Devetak (2005). Active research area.

### private-capacity (cross-domain alias: `P(ℰ)`, `secret-key-capacity`, `Devetak-private`)
**Domain:** Quantum Computing
**Definition:** Rate of classical bits that can be transmitted over ℰ with secrecy against environment eavesdropper. P = regularized private information.
**Atom or composite:** Composite — regularized.
**Cost model:** Intractable in general.
**Real wall?** Generally Q ≤ P ≤ C; can be strictly less than classical capacity.
**Cross-domain wiring:** cryptography-advanced: QKD rate bound. information-theory-coding: Wyner wiretap analogue. statistics-probability: eavesdropper-marginalized rate.
**Notes:** Devetak (2005). Crucial for QKD over noisy channels.

---

## Measurement Formalism

### povm (cross-domain alias: `positive-operator-valued-measure`, `generalized-measurement`, `effects`)
**Domain:** Quantum Computing
**Definition:** Set {E_k} of PSD operators with Σ_k E_k = I. Probability of outcome k: p(k|ρ) = Tr(E_k ρ). Generalizes projective measurement.
**Atom or composite:** Atom — measurement formalism.
**Cost model:** Implemented via Naimark dilation: ancilla + projective measurement.
**Real wall?** No — strictly more general than PVM (e.g., for unambiguous state discrimination).
**Cross-domain wiring:** statistics-probability: positive measures forming POI. linear-algebra-matrix: PSD operator partition. signal-processing-rf: detection with multiple outcomes.
**Notes:** Davies, Lewis (1970). Operational extension of measurement; foundation of quantum statistics.

### naimark-dilation (cross-domain alias: `Neumark-extension`, `POVM-to-PVM`, `ancilla-projection`)
**Domain:** Quantum Computing
**Definition:** Any POVM on H can be realized as PVM on H⊗H_anc with appropriate isometry. Required ancilla dim ≤ number of POVM elements.
**Atom or composite:** Composite — dilation + projective measurement.
**Cost model:** O(log K) ancilla qubits for K-outcome POVM.
**Real wall?** No — universal construction.
**Cross-domain wiring:** linear-algebra-matrix: positive-form to projection lifting. signal-processing-rf: oversampling to orthogonalize. statistics-probability: enlarging sample space for independence.
**Notes:** Neumark (1940). Practical recipe to implement any POVM.

### weak-measurement (cross-domain alias: `AAV-measurement`, `Aharonov-Albert-Vaidman`, `partial-collapse`)
**Domain:** Quantum Computing
**Definition:** Weakly couple system to ancilla then strongly measure ancilla; minimally disturbs system. Yields "weak value" ⟨φ|A|ψ⟩/⟨φ|ψ⟩ when pre/post selecting.
**Atom or composite:** Composite — coupling + projective measurement of meter.
**Cost model:** Small coupling g → many repetitions to average.
**Real wall?** Weak value can exceed operator spectrum (amplification).
**Cross-domain wiring:** statistics-probability: noisy estimator. signal-processing-rf: weak signal extraction. physics: foundational paradoxes.
**Notes:** Aharonov, Albert, Vaidman (1988). Used in quantum metrology, foundations experiments.

### qnd-measurement (cross-domain alias: `quantum-non-demolition`, `QND`, `non-disturbing-readout`)
**Domain:** Quantum Computing
**Definition:** Measurement that doesn't disturb eigenstates of measured observable: [H_int, A]=0. Repeated QND gives same outcome.
**Atom or composite:** Composite — engineered Hamiltonian coupling.
**Cost model:** Hardware-specific; longitudinal coupling preferred.
**Real wall?** Photon shot noise, finite QND infidelity.
**Cross-domain wiring:** signal-processing-rf: non-perturbative sensing. physics: Heisenberg-limited metrology. quantum-error-correction: stabilizer measurement is QND.
**Notes:** Braginsky (1970s). Essential for repeated stabilizer measurement in QEC.

### indirect-measurement (cross-domain alias: `meter-measurement`, `von-Neumann-scheme`, `ancilla-coupled-readout`)
**Domain:** Quantum Computing
**Definition:** Couple system to meter via U, measure meter projectively. Recovers POVM on system. Foundational von Neumann measurement model.
**Atom or composite:** Composite — coupling + meter measurement.
**Cost model:** Ancilla overhead + coupling unitary.
**Real wall?** Coupling strength determines back-action.
**Cross-domain wiring:** physics: pointer states. signal-processing-rf: sensor + readout. quantum-error-correction: syndrome extraction.
**Notes:** von Neumann (1932). Universal recipe — strong coupling = projective; weak = AAV.

### postselection (cross-domain alias: `conditional-on-outcome`, `heralded-measurement`, `accept-or-reject`)
**Domain:** Quantum Computing
**Definition:** Keep only runs where measurement returned target outcome. Renormalized state: ρ→E_k ρ E_k†/Tr(E_k ρ E_k†).
**Atom or composite:** Atom — classical conditioning.
**Cost model:** O(1/p) trial overhead for postselection probability p.
**Real wall?** Exponential overhead if p decays exponentially.
**Cross-domain wiring:** statistics-probability: conditioning on event. signal-processing-rf: heralding. cryptography-advanced: rejection sampling.
**Notes:** PostBQP = PP (Aaronson) — postselection gives huge classical complexity boost.

### quantum-erasure (cross-domain alias: `delayed-choice-erasure`, `Scully-Drühl`, `which-path-erasure`)
**Domain:** Quantum Computing
**Definition:** "Erase" which-path information by measuring entangled marker in conjugate basis; restores interference in sub-ensemble.
**Atom or composite:** Composite — entanglement + postselected measurement.
**Real wall?** Doesn't allow superluminal signaling (no-signaling).
**Cross-domain wiring:** signal-processing-rf: phase-coherent restoration. physics: complementarity. cryptography-advanced: basis-choice security.
**Notes:** Scully, Drühl (1982). Foundational interpretive experiment.

### unambiguous-state-discrimination (cross-domain alias: `USD`, `Ivanovic-Dieks-Peres`, `error-free-but-inconclusive`)
**Domain:** Quantum Computing
**Definition:** Discriminate non-orthogonal |ψ_1⟩,|ψ_2⟩ with NO errors but allow inconclusive outcome ‘?’. Optimal success: 1−|⟨ψ_1|ψ_2⟩|.
**Atom or composite:** Composite — 3-outcome POVM.
**Cost model:** Single round.
**Real wall?** Cannot make success probability 1 for non-orthogonal states.
**Cross-domain wiring:** statistics-probability: hypothesis test with abstain. cryptography-advanced: B92 protocol attacks. signal-processing-rf: error-free demod.
**Notes:** Ivanovic (1987), Dieks (1988), Peres (1988). Backbone of B92 QKD.

---

## Quantum Tomography

### state-tomography-linear-inversion (cross-domain alias: `linear-inversion-tomography`, `LIT`, `direct-tomography`)
**Domain:** Quantum Computing
**Definition:** Reconstruct ρ from frequencies f_i ≈ Tr(M_i ρ) by inverting linear map. Estimator: ρ̂ = Σ_i f_i · D_i where D_i is dual basis.
**Atom or composite:** Composite — measurement + linear algebra.
**Cost model:** O(d²) measurement settings for full tomography.
**Real wall?** ρ̂ may not be PSD; finite samples → unphysical estimates.
**Cross-domain wiring:** linear-algebra-matrix: matrix inversion. statistics-probability: method of moments. signal-processing-rf: parameter estimation.
**Notes:** Classical inversion. Forms basis but typically followed by projection onto density matrices.

### state-tomography-mle (cross-domain alias: `MLE-tomography`, `Hradil-MLE`, `maximum-likelihood-tomography`)
**Domain:** Quantum Computing
**Definition:** Maximize ∏_i Tr(M_i ρ)^{n_i} over PSD ρ with Tr ρ=1. Guarantees physical estimator.
**Atom or composite:** Composite — measurement + constrained MLE.
**Cost model:** Convex optimization in O(d²) parameters; iterative algorithm (R-rho-R).
**Real wall?** Bias for low-eigenvalue components ("zero-eigenvalue catastrophe").
**Cross-domain wiring:** statistics-probability: MLE on density matrix. linear-algebra-matrix: PSD constrained optim. ml-training: likelihood maximization.
**Notes:** Hradil (1997). Standard method for state characterization.

### process-tomography (cross-domain alias: `QPT`, `channel-tomography`, `chi-matrix-tomography`)
**Domain:** Quantum Computing
**Definition:** Reconstruct channel ℰ by preparing basis of input states, measuring outputs, solving for χ in ℰ(ρ)=Σ χ_{mn} E_m ρ E_n†.
**Atom or composite:** Composite — state-tomography on Choi state.
**Cost model:** O(d⁴) measurements for d×d channel.
**Real wall?** SPAM (state-prep and measurement) errors confound channel errors.
**Cross-domain wiring:** linear-algebra-matrix: tensor reconstruction. signal-processing-rf: black-box system identification. ml-training: system ID.
**Notes:** Poyatos, Cirac, Zoller (1997); Chuang, Nielsen (1997). Superseded by GST when SPAM matters.

### gate-set-tomography (cross-domain alias: `GST`, `self-consistent-tomography`, `Nielsen-Stark-Blume-Kohout`)
**Domain:** Quantum Computing
**Definition:** Simultaneously fit gates + state-prep + measurement to long sequences of gates; self-consistently identifies entire gate set.
**Atom or composite:** Composite — global MLE over gate set + SPAM.
**Cost model:** Thousands to millions of circuit executions; massive classical post-processing.
**Real wall?** Gauge freedom (set of gates equivalent under similarity transform).
**Cross-domain wiring:** statistics-probability: hierarchical model fit. ml-training: large-scale optimization. signal-processing-rf: system identification with unknown probe.
**Notes:** Blume-Kohout et al. (2013, 2017). Most precise calibration tool, but expensive.

### randomized-benchmarking (cross-domain alias: `RB`, `Knill-RB`, `Magesan-RB`)
**Domain:** Quantum Computing
**Definition:** Run random Clifford sequences of length m, fit average survival F(m)=A·p^m+B. Average gate fidelity F_avg = 1 − (d−1)(1−p)/d.
**Atom or composite:** Composite — random Clifford sequences + curve fit.
**Cost model:** ~100 sequences × 10 lengths × 100 shots.
**Real wall?** SPAM-immune; doesn't characterize gate-dependent errors.
**Cross-domain wiring:** statistics-probability: exponential decay fit. signal-processing-rf: averaging filter. quantum-error-correction: estimates threshold-relevant fidelity.
**Notes:** Knill et al. (2008); Magesan-Gambetta-Emerson (2011). Industry standard for gate fidelity.

### direct-fidelity-estimation (cross-domain alias: `DFE`, `Flammia-Liu`, `Pauli-sampling-fidelity`)
**Domain:** Quantum Computing
**Definition:** Estimate ⟨ψ|ρ|ψ⟩ by sampling Pauli operators with prob proportional to |⟨ψ|P|ψ⟩|² and measuring their expectations.
**Atom or composite:** Composite — importance sampling over Paulis.
**Cost model:** O(1/ε²) Pauli measurements — independent of dimension for stabilizer states.
**Real wall?** Pauli expectation must be classically computable.
**Cross-domain wiring:** statistics-probability: importance sampling. signal-processing-rf: matched filter on Pauli basis. ml-training: kernel estimation.
**Notes:** Flammia & Liu (2011). Exponentially cheaper than full tomography for fidelity-only questions.

### classical-shadows (cross-domain alias: `Huang-Kueng-Preskill`, `shadow-tomography`, `randomized-measurement-CS`)
**Domain:** Quantum Computing
**Definition:** Apply random Clifford U, measure in computational basis to get b; ρ̂ = U†|b⟩⟨b|U inverted via shadow channel. Predict many observables from one shadow set.
**Atom or composite:** Composite — randomized measurement + median-of-means.
**Cost model:** O(log M / ε²) samples to predict M expectations.
**Real wall?** Exponential variance for nonlocal observables in random Pauli shadow.
**Cross-domain wiring:** statistics-probability: u-statistic, median-of-means. linear-algebra-matrix: random projection. ml-training: feature-importance estimation.
**Notes:** Huang, Kueng, Preskill (2020). Game-changer for many-body verification.

### compressed-sensing-tomography (cross-domain alias: `low-rank-tomography`, `Gross-Liu-Flammia-Becker-Eisert`, `matrix-completion-QT`)
**Domain:** Quantum Computing
**Definition:** If ρ has rank r ≪ d, reconstruct from O(r·d·log d) Pauli measurements via nuclear-norm minimization.
**Atom or composite:** Composite — compressed sensing on density matrix.
**Cost model:** Polynomial in r and log d.
**Real wall?** Requires low-rank prior.
**Cross-domain wiring:** signal-processing-rf: compressed sensing. linear-algebra-matrix: matrix completion. ml-training: nuclear-norm regularization.
**Notes:** Gross et al. (2010). Quantum analogue of Candès-Recht matrix completion.

---

## Quantum Chemistry

### jordan-wigner (cross-domain alias: `JW-transform`, `fermion-to-qubit`, `nonlocal-string-mapping`)
**Domain:** Quantum Computing
**Definition:** Map fermionic c_j = (Π_{k<j} Z_k)·(X_j+iY_j)/2. Encodes anti-commutation via nonlocal Pauli string.
**Atom or composite:** Atom — fermion-qubit mapping.
**Cost model:** O(N) Pauli weight per fermionic operator — long Z-strings.
**Real wall?** Long strings → high gate cost on chip with limited connectivity.
**Cross-domain wiring:** linear-algebra-matrix: change of basis fermion↔qubit. physics-condensed-matter: spin-fermion duality. signal-processing-rf: 1D string operators.
**Notes:** Jordan-Wigner (1928). Default mapping in quantum chemistry algorithms.

### bravyi-kitaev (cross-domain alias: `BK-transform`, `logarithmic-fermion-mapping`, `parity-tree`)
**Domain:** Quantum Computing
**Definition:** Encodes occupation via binary-tree partial-sum structure. Weight of each fermionic operator O(log N).
**Atom or composite:** Composite — tree-structured encoding.
**Cost model:** O(log N) Pauli weight per fermionic op.
**Real wall?** No — log-depth scaling.
**Cross-domain wiring:** information-theory-coding: tree codes. linear-algebra-matrix: linear binary code mapping. signal-processing-rf: hierarchical decomposition.
**Notes:** Bravyi-Kitaev (2002). Preferable to JW for large molecules on devices without all-to-all connectivity.

### parity-transform (cross-domain alias: `Bravyi-Kitaev-parity`, `partial-sum-encoding`, `cumulative-occupation`)
**Domain:** Quantum Computing
**Definition:** Each qubit holds parity of occupations up to its mode: p_j = Σ_{k≤j} n_k mod 2.
**Atom or composite:** Atom — encoding choice.
**Cost model:** Trade-off between JW and BK; one-qubit savings via Z₂-symmetries.
**Real wall?** No.
**Cross-domain wiring:** linear-algebra-matrix: change-of-basis. information-theory-coding: parity encoding. statistics-probability: cumulative sums.
**Notes:** Used in qubit-reduction techniques (Bravyi-Gambetta-Mezzacapo-Temme).

### ternary-tree-transform (cross-domain alias: `Jiang-Kalev-Mruczkiewicz-Miyake`, `optimal-fermion-mapping`, `TT-encoding`)
**Domain:** Quantum Computing
**Definition:** Maps fermionic ops to Pauli strings of weight ⌈log₃(2N+1)⌉. Asymptotically optimal among Pauli weight–minimal mappings.
**Atom or composite:** Composite — ternary partition structure.
**Cost model:** Pauli weight ⌈log₃(2N+1)⌉ — slightly better than BK.
**Real wall?** No.
**Cross-domain wiring:** information-theory-coding: ternary codes. linear-algebra-matrix: 3-ary tree decomposition. signal-processing-rf: 3-band decomposition.
**Notes:** Jiang et al. (2020). Pareto-optimal Pauli-weight reduction.

### second-quantization (cross-domain alias: `occupation-number-formalism`, `fermion-creation-annihilation`, `Fock-space`)
**Domain:** Quantum Computing
**Definition:** Express Hamiltonian as H = Σ h_{pq} a_p†a_q + ½ Σ h_{pqrs} a_p†a_q†a_r a_s. Operators on Fock space.
**Atom or composite:** Atom — formalism for many-body QM.
**Cost model:** O(N⁴) coefficients for electronic Hamiltonians.
**Real wall?** No — but Fock space dimension grows exponentially.
**Cross-domain wiring:** linear-algebra-matrix: operators on antisymmetric tensor product. statistics-probability: occupation number. physics-condensed-matter: foundation.
**Notes:** Dirac (1927), Fock (1932). Lingua franca of quantum chemistry.

### hartree-fock-state-prep (cross-domain alias: `HF-state`, `Slater-determinant-prep`, `Givens-rotation-network`)
**Domain:** Quantum Computing
**Definition:** Prepare Slater determinant |ψ_HF⟩ via initial product state + Givens rotation network of depth O(N) that transforms orbital basis.
**Atom or composite:** Composite — Givens rotation circuit.
**Cost model:** O(N²) two-qubit gates; depth O(N).
**Real wall?** No — exact for product of orbitals.
**Cross-domain wiring:** linear-algebra-matrix: orthogonal-transformation by Givens. signal-processing-rf: butterfly structure of rotations. ml-training: initial point for VQE.
**Notes:** Kivlichan et al. (2018). Standard starting point for variational chemistry.

### uccsd-ansatz (cross-domain alias: `unitary-CCSD`, `coupled-cluster-singles-doubles`, `chemistry-ansatz`)
**Domain:** Quantum Computing
**Definition:** |ψ(θ)⟩ = e^{T(θ)−T†(θ)}|HF⟩ with T = Σ θ_{ia} a_a†a_i + Σ θ_{ijab} a_a†a_b†a_i a_j.
**Atom or composite:** Composite — exponential of cluster operator.
**Cost model:** O(N⁴) parameters; Trotterized expansion → deep circuits.
**Real wall?** Deep circuits incompatible with NISQ coherence.
**Cross-domain wiring:** chemistry: classical coupled-cluster CCSD. linear-algebra-matrix: matrix exponential. ml-training: parameterized model for energy.
**Notes:** Peruzzo et al. (2014); McClean et al. Chemically motivated but practically expensive.

### hardware-efficient-ansatz (cross-domain alias: `HEA`, `Kandala-ansatz`, `brick-wall-ansatz`)
**Domain:** Quantum Computing
**Definition:** Layers of single-qubit rotations + native entanglers (CNOT/CZ) in brick-wall pattern. Hardware-tailored.
**Atom or composite:** Composite — repeated layers.
**Cost model:** O(L·N) parameters for L layers and N qubits.
**Real wall?** Severe barren plateaus at high depth and randomness.
**Cross-domain wiring:** ml-training: deep parameterized circuit. signal-processing-rf: tensor network ansatz. linear-algebra-matrix: parameterized unitary manifold.
**Notes:** Kandala et al. (2017). NISQ workhorse but problematic at scale.

### adapt-vqe (cross-domain alias: `ADAPT-VQE`, `Grimsley-Economou-Barnes-Mayhall`, `gradient-grown-ansatz`)
**Domain:** Quantum Computing
**Definition:** Iteratively grow ansatz by selecting operator with largest gradient ⟨ψ|[H, A_i]|ψ⟩ from pool; reoptimize all parameters.
**Atom or composite:** Composite — greedy ansatz construction.
**Cost model:** Adaptive — typically far fewer parameters than UCCSD for same accuracy.
**Real wall?** Pool selection matters; can stall on hard problems.
**Cross-domain wiring:** ml-training: greedy feature selection. signal-processing-rf: matching pursuit. statistics-probability: forward-selection regression.
**Notes:** Grimsley et al. (2019). Substantial parameter savings; "qubit-ADAPT" uses Pauli pool.

### qubit-adapt (cross-domain alias: `qubit-ADAPT-VQE`, `Pauli-pool-ADAPT`, `Tang-Shkolnikov-Barnes-Mayhall`)
**Domain:** Quantum Computing
**Definition:** ADAPT-VQE variant with operator pool = generators of e^{iθ·P} for low-weight Paulis P. Hardware-friendly.
**Atom or composite:** Composite.
**Cost model:** Fewer CNOTs than fermionic ADAPT.
**Real wall?** No.
**Cross-domain wiring:** ml-training: greedy hardware-aware feature selection. linear-algebra-matrix: Pauli generator pool. signal-processing-rf: sparse representation.
**Notes:** Tang et al. (2021). Recommended for current devices.

### k-upccgsd (cross-domain alias: `k-UpCCGSD`, `Lee-Huggins-Head-Gordon-Whaley`, `unitary-pair-coupled-cluster`)
**Domain:** Quantum Computing
**Definition:** Generalized pair-coupled cluster with k layers: e^{T_k}·...·e^{T_1}|HF⟩. Polynomial-depth approximation of UCCSD.
**Atom or composite:** Composite — k stacked pair-CC layers.
**Cost model:** O(k·N²) parameters — much fewer than full UCCSD.
**Real wall?** No — but expressivity depends on k.
**Cross-domain wiring:** chemistry: pair coupled cluster. ml-training: layered ansatz. linear-algebra-matrix: product of exponentials.
**Notes:** Lee et al. (2018). Strong expressivity / depth tradeoff.

### projective-quantum-eigensolver (cross-domain alias: `PQE`, `Stair-Evangelista`, `non-variational-eigensolver`)
**Domain:** Quantum Computing
**Definition:** Solve ⟨HF|e^{−T†}H e^T|HF⟩ + projection conditions instead of variational minimization. Like classical CCSD on quantum device.
**Atom or composite:** Composite — projection-based root-finding.
**Cost model:** Avoids barren plateaus inherent to variational landscape.
**Real wall?** No — Newton-Raphson style updates.
**Cross-domain wiring:** chemistry: CCSD residual equations. ml-training: alternative to gradient descent. linear-algebra-matrix: nonlinear system solving.
**Notes:** Stair & Evangelista (2021). Avoids barren-plateau pathology of VQE.

---

## Variational Algorithms

### ssvqe (cross-domain alias: `subspace-search-VQE`, `Nakanishi-Mitarai-Fujii`, `excited-state-VQE`)
**Domain:** Quantum Computing
**Definition:** Minimize Σ w_k ⟨ψ_k|H|ψ_k⟩ with orthogonal start states {|ψ_k⟩} and shared ansatz U(θ); recovers lowest k eigenvalues.
**Atom or composite:** Composite — VQE on weighted subspace.
**Cost model:** k VQE-circuits per iteration.
**Real wall?** Orthogonality preserved only if U is unitary (yes by construction).
**Cross-domain wiring:** linear-algebra-matrix: subspace iteration. ml-training: multi-task optimization. signal-processing-rf: parallel mode extraction.
**Notes:** Nakanishi et al. (2019). Cleanest excited-state algorithm.

### multivqe (cross-domain alias: `Multi-VQE`, `excited-state-projection`, `orthogonal-cost`)
**Domain:** Quantum Computing
**Definition:** After finding ground state |ψ_0⟩, add penalty β|⟨ψ_0|ψ⟩|² to cost; minimize for first excited.
**Atom or composite:** Composite — sequential VQE with overlap penalty.
**Cost model:** Each excited state ~ one ground-state VQE cost.
**Real wall?** Need accurate ground state first; errors compound.
**Cross-domain wiring:** linear-algebra-matrix: deflation. ml-training: regularized optimization. signal-processing-rf: orthogonal projection.
**Notes:** Higgott, Wang, Brierley (2019). Practical excited-state recipe.

### qaoa-depth-p (cross-domain alias: `QAOA-p-layer`, `bang-bang-control`, `Farhi-Goldstone-Gutmann`)
**Domain:** Quantum Computing
**Definition:** Apply alternation U(γ_p)=∏(e^{−iβ_k H_M} e^{−iγ_k H_C}); optimize 2p parameters. At p→∞ equivalent to AQC.
**Atom or composite:** Composite — bang-bang Trotter ansatz.
**Cost model:** Depth O(p) gates per layer.
**Real wall?** Slow convergence with p; barren plateaus emerge.
**Cross-domain wiring:** signal-processing-rf: bang-bang control. ml-training: layered ansatz. statistics-probability: simulated annealing analogue.
**Notes:** Farhi, Goldstone, Gutmann (2014). Most-studied near-term combinatorial algorithm.

### warm-start-qaoa (cross-domain alias: `WS-QAOA`, `Egger-Marecek-Woerner`, `relaxation-warm-start`)
**Domain:** Quantum Computing
**Definition:** Initialize state from continuous relaxation (e.g., GW SDP); replace |+⟩^N with rotated product state encoding relaxation solution.
**Atom or composite:** Composite — modified initial state + tuned mixer.
**Cost model:** Same depth as standard QAOA; better quality at p=1.
**Real wall?** No.
**Cross-domain wiring:** classical-optimization: SDP/LP relaxation. ml-training: warm-start initialization. statistics-probability: informed prior.
**Notes:** Egger et al. (2021). Often gives QAOA p=1 results matching higher p.

### recursive-qaoa (cross-domain alias: `RQAOA`, `Bravyi-Kliesch-Koenig-Tang`, `variable-elimination-QAOA`)
**Domain:** Quantum Computing
**Definition:** Run QAOA-p, measure correlations Z_iZ_j, eliminate variable with largest |⟨Z_iZ_j⟩|; recurse on reduced problem.
**Atom or composite:** Composite — iterative variable elimination.
**Cost model:** O(N) QAOA runs on shrinking instance.
**Real wall?** Heuristic, but provably better than p=1 QAOA on some graphs.
**Cross-domain wiring:** classical-optimization: variable elimination. ml-training: greedy reduction. statistics-probability: conditional inference.
**Notes:** Bravyi et al. (2019). Closes gap to Goemans-Williamson on some instances.

### ma-qaoa (cross-domain alias: `multi-angle-QAOA`, `Herrman-Lotshaw-Ostrowski`, `parameter-per-edge`)
**Domain:** Quantum Computing
**Definition:** Replace single γ,β per layer with per-edge / per-qubit angles. Larger parameter space, better landscape.
**Atom or composite:** Composite — extended parameterization.
**Cost model:** O(|E|+N) parameters per layer.
**Real wall?** Optimization harder due to larger space; no barren plateau improvement guaranteed.
**Cross-domain wiring:** ml-training: parameter expansion. signal-processing-rf: per-channel control. statistics-probability: hierarchical model.
**Notes:** Herrman et al. (2021). Modest gains; useful when standard QAOA stalls.

### adapt-qaoa (cross-domain alias: `ADAPT-QAOA`, `Zhu-Tang-Barnes-Mayhall`, `adaptive-mixer-QAOA`)
**Domain:** Quantum Computing
**Definition:** Iteratively add mixer Hamiltonian from pool maximizing gradient. Adaptive analogue to ADAPT-VQE for combinatorial.
**Atom or composite:** Composite.
**Cost model:** Adaptive — fewer layers for same accuracy.
**Real wall?** Pool engineering needed.
**Cross-domain wiring:** ml-training: ADAPT-style construction. classical-optimization: adaptive heuristic. statistics-probability: information-greedy.
**Notes:** Zhu et al. (2022). Outperforms vanilla QAOA on MaxCut.

### layerwise-learning (cross-domain alias: `Skolik-McClean-Mohseni`, `layer-by-layer-training`, `staged-VQE`)
**Domain:** Quantum Computing
**Definition:** Train ansatz one layer at a time, freezing earlier layers; reduces barren-plateau exposure.
**Atom or composite:** Composite — sequential optimization schedule.
**Cost model:** Same circuit, training schedule differs.
**Real wall?** Suboptimal in landscape sense; symptom-management not cure.
**Cross-domain wiring:** ml-training: greedy layer-wise pretraining (Hinton). signal-processing-rf: progressive parameter unfreezing. statistics-probability: stagewise estimation.
**Notes:** Skolik et al. (2021). Mitigates but doesn't eliminate barren plateaus.

### barren-plateau (cross-domain alias: `BP`, `McClean-Boixo-Smelyanskiy-Babbush-Neven`, `gradient-vanishing-VQA`)
**Domain:** Quantum Computing
**Definition:** For random parameterized circuits, Var[∂C/∂θ] decays exponentially with N. Gradients drown in shot noise; trainability dies.
**Atom or composite:** Atom — diagnostic phenomenon.
**Cost model:** Sample complexity to detect gradient ~ 2^N shots.
**Real wall?** Yes — exponential vanishing limits scalability of expressive ansätze.
**Cross-domain wiring:** ml-training: vanishing gradients in deep nets. statistics-probability: concentration of measure on Haar. linear-algebra-matrix: spectra of random unitaries.
**Notes:** McClean et al. (2018). Subsequent work links to entanglement, expressibility, noise.

### parameter-shift-rule (cross-domain alias: `PSR`, `analytic-gradient`, `Mitarai-Negoro-Kitagawa-Fujii`)
**Domain:** Quantum Computing
**Definition:** For gate e^{iθP/2} with P²=I, ∂_θ⟨H⟩ = (⟨H⟩_{θ+π/2} − ⟨H⟩_{θ−π/2})/2. Exact gradient via two circuit evaluations.
**Atom or composite:** Atom — analytic gradient evaluation.
**Cost model:** 2 evaluations per parameter — same big-O as classical autodiff per parameter.
**Real wall?** Doesn't generalize to arbitrary generators (need spectral-3-point rules).
**Cross-domain wiring:** ml-training: REINFORCE/score-function gradient analogue. signal-processing-rf: finite-difference but exact. statistics-probability: unbiased gradient estimator.
**Notes:** Mitarai et al. (2018); Schuld et al. (2019). Standard quantum gradient method.

### quantum-natural-gradient (cross-domain alias: `QNG`, `Stokes-Izaac-Killoran-Carleo`, `Fubini-Study-metric`)
**Domain:** Quantum Computing
**Definition:** Update θ ← θ − η · F⁺(θ) · ∇C, where F is quantum Fisher information / Fubini-Study metric on state manifold.
**Atom or composite:** Composite — preconditioned gradient.
**Cost model:** Estimating F: O(P²) circuits for P parameters.
**Real wall?** F can be near-singular; pseudo-inverse needed.
**Cross-domain wiring:** ml-training: Amari natural gradient. statistics-probability: Fisher information metric. linear-algebra-matrix: Riemannian optimization.
**Notes:** Stokes et al. (2020). Faster convergence than vanilla gradient descent.

### quantum-kernel (cross-domain alias: `QK`, `Schuld-Killoran`, `inner-product-feature-map`)
**Domain:** Quantum Computing
**Definition:** k(x,x′) = |⟨φ(x′)|φ(x)⟩|² where |φ(x)⟩ = U(x)|0⟩. Quantum feature map embedding into Hilbert space.
**Atom or composite:** Composite — quantum feature map + classical kernel methods.
**Cost model:** O(M²) circuit evaluations for M-sample kernel matrix.
**Real wall?** Exponentially small kernel values → exponentially many samples (curse of dimensionality).
**Cross-domain wiring:** ml-training: kernel methods, SVM. statistics-probability: covariance via inner product. linear-algebra-matrix: Gram matrix.
**Notes:** Havlicek et al. (2019); Schuld & Killoran (2019). Hope for quantum-advantage classifiers.

### projected-quantum-kernel (cross-domain alias: `PQK`, `Huang-Broughton-et-al`, `RDM-kernel`)
**Domain:** Quantum Computing
**Definition:** Compute kernel from reduced density matrices: k(x,x′) = Σ_i Tr(ρ_i(x)·ρ_i(x′)) where ρ_i is k-body RDM.
**Atom or composite:** Composite — kernel on projected states.
**Cost model:** Polynomial-size feature space avoiding exponential decay.
**Real wall?** Loss of expressive power compared to full-state kernel.
**Cross-domain wiring:** ml-training: feature engineering. signal-processing-rf: subspace decomposition. statistics-probability: marginal distributions.
**Notes:** Huang et al. (2021). Empirical advantage when classical features are insufficient.

### data-reuploading (cross-domain alias: `Pérez-Salinas-Cervera-Lierta`, `repeated-encoding`, `data-uploading-classifier`)
**Domain:** Quantum Computing
**Definition:** Alternate data-encoding U(x) and trainable U(θ) layers: U(θ_L)U(x)U(θ_{L−1})...U(x)|0⟩. Single qubit can express universal classifier.
**Atom or composite:** Composite — interleaved encoding.
**Cost model:** O(L) layers; constant qubit count.
**Real wall?** Trainability limited but no curse of dimensionality.
**Cross-domain wiring:** ml-training: classifier as Fourier model. signal-processing-rf: chirp encoding. linear-algebra-matrix: alternating products.
**Notes:** Pérez-Salinas et al. (2020). Foundation of single-qubit ML models.

### qsvm (cross-domain alias: `quantum-SVM`, `kernel-SVM-quantum`, `Havlicek-Corcoles-Temme`)
**Domain:** Quantum Computing
**Definition:** SVM training using quantum kernel matrix K_{ij}=|⟨φ(x_i)|φ(x_j)⟩|². Classical SVM solver on K.
**Atom or composite:** Composite — quantum kernel + classical SVM dual.
**Cost model:** O(M²) circuit pairs; classical QP solve.
**Real wall?** Empirical advantage problem-specific.
**Cross-domain wiring:** ml-training: kernel SVM. statistics-probability: max-margin classification. linear-algebra-matrix: QP on kernel matrix.
**Notes:** Havlicek et al. (2019). Demonstrated quantum-advantage on engineered datasets.

### qcnn (cross-domain alias: `quantum-CNN`, `Cong-Choi-Lukin`, `MERA-inspired-classifier`)
**Domain:** Quantum Computing
**Definition:** Hierarchical PQC mimicking CNN: convolution blocks + pooling (trace-out) layers. O(log N) depth, translation symmetry.
**Atom or composite:** Composite — convolution + pooling hierarchy.
**Cost model:** O(N·log N) gates.
**Real wall?** Empirical performance issue at scale.
**Cross-domain wiring:** ml-training: deep CNN. signal-processing-rf: multi-resolution filtering. linear-algebra-matrix: tensor network ansatz (MERA).
**Notes:** Cong, Choi, Lukin (2019). Provably free of barren plateaus (Pesah et al. 2021).

---

## Quantum Error Correction (Advanced)

### css-code (cross-domain alias: `Calderbank-Shor-Steane-code`, `dual-classical-codes`, `CSS-construction`)
**Domain:** Quantum Computing
**Definition:** Built from two classical codes C₁⊆C₂ with C₂⊥⊆C₁. X-stabilizers from C₂⊥, Z-stabilizers from C₁. Detects X and Z errors separately.
**Atom or composite:** Composite — dual-code construction.
**Cost model:** [[n, k₁−k₂⊥, d]] with k₁=dim C₁, parameters from classical codes.
**Real wall?** No — clean way to lift classical codes.
**Cross-domain wiring:** information-theory-coding: classical linear codes. linear-algebra-matrix: GF(2) parity-check matrices. cryptography-advanced: McEliece-style code uses.
**Notes:** Calderbank-Shor (1996), Steane (1996). Most quantum codes (Steane, surface, etc.) are CSS.

### stabilizer-code-general (cross-domain alias: `Gottesman-stabilizer`, `Pauli-group-code`, `[[n,k,d]]-code`)
**Domain:** Quantum Computing
**Definition:** Codespace = simultaneous +1 eigenspace of abelian Pauli subgroup S ⊂ P_n. Encodes k=n−|S| logical qubits with distance d.
**Atom or composite:** Atom — formalism for QEC.
**Cost model:** Stabilizer measurement = ancilla + Pauli measurement; classical decoder.
**Real wall?** Only handles errors of weight ≤ ⌊(d−1)/2⌋ deterministically.
**Cross-domain wiring:** linear-algebra-matrix: kernel of binary symplectic form. group-theory: abelian subgroups of P_n. information-theory-coding: stabilizer codes as additive codes.
**Notes:** Gottesman (1997). Universal framework underlying all stabilizer-based QEC.

### shor-9-qubit (cross-domain alias: `Shor-code`, `[[9,1,3]]`, `concatenated-phase-bit-flip`)
**Domain:** Quantum Computing
**Definition:** Concatenation of 3-qubit bit-flip code inside 3-qubit phase-flip code. Encodes 1 logical qubit in 9 physical, distance 3.
**Atom or composite:** Composite — code concatenation.
**Cost model:** 9 physical qubits, 8 stabilizers.
**Real wall?** Not transversal for non-Clifford gates.
**Cross-domain wiring:** information-theory-coding: concatenated code construction. linear-algebra-matrix: tensor of repetition codes. signal-processing-rf: hierarchical redundancy.
**Notes:** Shor (1995). First QEC code; proved QEC is possible.

### steane-7-qubit (cross-domain alias: `Steane-code`, `[[7,1,3]]`, `Hamming-CSS`)
**Domain:** Quantum Computing
**Definition:** CSS code from [7,4,3] Hamming code. Encodes 1 logical qubit; transversal Clifford gates.
**Atom or composite:** Composite — CSS from Hamming.
**Cost model:** 7 physical qubits, 6 stabilizers.
**Real wall?** No transversal T; needs magic-state injection for universality.
**Cross-domain wiring:** information-theory-coding: Hamming code. linear-algebra-matrix: parity-check from F₂⁷. group-theory: PGL₂(F₂) symmetry.
**Notes:** Steane (1996). Industry favorite for fault-tolerant logic demonstrations.

### perfect-5-qubit (cross-domain alias: `Laflamme-Miquel-Paz-Zurek-code`, `[[5,1,3]]`, `quantum-Hamming-bound-saturating`)
**Domain:** Quantum Computing
**Definition:** Smallest [[n,1,3]] code; saturates quantum Hamming bound. Stabilizers: cyclic permutations of XZZXI.
**Atom or composite:** Atom — minimal distance-3 code.
**Cost model:** 5 qubits, 4 stabilizers.
**Real wall?** Not CSS; no transversal Clifford set.
**Cross-domain wiring:** information-theory-coding: perfect code analogue. group-theory: 5-cyclic symmetry. linear-algebra-matrix: stabilizer over GF(4).
**Notes:** Laflamme, Miquel, Paz, Zurek (1996). Smallest error-correcting code; popular benchmark.

### reed-muller-code (cross-domain alias: `RM-code-quantum`, `triorthogonal-code`, `transversal-T-code`)
**Domain:** Quantum Computing
**Definition:** Punctured Reed-Muller(1,m) gives [[2^m−1, 1, 2^{m−1}−1]] code with transversal T-gate. E.g., [[15,1,3]].
**Atom or composite:** Composite.
**Cost model:** Trade-off: enables T transversally but high overhead.
**Real wall?** No transversal Clifford+T simultaneously (Eastin-Knill).
**Cross-domain wiring:** information-theory-coding: Reed-Muller code. linear-algebra-matrix: triorthogonal matrix. group-theory: clifford hierarchy.
**Notes:** Knill, Laflamme, Zurek (1996); Bravyi-Kitaev (2005). Key for non-Clifford fault tolerance.

### concatenated-codes (cross-domain alias: `code-concatenation`, `recursive-encoding`, `Knill-Laflamme-concatenation`)
**Domain:** Quantum Computing
**Definition:** Encode each qubit of outer code [[n,1,d]] using same code recursively L levels. Logical error rate scales (p/p_th)^{2^L}.
**Atom or composite:** Composite — recursive construction.
**Cost model:** n^L physical qubits for L levels.
**Real wall?** Resource overhead at large L.
**Cross-domain wiring:** information-theory-coding: concatenated codes. signal-processing-rf: hierarchical processing. statistics-probability: double-exponential error suppression.
**Notes:** Foundation of threshold theorem (Aharonov-Ben-Or, Knill-Laflamme-Zurek).

### color-code (cross-domain alias: `triangular-color-code`, `Bombin-Martin-Delgado`, `3-colorable-code`)
**Domain:** Quantum Computing
**Definition:** 2D code on 3-colorable trivalent lattice. Each face = stabilizer (X and Z products). Transversal Clifford group in 2D.
**Atom or composite:** Composite — topological code on colored lattice.
**Cost model:** Threshold ~0.1−1% (between surface and Reed-Muller).
**Real wall?** Lower threshold than surface code but richer transversal gates.
**Cross-domain wiring:** physics-condensed-matter: topological order. graph-theory: 3-coloring. group-theory: anyon model.
**Notes:** Bombin & Martin-Delgado (2006). Transversal Clifford in 2D; T via gauge-fixing in 3D.

### hyperbolic-codes (cross-domain alias: `hyperbolic-surface-codes`, `Breuckmann-Terhal`, `non-Euclidean-codes`)
**Domain:** Quantum Computing
**Definition:** Surface code on hyperbolic 2-manifold tessellation. Achieves constant rate k/n > 0 (vs O(1/n) for planar).
**Atom or composite:** Composite — topological code on hyperbolic geometry.
**Cost model:** k = Θ(n), d = Θ(log n).
**Real wall?** No 2D-local layout in flat space; needs long-range connections.
**Cross-domain wiring:** graph-theory: hyperbolic tessellations. information-theory-coding: high-rate codes. physics-condensed-matter: AdS/CFT analogues.
**Notes:** Breuckmann & Terhal (2016). Bridge between LDPC quantum codes and topology.

### qldpc-codes (cross-domain alias: `quantum-LDPC`, `low-density-parity-check`, `Tillich-Zémor-quasicyclic`)
**Domain:** Quantum Computing
**Definition:** Stabilizer codes with bounded-weight stabilizers and bounded-degree qubits (LDPC). Recent: rate Θ(1), distance Θ(n).
**Atom or composite:** Composite — code with LDPC parity-check matrix.
**Cost model:** Decoder: BP+OSD or other LDPC-style.
**Real wall?** No known 2D-local realization for good codes; requires nonlocal hardware (e.g., neutral atoms with shuttling).
**Cross-domain wiring:** information-theory-coding: classical LDPC. linear-algebra-matrix: sparse parity-check. graph-theory: Tanner graph expansion.
**Notes:** Panteleev-Kalachev (2022): asymptotically good qLDPC codes. Future of QEC.

### magic-state-injection (cross-domain alias: `magic-state-distillation`, `Bravyi-Kitaev-magic`, `T-state-protocol`)
**Domain:** Quantum Computing
**Definition:** Distill noisy |T⟩=(|0⟩+e^{iπ/4}|1⟩)/√2 via Clifford circuits; consume to apply T-gate via teleportation.
**Atom or composite:** Composite — Clifford distillation + teleported T.
**Cost model:** Dominant cost in fault-tolerant computation; 10-100× overhead.
**Real wall?** No transversal T (Eastin-Knill); must use magic states.
**Cross-domain wiring:** information-theory-coding: distillation protocols. cryptography-advanced: privacy amplification analogue. statistics-probability: rare-event sampling.
**Notes:** Bravyi-Kitaev (2005). Numerous variants: 15-to-1, 14-to-2, multiblock.

### lattice-surgery (cross-domain alias: `surface-code-merge-split`, `Horsman-Fowler-Devitt-Van-Meter`, `LS`)
**Domain:** Quantum Computing
**Definition:** Logical operations on surface code via merging/splitting patches: temporarily measure boundary stabilizers to entangle two patches; protocol enables ZZ-measurement and CNOT.
**Atom or composite:** Composite — boundary-stabilizer surgery.
**Cost model:** O(d) cycles per surgery.
**Real wall?** Routing constraints on 2D layout; teleportation needed for some patterns.
**Cross-domain wiring:** linear-algebra-matrix: tensor surgery. graph-theory: planar graph surgery. signal-processing-rf: time-multiplexed channels.
**Notes:** Horsman et al. (2012). De-facto standard for fault-tolerant logic in 2D codes.

### transversal-gates (cross-domain alias: `pointwise-gates`, `Eastin-Knill-context`, `fault-tolerant-by-construction`)
**Domain:** Quantum Computing
**Definition:** Logical gate implemented by applying single-qubit operations to each physical qubit independently. Errors don't spread.
**Atom or composite:** Atom — fault-tolerance class.
**Cost model:** Trivial — n physical gates.
**Real wall?** Eastin-Knill: no code has transversal universal gate set.
**Cross-domain wiring:** group-theory: Clifford hierarchy. information-theory-coding: code automorphism. signal-processing-rf: per-channel processing.
**Notes:** Cleanest fault-tolerance recipe; combined with magic-state injection gives universality.

### code-deformation (cross-domain alias: `topological-defect-moves`, `Raussendorf-Bombin`, `lattice-evolution`)
**Domain:** Quantum Computing
**Definition:** Smoothly modify stabilizer group over time (e.g., move surface code defects) to implement logical gates via braiding holes.
**Atom or composite:** Composite — time-varying stabilizer group.
**Cost model:** O(d²) time steps per move.
**Real wall?** Slow compared to lattice surgery.
**Cross-domain wiring:** physics-topological: defect braiding. graph-theory: cellular deformations. linear-algebra-matrix: continuous code family.
**Notes:** Bombin (2010); Raussendorf-Harrington (2007). Historical approach, partially superseded by surgery.

### code-switching (cross-domain alias: `code-conversion`, `Anderson-Duclos-Cianci-Poulin`, `cross-code-gate`)
**Domain:** Quantum Computing
**Definition:** Switch between two codes (e.g., 2D color code ↔ 3D color code) to access transversal gates of each.
**Atom or composite:** Composite — encoded mapping protocol.
**Cost model:** Encoded teleportation overhead.
**Real wall?** Conversion noise must be below threshold of both codes.
**Cross-domain wiring:** information-theory-coding: code equivalence. linear-algebra-matrix: change-of-stabilizer. signal-processing-rf: modulation switching.
**Notes:** Anderson et al. (2014). Bypasses Eastin-Knill via runtime code change.

### gkp-code (cross-domain alias: `Gottesman-Kitaev-Preskill`, `oscillator-encoding`, `qubit-in-an-oscillator`)
**Domain:** Quantum Computing
**Definition:** Encode qubit in continuous variable (oscillator) via grid states in phase space. Protected against small displacements in q and p.
**Atom or composite:** Composite — CV-to-DV encoding.
**Cost model:** Requires non-Gaussian resources to prepare.
**Real wall?** Idealized GKP needs infinite squeezing; finite-energy approximations realistic.
**Cross-domain wiring:** signal-processing-rf: phase-space grid. linear-algebra-matrix: bosonic mode operators. cryptography-advanced: lattice cryptography analogue.
**Notes:** Gottesman, Kitaev, Preskill (2001). Reality: realized in trapped ions, microwave cavities (2020-2024).

### cat-code (cross-domain alias: `Schrödinger-cat-code`, `coherent-state-superposition`, `Mirrahimi-cat`)
**Domain:** Quantum Computing
**Definition:** Encode qubit as |0_L⟩∝|α⟩+|−α⟩, |1_L⟩∝|α⟩−|−α⟩. Biased noise: dephasing exponentially suppressed in |α|².
**Atom or composite:** Composite — CV qubit encoding.
**Cost model:** Two-photon pumping for autonomous stabilization.
**Real wall?** Hardware: requires nonlinear Hamiltonian in microwave cavity.
**Cross-domain wiring:** signal-processing-rf: coherent state superposition. linear-algebra-matrix: 2D code subspace. physics: macroscopic superposition.
**Notes:** Mirrahimi et al. (2014). Excellent for biased-noise QEC; Yale (Devoret/Schoelkopf) experiments.

### binomial-code (cross-domain alias: `Michael-Silveri-Brierley-code`, `bosonic-binomial-encoding`, `loss-protected-code`)
**Domain:** Quantum Computing
**Definition:** Logical states are superpositions of Fock states with binomial coefficients tailored to correct photon loss.
**Atom or composite:** Composite — Fock-basis encoding.
**Cost model:** Photon number truncation; finite-energy code.
**Real wall?** Photon loss is dominant in optical/microwave; this code handles it natively.
**Cross-domain wiring:** information-theory-coding: bosonic codes. linear-algebra-matrix: bosonic operators. signal-processing-rf: photon-number-resolved detection.
**Notes:** Michael et al. (2016). Best small-overhead loss-protected bosonic code.

### dual-rail-code (cross-domain alias: `dual-rail-encoding`, `photonic-DR`, `two-mode-qubit`)
**Domain:** Quantum Computing
**Definition:** |0_L⟩=|10⟩, |1_L⟩=|01⟩ in two photonic modes. Photon loss is detected (vacuum heralding).
**Atom or composite:** Composite — two-mode encoding.
**Cost model:** 2 modes per logical qubit.
**Real wall?** Erasure-only protection — needs additional code for other errors.
**Cross-domain wiring:** signal-processing-rf: differential signaling. information-theory-coding: erasure code. physics: linear optics.
**Notes:** Foundational in linear-optical QC (KLM); also revived for superconducting cavities (Levine et al. 2023).

---

## Decoders for QEC

### mwpm-decoder (cross-domain alias: `minimum-weight-perfect-matching`, `Edmonds-blossom-decoder`, `MWPM`)
**Domain:** Quantum Computing
**Definition:** Treat syndrome violations as graph vertices, weights = log probabilities; find min-weight perfect matching. Recovery = matched pairs.
**Atom or composite:** Composite — graph matching on syndrome.
**Cost model:** O(n³) for Edmonds; faster with sparse approximations.
**Real wall?** Surface code threshold ~0.7% under MWPM.
**Cross-domain wiring:** graph-theory: weighted matching. signal-processing-rf: optimal decoding. statistics-probability: MAP estimation in pairwise model.
**Notes:** Dennis et al. (2002). Standard surface-code decoder.

### union-find-decoder (cross-domain alias: `UF-decoder`, `Delfosse-Nickerson`, `almost-linear-decoder`)
**Domain:** Quantum Computing
**Definition:** Grow clusters around syndrome vertices, merge via union-find; once each cluster spans an even number of syndromes, decode locally.
**Atom or composite:** Composite — disjoint-set growth.
**Cost model:** Almost-linear O(n·α(n)) where α is inverse Ackermann.
**Real wall?** Slightly worse threshold than MWPM (~0.6%).
**Cross-domain wiring:** graph-theory: union-find / DSU. classical-algorithms: percolation analogue. signal-processing-rf: cluster decoder.
**Notes:** Delfosse & Nickerson (2017). Fast enough for real-time decoding.

### bp-decoder (cross-domain alias: `belief-propagation-decoder`, `LDPC-BP`, `sum-product-decoder`)
**Domain:** Quantum Computing
**Definition:** Iterative message passing on Tanner graph: variables ↔ checks. Local marginals approximate posterior over errors.
**Atom or composite:** Composite — message passing on factor graph.
**Cost model:** O(iterations × |E|) per round.
**Real wall?** Suffers on quantum codes due to degeneracy + short cycles; needs post-processing (OSD).
**Cross-domain wiring:** information-theory-coding: LDPC decoding. statistics-probability: factor graph inference. ml-training: message passing networks.
**Notes:** Gallager (1962) classical; Poulin-Chung (2008) quantum. Combined with OSD = SOTA for qLDPC.

### neural-decoder (cross-domain alias: `NN-decoder`, `Torlai-Melko`, `learned-decoder`)
**Domain:** Quantum Computing
**Definition:** Train neural net (CNN/RNN/Transformer) to map syndrome → recovery Pauli. Latency advantage over MWPM with comparable accuracy.
**Atom or composite:** Composite — supervised learning on synthetic syndromes.
**Cost model:** Training: O(10⁶) examples. Inference: forward pass.
**Real wall?** Generalization off training distribution; calibration drift.
**Cross-domain wiring:** ml-training: supervised classification. signal-processing-rf: learned demod. statistics-probability: posterior approximation.
**Notes:** Torlai & Melko (2017); Davaasuren et al. (2020). Active commercial development (Google, IBM).

### tensor-network-decoder (cross-domain alias: `TN-decoder`, `Bravyi-Suchara-Vargo`, `contracted-tensor-decoder`)
**Domain:** Quantum Computing
**Definition:** Express maximum likelihood decoding as tensor network contraction; approximate via MPS/PEPS contraction algorithms.
**Atom or composite:** Composite — tensor network on syndrome.
**Cost model:** Exponential exact; polynomial approximate (bond dim χ).
**Real wall?** Achieves near-optimal performance but slow.
**Cross-domain wiring:** linear-algebra-matrix: tensor contraction. statistics-probability: partition function evaluation. physics-condensed-matter: 2D Ising-like models.
**Notes:** Bravyi, Suchara, Vargo (2014). Benchmark "ground-truth" decoder.

### renormalization-decoder (cross-domain alias: `Duclos-Cianci-Poulin`, `RG-decoder`, `hierarchical-decoder`)
**Domain:** Quantum Computing
**Definition:** Recursively coarse-grain syndrome lattice; decode at each scale. Polynomial in code distance.
**Atom or composite:** Composite — RG-style sweeps.
**Cost model:** O(log d) levels of decimation.
**Real wall?** Slightly suboptimal threshold but fast.
**Cross-domain wiring:** physics-condensed-matter: renormalization group. signal-processing-rf: multiresolution. ml-training: hierarchical pooling.
**Notes:** Duclos-Cianci & Poulin (2010). Easy hardware implementation; parallelizable.

---

## Quantum Noise & Error Mitigation

### probabilistic-error-cancellation (cross-domain alias: `PEC`, `Temme-Bravyi-Gambetta`, `quasi-probabilistic-mitigation`)
**Domain:** Quantum Computing
**Definition:** Decompose ideal gate as quasi-probability ∑γ_i·N_i with noisy basis N_i; Monte Carlo with sign yields unbiased expectation, variance grows like γ²ⁿ.
**Atom or composite:** Composite — signed Monte Carlo over noisy gates.
**Cost model:** Sampling overhead γ²ⁿ; exponential in circuit size.
**Real wall?** Exponential variance limits to shallow circuits.
**Cross-domain wiring:** statistics-probability: importance sampling with signs. signal-processing-rf: noise inversion. ml-training: debiased estimators.
**Notes:** Temme, Bravyi, Gambetta (2017). Removes bias if noise model known.

### zne (cross-domain alias: `zero-noise-extrapolation`, `Li-Benjamin`, `Temme-error-extrapolation`)
**Domain:** Quantum Computing
**Definition:** Run circuit at amplified noise levels (gate folding, pulse stretching) and extrapolate observable to zero-noise limit via Richardson/exponential fit.
**Atom or composite:** Composite — noise scaling + classical extrapolation.
**Cost model:** O(k) runs at k noise levels.
**Real wall?** Polynomial bias; cannot extrapolate noise out of regime.
**Cross-domain wiring:** numerics: Richardson extrapolation. signal-processing-rf: deconvolution. statistics-probability: regression on noise parameter.
**Notes:** Li-Benjamin (2017); Temme et al. (2017). Most-used mitigation in NISQ experiments.

### richardson-extrapolation (cross-domain alias: `polynomial-extrapolation`, `deferred-acceptance`, `weighted-noise-sum`)
**Domain:** Quantum Computing
**Definition:** Linear combination Σ a_k ⟨O⟩_{λ_k} chosen so that O(λ^n) noise terms cancel, leaving zero-noise estimate.
**Atom or composite:** Atom — classical extrapolation method.
**Cost model:** k+1 measurements for k-th order extrapolation.
**Real wall?** Coefficient amplification → variance blow-up.
**Cross-domain wiring:** numerics: Richardson method for ODE/quadrature. statistics-probability: bias-variance tradeoff. signal-processing-rf: filter cancellation.
**Notes:** Richardson (1911). Foundation of ZNE.

### virtual-distillation (cross-domain alias: `Huggins-McArdle-Brierley`, `exponential-error-suppression`, `M-copy-purification`)
**Domain:** Quantum Computing
**Definition:** Measure ⟨O⟩_M = Tr(O·ρ^M)/Tr(ρ^M) using M copies of state; converges to dominant eigenvector of ρ — typically the ideal state.
**Atom or composite:** Composite — multi-copy circuit + classical post-processing.
**Cost model:** M·n qubits + entangled measurement.
**Real wall?** Coherent errors limit benefit; sampling overhead grows.
**Cross-domain wiring:** linear-algebra-matrix: power iteration. statistics-probability: state purification. signal-processing-rf: bootstrapping.
**Notes:** Huggins et al. (2021); Koczor (2021). Exponential suppression of incoherent noise.

### dynamical-decoupling (cross-domain alias: `DD`, `bang-bang-decoupling`, `Viola-Lloyd-Knill`)
**Domain:** Quantum Computing
**Definition:** Apply periodic pulses (X, XY-4, CPMG) that average out slow environmental coupling. Echo refocuses dephasing.
**Atom or composite:** Composite — pulse sequence during idle.
**Cost model:** Negligible — idle-time gates.
**Real wall?** Pulse imperfections; only works for slow noise.
**Cross-domain wiring:** signal-processing-rf: spin echo (NMR Hahn echo). physics: rotating-wave averaging. statistics-probability: noise averaging.
**Notes:** Hahn (1950) NMR; Viola-Lloyd (1998). Standard idle-protection on superconducting hardware.

### composite-pulses (cross-domain alias: `BB1`, `Wimperis-pulses`, `error-correcting-pulses`)
**Domain:** Quantum Computing
**Definition:** Replace single rotation with sequence of pulses designed to cancel pulse-amplitude or detuning errors to high order.
**Atom or composite:** Composite — sequence of single-qubit rotations.
**Cost model:** 3-5x duration for first-order BB1.
**Real wall?** Doesn't help with stochastic noise.
**Cross-domain wiring:** signal-processing-rf: equalization sequences. physics: NMR composite pulses. statistics-probability: bias correction.
**Notes:** Wimperis (1994); Brown-Harrow-Chuang (2004). Coherent-error suppression in calibration.

### randomized-compiling (cross-domain alias: `RC`, `Wallman-Emerson`, `Pauli-randomization`)
**Domain:** Quantum Computing
**Definition:** Insert random Pauli before each cycle, undo after; coherent errors twirl into stochastic Pauli channel. Improves accuracy of error analysis.
**Atom or composite:** Composite — random insertion + correction.
**Cost model:** O(1) overhead per cycle.
**Real wall?** Reduces but doesn't eliminate errors.
**Cross-domain wiring:** statistics-probability: noise whitening. quantum-error-correction: simplifies decoder analysis. signal-processing-rf: dithering.
**Notes:** Wallman & Emerson (2016). Routinely used with PEC and ZNE.

### m3-measurement-mitigation (cross-domain alias: `M3`, `Nation-Kang-Sundaresan-Gambetta`, `matrix-free-readout-mitigation`)
**Domain:** Quantum Computing
**Definition:** Calibrate full assignment matrix A (Pr(measured|true)); use iterative/iterative-refinement to invert A on observed counts, returning corrected probabilities.
**Atom or composite:** Composite — calibration + iterative classical inversion.
**Cost model:** Cal: 2N circuits; inversion: subspace iterative.
**Real wall?** Approximate — assumes Markovian readout.
**Cross-domain wiring:** statistics-probability: confusion matrix inversion. signal-processing-rf: detector calibration. linear-algebra-matrix: matrix-free inversion.
**Notes:** Nation et al. (2021). IBM-Qiskit default readout mitigator.

### symmetry-verification (cross-domain alias: `Bonet-Monroig-symmetry`, `parity-check-mitigation`, `McClean-purity-recovery`)
**Domain:** Quantum Computing
**Definition:** Project out states violating known symmetries (particle number, spin, parity) by postselection on ancilla measurements.
**Atom or composite:** Composite — symmetry measurement + postselect.
**Cost model:** O(1) ancilla; rejection overhead.
**Real wall?** Only catches symmetry-breaking errors.
**Cross-domain wiring:** physics: conservation laws. quantum-error-correction: stabilizer postselection. statistics-probability: rejection sampling.
**Notes:** McArdle et al. (2019); Bonet-Monroig et al. (2018). Cheap mitigation in chemistry VQE.

### clifford-data-regression (cross-domain alias: `CDR`, `Czarnik-Arrasmith-Coles-Sornborger`, `ML-mitigation`)
**Domain:** Quantum Computing
**Definition:** Train regression model on near-Clifford circuits (classically simulable) where exact answer known; apply to target observable.
**Atom or composite:** Composite — classical regression + circuit modification.
**Cost model:** Many auxiliary circuit evaluations.
**Real wall?** Generalization across circuit families is fragile.
**Cross-domain wiring:** ml-training: regression-based calibration. statistics-probability: importance reweighting. signal-processing-rf: calibration sequences.
**Notes:** Czarnik et al. (2021). Combines well with ZNE for layered mitigation.

---

## Continuous-Variable Quantum Computing

### gaussian-state (cross-domain alias: `Gaussian-quantum-state`, `Wigner-Gaussian`, `bosonic-Gaussian`)
**Domain:** Quantum Computing
**Definition:** Bosonic state with Gaussian Wigner function; fully characterized by displacement vector d and covariance matrix σ.
**Atom or composite:** Atom — class of states closed under Gaussian operations.
**Cost model:** Classical: O(N²) covariance update per Gaussian gate.
**Real wall?** Gaussian-only computation is classically simulable.
**Cross-domain wiring:** statistics-probability: multivariate Gaussian. signal-processing-rf: thermal & coherent states. linear-algebra-matrix: symplectic transformations.
**Notes:** Foundation of continuous-variable quantum information; non-Gaussian resource needed for universality.

### squeezed-state (cross-domain alias: `Sq(ξ)|0⟩`, `quadrature-squeezed`, `vacuum-squeezing`)
**Domain:** Quantum Computing
**Definition:** S(ξ)|0⟩ where S(ξ)=exp(½(ξ*a²−ξ a†²)). Variance in one quadrature suppressed below ½, other amplified.
**Atom or composite:** Atom — Gaussian state with reduced quadrature variance.
**Cost model:** Optical parametric amplifier produces squeezing in physical systems.
**Real wall?** Squeezing limited by parametric gain & loss (typically 10-15 dB).
**Cross-domain wiring:** signal-processing-rf: noise reduction below shot noise. statistics-probability: anisotropic Gaussian. physics: LIGO noise reduction.
**Notes:** Caves (1981). Backbone of CV quantum information; key resource for Gaussian boson sampling.

### coherent-state (cross-domain alias: `|α⟩`, `Glauber-state`, `displaced-vacuum`)
**Domain:** Quantum Computing
**Definition:** Eigenstate of annihilation operator a|α⟩=α|α⟩. Minimum uncertainty, classical-like; produced by lasers.
**Atom or composite:** Atom — Gaussian, Poissonian photon number.
**Cost model:** Trivially produced from laser source.
**Real wall?** No — these ARE classical light states.
**Cross-domain wiring:** signal-processing-rf: monochromatic carrier. statistics-probability: Poisson statistics. physics: lasing.
**Notes:** Glauber (1963). Foundation of quantum optics; reference state for CV.

### two-mode-squeezing (cross-domain alias: `TMSV`, `EPR-state-CV`, `nondegenerate-OPO`)
**Domain:** Quantum Computing
**Definition:** S₂(ξ)|00⟩ = exp(ξ*a₁a₂−ξ a₁†a₂†)|00⟩. Photon-number correlations between modes (EPR-like).
**Atom or composite:** Composite — two-mode entangled Gaussian.
**Cost model:** Spontaneous parametric down-conversion (SPDC) produces TMSV.
**Real wall?** Anti-correlated photon noise quantified by gain.
**Cross-domain wiring:** physics: EPR pair in CV. signal-processing-rf: correlated noise. statistics-probability: anti-correlated Gaussians.
**Notes:** Source of entanglement in CV QKD, photonic quantum computing.

### displacement-operator (cross-domain alias: `D(α)`, `phase-space-displacement`, `α-displacement`)
**Domain:** Quantum Computing
**Definition:** D(α) = exp(αa†−α*a). Shifts coherent state |β⟩ → e^{iIm(αβ*)}|α+β⟩.
**Atom or composite:** Atom — single-mode Gaussian unitary.
**Cost model:** Implemented via mixing with strong coherent state.
**Real wall?** No — easily implemented.
**Cross-domain wiring:** signal-processing-rf: IQ modulation. linear-algebra-matrix: Heisenberg-Weyl group. physics: coherent driving.
**Notes:** Generates Heisenberg-Weyl group with phase shifts.

### wigner-function (cross-domain alias: `W(q,p)`, `Wigner-quasi-probability`, `phase-space-distribution`)
**Domain:** Quantum Computing
**Definition:** W(q,p) = (1/π) ∫ dy ⟨q−y|ρ|q+y⟩ e^{2ipy}. Quasi-probability distribution; can be negative.
**Atom or composite:** Atom — phase-space representation of quantum state.
**Cost model:** Measured via homodyne/heterodyne + tomographic reconstruction.
**Real wall?** Negativity signals non-classicality.
**Cross-domain wiring:** signal-processing-rf: time-frequency distributions. statistics-probability: signed measure. linear-algebra-matrix: density-matrix Fourier transform.
**Notes:** Wigner (1932). Negativity = resource for quantum computational advantage (Mari-Eisert).

### p-function (cross-domain alias: `Glauber-Sudarshan-P`, `coherent-state-decomposition`, `P(α)`)
**Domain:** Quantum Computing
**Definition:** ρ = ∫ P(α)|α⟩⟨α| d²α. Singular distribution for non-classical states.
**Atom or composite:** Atom — diagonal coherent-state representation.
**Cost model:** Often ill-defined (delta-prime distributions).
**Real wall?** P-positivity ⇔ classical state.
**Cross-domain wiring:** signal-processing-rf: amplitude distribution. statistics-probability: pseudo-probability. physics: classicality criterion.
**Notes:** Glauber-Sudarshan (1963). Non-positive P ⇔ quantum non-classicality.

### q-function (cross-domain alias: `Husimi-Q`, `coherent-state-projection`, `Q(α)`)
**Domain:** Quantum Computing
**Definition:** Q(α) = (1/π)⟨α|ρ|α⟩. Always nonnegative; smoothed Wigner function.
**Atom or composite:** Atom — coherent-state representation.
**Cost model:** Direct projection on coherent state.
**Real wall?** Loses fine phase-space features.
**Cross-domain wiring:** statistics-probability: classical-like density. signal-processing-rf: smoothed phase-space. physics: heterodyne detection.
**Notes:** Husimi (1940). Q always positive — link to classical limit.

### husimi-function (cross-domain alias: `Husimi-distribution`, `Q-distribution`, `Gaussian-smoothed-Wigner`)
**Domain:** Quantum Computing
**Definition:** Same as Q-function: convolution of Wigner with vacuum Wigner G(q,p)=2 exp(−q²−p²)/π.
**Atom or composite:** Atom — phase-space distribution.
**Cross-domain wiring:** signal-processing-rf: Gaussian-filtered TF representation. statistics-probability: nonnegative density.
**Notes:** Standard tool for visualizing quantum states in phase space.

### beam-splitter (cross-domain alias: `BS-transformation`, `linear-optical-mixer`, `two-mode-rotation`)
**Domain:** Quantum Computing
**Definition:** Two-mode passive Gaussian unitary: a → cos θ·a + sin θ·b, b → −sin θ·a + cos θ·b.
**Atom or composite:** Atom — two-mode linear optic.
**Cost model:** Single passive optical element.
**Real wall?** No — fundamental optical building block.
**Cross-domain wiring:** signal-processing-rf: hybrid coupler / power divider. linear-algebra-matrix: SU(2) rotation. physics: linear optics universality.
**Notes:** Together with phase shifters spans all passive Gaussian unitaries (Reck-Zeilinger).

### phase-shifter (cross-domain alias: `R(φ)`, `single-mode-phase-rotation`, `optical-delay`)
**Domain:** Quantum Computing
**Definition:** R(φ) = e^{iφa†a}. Rotates quadratures in phase space by angle φ.
**Atom or composite:** Atom — single-mode passive op.
**Cost model:** Pathlength adjustment (waveguide modulator).
**Real wall?** No — but stability sensitive.
**Cross-domain wiring:** signal-processing-rf: delay line / phase modulator. linear-algebra-matrix: U(1) rotation. physics: free-space propagation.
**Notes:** Combined with beam splitters realizes any passive Gaussian unitary (Reck-Zeilinger).

### klm-scheme (cross-domain alias: `Knill-Laflamme-Milburn`, `linear-optical-QC`, `LOQC`)
**Domain:** Quantum Computing
**Definition:** Universal photonic computing using only single photons, linear optics (BS+phase), photodetectors, feed-forward; nonlinearity from measurement.
**Atom or composite:** Composite — measurement-induced nonlinearity.
**Cost model:** Probabilistic gates; teleportation-based scaling.
**Real wall?** Photon-loss tolerance; nondeterministic gates demand multiplexing.
**Cross-domain wiring:** signal-processing-rf: linear optics + post-processing. physics: photonic quantum information. statistics-probability: heralded gates.
**Notes:** Knill-Laflamme-Milburn (2001). Established photonics as viable QC platform.

### photonic-mbqc (cross-domain alias: `photonic-cluster-state-MBQC`, `Browne-Rudolph`, `LOQC-MBQC`)
**Domain:** Quantum Computing
**Definition:** Build photonic cluster state via fusion gates; implement universal QC by adaptive measurements.
**Atom or composite:** Composite — fusion-based cluster generation + measurement.
**Cost model:** Probabilistic fusion → multiplexed sources for percolation.
**Real wall?** Photon loss kills percolation below threshold.
**Cross-domain wiring:** physics: photonic quantum networks. signal-processing-rf: multiplexed photon sources. graph-theory: percolation on graph.
**Notes:** Browne-Rudolph (2005); architecture of PsiQuantum, Xanadu.

### gbs (cross-domain alias: `Gaussian-boson-sampling`, `Hamilton-Bradler-Cilluffo`, `photonic-supremacy`)
**Domain:** Quantum Computing
**Definition:** Sample photon-detection patterns from output of Gaussian state through linear interferometer. Probability ∝ |Haf(M)|².
**Atom or composite:** Composite — Gaussian input + linear optics + photon counting.
**Cost model:** Classical: #P-hard (hafnian); quantum: linear in modes.
**Real wall?** Photon loss & distinguishability erode advantage.
**Cross-domain wiring:** classical-complexity: #P-hardness. signal-processing-rf: random multiport. linear-algebra-matrix: hafnian/permanent.
**Notes:** Hamilton et al. (2017). Xanadu's Jiuzhang demonstrates quantum advantage with GBS.

---

## Measurement-Based Quantum Computing

### cluster-state (cross-domain alias: `Raussendorf-Briegel-state`, `lattice-graph-state`, `MBQC-resource`)
**Domain:** Quantum Computing
**Definition:** Graph state on regular lattice (2D, 3D). Universal resource for MBQC under adaptive single-qubit measurements.
**Atom or composite:** Composite — stabilizer state on lattice.
**Cost model:** Each vertex |+⟩, edges CZ.
**Real wall?** Requires high-fidelity entanglement across lattice.
**Cross-domain wiring:** graph-theory: lattice graph. statistics-probability: percolation if probabilistic. quantum-error-correction: foundation of fault-tolerant MBQC.
**Notes:** Raussendorf-Briegel (2001). Foundation of one-way QC.

### graph-state (cross-domain alias: `general-graph-state`, `Hein-Eisert-Briegel`, `stabilizer-graph`)
**Domain:** Quantum Computing
**Definition:** For graph G=(V,E): apply H⊗ⁿ to |0⟩^V, then CZ for each (i,j)∈E. Stabilizers S_v = X_v · Π_{u∈N(v)} Z_u.
**Atom or composite:** Composite — Hadamard + CZ pattern.
**Cost model:** O(|E|) CZ gates.
**Real wall?** No — universal stabilizer construction.
**Cross-domain wiring:** graph-theory: graph structure ↔ entanglement structure. quantum-error-correction: stabilizer codes are graph states up to LC. linear-algebra-matrix: GF(2) adjacency.
**Notes:** Hein-Eisert-Briegel (2004). Foundation for MBQC, QEC, communication.

### one-way-qc (cross-domain alias: `Raussendorf-1WQC`, `measurement-based-QC`, `MBQC`)
**Domain:** Quantum Computing
**Definition:** Algorithm = adaptive sequence of single-qubit measurements on a resource graph state. Computational power equivalent to circuit model.
**Atom or composite:** Composite — measurements + feed-forward.
**Cost model:** Same as circuit model up to constants.
**Real wall?** Resource state preparation is the bottleneck.
**Cross-domain wiring:** statistics-probability: adaptive measurement strategy. signal-processing-rf: forward error propagation. quantum-error-correction: topological MBQC for fault tolerance.
**Notes:** Raussendorf-Briegel (2001). Equivalence of MBQC and circuit model.

### brickwork-state (cross-domain alias: `Broadbent-brickwork`, `UBQC-resource`, `MBQC-with-prep`)
**Domain:** Quantum Computing
**Definition:** Universal MBQC resource state on brickwork lattice; particularly suited to blind quantum computing (client prepares qubits hiding angles).
**Atom or composite:** Composite — fixed lattice topology.
**Cost model:** Same scaling as cluster state but more structured.
**Real wall?** No.
**Cross-domain wiring:** cryptography-advanced: blind quantum computing. graph-theory: brick lattice. quantum-error-correction: works with topological codes.
**Notes:** Broadbent, Fitzsimons, Kashefi (2009). Backbone of blind/verifiable QC.

### fusion-measurement (cross-domain alias: `type-I-II-fusion`, `Browne-Rudolph-fusion`, `photonic-entanglement-fusion`)
**Domain:** Quantum Computing
**Definition:** Two-photon Bell measurement that probabilistically fuses two graph states; type-I succeeds with 50%, type-II with 50%.
**Atom or composite:** Composite — projective Bell measurement on shared mode.
**Cost model:** O(1/0.5) trials per fusion; multiplexed sources for percolation.
**Real wall?** Probabilistic — overcome via multiplexing.
**Cross-domain wiring:** physics: photon Bell measurement. graph-theory: graph union via fusion. statistics-probability: percolation threshold.
**Notes:** Browne-Rudolph (2005). PsiQuantum's "fusion-based QC" foundation.

### stabilizer-graph-formalism (cross-domain alias: `LC-equivalence`, `local-Clifford-graph`, `Van-den-Nest-Dehaene-De-Moor`)
**Domain:** Quantum Computing
**Definition:** Local Clifford operations on graph state map graph G to G' via local complementation. Generates all stabilizer states up to LC.
**Atom or composite:** Atom — graph-theoretic stabilizer normal form.
**Cost model:** Local complementation = O(|V|²).
**Real wall?** No.
**Cross-domain wiring:** graph-theory: local complementation. linear-algebra-matrix: symplectic GF(2). quantum-error-correction: stabilizer-code transformations.
**Notes:** Van den Nest et al. (2004). Combinatorial framework for stabilizer states.

---

## Adiabatic & Quantum Annealing

### adiabatic-theorem (cross-domain alias: `Born-Fock-theorem`, `quantum-adiabatic-principle`, `slow-evolution`)
**Domain:** Quantum Computing
**Definition:** System evolving under slowly-varying H(t) stays in instantaneous eigenstate if T ≫ ‖∂_t H‖/g_min² where g_min is min spectral gap.
**Atom or composite:** Atom — foundational physics principle.
**Cost model:** Runtime scales as 1/g_min².
**Real wall?** Yes — small gap = exponential runtime.
**Cross-domain wiring:** physics: Born-Oppenheimer / adiabatic invariants. signal-processing-rf: chirped pulses. statistics-probability: slow MCMC mixing.
**Notes:** Born-Fock (1928). Basis of AQC; "adiabatic = slow w.r.t. inverse gap squared."

### aqc (cross-domain alias: `adiabatic-quantum-computing`, `Farhi-Goldstone-Gutmann`, `Hamiltonian-interpolation`)
**Domain:** Quantum Computing
**Definition:** H(t) = (1−s(t))·H_init + s(t)·H_target. Prepare easy ground state of H_init, evolve slowly to H_target.
**Atom or composite:** Composite — Hamiltonian interpolation.
**Cost model:** T ≫ 1/g_min².
**Real wall?** Polynomially equivalent to circuit model; in practice gaps may be tiny.
**Cross-domain wiring:** physics: ground-state finding. statistics-probability: simulated annealing analogue. classical-optimization: continuous interpolation.
**Notes:** Farhi-Goldstone-Gutmann (2000). Universal model of QC.

### quantum-annealing (cross-domain alias: `QA`, `Kadowaki-Nishimori`, `transverse-field-annealing`)
**Domain:** Quantum Computing
**Definition:** Anneal from transverse-field H = −Γ·Σ X_i to problem Ising H_P = Σ J_ij Z_i Z_j + h_i Z_i. Heuristic optimization at finite T.
**Atom or composite:** Composite — AQC with open-system dynamics.
**Cost model:** Polynomial annealing time on D-Wave-style hardware.
**Real wall?** Not universal (only stoquastic Hamiltonians); finite-temp effects.
**Cross-domain wiring:** statistics-probability: simulated quantum annealing. physics: transverse-field Ising model. classical-optimization: heuristic QUBO solver.
**Notes:** Kadowaki-Nishimori (1998). D-Wave commercial implementation.

### dwave-ising (cross-domain alias: `Ising-formulation`, `QUBO`, `binary-quadratic-model`)
**Domain:** Quantum Computing
**Definition:** Express problem as min over Ising H = Σ J_ij s_i s_j + Σ h_i s_i, s_i∈{−1,+1}. Polynomially equivalent to QUBO.
**Atom or composite:** Atom — combinatorial problem encoding.
**Cost model:** Reduction from many NP-hard problems (Lucas 2014).
**Real wall?** Embedding overhead onto hardware graph.
**Cross-domain wiring:** classical-optimization: NP-hard QUBO formulation. statistics-probability: Boltzmann distribution on Ising. physics-condensed-matter: spin glass.
**Notes:** Lucas (2014) catalogs Ising forms for many NP problems.

### minor-embedding (cross-domain alias: `chain-embedding`, `Cai-Macready-Roy`, `dense-to-sparse-mapping`)
**Domain:** Quantum Computing
**Definition:** Map logical problem graph G_L to physical hardware graph G_H by representing each logical variable with a chain of physical qubits.
**Atom or composite:** Composite — graph embedding.
**Cost model:** Chain length scales with problem density.
**Real wall?** Hardware connectivity (Chimera, Pegasus) limits embeddability.
**Cross-domain wiring:** graph-theory: graph minors. classical-optimization: layout problem. ml-training: similar to feature-graph mapping.
**Notes:** Cai-Macready-Roy (2014) heuristic embedder; SAPI tool in D-Wave SDK.

### chimera-pegasus (cross-domain alias: `Dwave-topology`, `Boothby-Bunyk`, `qubit-graph-topology`)
**Domain:** Quantum Computing
**Definition:** D-Wave qubit graphs: Chimera (4-regular bipartite K_{4,4} units); Pegasus (next-gen, higher connectivity).
**Atom or composite:** Atom — hardware topology.
**Cost model:** Connectivity drives embedding cost.
**Real wall?** Sparse connectivity is current bottleneck.
**Cross-domain wiring:** graph-theory: fixed sparse graph. classical-optimization: hardware-aware embedding. signal-processing-rf: chip layout.
**Notes:** Boothby et al. Pegasus enables denser problems; future Zephyr topology even denser.

### reverse-annealing (cross-domain alias: `RA`, `Perdomo-Ortiz-reverse-anneal`, `local-search-quantum`)
**Domain:** Quantum Computing
**Definition:** Initialize with classical state, ramp transverse field UP then back DOWN. Performs local search around initial state.
**Atom or composite:** Composite — non-monotonic annealing schedule.
**Cost model:** Comparable to forward annealing.
**Real wall?** No — but no proven advantage.
**Cross-domain wiring:** classical-optimization: tabu/local search. statistics-probability: Metropolis exploration. signal-processing-rf: bidirectional sweeping.
**Notes:** Available on D-Wave hardware; used for refinement of classical solutions.

### diabatic-transition (cross-domain alias: `Landau-Zener`, `level-crossing`, `non-adiabatic-passage`)
**Domain:** Quantum Computing
**Definition:** Probability P=exp(−2π·Δ²/(ħ·dE/dt)) of remaining in initial state through avoided crossing. Diabatic = high velocity, stays in same diabatic state.
**Atom or composite:** Atom — non-adiabatic process.
**Cost model:** Limits maximum annealing speed.
**Real wall?** Yes — diabatic loss at small gaps unavoidable.
**Cross-domain wiring:** physics: Landau-Zener formula. signal-processing-rf: rapid passage. statistics-probability: tunneling probability.
**Notes:** Landau & Zener (1932). Quantitative limit on adiabaticity violation.

### stoquastic-hamiltonian (cross-domain alias: `sign-free-Hamiltonian`, `Bravyi-Divincenzo-Oliveira`, `non-frustrated-sign`)
**Domain:** Quantum Computing
**Definition:** Hamiltonian with non-positive off-diagonal elements in computational basis (sign-problem-free). Includes transverse-field Ising, ferromagnetic Heisenberg.
**Atom or composite:** Atom — class of Hamiltonians.
**Cost model:** QMC simulable in polynomial time.
**Real wall?** Stoquastic AQC may NOT be universal (BQP).
**Cross-domain wiring:** statistics-probability: sign-problem-free QMC. physics-condensed-matter: ferromagnetic systems. classical-complexity: StoqMA vs BQP.
**Notes:** Bravyi et al. (2008). D-Wave hardware is stoquastic.

---

## Topological Quantum Computing

### anyons-abelian (cross-domain alias: `abelian-anyons`, `fractional-statistics`, `Wilczek-anyon`)
**Domain:** Quantum Computing
**Definition:** 2D quasi-particles with exchange phase e^{iθ}, θ∉{0,π}. Abelian fusion algebra (group).
**Atom or composite:** Atom — quasi-particle of 2D topological phase.
**Cost model:** Realized in FQHE (fractional quantum Hall effect) states.
**Real wall?** Abelian anyons NOT sufficient for universal QC.
**Cross-domain wiring:** physics-condensed-matter: FQHE. group-theory: braid group representations. signal-processing-rf: phase-encoded info.
**Notes:** Wilczek (1982). FQHE quasi-holes at ν=1/3 are abelian.

### anyons-non-abelian (cross-domain alias: `non-abelian-anyons`, `Kitaev-non-abelian`, `topological-qubits`)
**Domain:** Quantum Computing
**Definition:** 2D quasi-particles whose braiding implements non-commuting unitary operations on degenerate fusion-space.
**Atom or composite:** Atom — universal computational resource (some types).
**Cost model:** Computation = adiabatic braiding.
**Real wall?** Experimentally elusive; Ising anyons NOT universal (need T-magic).
**Cross-domain wiring:** physics-condensed-matter: ν=5/2 FQHE Moore-Read. group-theory: non-abelian braid representations. quantum-error-correction: inherent topological protection.
**Notes:** Kitaev (2003); Nayak-Simon-Stern-Freedman (2008). Holy grail of fault tolerance.

### majorana-zero-modes (cross-domain alias: `MZM`, `Kitaev-chain`, `topological-superconductor-modes`)
**Domain:** Quantum Computing
**Definition:** Self-conjugate fermionic modes γ=γ† at ends of 1D p-wave superconductor. Pair of MZMs = topological qubit.
**Atom or composite:** Atom — quasi-particle.
**Cost model:** Realized via proximity-induced p-wave in semiconducting nanowires.
**Real wall?** Experimental verification controversial; partial signatures only.
**Cross-domain wiring:** physics-condensed-matter: topological superconductivity. linear-algebra-matrix: BdG Hamiltonians. cryptography-advanced: braiding-based gates.
**Notes:** Kitaev (2001). Microsoft Station-Q centered around MZM-based QC.

### ising-anyons (cross-domain alias: `Majorana-anyons`, `non-universal-non-abelian`, `MR-Pfaffian-anyons`)
**Domain:** Quantum Computing
**Definition:** Non-abelian anyons with fusion rules σ×σ=1+ψ. Braiding implements Clifford gates; T-magic needed for universality.
**Atom or composite:** Atom — specific anyon model.
**Cost model:** Universal Clifford by braiding; T via magic state distillation.
**Real wall?** Experimental realization in ν=5/2 FQHE remains contested.
**Cross-domain wiring:** group-theory: braid group of σs. quantum-error-correction: Clifford-transversal-by-braid. physics-condensed-matter: paired states.
**Notes:** Moore-Read (1991). Bravyi (2006) showed Ising + magic states = universal.

### fibonacci-anyons (cross-domain alias: `τ-anyons`, `universal-non-abelian`, `Read-Rezayi-anyons`)
**Domain:** Quantum Computing
**Definition:** Single non-abelian τ with fusion τ×τ=1+τ. Braiding alone is universal for QC (dense in SU(2)).
**Atom or composite:** Atom — universal anyon model.
**Cost model:** Braid words approximate gates to arbitrary precision (Solovay-Kitaev).
**Real wall?** Even more elusive experimentally than Ising; ν=12/5 candidate.
**Cross-domain wiring:** group-theory: golden ratio braid representations. quantum-error-correction: universal-by-braid (no magic). physics-condensed-matter: SU(2)_3 anyons.
**Notes:** Bonesteel-Hormozi-Zikos-Simon (2005). Theoretical ideal for topological QC.

### braid-statistics (cross-domain alias: `braid-group-representation`, `non-trivial-monodromy`, `anyon-exchange`)
**Domain:** Quantum Computing
**Definition:** Exchanging two anyons implements unitary R; double exchange = monodromy R². For non-abelian: R is matrix on fusion space.
**Atom or composite:** Atom — exchange operator on anyon space.
**Cost model:** Topologically protected — exact unitary regardless of path details.
**Real wall?** Operation only protected against local noise.
**Cross-domain wiring:** group-theory: braid group B_n. physics: 2D particle exchange. quantum-error-correction: inherent gates.
**Notes:** Foundation of topological QC; replaces gates with braid words.

### toric-code (cross-domain alias: `Kitaev-toric-code`, `Z2-lattice-gauge`, `topological-stabilizer-code`)
**Domain:** Quantum Computing
**Definition:** Stabilizer code on torus: vertex operators A_v=∏X (around vertex), plaquette B_p=∏Z (around face). Encodes 2 logical qubits.
**Atom or composite:** Composite — 2D lattice stabilizer code.
**Cost model:** Threshold ~10% (with perfect syndrome), ~1% with noisy.
**Real wall?** Periodic boundary unphysical → planar surface code is realistic version.
**Cross-domain wiring:** physics-condensed-matter: Z₂ topological order. graph-theory: cellular complex. group-theory: ground space ↔ first homology of torus.
**Notes:** Kitaev (2003). Conceptual ancestor of surface code; idealized toy model.

### surface-code-topological (cross-domain alias: `planar-toric-code`, `2D-stabilizer-topological`, `Fowler-surface-code`)
**Domain:** Quantum Computing
**Definition:** Toric code with open boundaries — 2D planar stabilizer code with rough/smooth edges encoding logical qubit pair.
**Atom or composite:** Composite — stabilizer code on planar surface.
**Cost model:** [[d²,1,d]]; threshold ~1% with realistic noise.
**Real wall?** Practical scaling: thousands of physical per logical at low error.
**Cross-domain wiring:** physics-condensed-matter: topological order. graph-theory: planar duality (vertex ↔ face). information-theory-coding: distance from cycle length.
**Notes:** Bravyi-Kitaev (1998); Fowler et al. (2012). Industry default; Google's Willow chip.

### color-code-topological (cross-domain alias: `Bombin-color-code`, `3-colorable-trivalent`, `topological-color-code`)
**Domain:** Quantum Computing
**Definition:** Topological order on 3-colorable trivalent lattice. Each face = stabilizer; supports transversal Clifford in 2D.
**Atom or composite:** Composite.
**Cost model:** Lower threshold than surface but richer logical gate set.
**Real wall?** Threshold ~0.1-0.5% typically.
**Cross-domain wiring:** graph-theory: 3-colorings of trivalent graphs. group-theory: transversal Clifford. physics-condensed-matter: D₄ topological order.
**Notes:** Bombin (2006). Twist defects and code-deformation enable interesting logic.

---

## Quantum Supremacy Benchmarks

### random-circuit-sampling (cross-domain alias: `RCS`, `Boixo-Isakov-Smelyanskiy`, `Sycamore-benchmark`)
**Domain:** Quantum Computing
**Definition:** Run random shallow circuit U, sample bitstrings from |⟨x|U|0⟩|². Classical simulation cost: exponential.
**Atom or composite:** Composite — random circuit + measurement.
**Cost model:** Quantum: O(depth·N). Classical: exponential at 50+ qubits.
**Real wall?** Conjectured quantum advantage; tensor-network classical attacks erode by depth.
**Cross-domain wiring:** statistics-probability: Porter-Thomas distribution. classical-complexity: ANTI-concentration conjecture. signal-processing-rf: random matrix theory.
**Notes:** Google Sycamore (2019); USTC Zuchongzhi. Centerpiece of quantum-advantage demos.

### xeb (cross-domain alias: `cross-entropy-benchmarking`, `linear-XEB`, `Arute-Sycamore-fidelity`)
**Domain:** Quantum Computing
**Definition:** XEB = (2^N · ⟨P_U(x_i)⟩) − 1 estimates circuit fidelity. P_U(x) = |⟨x|U|0⟩|² from classical simulation.
**Atom or composite:** Composite — fidelity-estimation protocol.
**Cost model:** Classical computation of P_U scales exponentially.
**Real wall?** Beyond ~50 qubits, classical reference becomes intractable.
**Cross-domain wiring:** statistics-probability: log-likelihood ratio. signal-processing-rf: matched-filter fidelity. ml-training: cross-entropy loss.
**Notes:** Boixo et al. (2018). Currency of quantum-advantage claims.

### boson-sampling (cross-domain alias: `Aaronson-Arkhipov`, `permanent-sampling`, `linear-optical-supremacy`)
**Domain:** Quantum Computing
**Definition:** Sample n photons through random m-mode interferometer; output probability ∝ |Perm(M)|². Classical sampling #P-hard.
**Atom or composite:** Composite — single-photon source + linear optics + detectors.
**Cost model:** #P-hard classical permanent vs O(n·m) quantum.
**Real wall?** Photon distinguishability + loss; Gaussian boson sampling more realistic.
**Cross-domain wiring:** classical-complexity: #P-hardness, polynomial hierarchy collapse. linear-algebra-matrix: matrix permanent. signal-processing-rf: random interferometer.
**Notes:** Aaronson-Arkhipov (2011). Original quantum-advantage proposal; now GBS variant dominates.

### iqp-circuits (cross-domain alias: `Instantaneous-Quantum-Polynomial`, `Bremner-Jozsa-Shepherd`, `commuting-circuits`)
**Domain:** Quantum Computing
**Definition:** Circuits of form H⊗ⁿ·D·H⊗ⁿ with D commuting diagonal gates. Sampling classically hard under standard assumptions.
**Atom or composite:** Composite — diagonal sandwich.
**Cost model:** Constant-depth but sampling #P-hard classically.
**Real wall?** Cannot be made universal; restricted compute model.
**Cross-domain wiring:** classical-complexity: polynomial hierarchy collapse. statistics-probability: Ising partition functions. signal-processing-rf: Walsh-Hadamard transforms.
**Notes:** Bremner-Jozsa-Shepherd (2010). Conceptual ancestor of XEB-style supremacy.

### peaked-circuit (cross-domain alias: `Aaronson-Zhang-peaked`, `verifiable-supremacy`, `concentrated-output`)
**Domain:** Quantum Computing
**Definition:** Circuit whose output concentrates on a small set of strings, designed to be classically intractable but quantumly verifiable.
**Atom or composite:** Composite — specially structured circuit.
**Cost model:** Quantum: poly-time sampling. Classical: presumed hard.
**Real wall?** Construction techniques active research.
**Cross-domain wiring:** classical-complexity: avg-case hardness with verification. statistics-probability: concentrated distributions. ml-training: trapdoor functions analogue.
**Notes:** Aaronson-Zhang (2024); verifiable advantage candidate.

### anti-concentration (cross-domain alias: `anticoncentration-conjecture`, `output-spread`, `Brandão-Harrow`)
**Domain:** Quantum Computing
**Definition:** Random quantum circuit output probabilities are anti-concentrated: many strings have weight Ω(1/N). Required for hardness of sampling.
**Atom or composite:** Atom — distributional property.
**Real wall?** Holds for sufficient-depth random circuits; failure for shallow.
**Cross-domain wiring:** statistics-probability: anti-concentration inequalities. linear-algebra-matrix: random matrix theory. signal-processing-rf: ergodic distributions.
**Notes:** Brandão-Harrow-Horodecki (2013). Establishes Porter-Thomas in depth.

### porter-thomas-distribution (cross-domain alias: `PT-distribution`, `chaotic-Wigner`, `exponential-output-distribution`)
**Domain:** Quantum Computing
**Definition:** Distribution of |⟨x|ψ⟩|² for Haar-random |ψ⟩: 2^N·p ~ Exp(1). Mean = 1/N, but huge variance.
**Atom or composite:** Atom — universal random-circuit distribution.
**Cost model:** N/A — distributional.
**Real wall?** Empirical PT distribution is a fidelity signature.
**Cross-domain wiring:** statistics-probability: chi-squared with 2 DOF. linear-algebra-matrix: random Haar measure. physics: nuclear-physics random matrix theory.
**Notes:** Porter-Thomas (1956) for nuclear cross sections; rediscovered for random circuits.

---

## Hardware-Specific Primitives

### transmon-qubit (cross-domain alias: `Koch-transmon`, `superconducting-qubit`, `cQED-qubit`)
**Domain:** Quantum Computing
**Definition:** Superconducting qubit: Josephson junction + large capacitor; weakly anharmonic oscillator with E_J/E_C ≈ 50.
**Atom or composite:** Atom — physical qubit type.
**Cost model:** T1 ~ 100 µs, T2 ~ 50 µs typical; ~100 ns gates.
**Real wall?** Charge-noise insensitive (vs Cooper-pair box); residual leakage to higher states.
**Cross-domain wiring:** physics: anharmonic oscillator. signal-processing-rf: microwave control at 4-8 GHz. cryogenics: 10-30 mK dilution refrigerator.
**Notes:** Koch et al. (2007). Industry standard (Google, IBM, Rigetti).

### fluxonium-qubit (cross-domain alias: `Manucharyan-fluxonium`, `inductive-shunted-qubit`, `protected-qubit`)
**Domain:** Quantum Computing
**Definition:** Junction shunted by large inductance (kinetic). Highly anharmonic; long T1 due to small charge dispersion.
**Atom or composite:** Atom.
**Cost model:** T1 ~ 1 ms achievable.
**Real wall?** Slower gates than transmon; complex multilevel control.
**Cross-domain wiring:** physics: phase-slip junction. signal-processing-rf: low-frequency control (~500 MHz). cryogenics: similar regime.
**Notes:** Manucharyan-Koch-Glazman-Devoret (2009). Improving fast.

### ms-gate (cross-domain alias: `Mølmer-Sørensen-gate`, `ion-trap-XX-gate`, `entangling-via-vibrational-modes`)
**Domain:** Quantum Computing
**Definition:** Bichromatic laser drives create σ_x⊗σ_x interaction mediated by shared vibrational mode in ion trap.
**Atom or composite:** Atom — native ion-trap two-qubit gate.
**Cost model:** ~100 µs gates; all-to-all connectivity within trap.
**Real wall?** Heating, motional decoherence; trap-frequency stability.
**Cross-domain wiring:** physics: optical-vibrational coupling. signal-processing-rf: bichromatic light pulse design. linear-algebra-matrix: XX rotation.
**Notes:** Mølmer-Sørensen (1999). IonQ, Quantinuum native gate.

### rydberg-blockade (cross-domain alias: `Lukin-Rydberg-blockade`, `neutral-atom-CZ`, `dipole-blockade`)
**Domain:** Quantum Computing
**Definition:** Excite atom to Rydberg state |r⟩; strong dipole-dipole prevents nearby atom from being excited simultaneously → blockade-mediated CZ.
**Atom or composite:** Atom — native neutral-atom two-qubit gate.
**Cost model:** ~100 ns gates; selectable connectivity via optical tweezers.
**Real wall?** Spontaneous emission from Rydberg; atom loss.
**Cross-domain wiring:** physics: dipole interactions. signal-processing-rf: laser control of atomic levels. graph-theory: dynamic connectivity.
**Notes:** Jaksch et al. (2000). QuEra, Pasqal, Atom Computing exploit this.

### nv-center (cross-domain alias: `nitrogen-vacancy`, `diamond-spin-qubit`, `NV-magnetometry`)
**Domain:** Quantum Computing
**Definition:** Spin-1 defect in diamond: ms=0, ±1 ground states. Optical initialization & readout via spin-dependent fluorescence.
**Atom or composite:** Atom — solid-state spin qubit.
**Cost model:** Room-temperature operation; ms readout via PL.
**Real wall?** Limited scalability; low gate fidelity vs solid-state competitors.
**Cross-domain wiring:** physics: diamond defect. signal-processing-rf: microwave + optical control. sensing: nanoscale magnetometry.
**Notes:** Premier quantum sensor (single-spin NMR); secondary as qubit.

### silicon-spin-qubit (cross-domain alias: `Kane-qubit`, `silicon-MOS-qubit`, `donor-spin-qubit`)
**Domain:** Quantum Computing
**Definition:** Electron or nuclear spin in silicon (P donor, quantum dot). Long T2 due to weak hyperfine in isotopically pure ²⁸Si.
**Atom or composite:** Atom — solid-state spin qubit.
**Cost model:** T2 > 1 s for nuclear spins.
**Real wall?** Fabrication uniformity; readout via spin-to-charge conversion.
**Cross-domain wiring:** physics: ²⁸Si "semiconductor vacuum". signal-processing-rf: ESR/NMR-frequency control. semiconductor-fabrication: CMOS-compatible.
**Notes:** Kane (1998). Intel, Diraq, SQC pursuing.

### topological-qubit-hardware (cross-domain alias: `Microsoft-topological-qubit`, `MZM-qubit`, `nanowire-qubit`)
**Domain:** Quantum Computing
**Definition:** Pair of Majorana zero modes encoding qubit; braided via T-junction or measurement-only protocols.
**Atom or composite:** Composite — engineered topological superconductor.
**Cost model:** Hypothetically very low logical error rate.
**Real wall?** Experimental realization controversial as of 2024 (Microsoft 2023 retraction).
**Cross-domain wiring:** physics-condensed-matter: topological superconductivity. quantum-error-correction: hardware-level protection. group-theory: braid statistics.
**Notes:** Microsoft Station-Q's program; long-term bet on inherent error suppression.

### photonic-qpu (cross-domain alias: `photonic-quantum-computer`, `linear-optical-QPU`, `LOQC-architecture`)
**Domain:** Quantum Computing
**Definition:** Computing platform: single-photon sources, programmable interferometer, photodetectors, feed-forward. KLM or MBQC-based.
**Atom or composite:** Composite — full photonic stack.
**Cost model:** Room-temp operation; loss-limited rather than coherence-limited.
**Real wall?** Photon-loss tolerance threshold (~3 dB/km for fiber).
**Cross-domain wiring:** physics: linear optics. signal-processing-rf: optical multiplexing. cryptography-advanced: networking-ready.
**Notes:** PsiQuantum, Xanadu architectures. Promises room-temp scalability via fusion-based QC.

---

## Compiler Primitives

### solovay-kitaev (cross-domain alias: `SK-theorem`, `gate-decomposition-theorem`, `efficient-Clifford+T-approximation`)
**Domain:** Quantum Computing
**Definition:** Any single-qubit unitary can be approximated to error ε using O(log^c(1/ε)) gates from any universal finite set, c ≈ 3.97.
**Atom or composite:** Atom — gate-synthesis theorem.
**Cost model:** Recursive Solovay-Kitaev algorithm: O(log^c(1/ε)).
**Real wall?** No — but newer Ross-Selinger gives O(log(1/ε)) for Clifford+T.
**Cross-domain wiring:** group-theory: dense subgroup of SU(2). numerics: polynomial-log approximation. ml-training: function approximation by primitive set.
**Notes:** Solovay (1995), Kitaev (1997). Foundational result of fault-tolerant QC.

### ross-selinger (cross-domain alias: `optimal-Clifford+T`, `RS-synthesis`, `number-theoretic-T-count`)
**Domain:** Quantum Computing
**Definition:** Optimal Clifford+T approximation of single-qubit unitary using algebraic number theory; produces minimal T-count for diagonal rotations.
**Atom or composite:** Composite — number-theoretic decomposition.
**Cost model:** O(log(1/ε)) T-count — optimal up to constants.
**Real wall?** No — provably tight.
**Cross-domain wiring:** group-theory: ring Z[ω] of cyclotomic integers. number-theory: Diophantine approximation. cryptography-advanced: lattice algorithms.
**Notes:** Ross-Selinger (2014). Default for compiling rotations to fault-tolerant gates.

### t-count-optimization (cross-domain alias: `T-minimization`, `Amy-Maslov-Mosca`, `phase-polynomial-reduction`)
**Domain:** Quantum Computing
**Definition:** Minimize number of T-gates in a circuit while preserving function. Uses phase polynomial representation + matroid optimization.
**Atom or composite:** Composite — circuit optimization pass.
**Cost model:** NP-hard in general; heuristics achieve substantial reduction.
**Real wall?** T-gates dominate fault-tolerant cost → critical for resource estimates.
**Cross-domain wiring:** classical-optimization: combinatorial minimization. linear-algebra-matrix: GF(2) row ops. compilers-traditional: classical loop optimization analogue.
**Notes:** Amy-Maslov-Mosca-Roetteler (2014). Heavy active research; T-par, TODD, etc.

### clifford-t-decomposition (cross-domain alias: `CT-decomp`, `Selinger-decomp`, `exact-CT-synthesis`)
**Domain:** Quantum Computing
**Definition:** Decompose unitaries from {Cliffords + T} group exactly via canonical form using ring Z[ω], ω = e^{iπ/4}.
**Atom or composite:** Composite.
**Cost model:** Exact decomposition for unitaries representable in Z[ω].
**Real wall?** Not all unitaries — only those over the ring.
**Cross-domain wiring:** group-theory: Clifford hierarchy. number-theory: cyclotomic rings. quantum-error-correction: fault-tolerant gate set.
**Notes:** Kliuchnikov-Maslov-Mosca (2013). Exact T-count for representable unitaries.

### gate-teleportation (cross-domain alias: `teleportation-based-gate`, `Gottesman-Chuang-gate-teleportation`, `magic-state-injection-as-teleport`)
**Domain:** Quantum Computing
**Definition:** Apply gate U via teleportation through entangled "magic" resource state |Φ_U⟩ = (I⊗U)|Bell⟩. Bell measurement + Pauli correction yields U|ψ⟩.
**Atom or composite:** Composite — teleportation + ancillary state.
**Cost model:** Resource state preparation + Bell measurement.
**Real wall?** Used to apply non-Clifford gates in fault-tolerant schemes.
**Cross-domain wiring:** signal-processing-rf: receiver-side compensation. quantum-error-correction: magic-state injection. cryptography-advanced: blind quantum computation.
**Notes:** Gottesman-Chuang (1999). Foundation of fault-tolerant non-Clifford logic.

### ancilla-assisted-synthesis (cross-domain alias: `Jones-style-synthesis`, `catalyst-states`, `multi-qubit-magic`)
**Domain:** Quantum Computing
**Definition:** Implement gate using auxiliary qubits / catalytic resource states; reduce overall gate count by reusing ancillas.
**Atom or composite:** Composite — circuit + auxiliary state.
**Cost model:** Trade-off: more qubits for less depth/T-count.
**Real wall?** Ancilla budget limited in NISQ.
**Cross-domain wiring:** linear-algebra-matrix: dilation methods. quantum-error-correction: catalytic distillation. ml-training: reusable subcircuits.
**Notes:** Jones (2013); Beverland et al. Critical for resource minimization.

### qubit-routing-sabre (cross-domain alias: `SABRE-router`, `Li-Ding-Xie`, `swap-routing-heuristic`)
**Domain:** Quantum Computing
**Definition:** Heuristic algorithm to insert SWAPs satisfying hardware connectivity. Uses a "look-ahead" cost function over upcoming gates.
**Atom or composite:** Composite — compiler pass.
**Cost model:** Polynomial in circuit length and qubits.
**Real wall?** Optimal routing is NP-hard.
**Cross-domain wiring:** classical-optimization: token-swapping problem. graph-theory: vertex routing on connectivity graph. compilers-traditional: register allocation.
**Notes:** Li-Ding-Xie (2019). Qiskit default routing pass.

### layout-synthesis (cross-domain alias: `qubit-assignment`, `initial-layout`, `placement-problem`)
**Domain:** Quantum Computing
**Definition:** Map logical qubits to physical qubits minimizing communication. Uses graph isomorphism + locality heuristics.
**Atom or composite:** Composite — preprocessing pass.
**Cost model:** NP-hard exact; heuristics polynomial.
**Real wall?** No — but quality varies dramatically.
**Cross-domain wiring:** graph-theory: subgraph isomorphism. classical-optimization: VLSI placement. compilers-traditional: register allocation.
**Notes:** Maslov-Falconer-Mosca (2008); Cowtan et al. Quality affects circuit depth substantially.

### gate-scheduling (cross-domain alias: `quantum-scheduling`, `commuting-gate-parallelism`, `ASAP-scheduling`)
**Domain:** Quantum Computing
**Definition:** Schedule gates respecting dependencies, hardware connectivity, calibration constraints (e.g., crosstalk). ASAP / ALAP variants.
**Atom or composite:** Composite — compiler pass.
**Cost model:** Linear in gate count for greedy; optimal NP-hard.
**Real wall?** Crosstalk-aware scheduling improves observed fidelity.
**Cross-domain wiring:** compilers-traditional: instruction scheduling. classical-optimization: DAG scheduling. signal-processing-rf: pulse-program timing.
**Notes:** Tannu et al. (2019). Important for parallel-execution-rich architectures.

### circuit-knitting (cross-domain alias: `circuit-cutting`, `wire-cutting`, `gate-cutting`)
**Domain:** Quantum Computing
**Definition:** Decompose large circuit into smaller subcircuits via quasi-probability cutting; combine classical post-processing. Exponential sampling overhead.
**Atom or composite:** Composite — quasi-prob decomposition.
**Cost model:** O(γ^(2k)) shots for k cuts; γ depends on cut location.
**Real wall?** Exponential overhead — but enables larger problems on smaller QPUs.
**Cross-domain wiring:** statistics-probability: importance sampling. signal-processing-rf: tensor network contraction. linear-algebra-matrix: bilinear decomposition.
**Notes:** Peng et al. (2020); Mitarai-Fujii. Active research in distributed quantum computing.

---

## Quantum Communication

### superdense-coding (cross-domain alias: `Bennett-Wiesner-superdense`, `2-classical-bits-via-1-qubit`, `dense-coding`)
**Domain:** Quantum Computing
**Definition:** Share Bell pair; sender applies one of {I,X,Z,XZ} to encode 2 bits in 1 qubit; receiver does Bell measurement.
**Atom or composite:** Composite — Bell-pair + local Pauli + Bell measurement.
**Cost model:** 1 qubit transmitted, but 1 ebit pre-shared.
**Real wall?** No — but consumes entanglement.
**Cross-domain wiring:** information-theory-coding: channel capacity = 2 for ebit-assisted. signal-processing-rf: multiplex via shared resource. cryptography-advanced: secure 2-bit channel.
**Notes:** Bennett-Wiesner (1992). Doubles classical capacity with ebit assistance.

### entanglement-swapping-detailed (cross-domain alias: `ES`, `Bell-measurement-swap`, `quantum-relay-primitive`)
**Domain:** Quantum Computing
**Definition:** A-B and C-D pre-share Bell pairs. Bell measurement on B-C entangles A-D directly. Foundation of quantum repeaters.
**Atom or composite:** Composite — Bell measurement on middle pair.
**Cost model:** Probabilistic with linear optics; deterministic with deterministic Bell measurement.
**Real wall?** Distance scales linearly in repeaters (vs exp loss in fiber).
**Cross-domain wiring:** signal-processing-rf: relay station. information-theory-coding: extending entanglement range. cryptography-advanced: long-distance QKD.
**Notes:** Żukowski et al. (1993). Building block of quantum networks.

### bbpssw-purification (cross-domain alias: `BBPSSW-protocol`, `Bennett-Brassard-Popescu-Schumacher-Smolin-Wootters`, `entanglement-distillation-1way`)
**Domain:** Quantum Computing
**Definition:** Two noisy Bell pairs → CNOT and measure target → if outcomes match, keep source. Increases fidelity probabilistically.
**Atom or composite:** Composite — CNOT + measurement + selection.
**Cost model:** Yield drops geometrically; needs F > 0.5 to start.
**Real wall?** Asymptotic rate limited by Holevo-style bounds.
**Cross-domain wiring:** information-theory-coding: error correction in entanglement. cryptography-advanced: secret-key distillation. statistics-probability: postselection.
**Notes:** Bennett et al. (1996). Foundational distillation protocol.

### dejmps-purification (cross-domain alias: `DEJMPS-protocol`, `Deutsch-Ekert-Jozsa-Macchiavello-Popescu-Sanpera`, `improved-purification`)
**Domain:** Quantum Computing
**Definition:** Pre/post-rotate qubits with local Hadamards before BBPSSW. Better convergence, especially for Werner-state-like noise.
**Atom or composite:** Composite — basis-rotated BBPSSW.
**Cost model:** Same as BBPSSW; better fidelity gain per round.
**Real wall?** No.
**Cross-domain wiring:** information-theory-coding: rotated error decoder. cryptography-advanced: better key rates. statistics-probability: improved acceptance.
**Notes:** Deutsch et al. (1996). Often preferred over BBPSSW in practice.

### quantum-repeater (cross-domain alias: `QR`, `Briegel-Dur-Cirac-Zoller`, `nested-purification-protocol`)
**Domain:** Quantum Computing
**Definition:** Divide channel into segments; create entanglement per segment via ES + purification at each nesting level. Extends range without exp loss.
**Atom or composite:** Composite — hierarchical ES + purification.
**Cost model:** Polylog overhead in distance.
**Real wall?** Requires quantum memory, deterministic Bell measurement, high-fidelity local gates.
**Cross-domain wiring:** signal-processing-rf: repeater networks. information-theory-coding: end-to-end fidelity. cryptography-advanced: continental QKD.
**Notes:** Briegel-Dür-Cirac-Zoller (1998). Architecture for quantum internet.

### dlcz-protocol (cross-domain alias: `Duan-Lukin-Cirac-Zoller`, `atomic-ensemble-repeater`, `heralded-entanglement`)
**Domain:** Quantum Computing
**Definition:** Generate atom-photon entanglement via spontaneous Raman in atomic ensemble; entanglement swap via photon interference + detection.
**Atom or composite:** Composite — atom-photon entanglement + linear optics.
**Cost model:** Probabilistic heralding; multimode acceleration.
**Real wall?** Photon loss in transmission; ensemble memory time.
**Cross-domain wiring:** physics: atomic ensembles. signal-processing-rf: photon heralding. cryptography-advanced: practical quantum repeater.
**Notes:** Duan-Lukin-Cirac-Zoller (2001). Foundational for ensemble-based repeaters.

### satellite-qkd (cross-domain alias: `Micius-QKD`, `free-space-QKD`, `LEO-quantum-link`)
**Domain:** Quantum Computing
**Definition:** Distribute entangled / encoded photons from satellite to ground stations via free-space optical link. Avoids fiber loss for long distances.
**Atom or composite:** Composite — satellite-based entangled photon source + ground detection.
**Cost model:** Tens of kbit/s key rate for current Micius-class satellite.
**Real wall?** Atmospheric turbulence; daylight background; pointing precision.
**Cross-domain wiring:** signal-processing-rf: free-space optical comm. cryptography-advanced: intercontinental QKD. physics: photon flight time.
**Notes:** Yin et al. (2017). Chinese Micius first satellite QKD; further missions planned.

### mdi-qkd (cross-domain alias: `measurement-device-independent-QKD`, `Lo-Curty-Qi`, `untrusted-detector-QKD`)
**Domain:** Quantum Computing
**Definition:** Both Alice and Bob send weak coherent pulses to untrusted middle node performing Bell measurement; secure even with compromised detectors.
**Atom or composite:** Composite — decoy-state + Bell measurement.
**Cost model:** Lower rate than BB84 but closes detector side-channel.
**Real wall?** Photon loss limits practical range (~400 km in fiber).
**Cross-domain wiring:** cryptography-advanced: side-channel-free security. signal-processing-rf: optical Bell measurement. information-theory-coding: untrusted-relay model.
**Notes:** Lo, Curty, Qi (2012). Major step toward practical secure QKD.

### twin-field-qkd (cross-domain alias: `TF-QKD`, `Lucamarini-Yuan-Dynes-Shields`, `single-photon-interference-QKD`)
**Domain:** Quantum Computing
**Definition:** Phase-encoded single-photon interference at central node; key rate scales as √η (vs η for direct), bridging to repeater scaling.
**Atom or composite:** Composite — phase-encoded interference.
**Cost model:** Approaches PLOB rate bound √η; 600+ km demonstrated.
**Real wall?** Phase stabilization between distant lasers.
**Cross-domain wiring:** signal-processing-rf: optical phase locking. cryptography-advanced: repeaterless scaling. physics: single-photon interference.
**Notes:** Lucamarini et al. (2018). Closest practical approach to PLOB bound without repeaters.

### device-independent-qkd (cross-domain alias: `DI-QKD`, `Acín-Brunner-Gisin-Massar-Pironio-Scarani`, `Bell-test-QKD`)
**Domain:** Quantum Computing
**Definition:** Security from Bell-inequality violation; requires no assumptions about device internals — only loophole-free Bell test.
**Atom or composite:** Composite — Bell test + classical post-processing.
**Cost model:** Very low key rates with current devices; demonstrated in 2022.
**Real wall?** Detection-loophole-free Bell tests required.
**Cross-domain wiring:** cryptography-advanced: ultimate security. statistics-probability: Bell-inequality tests. physics-foundations: non-local correlations.
**Notes:** Acín et al. (2007); experimental demonstrations 2022 (Nadlinger, Zhang).

---

## Quantum Cryptography

### b92-protocol (cross-domain alias: `B92`, `Bennett-92`, `two-state-QKD`)
**Domain:** Quantum Computing
**Definition:** Send only two non-orthogonal states |0⟩, |+⟩; receiver makes USD-style measurement. Inconclusive outcomes discarded.
**Atom or composite:** Composite — 2-state QKD.
**Cost model:** Simpler than BB84 but lower efficiency.
**Real wall?** Vulnerable to PNS attacks without decoy states.
**Cross-domain wiring:** statistics-probability: unambiguous discrimination. cryptography-advanced: information-theoretic security. signal-processing-rf: non-orthogonal coding.
**Notes:** Bennett (1992). Simplified BB84 with security proofs.

### e91-protocol (cross-domain alias: `E91`, `Ekert-protocol`, `entanglement-based-QKD`)
**Domain:** Quantum Computing
**Definition:** Distribute Bell pairs; Alice/Bob measure in random bases including Bell-inequality-testing settings. Eavesdropping detected via reduced CHSH violation.
**Atom or composite:** Composite — Bell pair + CHSH test.
**Cost model:** Same as BB84 efficiency for key, plus statistics for CHSH.
**Real wall?** Requires high-quality entanglement distribution.
**Cross-domain wiring:** physics-foundations: Bell inequality. cryptography-advanced: monogamy-based security. statistics-probability: hypothesis testing on CHSH.
**Notes:** Ekert (1991). Connects QKD security to Bell-inequality violation.

### six-state-protocol (cross-domain alias: `6-state-QKD`, `Bruß-protocol`, `3-MUB-protocol`)
**Domain:** Quantum Computing
**Definition:** BB84 extended to 3 mutually unbiased bases (X, Y, Z). More information about eavesdropper; higher noise tolerance.
**Atom or composite:** Composite — 3-basis QKD.
**Cost model:** Slightly higher key rate after sifting at high noise.
**Real wall?** No.
**Cross-domain wiring:** linear-algebra-matrix: 3 mutually unbiased bases. cryptography-advanced: better eavesdropping detection. statistics-probability: more constraints per eavesdrop.
**Notes:** Bruß (1998). Better than BB84 above 11% QBER.

### decoy-state (cross-domain alias: `Hwang-decoy`, `Lo-Ma-Chen`, `PNS-defense`)
**Domain:** Quantum Computing
**Definition:** Randomly vary photon-number intensity of weak coherent pulses. Decoy statistics catch photon-number-splitting (PNS) attacks.
**Atom or composite:** Composite — randomized intensity + statistical detection.
**Cost model:** Minor protocol overhead; major security gain.
**Real wall?** Universal countermeasure to PNS — enables WCP-source QKD security proofs.
**Cross-domain wiring:** statistics-probability: hypothesis testing on intensities. cryptography-advanced: side-channel defense. signal-processing-rf: intensity modulation.
**Notes:** Hwang (2003), Lo et al. (2005). Standard in commercial QKD.

### finite-key-analysis (cross-domain alias: `Tomamichel-Lim-Gisin-Renner`, `non-asymptotic-QKD-security`, `finite-N-bounds`)
**Domain:** Quantum Computing
**Definition:** Provable QKD security for finite key length N including statistical fluctuation factors; bounds via smooth min/max entropy.
**Atom or composite:** Composite — statistical bound machinery.
**Cost model:** Stricter parameter choices for finite N; longer post-processing.
**Real wall?** No — but rates degrade for short keys.
**Cross-domain wiring:** statistics-probability: confidence intervals. information-theory-coding: smooth Rényi entropies. cryptography-advanced: real-world security guarantees.
**Notes:** Renner et al. (2005); Tomamichel et al. (2012). Required for deployable QKD.

### quantum-digital-signatures (cross-domain alias: `QDS`, `Gottesman-Chuang-signature`, `quantum-public-key-signatures`)
**Domain:** Quantum Computing
**Definition:** Sign classical message with quantum public key (set of non-orthogonal quantum states). Forgery resistant by no-cloning.
**Atom or composite:** Composite — distribute public-key states + verification protocol.
**Cost model:** Quantum memory for public key (in original); modern variants memory-free.
**Real wall?** No quantum memory for long-term key storage.
**Cross-domain wiring:** cryptography-advanced: signatures from no-cloning. statistics-probability: hypothesis testing on signature. information-theory-coding: error-correcting MAC.
**Notes:** Gottesman & Chuang (2001). Quantum analogue of digital signatures.

### quantum-coin-flipping (cross-domain alias: `QCF`, `Aharonov-Ta-Shma-Vazirani-Yao`, `weak-coin-flipping`)
**Domain:** Quantum Computing
**Definition:** Two distrusting parties flip fair coin. Quantum protocols achieve bias 1/√2 (strong) or arbitrarily small (weak).
**Atom or composite:** Composite — commit+reveal phases.
**Cost model:** Constant number of rounds.
**Real wall?** Cannot achieve bias 0 with finite resources (Kitaev impossibility).
**Cross-domain wiring:** cryptography-advanced: commitment schemes. statistics-probability: fair sampling. information-theory-coding: bit commitment relation.
**Notes:** Aharonov et al. (2000); Kitaev (2002) lower bound; Mochon (2007) weak coin flipping ε→0.

### quantum-bit-commitment-no-go (cross-domain alias: `Mayers-Lo-Chau`, `unconditional-QBC-impossibility`, `BC-no-go-theorem`)
**Domain:** Quantum Computing
**Definition:** No unconditionally secure quantum bit commitment exists: a cheating committer can perfectly cheat via EPR-style attack.
**Atom or composite:** Atom — no-go theorem.
**Cost model:** N/A — impossibility result.
**Real wall?** Fundamental — no entanglement-based escape.
**Cross-domain wiring:** cryptography-advanced: foundational limit. information-theory-coding: monogamy-of-entanglement consequence. physics-foundations: HJW theorem.
**Notes:** Mayers (1996); Lo-Chau (1997). Need additional assumptions (relativistic, computational) for QBC.

### wiesner-quantum-money (cross-domain alias: `quantum-money`, `Wiesner-money`, `unforgeable-banknote`)
**Domain:** Quantum Computing
**Definition:** Banknote = serial# + n qubits in random BB84-basis states. Bank verifies via known basis; counterfeiter blocked by no-cloning.
**Atom or composite:** Composite — quantum state + classical serial.
**Cost model:** O(n) qubits per banknote; bank-online verification.
**Real wall?** No long-term quantum memory available; quantum money frozen.
**Cross-domain wiring:** cryptography-advanced: unforgeable tokens. linear-algebra-matrix: 4-state encoding. statistics-probability: verification by sampling.
**Notes:** Wiesner (1970, published 1983). First quantum cryptography concept; sparked QKD.

### public-key-quantum-money (cross-domain alias: `Aaronson-Christiano`, `Zhandry-quantum-money`, `publicly-verifiable-money`)
**Domain:** Quantum Computing
**Definition:** Anyone (not just bank) can verify quantum money. Requires public-verification quantum state + computational assumption.
**Atom or composite:** Composite — public-key verification scheme.
**Cost model:** Verification = sequence of measurements + classical checks.
**Real wall?** Many proposals broken; secure constructions remain elusive.
**Cross-domain wiring:** cryptography-advanced: public-key crypto + quantum. linear-algebra-matrix: random states from cryptographic generators. ml-training: hard-instance generation.
**Notes:** Aaronson-Christiano (2012). Active research; lattice-based assumption attempts.

---

## Foundations

### no-cloning-theorem (cross-domain alias: `Wootters-Zurek`, `Dieks-no-clone`, `unclonability`)
**Domain:** Quantum Computing
**Definition:** No unitary U with U(|ψ⟩|0⟩) = |ψ⟩|ψ⟩ for arbitrary |ψ⟩. Implies inability to copy unknown quantum states.
**Atom or composite:** Atom — no-go theorem.
**Cost model:** N/A.
**Real wall?** Yes — fundamental quantum constraint.
**Cross-domain wiring:** cryptography-advanced: QKD security foundation. information-theory-coding: limits to channel coding. linear-algebra-matrix: linearity of QM.
**Notes:** Wootters-Zurek (1982); Dieks (1982). Foundational quantum information theorem.

### no-broadcasting-theorem (cross-domain alias: `Barnum-Caves-Fuchs-Jozsa-Schumacher`, `mixed-state-no-clone`, `noncommuting-broadcast`)
**Domain:** Quantum Computing
**Definition:** A set of mixed states {ρ_i} can be broadcast (each marginal = ρ_i) iff they pairwise commute.
**Atom or composite:** Atom — no-go theorem (generalization of no-cloning to mixed states).
**Cost model:** N/A.
**Real wall?** Yes.
**Cross-domain wiring:** linear-algebra-matrix: commuting sets are simultaneously diagonalizable. information-theory-coding: classical info can be broadcast, quantum cannot. statistics-probability: noncommuting probability distributions.
**Notes:** Barnum et al. (1996). Strict mixed-state generalization of no-cloning.

### no-deleting-theorem (cross-domain alias: `Pati-Braunstein`, `quantum-no-delete`, `unitary-information-conservation`)
**Domain:** Quantum Computing
**Definition:** Given two identical copies of |ψ⟩, cannot delete one to leave blank state. Unitary preserves information.
**Atom or composite:** Atom — no-go theorem.
**Cost model:** N/A.
**Real wall?** Yes — reversibility constraint.
**Cross-domain wiring:** physics: time reversal symmetry. information-theory-coding: information conservation. linear-algebra-matrix: invertibility of U.
**Notes:** Pati-Braunstein (2000). Counterpart to no-cloning.

### no-signaling-principle (cross-domain alias: `no-superluminal-comm`, `relativistic-no-signal`, `marginal-invariance`)
**Domain:** Quantum Computing
**Definition:** Local operations cannot affect remote marginal density matrices. ρ_A = Tr_B(ρ_{AB}) is invariant under unitary on B.
**Atom or composite:** Atom — fundamental constraint.
**Cost model:** N/A.
**Real wall?** Yes — built-into QM, consistent with relativity.
**Cross-domain wiring:** physics: special relativity. cryptography-advanced: secure communication. information-theory-coding: locality constraint.
**Notes:** Constrains all QM operations; PR boxes saturate but exceed QM correlations.

### wigner-araki-yanase-theorem (cross-domain alias: `WAY-theorem`, `conservation-constrains-measurement`, `non-projective-implementability`)
**Domain:** Quantum Computing
**Definition:** Sharp measurement of observable not commuting with a globally conserved quantity is impossible without external resources.
**Atom or composite:** Atom — measurement constraint theorem.
**Cost model:** Resource overhead to achieve approximate measurement.
**Real wall?** Yes — fundamental constraint on PVMs in presence of conservation laws.
**Cross-domain wiring:** physics: conservation laws. quantum-error-correction: measurement constraints. signal-processing-rf: ideal-detection limits.
**Notes:** Wigner (1952), Araki-Yanase (1960). Relevant for fault-tolerant gates under conservation laws.

### chsh-inequality (cross-domain alias: `Clauser-Horne-Shimony-Holt`, `Bell-CHSH`, `S≤2-classical-bound`)
**Domain:** Quantum Computing
**Definition:** For local hidden-variable model: |E(A,B)+E(A,B')+E(A',B)−E(A',B')| ≤ 2. Quantum violates: S=2√2 (Tsirelson).
**Atom or composite:** Atom — inequality bounding LHV correlations.
**Cost model:** Statistical estimation of correlators.
**Real wall?** Loophole-free violation experimentally established (2015 onward).
**Cross-domain wiring:** statistics-probability: correlation bounds. physics-foundations: non-locality test. cryptography-advanced: DI-QKD.
**Notes:** Clauser-Horne-Shimony-Holt (1969). Cleanest Bell-type test.

### ghz-inequality (cross-domain alias: `Greenberger-Horne-Zeilinger`, `3-particle-Bell`, `deterministic-non-locality`)
**Domain:** Quantum Computing
**Definition:** Tripartite |GHZ⟩=(|000⟩+|111⟩)/√2 produces measurement outcomes inconsistent with LHV for single-shot, not just statistical.
**Atom or composite:** Atom — deterministic non-locality test.
**Cost model:** Need 3-photon coincidence.
**Real wall?** Strongest form of non-locality (single-event disproof).
**Cross-domain wiring:** physics-foundations: non-locality without statistics. group-theory: stabilizer subgroup (X⊗X⊗X, etc.). cryptography-advanced: secret sharing.
**Notes:** Greenberger-Horne-Zeilinger (1989). Sharper than CHSH.

### tsirelson-bound (cross-domain alias: `Cirelson-bound`, `quantum-CHSH-limit`, `2√2-bound`)
**Domain:** Quantum Computing
**Definition:** Maximum CHSH value for quantum systems is 2√2 ≈ 2.828, far below algebraic max 4 (achievable by PR boxes).
**Atom or composite:** Atom — quantum mechanical bound.
**Cost model:** N/A.
**Real wall?** Yes — saturated by maximally entangled qubit pair.
**Cross-domain wiring:** statistics-probability: quantum correlation bounds. linear-algebra-matrix: spectral bound on commutators. cryptography-advanced: DI security relies on this gap.
**Notes:** Tsirelson (1980). Many information-theoretic derivations (information causality, etc.).

### pseudo-telepathy (cross-domain alias: `Brassard-Broadbent-Tapp`, `quantum-coordination-games`, `winning-without-communication`)
**Domain:** Quantum Computing
**Definition:** Two players sharing entanglement win coordination games with certainty, where classical players cannot win with probability 1.
**Atom or composite:** Composite — entangled strategy.
**Cost model:** Bell pair shared once.
**Real wall?** No-signaling preserved.
**Cross-domain wiring:** classical-complexity: communication-complexity separation. game-theory: cooperative games. cryptography-advanced: device verification.
**Notes:** Brassard-Broadbent-Tapp (2005). Mermin's GHZ game; Magic Square game.

### kochen-specker-theorem (cross-domain alias: `KS-theorem`, `contextuality`, `Kochen-Specker-no-go`)
**Domain:** Quantum Computing
**Definition:** No assignment of values to all quantum observables consistent with their algebraic relations. Reveals contextuality of QM.
**Atom or composite:** Atom — no-go theorem.
**Cost model:** N/A.
**Real wall?** Yes — value-assignment is impossible above dim 3.
**Cross-domain wiring:** physics-foundations: contextuality. classical-complexity: contextuality-cost. statistics-probability: non-Boolean event structures.
**Notes:** Kochen-Specker (1967). Foundation of quantum contextuality.

### contextuality (cross-domain alias: `quantum-contextuality`, `Spekkens-contextuality`, `non-classical-correlations`)
**Domain:** Quantum Computing
**Definition:** Measurement outcomes depend on which compatible observables are measured together. Witnessable by Bell-like inequalities.
**Atom or composite:** Atom — general non-classicality property.
**Cost model:** Statistical witnessing.
**Real wall?** Resource theory under "non-contextual operations".
**Cross-domain wiring:** classical-complexity: source of magic resource. quantum-error-correction: stabilizer ops are non-contextual. ml-training: hidden-variable-like models excluded.
**Notes:** Howard et al. (2014): contextuality = magic resource for quantum advantage.

### pbr-theorem (cross-domain alias: `Pusey-Barrett-Rudolph`, `ψ-ontic-theorem`, `wavefunction-reality-theorem`)
**Domain:** Quantum Computing
**Definition:** Under product-state preparation assumption, hidden-variable models with ψ-epistemic interpretation are inconsistent with QM predictions.
**Atom or composite:** Atom — no-go theorem.
**Cost model:** N/A.
**Real wall?** Conceptual — wave function is ontologically real (under PIP).
**Cross-domain wiring:** physics-foundations: realist interpretation of QM. statistics-probability: hidden-variable models. ml-training: relevance to latent-variable models.
**Notes:** Pusey-Barrett-Rudolph (2012). Strengthens against ψ-epistemic views.

---

## Quantum Metrology

### cramer-rao-bound-quantum (cross-domain alias: `quantum-CRB`, `Helstrom-bound`, `QFI-bound`)
**Domain:** Quantum Computing
**Definition:** Variance of any unbiased estimator θ̂ of θ from quantum measurements: Var(θ̂) ≥ 1/(N·F_Q), F_Q = quantum Fisher information.
**Atom or composite:** Atom — fundamental estimation bound.
**Cost model:** N independent measurements achieve bound asymptotically.
**Real wall?** Yes — optimal estimator achievable in limit.
**Cross-domain wiring:** statistics-probability: classical Cramér-Rao analogue. linear-algebra-matrix: symmetric logarithmic derivative. signal-processing-rf: estimation theory.
**Notes:** Helstrom (1969); Braunstein-Caves (1994). Foundation of quantum estimation.

### quantum-fisher-information (cross-domain alias: `QFI`, `F_Q`, `SLD-Fisher`)
**Domain:** Quantum Computing
**Definition:** F_Q(θ) = Tr(ρ_θ L_θ²) where L_θ is symmetric logarithmic derivative defined by ∂_θ ρ_θ = ½(L_θ ρ_θ + ρ_θ L_θ).
**Atom or composite:** Atom — information measure.
**Cost model:** Eigendecomposition of ρ_θ.
**Real wall?** No — defines optimal sensitivity.
**Cross-domain wiring:** statistics-probability: Fisher info quantum analogue. linear-algebra-matrix: SLD via Lyapunov equation. ml-training: natural gradient (FIM).
**Notes:** Helstrom (1976). Pure state: F_Q = 4(⟨∂_θψ|∂_θψ⟩ − |⟨ψ|∂_θψ⟩|²).

### heisenberg-limit (cross-domain alias: `1/N-scaling`, `quantum-enhanced-sensing`, `Caves-Heisenberg`)
**Domain:** Quantum Computing
**Definition:** Best precision Δθ = O(1/N) with N entangled probes (vs O(1/√N) SQL classical). Achieved with NOON states, GHZ states.
**Atom or composite:** Atom — fundamental sensitivity limit.
**Cost model:** Requires entanglement; decoheres N× faster.
**Real wall?** Decoherence collapses Heisenberg to SQL in noisy regimes.
**Cross-domain wiring:** signal-processing-rf: super-resolution metrology. statistics-probability: improved estimation. physics: NOON states, squeezed light.
**Notes:** Caves (1981); Giovannetti-Lloyd-Maccone (2004). Maximum quantum metrology advantage.

### noon-state (cross-domain alias: `|N0⟩+|0N⟩`, `path-entangled-state`, `interferometric-resource`)
**Domain:** Quantum Computing
**Definition:** Path-entangled state (|N,0⟩+|0,N⟩)/√2. Interferometric phase sensitivity scales as 1/N (Heisenberg).
**Atom or composite:** Composite — N-photon path entanglement.
**Cost model:** Hard to generate beyond N≈5.
**Real wall?** Generation and detection difficulty scale poorly.
**Cross-domain wiring:** signal-processing-rf: super-resolved interferometry. physics: Mach-Zehnder + N-photons. statistics-probability: minimum-variance phase estimation.
**Notes:** Boto et al. (2000). Standard demonstration of Heisenberg-limited sensing.

### nv-magnetometry (cross-domain alias: `NV-sensor`, `diamond-magnetometer`, `single-spin-magnetic-sensing`)
**Domain:** Quantum Computing
**Definition:** Measure magnetic field via Zeeman shift of NV center spin levels detected by ODMR or Ramsey interferometry.
**Atom or composite:** Composite — NV initialization + interference + readout.
**Cost model:** Sensitivity ~nT/√Hz with single NV; sub-pT/√Hz with ensembles.
**Real wall?** T2-limited at high frequencies; spatial resolution limited by NV depth.
**Cross-domain wiring:** physics: Zeeman effect. signal-processing-rf: lock-in detection. sensing: nanoscale MRI.
**Notes:** Maze et al. (2008). Premier nanoscale magnetic sensor.

### atomic-clock (cross-domain alias: `optical-lattice-clock`, `atomic-frequency-standard`, `Sr-clock`)
**Domain:** Quantum Computing
**Definition:** Time standard from atomic transition frequency. Optical lattice clocks (Sr, Yb) reach 10⁻¹⁸ fractional stability.
**Atom or composite:** Composite — laser-locked atomic transition.
**Cost model:** Many-second averaging for state-of-art stability.
**Real wall?** Black-body radiation shifts, lattice light shifts, gravitational frequency shift.
**Cross-domain wiring:** physics: atomic spectroscopy. signal-processing-rf: ultra-stable frequency reference. metrology: SI second redefinition candidate.
**Notes:** Used in geodesy, GPS, fundamental constant tests, search for dark matter.

### quantum-imaging (cross-domain alias: `ghost-imaging`, `quantum-illumination`, `entangled-photon-imaging`)
**Domain:** Quantum Computing
**Definition:** Image objects via entangled photon pairs — one interacts with object, other measured for image reconstruction. Beats SNR in lossy/noisy environments.
**Atom or composite:** Composite — entangled source + correlated detection.
**Cost model:** Photon-pair coincidence rate.
**Real wall?** Loss tolerance modest; needs bright source.
**Cross-domain wiring:** signal-processing-rf: lock-in imaging. physics: SPDC photon pairs. sensing: low-light biological imaging.
**Notes:** Pittman et al. (1995); Shapiro quantum illumination (2008).

### quantum-interferometry (cross-domain alias: `Mach-Zehnder-quantum`, `interferometric-sensing`, `phase-estimation-metrology`)
**Domain:** Quantum Computing
**Definition:** Phase encoded in path-split state; recombination yields interference pattern from which phase is estimated.
**Atom or composite:** Atom — interferometer + state.
**Cost model:** Depends on input state quality.
**Real wall?** Path-loss; photon noise; phase noise.
**Cross-domain wiring:** signal-processing-rf: Mach-Zehnder coherent receiver. physics: gravitational wave detection (LIGO). statistics-probability: phase estimation.
**Notes:** LIGO uses squeezed-light enhanced interferometry for GW detection.

### fisher-information-classical (cross-domain alias: `FIM`, `classical-Fisher`, `score-variance`)
**Domain:** Quantum Computing
**Definition:** F_C(θ) = E[(∂_θ log p(x|θ))²]. Sets classical Cramér-Rao bound; F_C ≤ F_Q for any measurement.
**Atom or composite:** Atom — classical statistics quantity.
**Cost model:** Estimated from data.
**Real wall?** Lower bound on measurement-conditional sensitivity.
**Cross-domain wiring:** statistics-probability: Fisher info. ml-training: natural gradient. quantum: classical limit of QFI.
**Notes:** Foundation of MLE asymptotic theory; classical analogue of QFI.

### parameter-estimation-multiparameter (cross-domain alias: `multi-parameter-CRB`, `Holevo-Cramér-Rao`, `quantum-statistical-model`)
**Domain:** Quantum Computing
**Definition:** For multi-parameter θ ∈ ℝⁿ: Cov(θ̂) ≥ F_Q⁻¹. Multiparameter QFI matrix F_Q,ij; bound not always attainable (operator non-commutativity).
**Atom or composite:** Composite — matrix Cramér-Rao bound.
**Cost model:** Non-trivial optimization over compatible measurements.
**Real wall?** Tight bound (Holevo) generally not single-copy attainable.
**Cross-domain wiring:** statistics-probability: vector CRB. linear-algebra-matrix: positive-semidefinite ordering. signal-processing-rf: multi-tone estimation.
**Notes:** Holevo (1976); Yang-Chiribella (2019). Active area of quantum metrology theory.

---

## Additional Quantum Primitives

### swap-test (cross-domain alias: `Buhrman-Cleve-Watrous-De-Wolf`, `overlap-estimator`, `state-comparison`)
**Domain:** Quantum Computing
**Definition:** Apply H to ancilla, controlled-SWAP between two state registers, H to ancilla, measure: P(0) = ½ + ½|⟨ψ|φ⟩|².
**Atom or composite:** Composite — H + CSWAP + H + measurement.
**Cost model:** O(N) CSWAPs for N-qubit states; O(1/ε²) shots for fidelity ε.
**Real wall?** No — but deep for large state.
**Cross-domain wiring:** statistics-probability: overlap estimation via sampling. signal-processing-rf: correlation-based detector. ml-training: kernel evaluation.
**Notes:** Buhrman, Cleve, Watrous, de Wolf (2001). Basic primitive for quantum verification, ML, fingerprinting.

### hadamard-test (cross-domain alias: `Re-Im-of-overlap`, `controlled-U-overlap`, `expectation-value-circuit`)
**Domain:** Quantum Computing
**Definition:** H on ancilla, controlled-U on |ψ⟩, H on ancilla, measure: P(0)−P(1) = Re⟨ψ|U|ψ⟩. Im-version uses S-gate.
**Atom or composite:** Composite — phase-estimation primitive.
**Cost model:** 1 controlled-U per shot; O(1/ε²) shots.
**Real wall?** Controlled-U often expensive; alternatives exist (direct measurement).
**Cross-domain wiring:** signal-processing-rf: coherent overlap extractor. statistics-probability: bias-controlled estimator. ml-training: expectation in PQC.
**Notes:** Aharonov-Jones-Landau (2009). Building block for QPE, VQE, QML.

### t-gate (cross-domain alias: `π/8-gate`, `T = diag(1,e^{iπ/4})`, `non-Clifford-essential`)
**Domain:** Quantum Computing
**Definition:** Single-qubit gate T = diag(1, e^{iπ/4}). Combined with Clifford = universal gate set.
**Atom or composite:** Atom — most-used non-Clifford gate.
**Cost model:** Fault-tolerant T extremely expensive (magic state distillation).
**Real wall?** Eastin-Knill: T cannot be transversal in any stabilizer code.
**Cross-domain wiring:** group-theory: third-level Clifford hierarchy. linear-algebra-matrix: 8th root of unity. quantum-error-correction: magic-state injection target.
**Notes:** Bottleneck of fault-tolerant computation; T-count drives resource estimates.

### toffoli-gate (cross-domain alias: `CCNOT`, `controlled-controlled-NOT`, `reversible-AND-gate`)
**Domain:** Quantum Computing
**Definition:** 3-qubit gate: flip target iff both controls = |1⟩. Universal for reversible classical computation; with H universal for QC.
**Atom or composite:** Composite — decomposable into 6 CNOTs + 7 T-gates (Clifford+T).
**Cost model:** 7 T-gates in standard decomposition; Toffoli measurement-based saves T.
**Real wall?** Reduced T-count active research (e.g., Gidney's Toffoli with 4 T).
**Cross-domain wiring:** classical-reversible: universal classical gate. linear-algebra-matrix: 8×8 permutation. cryptography-advanced: reversible computation core.
**Notes:** Toffoli (1980). Central gate in arithmetic circuits (modular exponentiation in Shor).

### fredkin-gate (cross-domain alias: `controlled-SWAP`, `CSWAP`, `reversible-mux`)
**Domain:** Quantum Computing
**Definition:** 3-qubit: if control=|1⟩, swap targets; else identity. Universal for reversible classical computation.
**Atom or composite:** Composite — decomposable into Toffolis or CZ + CNOTs.
**Cost model:** 7 T-gates in Clifford+T.
**Real wall?** Used heavily in SWAP-test and switching networks.
**Cross-domain wiring:** classical-reversible: data routing. linear-algebra-matrix: 8×8 permutation. ml-training: data-conditional swapping.
**Notes:** Fredkin-Toffoli (1982). Preserves Hamming weight (conserves "particle number").

### phase-kickback (cross-domain alias: `phase-on-control`, `inverse-control-trick`, `oracle-action-on-ancilla`)
**Domain:** Quantum Computing
**Definition:** Apply oracle U_f|x⟩|y⟩=|x⟩|y⊕f(x)⟩ with target in |−⟩ state: U_f|x⟩|−⟩ = (−1)^{f(x)}|x⟩|−⟩. Phase moves to control.
**Atom or composite:** Atom — circuit identity.
**Cost model:** No overhead — just choice of ancilla state.
**Real wall?** No.
**Cross-domain wiring:** signal-processing-rf: phase shifting via gating. linear-algebra-matrix: action on eigenbasis. cryptography-advanced: classical-to-phase oracle conversion.
**Notes:** Used in Deutsch-Jozsa, Bernstein-Vazirani, Grover, QPE — workhorse trick of quantum algorithms.

### uncomputation (cross-domain alias: `reverse-computation`, `Bennett-uncompute`, `garbage-clearance`)
**Domain:** Quantum Computing
**Definition:** Reverse computation of auxiliary qubits after their use to disentangle from main register. Crucial for amplitude amplification.
**Atom or composite:** Composite — explicit U† to restore ancilla.
**Cost model:** Roughly doubles gate count for compute-uncompute pattern.
**Real wall?** Required for coherent reuse of ancillas; failure causes decoherence.
**Cross-domain wiring:** classical-reversible: Bennett-style garbage collection. signal-processing-rf: matched-inverse filter. linear-algebra-matrix: U·U† = I.
**Notes:** Bennett (1973) reversible computation. Essential idiom in quantum algorithms.

### compute-uncompute (cross-domain alias: `Bennett-trick`, `reversibility-pattern`, `temp-register-clean`)
**Domain:** Quantum Computing
**Definition:** Pattern: compute f(x) into ancilla, use, then run circuit backwards to disentangle ancilla.
**Atom or composite:** Composite — design idiom.
**Cost model:** 2× compute cost.
**Real wall?** No.
**Cross-domain wiring:** classical-reversible: garbage management. compilers-traditional: register liveness. signal-processing-rf: matched filter pair.
**Notes:** Foundation of clean ancilla management in QC.

### controlled-rotation (cross-domain alias: `CRY-CRZ`, `conditional-rotation`, `controlled-phase`)
**Domain:** Quantum Computing
**Definition:** Rotation R(θ) on target only if control=|1⟩. Decomposable into CNOT + single-qubit rotations.
**Atom or composite:** Composite — 2 CNOTs + 3 single-qubit rotations.
**Cost model:** O(1) gates.
**Real wall?** No.
**Cross-domain wiring:** linear-algebra-matrix: block-diagonal embedding. signal-processing-rf: conditional phase shifter. ml-training: gated rotation in PQC.
**Notes:** Building block of QFT, HHL, QSP. Native on some hardware (cross-resonance).

### multi-controlled-gate (cross-domain alias: `Cⁿ-X`, `multi-controlled-NOT`, `Toffoli-generalization`)
**Domain:** Quantum Computing
**Definition:** n-control X gate; flip target iff all n controls = |1⟩. Generalizes Toffoli.
**Atom or composite:** Composite — O(n²) two-qubit gates without ancilla; O(n) with ancilla.
**Cost model:** Many decompositions; T-count critical metric.
**Real wall?** Deep without ancilla; "v-chain" + "log-depth" trade-offs.
**Cross-domain wiring:** classical-reversible: AND-tree. signal-processing-rf: conditional gating. linear-algebra-matrix: rank-1 reflection.
**Notes:** Barenco et al. (1995). Foundation for Grover diffusion operator, multi-control oracles.

### qubit-reset (cross-domain alias: `mid-circuit-reset`, `unconditional-init`, `dynamic-circuits`)
**Domain:** Quantum Computing
**Definition:** Measure qubit, then conditionally apply X if outcome=1, projecting to |0⟩. Or use spontaneous emission to |0⟩.
**Atom or composite:** Composite — measurement + classical control.
**Cost model:** Time-dominated by measurement (~µs on superconducting).
**Real wall?** Inevitable measurement back-action; reset infidelity ~10⁻²-10⁻³.
**Cross-domain wiring:** signal-processing-rf: feedback control. quantum-error-correction: enables continuous syndrome extraction. statistics-probability: conditional outcome.
**Notes:** Enables mid-circuit reuse of qubits; fundamental for dynamic circuits, holographic algorithms.

### bell-state-measurement (cross-domain alias: `BSM`, `Bell-basis-projection`, `Bell-measurement-primitive`)
**Domain:** Quantum Computing
**Definition:** Projective measurement in Bell basis {|Φ±⟩, |Ψ±⟩}. Implemented via CNOT + H + Z-measurement.
**Atom or composite:** Composite — CNOT + H + 2 measurements.
**Cost model:** O(1) gates + 2 measurements.
**Real wall?** Linear-optical BSM only 50% efficient with passive elements.
**Cross-domain wiring:** signal-processing-rf: 4-state demodulation. linear-algebra-matrix: maximally entangled basis. cryptography-advanced: teleportation, entanglement swapping, MDI-QKD.
**Notes:** Basic ingredient in many quantum protocols; deterministic on circuit-model QC, probabilistic photonic.

### symmetry-protected-topological-state (cross-domain alias: `SPT-state`, `Haldane-phase`, `protected-edge-modes`)
**Domain:** Quantum Computing
**Definition:** Gapped state with topological order protected by global symmetry. Edge modes are robust under symmetric perturbations.
**Atom or composite:** Atom — class of quantum states.
**Cost model:** Realized by AKLT-like Hamiltonians; preparation via adiabatic + measurement.
**Real wall?** No — but bulk gap closure under symmetry-breaking can destroy.
**Cross-domain wiring:** physics-condensed-matter: SPT phases (Chen-Gu-Wen). group-theory: cohomology classification. quantum-error-correction: SPT-based codes.
**Notes:** Pollmann-Turner (2012). Foundational for MBQC universality on lattice.

### mera-network (cross-domain alias: `multi-scale-entanglement-renormalization-ansatz`, `Vidal-MERA`, `hierarchical-tensor-network`)
**Domain:** Quantum Computing
**Definition:** Tensor network with hierarchical structure: disentanglers + isometries at each scale. Efficient ansatz for critical systems.
**Atom or composite:** Composite — multi-layer tensor network.
**Cost model:** Polynomial contraction (vs exponential PEPS).
**Real wall?** Limited expressivity vs general tensor networks.
**Cross-domain wiring:** ml-training: hierarchical autoencoder. signal-processing-rf: wavelet decomposition. linear-algebra-matrix: rank-truncated tree network.
**Notes:** Vidal (2007); inspires QCNN architecture; AdS/CFT toy model.

### mps-state (cross-domain alias: `matrix-product-state`, `1D-tensor-network`, `Vidal-canonical-form`)
**Domain:** Quantum Computing
**Definition:** |ψ⟩ = Σ Tr(A^{x_1}A^{x_2}...A^{x_n})|x⟩. Efficient representation for low-entanglement 1D states. Bond dimension χ = Schmidt rank.
**Atom or composite:** Composite — product of bond matrices.
**Cost model:** Polynomial in χ; classical exactly when χ=O(poly).
**Real wall?** Cannot capture volume-law entangled states.
**Cross-domain wiring:** linear-algebra-matrix: tensor train decomposition. signal-processing-rf: HMM analogue. ml-training: tensor train ML model.
**Notes:** White (1992) DMRG; Vidal canonical form (2003). Underlies modern many-body simulation.

### peps-network (cross-domain alias: `projected-entangled-pair-states`, `2D-tensor-network`, `Verstraete-Cirac`)
**Domain:** Quantum Computing
**Definition:** 2D generalization of MPS: tensors with virtual bond on each lattice edge. Represents 2D area-law states.
**Atom or composite:** Composite — 2D tensor network.
**Cost model:** Contraction exponentially hard; approximate via boundary MPS.
**Real wall?** Exact contraction #P-hard.
**Cross-domain wiring:** physics-condensed-matter: 2D ground states. linear-algebra-matrix: tensor contraction. ml-training: graph neural networks analogue.
**Notes:** Verstraete-Cirac (2004). Variational ansatz for 2D condensed matter.

### entanglement-witness (cross-domain alias: `EW`, `Horodecki-witness`, `linear-separability-test`)
**Domain:** Quantum Computing
**Definition:** Hermitian W such that Tr(W·σ) ≥ 0 ∀ separable σ but Tr(W·ρ) < 0 for some entangled ρ. Detects entanglement linearly.
**Atom or composite:** Atom — observable detecting entanglement.
**Cost model:** Measurement of W in Pauli basis.
**Real wall?** Cannot detect ALL entangled states with finite witness set.
**Cross-domain wiring:** linear-algebra-matrix: hyperplane separating convex sets. statistics-probability: hypothesis test. ml-training: linear classifier.
**Notes:** Horodecki² (1996). Cheaper than full tomography; tailored for specific states.

### no-pure-state-disentangling (cross-domain alias: `Bennett-disentangling-no-go`, `unitary-cannot-purify-without-info`, `Plenio-Vedral`)
**Domain:** Quantum Computing
**Definition:** No universal unitary can disentangle an arbitrary unknown entangled pair into product state.
**Atom or composite:** Atom — no-go theorem.
**Cost model:** N/A.
**Real wall?** Yes.
**Cross-domain wiring:** information-theory-coding: entanglement irreversibility. linear-algebra-matrix: Schmidt decomp uniqueness. cryptography-advanced: entanglement-based primitives.
**Notes:** Bennett et al. (1996). Disentangling requires measurement.

### swap-network (cross-domain alias: `linear-SWAP-network`, `Kivlichan-Kim-Kim-Eskandari`, `nearest-neighbor-routing`)
**Domain:** Quantum Computing
**Definition:** Layered SWAPs to bring all pairs adjacent on linear connectivity in O(N) depth. Used in fermionic simulation and QAOA on 1D hardware.
**Atom or composite:** Composite — O(N²) SWAPs in O(N) depth.
**Cost model:** Depth N for all pairs to meet.
**Real wall?** No — but SWAPs cost O(3 CNOTs) each.
**Cross-domain wiring:** classical-algorithms: bubble sort. graph-theory: routing. signal-processing-rf: time-multiplex.
**Notes:** Kivlichan et al. (2018). Standard routing primitive for limited-connectivity devices.

### givens-rotation-network (cross-domain alias: `Givens-network`, `orthogonal-basis-rotation`, `Hartree-Fock-circuit`)
**Domain:** Quantum Computing
**Definition:** Sequence of single-mode Givens rotations and CNOTs implementing arbitrary single-particle basis change for fermions.
**Atom or composite:** Composite — O(N²) Givens rotations.
**Cost model:** O(N²) two-qubit gates; depth O(N).
**Real wall?** No.
**Cross-domain wiring:** linear-algebra-matrix: QR decomposition by Givens. signal-processing-rf: orthogonal transform butterfly. chemistry: orbital rotations.
**Notes:** Wecker, Hastings, Troyer (2015); Kivlichan et al. (2018). Used in HF state prep, Slater rotations.

### qrom (cross-domain alias: `quantum-read-only-memory`, `Babbush-Berry-Sanders-Kivlichan`, `data-loading-oracle`)
**Domain:** Quantum Computing
**Definition:** Unitary O|i⟩|0⟩ = |i⟩|d_i⟩ that loads classical data into quantum register based on address.
**Atom or composite:** Composite — circuit implementing oracle.
**Cost model:** O(N) gates for N-entry QROM; O(N) depth or with ancilla log-depth.
**Real wall?** Loading large datasets coherently is non-trivial.
**Cross-domain wiring:** classical-memory: ROM analogue. linear-algebra-matrix: encoding diagonal matrices. ml-training: data-loading bottleneck.
**Notes:** Babbush et al. (2018). Major NISQ bottleneck for quantum ML, chemistry.

### qrac (cross-domain alias: `quantum-random-access-code`, `Ambainis-Nayak-Ta-Shma-Vazirani`, `compressed-encoding`)
**Domain:** Quantum Computing
**Definition:** Encode n classical bits in m < n qubits such that any single bit can be recovered with bias > 1/2.
**Atom or composite:** Composite — encoding + measurement strategy.
**Cost model:** Trade-off bias vs compression.
**Real wall?** Limits set by Holevo bound: n classical bits compress at best to n qubits.
**Cross-domain wiring:** information-theory-coding: lossy compression. statistics-probability: biased estimators. cryptography-advanced: random-access compression.
**Notes:** Ambainis et al. (1999). Surprising 2→1 QRAC inspired QKD analysis.

### quantum-pca (cross-domain alias: `qPCA`, `Lloyd-Mohseni-Rebentrost`, `density-matrix-exponentiation`)
**Domain:** Quantum Computing
**Definition:** Use multiple copies of ρ + SWAP gates to apply e^{−iρt} (density-matrix exponentiation); apply QPE to extract eigenvalues of ρ.
**Atom or composite:** Composite — sample-based Hamiltonian sim.
**Cost model:** O(t²/ε) copies of ρ.
**Real wall?** Quadratic in time t; dequantized by Tang (2018) for low-rank ρ.
**Cross-domain wiring:** linear-algebra-matrix: principal component analysis. ml-training: dim reduction. signal-processing-rf: spectral decomposition.
**Notes:** Lloyd-Mohseni-Rebentrost (2014). Subject of "dequantization" by classical sampling for low-rank inputs.

### holographic-quantum-circuits (cross-domain alias: `holographic-QC`, `mid-circuit-measurement-and-reset`, `Foss-Feig-holographic-VQE`)
**Domain:** Quantum Computing
**Definition:** Simulate large 1D system using few qubits via mid-circuit measurement and reuse, mirroring tensor-network contraction.
**Atom or composite:** Composite — circuit pattern using mid-circuit reset.
**Cost model:** O(bond dim²) qubits for χ-bond MPS.
**Real wall?** Mid-circuit reset fidelity is critical.
**Cross-domain wiring:** linear-algebra-matrix: MPS contraction. statistics-probability: state recycling. signal-processing-rf: time-multiplexed simulation.
**Notes:** Foss-Feig et al. (2021). Realized on Quantinuum H1; bridges tensor networks and QC.

### oracle-model (cross-domain alias: `query-complexity-model`, `black-box-quantum`, `Beals-Buhrman-oracle`)
**Domain:** Quantum Computing
**Definition:** Computational model where access to function is via reversible oracle U_f|x⟩|y⟩ = |x⟩|y⊕f(x)⟩. Used for query complexity bounds.
**Atom or composite:** Atom — computational model.
**Cost model:** Measured in oracle queries.
**Real wall?** Polynomial method, adversary method give matching lower bounds.
**Cross-domain wiring:** classical-complexity: query complexity. linear-algebra-matrix: oracle as block-diagonal matrix. cryptography-advanced: black-box separations.
**Notes:** Beals et al. (2001). Framework for proving quantum-classical separations.

### polynomial-method (cross-domain alias: `Beals-Buhrman-Cleve-Mosca-de-Wolf`, `lower-bound-via-polynomials`, `approximate-degree`)
**Domain:** Quantum Computing
**Definition:** Q-query algorithm computes Boolean f → ∃ degree-2Q polynomial approximating f. Lower bound from approximate degree.
**Atom or composite:** Atom — lower-bound technique.
**Cost model:** N/A — proof method.
**Real wall?** Polynomial method gives tight bounds for many problems.
**Cross-domain wiring:** classical-complexity: query lower bounds. linear-algebra-matrix: polynomial representations. signal-processing-rf: Chebyshev approximation theory.
**Notes:** Beals et al. (2001). Proves O(N^{2/3}) lower bound for element distinctness, matching Ambainis algorithm.

### adversary-method (cross-domain alias: `Ambainis-adversary`, `quantum-adversary-lower-bound`, `negative-weight-adversary`)
**Domain:** Quantum Computing
**Definition:** Lower bound via quantum-state distinguishability arguments: large adversary matrix Γ ⇒ many queries needed.
**Atom or composite:** Atom — lower-bound proof technique.
**Cost model:** N/A — proof method.
**Real wall?** Achieves tight bounds for all decision problems (Reichardt 2009).
**Cross-domain wiring:** statistics-probability: distinguishability. linear-algebra-matrix: spectral norm bounds. classical-complexity: matching upper bound by span programs.
**Notes:** Ambainis (2000); Høyer-Lee-Špalek (2007) negative weights; Reichardt (2009) tight.

### span-program-algorithm (cross-domain alias: `Reichardt-span-program`, `quantum-span-algorithm`, `dual-of-adversary`)
**Domain:** Quantum Computing
**Definition:** Computational model based on linear algebra: f computed by span program with complexity matching adversary lower bound.
**Atom or composite:** Composite — quantum algorithm from span program.
**Cost model:** Matches optimal query complexity for decision problems.
**Real wall?** No — generic upper-bound technique.
**Cross-domain wiring:** linear-algebra-matrix: subspace inclusion problem. classical-complexity: span programs (classical). ml-training: linear subspace methods.
**Notes:** Reichardt (2009). Proved span programs achieve adversary bound — tight characterization.

### bqp-vs-pp (cross-domain alias: `BQP-containment`, `quantum-class-relations`, `Adleman-DeMarrais-Huang`)
**Domain:** Quantum Computing
**Definition:** BQP ⊆ PP ⊆ PSPACE. Quantum poly-time inside probabilistic polynomial space; collapse implications open.
**Atom or composite:** Atom — complexity-class relation.
**Cost model:** N/A.
**Real wall?** PP closure under intersection unknown; BQP⊆AWPP (Fortnow-Rogers).
**Cross-domain wiring:** classical-complexity: counting hierarchy. statistics-probability: probabilistic algorithms with unbounded error. cryptography-advanced: complexity-based security.
**Notes:** Adleman-DeMarrais-Huang (1997). BQP not yet known equal to BPP nor strictly contained.

### qma-class (cross-domain alias: `quantum-Merlin-Arthur`, `Kitaev-QMA`, `quantum-NP`)
**Domain:** Quantum Computing
**Definition:** Quantum analogue of NP: poly-time quantum verifier accepts true instances with prob ≥ 2/3 given quantum witness, rejects false with prob ≥ 2/3.
**Atom or composite:** Atom — quantum complexity class.
**Cost model:** Verifier is poly-time quantum circuit.
**Real wall?** QMA-complete: local Hamiltonian problem.
**Cross-domain wiring:** classical-complexity: NP analogue. physics-condensed-matter: ground state energy estimation. cryptography-advanced: quantum proofs.
**Notes:** Kitaev (1999). Local Hamiltonian is QMA-complete; basis for "quantum PCP" conjecture.

### local-hamiltonian-problem (cross-domain alias: `LH-problem`, `Kitaev-LH`, `QMA-complete-problem`)
**Domain:** Quantum Computing
**Definition:** Given k-local Hamiltonian H = ΣH_i, determine if ground-state energy ≤ a or ≥ b with b−a ≥ 1/poly. QMA-complete for k≥2.
**Atom or composite:** Atom — computational problem.
**Cost model:** Verification: prepare witness ψ, measure ⟨H⟩.
**Real wall?** QMA-complete; no efficient quantum algorithm expected.
**Cross-domain wiring:** physics-condensed-matter: ground state finding. classical-complexity: QMA. linear-algebra-matrix: smallest eigenvalue.
**Notes:** Kitaev (1999); Kempe-Kitaev-Regev (2004): 2-local QMA-complete.

### qma-completeness-via-feynman-kitaev (cross-domain alias: `Feynman-Kitaev-clock`, `history-state-circuit-to-Hamiltonian`, `QMA-reduction`)
**Domain:** Quantum Computing
**Definition:** Encode quantum circuit history into ground state of local Hamiltonian via "clock register"; reduction from any QMA problem.
**Atom or composite:** Composite — reduction technique.
**Cost model:** Poly-time mapping.
**Real wall?** Foundation of QMA-completeness proofs.
**Cross-domain wiring:** physics-condensed-matter: encoding computation in ground states. classical-complexity: NP-completeness analogue. linear-algebra-matrix: low-energy subspace.
**Notes:** Kitaev (1999). Quantum Cook-Levin theorem.

### lieb-robinson-bound (cross-domain alias: `LR-bound`, `quantum-information-light-cone`, `Lieb-Robinson`)
**Domain:** Quantum Computing
**Definition:** Information propagation in local Hamiltonians has effective light cone: commutator ‖[A(t), B(0)]‖ ≤ c·exp(−(r−vt)/ξ).
**Atom or composite:** Atom — physical bound.
**Cost model:** Implies simulation cost bounded by light-cone size.
**Real wall?** Strict for finite-range Hamiltonians; modified for power-law.
**Cross-domain wiring:** physics-condensed-matter: emergent locality. signal-processing-rf: causality bound. linear-algebra-matrix: operator-spreading rate.
**Notes:** Lieb-Robinson (1972). Foundation of quantum simulation algorithms; bounds entanglement spreading.

### quantum-pcp-conjecture (cross-domain alias: `qPCP`, `Aharonov-Arad-Vidick`, `gapped-LH-conjecture`)
**Domain:** Quantum Computing
**Definition:** QMA-hardness of local Hamiltonian preserved under constant-ratio energy approximation. Open since 2006.
**Atom or composite:** Atom — conjecture.
**Cost model:** N/A.
**Real wall?** Major open problem in quantum complexity.
**Cross-domain wiring:** classical-complexity: classical PCP theorem (Dinur). physics-condensed-matter: gapped phases. cryptography-advanced: zero-knowledge proofs.
**Notes:** Aharonov-Arad-Vidick (2013). Hardness of approximation analogue for quantum.

### blind-quantum-computing (cross-domain alias: `BQC`, `Broadbent-Fitzsimons-Kashefi`, `delegated-quantum-computation`)
**Domain:** Quantum Computing
**Definition:** Client with limited quantum capability delegates computation to server while hiding inputs/outputs/computation from server.
**Atom or composite:** Composite — MBQC + rotated qubits.
**Cost model:** Polynomial-overhead universal blind QC.
**Real wall?** Server must trust client measurement model.
**Cross-domain wiring:** cryptography-advanced: secure delegation. signal-processing-rf: encrypted computation. ml-training: encrypted inference.
**Notes:** Broadbent et al. (2009). Backbone of "quantum cloud" security.

### verifiable-quantum-computing (cross-domain alias: `Mahadev-verification`, `classical-verifier-quantum-prover`, `quantum-supremacy-verifiable`)
**Domain:** Quantum Computing
**Definition:** Classical client verifies quantum server's computation with poly-time classical post-processing, assuming computational hardness (LWE).
**Atom or composite:** Composite — cryptographic protocol.
**Cost model:** Poly-time + LWE assumption.
**Real wall?** No — but server-prover quantum demands remain steep.
**Cross-domain wiring:** cryptography-advanced: post-quantum hardness. classical-complexity: interactive proofs. ml-training: cryptographic ML verification.
**Notes:** Mahadev (2018). Major breakthrough — classical verification of arbitrary quantum computation.

### feynman-path-integral-quantum-simulation (cross-domain alias: `Feynman-PI-sim`, `path-integral-MC-on-QC`, `quantum-PI`)
**Domain:** Quantum Computing
**Definition:** Express evolution as sum over paths; use quantum computer for evaluating amplitudes in superposition.
**Atom or composite:** Composite — discretized path sum.
**Cost model:** Polynomial vs exponential classical path sums.
**Real wall?** Sign problem mitigated but not eliminated.
**Cross-domain wiring:** physics: Feynman path integral. statistics-probability: Monte Carlo on paths. signal-processing-rf: time-discretized propagator.
**Notes:** Foundational concept: Feynman's 1982 quantum simulation proposal.

### digital-quantum-simulation (cross-domain alias: `DQS`, `Trotter-based-sim`, `gate-model-Hamiltonian-simulation`)
**Domain:** Quantum Computing
**Definition:** Simulate continuous H-evolution via discrete gate sequences (Trotter, QSP, qDRIFT) on universal gate-model QC.
**Atom or composite:** Composite — high-level approach.
**Cost model:** Choice of method (Trotter/QSP/qDRIFT) determines scaling.
**Real wall?** No — universal in principle.
**Cross-domain wiring:** physics: continuous quantum dynamics. signal-processing-rf: discrete-time simulation. linear-algebra-matrix: matrix exponential approximation.
**Notes:** Lloyd (1996). Categorically distinct from analog simulation.

### analog-quantum-simulation (cross-domain alias: `AQS`, `Hamiltonian-engineering`, `quantum-emulator`)
**Domain:** Quantum Computing
**Definition:** Engineer physical Hamiltonian H_emulator that mimics target H_target. Continuous evolution under H_emulator simulates target system.
**Atom or composite:** Composite — Hamiltonian engineering.
**Cost model:** Restricted by available physical interactions.
**Real wall?** No error correction; limited universality.
**Cross-domain wiring:** physics-condensed-matter: cold-atom experiments. signal-processing-rf: continuous-time dynamics. classical-complexity: special-purpose vs universal.
**Notes:** Lewenstein et al. (2007); used for Hubbard, spin-glass, lattice gauge theory studies.

### variational-quantum-state-eigensolver (cross-domain alias: `VQSE`, `Cerezo-Sharma-Arrasmith-Coles`, `eigenstate-VQE`)
**Domain:** Quantum Computing
**Definition:** Learn eigenstates of unknown Hamiltonian from samples by minimizing variance ⟨H²⟩ − ⟨H⟩².
**Atom or composite:** Composite — variance-minimization VQE.
**Cost model:** Higher variance cost than VQE.
**Real wall?** Local minima; barren plateaus.
**Cross-domain wiring:** ml-training: variance loss. statistics-probability: second-moment estimator. linear-algebra-matrix: eigenvector extraction.
**Notes:** Cerezo et al. (2020). Eigenstate finding without ground-state restriction.

### quantum-imaginary-time-evolution (cross-domain alias: `QITE`, `McArdle-Yan-Endo-Aspuru-Guzik`, `quantum-Wick-rotation`)
**Domain:** Quantum Computing
**Definition:** Approximate non-unitary e^{−βH}|ψ⟩/‖.‖ via unitary updates determined by McLachlan variational principle. Cools to ground state.
**Atom or composite:** Composite — variational non-unitary evolution.
**Cost model:** Polynomial in entanglement of evolved state.
**Real wall?** Variational ansatz expressivity.
**Cross-domain wiring:** physics: imaginary-time MC. ml-training: gradient flow analogue. linear-algebra-matrix: power iteration variant.
**Notes:** McArdle et al. (2019); Motta et al. (2020). Quantum ground-state preparation alternative to VQE.

### symmetric-purification (cross-domain alias: `symmetry-respecting-purification`, `Cotler-Wilczek`, `symmetric-Stinespring`)
**Domain:** Quantum Computing
**Definition:** Choose purification of mixed state respecting global symmetry of system. Useful for resource theories under symmetry.
**Atom or composite:** Composite — symmetry-aware Stinespring.
**Cost model:** Ancilla dimension bounded by symmetry sector dimensions.
**Real wall?** No.
**Cross-domain wiring:** group-theory: symmetric representations. quantum-error-correction: covariant codes. physics: charge sectors.
**Notes:** Foundation of symmetry-resource theories; covariant gate sets.

### quantum-channel-discrimination (cross-domain alias: `channel-discrimination`, `Acín-channel-discrimination`, `optimal-input-state`)
**Domain:** Quantum Computing
**Definition:** Distinguish two channels ℰ₀ vs ℰ₁ using optimal input + measurement. Entangled inputs can help (Pirandola channel discrimination).
**Atom or composite:** Composite — input + channel + measurement protocol.
**Cost model:** Single-copy error bounded by trace distance of Choi states.
**Real wall?** Some channels require entangled probe for optimal discrimination.
**Cross-domain wiring:** statistics-probability: hypothesis testing. signal-processing-rf: channel sensing. cryptography-advanced: side-channel detection.
**Notes:** Acín (2001); Pirandola et al. extensions. Foundation of quantum illumination.

### quantum-illumination (cross-domain alias: `Lloyd-quantum-illumination`, `entanglement-enhanced-target-detection`, `noisy-radar-quantum`)
**Domain:** Quantum Computing
**Definition:** Detect weakly-reflecting target in noisy thermal background using entangled probe (signal+idler); 6 dB advantage in error exponent.
**Atom or composite:** Composite — entangled probe + joint measurement.
**Cost model:** Many photon pairs; advantage robust to entanglement-breaking noise.
**Real wall?** Receiver design — collective measurement on returned signal + idler.
**Cross-domain wiring:** signal-processing-rf: matched-filter radar. statistics-probability: signal detection theory. cryptography-advanced: low-probability-of-intercept comms.
**Notes:** Lloyd (2008); Tan et al. (2008). Surprising — entanglement advantage even when destroyed.

### secure-multi-party-quantum-computation (cross-domain alias: `quantum-MPC`, `Crépeau-Gottesman-Smith`, `quantum-SMC`)
**Domain:** Quantum Computing
**Definition:** Multiple parties compute joint function on private inputs using quantum communication; security against active corruption.
**Atom or composite:** Composite — quantum verifiable secret sharing + secure circuits.
**Cost model:** Polynomial overhead; t<n/2 corruption threshold.
**Real wall?** Active corruption requires verifiable quantum secret sharing.
**Cross-domain wiring:** cryptography-advanced: classical MPC analogue. quantum-error-correction: VQSS = QECC. information-theory-coding: secret sharing.
**Notes:** Crépeau, Gottesman, Smith (2002). Quantum extension of secure multi-party computation.

### quantum-secret-sharing (cross-domain alias: `QSS`, `Hillery-Buzek-Berthiaume`, `Cleve-Gottesman-Lo`)
**Domain:** Quantum Computing
**Definition:** Encode secret quantum state across n parties such that any k can reconstruct, fewer than k learn nothing. (k,n) threshold scheme.
**Atom or composite:** Composite — quantum-error-correcting code (CSS).
**Cost model:** Encoded state size n; no-cloning forces k > n/2.
**Real wall?** Threshold k constrained by no-cloning to k > n/2.
**Cross-domain wiring:** information-theory-coding: secret sharing analogue. cryptography-advanced: distributed-trust quantum. quantum-error-correction: stabilizer codes provide schemes.
**Notes:** Hillery-Buzek-Berthiaume (1999); Cleve-Gottesman-Lo (1999). QSS = special QECC.

### one-clean-qubit-model (cross-domain alias: `DQC1`, `Knill-Laflamme-DQC1`, `mixed-state-quantum-computing`)
**Domain:** Quantum Computing
**Definition:** Computational model: one pure qubit, rest maximally mixed. Restricted but provably gives quantum advantage (estimating normalized trace).
**Atom or composite:** Atom — restricted computational model.
**Cost model:** Polynomial-time DQC1; not believed BQP-complete.
**Real wall?** Power between BPP and BQP — partial quantum advantage with minimal coherence.
**Cross-domain wiring:** classical-complexity: structured computational power. linear-algebra-matrix: trace estimation. statistics-probability: randomized algorithms.
**Notes:** Knill-Laflamme (1998). Solves Jones polynomial, knot invariants exponentially faster than known classical.

### no-hiding-theorem (cross-domain alias: `Braunstein-Pati`, `quantum-information-conservation`, `unitary-information-preservation`)
**Domain:** Quantum Computing
**Definition:** Information apparently lost in a quantum process is always recoverable from environment via some unitary. Information cannot disappear.
**Atom or composite:** Atom — conservation theorem.
**Cost model:** N/A.
**Real wall?** Yes — fundamental conservation.
**Cross-domain wiring:** physics: information conservation under unitary evolution. cryptography-advanced: black-hole information paradox connection. linear-algebra-matrix: unitarity preserves info.
**Notes:** Braunstein-Pati (2007). Resolves apparent information loss in randomization channels.

---

*Catalog extended: 2026-06-21 (≈300 additional primitives spanning algorithms, walks, simulation, linear algebra, info theory, channels, measurement, tomography, chemistry, variational, QML, advanced QEC, decoders, mitigation, CV, MBQC, annealing, topological, supremacy, hardware, compilers, communication, cryptography, foundations, metrology).*
*Source doctrine: The Painted Fence — Jesse*






