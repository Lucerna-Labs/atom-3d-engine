# Cryptography / Hashing — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## Hashing Atoms

### hash-cryptographic (cross-domain alias: `SHA-family`, `collision-resistant-hash`, `one-way-hash`)
**Domain:** Cryptography / Hashing
**Definition:** Map arbitrary input to fixed-length output with: pre-image resistance (given h, hard to find x: hash(x)=h), second pre-image resistance (given x, hard to find x'≠x: hash(x')=hash(x)), collision resistance (hard to find any x≠x': hash(x)=hash(x')).
**Atom or composite:** Composite: Merkle-Damgård construction (for MD/SHA family): divide input into blocks → compress(block, state) iteratively → output final state.
**Cost model:** One block cipher per 512 bits (for SHA-256: 64 rounds of SHA-256 compression). Purposefully slow — this is the generator. The HASH OPERATION (one compression) is the primitive.
**Real wall?** Yes — collision resistance requires 2^{n/2} work for n-bit output (birthday paradox). The output length n IS the security parameter — no way around it.
**Cross-domain wiring:** In streaming: the hash function IS the stream cipher. In retrieval: cryptographic hash is the expensive overkill; non-cryptographic hash is the cheap primitive.
**Notes:** The distinction: cryptographic hash (SHA-256, BLAKE3) = expensive, collision-resistant, one-way. Non-cryptographic hash (MurmurHash, xxHash) = cheap, non-resistant, used for hashing-as-dictionary. BLAKE3 is the modern fast cryptographic hash — uses the Bao tree mode for parallel hashing.

### hash-fast (cross-domain alias: `MurmurHash`, `xxHash`, `non-crypto-hash`)
**Domain:** Cryptography / Hashing
**Definition:** Fast, non-cryptographic hash function designed for hash tables and Bloom filters. Provides good avalanche (output bits depend on all input bits) but no security guarantees.
**Atom or composite:** Atom
**Cost model:** 1-3 bytes per CPU cycle on modern processors. xxHash3 achieves ~20 GB/s on single core.
**Real wall?** No.
**Cross-domain wiring:** In streaming: fast hash = fast sketch primitive. In retrieval: fast hash = feature hashing, MinHash, locality-sensitive hashing.
**Notes:** The generator/primitive split is clearest here: MurmurHash3 IS the primitive (one hash). SHA-256 IS the expensive generator. Most retrieval and streaming use fast hash, not cryptographic hash.

### hash-rolling (cross-domain alias: `rolling-hash`, `Rabin-Karp`, `polynomial-hash`)
**Domain:** Cryptography / Hashing
**Definition:** Maintain a running hash over a sliding window: H_{i+1} = (H_i − a_i·b^{k-1} + a_{i+k}·b^{0}) mod M. Allows O(1) update as the window slides.
**Atom or composite:** Composite: initialize with first k chars → for each new char: subtract old contribution + add new contribution + multiply by base.
**Cost model:** O(1) per character — extremely cheap. This is the entire value of rolling hash.
**Real wall?** No. But collisions are possible (polynomial hash with finite modulus). Use large prime modulus or multiple hash functions to reduce.
**Cross-domain wiring:** In text retrieval: rolling hash finds all k-gram occurrences in O(1) per position. In streaming: sliding window frequency estimation. In signal: running autocorrelation.
**Notes:** This is the rolling hash: updating a hash by removing the leftmost character and adding the rightmost in O(1). It is the foundation of Rabin-Karp string matching.

### hash-murmur3 (cross-domain alias: `fast-avalanche-hash`, `pipeline-hash`)
**Domain:** Cryptography / Hashing
**Definition:** Fast non-cryptographic hash with excellent avalanche properties (1-bit change in input → ~50% of output bits flip). Used in Cassandra, Elasticsearch, Spark, many production systems.
**Atom or composite:** Atom
**Cost model:** ~2.5 bytes per cycle on modern x86.
**Real wall?** No.
**Cross-domain wiring:** In streaming: MurmurHash3 as the hash function in Bloom filters and Count-Min sketches. In databases: partition routing. In ML: feature hashing.
**Notes:** Has two main variants: x86 (fast, non-aligned reads) and x64 (better avalanche). The two produce different outputs — don't mix them.

---

## Locality-Sensitive Hashing Atoms

### lsh-jl (cross-domain alias: `random-projection-hash`, `cosine-LSH`, `E2LSH`)
**Domain:** Cryptography / Hashing
**Definition:** Hash vectors so that similar vectors collide with high probability: pick random hyperplane directions, hash by which side of each hyperplane the vector falls. cos(θ) ≈ fraction of matching bits.
**Atom or composite:** Composite: sample random directions from Gaussian(0,1) → project(vector, direction) → sign → bit. Repeat k times → k-bit signature.
**Cost model:** k·d operations per vector (k hash values, each dot product of dimension d). d can be large (embedding dimension).
**Real wall?** Yes — the number of bits k required to get collision probability P = (1 − cos(θ))/2 is k = O(1/θ²). Close angles need many bits.
**Cross-domain wiring:** This IS the random projection for dimensionality reduction (Johnson-Lindenstrauss). In retrieval: the first LSH for cosine similarity. In signal: random direction measurement.
**Notes:** LSH for cosine similarity: two vectors have the same hash on bit i iff they are on the same side of the i-th random hyperplane. The probability of matching bits = 1 − arccos(cos(θ))/π.

### minhash-lsh (cross-domain alias: `jaccard-lsh`, `set-similarity-hash`, `banding`)
**Domain:** Cryptography / Hashing
**Definition:** For Jaccard similarity: MinHash signature + LSH banding. Split k minhash values into b bands of r rows each. Two sets collide if all r rows in any band match.
**Atom or composite:** Composite: compute k minhash values → split into b bands → hash each band to a bucket → candidates collide in the same bucket.
**Cost model:** k hash computations per set + b hashing operations. Banding changes the complexity curve: S-curves control false positive/negative rates.
**Real wall?** No. But the S-curve trade-off is fundamental: bands/rows control the probability curve shape.
**Cross-domain wiring:** LSH banding is a threshold function on the S-curve — it concentrates collision probability near the Jaccard threshold. In retrieval: banding is the same as a hard threshold on similarity.
**Notes:** S-curve: P(collision) ≈ 1 − (1 − s^r)^b where s = Jaccard similarity. The peak of the derivative is at s ≈ (1/b)^{1/r}.

### simhash-lsh (cross-domain alias: `hamming-LSH`, `fingerprint-hash`, `differential-LSH`)
**Domain:** Cryptography / Hashing
**Definition:** Directional hashing for Hamming distance: hash(v) = sign(A·v) where A is random ±1 matrix. Two vectors with Hamming distance d have hash distance ≈ d/bit_count.
**Atom or composite:** Composite: A = random ±1 matrix → h = sign(A·v) → quantize to 0/1.
**Cost model:** O(d·k) where k is the number of hash bits. Simpler than MinHash.
**Real wall?** No. But the Hamming distance in hash space is an approximation — small changes in the vector can flip many hash bits.
**Cross-domain wiring:** SimHash is the basis of Google's web duplicate detection. In retrieval: near-duplicate detection via Hamming distance of SimHash fingerprints. In signal: one-bit quantization of random projections.
**Notes:** Finding all near-duplicates within Hamming distance ≤ t: for each bit position, flip the bit and query that bucket — total (k+1) bucket lookups for Hamming distance ≤ 1. Generalizes to Hamming distance ≤ t.

### lsh-hamming (cross-domain alias: `bitstring-LSH`, `binary-code-LSH`)
**Domain:** Cryptography / Hashing
**Definition:** For binary vectors: two vectors collide if their Hamming distance is below a threshold. Can be implemented by hashing all (n choose d) subsets of d positions.
**Atom or composite:** Composite: for each subset of d positions: extract bits → combine → hash to bucket.
**Cost model:** Exponential in d — only feasible for small d. Alternative: use Gray codes to enumerate close Hamming neighbors.
**Real wall?** Yes — enumerating all Hamming neighbors is 2^d for distance d. For d=3, that's 8 neighbors — manageable. For d=10, it's 1024. Beyond that, impractical.
**Cross-domain wiring:** In retrieval: Hamming-constrained search = find all binary vectors within Hamming distance d. In databases: Hamming distance index.
**Notes:** For Hamming distance ≤ 1: just check all buckets with 1 bit flipped. For distance ≤ 2: check all buckets with 2 bits flipped — still manageable with optimization.

---

## Privacy / Security Atoms

### hmac (cross-domain alias: `keyed-hash`, `message-auth`, `authenticated-hash`)
**Domain:** Cryptography / Hashing
**Definition:** HMAC = H(K ⊕ opad || H(K ⊕ ipad || message)). Provides keyed authentication using an underlying hash function H. Proves message integrity and sender authenticity.
**Atom or composite:** Composite: pad key to block size → xor with ipad/opad → hash → xor → hash again.
**Cost model:** Two hash evaluations — roughly 2× the cost of a bare hash.
**Real wall?** No.
**Cross-domain wiring:** In APIs: HMAC for request authentication (AWS, many OAuth). In streaming: keyed sketches for authenticated frequency estimation.
**Notes:** The key K is the generator; HMAC is the primitive that uses it. Without the key, you can't verify the hash.

### secret-share (cross-domain alias: `Shamir-share`, `threshold-crypto`, `polynomial-split`)
**Domain:** Cryptography / Hashing
**Definition:** Shamir's secret sharing: share secret s into n shares such that any k shares reconstruct s, but fewer than k give no information. Share i = (i, f(i)) where f is a random polynomial of degree k−1 with f(0) = s.
**Atom or composite:** Composite: sample random polynomial f(x) with f(0) = s → evaluate f at n points → distribute shares. Reconstruct via Lagrange interpolation at x=0.
**Cost model:** Polynomial evaluation: O(k) per share. Reconstruction: O(k²) for Lagrange interpolation.
**Real wall?** No.
**Cross-domain wiring:** In distributed systems: secret sharing for threshold cryptography (e.g., n-of-n signing). In retrieval: federated learning could use secret sharing to aggregate gradients without any party seeing individual gradients.
**Notes:** Information-theoretically secure: fewer than k shares give zero information about s (in information-theoretic sense, not computational sense).

### commitment (cross-domain alias: `hash-commit`, `binding-commit`, `commit-reveal`)
**Domain:** Cryptography / Hashing
**Definition:** Commit to a value v by publishing H(r || v) where r is a random nonce. The commitment is binding (can't be opened to a different value) and hiding (doesn't reveal v).
**Atom or composite:** Composite: choose random r → compute commit = H(r || v) → publish commit.
**Cost model:** One hash evaluation.
**Real wall?** No.
**Cross-domain wiring:** In zero-knowledge proofs: the commitment is the "sealed envelope." In retrieval: a hash of the document = commitment to the document's content.
**Notes:** The binding property requires H to be collision-resistant. The hiding property requires r to be large enough that H(r || v) doesn't leak v.

---

## Key Derivation Atoms

### kdf (cross-domain alias: `PBKDF2`, `Argon2`, `HKDF`, `derive`)
**Domain:** Cryptography / Hashing
**Definition:** Derive cryptographic keys from a password or master key using a deliberately slow function. PBKDF2: apply HMAC repeatedly (typically 100,000+ iterations). Argon2: memory-hard + CPU-hard.
**Atom or composite:** Composite: PBKDF2 = HMAC^iterations(password, salt) → repeat with output as next key. Argon2 = Blake2b with memory allocation + parallelism parameters.
**Cost model:** Deliberately expensive — iterations/memory/cost are the security parameters. The expense is the feature.
**Real wall?** Yes — the cost of password cracking is proportional to 1/cost. With Argon2 and 100MB memory, the cost is measured in seconds per guess — infeasible for real-time cracking.
**Cross-domain wiring:** In streaming: KDF is used to derive multiple independent keys from a master key for different streams. In retrieval: deterministic key derivation = deterministic salt for hashing.
**Notes:** The distinction: HKDF (fast, for deriving keys from high-entropy inputs like DH outputs) vs PBKDF2/Argon2 (slow, for password-based key derivation).

### hkdf (cross-domain alias: `extract-expand`, `fast-KDF`, `information-distill`)
**Domain:** Cryptography / Hashing
**Definition:** Two-phase: extract (convert entropy to a pseudo-random key) + expand (derive multiple keys from the key). HKDF-Extract: HMAC-SHA256(salt, IKM). HKDF-Expand: HMAC-SHA256(K, info || counter) repeated.
**Atom or composite:** Composite: extract = HMAC(salt, input) → expand = HMAC(K, info || i) for i=1..n.
**Cost model:** One extract + n expand rounds. Fast — one HMAC per round.
**Real wall?** No.
**Cross-domain wiring:** HKDF-Extract = information-theoretic key distillation (removing structure from entropy). HKDF-Expand = producing independent derived keys = producing independent random numbers from one seed.
**Notes:** The "salt" in HKDF-Extract is non-secret randomness that improves the extractor's properties. In retrieval: this is analogous to IDF being the "salt" that decorrelates the retrieval weights from the raw frequencies.

---

## Fuzzy / Similarity Hashing Atoms

### fuzzy-hash (cross-domain alias: `ssdeep`, `sdhash`, `context-triggered-pieces`)
**Domain:** Cryptography / Hashing
**Definition:** For near-duplicate detection of binary/text data: segment input by context (looking for low-entropy regions), hash each segment, combine the segment hashes. Changes near segment boundaries change few segment hashes.
**Atom or composite:** Composite: find context breakpoints → hash each block → combine hashes in sequence → output variable-length fingerprint.
**Cost model:** One pass over the input + hash per block. More expensive than a single hash.
**Real wall?** No. But the segmentation strategy matters — bad segmentation = poor similarity detection.
**Cross-domain wiring:** In retrieval: fuzzy hashing = sub-document fingerprinting = shingles with position encoding. In streaming: variable-size block hashing = adaptive summarization.
**Notes:** ssdeep uses a rolling hash to find variable-size block boundaries, then hashes each block. The resulting fingerprint can be compared for similarity.

---

## Summary: Cryptography/Hashing Atom → Cross-Domain Wiring

| Crypto Primitive | Retrieval Alias | Signal Alias | Linear Algebra Alias |
|---|---|---|---|
| hash-cryptographic | signature verification | secure hash | matrix commitment |
| hash-fast | Bloom filter hash | stream cipher | random projection basis |
| hash-rolling | k-gram hash | running hash | sliding window |
| lsh-jl | random embed LSH | random projection | JL transform |
| minhash-lsh | Jaccard LSH | set overlap | collision coding |
| simhash-lsh | near-duplicate | directional hash | 1-bit projection |
| lsh-hamming | binary-code search | Hamming distance | binary signature |
| hmac | API auth | keyed authentication | authenticated sketch |
| kdf | derived-key | key expansion | entropy extraction |
| fuzzy-hash | sub-doc fingerprint | piecewise hash | adaptive chunk |
| secret-share | threshold retrieval | multi-party computation | distributed SVD |

---

*Last updated: 2026-06-21*
*Source doctrine: The Painted Fence — Jesse*


---

## Advanced LSH Atoms

### lsh-multi-probe (cross-domain alias: `probe-neighbors`, `query-adaptive-LSH`, `multi-lane`)
**Domain:** Cryptography / Hashing
**Definition:** Given a query hash, probe not just the bucket but neighboring buckets by perturbing one hash bit at a time. This dramatically improves recall with minimal extra cost.
**Atom or composite:** Composite: compute query hash → for each bit position: flip bit → probe resulting bucket → accumulate candidates.
**Cost model:** b probes for b bit flips. With k=24 bits per bucket, probing b=2 flips adds ~24 extra probes and dramatically improves recall.
**Real wall?** Yes — the multi-probe sequence must be precomputed. The optimal probe sequence depends on the LSH parameters.
**Cross-domain wiring:** Multi-probe LSH = querying neighboring hash buckets = checking nearby hash values = same as checking nearby vectors in the Hamming space.
**Notes:** Multi-probe LSH (Qin et al.) shows that for 24-bit hashes, probing b=2 flips recovers most of the recall lost to the S-curve.

### lsh-learned (cross-domain alias: `data-dependent-LSH`, `learned-hashing`, `semantic-LSH`)
**Domain:** Cryptography / Hashing
**Definition:** Instead of random hyperplanes, learn the hash functions from data. Data-dependent LSH can achieve much better recall than data-independent (random) LSH for a given hash length.
**Atom or composite:** Composite: train hash functions on data → use learned hash functions for bucketing.
**Cost model:** Training is expensive (supervised hashing, contrastive learning). But the hash evaluation is as cheap as random LSH.
**Real wall?** Yes — learned hash functions may overfit to the training distribution and fail on out-of-distribution queries.
**Cross-domain wiring:** Learned LSH = trained feature extractor + hash function. In retrieval: BERT + hash = learned LSH for semantic similarity.
**Notes:** SPTAG (Microsoft) uses random k-d trees for approximate nearest neighbor search — a data-dependent spatial partitioning rather than hash-based.

### lsh-stable-projection (cross-domain alias: `LSH-stable-distribution`, `p-stable-LSH`, `Euclidean-LSH`)
**Domain:** Cryptography / Hashing
**Definition:** For ℓ₂ (Euclidean) distance, use p-stable distributions (Gaussian). Each hash bucket is defined by a random projection and a bucketing interval. The distance between vectors is approximated by the collision probability.
**Atom or composite:** Composite: sample from p-stable distribution → project vectors → quantize to intervals → hash = interval ID.
**Cost model:** k projections per vector. Each projection = dot product of dimension d.
**Real wall?** Yes — the number of buckets is 2^{k} which can be very large. The resolution is limited by the projection noise.
**Cross-domain wiring:** p-stable LSH = measuring Euclidean distance via random projections. In signal: measuring signal energy via random projections.
**Notes:** The collision probability for p-stable LSH: P(collision) = F_{p}(δ/τ) where δ = ||v₁−v₂||₂, τ is the bucket width, and F_p is the CDF of the p-stable distribution.

---

## Secure Computation Atoms

### pir-query (cross-domain alias: `private-retrieval`, ` oblivous-fetch`, `client-privacy`)
**Domain:** Cryptography / Hashing
**Definition:** Query a database without revealing which record was accessed. PIR: client generates a query that the server evaluates without knowing which item was requested. Simple PIR: replicated databases with RSA blinding.
**Atom or composite:** Composite: client: generate query vector (all 0s except 1 at desired index) → encrypt → server: dot product of encrypted query with database → return → client: decrypt to get record.
**Cost model:** Communication complexity: O(N) for simple PIR. Computational PIR: O(N^δ) for small δ. Practical PIR uses homomorphic encryption or multi-server PIR.
**Real wall?** Yes — PIR communication cost is very high. Even the most efficient PIR schemes require downloading the entire database (or a large fraction) to hide the query.
**Cross-domain wiring:** PIR = encrypted retrieval without revealing the query = the theoretical foundation of privacy-preserving retrieval. In retrieval: PIR = semantic search without the server knowing the query.
**Notes:** The information-theoretic PIR (IT-PIR) uses multiple non-colluding servers. The computational PIR (CPIR) uses homomorphic encryption. Popcorn (Gonen et al.) shows practical PIR is feasible with ~1MB bandwidth per query.

###oram-access (cross-domain alias: `oblivious-RAM`, `access-pattern-hide`, `ORAM`)
**Domain:** Cryptography / Hashing
**Definition:** Access a memory location without revealing which location was accessed. Path ORAM: map all blocks to a tree; on each access, shuffle the path back to the root.
**Atom or composite:** Composite: on access: download full path → decrypt blocks → find target → evict and reassign blocks → write path back → update stash.
**Cost model:** O(log N) per access (download path). This is much more expensive than direct memory access (1 access).
**Real wall?** Yes — ORAM has ~100× overhead over direct memory access. This makes it impractical for fine-grained memory accesses. Coarse-grained ORAM (for large blocks) is more practical.
**Cross-domain wiring:** ORAM hides the access pattern = the same as hiding which documents you retrieved. In retrieval: ORAM = private retrieval without even the access pattern revealing the query.
**Notes:** The Squoram and Ring ORAM are practical variants. The fundamental limit: you cannot access memory without some information leakage (otherwise you can't find the data).

### mpc-garble (cross-domain alias: `Yao-garbled-circuit`, `secure-two-party`, `boolean-evaluate`)
**Domain:** Cryptography / Hashing
**Definition:** Yao's garbled circuits: one party (garbler) encrypts a Boolean circuit; the other party (evaluator) evaluates it on encrypted inputs (using oblivious transfer). The evaluator learns the circuit output but nothing about the other party's input.
**Atom or composite:** Composite: garbler: build circuit → assign labels to wires → encrypt gates → send. Evaluator: OT for input labels → evaluate gates → reveal output label.
**Cost model:** Circuit generation: O(gates). Evaluation: O(gates) homomorphic operations. OT: O(k) for k-bit input.
**Real wall?** Yes — garbled circuits are expensive for large circuits. The communication complexity = circuit size. Practical MPC uses secret sharing (SPDZ,aby) instead for arithmetic circuits.
**Cross-domain wiring:** Garbled circuits = encrypted computation = universal. In ML: evaluating a neural network via garbled circuits = secure inference without revealing the model or input.
**Notes:** The half-gate optimization (Zahur et al., 2015) reduced garbled circuit size by 2× per AND gate — the main efficiency improvement that made Yao practical.

### zk-proof (cross-domain alias: `SNARK`, `STARK`, `succinct-proof`)
**Domain:** Cryptography / Hashing
**Definition:** Prove the knowledge of a witness without revealing it. SNARK: Succinct Non-interactive ARgument of Knowledge. Proof is short (O(1) size) and verification is fast (O(1) or O(log n)).
**Atom or composite:** Composite: prover: encode witness into polynomial → commit → send proof. Verifier: check polynomial evaluations via elliptic curve pairings.
**Cost model:** Proving: O(N·log N) for SNARKs (polynomial IOP). Verification: O(1) or O(log N).
**Real wall?** Yes — the trusted setup requirement for most SNARKs (Groth16, Plonk) is a security concern. STARKs avoid trusted setup but have larger proof sizes.
**Cross-domain wiring:** ZK proof = proving a computation was done correctly without revealing the inputs. In ML: ZKML = proving that a model inference was computed correctly without revealing the model or input.
**Notes:** Groth16 (2016) has the smallest proofs (3 elements) but requires a per-circuit trusted setup. Plonk (2020) and Halo2 (2020) allow universal trusted setup. STARKs (Ben-Sasson et al.) are transparent (no trusted setup) but proofs are larger (~100KB).

---

## Advanced Cryptographic Primitives Atoms

### digital-signature (cross-domain alias: `ECDSA`, `Ed25519`, `BLS-signature`)
**Domain:** Cryptography / Hashing
**Definition:** Bind a message to a public key: sign(message, private_key) → signature. Verify(signature, message, public_key) → accept/reject. Provides authentication and non-repudiation.
**Atom or composite:** The signature scheme is the generator. The primitive: sign + verify.
**Cost model:** ECDSA: 1 modular multiplication + 1 hash. Ed25519: 1 hash + point arithmetic. BLS: 1 hash + 1 pairing (verification only, signing is cheap).
**Real wall?** No. But ECDSA has subtle implementation pitfalls (nonce reuse → key recovery). Ed25519 is designed to avoid these.
**Cross-domain wiring:** Digital signature = message authentication code with public verifiability. In retrieval: signing a retrieved document = proving its provenance.
**Notes:** BLS signatures (Boneh–Lynn–Shacham) have the special property that signatures can be aggregated: n signatures can be combined into one aggregate signature. This is very useful for distributed systems (threshold signatures).

### key-exchange (cross-domain alias: `DH`, `ECDH`, `X25519`)
**Domain:** Cryptography / Hashing
**Definition:** Derive a shared secret over a public channel. DH: g^a mod p, g^b mod p → shared = g^{ab} mod p. ECDH: same on an elliptic curve group.
**Atom or composite:** Composite: each party generates ephemeral key → exchange public values → compute shared secret.
**Cost model:** Modular exponentiation (DH) or point multiplication (ECDH). ECDH with X25519 is much faster than mod-p DH.
**Real wall?** No.
**Cross-domain wiring:** Key exchange = establishing shared randomness = the same as the DH key exchange in cryptography and the shared secret derivation in HKDF.
**Notes:** X25519 (Bernstein, 2006) is the recommended ECDH curve — it is fast, constant-time, and has a clean specification. Post-quantum key exchange (Kyber, Dilithium) is being standardized.

### authenticated-encrypt (cross-domain alias: `AES-GCM`, `ChaCha20-Poly1305`, `AEAD`)
**Domain:** Cryptography / Hashing
**Definition:** Encrypt and authenticate: (ciphertext, tag) = AEAD(key, nonce, plaintext, associated_data). Provides confidentiality + integrity. AES-GCM uses CTR mode + GMAC authentication.
**Atom or composite:** Composite: encrypt (CTR) → compute GMAC(tag) → append tag.
**Cost model:** AES-GCM is ~3 cycles/byte in hardware AES-NI. ChaCha20-Poly1305 is fast in software without AES-NI.
**Real wall?** Yes — nonce reuse in GCM is catastrophic (forges are possible). Each nonce must be used at most once.
**Cross-domain wiring:** AEAD = authenticated channel = same as TLS record protection. In retrieval: AEAD = encrypted retrieval with integrity verification.
**Notes:** TLS 1.3 mandates AEAD ciphersuites. AES-GCM + ChaCha20-Poly1305 are the two dominant AEAD schemes.

### MPC-additive-share (cross-domain alias: `secret-share`, `additive-share`, `Shamir-share`)
**Domain:** Cryptography / Hashing
**Definition:** Secret-share a value x into n shares s₁,...,s_n such that any t shares reconstruct x, but fewer than t give no information. Additive: x = s₁ ⊕ s₂ ⊕ ... ⊕ s_n. Shamir: polynomial interpolation.
**Atom or composite:** Composite: secret-share: generate random s₁,...,s_{t−1} → s_t = x ⊕ s₁ ⊕ ... ⊕ s_{t−1}. Reconstruct: XOR all shares.
**Cost model:** Sharing: O(n) random number generation. Reconstruction: O(n) XOR operations.
**Real wall?** No.
**Cross-domain wiring:** Additive secret sharing = the same as decomposing a vector into orthogonal components in linear algebra. In ML: secret sharing = splitting a model into shares that sum to the model.
**Notes:** Shamir's secret sharing is more flexible (any threshold t of n shares works) but requires field arithmetic. Additive sharing is simpler (XOR) but requires all n shares.

---

## Similarity / Distance Atoms

### distance-jaro (cross-domain alias: `string-similarity`, `edit-proximity`, `fuzzy-match`)
**Domain:** Cryptography / Hashing
**Definition:** Jaro similarity: count matching characters within a window of half the string length + 1, then count transpositions. Jaro-Winkler adds a prefix bonus for matching first characters.
**Atom or composite:** Composite: scan both strings → find matching characters within window → count transpositions → compute similarity.
**Cost model:** O(min(m,n)) per comparison. Near-linear.
**Real wall?** No.
**Cross-domain wiring:** Jaro-Winkler = weighted string edit distance with prefix emphasis. In retrieval: fuzzy name matching for entity resolution.
**Notes:** Jaro-Winkler gives high scores to strings with matching prefixes — good for names ("Smith" vs "Smithe") but can miss suffixes ("John" vs "Johan").

### distance-dice (cross-domain alias: `dice-coefficient`, `bigram-similarity`, `set-overlap`)
**Domain:** Cryptography / Hashing
**Definition:** Dice coefficient = 2·|A ∩ B| / (|A| + |B|). For bigrams, it is a set-similarity measure over character bigrams.
**Atom or composite:** Composite: extract bigrams from both strings → compute Dice = 2·intersection / total.
**Cost model:** O(m+n) for extracting bigrams + O(min(|A|,|B|)) for intersection.
**Real wall?** No.
**Cross-domain wiring:** Dice = 2·Jaccard for sets (Jaccard = |A∩B|/|A∪B|, Dice = 2·|A∩B|/(|A|+|B|)). For bigram sets, Dice = 2·Jaccard.
**Notes:** Dice is closely related to Jaccard: Dice = 2·Jaccard / (1 + Jaccard). Dice ∈ [0,1], Jaccard ∈ [0,1], the mapping is monotonic.

### locality-compare (cross-domain alias: `semantic-similarity`, `embedding-cosine`, `learned-metric`)
**Domain:** Cryptography / Hashing
**Definition:** Compare two objects using a learned metric (e.g., a neural network that maps both to a comparison score). Contrastive learning (SimCLR, triplet loss) trains the metric.
**Atom or composite:** The metric network is the generator. The primitive: score = metric(obj₁, obj₂).
**Cost model:** One forward pass of the metric network per comparison.
**Real wall?** Yes — the learned metric is specific to the training distribution. Out-of-distribution queries may have poor metric quality.
**Cross-domain wiring:** Learned metric = the same as learned similarity in retrieval (neural reranker). In ML: Siamese networks = learned metric learning.
**Notes:** Metric learning with triplet loss: L = max(0, d(a,p) − d(a,n) + margin). The metric learns to pull similar pairs closer and dissimilar pairs apart.

---

*Last updated: 2026-06-21 (expanded with advanced LSH, secure computation, advanced crypto, similarity metrics)*
*Source doctrine: The Painted Fence — Jesse*


---

## Side-Channel & Hardware Security Atoms

### dpa-attack (cross-domain alias: `differential-power-analysis`, `power-side-channel`, ` DPA`)
**Domain:** Cryptography / Hashing
**Definition:** Exploit power consumption variations during cryptographic operations to recover secret keys. Attack: measure power traces during encryptions → partition by hypothetical key bit → compute difference of means → peaks indicate key-dependent leakage.
**Atom or composite:** Composite: acquire many power traces → for each key hypothesis: partition traces → compute difference mean → detect key-dependent pattern.
**Cost model:** O(N_traces × N_key_bits × N_time_points) for classical DPA. Template DPA: fewer traces but requires characterization.
**Real wall?** Yes — countermeasures (masking, hiding) break DPA by making power consumption independent of intermediate values.
**Cross-domain wiring:** DPA = statistical difference detection between subgroups. In ML: the intermediate computation of a model = the side-channel leakage point.
**Notes:** First published by Kocher et al. in 1999. DPA is the foundation of all power analysis attacks.

### cpa-attack (cross-domain alias: `correlation-power-analysis`, `Pearson-correlation`, `key-recovery`)
**Domain:** Cryptography / Hashing
**Definition:** Compute the correlation between the hypothetical intermediate value (e.g., AES S-box output) and measured power consumption. The correct key byte shows the highest correlation.
**Atom or composite:** Composite: for each key guess: compute hypothetical intermediate value → correlate with power traces → select key guess with max |correlation|.
**Cost model:** O(N_traces × N_key_guesses) for correlation computation. N_key_guesses = 256 for one byte of AES.
**Real wall?** Yes — CPA is more powerful than DPA (uses all power samples, not just difference of means). Countermeasures must reduce the correlation.
**Cross-domain wiring:** CPA = maximizing Pearson correlation between model and observation. In statistics: the key guess is the MLE of the key.
**Notes:** CPA typically requires fewer traces than DPA for the same success rate.

### template-attack (cross-domain alias: `profiling-attack`, `stochastic-gate`, `maximum-likelihood`)
**Domain:** Cryptography / Hashing
**Definition:** Two-phase: (1) profiling: characterize leakage on a device with known key; (2) attack: apply Bayesian inference using the profiling model to recover the target key.
**Atom or composite:** Composite: profiling phase: measure traces with known key → estimate probability distribution of power given intermediate value → attack phase: apply Bayes' theorem.
**Cost model:** Profiling: many traces with known key. Attack: one trace (or few) with unknown key.
**Real wall?** Yes — template attacks require an identical or similar target device. Device-to-device variation reduces template quality.
**Cross-domain wiring:** Template attack = statistical profiling attack = using a surrogate model (profiling phase) to attack the real target. In ML: transfer learning attack.
**Notes:** Machine learning classifiers (SVM, CNN) can serve as the profiling model — deep learning side-channel attacks (DL-SCA) often outperform classical templates.

### fault-injection (cross-domain alias: `glitch`, `laser-fault`, `differential-fault-analysis`)
**Domain:** Cryptography / Hashing
**Definition:** Deliberately inject faults into a cryptographic device (voltage glitch, clock glitch, laser) to cause incorrect computation. DFA: compare correct and faulty outputs to deduce the key.
**Atom or composite:** Composite: inject fault → collect faulty output → compare with correct output → solve for key bits.
**Cost model:** Requires physical access (probing, laser, EM probe). The fault injection device is specialized and expensive.
**Real wall?** Yes — fault injection requires precise timing and physical proximity. Countermeasures (redundancy, sensors) can detect and mitigate.
**Cross-domain wiring:** Fault injection = adversarial perturbation of computation. In ML: adversarial examples = fault injection in the ML domain.
**Notes:** Differential Fault Analysis (DFA) on AES: injecting one fault into the last round lets you recover the last round key in seconds.

### em-sidechannel (cross-domain alias: `EM-emission`, ` TEMPEST`, `electromagnetic-leakage`)
**Domain:** Cryptography / Hashing
**Definition:** Electromagnetic emissions from a device carry information about the data being processed. EM attack: measure EM field with a near-field probe → correlate with intermediate computations → recover key.
**Atom or composite:** Composite: position EM probe near device → acquire EM traces → apply CPA/DPA (same as power analysis) → recover key.
**Cost model:** Requires physical proximity (cm to m) and equipment (EM probe, oscilloscope). Near-field probes are cheap (<$100); real-time oscilloscopes are expensive.
**Real wall?** Yes — EM shielding (Faraday cage) and filtering reduce but do not eliminate EM leakage. TEMPEST is the US specification for EM security.
**Cross-domain wiring:** EM leakage = information leakage through a physical side channel. In retrieval: timing side channels in a retrieval system.
**Notes:** Simple EM analysis (SEMA) uses the EM trace shape to identify operations. Correlation EM analysis (CEMA) = CPA on EM traces.

### power-analysis-countermeasure (cross-domain alias: `masking`, `hiding`, `shuffling`)
**Domain:** Cryptography / Hashing
**Definition:** Countermeasures against power analysis: (1) masking: split sensitive values into n random shares that recombine; (2) hiding: make power consumption independent of data (dual-rail logic); (3) shuffling: randomize the order of operations.
**Atom or composite:** Composite: masking: share computation across shares → recombine only at the end. Hiding: use logic styles with data-independent power. Shuffling: randomize operation order.
**Cost model:** Masking: 2-4× overhead per operation (more shares = more overhead). Hiding: significant area/power overhead. Shuffling: minimal overhead.
**Real wall?** Yes — all countermeasures add overhead. Masking is the most common but can be broken by higher-order attacks (exploiting leakage from multiple shares simultaneously).
**Cross-domain wiring:** Masking = secret sharing applied to computation = same as MPC's additive sharing. In ML: model cloaking = adding noise to hide model internals.
**Notes:** First-order masking defeats first-order DPA. Second-order DPA defeats first-order masking. Security requires masking order > attack order.

### hardware-trojan-detect (cross-domain alias: `Trojan-insertion`, `side-channel-detection`, `logic-test`)
**Domain:** Cryptography / Hashing
**Definition:** Detect malicious modifications to hardware (Trojan circuits). Methods: (1) logic testing: apply test vectors and look for unexpected behavior; (2) side-channel: compare power/EM fingerprints of genuine vs Trojan-infected chips.
**Atom or composite:** Composite: logic testing: apply test vectors → detect unexpected outputs. Side-channel: compare fingerprints → detect statistical deviation.
**Cost model:** Logic testing: O(N_test_vectors) — exhaustive testing is infeasible. Side-channel: O(N_traces) for fingerprinting.
**Real wall?** Yes — Trojans can be rare-trigger (activated by specific rare conditions) that logic testing may not exercise. Hardware Trojans are a supply-chain threat.
**Cross-domain wiring:** Hardware Trojan detection = anomaly detection in hardware behavior. In ML: detecting backdoored models.
**Notes:** The golden chip approach: compare a known-good chip to the device under test. Golden-free detection uses statistical methods without a reference chip.

### secure-enclave-sgx (cross-domain alias: `Intel-SGX`, `trusted-execution`, `enclave-memory`)
**Domain:** Cryptography / Hashing
**Definition:** SGX: create an enclave (isolated memory region) that is protected from the OS, hypervisor, and even the BIOS. Memory encryption (MEE) protects enclave data in DRAM.
**Atom or composite:** Composite: initialize enclave (measurement-based) → enter enclave via EENTER → execute in isolated environment → exit via EEXIT.
**Cost model:** Enclave entry/exit: ~10,000 cycles. Memory encryption/decryption overhead: 10-15% for enclave memory.
**Real wall?** Yes — SGX is vulnerable to side-channel attacks (Spectre, Meltdown, cache timing). The enclave boundary is not as strong as the threat model implied.
**Cross-domain wiring:** Secure enclave = isolated computation = same as secure multi-party computation without the communication. In retrieval: private retrieval using hardware enclaves.
**Notes:** SGX was followed by TDX (Trust Domain Extensions) which extends the enclave model to full VMs.

### secure-enclave-tee (cross-domain alias: `trusted-execution-environment`, `ARM-TrustZone`, `TEE`)
**Domain:** Cryptography / Hashing
**Definition:** TEE: a secure region of the processor (separate from the normal world). TrustZone (ARM): split the processor into Secure and Normal worlds. GlobalPlatform TEE: API standard for trusted applications.
**Atom or composite:** Composite: define secure region boundary → switch world via SMC (Secure Monitor Call) → execute in secure world.
**Cost model:** World switch: ~1000-2000 cycles. TEE provides strong isolation but less protection than SGX against physical attacks.
**Real wall?** Yes — TEE security depends on the correctness of the Trusted OS in the TEE. Bugs in the TEE compromise everything.
**Cross-domain wiring:** TEE = isolated execution environment = the same as a sandboxed computation. In retrieval: TEE-based private retrieval without cryptography.
**Notes:** TrustZone is used in billions of ARM devices (mobile phones, IoT). Apple Secure Enclave is a derivative of TrustZone.

### fault-resilient-enc (cross-domain alias: `redundant-encrypt`, `Triple-modular-redundancy`, `cure`)
**Domain:** Cryptography / Hashing
**Definition:** Detect and correct faults in cryptographic computation: (1) spatial redundancy: compute same operation on multiple cores and compare; (2) temporal redundancy: compute twice and compare; (3) information redundancy: compute check values (CRC, hash).
**Atom or composite:** Composite: redundant computation → compare outputs → if mismatch: reject or retry.
**Cost model:** 2-3× overhead for spatial/temporal redundancy. Information redundancy (check values): low overhead.
**Real wall?** Yes — fault injection can defeat simple redundancy if it attacks all copies simultaneously (e.g., a global clock glitch).
**Cross-domain wiring:** Fault-resilient encryption = Byzantine fault tolerance applied to cryptography. In ML: ensemble methods = redundancy for model robustness.
**Notes:** Concurrent error detection (CED): inject checking logic into the cryptographic datapath. The checker detects faults but adds area/power overhead.

### constant-time-crypto (cross-domain alias: `timing-attack-counter`, `branchless`, `table-lookup-masking`)
**Domain:** Cryptography / Hashing
**Definition:** Prevent timing attacks by ensuring execution time is independent of secret data. Key: no conditional branches on secret values, no secret-dependent table lookups without masking.
**Atom or composite:** Composite: replace all secret-dependent branches with constant-time selects → replace table lookups with masked lookups (or use bitslicing).
**Cost model:** Constant-time implementations are typically 1.5-3× slower than optimized (non-constant-time) versions.
**Real wall?** No. But constant-time is harder to write and maintain than branchy code. Compilers can introduce secret-dependent branches (e.g., from strcmp).
**Cross-domain wiring:** Constant-time code = timing side-channel free. In retrieval: ensuring retrieval latency does not leak query intent.
**Notes:** Bitslicing (Biham, 1997) computes multiple instances of a block cipher in parallel using only AND/OR/XOR — naturally constant-time.

### phys unclonable func (cross-domain alias: `PUF`, `hardware-fingerprint`, `silicon-biometrics`)
**Domain:** Cryptography / Hashing
**Definition:** Extract a unique "fingerprint" from manufacturing variations in silicon. Challenge-response: apply challenge C → measure response R (delay-based or memory-based). The response is unique per chip.
**Atom or composite:** Composite: apply challenge → measure response (timing or power) → hash to stable output.
**Cost model:** PUF evaluation: very fast (one-time challenge-response). But PUF responses are noisy (silicon variation is not perfectly stable).
**Real wall?** Yes — environmental factors (temperature, voltage) change PUF responses. Error correction (Fuzzy Extractor) is needed to convert noisy PUF output to a stable key.
**Cross-domain wiring:** PUF = hardware-unique identifier = the same as a biometric (iris scan, fingerprint) for silicon. In retrieval: PUF = content-addressable storage for hardware.
**Notes:** ARB-PUFs (Arbitration PUFs) use race conditions in ring oscillators. SRAM-PUFs use the startup values of SRAM cells.

### key-wrap-hw (cross-domain alias: `key-encryption`, `envelope-encryption`, `hardware-key-store`)
**Domain:** Cryptography / Hashing
**Definition:** Protect cryptographic keys in hardware: (1) encrypt the key with a hardware-embedded KEK (Key Encryption Key); (2) store the encrypted key in non-volatile memory; (3) derive the KEK from a PUF or master key in secure hardware.
**Atom or composite:** Composite: derive KEK → encrypt key with KEK → store encrypted blob + metadata → on use: decrypt → load into hardware.
**Cost model:** AES-GCM key wrap: one AES-GCM operation. The hardware key store must be tamper-evident.
**Real wall?** Yes — key wrapping is only as strong as the hardware security (secure enclave, TPM). If the hardware is physically compromised, keys can be extracted.
**Cross-domain wiring:** Key wrapping = encrypting a key with another key = hierarchical key management. In retrieval: encrypting an index with a search key.
**Notes:** TPM 2.0 (Trusted Platform Module) provides hardware key storage with attestation — the hardware can prove its identity to a remote party.

### random-pool-hw (cross-domain alias: `TRNG`, `thermal-noise`, `hardware-RNG`)
**Domain:** Cryptography / Hashing
**Definition:** Generate true random numbers from physical entropy sources (thermal noise, jitter, quantum photon detection). A TRNG uses entropy source → health tests → whitening (Von Neumann or AES-CTR) → output.
**Atom or composite:** Composite: entropy source → health test (check for bias) → whitener (remove bias) → random bits.
**Cost model:** TRNG produces bits at a limited rate (kbps to Mbps). The entropy source is the bottleneck.
**Real wall?** Yes — TRNG can fail (entropy source dies, adversarial tampering). Continuous health tests are mandatory for security certification.
**Cross-domain wiring:** TRNG = true randomness from physical process = same as hardware random sampling. In retrieval: TRNG = random seed for hashing.
**Notes:** Intel RDSEED/RDRAND: the CPU uses thermal noise and oscillator jitter as entropy sources. RDRAND has been controversial — no verified backdoor found, but the design is proprietary.

### crypto-agility (cross-domain alias: `post-quantum-migration`, `algorithm-flexibility`, `hybrid-crypto`)
**Domain:** Cryptography / Hashing
**Definition:** Design cryptographic systems to be algorithm-agnostic so that algorithms can be replaced as needed. Hybrid: combine classical + post-quantum (e.g., X25519 + Kyber) for immediate post-quantum readiness.
**Atom or composite:** Composite: abstract cryptographic operations behind an interface → swap implementations → key wrapping with both classical and PQC keys.
**Cost model:** Algorithm abstraction adds indirection overhead. Hybrid schemes add ~1KB per handshake (Kyber-768 ≈ 2.4KB public key).
**Real wall?** Yes — post-quantum migration is urgent because "harvest now, decrypt later" attacks collect ciphertexts today for future decryption.
**Cross-domain wiring:** Crypto agility = polymorphic security. In retrieval: algorithm agility = swapping retrieval models without breaking the system.
**Notes:** NIST PQC standardization (2024): ML-KEM (Kyber) for KEM, ML-DSA (Dilithium) for signatures. Both are lattice-based.

*Last updated: 2026-06-25 (expanded with side-channel and hardware security)*
*Source doctrine: The Painted Fence — Jesse*
