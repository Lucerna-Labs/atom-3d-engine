# Information Theory & Coding — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## Entropy / Divergence Atoms

### entropy (cross-domain alias: `surprise`, `information-content`, `Shannon-entropy`)
**Domain:** Information Theory & Coding
**Definition:** H(X) = −Σ p(x)·log₂ p(x). Measures the expected information content of a random variable — the average number of bits needed to describe an outcome.
**Atom or composite:** Composite: fold over distribution: H = fold(−p·log₂(p)).
**Cost model:** One pass over the distribution. Requires estimating p(x) from data.
**Real wall?** No.
**Cross-domain wiring:** In physics: thermodynamic entropy ≈ information entropy (Boltzmann entropy S = k·log W). In retrieval: entropy of term distribution measures the "spread" of a term across documents — high entropy = ubiquitous, low entropy = specific.
**Notes:** Maximum entropy principle: the distribution with maximum entropy for given constraints is the least informative (most conservative) assumption.

### mutual-information (cross-domain alias: `shared-information`, `dependency-measure`, `I(X;Y)`)
**Domain:** Information Theory & Coding
**Definition:** I(X;Y) = H(X) − H(X|Y) = H(Y) − H(Y|X). Measures how much knowing X reduces uncertainty about Y (and vice versa). Non-negative, symmetric.
**Atom or composite:** Composite: I(X;Y) = fold(p(x,y)·log(p(x,y)/(p(x)·p(y)))).
**Cost model:** Requires estimating the joint distribution p(x,y) — needs sufficient samples. Computed from joint histogram.
**Real wall?** No.
**Cross-domain wiring:** In retrieval: shared information between query and document = relevance signal. In signal processing: mutual information between transmitted and received signal = channel capacity component. In physics: mutual information measures correlations in statistical mechanics.
**Notes:** I(X;Y) = 0 iff X and Y are independent. This makes it a perfect test for statistical dependency.

### pmi (cross-domain alias: `association`, `pointwise-dependency`, `log-odds-cooccurrence`)
**Domain:** Information Theory & Coding
**Definition:** PMI(x,y) = log₂ P(x,y)/(P(x)·P(y)). The pointwise version of mutual information — measures association for a specific pair of events.
**Atom or composite:** Composite: fold(cooccurrence) + fold(marginals) → combine(log₂-ratio).
**Cost model:** Requires co-occurrence estimation. Can be computed from a co-occurrence matrix.
**Real wall?** Yes — PMI has a rare-term bias. Two words that co-occur once and each appear once have infinite PMI. PPMI (positive PMI only) or NPMI (normalized PMI) fixes this.
**Cross-domain wiring:** PMI ≈ log Bayes factor for co-occurrence. In retrieval: PMI(x,y) > 0 means x and y co-occur more than expected by chance. In physics: co-occurrence entropy = same information-theoretic measure.
**Notes:** NPMI = PMI / H(X,Y) ∈ [−1, 1]. This normalization removes the scale-dependence and makes NPMIs comparable across pairs with different frequencies.

### kl-divergence (cross-domain alias: `relative-entropy`, `information-gain`, `divergence`)
**Domain:** Information Theory & Coding
**Definition:** D(P||Q) = Σ P(x)·log(P(x)/Q(x)). Measures the extra bits needed to encode samples from P using a code optimized for Q. Asymmetric: D(P||Q) ≠ D(Q||P).
**Atom or composite:** Composite: fold over P: D += P(x)·log(P(x)/Q(x)).
**Cost model:** One pass over P. Requires Q(x) > 0 wherever P(x) > 0 (support condition).
**Real wall?** Yes — D(P||Q) = ∞ if Q(x) = 0 where P(x) > 0. This is the "zero-probability" wall: using the wrong reference distribution makes the divergence infinite.
**Cross-domain wiring:** In ML: KL divergence is the basis of variational inference. In retrieval: cross-entropy loss = D(P_true||P_pred). In physics: free energy = k·T·D(posterior||prior).
**Notes:** The asymmetric property is critical: D(P_model||P_data) is the forward KL (zero-forcing — it avoids Q having mass where P has none). D(P_data||P_model) is the reverse KL (mass-covering — it avoids P having mass where Q has none).

---

## Compression Atoms

### compress (cross-domain alias: `encode`, `entropy-code`, `source-code`)
**Domain:** Information Theory & Coding
**Definition:** Map source symbols to codewords with average length ≥ H(source). Lossless compression: Huffman coding (fixed probabilities), arithmetic coding (near-optimal,接近熵极限).
**Atom or composite:** Composite: build frequency model → assign codewords → scan and encode. Arithmetic: build interval model → map symbol to subinterval → emit bits.
**Cost model:** Encoding is cheap (table lookup). Building the model (for adaptive coding) requires online updates.
**Real wall?** Yes — the entropy H is the lower bound. No lossless compressor can beat H bits on average. This is Shannon's source coding theorem — a real wall.
**Cross-domain wiring:** Compression = entropy reduction = removing redundancy. In retrieval: IDF weighting = compressing the document representation by removing common-term information. In signal: pre-whitening removes redundancy before encoding.
**Notes:** Arithmetic coding achieves length ≈ H(P) bits for the entire sequence — better than Huffman, which wastes bits on per-symbol rounding.

### decompress (cross-domain alias: `decode`, `reconstruct`, `invert`)
**Domain:** Information Theory & Coding
**Definition:** Invert the compression: given the compressed bitstream and the same model, recover the original sequence.
**Atom or composite:** The decompressor is the inverse of the compressor — same model, reversed.
**Cost model:** Same as compression for arithmetic coding (one pass). Huffman decode is a tree traversal.
**Real wall?** No.
**Cross-domain wiring:** In retrieval: decompressing a retrieved document = rendering the token pack back into full text. In graphics: decoding a texture format = decompress + render.
**Notes:** The compressor and decompressor must share the same model — either a fixed model or transmitted as side information.

### delta-compress (cross-domain alias: `differential-encode`, `residual-code`, `predict-then-encode`)
**Domain:** Information Theory & Coding
**Definition:** Encode the difference from a prediction rather than the raw value. Prediction can be from neighboring values (in text: previous word), from a model, or from the previous state.
**Atom or composite:** Composite: predict(current) → compute residual(current − predict) → encode(residual).
**Cost model:** One extra prediction computation + encode. But residuals are often smaller → fewer bits.
**Real wall?** No.
**Cross-domain wiring:** Delta encoding in version control (git) = delta compress. In video: motion compensation = delta compress on image patches. In retrieval: residual quantization = delta compress on vector residuals after coarse quantization.
**Notes:** The key to delta compression is the quality of the predictor. A good predictor → small residuals → high compression. A bad predictor → residuals as large as the original.

### run-length-encode (cross-domain alias: `RLE`, `repeat-compress`, `count-encode`)
**Domain:** Information Theory & Coding
**Definition:** Compress consecutive identical symbols: "AAAAAABBBCC" → [(A,6),(B,3),(C,2)]. Works well for sparse data with long runs.
**Atom or composite:** Composite: scan → detect runs → fold(count) → emit (symbol, count).
**Cost model:** One pass. Very cheap.
**Real wall?** No.
**Cross-domain wiring:** RLE on binary images = the scanline representation of pixel runs. In signal: RLE on sparse signals = run-length encoding of sparse coefficient sequences. In retrieval: run-length encoding of posting lists (doc IDs in order).
**Notes:** Fax compression (Group 3/4) is RLE + Huffman. PDF uses RLE as one of several predictors. RLE is the simplest compression and works surprisingly well for structured sparse data.

---

## Channel Coding Atoms

### encode-block (cross-domain alias: `error-control-encode`, `redundancy-add`)
**Domain:** Information Theory & Coding
**Definition:** Map k message bits to n code bits (n ≥ k). Types: Hamming (detect 2 errors, correct 1), Reed-Solomon (correct burst errors), BCH (generalization of Hamming).
**Atom or composite:** Composite: for each message block: compute parity bits → append to message.
**Cost model:** Polynomial division for Reed-Solomon/BCH (requires Galois field arithmetic). Hamming is a simple parity check matrix multiplication.
**Real wall?** Yes — no code can exceed Shannon capacity. The gap from capacity to the code's actual performance is measured by the code's rate vs SNR.
**Cross-domain wiring:** Error-correcting codes = structured redundancy = same as adding redundant measurements in compressed sensing. In retrieval: error-correcting codes ≈ structured redundancy for robust retrieval.
**Notes:** Reed-Solomon codes are maximum distance separable (MDS) — they achieve the Singleton bound. This means they can correct up to (n−k)/2 symbols.

### decode-block (cross-domain alias: `error-correct`, `syndrome-decode`, `Berlekamp-Massey`)
**Domain:** Information Theory & Coding
**Definition:** Given a possibly corrupted codeword, recover the original message. Syndrome decoding: compute syndrome = H·r (where H is parity check matrix) → look up error pattern in syndrome table.
**Atom or composite:** The decoder is the generator. Syndrome computation is cheap; syndrome table lookup (or algebraic decoding) is the expensive part.
**Cost model:** Varies enormously: Hamming decode = one XOR + table lookup. Viterbi (convolutional) = O(n·states) per block. Belief propagation on LDPC = iterative.
**Real wall?** Yes — decoding general linear codes is NP-hard. The specific structure of each code family enables efficient decoding.
**Cross-domain wiring:** Viterbi = dynamic programming on a trellis = forward-backward algorithm in HMMs. Belief propagation on factor graphs = sum-product algorithm = same as loopy belief propagation.
**Notes:** LDPC (Low-Density Parity-Check) codes use iterative belief propagation on a sparse bipartite graph. They achieve near Shannon-limit performance at long block lengths.

### turbo-encode (cross-domain alias: `parallel-concatenate`, `iterative-encode`)
**Domain:** Information Theory & Coding
**Definition:** Parallel concatenation of two convolutional codes via an interleaver. The interleaver scrambles the input to the second encoder, decorrelating the parity streams.
**Atom or composite:** Composite: encode message with ConvEncoder1 → interleave(message) → encode with ConvEncoder2 → multiplex parities.
**Cost model:** Two convolutional encodings + interleaving. The interleaver is a permutation — O(k) for block of size k.
**Real wall?** Yes — the interleaver is typically pseudo-random, which makes the code's performance analysis approximate. The convergence of the iterative decoder is not guaranteed for all interleaver designs.
**Cross-domain wiring:** Turbo codes ≈ neural belief propagation with two message-passing networks and an interleaver. In retrieval: ensemble of retrievers with an interleaver = turbo-like diversity.

### fountain-encode (cross-domain alias: `rateless-encode`, `LT-code`, `raptor-code`)
**Domain:** Information Theory & Coding
**Definition:** Generate an unbounded stream of encoding symbols from k source symbols. Each encoding symbol = XOR of a random subset of source symbols (the "degree" determines subset size). Any k·(1+ε) encoding symbols suffice to decode.
**Atom or composite:** Composite: choose degree d from degree distribution → select d source symbols uniformly at random → XOR → emit.
**Cost model:** One XOR per source symbol in the encoding. Decoding cost depends on degree distribution.
**Real wall?** Yes — the degree distribution is critical. Too many low-degree symbols = linear dependence (not enough info). Too many high-degree symbols = expensive decoding. The Soliton distribution and its robust variant are the solutions.
**Cross-domain wiring:** Fountain codes = broadcasting with erasures = any-to-any reliable multicast. In streaming: the encoding symbols are the "droplets" — enough must be received before decoding is possible. In retrieval: any subset of candidates that covers all query terms = fountain-like coverage.
**Notes:** Raptor codes add a pre-code (LDPC) before the LT code, which makes decoding simpler and more robust to degree distribution variance.

---

## Rate-Distortion Atoms

### quantize-info (cross-domain alias: `rate-quantize`, `compress-scalar`, `scalar-quantize`)
**Domain:** Information Theory & Coding
**Definition:** Map a continuous source value to one of M reconstruction levels. Lloyd-Max quantization finds the optimal decision boundaries and reconstruction levels for a given source distribution.
**Atom or composite:** Composite: compare(x to thresholds) → assign to nearest level → emit reconstruction level.
**Cost model:** One comparison per level (binary search for M levels → O(log M)). Lloyd-Max requires an iterative optimization pass.
**Real wall?** Yes — the rate-distortion function R(D) gives the minimum bitrate achievable at distortion D. The quantization step size Δ = 2^(−R) for uniform quantizers. The bitrate and distortion are coupled — you can't independently minimize both.
**Cross-domain wiring:** In ML: k-means clustering = vector Lloyd-Max quantization. In image compression: JPEG's DCT quantization = perceptual weighting of the quantization step sizes. In retrieval: discretizing continuous scores to bins = scalar quantization.
**Notes:** Lloyd-Max quantization achieves the rate-distortion bound for a given source distribution. For uniform sources and MSE distortion, the optimal quantizer is uniform with step size Δ.

### rate-distort (cross-domain alias: `RD-optimize`, `trade-off-compute`, `optimal-compression`)
**Domain:** Information Theory & Coding
**Definition:** Given a bit budget, find the encoding that minimizes distortion. The rate-distortion function R(D) = minimum achievable rate at distortion ≤ D.
**Atom or composite:** Composite: for each candidate encoding: compute rate and distortion → find Pareto frontier.
**Cost model:** Expensive — requires evaluating many encoding options. In practice, Lagrangian relaxation: minimize L = D + λ·R.
**Real wall?** Yes — the rate-distortion curve is the real wall. You can trade rate for distortion but not escape the fundamental bound.
**Cross-domain wiring:** In compression: the RD curve is the efficiency frontier. In retrieval: the recall/latency trade-off is analogous — the Pareto frontier of the retrieval system. In graphics: quality/performance trade-off in rendering = similar Pareto structure.
**Notes:** The Shannon lower bound (SLB) gives a theoretical limit on R(D) for arbitrary sources. Getting close to the SLB requires sophisticated coding (DPC, multi-resolution).

### bits-back-encode (cross-domain alias: `information-theoretic-compress`, `entropy-code-with-side-info`)
**Domain:** Information Theory & Coding
**Definition:** Use the mutual information between source and side information to reduce the bitrate below the unconditional entropy. The receiver uses the side information to decode.
**Atom or composite:** Composite: sender: encode message using side information (as shared randomness). Receiver: decode using shared side information.
**Cost model:** The encoding rate = H(X) − I(X;Y) where Y is the side information. Achievable via Slepian-Wolf coding.
**Real wall?** No.
**Cross-domain wiring:** In retrieval: bits-back encoding = using the corpus as side information to compress the query representation. This is the foundation of learned compression and the information-theoretic view of retrieval.
**Notes:** This is the most intellectually powerful primitive in the kit — it shows that shared context (side information) reduces the bits needed to communicate. This is exactly what IDF weighting achieves: it reduces the bits needed to communicate which documents are relevant.

---

## Loss / Metric Atoms

### cross-entropy (cross-domain alias: `log-loss`, `log-score`, `logarithmic-loss`)
**Domain:** Information Theory & Coding
**Definition:** H(P_true, P_pred) = −Σ P_true(x)·log P_pred(x). The average number of bits needed to encode the true distribution using the predicted distribution's code.
**Atom or composite:** Composite: for each outcome: CE += P_true(x)·log(P_pred(x)).
**Cost model:** One pass over the distribution.
**Real wall?** No. But cross-entropy is unbounded — predictions very close to 0 for true events produce enormous losses.
**Cross-domain wiring:** Cross-entropy loss in neural networks = D(P_true||P_pred). In retrieval: NDCG loss = information-theoretic ranking loss. In language modeling: perplexity = 2^H(per-token).
**Notes:** Cross-entropy = H(P_true) + D(P_true||P_pred). The H(P_true) term is constant; the training gradient comes from the D(P_true||P_pred) term.

### perplexity (cross-domain alias: `average-uncertainty`, `effective-vocabulary-size`)
**Domain:** Information Theory & Coding
**Definition:** Perplexity = 2^{H(P_pred)} = (Π P(x_i))^{−1/N}. The effective size of the distribution — a model that perfectly predicts has perplexity = 1 (zero surprise).
**Atom or composite:** Composite: compute per-token cross-entropy → exponentiate.
**Cost model:** Same as cross-entropy.
**Real wall?** No.
**Cross-domain wiring:** Perplexity in language models = average branching factor. In retrieval: perplexity of the relevance distribution = how surprised the retriever is by each document.
**Notes:** Perplexity is the geometric mean of the inverse probabilities. A language model with perplexity 100 means it has the same uncertainty as a uniform distribution over 100 words.

### aic (cross-domain alias: `model-select`, `penalized-likelihood`)
**Domain:** Information Theory & Coding
**Definition:** AIC = 2·k − 2·log(L). Balances model fit (log-likelihood) against complexity (number of parameters k). Select the model with minimum AIC.
**Atom or composite:** Composite: fit model → compute log-likelihood → penalize by parameter count.
**Cost model:** One model fit + likelihood evaluation.
**Real wall?** No. But AIC is asymptotic (large N) — for small N, AICc (corrected) is better. BIC uses log(N) instead of 2 as the penalty, which is stricter.
**Cross-domain wiring:** In retrieval: model selection for LTR = similar penalized likelihood. In physics: maximum entropy model selection = same principle.
**Notes:** AIC is an approximation to out-of-sample prediction error. The 2k penalty means each extra parameter costs 2 bits of the effective likelihood budget.

---

## Summary: Information Theory Atom → Cross-Domain Wiring

| Info Theory Primitive | Retrieval Alias | Physics Alias | Linear Algebra Alias |
|---|---|---|---|
| entropy | term specificity | thermodynamic entropy | trace of covariance |
| mutual-information | query-doc shared info | correlation measure | dot product of distributions |
| pmi | synonym discovery | co-occurrence association | log-ratio |
| kl-divergence | cross-entropy loss | relative free energy | information projection |
| compress | IDF compression | entropy reduction | low-rank approximation |
| delta-compress | residual scoring | differential encoding | residual computation |
| fountain-encode | candidate coverage | reliable broadcast | rateless sampling |
| rate-distort | recall-latency frontier | efficiency frontier | approximation quality |
| bits-back | compressed query | side-information coding | leveraging shared basis |
| cross-entropy | NDCG, log-loss | Gibbs free energy | KL projection |
| perplexity | ranking uncertainty | ensemble entropy | effective rank |
| aic | hyperparameter model selection | Occam factor | complexity penalty |

---

*Last updated: 2026-06-21*
*Source doctrine: The Painted Fence — Jesse*


---

## Advanced Compression Atoms

### LZ77-compress (cross-domain alias: `sliding-window-compress`, `back-references`, `deflate`)
**Domain:** Information Theory & Coding
**Definition:** Replace repeated sequences with references to previous occurrences: (offset, length, next_char). The window is scanned backward to find the longest match.
**Atom or composite:** Composite: scan input → find longest match in window → emit (offset, length) or raw byte → advance.
**Cost model:** O(N·W) where W is the window size. Fast implementations use hash chains or suffix trees to accelerate match finding.
**Real wall?** No. But the match search dominates the runtime. Zstd uses dictionary compression + LZ77 and is one of the fastest compressors available.
**Cross-domain wiring:** LZ77 = the same as the "predict then encode" principle. The prediction is "the next character is the same as this one we saw earlier." In retrieval: predictive coding = document compression via self-references.
**Notes:** DEFLATE (zlib, gzip) combines LZ77 with Huffman coding. The LZ77 part handles repeated sequences; the Huffman part handles symbol frequency.

### LZW-compress (cross-domain alias: `dictionary-compress`, `adaptive-lexicon`, `gif-compress`)
**Domain:** Information Theory & Coding
**Definition:** Build a dictionary of substrings dynamically. Output indices into the dictionary rather than raw bytes. Starting with a dictionary of single characters, each new substring is added.
**Atom or composite:** Composite: scan input → if substring in dictionary: output index; else: output prefix, add new string = prefix + next_char → repeat.
**Cost model:** O(N) but dictionary grows with the input. Memory for dictionary can be large.
**Real wall?** Yes — dictionary size grows exponentially with string variety. For diverse data, the dictionary becomes too large to be useful.
**Cross-domain wiring:** LZW = the same as building a vocabulary of n-grams. In retrieval: building a vocabulary of document n-grams = dictionary compression of the document corpus.
**Notes:** LZW was used in GIF and early Unix compress. It is the basis of the LZW algorithm — patents were a major issue (GIF patent disputes).

### ANS-encode (cross-domain alias: `Asymmetric-Numeral-Systems`, `range-ans`, `tabled-compress`)
**Domain:** Information Theory & Coding
**Definition:** rANS: interleave information across multiple bits. The state is an integer; encoding maps (symbol, state) to a new state. Produces bits that are nearly optimal (close to entropy).
**Atom or composite:** Composite: initialize state → for each symbol: update state via ANS transition table → output bits.
**Cost model:** Fast — rANS can be implemented with table lookups and bit shifts. LZ4 uses rANS for entropy coding.
**Real wall?** No.
**Cross-domain wiring:** ANS = arithmetic coding with a different state space (integer vs real interval). In ML: ANS is the entropy coding inside neural compression models (e.g., VAE with ANS decoder).
**Notes:** tANS (table ANS) precomputes the ANS transition table for fast implementation. rANS (range ANS) is the streaming version. ANS achieves ~0.03 bits/byte from entropy.

### context-model (cross-domain alias: `PPM`, `prediction-by-partial-match`, `adaptive-order`)
**Domain:** Information Theory & Coding
**Definition:** Predict the next symbol using the context of the preceding symbols. PPM: use the longest matching context of order k; if no match, fall back to order k−1, and so on.
**Atom or composite:** Composite: for each order from k to 0: look up context in PPM tree → if found: emit prediction; else: fall back to lower order.
**Cost model:** Context tree grows exponentially with order. Large contexts capture long-range dependencies but are memory-intensive.
**Real wall?** Yes — the context tree grows exponentially. PPM with large order k is powerful but memory-intensive. Order 16 can handle meaningful long-range dependencies.
**Cross-domain wiring:** PPM = n-gram language model with backoff. In retrieval: context-based retrieval = predicting relevance from query context.
**Notes:** PPM-C (PPM with exclusion) excludes the current context's symbol when computing lower-order predictions. PPMd (PPM in the 7z archiver) is a widely used implementation.

---

## Advanced Error-Correcting Codes Atoms

### LDPC-decode (cross-domain alias: `belief-propagation`, `iterative-decode`, `Tanner-graph`)
**Domain:** Information Theory & Coding
**Definition:** LDPC codes use a sparse bipartite graph (Tanner graph) between variable nodes and check nodes. Belief propagation iteratively passes messages: variable → check = probability; check → variable = parity constraint satisfaction.
**Atom or composite:** Composite: initialize variable nodes → repeat: check → variable messages → variable → check messages → until convergence or max iterations.
**Cost model:** O(n·iterations) where n = block length. Iterations typically 20-100.
**Real wall?** Yes — belief propagation on loopy graphs can fail to converge. The girth (short cycles in the Tanner graph) degrades performance. Irregular LDPC codes (varying node degrees) mitigate this.
**Cross-domain wiring:** LDPC BP = message passing on factor graphs = sum-product algorithm = loopy belief propagation. In ML: BP unrolled networks = unrolled LDPC decoding.
**Notes:** The density evolution analysis tracks the message distribution as iterations increase. It predicts the threshold SNR below which decoding fails.

### polar-decode (cross-domain alias: `successive-cancellation`, `SC-list`, `channel-polarization`)
**Domain:** Information Theory & Coding
**Definition:** Polarization: after n recursive channel combinations, the channels become either nearly noiseless (frozen) or nearly noisiable. Encoding: XOR with the polarization transform. Decoding: successive cancellation — decode bit by bit.
**Atom or composite:** Composite: polar transform (butterfly structure) for encoding → SC decoding: for each bit: if frozen, decode 0; if not frozen, compute likelihood ratio and decide.
**Cost model:** O(N log N) for encoding. Decoding: O(N log N) with SC, or O(N log N) with SCL (successive cancellation list).
**Real wall?** Yes — SC decoding is sequential (can't parallelize across bits). SCL (list decoding) adds parallelism but increases complexity.
**Cross-domain wiring:** Polar codes = the first explicit construction that achieves channel capacity. The polarization is similar to the recursive partitioning of the space in hierarchical clustering.
**Notes:** 5G NR uses polar codes for control channels. The frozen set (which bits are frozen to 0) is optimized for the specific channel SNR.

### syndrome-decode (cross-domain alias: `invert-parity`, `syndrome-table`, `minimum-weight`)
**Domain:** Information Theory & Coding
**Definition:** For linear codes: syndrome = H·rᵀ where H is the parity check matrix and r is the received word. The syndrome maps to an error pattern via a lookup table.
**Atom or composite:** Composite: compute syndrome → look up error pattern → correct r = r + error.
**Cost model:** Syndrome computation: O(n) for n-bit word. Table lookup: O(1). But the syndrome table size is 2^{n−k} — impractical for large codes.
**Real wall?** Yes — syndrome table size grows exponentially with the number of parity bits. For large block codes, syndrome decoding requires algebraic methods (e.g., Berlekamp-Massey for BCH/RS).
**Cross-domain wiring:** Syndrome = the "error fingerprint" = a compact description of what went wrong. In databases: syndrome = the "error signature" from a hash check.
**Notes:** Berlekamp-Massey (BM) algorithm finds the shortest feedback polynomial that explains the syndrome — it is the key to decoding BCH and Reed-Solomon codes.

### trellis-decoding (cross-domain alias: `Viterbi-decode`, `BCJR`, `MAP-decode`)
**Domain:** Information Theory & Coding
**Definition:** Represent the code as a trellis (states over time). Viterbi: find the most likely path through the trellis — maximum a posteriori (MAP). BCJR: compute the a posteriori probability of each bit — soft output.
**Atom or composite:** Composite: construct trellis → Viterbi: forward-backward pass → select best path. BCJR: compute forward/backward messages → compute per-bit a posteriori probabilities.
**Cost model:** Viterbi: O(S·N) where S = states, N = length. BCJR: same complexity but produces soft outputs.
**Real wall?** No.
**Cross-domain wiring:** Viterbi = dynamic programming on a trellis = forward-backward algorithm for HMMs. BCJR = sum-product algorithm on the trellis = the forward-backward algorithm for belief propagation.
**Notes:** BCJR (Bahl-Cocke-Jelinek-Raviv) is the MAP decoder that gives the a posteriori probability of each bit — essential for turbo code decoding and iterative systems.

---

## Network Information Theory Atoms

### capacity-estimate (cross-domain alias: `Shannon-limit`, `channel-capacity`, `max-flow`)
**Domain:** Information Theory & Coding
**Definition:** Channel capacity C = max_{p(x)} I(X;Y). The maximum mutual information between input X and output Y over the channel. The theoretical upper bound on reliable transmission rate.
**Atom or composite:** Composite: search over input distributions → compute I(X;Y) → find maximum. For many channels, this has a closed form.
**Cost model:** Optimization over distributions — can be expensive for continuous channels.
**Real wall?** Yes — capacity is the real wall. No code can exceed capacity. Getting within 0.1 dB of capacity requires sophisticated coding (LDPC, polar, turbo).
**Cross-domain wiring:** Capacity = maximum flow = min-cut. In retrieval: retrieval capacity = the maximum number of distinguishable documents in the embedding space.
**Notes:** The Gaussian channel: C = B·log₂(1 + P/N₀B) bits/s. As B → ∞, C → P/(N₀·ln 2). This is the fundamental limit of broadband communication.

### network-coding (cross-domain alias: `random-linear-network`, `interflow-coding`, `butterfly-network`)
**Domain:** Information Theory & Coding
**Definition:** At each node in a network, combine incoming packets via linear combinations (over a finite field). This enables throughput beyond the cut-set bound that store-and-forward routing cannot achieve.
**Atom or composite:** Composite: at each node: receive packets → compute linear combination over GF(2ⁿ) → forward.
**Cost model:** Linear combination over GF(2⁸) is cheap (byte-level XOR). The key cost is decoding at the sink.
**Real wall?** Yes — the finite field size matters: GF(2⁸) avoids symbol errors from field overflow but costs more than GF(2). Randomized network coding works with high probability.
**Cross-domain wiring:** Network coding = mixing signals at intermediate nodes. In physics: network coding = Feynman diagram vertex interactions.
**Notes:** The butterfly network (Ahlswede et al., 2000) shows that network coding achieves the min-cut capacity while routing cannot.

### fountain-capacity (cross-domain alias: `erasure-channel`, `rateless-capacity`, `feedbackless-coding`)
**Domain:** Information Theory & Coding
**Definition:** For an erasure channel with erasure probability ε: capacity = 1 − ε. A fountain code achieves this by generating an unbounded stream of encoding symbols, and the receiver decodes after collecting (1+δ)·k symbols.
**Atom or composite:** Composite: source: generate encoding symbols → transmit. Sink: collect symbols until rank = k → decode.
**Cost model:** The encoder is cheap (rateless). The decoder is the cost.
**Real wall?** Yes — the decoding overhead δ must be small. LT codes with near-optimal degree distributions achieve δ ≈ 0.
**Cross-domain wiring:** Fountain codes = any-to-any reliable delivery = the same as any-set-of-candidates that covers the query terms.
**Notes:** The Soliton distribution (ideal degree distribution) has theoretical optimality but practical instability. The robust Soliton distribution (Luby) adds a small amount of low-degree redundancy to ensure recovery.

---

## Information-Theoretic Learning Atoms

### information-bottleneck (cross-domain alias: `IB-principle`, `compression-relevance`, `relevant-compression`)
**Domain:** Information Theory & Coding
**Definition:** Minimize I(X;Z) subject to I(Y;Z) ≥ I_target, where Z is the compressed representation of X. Lagrangian: L = β·I(X;Z) − I(Y;Z). Finds the optimal compression that preserves label-relevant information.
**Atom or composite:** Composite: optimize over the encoder p(z|x) → find bottleneck variables that minimize mutual information with X while preserving mutual information with Y.
**Cost model:** Estimating mutual information in high dimensions is expensive — requires density estimation.
**Real wall?** Yes — I(X;Z) and I(Y;Z) are hard to estimate in high dimensions. Variational bounds are used in practice.
**Cross-domain wiring:** IB = rate-distortion theory applied to learning. In retrieval: the bottleneck is the embedding space — compress document to embedding while preserving relevance.
**Notes:** Deep IB (Tishby & Zaslavsky, 2015) proposes that DNNs optimize an information bottleneck — training proceeds from memorization (high I(X;Z)) to generalization (low I(X;Z), high I(Y;Z)).

### rate-distortion-optimal (cross-domain alias: `blending-function`, `optimal-encoding`, `RD-curve`)
**Domain:** Information Theory & Coding
**Definition:** For a given distortion level D, find the minimum rate R(D) needed. The RD function is the fundamental trade-off: lower rate → higher distortion. Optimal encoding achieves R(D).
**Atom or composite:** Composite: for each target distortion D: find encoding that minimizes rate at ≤ D distortion.
**Cost model:** Expensive — requires optimization over the encoding distribution. In practice, use Lagrangian relaxation: minimize D + λ·R.
**Real wall?** Yes — the RD function is the theoretical minimum rate for a given distortion. No encoder can beat it.
**Cross-domain wiring:** RD curve = the Pareto frontier of quality vs bitrate. In retrieval: the RD curve = the recall vs latency trade-off.
**Notes:** The Blahut-Arimoto algorithm computes R(D) and D(R) iteratively for discrete sources. It is the information-theoretic foundation of optimal quantization.

### perplexity-compute (cross-domain alias: `effective-vocab`, `cross-entropy-exp`, `language-entropy`)
**Domain:** Information Theory & Coding
**Definition:** Perplexity = 2^{H} where H is the cross-entropy of the language model. The effective vocabulary size — a uniform distribution over PP words has the same entropy.
**Atom or composite:** Composite: H = (1/T)·Σ_t log₂ P(w_t | w_{<t}) → PP = 2^H.
**Cost model:** One forward pass per token to compute log probabilities. Expensive for large models.
**Real wall?** No.
**Cross-domain wiring:** Perplexity = the branching factor. In physics: perplexity = the effective number of states in a thermodynamic system.
**Notes:** A perplexity of 20 means the model is as uncertain as if it were choosing uniformly among 20 words. GPT-3 at its release had a perplexity ~20 on its training data.

---

*Last updated: 2026-06-21 (expanded with advanced compression, advanced ECC, network IT, info-theoretic learning)*
*Source doctrine: The Painted Fence — Jesse*


---

## Hypothesis Testing & Statistical Decision Atoms

### Neyman-Pearson (cross-domain alias: `NP-test`, `likelihood-ratio-test`, `optimal-detector`)
**Domain:** Information Theory & Coding
**Definition:** Given two distributions P₀ and P₁, the likelihood ratio test Λ(x) = P₁(x)/P₀(x) is the most powerful test at a given false alarm rate. Threshold τ: accept P₁ if Λ(x) > τ.
**Atom or composite:** Composite: compute likelihood ratio Λ(x) → compare to threshold τ → decide H₀ or H₁.
**Cost model:** One ratio computation per test. The threshold τ is set to achieve the desired false alarm rate.
**Real wall?** Yes — the Neyman-Pearson lemma gives the optimal test, but it requires knowing P₀ and P₁ exactly. In practice, these are estimated from data.
**Cross-domain wiring:** Neyman-Pearson = optimal binary decision = the matched filter. In signal: the detector that minimizes false-alarm for a given detection probability. In retrieval: the decision boundary between relevant and non-relevant.
**Notes:** The ROC curve traces the trade-off between probability of detection (power) and false alarm rate for all possible thresholds.

### Chernoff-bound (cross-domain alias: `exponential-bound`, `error-exponential`, `Cramér-Chernoff`)
**Domain:** Information Theory & Coding
**Definition:** Bound the probability that a sum of random variables exceeds its mean by δ: P(S_n ≥ nμ + δ) ≤ e^{−n·D(μ+δ||μ)}. The divergence D is the Cramér rate function.
**Atom or composite:** Composite: for the distribution pair: compute Chernoff information C = max_{λ∈(0,1)} −log Σ p^λ q^{1−λ} → bound P(error) ≤ e^{−n·C}.
**Cost model:** One optimization over λ per distribution pair.
**Real wall?** No. But the bound is loose for small n — it's asymptotic.
**Cross-domain wiring:** Chernoff bound = the large deviations rate function = the same as the error exponent in channel coding. In ML: the PAC learning bound uses Chernoff bounds.
**Notes:** The Chernoff bound gives the best exponential bound on the probability of error. The Chernoff information C(P,Q) is the Rényi divergence of order 1/2.

### Fano-inequality (cross-domain alias: `converse-bound`, `minimum-entropy-bound`, `error-entropy`)
**Domain:** Information Theory & Coding
**Definition:** H(U|V) ≤ H(P_e) + P_e·log(|U|−1). Relates the conditional entropy of a guess U given observation V to the probability of error P_e. Used to prove converses (impossibility results) for coding theorems.
**Atom or composite:** Composite: compute conditional entropy H(U|V) → relate to error probability P_e → compute minimum achievable rate.
**Cost model:** Trivial — Fano is a mathematical identity.
**Real wall?** No. But the bound is tight — achieving equality requires the optimal test.
**Cross-domain wiring:** Fano = the converse of the channel capacity theorem = the minimum description length bound. In retrieval: Fano bounds the probability of retrieval error given the mutual information between query and document.
**Notes:** Fano's inequality is the standard tool for proving converses: if you can't distinguish between M messages with error ε, then you need at least log M − H(P_e) bits of information.

### Stein-lemma (cross-domain alias: `error-exponent`, `type-1-plus-type-2`, `asymptotic- Neyman-Pearson`)
**Domain:** Information Theory & Coding
**Definition:** In hypothesis testing between P and Q, the optimal error exponent for type 1 + type 2 errors is D(P||Q) (the KL divergence). The error probability decays as e^{−n·D(P||Q)}.
**Atom or composite:** Composite: compute D(P||Q) → error probability ≈ e^{−n·D(P||Q)}.
**Cost model:** Trivial — the exponent is the KL divergence.
**Real wall?** No.
**Cross-domain wiring:** Stein's lemma = the same as the channel coding error exponent. In ML: Stein's lemma connects the classification error to the KL divergence between classes.
**Notes:** The Chernoff bound is the exponential bound; Stein's lemma gives the exact exponent (the two are equivalent asymptotically).

### Sanov-theorem (cross-domain alias: `type-class-bound`, `large-deviations`, `empirical-distribution`)
**Domain:** Information Theory & Coding
**Definition:** The probability that n i.i.d. samples have empirical distribution Q, while the true distribution is P, is approximately e^{−n·D(Q||P)}. The most likely empirical distribution under the constraint is the information projection of P onto the constraint set.
**Atom or composite:** Composite: for the constraint set: find Q* that minimizes D(Q||P) subject to constraints → Sanov bound: P(T_n ∈ A) ≈ e^{−n·D(Q*||P)}.
**Cost model:** One optimization over Q subject to constraints.
**Real wall?** No.
**Cross-domain wiring:** Sanov = the large deviations principle for empirical distributions. In ML: Sanov governs the probability of overfitting — the deviation from the training distribution.
**Notes:** The type class (the set of sequences with empirical distribution Q) has size ≈ 2^{n·H(Q)}. Sanov tells you which type classes are most likely under P.

### Cramér-Rao (cross-domain alias: `CRB`, `variance-bound`, `efficient-estimator`)
**Domain:** Information Theory & Coding
**Definition:** For any unbiased estimator θ̂ of parameter θ: Var(θ̂) ≥ 1 / (n·I(θ)) where I(θ) is the Fisher information. The CRLB gives the minimum variance achievable.
**Atom or composite:** Composite: compute Fisher information I(θ) = E[(∂/∂θ log p(X;θ))²] → CRB = 1/(n·I(θ)).
**Cost model:** One Fisher information computation.
**Real wall?** Yes — the CRB assumes the model is correct and the estimator is unbiased. Misspecified models break the bound.
**Cross-domain wiring:** Fisher information = the curvature of the log-likelihood = the same as the metric in information geometry. In retrieval: Fisher information measures the discriminative power of a retrieval model.
**Notes:** The efficient estimator (one that achieves the CRB) exists only for exponential family distributions. Most practical estimators are asymptotically efficient.

---

## Multiple Access & Broadcast Channel Atoms

### MAC-capacity (cross-domain alias: `multi-access-channel`, `sum-rate-bound`, `FDMA`)
**Domain:** Information Theory & Coding
**Definition:** For a K-user AWGN MAC: the capacity region is the set of rate tuples (R₁,...,R_K) such that Σ_{i∈S} R_i ≤ C(Σ_{i∈S} P_i / N) for all S ⊆ {1..K}. Each subset of users has a sum-rate constraint.
**Atom or composite:** Composite: for each subset S: compute sum-rate bound C(Σ P_i_S / N) → capacity region = intersection of all such bounds.
**Cost model:** One optimization per subset.
**Real wall?** Yes — the capacity region is achievable only with sophisticated coding (TDMA, CDMA, or superposition coding).
**Cross-domain wiring:** MAC capacity = the convex hull of individual rate constraints. In retrieval: multi-user retrieval capacity = the same achievable rate region.
**Notes:** The sum-rate capacity of the K-user MAC is C(K·P/N) — the same as a single user with K× power. This is the "water-filling in power" result.

### BC-capacity (cross-domain alias: `broadcast-channel`, `dirty-paper`, `superposition-coding`)
**Domain:** Information Theory & Coding
**Definition:** For a degraded Gaussian BC: capacity is achieved by superposition coding (nested codebooks). The stronger user's message is encoded first (as noise for the weaker user); the weaker user's codebook is superimposed.
**Atom or composite:** Composite: encode weak user codebook → treat as noise → encode strong user → superimpose → at receiver: decode strong first (canceling weak) → decode weak.
**Cost model:** Two encoders running simultaneously. The cost is in the encoding, not the decoding.
**Real wall?** Yes — the BC capacity for the general (non-degraded) case is still an open problem in network information theory.
**Cross-domain wiring:** Dirty paper coding (Costa, 1983) shows that if the encoder knows the interference non-causally, it can pre-subtract it — the interference is not a real wall.
**Notes:** The capacity of the Gaussian BC: R₁ ≤ C(P₁/N₀), R₂ ≤ C(P₂/(P₁+N₀)). The two rates are coupled.

### Slepian-Wolf (cross-domain alias: `distributed-compress`, `no- Side-info`, `separate-entropy`)
**Domain:** Information Theory & Coding
**Definition:** Two correlated sources X and Y can be compressed separately (without knowing each other's data) at rates R_X ≥ H(X|Y) and R_Y ≥ H(Y|X), achieving joint decoding at the receiver.
**Atom or composite:** Composite: source A: compress at H(A|B) → source B: compress at H(B|A) → joint decode using syndrome bits.
**Cost model:** Rate savings over separate compression: each source only needs its conditional entropy, not its full entropy.
**Real wall?** No. But Slepian-Wolf requires synchronization and error-free transmission of the compressed streams.
**Cross-domain wiring:** Slepian-Wolf = the compressed sensing of correlated sources = the information-theoretic lower bound on distributed storage. In ML: federated averaging = Slepian-Wolf compression of model updates.
**Notes:** The key result: knowing the correlation structure (X→Y mapping) is enough to compress jointly without communication between encoders.

### Gelfand-Pinsker (cross-domain alias: `writing-on-dirty-paper`, `Gel-Pin`, `known-interference`)
**Domain:** Information Theory & Coding
**Definition:** If the transmitter knows interference I non-causally before encoding, it can achieve the same capacity as if the interference were not present. The dirty paper codebook is structured to be orthogonal to the interference.
**Atom or composite:** Composite: generate random binning codebook → superimpose codeword on known interference → at receiver: cancel interference structure → decode.
**Cost model:** The codebook generation requires knowledge of the interference realization. The encoding is no more expensive than standard coding.
**Real wall?** Yes — the transmitter must know the interference *before* encoding it. For some channels (e.g., fading), this is impossible.
**Cross-domain wiring:** Gelfand-Pinsker = the key insight behind Costa's dirty paper coding, which enables WatermarkGAN and Tomlinson-Harashima precoding in MIMO.
**Notes:** The capacity with known interference is C(P/(P+I)), not C(P/(P+N+I)). The interference power is "free" — it doesn't hurt you if you know it.

---

## Universal Coding Atoms

### universal-compress (cross-domain alias: `KT-estimator`, `Bayesian-mixture`, `offline-compress`)
**Domain:** Information Theory & Coding
**Definition:** A universal code works for any source without knowing its distribution a priori. The Krichevsky-Troffimov estimator achieves redundancy O(log n / n) over all Bernoulli sources.
**Atom or composite:** Composite: for each possible parameter θ: weight by prior p(θ) → predict using mixture Σ p(θ|x_{<n})·P(x_n|θ).
**Cost model:** Mixture over all possible sources — can be expensive for large alphabets. Context tree weighting handles this efficiently.
**Real wall?** No. But universal coding requires the source to be stationary and ergodic — non-ergodic sources may not be compressible.
**Cross-domain wiring:** Universal coding = the same as the minimum description length (MDL) principle. In ML: the Bayesian mixture is the optimal universal learner.
**Notes:** The redundancy R_n = min_{θ} D(P_θ || Q) is the minimax regret. The KT estimator achieves R_n = (|A|−1)/2·log n + O(1) for discrete alphabets.

### Lempel-Ziv (cross-domain alias: `LZ78`, `LZ-complexity`, `incremental-parsing`)
**Domain:** Information Theory & Coding
**Definition:** LZ78 parses the sequence into phrases by building a dictionary of previously seen phrases. Each new phrase is the shortest phrase not in the dictionary, written as a pointer to a previous phrase plus one new symbol.
**Atom or composite:** Composite: scan input → find shortest new phrase → add to dictionary → output (pointer, new_symbol).
**Cost model:** Dictionary lookup per character. The dictionary grows but is bounded by the input length.
**Real wall?** Yes — LZ78 achieves the entropy rate for any stationary ergodic source, but the dictionary can grow to the size of the input.
**Cross-domain wiring:** LZ78 = incremental parsing = building a grammar for the source. In retrieval: LZ78-like parsing = building a phrase vocabulary from the corpus.
**Notes:** LZ78 is the basis of the Unix `compress` command. LZMW (Miller & Wegman, 1984) uses the last match rather than the longest match.

### MDL-principle (cross-domain alias: `minimum-description-length`, `Ockham-encoding`, `model-selection`)
**Domain:** Information Theory & Coding
**Definition:** The best model for data minimizes the total description length: L(model) + L(data|model). MDL = -log P(data|model) + log P(model). Trades off fit vs complexity.
**Atom or composite:** Composite: for each candidate model: compute L(data|model) + L(model) → select minimum total.
**Cost model:** One evaluation per model.
**Real wall?** No. But the description length depends on the encoding scheme — different encodings give different MDL values.
**Cross-domain wiring:** MDL = Occam's razor in information-theoretic form. In statistics: BIC = approximate MDL. In ML: the minimum description length is the same as the minimum message length for communicating the data.
**Notes:** MDL is particularly well-suited for model selection because it automatically penalizes complexity — no separate test set needed.

### Solomonoff-induction (cross-domain alias: `universal-prior`, `Algorithmic-MDL`, `Kolmogorov-complexity`)
**Domain:** Information Theory & Coding
**Definition:** The universal prior assigns probability P(x) = Σ_{U: U outputs x} 2^{−|U|} — the sum over all programs that output x, weighted by program length. This is the optimal predictor for any computable sequence.
**Atom or composite:** Composite: enumerate all programs → sum weights → predict using the mixture.
**Cost model:** Intractable — requires enumerating all programs. The Solomonoff prior is uncomputable.
**Real wall?** Yes — Solomonoff induction is not computable. In practice, we approximate it with bounded-complexity models.
**Cross-domain wiring:** Solomonoff = the universal Bayesian mixture = the theoretical foundation of minimum message length. In retrieval: Solomonoff = the ideal information-theoretic retriever.
**Notes:** Kolmogorov complexity K(x) = the length of the shortest program that outputs x. It is incomputable (no algorithm finds it) but is the ultimate lower bound on compressibility.

---

## Gambling & Portfolio Theory Atoms

### Kelly-criterion (cross-domain alias: `bet-sizing`, `log-optimal`, `growth-rate-maximize`)
**Domain:** Information Theory & Coding
**Definition:** For a gambling game with odds b and probability p of winning: bet fraction f* = (b·p − q) / b = (E[log return]). Maximizes the long-term growth rate of wealth: G = E[log(1 + f·W)].
**Atom or composite:** Composite: for each gamble: compute G(f) = Σ p_i·log(1 + f·b_i) → maximize → bet f*.
**Cost model:** One optimization per gamble. Closed form for binary gambles.
**Real wall?** No. But Kelly assumes you can tolerate the variance — a Kelly bettor experiences large swings. Fractional Kelly (b·f*) is used to reduce variance.
**Cross-domain wiring:** Kelly = information-theoretic portfolio theory = maximizing expected log wealth = maximizing the mutual information between bets and outcomes. In retrieval: Kelly sizing = optimal exploration/exploitation trade-off.
**Notes:** The doubling rate W = E[log(1 + f·W)] is the mutual information between the gambler's inside information and the outcome.

### information-gain (cross-domain alias: `expected-revelation`, `value-of-information`, `Kullback`)
**Domain:** Information Theory & Coding
**Definition:** The value of side information Y for gambling on X: V = max_{gambling policy} E[log(1 + f(X)·W)] − max_{gambling policy without Y} E[log(1 + f(X)·W)]. Information reduces uncertainty, which reduces risk.
**Atom or composite:** Composite: compute Kelly optimal log return without Y → with Y → value = difference.
**Cost model:** Two Kelly optimizations.
**Real wall?** No.
**Cross-domain wiring:** Value of information = the mutual information between the side information and the outcome. In ML: information gain in decision trees = the same measure.
**Notes:** This is the link between information theory and decision theory: the value of information is bounded by the mutual information between the information and the outcome.

---

## Quantum Information Theory Atoms

### quantum-entropy (cross-domain alias: `von-Neumann`, `quantum-surprise`, `density-matrix`)
**Domain:** Information Theory & Coding
**Definition:** S(ρ) = −Tr(ρ log ρ). The quantum entropy of a density matrix ρ. For pure states |ψ⟩: S = 0. For maximally mixed: S = log d (d = dimension).
**Atom or composite:** Composite: compute eigenvalues of ρ → S = −Σ λ_i log λ_i.
**Cost model:** Diagonalization costs O(d³) for d-dimensional ρ.
**Real wall?** No.
**Cross-domain wiring:** von Neumann entropy = the quantum version of Shannon entropy. In physics: S = −k·Tr(ρ·ln ρ) (Boltzmann entropy, same form). In ML: mixed states = mixture of pure states.
**Notes:** For a bipartite pure state |ψ_AB⟩, S(A) = S(B) = entanglement entropy. This is the basis of quantum information theory.

### quantum-channel (cross-domain alias: `CPTP-map`, `quantum-capacity`, `noisy-channel`)
**Domain:** Information Theory & Coding
**Definition:** A quantum channel N is a completely positive trace-preserving (CPTP) map: N(ρ) = Σ_k A_k ρ A_k^† with Σ A_k^† A_k = I. The quantum capacity Q(N) is the maximum reliable rate of quantum information transmission.
**Atom or composite:** Composite: characterize the channel → compute quantum capacity Q(N) → design code for that capacity.
**Cost model:** Computing Q(N) is hard — there is no known single-letter formula for most channels.
**Real wall?** Yes — the quantum capacity is not well understood for general channels. The degradability and anti-degradability properties determine the structure of the capacity region.
**Cross-domain wiring:** Quantum channel = the quantum version of the classical channel. In retrieval: a quantum channel = a quantum state preparation and measurement process.
**Notes:** The quantum capacity is defined as the maximum rate at which quantum information can be transmitted reliably over the channel. It is zero for channels that are entanglement-breaking.

### superdense-coding (cross-domain alias: `quantum-communic`, `2-qubit-1-bit`, `entanglement-utility`)
**Domain:** Information Theory & Coding
**Definition:** With a shared entangled pair, Alice can transmit 2 classical bits by sending only 1 qubit. She applies one of four Pauli operations to her half of the entangled pair; Bob measures in the Bell basis to recover 2 bits.
**Atom or composite:** Composite: prepare Bell state |Φ⁺⟩ → Alice: apply Pauli operator → send 1 qubit → Bob: Bell measurement → recover 2 bits.
**Cost model:** One qubit of communication for 2 classical bits. The entanglement is a "pre-shared resource."
**Real wall?** No.
**Cross-domain wiring:** Superdense coding = the quantum advantage in communication. In networking: it demonstrates the power of pre-shared entanglement as a resource.
**Notes:** The reverse is quantum teleportation: sending 1 qubit of quantum state requires 2 classical bits + 1 shared entanglement pair.

### quantum-key-dist (cross-domain alias: `BB84`, `E91`, `entanglement-QKD`)
**Domain:** Information Theory & Coding
**Definition:** BB84: Alice sends qubits in one of two bases (rectilinear or diagonal); Bob measures in a random basis; they announce bases and keep only the bits where they chose the same basis. Eavesdropper Eve causes detectable errors.
**Atom or composite:** Composite: prepare qubits in random states → measure → basis reconciliation → error estimation → privacy amplification.
**Cost model:** Error rate estimation requires a sample of the raw key. Privacy amplification compresses the key to remove Eve's information.
**Real wall?** Yes — QKD requires physical hardware (single photon sources, detectors). Practical QKD systems have imperfections that can be exploited by advanced adversaries.
**Cross-domain wiring:** QKD = information-theoretically secure key exchange = the same as the key distillation problem in classical cryptography, but with physics instead of computational hardness.
**Notes:** The BB84 security proof shows that any eavesdropping strategy necessarily introduces errors, which Alice and Bob can detect.

---

## Thermodynamic Entropy & Statistical Physics Atoms

### Boltzmann-entropy (cross-domain alias: `microstate-count`, `thermodynamic-entropy`, `W-log`)
**Domain:** Information Theory & Coding
**Definition:** S = k·log W where W is the number of microstates consistent with a macrostate. Boltzmann's entropy connects statistical mechanics to information theory.
**Atom or composite:** Composite: count microstates W → S = k·log W.
**Cost model:** Counting W for large systems is intractable. Use partition function Z instead.
**Real wall?** No.
**Cross-domain wiring:** Boltzmann entropy = Shannon entropy for the uniform distribution over microstates. Gibbs entropy S = −k·Σ p_i log p_i generalizes this to non-uniform distributions.
**Notes:** Jaynes (1957) showed that maximum entropy is the fundamental principle of statistical mechanics — it is the least biased inference consistent with known constraints.

### partition-function (cross-domain alias: `free-energy`, `Z-sum`, `state-sum`)
**Domain:** Information Theory & Coding
**Definition:** Z = Σ_i e^{−βE_i}. All thermodynamic quantities follow from Z: F = −kT·log Z, S = −∂F/∂T, U = ⟨E⟩. The partition function is the generating function of the system.
**Atom or composite:** Composite: for each energy level: compute e^{−βE} → sum → compute free energy and derivatives.
**Cost model:** Computing Z exactly requires enumerating all states — exponential in system size. Mean field and Monte Carlo are approximations.
**Real wall?** Yes — Z cannot be computed for large systems without approximation.
**Cross-domain wiring:** Z = the moment generating function = Σ p_i·e^{β·x_i} (with p_i uniform). In ML: the partition function of a Boltzmann machine = the normalization constant that makes it a proper distribution.
**Notes:** Mean field theory approximates Z by factoring it: Z ≈ ∏ Z_i, treating each spin independently. This is the mean field approximation for the partition function.

### maxent-inference (cross-domain alias: `maximum-entropy`, `constraint-satisfaction`, `Jaynes-inference`)
**Domain:** Information Theory & Coding
**Definition:** Given constraints on observables (e.g., known mean energy), the maximum entropy distribution is the one with the least bias (maximum H) subject to those constraints. Result: Boltzmann-Gibbs distribution.
**Atom or composite:** Composite: set up Lagrange multipliers for constraints → maximize H(p) = −Σ p_i log p_i subject to Σ p_i·f_j(x_i) = ⟨f_j⟩ → solve → p_i ∝ exp(−Σ λ_j·f_j(x_i)).
**Cost model:** Solving for the Lagrange multipliers requires optimization — typically via Newton's method.
**Real wall?** No. But the constraints must be correct and complete — wrong constraints → wrong distribution.
**Cross-domain wiring:** MaxEnt = the principle of insufficient reason = Bayesian inference with uniform priors. In ML: logistic regression = MaxEnt with log-odds constraints.
**Notes:** Jaynes (1957) showed that MaxEnt is the consistent inference procedure for any domain — it is the only method that doesn't introduce unwarranted assumptions.

---

## Large Deviations & Concentration Atoms

### Cramér-theorem (cross-domain alias: `LD-principle`, `rate-function`, `sample-concentration`)
**Domain:** Information Theory & Coding
**Definition:** For i.i.d. samples, P((1/n)·Σ X_i ∈ A) ≈ exp(−n·inf_{x∈A} I(x)) where I(x) is the rate function. The large deviations principle governs the exponential decay of probabilities of rare events.
**Atom or composite:** Composite: compute I(x) = sup_{λ} [λ·x − log E[e^{λX}]] → rate function. For sums, I(x) = D(x||μ) for the mean.
**Cost model:** One Legendre transform of the cumulant generating function.
**Real wall?** No.
**Cross-domain wiring:** Cramér = the general form of Chernoff bound. In ML: Cramér governs the probability of tail events in empirical risk.
**Notes:** The Gartner-Ellis theorem extends Cramér's theorem to non-i.i.d. and non-independent processes.

### concentration-inequality (cross-domain alias: `Hoeffding`, `Azuma`, `McDiarmid`)
**Domain:** Information Theory & Coding
**Definition:** Hoeffding: for bounded independent random variables: P(Σ X_i − E[Σ X_i] ≥ t) ≤ exp(−2t²/Σ(b_i−a_i)²). Azuma: for martingale differences with bounded jumps: similar exponential bound.
**Atom or composite:** Composite: verify boundedness/martingale condition → apply inequality → bound tail probability.
**Cost model:** Trivial.
**Real wall?** No.
**Cross-domain wiring:** Concentration = the probability of large deviations in sums of bounded variables. In ML: used to bound generalization error via uniform convergence.
**Notes:** McDiarmid's inequality gives concentration for functions with bounded differences (sensitivity): if changing one input changes the output by at most c_i, then the function concentrates.

### PAC-Bayes (cross-domain alias: `Gibbs-posterior`, `prior-posterior`, `generalization-bound`)
**Domain:** Information Theory & Coding
**Definition:** For any prior P and posterior Q: E_{x~D}[L(Q, x)] ≤ E_{w~Q}[L(P, w)] + D(Q||P)/m. The Gibbs posterior is the minimizer of this bound.
**Atom or composite:** Composite: define prior P over hypotheses → compute KL to posterior Q → bound expected loss.
**Cost model:** One KL computation per posterior.
**Real wall?** No.
**Cross-domain wiring:** PAC-Bayes = the information-theoretic framework for generalization. The KL term is the information cost of changing from the prior to the posterior.
**Notes:** The Bayesian Occam's razor: the posterior automatically penalizes complex hypotheses through the KL term. The evidence (marginal likelihood) is the normalizing constant.

---

## Side Information & Interactive Coding Atoms

### cascade-entropy (cross-domain alias: `data-processing`, `Markov-chain`, `DPE`)
**Domain:** Information Theory & Coding
**Definition:** Data Processing Inequality: if X → Y → Z (Markov chain), then I(X;Y) ≥ I(X;Z). Processing Y cannot increase the information about X.
**Atom or composite:** Composite: check Markov chain condition → apply DPI → I(X;Y) ≥ I(X;Z).
**Cost model:** Trivial.
**Real wall?** No. But DPI is an inequality — it doesn't tell you how much information is lost at each step.
**Cross-domain wiring:** DPI = the arrow of information degradation in cascaded systems. In ML: the information bottleneck applies DPI iteratively: X → Z → Y. In retrieval: each processing step (tokenization, normalization) can only reduce information about the original query intent.
**Notes:** Strong DPI: I(f(X);Y) ≤ I(X;Y) for any function f. This is the most general form.

### mutual-info-flow (cross-domain alias: `directed-info`, `causal-info`, `Granger-info`)
**Domain:** Information Theory & Coding
**Definition:** Directed information I(X_n^m → Y_n^m) = Σ_{i=1}^m I(X_i^m; Y_i | Y_{i-1}). Measures the flow of information from past X to Y, accounting for causal direction.
**Atom or composite:** Composite: for each time step: compute I(X_i^m; Y_i | Y_{i-1}) → sum → directed information.
**Cost model:** Computing directed information requires the causal conditioning.
**Real wall?** No.
**Cross-domain wiring:** Directed information = the causal version of mutual information. In networking: it measures the information flow from sender to receiver in the presence of feedback. In ML: it measures causal influence.
**Notes:** I(X→Y) > 0 iff X Granger-causes Y (X helps predict Y beyond Y's own past). This is the information-theoretic version of Granger causality.

### capacity-with-feedback (cross-domain alias: `feedback-capacity`, `Schalkwijk-Kcoding`, `silent-feedback`)
**Domain:** Information Theory & Coding
**Definition:** With noiseless feedback from receiver to transmitter, the capacity of some channels increases. Schalkwijk-K coding: for the AWGN channel, a simple linear coding scheme achieves capacity with feedback.
**Atom or composite:** Composite: encoder: x_i = a_i·message + b_i·feedback → channel → decoder: update estimate → iterate.
**Cost model:** The coding scheme is simple — the cost is in the precision of the feedback channel.
**Real wall?** Yes — many channels (e.g., the Z-channel) have strictly higher capacity with feedback. The AWGN channel does not — its capacity is unchanged by feedback.
**Cross-domain wiring:** Feedback capacity = the mutual information with the causal conditioning of feedback. In networking: TCP ack is a feedback channel.
**Notes:** The feedback capacity of the binary symmetric channel is still unknown — an open problem in network information theory.

### compound-channel (cross-domain alias: `uncertain-channel`, `robust-coding`, `average-capacity`)
**Domain:** Information Theory & Coding
**Definition:** The channel state is unknown to both transmitter and receiver (belongs to a set). Capacity is the maximum rate achievable simultaneously for all states — the common code must work for all channel realizations.
**Atom or composite:** Composite: for each possible channel state: find capacity C(θ) → compound capacity = inf_θ C(θ).
**Cost model:** The infimum over all possible states determines the achievable rate.
**Real wall?** Yes — compound channel capacity is generally less than the average capacity. The uncertainty reduces the achievable rate.
**Cross-domain wiring:** Compound channel = worst-case robustness. In networking: designing a protocol that works for all network conditions.
**Notes:** The cognitive radio problem (where one transmitter knows the channel state) is the Gel'fand-Pinsker problem for channels.

---

*Last updated: 2026-06-21 (expanded with hypothesis testing, MAC/BC, universal coding, gambling, quantum IT, thermodynamics, large deviations, interactive coding)*
*Source doctrine: The Painted Fence — Jesse*
