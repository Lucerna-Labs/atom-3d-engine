# Cryptography — Advanced Primitives Domain
> Cross-domain wiring: ZKP = fold + compare + project; MPC = split-combine over shares;
> FHE = homomorphic-fold over encrypted data; commitment = hash + bind; functional encryption = selective-project

## 1. Zero-Knowledge Proofs

### [PRIM-001] commitment-scheme
- **Atom/Composite:** Composite
- **Definition:** A commitment scheme allows a committer to lock a value (binding) without revealing it (hiding), then open it later (verifiable). Two phases: commit(value) → (commitment, decommit), verify(commitment, value, decommit) → accept/reject.
- **Cost Model:** Commitment size and verification cost vary by scheme; cryptographic security depends on hardness assumptions.
- **Real Wall:** Binding property (cannot change committed value) vs. hiding property (cannot see committed value before opening) — some schemes trade one for the other.
- **Cross-Domain Aliases:** hash-lock (cryptography-hashing), sealed-bid (distributed-systems), accumulator (database-streaming).
- **Notes:** Pedersen commitment (elliptic curve) is homomorphic; hash-based commitment (Merkle tree) is post-quantum secure.

### [PRIM-002] pedersen-commitment
- **Atom/Composite:** Composite
- **Definition:** Pedersen commitment: commit to m as C = g^m · h^r, where g, h are generators of a group of unknown order, and r is randomness. Perfectly hiding (given m, g^r is uniform) and computationally binding under the discrete log assumption.
- **Cost Model:** One or two exponentiations per commit; verification requires one exponentiation; size: one group element.
- **Real Wall:** Requires trusted setup to generate g, h such that discrete log of h wrt g is unknown; no known trapdoor.
- **Cross-Domain Aliases:** homomorphic-commitment (cryptography-hashing), elliptic-curve-bind (linear-algebra-matrix).
- **Notes:** Additive homomorphic: C₁ · C₂ commits to m₁ + m₂ (mod p); ElGamal commitment is similar.

### [PRIM-003] merkle-commitment
- **Atom/Composite:** Composite
- **Definition:** Merkle commitment: hash data into a binary tree, commit to root hash; opening = path of sibling hashes to root. Post-quantum secure (only requires collision-resistant hash).
- **Cost Model:** Commitment = hash(root) = O(1); proof size = O(log n) hashes for n leaves; verification = O(log n).
- **Real Wall:** Tree construction O(n) for n data items; bandwidth of proofs; not additive-homomorphic.
- **Cross-Domain Aliases:** hash-tree (cryptography-hashing), accumulator (database-streaming), merkle-proof (networking).

### [PRIM-004] polynomial-commitment
- **Atom/Composite:** Composite
- **Definition:** Commitment to a polynomial f(x) = Σ aᵢxⁱ such that later the committer can prove statements about f at arbitrary points without revealing the full polynomial. KZG (Kate-Zaverucha-Goldberg) uses trusted setup + pairing.
- **Cost Model:** KZG: one pairing-based exponentiation per commitment; proof of f(ζ) = y = single group element; batch proof for multiple points via polynomial division.
- **Real Wall:** KZG requires a trusted setup ceremony (toxic waste must be destroyed); DARK (Diophantine) uses no trusted setup; IPA (inner product argument) is post-quantum.
- **Cross-Domain Aliases:** polynomial-binding (linear-algebra-matrix), kzg-commitment (information-theory-coding).
- **Notes:** Core of Succinct ARGS (Groth16, PLONK, Marlin, STARKs); opening = evaluate at point.

### [PRIM-005] sigma-protocol
- **Atom/Composite:** Composite
- **Definition:** Three-move proof: Prover sends a (commitment to randomness), Verifier sends a (challenge), Prover sends z (response). Special honest verifier zero-knowledge (SHVZK) when verifier is honest.
- **Cost Model:** Three rounds; communication: |a| + |c| + |z|; soundness error = 1/|challenge_space| per round; parallel repetition reduces error.
- **Real Wall:** Challenge space must be large enough (≥ 2^128 for computational security); many protocols require Fiat-Shamir to remove interaction (non-interactive).
- **Cross-Domain Aliases:** interactive-proof (information-theory-coding), commit-challenge-response (cryptography-hashing).
- **Notes:** Schnorr identification protocol is a sigma protocol; classic building block for many ZK systems.

### [PRIM-006] fiat-shamir-heuristic
- **Atom/Composite:** Primitive
- **Definition:** Transform an interactive sigma protocol into a non-interactive proof: replace verifier's random challenge c with H(a || transcript), where H is a cryptographic hash. Prover generates c = H(a) then z without verifier interaction.
- **Cost Model:** One hash evaluation replaces verifier challenge; deterministic proof generation; proof is unique (not randomizable).
- **Real Wall:** Random oracle model required; Fiat-Shamir can introduce insecurity if protocol has subtle issues (e.g.,吃完 =吃完 attack); not all protocols safely transform.
- **Cross-Domain Aliases:** hash-as-random (cryptography-hashing), non-interactive-transform (information-theory-coding).
- **Notes:** Standard trick to make ZK protocols non-interactive; used in Schnorr signatures, zk-SNARKs (except STARKs).

### [PRIM-007] groth16-snark
- **Atom/Composite:** Composite
- **Definition:** Groth16: pairing-based SNARK with proof = (A, B, C) ∈ G₁ × G₂ × G₁; 3 pairings to verify; proof size = 3 elements (very short). Circuit-specific trusted setup (P = setup + preprocessing).
- **Cost Model:** Prover: O(N) exponentiations where N = circuit gates; Verification: 3 pairings; proof size: 3 elements (≈ 128 bytes at ~256-bit security).
- **Real Wall:** Trusted setup is circuit-specific (toxic waste → must destroy); per-circuit ceremony; proofs not universal (cannot verify different circuits with same proving/verification keys).
- **Cross-Domain Aliases:** circuit-proof (information-theory-coding), pairing-based-argument (cryptography-hashing).
- **Notes:** Groth16 (2016) was the first practical SNARK; used in Zcash Sprout; proven secure in the generic group model.

### [PRIM-008] plonk-snark
- **Atom/Composite:** Composite
- **Definition:** PLONK (Permutations over Lagrange-bases for Oecumenical Noninteractive arguments of Knowledge): universal trusted setup (one ceremony for any circuit up to size N); proof = 3 group elements + one evaluation proof.
- **Cost Model:** Prover: O(N log N) via FFT-based interpolation; Verification: O(log N) opening checks; proof size: constant (~3 group elements + polynomial evaluations).
- **Real Wall:** Universal setup = one trusted ceremony, but setup size grows with max circuit size; constraint system: R1CS → Plonkish constraints (more flexible than R1CS).
- **Cross-Domain Aliases:** universal-proof (information-theory-coding), fft-based-prover (linear-algebra-matrix).
- **Notes:** turboPLONK/plonkup add custom gates; Son阱 = PLONK + lookup tables; Halo2 = PLONK-like recursive proof system.

### [PRIM-009] stark
- **Atom/Composite:** Composite
- **Definition:** STARK (Scalable Transparent Arguments of Knowledge): no trusted setup; uses only collision-resistant hash; post-quantum secure; proof size polylogarithmic; verification time polylogarithmic.
- **Cost Model:** Proof generation: O(N · log N) via FRI (Fast Reed-Solomon IOP); proof size: polylog(N) kilobytes; verification: O(N) hash computations (fast).
- **Real Wall:** Proof size and verification time still larger than SNARKs; STARK verifier is not succinct (sublinear but not constant); requires large fields (FRI over extended fields).
- **Cross-Domain Aliases:** transparent-proof (information-theory-coding), hash-based-argument (cryptography-hashing).
- **Notes:** E.STARKs = Ethereum STARKs (Stone); LPD (Low Prosecutorial Degree) STARKs for small fields; Fast Reed-Solomon IOP (FRI) is the core commitment scheme.

### [PRIM-010] bulletproof
- **Atom/Composite:** Composite
- **Definition:** Bulletproof: short zero-knowledge proof for arithmetic circuits (specifically range proofs); no trusted setup; proof size = O(log n) where n = bit length; proof = two group elements + inner product proof.
- **Cost Model:** Prover: O(n · log n) via inner product argument; verifier: O(log n); proof size ~log n · 32 bytes (≈ 672 bytes for 64-bit range).
- **Real Wall:** Not as succinct as SNARKs for complex statements; inner product argument dominates cost; aggregation of multiple range proofs reduces verification cost.
- **Cross-Domain Aliases:** range-proof (information-theory-coding), log-proof (linear-algebra-matrix).
- **Notes:** Used in Monero RingCT (compact commitments); inner product argument from Bootle et al. (2016).

### [PRIM-011] groks16-proof-system
- **Atom/Composite:** Composite
- **Definition:** Marlin: preprocessing SNARK with universal setup (same as PLONK circuit model); proof = 3 group elements + polynomial opening; universal + succinct; used in Ethereum.
- **Cost Model:** Prover: O(N log N); proof size: 3 elements + openings; verification: O(log N) via polynomial commitment scheme; R1CS constraint system.
- **Real Wall:** Trusted setup universality; circuit must be represented in R1CS; slightly larger proofs than Groth16.
- **Cross-Domain Aliases:** preprocessing-snark (information-theory-coding), r1cs-proof (linear-algebra-matrix).

### [PRIM-012] sonic-snark
- **Atom/Composite:** Composite
- **Definition:** Sonic: updatable universal SNARK (subsequent update ceremonies can add to trust without knowing original toxic waste); proof = one element; verification = O(1).
- **Cost Model:** Prover: O(N · log N); verification: O(1) (constant pairings); proof size: one group element (~32 bytes at 128-bit security).
- **Real Wall:** Proof generation slower than Groth16; updatable setup is novel but complex; not widely deployed.
- **Cross-Domain Aliases:** updatable-proof (information-theory-coding), constant-verification (cryptography-hashing).

### [PRIM-013] dark-commitment
- **Atom/Composite:** Composite
- **Definition:** DARK (Diophantine Arguments of Knowledge): polynomial commitment from the hardness of the subset-sum / LPN problem; no trusted setup; post-quantum; works over generic groups.
- **Cost Model:** Commitment uses lattice-based hardness; proof size: O(log D) where D = max degree; verification: O(log D) hash checks.
- **Real Wall:** Newer scheme (2022), less battle-tested; proof sizes larger than pairing-based schemes; not yet production-ready.
- **Cross-Domain Aliases:** lattice-commitment (cryptography-hashing), post-quantum-polynomial (information-theory-coding).

### [PRIM-014] polynomial-evaluation-proof
- **Atom/Composite:** Composite
- **Definition:** Proof that a committed polynomial f evaluates to y at point ζ without revealing f. Uses polynomial commitment scheme (KZG, IPA) + batched opening argument.
- **Cost Model:** Prover: evaluate f(ζ) + prove correctness; KZG opening = 1 group element; IPA opening = O(log n) group elements.
- **Real Wall:** Batch opening (multiple points) can be done via quotient polynomial: Q(x) = (f(x) - y) / (x - ζ); opening of Q is cheaper than separate openings.
- **Cross-Domain Aliases:** polynomial-open (linear-algebra-matrix), batch-evaluate (control-numerical-opt).
- **Notes:** Core of SNARK verifier circuits (verifier checks polynomial openings).

### [PRIM-015] knowledge-soundness
- **Atom/Composite:** Primitive
- **Definition:** Proof of knowledge: extractor algorithm that, given prover's code, can extract the witness from a valid proof. Stronger than just existence of a proof — proves the prover *knows* the witness.
- **Cost Model:** Extractor complexity varies by protocol; rewinding techniques (rewind to challenge point) used in sigma protocols; knowledge assumptions (e.g., q-SDH, SXDH) formalized as hardness of extraction.
- **Real Wall:** Some protocols are only witness-extendable (extraction requires specific prover behavior); knowledge errors.
- **Cross-Domain Aliases:** witness-extraction (information-theory-coding), prover-knowledge (agentic-reasoning).
- **Notes:** Key distinction: "soundness" (proof exists) vs. "knowledge soundness" (prover knows witness). Groth16/PLONK/Marlin are knowledge-sound under respective assumptions.

### [PRIM-016] crs-model
- **Atom/Composite:** Primitive
- **Definition:** Common Reference String model: all parties receive a shared random string CRS from a trusted setup; SNARKs based on pairing groups require CRS. Universal CRS vs. circuit-specific CRS.
- **Cost Model:** Setup ceremony: multi-party computation (MPC) where each participant contributes randomness; as long as one honest participant destroys their input, CRS is safe.
- **Real Wall:** Trusted setup must be trusted (if corrupted, adversary can forge proofs); powers-of-tau ceremonies (e.g., for Groth16, PLONK) scale to large circuits.
- **Cross-Domain Aliases:** shared-randomness (information-theory-coding), trusted-setup (distributed-systems).
- **Notes:** Transparent SNARKs (STARKs) avoid CRS entirely; universal SRS (Powers of Tau) = one ceremony supports many circuits up to size N.

### [PRIM-017] linear-interactive-proof
- **Atom/Composite:** Composite
- **Definition:** Linear PCP (LPCP): probabilistic proof where verifier's check is a linear combination of proof bits; combined with cryptographic commitment to create arguments. Linear PCP + Merkle tree = IOP (Interactive Oracle Proof).
- **Cost Model:** PCP oracle: proof length; linearity allows batch verification; SNARKs = IOP + CRS.
- **Real Wall:** PCP hardness: optimal PCP proof length is polynomial; SNARKs compress this via cryptographic hardness.
- **Cross-Domain Aliases:** probabilistic-proof-check (information-theory-coding), linear-combination-check (linear-algebra-matrix).

### [PRIM-018] succinct-verification
- **Atom/Composite:** Primitive
- **Definition:** Succinct verification: verifier runs in time sublinear in the circuit size (typically O(log N) or O(√N)). Key property of SNARKs; enables blockchain verifiers that run on-chain.
- **Cost Model:** Sublinear verification = complex proof system; proof size trade-off; universal setup enables preprocessing.
- **Real Wall:** Succinctness often at odds with post-quantum security or proof generation time; STARKs are transparent but not succinct (verifier = poly(N)).
- **Cross-Domain Aliases:** sublinear-verify (information-theory-coding), on-chain-verification (networking).

### [PRIM-019] recursive-proof-composition
- **Atom/Composite:** Composite
- **Definition:** Proof recursion (proof of a proof): verify a SNARK proof inside a SNARK circuit, enabling aggregation and incrementality. Halo2 and Nova use this for accumulation schemes (no trusted setup chain).
- **Cost Model:** Nested verification: outer SNARK circuit verifies inner proof; amortization via proof accumulation; Nova = folding scheme (no pairings needed).
- **Real Wall:** Recursive verification requires arithmetic circuit for curve operations (expensive); accumulation reduces cost by folding many proofs into one.
- **Cross-Domain Aliases:** proof-accumulation (information-theory-coding), nested-verification (control-numerical-opt).
- **Notes:** Nova (RISC-V ZK proof); Halo2 (Zcash Orchard); Pickles (Mina blockchain).

### [PRIM-020] zkvm-verification
- **Atom/Composite:** Composite
- **Definition:** zkVM: zero-knowledge execution of a virtual machine (RISC-V, MIPS, WASM); witness = state machine transitions; proof verifies execution trace correctness.
- **Cost Model:** Instruction set → constraints; RAM/registers → lookup tables or memory checking; proving time scales with trace length.
- **Real Wall:** RAM modeling (read/write ordering) is the hardest part; zkEVM vs. native ZK circuits (different trade-offs for Ethereum compatibility).
- **Cross-Domain Aliases:** execution-trace-proof (agentic-reasoning), vm-witness (control-numerical-opt).
- **Notes:** zkSync Era, StarkNet (Cairo), RISC Zero, Polygon zkEVM; each has different constraint system.

## 2. Secure Multi-Party Computation

### [PRIM-021] secret-sharing
- **Atom/Composite:** Composite
- **Definition:** Split a secret s into n shares such that any t shares (threshold) can reconstruct s, but fewer than t shares give no information (完美隐藏). Shamir's Secret Sharing: polynomial interpolation over finite field; additive: XOR shares.
- **Cost Model:** Share size = size of secret; Shamir: O(n) share generation, O(t) reconstruction (Lagrange interpolation); additive: O(n) share, XOR reconstruction.
- **Real Wall:** Shamir requires field arithmetic; honest majority assumption for security; additive is information-theoretic (no computational assumption) but requires all shares for reconstruction.
- **Cross-Domain Aliases:** split-share (cryptography-hashing), threshold-decomposition (distributed-systems).
- **Notes:** (t, n) threshold scheme: t of n needed; monotone SS (access structure) extends to arbitrary subsets.

### [PRIM-022] multiplicative-homomorphic-encryption
- **Atom/Composite:** Primitive
- **Definition:** Encryption scheme where product of ciphertexts = encryption of product of plaintexts: Enc(a) · Enc(b) = Enc(a × b). Paillier (RSA variant) has additive homomorphic property; RSA has multiplicative.
- **Cost Model:** Paillier: ciphertext expansion ≈ 2× plaintext; one exponentiation per multiplication; RSA-OAEP: deterministic encryption (not IND-CPA secure without padding).
- **Real Wall:** Homomorphic properties limited (cannot do both addition and multiplication efficiently without FHE); semantic security vs. homomorphism trade-off.
- **Cross-Domain Aliases:** encrypted-compute (cryptography-hashing), multiplicative-encrypt (information-theory-coding).

### [PRIM-023] fully-homomorphic-encryption
- **Atom/Composite:** Composite
- **Definition:** FHE: evaluate arbitrary circuits on encrypted data without decrypting. Computations: (Eval(f, Enc(x₁), ..., Enc(xₙ)) = Enc(f(x₁, ..., xₙ)). Requires bootstrapping (refreshing noisy ciphertexts) to handle unbounded circuit depth.
- **Cost Model:** Bootstrapping is expensive (full homomorphic evaluation of a circuit that includes its own decryption); leveled FHE avoids bootstrapping with depth-bounded circuits; GSW, BFV, CKKS, TFHE.
- **Real Wall:** Noise management (key-switching, bootstrapping); large ciphertext size; performance gap vs. plaintext computation ≈ 10⁴–10⁶×.
- **Cross-Domain Aliases:** encrypted-circuit-eval (cryptography-hashing), noise-management (control-numerical-opt).
- **Notes:** GSW (Gentry-Sahai-Waters) 2013: first bootstrappable FHE; CKKS for approximate arithmetic (machine learning); TFHE for Boolean gate evaluation.

### [PRIM-024] yaos-garbled-circuit
- **Atom/Composite:** Composite
- **Definition:** Yao's garbled circuit: transform any Boolean circuit into a garbled version; evaluator learns only output, not intermediate values; oblivious transfer (OT) lets receiver learn one of two inputs without learning the other.
- **Cost Model:** Garbling cost: 2× circuit size; oblivious transfer = several DH exchanges (expensive); point-and-permute optimization reduces evaluation cost.
- **Real Wall:** Free-XOR optimization (XOR gates = no communication); circuit size is bottleneck; AND gates cost 2 hash evaluations in modern garbling.
- **Cross-Domain Aliases:** oblivious-evaluation (cryptography-hashing), circuit-oblivious (distributed-systems).
- **Notes:** Semi-honest security (corrupt but follow protocol); malicious security requires cut-and-choose (expensive, 2-3× overhead).

### [PRIM-025] oblivious-random-access-memory
- **Atom/Composite:** Composite
- **Definition:** ORAM: client stores data on untrusted server but server learns nothing about access patterns. Client holds small local state; server stores encrypted data blocks; path ORAM (N/blocks per path, O(log N) access).
- **Cost Model:** Path ORAM: O(log N) server accesses per read/write; bandwidth amplification (N to O(N log N)); recursive ORAM reduces client memory.
- **Real Wall:** ORAM is expensive (× log N overhead vs. direct access); practical ORAMs (Silver, DIR, Ring) trade some privacy for performance; Oblivious RAM (Goldreich) = O(log² N).
- **Cross-Domain Aliases:** access-pattern-hiding (cryptography-hashing), oblivious-storage (distributed-systems).
- **Notes:** Circuit ORAM = constant bandwidth but large circuits; functional ORAM (FORAM) = allows computation without full data access.

### [PRIM-026] oblivious-transfer
- **Atom/Composite:** Primitive
- **Definition:** OT: sender has two messages m₀, m₁; receiver chooses a bit b; after OT, receiver learns only m_b, sender learns nothing about b. Correlated OT (COT) = sender's messages are correlated.
- **Cost Model:** 1-out-of-2 OT via DH key exchange (Naor-Pinkas); 1-out-of-N OT via OT extension (IKN, ALS); correlated OT extension = linear cost.
- **Real Wall:** OT extension allows cheap OT from few base OTs using hash functions; silent OT (using LPN) = near-optimal communication; malicious OT = expensive (cut-and-choose or dual execution).
- **Cross-Domain Aliases:** private-selection (cryptography-hashing), secret-choice (distributed-systems).
- **Notes:** OT is the main bottleneck in garbled circuits; cheap OT = cheap secure computation; OT extension (Ishai-Kilian-Nissim-Petrank) reduces base OTs from O(N) to O(k log N).

### [PRIM-027] gmw-protocol
- **Atom/Composite:** Composite
- **Definition:** Goldreich-Micali-Wigderson (GMW): secure multi-party computation for any Boolean circuit; parties hold secret shares of inputs; gates evaluated by local computation + communication; supports addition (local) and multiplication (communication).
- **Cost Model:** Addition: local (no communication); multiplication: O(1) rounds per multiplication gate; total = O(depth × communication_per_gate).
- **Real Wall:** Requires honest majority (fewer than t/2 corrupted); secret sharing over a field; Boolean circuits (vs. arithmetic over rings); communication cost dominates.
- **Cross-Domain Aliases:** mpc-circuit-evaluation (distributed-systems), share-computation (cryptography-hashing).
- **Notes:** GMW vs. Yao: GMW scales with circuit depth (rounds), Yao scales with circuit size (communication); GMW suits low-depth high-computation circuits.

### [PRIM-028] shamir-secret-sharing-mpc
- **Atom/Composite:** Composite
- **Definition:** Secret sharing as MPC substrate: each party holds a share of each secret; addition = local share addition; multiplication = cross-party communication ( Beaver triples). SPDZ, MASCOT, OVERdrive use this.
- **Cost Model:** Beaver triple generation (preprocessing) is expensive; online phase: addition = free, multiplication = 1 round of communication; security: malicious (cheating detection via MACs).
- **Real Wall:** Preprocessing phase (triple generation) must be secure; communication complexity for preprocessing; active security (against malicious parties) costs MAC verification.
- **Cross-Domain Aliases:** mpc-additive-share (distributed-systems), triple-generation (cryptography-hashing).
- **Notes:** SPDZ-2: MACs for active security; Threshold FHE: combine FHE with threshold decryption (no preprocessing needed).

### [PRIM-029] homomorphic-encryption-mpc-hybrid
- **Atom/Composite:** Composite
- **Definition:** Hybrid MPC: use FHE for public operations (known to all), MPC only for private operations. Client encrypts → server computes on encrypted data → client decrypts.
- **Cost Model:** Client-side encryption/decryption (fast); server-side FHE computation (expensive); minimizes server-side MPC overhead.
- **Real Wall:** Server learns nothing about inputs/outputs (semantic security); requires FHE scheme with appropriate homomorphisms; hybrid reduces but doesn't eliminate MPC need.
- **Cross-Domain Aliases:** encrypted-server-compute (networking), client-side-trust (distributed-systems).

### [PRIM-030] secret-sharing-verifiable
- **Atom/Composite:** Composite
- **Definition:** Verifiable Secret Sharing (VSS): Shamir secret sharing + polynomial commitment; each share is verified as consistent with the committed polynomial. Feldman VSS (based on Pedersen) or Pedersen VSS.
- **Cost Model:** Share = one field element + one group element (Feldman); verification = one pairing; VSS needed in DKG (Distributed Key Generation).
- **Real Wall:** In DKG, each party acts as dealer in VSS; if dealer is malicious, VSS ensures consistency but doesn't prevent invalid secret; committed Shamir requires discrete log hardness.
- **Cross-Domain Aliases:** verifiable-share (cryptography-hashing), dkg-share (distributed-systems).

### [PRIM-031] distributed-key-generation
- **Atom/Composite:** Composite
- **Definition:** DKG: generate a public key in a distributed way such that no single party knows the private key. Combines VSS from each party + consistency check. Pedersen DKG or Feldman DKG.
- **Cost Model:** O(n²) messages for n parties; each party runs VSS; consistency check eliminates malicious shares; communication complexity = O(n²).
- **Real Wall:** Dishonest majority: protocol becomes more complex ( Feldman-VSSS + consistency check insufficient); recent protocols (GJKR+) handle this.
- **Cross-Domain Aliases:** multi-party-keygen (distributed-systems), threshold-crypto (cryptography-hashing).
- **Notes:** DKG + threshold signature = threshold BLS, threshold ECDSA (used in blockchain multi-sig wallets).

## 3. Signature Primitives

### [PRIM-032] ring-signature
- **Atom/Composite:** Composite
- **Definition:** Ring signature: signer proves they are one of n possible signers without revealing which one. Unlinkable (verifier cannot tell which member signed). Linkable variant (LSAG) enables double-spending detection in Monero.
- **Cost Model:** Signature size = O(n) group elements for basic ring sigs (e.g., CryptoNote); linkable ring signatures (LSAG, CLSAG, Bulletproofs) improve efficiency.
- **Real Wall:** Ring size n trades off anonymity vs. signature size; large n = expensive verification; Monero's RingCT uses CLSAG (Compact LSAG + Bulletproofs).
- **Cross-Domain Aliases:** anonymity-signature (cryptography-hashing), group-anonymous-sign (information-theory-coding).
- **Notes:** Ronald Rivest's "How to leak a secret" (2001); used in privacy coins (Monero), whistleblowing, credential systems.

### [PRIM-033] group-signature
- **Atom/Composite:** Composite
- **Definition:** Group signature: any group member can sign on behalf of the group; only the group manager can identify the signer. Revocation: if a member is expelled, old signatures from that member must remain verifiable (traceable).
- **Cost Model:** Signature size = O(1) (constant); group manager overhead for joining/revoking; revocable group signatures require efficient zero-knowledge proofs.
- **Real Wall:** Revocation is complex (dynamic group signatures require CRL or cryptographic accumulator); pairing-based group signatures have constant-size signatures.
- **Cross-Domain Aliases:** issuer-anonymous-sign (cryptography-hashing), anonymous-authentication (distributed-systems).
- **Notes:** Camenisch-Stadler framework; used in privacy-preserving authentication, corporate whistleblowing, vehicular communications.

### [PRIM-034] blind-signature
- **Atom/Composite:** Composite
- **Definition:** Blind signature: signer computes a signature on a message without seeing the message. Two-step: (1) blinding: multiply message by random factor, (2) signing: sign blinded message, (3) unblinding: remove blind factor.
- **Cost Model:** RSA blind signature: Enc(m) · r^e = Enc(m·r) (mod N), sign → multiply by r^{-1} to get sign(m); communication: two rounds; cryptographic cost: one RSA exponentiation.
- **Real Wall:** Blind signatures can enable surveillance if misused (e-cash, anonymous credentials); Chaum's RSA-based scheme is not formally secure; formal security models (Fujisaki-Okamoto) exist.
- **Cross-Domain Aliases:** hidden-message-signature (cryptography-hashing), anonymous-authorization (information-theory-coding).
- **Notes:** Chaum (1982); e-cash (Chaum-Fiat-Naor); used in privacy-preserving digital cash, voting systems, credential issuance.

### [PRIM-035] threshold-signature
- **Atom/Composite:** Composite
- **Definition:** (t, n) threshold signature: any t of n signers can produce a valid signature; fewer than t parties learn nothing. Threshold BLS, Threshold ECDSA (Musig, GG20).
- **Cost Model:** Threshold signing: partial signatures from t participants → combine via Lagrange interpolation; key generation requires DKG; communication rounds = O(t).
- **Real Wall:** ECDSA threshold is complex (Paillier + homomorphic encryption needed for additive sharing); BLS threshold is simpler (linear secret sharing).
- **Cross-Domain Aliases:** distributed-signature (distributed-systems), multi-party-signature (cryptography-hashing).
- **Notes:** Used in blockchain multi-sig wallets (2-of-3), distributed validators (ETH 2.0 deposit contract), DNSSEC.

### [PRIM-036] aggregate-signature
- **Atom/Composite:** Composite
- **Definition:** Aggregate signatures: compress n signatures on n messages into one short signature. Sequential aggregation (ordered) or aggregate (unordered). BLS signatures support native aggregation.
- **Cost Model:** BLS aggregation: multiply n signatures → O(n) operations; verification: one pairing (instead of n); signature size = one group element (constant).
- **Real Wall:** Aggregate of signatures on different messages requires sorted ordering or random oracle; BGLS (Boneh-Gentry-Lynn-Shacham) aggregate is combinable but not strictly shorter.
- **Cross-Domain Aliases:** signature-compression (cryptography-hashing), proof-of-aggregation (information-theory-coding).

### [PRIM-037] chameleon-hash
- **Atom/Composite:** Composite
- **Definition:** Chameleon hash: collision-resistant hash with a trapdoor. Anyone can compute hash, but holder of trapdoor can find collisions efficiently. Used for designated-verifier proofs, sanitizable signatures.
- **Cost Model:** Trapdoor (private key): efficient collision finding; hash function: standard collision-resistant hash (e.g., RSA or elliptic curve); no computational cost to compute without trapdoor.
- **Real Wall:** If trapdoor is leaked, hash loses collision resistance; chameleon hash commitment = extractable commitment (not hiding).
- **Cross-Domain Aliases:** trapdoor-hash (cryptography-hashing), mutable-commitment (information-theory-coding).

## 4. Functional Encryption & Advanced

### [PRIM-038] functional-encryption
- **Atom/Composite:** Composite
- **Definition:** FE: decryption key allows computing a function f(x) on ciphertext x without revealing x. Private-key FE (PKE) or public-key FE. Example: compute Σ xᵢ without revealing individual xᵢ.
- **Cost Model:** Circuit FE (iO-based): complex; attribute-based encryption (ABE) is a special case where f is a circuit checking attributes; inner-product FE: compute x · y from encryption of x.
- **Real Wall:** Full FE from iO (indistinguishability obfuscation) is theoretical; ABE-based FE is practical; inner-product FE from pairing groups ( Bourse-Lefever).
- **Cross-Domain Aliases:** selective-decrypt (cryptography-hashing), function-private-encryption (information-theory-coding).
- **Notes:** FE generalizes: signatures (function = hash), encryption (function = identity), attribute-based access control (function = policy check).

### [PRIM-039] attribute-based-encryption
- **Atom/Composite:** Composite
- **Definition:** ABE: decryption key is associated with a policy (CP-ABE) or attributes (KP-ABE); ciphertext is labeled with attributes; user can decrypt if attributes satisfy policy.
- **Cost Model:** Key generation = O(|policy|) for CP-ABE; ciphertext size = O(|attributes|); decryption = O(|policy|) pairings.
- **Real Wall:** Key revocation is complex (revoking one attribute requires re-keying all users); traceable ABE adds accountability; large attribute universes are expensive.
- **Cross-Domain Aliases:** policy-encryption (cryptography-hashing), access-control-encryption (distributed-systems).

### [PRIM-040] predicate-encryption
- **Atom/Composite:** Composite
- **Definition:** Predicate encryption: stronger than ABE; hides both the policy AND the ciphertext attributes/function from unauthorized users. Supports private predicates (conjunction, range, polynomial).
- **Cost Model:** Predicate encryption from pairings (Katz-Sahai-Waters); ciphertext and key sizes depend on predicate complexity; predicates = circuit (general FE).
- **Real Wall:** Predicate encryption with hidden policies is more expensive than ABE; support for arbitrary predicates requires indistinguishability obfuscation (impractical).
- **Cross-Domain Aliases:** hidden-policy-encrypt (cryptography-hashing), fine-grained-access (distributed-systems).

### [PRIM-041] identity-based-encryption
- **Atom/Composite:** Composite
- **Definition:** IBE: encrypt to arbitrary identity (e.g., email address) without retrieving public key; PKG (Private Key Generator) issues private keys. Boneh-Franklin (pairing-based) or lattice-based (GGH).
- **Cost Model:** Setup (PKG): generate master public/secret key; Extract: key for identity ID = H(ID)^s; Encrypt: g^(H(ID)·r); Decrypt: apply key.
- **Real Wall:** Key escrow (PKG knows all private keys) — identity-based encryption requires trusted PKG; Private Key Generator is single point of failure.
- **Cross-Domain Aliases:** identity-crypt (cryptography-hashing), key-generator-trust (distributed-systems).
- **Notes:** Used in email encryption, secure DNS (Zone signing in DNSSEC), key distribution for IoT; Fuzzy IBE (Shamir) enables attribute-based features.

### [PRIM-042] lattice-based-cryptography
- **Atom/Composite:** Composite
- **Definition:** Cryptosystems based on lattice problems: SIVP (Shortest Independent Vectors Problem), LWE (Learning With Errors), SIS (Short Integer Solution). Post-quantum secure under quantum attacks.
- **Cost Model:** LWE encryption: matrix-vector multiplication over Z_q (fast); signature schemes (Dilithium) use NTRU lattices; key sizes larger than ECC (≈ 1–4 KB).
- **Real Wall:** Ring-LWE (RLWE) reduces key sizes by using polynomial rings;Kyber (KEM), Dilithium (signatures) are NIST PQC standard; Frodo uses raw LWE (larger but simpler).
- **Cross-Domain Aliases:** post-quantum-encrypt (cryptography-hashing), lattice-hard (linear-algebra-matrix).
- **Notes:** NIST PQC Standard (2024): CRYSTALS-Kyber (KEM), CRYSTALS-Dilithium (signatures), SPHINCS+ (hash-based signatures), Falcon (NTRU lattice signatures).

### [PRIM-043] code-based-cryptography
- **Atom/Composite:** Composite
- **Definition:** Cryptosystems based on hardness of decoding random linear codes (McEliece, Niederreiter). Classic McEliece uses Goppa codes; QC-MDPC for compact keys. Post-quantum secure.
- **Cost Model:** McEliece: large public key (≈ 1 MB for 128-bit security); encryption/decryption: linear coding operations (fast). QC-MDPC reduces key size but requires careful decoding.
- **Real Wall:** Key size is the main practical barrier; recent improvements (BIKE, HQC) target smaller keys; decoding attacks are the main security concern.
- **Cross-Domain Aliases:** decoding-hard (cryptography-hashing), code-based-security (information-theory-coding).

### [PRIM-044] hash-based-signature
- **Atom/Composite:** Composite
- **Definition:** Hash-based signatures: only require collision-resistant hash function. Lamport one-time signature (OTS): publish hash of secret key, sign by revealing preimage of hash of message. Merkle tree of OTS keys = many-time (MSS), + XMSS, SPHINCS+.
- **Cost Model:** Lamport: O(n) hash operations per sign/verify (n = output length × security parameter); Merkle: O(log N) tree traversal for MSS; stateful vs. stateless (SPHINCS+ is stateless, LMS is stateful).
- **Real Wall:** One-time signature (OTS) keys can only sign once; MSS requires state management (cannot reuse leaf); SPHINCS+ is large (~40 KB signatures) but simple and well-understood.
- **Cross-Domain Aliases:** hash-only-signature (cryptography-hashing), merkle-signature (information-theory-coding).
- **Notes:** SPHINCS+ is NIST PQC standard (hash-based); LMS (Leighton-Micali) used in DNSSEC; statelessness is crucial for practical deployment.

### [PRIM-045] designated-verifier-proof
- **Atom/Composite:** Composite
- **Definition:** Designated verifier proof: prover convinces a specific verifier (who holds a secret key) that a statement is true, without the verifier being able to convince third parties. Used in whistleblowing, verifiable credential issuance.
- **Cost Model:** Prover: runs sigma protocol with verifier's public key as challenge input; verifier: can simulate the proof using their secret key → cannot prove to third parties.
- **Real Wall:** Non-transferability: if verifier's secret key is leaked, the proof becomes transferable; designated verifier property relies on verifier's secret key not being revealed.
- **Cross-Domain Aliases:** non-transferable-proof (cryptography-hashing), designated-attestation (information-theory-coding).

### [PRIM-046] proof-of-knowledge
- **Atom/Composite:** Composite
- **Definition:** Proof of knowledge: zero-knowledge variant where prover demonstrates knowledge of a witness w such that R(x, w) (relation). Extractor exists to recover w from valid proof. Key for digital signatures (Schnorr) and authentication.
- **Cost Model:** Same as sigma protocol or ZK proof system; knowledge error: probability that prover can convince without knowing witness; witness extraction time complexity.
- **Real Wall:** Some protocols (e.g., Schnorr) have perfect special soundness (extractor runs in rewinding); others require stronger assumptions.
- **Cross-Domain Aliases:** witness-proof (information-theory-coding), knowledge-extraction (agentic-reasoning).

### [PRIM-047] verifiable-random-function
- **Atom/Composite:** Composite
- **Definition:** VRF: pseudorandom function where only holder of private key can compute output, but anyone with public key can verify correctness. Used in randomness beacons, blockchain leader election (Ouroboros, Algorand).
- **Cost Model:** VRF output = PRF(private_key, input) + proof of correctness (sigma protocol or zkSNARK); verification: public key + proof; output is unpredictable (until revealed).
- **Real Wall:** VRF must be unique (same input always produces same output); randomness beacon must be unbiasable; VRF+proof prevents manipulation by block producer.
- **Cross-Domain Aliases:** pseudorandom-verifiable (cryptography-hashing), randomness-beacon (networking).
- **Notes:** ECVRF (Elliptic Curve VRF, IETF draft); Algorand's VRF = proof of elapsed time; Chainlink VRF.

### [PRIM-048] time-lock-puzzle
- **Atom/Composite:** Primitive
- **Definition:** Time-lock puzzle: encrypt a message that can only be decrypted after a predetermined time, without relying on a trusted third party. Rivest-Shamir-Wagner (RSW): sequential squaring in RSA group; VDF (Verifiable Delay Function) is the function-like version.
- **Cost Model:** Sequential squaring: T steps of modular multiplication; parallel speedup limited by circuit depth; parallelism doesn't help much.
- **Real Wall:** Requires assumption that squaring is sequential (no shortcut); parallel hardware could accelerate; newer VDF constructions use modular exponentiation (T steps = exponent of size T).
- **Cross-Domain Aliases:** timed-release (cryptography-hashing), sequential-computation (control-numerical-opt).

### [PRIM-049] mix-network
- **Atom/Composite:** Composite
- **Definition:** Mixnet: chain of mix servers; each message is encrypted with each server's public key in layers; each server decrypts one layer and forwards; observers see only entry and exit, not who sent to whom.
- **Cost Model:** n mix servers, n layers of encryption; each mix node: decrypt + shuffle + output; latency: sequential (each mix must process before next); bandwidth: constant per hop.
- **Real Wall:** Active attacks (malicious mixes that correlate inputs/outputs); Sphinx format (Dan Boneh) ensures fixed-size packets; decryption consistency (each mix verifies output format).
- **Cross-Domain Aliases:** onion-routing (networking), traffic-decorrelation (information-theory-coding).
- **Notes:** Mixnet vs. DC-mix (simple algebraic property); Sphinx (Golle et al., 2002) standardized format; used in voting systems (Helios), anonymous email (Mixminion).

### [PRIM-050] private-set-intersection
- **Atom/Composite:** Composite
- **Definition:** PSI: two parties each hold a set; they compute the intersection without revealing their full sets. Variants: circuit-based (Yao's GC), OT-based (fast), DH-based (efficient for unbalanced sets).
- **Cost Model:** Balanced PSI (|A| ≈ |B|): O(|A| log |B|) using OPRF + bloom filter; unbalanced PSI: server with large set, client with small set: O(|A|) OPRF evaluations.
- **Real Wall:** Communication: O(|A|) for unbalanced; computational: O(|A| log |B|); malicious security increases cost; threshold PSI = only count of intersection.
- **Cross-Domain Aliases:** set-intersection-private (information-theory-coding), privacy-preserving-join (database-streaming).
- **Notes:** Used in contact discovery (Signal), ad targeting without sharing user lists; related: PSI-CA (cardinality, just the count).

## Appendix: Primitive Count

Total primitives in cryptography-advanced domain: **50**
