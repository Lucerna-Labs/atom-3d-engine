# Signal Processing / RF — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions these specialize from.

---

## Modulation / Demodulation Atoms

### mix (cross-domain alias: `project`, `multiply`)
**Domain:** Signal Processing / RF
**Definition:** Multiply two signals — typically a carrier wave by a baseband signal (modulation) or a received signal by a LO (demodulation). In the frequency domain this shifts the spectrum.
**Atom or composite:** Atom
**Cost model:** 1 multiply-add per sample. Cheap. On modern SIMD, a vector of 8+ samples in parallel.
**Real wall?** No.
**Cross-domain wiring:** Same operation as `multiply` in linear algebra and `correlate` in signal processing. In retrieval: the dot product IS a mix in the vector domain.
**Notes:** The cost is in the multiply; the carrier/LO generation is where complexity hides. The mix itself is free.

### integrate (cross-domain alias: `fold`, `accumulate`)
**Domain:** Signal Processing / RF
**Definition:** Sum a signal over a window — matched filter output, energy detector, coherent integrator. y[n] = Σ x[n−k]·h[k] for k in window.
**Atom or composite:** Atom
**Cost model:** Additions over window length. Integrates noise power as N, signal power as N² — SNR improves as √N.
**Real wall?** No. Finite integration time is a real wall: SNR improvement stops when integration time = coherence time.
**Cross-domain wiring:** Equivalent to `fold(sum)` in retrieval. In linear algebra: dot product of signal vector with rectangular window. In physics: flux integration.
**Notes:** This is the fundamental SNR improvement primitive in radar, sonar, and spread-spectrum receivers.

### sample (cross-domain alias: `quantize`, `discretize`)
**Domain:** Signal Processing / RF
**Definition:** Capture signal amplitude at discrete time intervals. Governed by the Nyquist-Shannon theorem: sample at >2× the highest frequency or fold spectrum.
**Atom or composite:** Composite: sample + quantize. The quantize step introduces quantization error (distortion floor).
**Cost model:** ADC hardware cost scales with sample rate × bit depth (ENOB). At high frequencies, this is the real bottleneck.
**Real wall?** Yes — the Nyquist limit is a real wall. The Shannon-Nyquist sampling theorem states you cannot reconstruct a signal from sub-Nyquist samples. Conservation: bandwidth ↔ time resolution (uncertainty principle).
**Cross-domain wiring:** Analogous to `bin` in histograms and `bucket` in Count-Min Sketch. Discretization is universal.
**Notes:** Compressive sensing is a painted wall on this — sparsity lets you go sub-Nyquist, but the real wall is information content, not sample rate.

### quantize (cross-domain alias: `bin`, `bucket`)
**Domain:** Signal Processing / RF
**Definition:** Map a continuous amplitude to a discrete level. Bit depth determines dynamic range (6 dB per bit for uniform quantizers).
**Atom or composite:** Atom
**Cost model:** Truncation/rounding cost is near-zero. The cost is in the ENOB (effective number of bits) of the ADC hardware.
**Real wall?** Yes — dynamic range is conserved. Adding bits costs more ADC hardware. This is why Sigma-Delta converters exist: they trade speed for bits.
**Cross-domain wiring:** Histogram binning = quantize + count. Bloom filter bit-setting = quantize to {0,1}.
**Notes:** The distinction between a cheap quantizer (floor/round) and an expensive one (sigma-delta, successive approximation) is exactly the generator/primitive split.

---

## Filter Atoms

### convolve (cross-domain alias: `dot`, `inner product`, `project`)
**Domain:** Signal Processing / RF
**Definition:** Slide one signal (kernel/filter) over another: y[n] = Σ x[n−k] · h[k]. In the frequency domain: Y(f) = X(f) · H(f). The core linear transformation.
**Atom or composite:** Composite: fold(sum) over paired samples of two sequences. = fold over (element-wise multiply).
**Cost model:** O(N·M) for naive convolution. FFT-based: O(N log N). FFT convolution is cheap; naive is only cheap for small kernels.
**Real wall?** No.
**Cross-domain wiring:** Matrix multiplication is a 2D convolution. Dot product is 1D convolution. Cross-correlation is convolution with one signal time-reversed. Correlate in retrieval ≈ convolution in signal.
**Notes:** This is the most cross-domain primitive in the entire kit. Nearly every "similarity" or "matching" operation is a convolution.

### correlate (cross-domain alias: `cosine-similarity`, `jaccard`, `compare`)
**Domain:** Signal Processing / RF
**Definition:** Measure of similarity between two signals as a function of displacement (lag). Unnormalized: r[lag] = Σ x[n] · y[n+lag]. Normalized: divide by √(Σx² · Σy²).
**Atom or composite:** Composite: fold(sum) over (x[n] · y[n+lag]) for each lag.
**Cost model:** O(N·M) naive; O(N log N) via FFT correlation theorem. Normalization costs a square root.
**Real wall?** No.
**Cross-domain wiring:** Cosine similarity = normalized dot product = normalized cross-correlation at lag 0. Pearson correlation = zero-mean cosine. Jaccard similarity = set cross-correlation / set union. Matched filter = correlate signal with template.
**Notes:** The matched filter IS a correlate with a known template — it's optimal because it maximizes SNR at lag 0.

### FIR (cross-domain alias: `sliding-window`, `moving-average`, `finite-kernel convolve`)
**Domain:** Signal Processing / RF
**Definition:** Finite impulse response filter — output is weighted sum of current and past samples. Coefficients are fixed (no feedback). Always stable, can be linear-phase.
**Atom or composite:** Composite: convolve(signal, kernel) with finite kernel. Implementable as shift-register + multiply-accumulate.
**Cost model:** Cost = kernel_length × samples_per_second. Modern FIR can be implemented with MAC units at near-zero incremental cost per tap.
**Real wall?** No. Linear phase is achievable (no phase distortion) — this is a real advantage over IIR.
**Cross-domain wiring:** Exponential moving average (EMA) is a special case of FIR. Averaging filter = uniform-weight FIR. Weighted averaging = weighted FIR. In graphics: box blur = uniform-weight FIR.
**Notes:** An FIR filter IS a convolution. The "primitive" is the coefficient set — different coefficients = different filter = different capability.

### IIR (cross-domain alias: `feedback`, `autoregressive`, `recursive`)
**Domain:** Signal Processing / RF
**Definition:** Infinite impulse response filter — output depends on past outputs (feedback). More efficient per pole than FIR, but can be unstable.
**Atom or composite:** Composite: uses both FIR feedforward terms and feedback terms. y[n] = Σ a[k]·x[n−k] + Σ b[k]·y[n−k].
**Cost model:** Cheaper per pole than FIR (fewer coefficients for equivalent selectivity). Risk: instability if coefficients quantize or drift.
**Real wall?** Yes — IIR stability is a real constraint. Pole locations must stay inside the unit circle. Feedback loops can oscillate.
**Cross-domain wiring:** AR(p) models in time-series = IIR with specific coefficient constraints. RC/RL/LC circuits = analog IIR. PID controllers = a specific IIR shape.
**Notes:** The "IIR vs FIR" tradeoff is exactly the "recursive vs non-recursive" tradeoff in algorithms — one has state, one doesn't.

### matched-filter (cross-domain alias: `template-correlate`, `whitened-correlate`, `project`)
**Domain:** Signal Processing / RF
**Definition:** Correlate the received signal with the known pulse shape (template). Optimal linear detector for known signal in AWGN. Output is maximized when signal aligns with template.
**Atom or composite:** Composite: correlate(signal, template) after pre-whitening the noise (optional — makes it truly optimal).
**Cost model:** One correlate per symbol/pulse. Whitening requires estimating noise PSD (additional cost).
**Real wall?** No. But the optimality requires known signal shape and AWGN — deviations from these degrade performance.
**Cross-domain wiring:** In retrieval: the dot product / cosine similarity IS a matched filter in the vector space. The IDF-whitened project IS a whitened matched filter. In graphics: template matching = cross-correlation at lag 0.
**Notes:** This is the bridge primitive between RF and retrieval. Retrieval is a matched filter — you're correlating the query template against documents. Only the basis changes.

### whiten (cross-domain alias: `prewhiten`, `IDF-weight`, `background-subtract`)
**Domain:** Signal Processing / RF
**Definition:** Decorrelate and normalize the noise so its power spectral density is flat (white). Makes the matched filter optimal. Requires estimating the noise covariance and applying a whitening filter.
**Atom or composite:** Composite: estimate covariance → compute whitening filter → apply filter. Or in the frequency domain: divide by noise spectrum magnitude.
**Cost model:** Estimating the noise PSD costs a lot (need long observation). Whitening filter application costs one extra filter pass.
**Real wall?** No. But imperfect noise estimation degrades the whitened matcher's advantage over the unwhitened one.
**Cross-domain wiring:** IDF weighting in retrieval is an approximate pre-whitening — it down-weights common background terms before matching. LSA/SVD-based projections are a stronger (but corpus-specific) whitening.
**Notes:** This is the "matched filter without whitening" vs "with whitening" rung — exactly the IDF-whitened vs random project rung in SCG's `project` primitive.

---

## Fourier Domain Atoms

### FFT / DFT (cross-domain alias: `basis-transform`, `diagonalize`, `eigen-decompose`)
**Domain:** Signal Processing / RF
**Definition:** Transform a signal from time/spatial domain to frequency domain. Decomposes signal into sinusoidal basis functions. Inverse FFT reconstructs.
**Atom or composite:** Composite in implementation, but a primitive in the signal processing vocabulary — the operation you take when you want to work in the frequency domain.
**Cost model:** O(N log N) with Cooley-Tukey. Specialized FFT for real signals costs half. GPU FFT libraries achieve near-theoretical throughput.
**Real wall?** No. But the Fourier basis assumes stationarity — non-stationary signals (frequency content changes over time) need time-frequency transforms (wavelet, STFT), which are more expensive.
**Cross-domain wiring:** SVD in linear algebra is the continuous-domain analogue of FFT (diagonalizing a linear operator into its eigenbasis). The Karhunen-Loève transform is the continuous-domain version. DCT is a real-valued FFT variant used in JPEG/MPEG.
**Notes:** This is the canonical "transform to a cheap basis" primitive — most frequency-domain operations are cheaper than their time-domain equivalents.

### convolve-via-FFT (cross-domain alias: `fast-convolution`, `O(NlogN) multiply`)
**Domain:** Signal Processing / RF
**Definition:** Compute convolution via FFT: FFT(x) · FFT(h) → IFFT. Saves when kernel is large (>> log N).
**Atom or composite:** Composite: FFT(x) + FFT(h) + pointwise-multiply + IFFT.
**Cost model:** O(N log N) total vs O(N·M) naive. Break-even is when kernel length M ≈ log N.
**Real wall?** No.
**Cross-domain wiring:** Same as fast matrix multiplication via Strassen or Coppersmith-Winograd — use a faster basis (FFT domain) to do the multiply cheaper. In retrieval: product quantization is a quantized FFT-domain multiply.
**Notes:** The FFT is a general-purpose "transform to make multiplication cheaper" primitive. Useful whenever you need to multiply two large sequences.

### spectral-analysis (cross-domain alias: `power-spectrum`, `periodogram`, `PSD-estimate`)
**Domain:** Signal Processing / RF
**Definition:** Estimate the power spectral density of a signal — how power distributes across frequencies. Methods: periodogram (magnitude-squared FFT), Welch's method (averaged periodograms), Yule-Walker (AR-based).
**Atom or composite:** Composite: FFT → magnitude² → average (over windows or frequency bins).
**Cost model:** One or more FFTs per estimate. Non-parametric methods (periodogram) are cheap; parametric (AR) are more expensive but higher resolution.
**Real wall?** No.
**Cross-domain wiring:** Graph Laplacian spectral analysis = same structure as PSD. SVD of a matrix = spectral decomposition = power spectrum of the operator. PCA = spectral analysis of the covariance matrix.
**Notes:** Spectral analysis tells you the frequency content of a signal — analogous to SVD telling you the dominant singular vectors of a matrix.

---

## Synthesis / Generation Atoms

### synthesize (cross-domain alias: `generate`, `compose`, `reconstruct`)
**Domain:** Signal Processing / RF
**Definition:** Generate a waveform from mathematical description — oscillator output, synthesized chirp, OFDM subcarrier, pulsed radar waveform.
**Atom or composite:** Composite: oscillator primitive + modulation primitive + shaping primitive.
**Cost model:** Pure math — costs nothing in isolation. The cost is in the fidelity and bandwidth of the output.
**Real wall?** Yes — bandwidth and energy are conserved. A wider-bandwidth signal carries more information but requires more energy or shorter duration (time-bandwidth product).
**Cross-domain wiring:** In retrieval: packing tokens into a context window = synthesizing a composite signal from basis vectors. In graphics: compositing = synthesizing a final pixel from layered signals.
**Notes:** This is the "recompose" step in the four-step doctrine. You decompose, transform, operate, then synthesize the output.

### modulate (cross-domain alias: `encode`, `impose`)
**Domain:** Signal Processing / RF
**Definition:** Impose information onto a carrier: amplitude (AM), phase (PM), frequency (FM), or a combination (QAM, OFDM).
**Atom or composite:** Composite: oscillator(carrier) + mix(data) + optionally pulse-shape.
**Cost model:** Modulation itself is cheap (mixing). Demodulation can be expensive if coherent detection requires carrier recovery.
**Real wall?** Yes for coherent modulation — carrier phase recovery is hard and expensive in fading channels. Non-coherent demodulation (envelope detection) is cheaper but less efficient.
**Cross-domain wiring:** In coding theory: encoding is modulation onto a code-space. In information theory: modulation is the physical-layer encoding of bits onto a channel.
**Notes:** OFDM is particularly interesting: it's a bank of orthogonal narrowband modulators, implementable as IFFT (very cheap).

### pulse-shape (cross-domain alias: `kernel`, `window`, `interpolate`)
**Domain:** Signal Processing / RF
**Definition:** Apply a bandlimited pulse shape to a symbol stream to control spectral containment and intersymbol interference (ISI). Common shapes: raised cosine, sinc, Gaussian.
**Atom or composite:** Composite: convolve(symbols, pulse_shape). The pulse shape IS a kernel.
**Cost model:** One convolution per symbol (or one convolution for the whole burst).
**Real wall?** Yes — the pulse shape determines the minimum bandwidth. You can't squeeze a signal into less bandwidth than its pulse shape allows without introducing ISI or out-of-band emissions.
**Cross-domain wiring:** Spline interpolation = pulse shaping for 1D data. Bilinear/bicubic texture interpolation = 2D pulse shaping. Gaussian blur kernel = pulse shaping in the image domain.
**Notes:** The raised-cosine family is the workhorse: it meets the Nyquist ISI criterion (zero ISI at sample points) while being bandlimited.

---

## Channel Coding Atoms

### encode (cross-domain alias: `compress`, `project-to-code-space`)
**Domain:** Signal Processing / RF
**Definition:** Map information bits to a codeword — adds structured redundancy. Types: block codes ( Reed-Solomon, BCH), convolutional codes, LDPC, turbo codes, polar codes.
**Atom or composite:** Composite: the specific encoder is a generator; the general operation is projection into a higher-dimensional space (redundancy = extra dimensions).
**Cost model:** Encoding is generally cheap (polynomial in block length). Decoding can be expensive (LDPC/turbo are iterative, NP-hard in general).
**Real wall?** Yes — Shannon capacity is the real wall. No code can exceed channel capacity. The gap to capacity is the conserved cost of complexity.
**Cross-domain wiring:** Fountain codes (LT, Raptor) = rateless encoding ≈ compression with guaranteed coverage. Any error-correcting code is a structured redundancy — analogous to structured noise in measurement systems.
**Notes:** LDPC and polar codes achieve within 0.004 dB of Shannon limit at block length 10⁷ — essentially touching the real wall.

### decode (cross-domain alias: `inference`, `correct`, `retrieve`)
**Domain:** Signal Processing / RF
**Definition:** Given a possibly corrupted codeword, recover the original information. Includes error detection, error correction, and erasure decoding.
**Atom or composite:** The decoding algorithm is the generator. Same code can have multiple decoders (hard vs soft decision, etc.).
**Cost model:** Decoding cost varies enormously: Hamming decoder is cheap (look-up table), Viterbi decoder is moderate (trellis), Belief Propagation for LDPC is iterative and can be expensive.
**Real wall?** Yes — decoding NP-hard codes is intractable. Most practical decoders are approximate and run in feasible time.
**Cross-domain wiring:** MAP decoding ≈ Bayesian inference. Belief propagation on factor graphs = message passing = sum-product algorithm = same structure as loopy belief propagation in graphical models.
**Notes:** The Viterbi algorithm for convolutional codes IS the same as forward-backward algorithm for HMMs — both are dynamic programming on a trellis.

---

## Spread Spectrum Atoms

### spread (cross-domain alias: `hash-to-space`, `expand`, `project-to-high-dim`)
**Domain:** Signal Processing / RF
**Definition:** Multiply data by a high-bandwidth spreading code (PN sequence) — spreads the signal across a wider bandwidth. DSSS: multiply by chip sequence. FHSS: hop carrier frequency.
**Atom or composite:** Composite: generate(PN_code) + mix(data, PN_code).
**Cost model:** Spreading/despreading costs one extra mix. The code generation is pseudo-random — cheap to generate.
**Real wall?** No. The spreading gain (processing gain) improves SNR in interference-limited environments but requires proportionally more bandwidth.
**Cross-domain wiring:** Random projection / Johnson-Lindenstrauss = spreading in the vector domain. MinHash = spreading to a high-dimensional space with collision probability encoding similarity.
**Notes:** CDMA cellular is spread spectrum — everyone shares the same bandwidth by using orthogonal spreading codes. The spreading IS the primitive; the orthogonal codes are the generator.

### despread (cross-domain alias: `correlate`, `match`, `project`)
**Domain:** Signal Processing / RF
**Definition:** Multiply the received spread signal by the same PN code and integrate. Rejects narrowband interference (which appears as noise after spreading) and recovers the data signal.
**Atom or composite:** Composite: mix(received, PN_code) + integrate(window).
**Cost model:** Same as spread + one integration.
**Real wall?** No.
**Cross-domain wiring:** Correlating a query vector against a document corpus = despreading the "signal" of relevance from the "noise" of irrelevant content.
**Notes:** This is the spread-spectrum version of the matched filter. Despreading IS the matched filter when the PN code is the template.

---

## Antenna / Array Atoms

### beamform (cross-domain alias: `weighted-sum`, `combine`, `focus`)
**Domain:** Signal Processing / RF
**Definition:** Weight and sum signals from multiple antenna elements to form a directional beam. Simple: phase-shift beamforming. Advanced: MVDR (minimum variance distortionless response), MUSIC, ESPRIT.
**Atom or composite:** Composite: for each element: shift-phase(element_signal) + scale(element_signal) + sum(all_elements). The beam pattern = sum of weighted element patterns.
**Cost model:** Phase shifting is near-zero cost (in digital). MVDR requires matrix inversion (covariance matrix). Real-time beamforming is cheap for small arrays.
**Real wall?** Yes — array size vs angular resolution: D·sin(θ) ≈ λ/N for N elements. Bigger aperture = finer beam = better resolution. Physical size of the array IS the conserved quantity.
**Cross-domain wiring:** Weighted sum of element signals = `combine` primitive in the retrieval kit. The phase shifts are `project` operations (steering vector = projection basis).
**Notes:** Hybrid beamforming (digital + analog) is the common architecture for 5G mmWave — reduces RF chains while maintaining beamforming capability.

---

## Summary: RF Atom → Cross-Domain Wiring

| RF Primitive | Retrieval Alias | Linear Algebra Alias | Graphics Alias |
|---|---|---|---|
| mix | dot product | element-wise multiply | lerp / blend |
| integrate | fold(sum) | dot product | accumulation buffer |
| convolve | BM25 = tf-fold + idf-combine | matrix multiply | Gaussian blur kernel |
| correlate | cosine similarity | dot product (normalized) | template match |
| matched-filter | BM25 / vector search | dot product after whitening | cross-correlation |
| whiten | IDF-weighting | ZCA whitening | background subtraction |
| FFT | SVD / eigendecompose | basis transform | frequency-domain rendering |
| spread | random projection / MinHash | JL projection | supersampling |
| beamform | weighted combine | weighted sum | alpha compositing |
| synthesize | token packing | linear combination | rendering / compositing |

---

*Last updated: 2026-06-21*
*Source doctrine: The Painted Fence — Jesse*


---

## Spectral / Time-Frequency Atoms

### STFT (cross-domain alias: `short-time-fourier`, `windowed-FFT`, `time-frequency-map`)
**Domain:** Signal Processing / RF
**Definition:** Slide a window over the signal and compute FFT within each window. Produces a time-frequency representation: energy as a function of time and frequency.
**Atom or composite:** Composite: window(signal, position) → FFT(windowed_segment) → accumulate into spectrogram.
**Cost model:** O(N·log N) per window. Overlap-add (OLA) reduces artifacts from windowing.
**Real wall?** Yes — the time-frequency resolution trade-off is a real wall: short windows give good time resolution but poor frequency resolution; long windows give good frequency but poor time. No window can simultaneously maximize both.
**Cross-domain wiring:** STFT = sliding Fourier transform = the discrete version of the Gabor transform. In retrieval: sliding window over a document = n-gram extraction. In graphics: a time-frequency map = a 2D heatmap of spectral content.
**Notes:** The window function (Hamming, Hann, Blackman) controls the sidelobe level. The overlap factor (typically 50-75%) controls the smoothness of the time axis.

### wavelet-transform (cross-domain alias: `DWT`, `multiresolution-decompose`, `scale-space`)
**Domain:** Signal Processing / RF
**Definition:** Decompose a signal into scales using wavelets (small waves) instead of sines. The continuous WT: W(a,b) = ∫ x(t)·ψ*((t−b)/a) dt. Discrete WT: cascade high-pass (detail) and low-pass (approximation) filters → downsample.
**Atom or composite:** Composite: low-pass filter → downsample → approximation. High-pass filter → downsample → detail. Recurse on approximation.
**Cost model:** O(N) per level via filter bank (Mallat algorithm). Each level halves the time resolution and doubles the frequency resolution.
**Real wall?** No. But the choice of wavelet (Haar, Daubechies, Symlet, Coiflet) determines what features are captured. Different wavelets are optimal for different signal types.
**Cross-domain wiring:** Wavelet pyramid = Laplacian pyramid in graphics (difference between scales). In retrieval: wavelet decomposition of a time series = multi-resolution token representation. In physics: wavelet analysis = analyzing signals at different diffusion lengths.
**Notes:** Haar wavelets are the simplest — they detect jumps/discontinuities. Daubechies wavelets are smoother and better for continuous signals. The wavelet transform is the natural basis for analyzing signals with transients.

### spectral-estimate (cross-domain alias: `periodogram`, `Welch-method`, `Yule-Walker`)
**Domain:** Signal Processing / RF
**Definition:** Estimate the power spectral density (PSD) from a finite signal. Periodogram: |X(f)|². Welch's method: average periodograms of overlapping segments (reduces variance). Yule-Walker: AR-based spectral estimate (higher resolution).
**Atom or composite:** Composite: segment signal → FFT → |²| → average (Welch) or solve Yule-Walker equations (AR).
**Cost model:** O(N log N) per segment. AR estimation requires solving a Toeplitz system (Levinson recursion O(n²)).
**Real wall?** Yes — the bias/variance trade-off: more segments (Welch) reduces variance but increases bias. AR order selection is a bias/variance trade-off.
**Cross-domain wiring:** PSD = graph Laplacian spectrum = covariance matrix spectrum = singular value spectrum. All are spectral decompositions of operators/data.
**Notes:** Yule-Walker AR gives higher frequency resolution than Welch for short signals but can produce spurious peaks (spectral artifacts). The Burg method is a popular alternative that is intermediate.

---

## Channel Modeling Atoms

### multipath-channel (cross-domain alias: `delay-spread`, `echo`, `reflection-model`)
**Domain:** Signal Processing / RF
**Definition:** Model a wireless channel as L paths each with gain α_l, delay τ_l, and Doppler shift ν_l. Channel impulse response: h(t) = Σ α_l·e^{j2πν_lt}·δ(t−τ_l).
**Atom or composite:** Composite: generate path parameters → construct h(t) → convolve(signal, h).
**Cost model:** Convolution of signal with channel impulse response. Number of paths L is the quality knob.
**Real wall?** Yes — the coherence bandwidth B_c ≈ 1/(5τ_rms) determines whether the channel is flat or frequency-selective. If signal bandwidth > B_c, frequency-selective fading occurs.
**Cross-domain wiring:** Multipath = echoes in acoustics = reflections in optics = ISI (inter-symbol interference) in digital comms. All are the same phenomenon: delayed copies arriving at different times.
**Notes:** Rayleigh fading models the envelope of a multipath channel with no dominant line-of-sight. Rician fading includes a dominant LOS component.

### fading-model (cross-domain alias: `Rayleigh-fade`, `Rician-fade`, `log-normal-shadow`)
**Domain:** Signal Processing / RF
**Definition:** Model the statistical distribution of signal amplitude over time/frequency. Rayleigh: envelope of sum of many uncorrelated multipath components (no LOS). Rician: LOS component + Rayleigh. Log-normal: shadowing from obstacles.
**Atom or composite:** Composite: draw amplitude from the appropriate distribution → multiply signal by fading coefficient.
**Cost model:** One random draw per channel update period. Fading is typically slow enough that this is negligible.
**Real wall?** Yes — the coherence time T_c ≈ 1/(4ν_max) determines how fast the channel changes. Fast fading (T_c << symbol time) requires interleaving and strong coding.
**Cross-domain wiring:** Fading = multiplicative noise = signal attenuation = relevance decay in retrieval. Log-normal shadowing = Gaussian in dB space = same as log-normal distribution in many biological and economic models.
**Notes:** The Doppler spectrum (Jakes model) describes the frequency spread of the fading. Jake's assumption: all angles of arrival are equally likely → classic U-shaped Doppler spectrum.

### SNR-evaluate (cross-domain alias: `signal-to-noise`, `Eb/N0`, `CNR`)
**Domain:** Signal Processing / RF
**Definition:** SNR = P_signal / P_noise. Eb/N0 = energy per bit divided by noise spectral density (used in digital comms). CNR = carrier-to-noise ratio (used in analog).
**Atom or composite:** Atom (for the mathematical form). Composite in practice: estimate signal power + estimate noise power.
**Cost model:** Estimating noise power requires a noise-only measurement (silent period, or signal-free bins in OFDM).
**Real wall?** Yes — SNR is the fundamental channel resource. All communication, sensing, and detection performance is bounded by SNR. Shannon's capacity formula C = B·log₂(1+SNR) is the real wall for communication.
**Cross-domain wiring:** In retrieval: SNR corresponds to the signal-to-noise ratio of relevance vs non-relevance. The IDF-weighted project increases the "SNR" of retrieval by down-weighting common (noisy) terms. In ML: SNR = signal-to-noise ratio of the learned signal vs label noise.
**Notes:** In detection theory, the SNR determines the probability of detection for a given probability of false alarm (Neyman-Pearson criterion). The ROC curve is parameterized by SNR.

### interference-manage (cross-domain alias: `cancel`, `MUI-suppress`, `co-channel`)
**Domain:** Signal Processing / RF
**Definition:** Suppress co-channel interference (MUI) using adaptive filters (LMS, RLS) or spatial filtering (MIMO beamforming). The adaptive filter learns the interference channel and subtracts it.
**Atom or composite:** Composite: estimate interference channel → compute cancellation signal → subtract.
**Cost model:** LMS is cheap (O(N) per step) but slow to converge. RLS is faster-converging but O(N²).
**Real wall?** Yes — interference cancelation requires an estimate of the interference channel. If the interference is unknown or changing faster than adaptation, cancellation fails.
**Cross-domain wiring:** Interference cancellation = echo cancellation in acoustics. In retrieval: disambiguation = interference cancellation for the intended query meaning vs ambiguous interpretations.
**Notes:** Successive interference cancellation (SIC) in NOMA (non-orthogonal multiple access) cancels users one by one in descending power order. This is the same as peeling off components in peeling decoding.

---

## OFDM / Multicarrier Atoms

### ofdm-modulate (cross-domain alias: `IFFT-modulate`, `subcarrier-map`, `cyclic-prefix`)
**Domain:** Signal Processing / RF
**Definition:** Map data onto N orthogonal subcarriers via IFFT: x[n] = (1/N)·Σ X[k]·e^{j2πnk/N}. Add cyclic prefix (CP) of length L_cp ≥ τ_max to convert linear convolution to circular convolution.
**Atom or composite:** Composite: map(symbols to subcarriers) → IFFT → add CP.
**Cost model:** One N-point IFFT per OFDM symbol = O(N log N). This is extremely cheap — OFDM's efficiency comes from using FFT.
**Real wall?** Yes — OFDM is sensitive to frequency offset (breaks orthogonality) and to phase noise (rotates constellation). The CP must be ≥ the channel delay spread or ISI contaminates the FFT window.
**Cross-domain wiring:** OFDM IFFT = the same operation as IFFT in signal processing, but applied to the carrier domain. In retrieval: packing tokens into an OFDM-style symbol = the same mapping into an orthogonal basis.
**Notes:** The CP turns the linear convolution with the channel into circular convolution, which the FFT diagonalizes. This makes channel equalization a single-tap multiplication per subcarrier.

### ofdm-equalize (cross-domain alias: `single-tap-equalize`, `channel-correct`, `frequency-equalize`)
**Domain:** Signal Processing / RF
**Definition:** After removing the CP and taking FFT, each subcarrier k has been multiplied by H[k] (the channel at that frequency). Equalization: X̂[k] = Y[k] / H[k] (ZF) or MMSE: X̂[k] = H*[k]·Y[k] / (|H[k]|² + 1/SNR).
**Atom or composite:** Composite: estimate H[k] (via pilots) → compute equalizer G[k] → multiply Y[k]·G[k].
**Cost model:** One complex division per subcarrier. Pilots require extra subcarriers (overhead).
**Real wall?** Yes — the equalizer assumes the channel is constant over the OFDM symbol duration. Fast fading (high Doppler) breaks this assumption.
**Cross-domain wiring:** Single-tap equalization = diagonalizing a circulant matrix via FFT (the channel matrix is circulant). In retrieval: each SCG pack = a subcarrier; the pack's representation = the channel gain; retrieval = equalizing and detecting.
**Notes:** Zero-forcing (ZF) equalization amplifies noise on deep fades (small H[k]). MMSE equalization is better when SNR is limited — it trades some ISI for less noise amplification.

---

## Synchronization Atoms

### timing-recover (cross-domain alias: `symbol-sync`, `clock-recovery`, `Gardner-TED`)
**Domain:** Signal Processing / RF
**Definition:** Estimate and correct the sampling timing offset between transmitter and receiver. Gardner timing error detector: use two samples per symbol (at 0.5T and T) to compute timing error.
**Atom or composite:** Composite: sample at 2× symbol rate → compute timing error → loop filter → NCO (numerically controlled oscillator) → adjust sampling phase.
**Cost model:** One timing error computation per 2×-rate sample. The loop filter bandwidth must be tuned to the dynamics.
**Real wall?** Yes — timing recovery bandwidth trades off between tracking speed (high bandwidth) and noise sensitivity (low bandwidth). Too much noise causes jitter; too little loses lock on fast changes.
**Cross-domain wiring:** Timing recovery = phase-locked loop (PLL) = same as the phase tracking loop in GPS. In retrieval: query timing analysis = temporal relevance weighting.
**Notes:** The early-late gate is the classic analog timing detector: compare early (advance) and late (delay) samples — the difference is the timing error.

### carrier-sync (cross-domain alias: `phase-track`, `CFO-correct`, `coherent-detect`)
**Domain:** Signal Processing / RF
**Definition:** Track residual carrier frequency and phase offset after coarse CFO correction. Phase-locked loop (PLL) on the constellation: compute phase error from known pilot symbols → loop filter → rotate constellation.
**Atom or composite:** Composite: compute phase error → loop filter → NCO → derotate(complex samples).
**Cost model:** One phase rotation per sample. Very cheap in digital.
**Real wall?** Yes — phase noise from the LO (local oscillator) accumulates over time, causing constellation rotation. Wide loop bandwidth tracks faster but amplifies more noise.
**Cross-domain wiring:** Carrier sync = phase tracking in GPS and radar. In retrieval: coherence tracking in conversational context = maintaining phase continuity across turns.
**Notes:** Phase tracking is unnecessary for differential modulation (DPSK) — the information is in the phase *difference*, not the absolute phase.

### frame-sync (cross-domain alias: `preamble-detect`, `sync-word`, `packet-detect`)
**Domain:** Signal Processing / RF
**Definition:** Find the start of a frame or packet by detecting a known preamble or sync word. Correlate the received signal with the known sync word; peak detection gives the frame start.
**Atom or composite:** Composite: correlate(received, sync_word) → find(peak) → set frame start.
**Cost model:** One correlation per possible start position. FFT-based correlation can speed this up.
**Real wall?** No. But false sync detection (noise mimicking the sync word) degrades performance. The sync word should have good autocorrelation properties (sharp peak, low sidelobes).
**Cross-domain wiring:** Frame sync = header detection = finding the start of a structured record. In retrieval: finding the start of a relevant passage = frame sync.
**Notes:** Zadoff-Chu sequences are commonly used as sync words in LTE because they have perfect periodic autocorrelation (delta function) and low cross-correlation.

---

## Advanced Beamforming Atoms

### phased-array (cross-domain alias: `array-factor`, `steering-vector`, `delayed-sum`)
**Domain:** Signal Processing / RF
**Definition:** Phase-shift each element by the steering vector φ_k = e^{j2π/λ·d·sin(θ)·k} to form a beam toward angle θ. The array factor = Σ w_k·e^{jφ_k} where w_k are element weights.
**Atom or composite:** Composite: compute steering vector for θ → multiply each element by steering phase → sum all elements.
**Cost model:** One complex multiply-add per element. For N elements, this is N operations — trivial for small arrays, expensive for large (massive MIMO, 64-256 elements).
**Real wall?** Yes — grating lobes appear when element spacing d > λ/2. The beam pattern repeats at angles where d·sin(θ) = m·λ.
**Cross-domain wiring:** Phased array steering = focused beam = constructive interference of delayed signals. In retrieval: beamforming weights = the SCG pack weights — each pack steers toward a different query direction.
**Notes:** True time delay (TTD) beamformers are preferred over phase shifters for wideband arrays — they delay the signal, not just rotate its phase, avoiding beam squint (beam direction changing with frequency).

### MUSIC (cross-domain alias: `super-resolution-DOA`, `eigenvector-spectral`, `angle-estimate`)
**Domain:** Signal Processing / RF
**Definition:** Multiple Signal Classification — super-resolution direction-of-arrival estimator. Compute the noise subspace from the covariance matrix; the spatial spectrum = 1 / (eᵛH(θ)·E_n·E_nH·e(θ)) has peaks at the DOAs.
**Atom or composite:** Composite: estimate covariance R̂ → eigendecompose → separate signal/noise subspaces → sweep θ and compute spatial spectrum.
**Cost model:** O(N³) for eigendecomposition of N×N covariance matrix. The spatial spectrum sweep requires computing the steering vector for each θ.
**Real wall?** Yes — MUSIC requires M > K (more snapshots than sources) and well-separated sources. It degrades rapidly when sources are correlated (coherent).
**Cross-domain wiring:** MUSIC = spectral analysis in the spatial domain. In retrieval: MUSIC on the document similarity matrix = finding dominant topic clusters. In physics: MUSIC on the Hessian = finding normal modes.
**Notes:** ESPRIT (Estimation of Signal Parameters via Rotational Invariance Techniques) is a related method that uses the rotational invariance of the signal subspace to estimate DOAs without a spectral sweep — it is computationally cheaper.

### hybrid-beamform (cross-domain alias: `digital-analog-hybrid`, `mmWave-beamform`, `subconnected`)
**Domain:** Signal Processing / RF
**Definition:** For large arrays (mmWave, 5G), split beamforming between a few RF chains (digital) and many phase shifters (analog). Reduces hardware cost but constrains the beamforming matrix to be a product of analog × digital.
**Atom or composite:** Composite: digital precoder (F_BB) → analog precoder (F_RF, phase-only) → apply to streams.
**Cost model:** The phase-only constraint on F_RF makes optimization non-convex — alternating minimization is the standard approach.
**Real wall?** Yes — the phase-only constraint limits achievable beam patterns. Some beam patterns that are achievable with full digital (fully connected) are impossible with hybrid.
**Cross-domain wiring:** Hybrid beamforming = product of two rank-limited matrices = low-rank factorization of the beamforming matrix. This is exactly the same structure as matrix factorization for recommendation systems.
**Notes:** The fully connected hybrid architecture has one phase shifter per antenna per stream; the subconnected architecture shares phase shifters among groups of antennas — cheaper but more constrained.

---

## Radar / Sensing Atoms

### FMCW-chirp (cross-domain alias: `linear-chirp`, `range-Doppler`, `continuous-wave`)
**Domain:** Signal Processing / RF
**Definition:** Frequency-Modulated Continuous Wave radar: transmit a chirp (frequency linearly increasing with time) and mix received echo with transmitted signal. Beat frequency f_b = S·τ = S·2R/c directly gives range R.
**Atom or composite:** Composite: generate chirp → transmit → receive echo → mix with reference → FFT over time → find peak → compute range.
**Cost model:** One FFT over the chirp duration per range bin. 2D FFT over multiple chirps gives Doppler.
**Real wall?** No. But the chirp bandwidth B and duration T set the range resolution ΔR = c/(2B) and maximum range R_max = c·T/2.
**Cross-domain wiring:** FMCW beat frequency = the range-Doppler map = a 2D Fourier transform of the delay-Doppler domain. In retrieval: FMCW = a range query in the delay-Doppler space.
**Notes:** The range resolution ΔR = c/(2B) is determined only by the chirp bandwidth B — not by the carrier frequency. A 4 GHz chirp (UWB radar) has ~3.75 cm range resolution.

### SAR-process (cross-domain alias: `synthetic-aperture`, `azimuth-focus`, `phase-history`)
**Domain:** Signal Processing / RF
**Definition:** Synthetic Aperture Radar — use platform motion to synthesize a large aperture. The phase history across positions encodes the scene. Focused SAR: apply phase correction per position so all scatterers add coherently at their true location.
**Atom or composite:** Composite: collect phase history → for each pixel: compute phase correction per position → sum coherently → form image.
**Cost model:** O(N²) for N platform positions. Back-projection is exact but slow; range migration (ω-k) algorithms are faster.
**Real wall?** No. But motion errors (platform deviation from straight flight) must be compensated — uncompensated errors create phase errors that blur the image.
**Cross-domain wiring:** SAR = synthetic aperture = the same as aperture synthesis in radio interferometry (VLBI). In retrieval: SAR processing = reconstructive imaging of the semantic space.
**Notes:** Spotlight SAR focuses on a fixed ground patch. Stripmap SAR covers a swath. ScanSAR alternates between swaths. Each mode has different azimuth resolution vs swath coverage trade-offs.

### matched-filter-radar (cross-domain alias: `pulse-compression`, `range-compress`, `stretch-processing`)
**Domain:** Signal Processing / RF
**Definition:** Matched filter in radar: correlate the received pulse with the transmitted waveform. The output is compressed — a long pulse gives high range resolution and high SNR simultaneously (pulse compression gain = BT, bandwidth × time).
**Atom or composite:** Composite: generate reference waveform → correlate received with reference → output is the compressed pulse.
**Cost model:** One match filter per pulse. FFT-based (stretch processing) is used for very long chirps.
**Real wall?** No. But range sidelobes (from the waveform's autocorrelation) can obscure nearby targets. Waveforms with low sidelobes (Hamming-weighted chirp) sacrifice SNR for sidelobe level.
**Cross-domain wiring:** Pulse compression = matched filter = the same as the retrieval matched filter. The waveform is the "query"; the echo is the "document." The compressed output is the retrieval score.
**Notes:** The ambiguity function (range-Doppler map of the matched filter output) characterizes a radar waveform's resolution and sidelobe properties. A narrow central peak = good range resolution; low Doppler sidelobes = good Doppler resolution.

---

## Audio / Speech Atoms

### mfcc-compute (cross-domain alias: `mel-frequency-cepstrum`, `filterbank-coeff`, `speech-feature`)
**Domain:** Signal Processing / RF
**Definition:** Mel-frequency cepstral coefficients — the standard speech feature. Steps: pre-emphasis → window → FFT → mel filterbank → log → DCT → liftering. Gives ~13-40 coefficients per frame.
**Atom or composite:** Composite: pre-emphasize → frame → FFT → mel-filterbank → log → DCT → lifter.
**Cost model:** One FFT per frame + mel filterbank (N filter operations per frame) + DCT. Widely accelerated via hardware/FFT libraries.
**Real wall?** No. But the mel scale is a perceptual approximation — it is most accurate for the human speech range and degrades outside it.
**Cross-domain wiring:** MFCC = Fourier transform + perceptual filterbank + cosine transform = a perceptually-weighted spectral analysis. The DCT decorrelates the mel filterbank outputs (similar to PCA). In retrieval: MFCC-like features for audio retrieval.
**Notes:** The delta and delta-delta coefficients (first and second derivatives of MFCCs over time) capture the temporal dynamics of speech — essential for speech recognition.

### gammatone-filter (cross-domain alias: `auditory-filterbank`, `cochlea-model`, `binaural-filter`)
**Domain:** Signal Processing / RF
**Definition:** Model the frequency selectivity of the cochlea using a gammatone filterbank. The gammatone impulse response: g(t) = a·t^{n-1}·cos(2πf_c·t + φ)·e^{-2πbt}. Each filter models one auditory channel.
**Atom or composite:** Composite: for each center frequency f_c: apply gammatone filter → extract envelope.
**Cost model:** One IIR filter per channel (4th order IIR approximates the gammatone well). Real-time for typical numbers of channels (64).
**Real wall?** Yes — the cochlea has ~3500 inner hair cells, each tuned to a different frequency. Computational models use far fewer channels (~64-128).
**Cross-domain wiring:** Gammatone = a bandpass filter with asymmetric response (steeper low-frequency tail). In retrieval: filterbank analysis = multi-resolution tokenization of an audio signal.
**Notes:** The auditory nerve fires roughly at the envelope of the gammatone filter output. This is why amplitude modulation (at ~4 Hz) in speech is perceptually salient — it maps to the temporal firing pattern of auditory nerves.

### vocoder (cross-domain alias: `source-filter`, `pitch-extract`, `speech-synthesize`)
**Domain:** Signal Processing / RF
**Definition:** Source-filter model of speech: glottal pulse train (source) convolved with vocal tract filter (formants). Analysis: extract pitch (F0) and formants (F1, F2, F3). Synthesis: excite filter with pitch pulses + noise.
**Atom or composite:** Composite: LPC analysis (linear predictive coding) → extract residual + coefficients → synthesize by filtering excitation.
**Cost model:** LPC analysis requires solving a Toeplitz system (autocorrelation-based Levinson recursion O(p²)).
**Real wall?** No. But the source-filter model is a simplification — it assumes the glottis and vocal tract are independent, which is not perfectly true.
**Cross-domain wiring:** Speech vocoder = source-filter decomposition = separating the generator (pitch) from the filter (formants). In retrieval: separating the topic (formants) from the specific instance (pitch/excitation).
**Notes:** Neural vocoders (WaveNet, WaveRNN) replaced classical vocoders for synthesis quality, but they lose the interpretability of the source-filter decomposition.

---

*Last updated: 2026-06-21 (expanded with spectral, channel, OFDM, sync, beamforming, radar, audio)*
*Source doctrine: The Painted Fence — Jesse*


---

## Audio / Speech / Acoustic Atoms

### audio-stft-rolling (cross-domain alias: `sliding-STFT`, `overlap-add`, `OLA`)
**Domain:** Signal Processing / RF
**Definition:** Continuous STFT via overlap-add: window the signal → FFT → store magnitude/phase → advance by hop size → repeat. The spectrogram is the accumulation of all frame-level spectra.
**Atom or composite:** Composite: window(frame) → FFT → accumulate magnitude² into spectrogram → hop.
**Cost model:** O(N·log N) total for N samples. Hop size H controls time resolution vs computation tradeoff.
**Real wall?** Yes — the STFT is a redundant representation: for H=1 (no overlap), it is N frames for N samples. Overlap adds redundancy.
**Cross-domain wiring:** STFT = time-frequency analysis = the Gabor transform. In retrieval: sliding window over text = token n-gram extraction.
**Notes:** 50% overlap (H = N_fft/2) is standard. TheOLA reconstruction property: with a tight frame (e.g., Hann window with 50% overlap), perfect reconstruction is possible.

### audio-istft (cross-domain alias: `inverse-STFT`, `OLA-synthesis`, `Griffin-Lim`)
**Domain:** Signal Processing / RF
**Definition:** Reconstruct time-domain signal from STFT magnitude + phase. Griffin-Lim: initialize phase → repeat: STFT → update phase to match magnitude → ISTFT. Converges to a consistent magnitude-only reconstruction.
**Atom or composite:** Composite (Griffin-Lim): initialize phase → for iter=1..N: forward STFT → replace magnitude → inverse STFT.
**Cost model:** N iterations of STFT/ISTFT — expensive (~50-200 iterations for quality). Neural vocoders have replaced Griffin-Lim for quality.
**Real wall?** Yes — Griffin-Lim is slow (many iterations) and produces artifacts (musical noise). Neural vocoders (WaveNet, DiffWave) produce much better quality.
**Cross-domain wiring:** Phase reconstruction = consistent magnitude-to-signal recovery. In retrieval: reconstructing document embeddings from magnitude-only similarity scores.
**Notes:** The consistency condition: STFT(ISTFT(X)) ≈ X. Griffin-Lim finds the signal with the closest STFT magnitude to the input.

### audio-mel-filterbank (cross-domain alias: `mel-scale`, `bark-scale`, `perceptual-filterbank`)
**Domain:** Signal Processing / RF
**Definition:** Map linear frequency bins to mel scale: m = 2595·log₁₀(1 + f/700). The mel filterbank consists of overlapping triangular filters spanning the frequency range.
**Atom or composite:** Composite: define mel filter centers → for each frame: for each filter: dot(FFT-magnitude, filter-bank-row) → accumulate.
**Cost model:** N_mel × N_fft operations per frame. Trivial compared to FFT.
**Real wall?** No. But the mel scale is a perceptual approximation — it is most accurate for the human speech range and degrades outside it.
**Cross-domain wiring:** Mel filterbank = perceptually weighted spectral analysis. In retrieval: bucketing frequencies into perceptual bins = bucketing terms into IDF bins.
**Notes:** The number of mel bins (typically 40-128) controls the trade-off between frequency resolution and dimensionality. More bins = higher resolution, more parameters.

### audio-mfcc-extract (cross-domain alias: `MFCC`, `cepstral-coeff`, `speech-feature`)
**Domain:** Signal Processing / RF
**Definition:** Mel-frequency cepstral coefficients: STFT → mel filterbank → log → DCT → lifter. The DCT decorrelates the mel filterbank outputs (similar to PCA), yielding compact ~13-40 coefficients.
**Atom or composite:** Composite: STFT → mel-filterbank → log → DCT → lifter.
**Cost model:** One FFT + N_mel filter operations + one DCT per frame. DCT is O(N_mel²) — cheap for small N_mel.
**Real wall?** Yes — MFCC discards phase information entirely and is optimized for speech. For music, use chroma or CQT features instead.
**Cross-domain wiring:** MFCC = perceptually weighted + decorrelated spectral features. In retrieval: MFCC as audio fingerprint for retrieval. In ML: MFCC as input features for speaker identification.
**Notes:** Delta and delta-delta coefficients (first and second derivatives of MFCCs) capture temporal dynamics and are essential for speech recognition.

### audio-chroma-cqt (cross-domain alias: `CQT`, `constant-Q-transform`, `music-pitch`)
**Domain:** Signal Processing / RF
**Definition:** Constant-Q transform: frequencies are spaced logarithmically, matching musical pitch intervals (equal temperament: each semitone = 2^{1/12} frequency ratio). Q = f/Δf = constant across frequencies.
**Atom or composite:** Composite: for each target frequency: apply frequency-domain window (Q-constrained) → accumulate energy → next frequency.
**Cost model:** O(N·log N) via FFT-based CQT using zero-padding. Q determines the ratio of frequency resolution to time resolution.
**Real wall?** No. But low-frequency bins have very high Q (long windows, poor time resolution).
**Cross-domain wiring:** CQT = log-frequency spectral analysis = the natural representation for music. In retrieval: log-scale bucketing = log-scale term frequency bucketing.
**Notes:** Chroma features (pitch class profiles) summarize the CQT into 12 bins (one per pitch class), capturing harmonic content irrespective of octave.

### audio-onsets-detect (cross-domain alias: `onset-detection`, `beat-tracking`, `transient-detect`)
**Domain:** Signal Processing / RF
**Definition:** Detect note onsets: compute spectral flux (difference in spectral magnitude between consecutive frames) → peak pick with adaptive threshold. High flux = sudden energy increase = onset.
**Atom or composite:** Composite: compute spectral difference → full-wave rectify → smooth → peak pick.
**Cost model:** One spectral flux computation per frame — cheap.
**Real wall?** Yes — spectral flux is sensitive to spectral changes that aren't onsets (vibrato, frequency modulation). Heuristic post-processing is needed.
**Cross-domain wiring:** Onset detection = change-point detection in the time-frequency plane. In signal: edge detection. In retrieval: topic boundary detection.
**Notes:** Complex domain onset detection (Duxbury et al.) uses phase information too, reducing false positives from energy-only methods.

### audio-source-separate (cross-domain alias: `NMF-separation`, `ICA-audio`, `cocktail-party`)
**Domain:** Signal Processing / RF
**Definition:** Separate a mixture into its constituent sources. NMF: decompose magnitude spectrogram V ≈ WH where W = dictionary (basis spectra), H = activations (source envelopes). ICA: separate by maximizing statistical independence.
**Atom or composite:** Composite: NMF: factorize magnitude spectrogram → assign components to sources → reconstruct via ISTFT. ICA: whiten → rotate to maximize kurtosis → separate.
**Cost model:** NMF: iterative multiplicative updates — O(IJK) per iteration where I=frames, J=components, K=frequency bins. ICA: eigendecomposition + rotations.
**Real wall?** Yes — NMF is non-unique (many valid decompositions). ICA assumes sources are independent and non-Gaussian — both assumptions can fail.
**Cross-domain wiring:** NMF = topic modeling for audio (W = topics, H = document-topic distribution). In retrieval: NMF = the same as its cousin LSA.
**Notes:** Deep learning (Spleeter, Demucs) has largely replaced NMF/ICA for source separation due to much better quality.

### audio-echo-cancel (cross-domain alias: `AEC`, `acoustic-echo`, `NLP`)
**Domain:** Signal Processing / RF
**Definition:** Remove acoustic echo from a microphone signal in a full-duplex communication system. Estimate the room impulse response R(n) and subtract: y_mic − (x_spkr * R).
**Atom or composite:** Composite: estimate echo path R → compute echo estimate (convolution) → subtract from microphone signal.
**Cost model:** Adaptive filter (NLMS: Normalized LMS) runs in O(N) per sample. The filter length must cover the room reverberation time.
**Real wall?** Yes — the echo path changes when the room changes (furniture moves, people move). AEC must adapt in real-time.
**Cross-domain wiring:** AEC = adaptive interference cancellation in signal processing. In retrieval: echo cancellation = removing the "echo" of a previous query from a new query.
**Notes:** Double-talk detection (both ends speaking) is critical — during double-talk, the AEC should freeze adaptation to avoid diverging.

### audio-beamform-mic (cross-domain alias: `mic-array-beamform`, `delay-sum`, `MVDR-mic`)
**Domain:** Signal Processing / RF
**Definition:** Beamform using a microphone array to spatially filter acoustic sources. Delay-and-sum: delay each mic by τ_i = (d_i·sin θ)/c → sum → beam toward θ. MVDR: minimize output power while keeping response at θ flat.
**Atom or composite:** Composite: estimate TDOA (time-difference-of-arrival) → delay each channel → sum → output.
**Cost model:** One delay + multiply-add per microphone element. MVDR requires covariance estimation + matrix inversion.
**Real wall?** Yes — array aperture (element spacing × number of elements) determines angular resolution. Small arrays (2-4 mics) have coarse resolution.
**Cross-domain wiring:** Mic beamforming = the same as RF phased array beamforming. In retrieval: beamforming over document vectors = multi-query retrieval.
**Notes:** Subband beamforming (GCC-PHAT for TDOA, then MVDR per frequency band) is the standard approach for speech enhancement.

### audio-aec-reference (cross-domain alias: `reference-cancel`, `residual-echo-suppress`)
**Domain:** Signal Processing / RF
**Definition:** After AEC, residual echo remains (model error). Suppress it by: detect residual echo periods (when far-end only is present) → spectral subtraction or Wiener filtering to suppress.
**Atom or composite:** Composite: double-talk detection → residual echo estimation → spectral subtraction.
**Cost model:** Additional processing after AEC — cheap compared to AEC itself.
**Real wall?** Yes — over-suppression causes speech distortion. Under-suppression leaves audible echo.
**Cross-domain wiring:** Residual echo suppression = error correction after the main correction step. In retrieval: reranking after initial retrieval.
**Notes:** Comfort noise injection (CNI) replaces harsh muting during echo suppression with synthetic noise — perceptually better than complete muting.

### audio-dereverb (cross-domain alias: `de reverberation`, `spectral-subtract`, `WPE`)
**Domain:** Signal Processing / RF
**Definition:** Remove reverberation (late reflections) from a microphone signal. Weighted prediction error (WPE): model late reflections as a linear prediction from past samples → subtract.
**Atom or composite:** Composite: estimate late reflection model → predict late reflections → subtract.
**Cost model:** Iterative ( Expectation-Maximization): O(T·N²) where T = iterations, N = filter length.
**Real wall?** Yes — dereverberation requires estimating the room impulse response from the observed signal alone — an ill-posed problem.
**Cross-domain wiring:** Dereverberation = deconvolution = inverse filtering. In retrieval: removing "reverberation" from a noisy retrieval signal.
**Notes:** DNN-based dereverberation (e.g., Wu et al.) has significantly improved over classical WPE for far-field speech recognition.

### audio-pitch-detect (cross-domain alias: `F0-estimate`, `YIN`, `autocorrelation-pitch`)
**Domain:** Signal Processing / RF
**Definition:** Estimate fundamental frequency F0 (pitch) of a voiced signal. YIN: difference function D(τ) = Σ(x[t] − x[t+τ])² → cumulative mean normalized difference → find global minimum below threshold.
**Atom or composite:** Composite: compute difference function → CMNDF → find dip → parabolic interpolation.
**Cost model:** O(N²) for full difference function, or O(N·log N) using FFT-based autocorrelation.
**Real wall?** Yes — pitch detection is ambiguous for unvoiced speech or mixed voicing. The voicing decision (voiced/unvoiced) is as hard as pitch detection itself.
**Cross-domain wiring:** Pitch detection = fundamental frequency estimation = periodicity finding. In retrieval: periodicity detection in time series.
**Notes:** Praat's pitch detection algorithm is the de facto standard for speech analysis.

### audio-snr-estim (cross-domain alias: `segmental-SNR`, `PESQ`, `SISDR`)
**Domain:** Signal Processing / RF
**Definition:** Estimate SNR of a speech signal: SegSNR = (1/M) Σ_m 10·log₁₀( signal_power / noise_power ). SISDR (Scale-Invariant SNR): separate signal and noise using an ideal ratio mask.
**Atom or composite:** Composite: estimate noise from speech-free frames → compute power ratio → average.
**Cost model:** Requires a noise estimate — either from speech pause detection or from a noise model.
**Real wall?** Yes — noise estimation is hard in non-stationary noise (babble, music). The quality of the noise estimate determines SNR estimation quality.
**Cross-domain wiring:** SNR estimation = signal-to-noise ratio in the perceptual domain. In retrieval: the IDF-weight = an estimate of the "SNR" of term relevance vs background.
**Notes:** PESQ (Perceptual Evaluation of Speech Quality) is the standard ITU-T metric for voice quality — used in telecom standards.

### audio-fingerprint (cross-domain alias: `chromaprint`, `Shazam-hash`, `content-ID`)
**Domain:** Signal Processing / RF
**Definition:** Generate a compact audio fingerprint from a recording for identification. Chromaprint: STFT → extract spectral peaks → hash pairs of peaks with time difference between them.
**Atom or composite:** Composite: STFT → peak detection → pair peaks within a time window → hash (peak_freq₁, peak_freq₂, time_diff) → accumulate into fingerprint.
**Cost model:** One STFT + peak detection per window. The fingerprint is very compact (bits per second).
**Real wall?** No. But the fingerprint is designed to be robust to noise, MP3 compression, and speed changes — this robustness is the generator.
**Cross-domain wiring:** Audio fingerprinting = content-addressable storage for audio. In retrieval: document fingerprinting = content-addressable retrieval.
**Notes:** Shazam's algorithm: constellation map (spectral peaks with time) → anchor points → hash (peak₁, peak₂, time_diff) → match in database.

### audio-wavelet-denoise (cross-domain alias: `wavelet-threshold`, `sparsify`, `denoise`)
**Domain:** Signal Processing / RF
**Definition:** Denoise via wavelet domain thresholding: Wavelet transform → threshold small coefficients to zero (soft threshold: sign(x)·max(|x|−λ, 0)) → inverse wavelet.
**Atom or composite:** Composite: wavelet transform → threshold → inverse wavelet transform.
**Cost model:** O(N) per level via Mallat algorithm. Threshold selection (VisuShrink, BayesShrink) adds complexity.
**Real wall?** Yes — threshold selection is critical: too low = insufficient denoising, too high = over-smoothing.
**Cross-domain wiring:** Wavelet denoising = sparse approximation of a signal = keeping the largest coefficients. In retrieval: IDF weighting = soft thresholding on term frequencies.
**Notes:** The universal threshold λ = σ·√(2·log N) is oversmooth for natural images/signals. SURE (Stein's Unbiased Risk Estimate) threshold adapts to the signal.

---

## Advanced Filter Design

### butterworth-filter (cross-domain alias: `maximally-flat`, `IIR-lowpass`, `analog-prototype`)
**Domain:** Signal Processing / RF
**Definition:** All-pole IIR filter with maximally flat magnitude in passband: |H(jω)|² = 1/(1+(ω/ω_c)^(2N)). Poles lie on a circle in s-plane at angles π/2 + (2k−1)π/(2N).
**Atom or composite:** Composite: analog prototype → frequency transform (LP→LP/HP/BP/BS) → bilinear transform → digital filter.
**Cost model:** O(N) MACs per sample for order N. Design is O(N) closed-form.
**Real wall?** Yes — maximal flatness comes at cost of wide transition band. Trades selectivity for in-band ripple-freeness.
**Cross-domain wiring:** Connects to control-numerical-opt (system identification), graphics-rendering-lod (anti-aliasing prefilter).
**Notes:** Butterworth 1930. Lowest-order analog prototype; widely used in audio crossovers and anti-alias filtering.

### chebyshev-type1 (cross-domain alias: `equiripple-passband`, `Cheby1`, `IIR-sharper`)
**Domain:** Signal Processing / RF
**Definition:** IIR filter with equiripple passband, monotonic stopband: |H(jω)|² = 1/(1+ε²·T_N²(ω/ω_c)) where T_N is Chebyshev polynomial of first kind.
**Atom or composite:** Composite: choose ripple ε → poles on ellipse in s-plane → transform → bilinear.
**Cost model:** O(N) per sample, order N. Sharper transition than Butterworth for same order.
**Real wall?** Yes — passband ripple ε is the price of steeper rolloff. Group delay is non-uniform.
**Cross-domain wiring:** Linear-algebra-matrix (Chebyshev polynomial recurrence T_{n+1} = 2x·T_n − T_{n−1}), ml-training (Chebyshev features).
**Notes:** Useful when passband flatness can be sacrificed for better stopband rejection.

### chebyshev-type2 (cross-domain alias: `inverse-Chebyshev`, `equiripple-stopband`, `Cheby2`)
**Domain:** Signal Processing / RF
**Definition:** IIR filter with monotonic passband, equiripple stopband: 1/|H|² = 1+1/(ε²·T_N²(ω_s/ω)). Has finite zeros (unlike Butterworth/Cheby1).
**Atom or composite:** Composite: dual of Cheby1 via 1/ω transformation.
**Cost model:** O(N) per sample. Same order as Cheby1 for same specs but better passband.
**Real wall?** Yes — equiripple stopband leaves "spurs" at ripple-peak frequencies; sensitive to component tolerance.
**Cross-domain wiring:** Information-theory-coding (stopband ripple = noise floor).
**Notes:** Preferred when passband flatness matters more than stopband monotonicity.

### elliptic-cauer (cross-domain alias: `Cauer-filter`, `equiripple-both`, `Zolotarev`)
**Domain:** Signal Processing / RF
**Definition:** Equiripple in both passband and stopband. Sharpest transition for given order using Jacobi elliptic rational functions R_N(ω,k).
**Atom or composite:** Composite: solve elliptic-function design (Zolotarev problem) → pole/zero placement → transform.
**Cost model:** O(N) per sample. Design requires elliptic integrals K(k), K(k').
**Real wall?** Yes — sharpest possible rolloff for given order, but highest group-delay distortion and pole sensitivity.
**Cross-domain wiring:** Linear-algebra-matrix (elliptic functions), control-numerical-opt (Remez-type optimization in elliptic variables).
**Notes:** Cauer 1931. Optimal in the Zolotarev sense. Used where transition bandwidth is the binding constraint.

### bessel-filter (cross-domain alias: `Thomson-filter`, `linear-phase-IIR`, `maximally-flat-delay`)
**Domain:** Signal Processing / RF
**Definition:** IIR filter with maximally flat group delay in passband. Poles are roots of reverse Bessel polynomials θ_N(s).
**Atom or composite:** Composite: Bessel polynomial roots → normalize → transform.
**Cost model:** O(N) per sample. Slow rolloff (worst of standard prototypes).
**Real wall?** Yes — flat group delay implies poor magnitude selectivity. Bilinear transform destroys linear phase in discrete domain.
**Cross-domain wiring:** Control-numerical-opt (pure-delay approximation, Padé delay).
**Notes:** Preferred for audio/pulse applications where waveshape preservation matters more than sharpness.

### parks-mcclellan (cross-domain alias: `Remez-FIR`, `equiripple-FIR`, `Chebyshev-FIR`)
**Domain:** Signal Processing / RF
**Definition:** Optimal equiripple FIR design via Remez exchange: minimize max weighted error E(ω) = W(ω)·(H_d(ω) − H(ω)). Alternation theorem guarantees N+2 extrema.
**Atom or composite:** Composite: initialize extrema → solve interpolation → find new extrema → iterate until converged.
**Cost model:** O(N²) per iteration; typically converges in <10 iterations for order N.
**Real wall?** Yes — equiripple is optimal in L∞ sense; bounded by N (length). Transition band must be specified explicitly.
**Cross-domain wiring:** Control-numerical-opt (Remez exchange = minimax optimization), retrieval-search (alternation = pivoting).
**Notes:** Parks & McClellan 1972. The standard for FIR design when sharp transition is required.

### frequency-sampling-fir (cross-domain alias: `freq-samp`, `DFT-design`, `IDFT-FIR`)
**Domain:** Signal Processing / RF
**Definition:** Design FIR by sampling desired H_d(ω) at N points {ω_k = 2πk/N} → take IDFT → impulse response h[n].
**Atom or composite:** Atom: IDFT of frequency samples → window/truncate.
**Cost model:** O(N log N) via FFT.
**Real wall?** Yes — interpolated response is exact only at sample points; between-sample error can be large (Gibbs-like ripple).
**Cross-domain wiring:** Graphics-rendering-lod (frequency-domain texture sampling), ml-training (Fourier features design).
**Notes:** Useful for arbitrary magnitude responses; transition-band relaxation samples reduce inter-sample error.

### window-method-fir (cross-domain alias: `truncated-sinc`, `windowed-FIR`, `apodize-impulse`)
**Domain:** Signal Processing / RF
**Definition:** Ideal h_d[n] = IDTFT(H_d(ω)) is infinite; truncate via window: h[n] = h_d[n]·w[n]. Magnitude = convolution H_d(ω) ∗ W(ω).
**Atom or composite:** Composite: ideal IDTFT → multiply by window.
**Cost model:** O(N) once; per-sample O(N) MACs.
**Real wall?** Yes — Gibbs phenomenon at discontinuities; window choice trades mainlobe width vs sidelobe level.
**Cross-domain wiring:** Statistics-probability (kernel density estimation = windowed sum), graphics-rendering-lod (texture filter kernels).
**Notes:** Simplest FIR design; foundation for all windowed methods (Hamming, Hann, Kaiser, Blackman).

### hamming-window (cross-domain alias: `raised-cosine-window`, `Hamming-apodization`)
**Domain:** Signal Processing / RF
**Definition:** w[n] = 0.54 − 0.46·cos(2πn/(N−1)). Optimized to cancel first sidelobe (≈ −43 dB) of rectangular window.
**Atom or composite:** Atom: deterministic taper function.
**Cost model:** O(N) precompute; O(N) apply.
**Real wall?** Yes — fixed mainlobe width ≈ 4·2π/N; sidelobe floor ≈ −43 dB.
**Cross-domain wiring:** Statistics-probability (smoothing kernel), graphics-rendering-lod (texture mipmap weighting).
**Notes:** Richard Hamming 1977. Default window in many DSP textbooks; not the best for any single metric but a good compromise.

### hann-window (cross-domain alias: `Hanning`, `raised-cosine`, `sine-squared-window`)
**Domain:** Signal Processing / RF
**Definition:** w[n] = 0.5·(1 − cos(2πn/(N−1))). Equivalently sin²(πn/(N−1)).
**Atom or composite:** Atom: deterministic taper.
**Cost model:** O(N).
**Real wall?** Yes — first sidelobe at −32 dB but rolls off at 18 dB/octave (faster than Hamming).
**Cross-domain wiring:** Graphics-rendering-lod (Hann = cubic B-spline kernel approximation), statistics-probability (smooth-overlap-add reconstructions in STFT).
**Notes:** Self-cancelling sidelobes far from main lobe; preferred for spectral estimation with broadband noise.

### kaiser-window (cross-domain alias: `Kaiser-Bessel`, `prolate-spheroidal-approx`)
**Domain:** Signal Processing / RF
**Definition:** w[n] = I₀(β·√(1−((n−M)/M)²))/I₀(β), parameter β controls mainlobe/sidelobe tradeoff. Approximates DPSS (Slepian) sequences.
**Atom or composite:** Atom: parametric taper.
**Cost model:** O(N) with Bessel I₀ evaluation; β chosen from desired sidelobe attenuation A_s.
**Real wall?** Yes — approaches optimal mainlobe-energy concentration (Slepian bound) for given length and sidelobe.
**Cross-domain wiring:** Linear-algebra-matrix (DPSS eigenfunctions), information-theory-coding (max time-frequency energy concentration).
**Notes:** β ≈ 0.1102(A_s − 8.7) for A_s > 50 dB. Standard for tunable sidelobe levels.

### blackman-window (cross-domain alias: `Blackman-3term`, `low-sidelobe-window`)
**Domain:** Signal Processing / RF
**Definition:** w[n] = a₀ − a₁·cos(2πn/(N−1)) + a₂·cos(4πn/(N−1)); a₀=0.42, a₁=0.5, a₂=0.08. Adds second cosine term to Hamming.
**Atom or composite:** Atom: three-term cosine window.
**Cost model:** O(N).
**Real wall?** Yes — mainlobe ≈ 6·2π/N (wider), sidelobe ≈ −58 dB.
**Cross-domain wiring:** Statistics-probability (low-bias kernel), graphics-rendering-lod (low-leakage texture sampling).
**Notes:** Blackman-Harris generalizes to 4+ terms for sidelobes down to −92 dB.

### dolph-chebyshev-window (cross-domain alias: `Dolph-window`, `equiripple-sidelobe`)
**Domain:** Signal Processing / RF
**Definition:** Window minimizing mainlobe width for fixed sidelobe level. DFT of T_{N−1}(β·cos(ω/2))/T_{N−1}(β) where β = cosh(arcosh(10^(A/20))/(N−1)).
**Atom or composite:** Composite: design in frequency domain → IDFT.
**Cost model:** O(N log N) via FFT.
**Real wall?** Yes — optimal in Chebyshev sense (equiripple sidelobes), but discrete-time-domain ringing at endpoints.
**Cross-domain wiring:** Linear-algebra-matrix (Chebyshev recurrence), retrieval-search (max-margin signal concentration).
**Notes:** Dolph 1946 (for antenna arrays). Used in array beamforming as well as spectral analysis.

### bilinear-transform (cross-domain alias: `Tustin`, `s-to-z`, `analog-to-digital-IIR`)
**Domain:** Signal Processing / RF
**Definition:** Map s-plane to z-plane: s = (2/T)·(z−1)/(z+1). Preserves stability (left-half s → unit-disk z) but warps frequency: ω_d = 2·arctan(ω_a·T/2).
**Atom or composite:** Atom: substitution into transfer function.
**Cost model:** O(N) for order-N filter substitution.
**Real wall?** Yes — frequency warping must be pre-compensated; aliases of analog response beyond Nyquist are folded.
**Cross-domain wiring:** Control-numerical-opt (Tustin discretization of continuous controllers), graphics-rendering-lod (continuous-domain → discrete-pixel mapping).
**Notes:** Standard discretization method; pre-warp critical frequencies before transform.

### impulse-invariance (cross-domain alias: `IIT`, `sampled-impulse-response`)
**Domain:** Signal Processing / RF
**Definition:** Design IIR by sampling analog impulse response: h[n] = T·h_a(nT). Preserves time-domain response but aliases frequency spectrum.
**Atom or composite:** Atom: sample → scale.
**Cost model:** O(N) per sample for order-N filter.
**Real wall?** Yes — aliasing limits to band-limited analog prototypes (no high-pass without pre-filter). Only works for stable analog filters.
**Cross-domain wiring:** Graphics-rendering-lod (impulse sampling = point sampling vs area sampling), statistics-probability (sampling a continuous density).
**Notes:** Alternative to bilinear transform; preferred when matched time-response matters.

### all-pass-filter (cross-domain alias: `phase-shifter`, `unitary-IIR`, `dispersion-filter`)
**Domain:** Signal Processing / RF
**Definition:** Filter with |H(ω)|=1 ∀ω, only phase varies: H(z) = z^{-N}·A(z^{-1})/A(z). Used for phase equalization without magnitude change.
**Atom or composite:** Atom: pole-zero pairs symmetric about unit circle.
**Cost model:** O(N) per sample.
**Real wall?** Yes — Bode's relation links phase and magnitude in causal systems; all-pass is the degree of freedom left when magnitude is unity.
**Cross-domain wiring:** Linear-algebra-matrix (unitary operator), graphics-rendering-lod (warping with energy preservation).
**Notes:** Used in phase equalizers, fractional delay (Thiran), and lattice filter sections.

### lattice-filter (cross-domain alias: `Schur-Levinson`, `reflection-coefficient-form`)
**Domain:** Signal Processing / RF
**Definition:** Structure with reflection coefficients k_m ∈ (−1, 1): lattice ⟺ direct-form via Schur recursion. Stable iff |k_m| < 1 for all m.
**Atom or composite:** Composite: cascade of lattice stages, each 2-input/2-output.
**Cost model:** O(N) per sample (2N MACs).
**Real wall?** Yes — finite-wordlength roundoff in lattice is far less sensitive than direct form; stability is provable by checking |k_m|.
**Cross-domain wiring:** Linear-algebra-matrix (Schur decomposition), information-theory-coding (PARCOR coefficients in LPC speech coding).
**Notes:** Used in adaptive filtering (gradient lattice), LPC speech, seismic deconvolution. Numerically robust.

### prony-method (cross-domain alias: `Prony-fit`, `damped-exponential-fit`)
**Domain:** Signal Processing / RF
**Definition:** Fit signal as sum of exponentials: x[n] = Σ A_k·exp(s_k·n). Solve linear prediction for poles, then linear least squares for amplitudes.
**Atom or composite:** Composite: AR fit → root finding → amplitude least-squares.
**Cost model:** O(N³) for matrix factorization; root finding O(p³).
**Real wall?** Yes — noise sensitivity is high; sensitivity to order p and to ill-conditioned Hankel matrices.
**Cross-domain wiring:** Linear-algebra-matrix (Hankel matrix factorization), ml-training (signal decomposition = mixture model).
**Notes:** Prony 1795. Foundation for modern methods (MUSIC, ESPRIT). For noisy data prefer matrix-pencil method.

### pade-approximation (cross-domain alias: `Pade-IIR`, `rational-approx`)
**Domain:** Signal Processing / RF
**Definition:** Match first M+N+1 impulse response samples by rational H(z) = B(z)/A(z) of order (M,N). Exact at first M+N+1 lags, approximate elsewhere.
**Atom or composite:** Composite: convolve identity → solve Toeplitz system for denominator → compute numerator.
**Cost model:** O((M+N)²) for Toeplitz solve via Levinson.
**Real wall?** Yes — interpolatory at first samples but may explode at long lags; stability not guaranteed.
**Cross-domain wiring:** Linear-algebra-matrix (Toeplitz inversion), control-numerical-opt (delay approximation in continuous systems).
**Notes:** Pade 1892. Useful for low-order IIR design when impulse-response samples are known.

### ar-model (cross-domain alias: `autoregressive`, `all-pole-model`, `AR(p)`)
**Domain:** Signal Processing / RF
**Definition:** x[n] = Σ_{k=1}^{p} a_k·x[n−k] + e[n] with e[n] white noise. Spectrum estimate: P(ω) = σ²/|1 − Σ a_k·exp(−jωk)|².
**Atom or composite:** Composite: estimate autocorrelation → solve Yule-Walker → compute spectrum.
**Cost model:** O(p²) Levinson-Durbin for p-order model; O(p) Burg recursion.
**Real wall?** Yes — model order p must be chosen (AIC, BIC, MDL). All-pole assumes no zeros in spectrum.
**Cross-domain wiring:** Statistics-probability (AR processes), ml-training (linear prediction), control-numerical-opt (state-space ARMA).
**Notes:** Foundation of LPC, speech coding, spectral estimation. Burg's method (1967) avoids autocorrelation windowing.

### ma-model (cross-domain alias: `moving-average`, `all-zero-model`, `MA(q)`)
**Domain:** Signal Processing / RF
**Definition:** x[n] = Σ_{k=0}^{q} b_k·e[n−k]. Spectrum P(ω) = σ²·|Σ b_k·exp(−jωk)|². Finite-impulse response process driven by white noise.
**Atom or composite:** Atom: FIR filter on white noise.
**Cost model:** Estimation is non-linear; uses iterative spectral factorization or innovations algorithm.
**Real wall?** Yes — MA estimation is non-convex; AR is preferred when poles dominate.
**Cross-domain wiring:** Statistics-probability (MA(q) time series), information-theory-coding (linear processes).
**Notes:** Often combined with AR into ARMA models. MA spectral factorization is the dual of AR Yule-Walker.

### arma-model (cross-domain alias: `Box-Jenkins`, `pole-zero-model`, `ARMA(p,q)`)
**Domain:** Signal Processing / RF
**Definition:** x[n] − Σ a_k·x[n−k] = Σ b_k·e[n−k]. Spectrum P(ω) = σ²·|B(ω)/A(ω)|². Combines AR and MA.
**Atom or composite:** Composite: AR identification → residual MA fit → joint refinement (Box-Jenkins).
**Cost model:** Iterative estimation; high-order ARMA may be ill-conditioned.
**Real wall?** Yes — order selection is harder than for AR alone; model identifiability requires non-overlapping pole-zero locations.
**Cross-domain wiring:** Statistics-probability (Box-Jenkins time-series), control-numerical-opt (state-space realizations).
**Notes:** Foundation of econometric time series. Cadzow's modified ARMA reduces noise sensitivity.

### yule-walker-equations (cross-domain alias: `normal-equations-AR`, `autocorrelation-fit`)
**Domain:** Signal Processing / RF
**Definition:** For AR(p): R·a = −r where R is p×p Toeplitz of autocorrelations and r = [R(1)…R(p)]ᵀ. Solves for AR coefficients given autocorrelation.
**Atom or composite:** Atom: Toeplitz linear system.
**Cost model:** O(p²) via Levinson-Durbin recursion.
**Real wall?** Yes — autocorrelation estimates from finite data are biased; bias propagates to AR coefficients.
**Cross-domain wiring:** Linear-algebra-matrix (Toeplitz inverse), ml-training (normal equations for linear regression).
**Notes:** Yule 1927, Walker 1931. The canonical AR estimation equation.

### burg-method (cross-domain alias: `Burg-AR`, `maximum-entropy-spectrum`)
**Domain:** Signal Processing / RF
**Definition:** Estimate AR coefficients by minimizing sum of forward+backward prediction errors. Equivalent to maximum-entropy spectral estimate.
**Atom or composite:** Composite: lattice recursion → reflection coefficients → AR coefficients.
**Cost model:** O(p·N) for p-order from N samples.
**Real wall?** Yes — biased toward poles slightly inside unit circle; spectral line splitting for short data.
**Cross-domain wiring:** Information-theory-coding (max-entropy = least-assumption spectrum), ml-training (regularized estimation).
**Notes:** Burg 1967. Avoids windowing; provides ML estimates under Gaussian assumption.

### levinson-durbin (cross-domain alias: `Levinson-recursion`, `Toeplitz-solve`)
**Domain:** Signal Processing / RF
**Definition:** Recursively solve Toeplitz system R_p·a_p = −r_p by extending order p → p+1: a_{p+1}[k] = a_p[k] + k_{p+1}·a_p[p−k+1].
**Atom or composite:** Atom: order-recursive update.
**Cost model:** O(p²) total (one fewer order of magnitude than general matrix solve).
**Real wall?** Yes — only works for Toeplitz/Hermitian-positive-definite systems; numerical conditioning degrades near unit circle.
**Cross-domain wiring:** Linear-algebra-matrix (Toeplitz structure), retrieval-search (recursive nearest-neighbor in nested subspaces).
**Notes:** Levinson 1947, Durbin 1960. Foundation of LPC and signal processing recursions.

### modified-covariance-method (cross-domain alias: `MCM-AR`, `forward-backward-AR`)
**Domain:** Signal Processing / RF
**Definition:** AR estimation minimizing forward+backward prediction errors without autocorrelation assumption: solves a covariance-matrix least-squares directly.
**Atom or composite:** Composite: build data matrix → solve least squares.
**Cost model:** O(N·p) for N samples, order p.
**Real wall?** Yes — better resolution than autocorrelation method for short data; not guaranteed stable.
**Cross-domain wiring:** Linear-algebra-matrix (QR factorization), statistics-probability (high-resolution spectral estimation).
**Notes:** Preferred for short data records where Burg might have line-splitting artifacts.

### linear-prediction (cross-domain alias: `LP`, `Wiener-Hopf-causal`, `prediction-error-filter`)
**Domain:** Signal Processing / RF
**Definition:** Predict x[n] from past: x̂[n] = Σ a_k·x[n−k]. Solve E[(x[n]−x̂[n])·x[n−j]] = 0 (orthogonality principle).
**Atom or composite:** Composite: estimate autocorrelation → solve Yule-Walker.
**Cost model:** O(p²) Levinson; O(p) per-sample prediction.
**Real wall?** Yes — linear predictor is optimal for Gaussian; non-Gaussian signals need nonlinear predictors.
**Cross-domain wiring:** Ml-training (linear regression = prediction), information-theory-coding (predictive coding compression).
**Notes:** Heart of LPC speech coding (Atal & Schroeder 1967). Prediction error filter whitens the input.

### second-order-section (cross-domain alias: `SOS`, `biquad-cascade`, `direct-form-II`)
**Domain:** Signal Processing / RF
**Definition:** Factor IIR transfer function into cascade of 2nd-order sections (biquads): H(z) = Π_k (b₀ₖ + b₁ₖz⁻¹ + b₂ₖz⁻²)/(1 + a₁ₖz⁻¹ + a₂ₖz⁻²).
**Atom or composite:** Composite: factor → group conjugate pole pairs → order by pole radius.
**Cost model:** O(N) MACs per sample for N-order filter (5 MACs/biquad direct-form II).
**Real wall?** Yes — direct-form high-order IIR is numerically unstable for poles near unit circle; SOS mitigates by limiting precision sensitivity per section.
**Cross-domain wiring:** Linear-algebra-matrix (Schur factorization of A(z)), graphics-rendering-lod (cascaded box filters approximate Gaussian).
**Notes:** Pairing/ordering of sections (largest-Q last) minimizes numeric overflow risk.

### group-delay-equalization (cross-domain alias: `phase-equalizer`, `delay-flat-cascade`)
**Domain:** Signal Processing / RF
**Definition:** Cascade all-pass sections with H_eq to make total group delay τ_total(ω) flat across passband.
**Atom or composite:** Composite: design IIR for magnitude → add all-pass for phase.
**Cost model:** O(K) per sample for K equalizer stages.
**Real wall?** Yes — flat group delay can only be achieved approximately for non-zero-phase IIR.
**Cross-domain wiring:** Graphics-rendering-lod (chromatic correction = phase equalization), control-numerical-opt (Padé delay matching).
**Notes:** Used in audio mastering and communications channel equalization.

### minimum-phase-filter (cross-domain alias: `min-phase`, `causal-invertible`, `Hilbert-pair`)
**Domain:** Signal Processing / RF
**Definition:** Filter with all zeros inside unit circle. Has minimum group delay among all filters with the same |H(ω)|. Causal inverse is also stable.
**Atom or composite:** Composite: factor H(z) → reflect outside-unit-circle zeros inward (preserves |H|).
**Cost model:** O(N²) factorization for order N.
**Real wall?** Yes — minimum phase is the unique decomposition: H = H_min · H_allpass where H_min has all zeros inside.
**Cross-domain wiring:** Information-theory-coding (cepstral analysis, complex cepstrum), ml-training (causal sequence models).
**Notes:** Useful for stable inverse filtering and deconvolution. Zeros on the unit circle are boundary cases.

---

## Adaptive Filters

### lms-adaptive (cross-domain alias: `least-mean-squares`, `Widrow-Hoff`, `stochastic-gradient-FIR`)
**Domain:** Signal Processing / RF
**Definition:** Update filter coefficients w[n+1] = w[n] + μ·e[n]·x[n] where e[n] = d[n] − w[n]ᵀx[n]. Stochastic gradient on instantaneous MSE.
**Atom or composite:** Atom: one-tap weight update per sample.
**Cost model:** O(N) per sample for N-tap filter (2N MACs).
**Real wall?** Yes — convergence requires 0 < μ < 2/λ_max(R_xx); excess MSE proportional to μ.
**Cross-domain wiring:** Ml-training (SGD = LMS on quadratic loss), control-numerical-opt (gradient methods).
**Notes:** Widrow & Hoff 1960. Foundational adaptive algorithm; used in echo cancellation, equalization, beamforming.

### nlms-adaptive (cross-domain alias: `normalized-LMS`, `power-normalized-LMS`)
**Domain:** Signal Processing / RF
**Definition:** LMS with step normalization: w[n+1] = w[n] + (μ/(‖x[n]‖² + δ))·e[n]·x[n]. δ prevents division near zero.
**Atom or composite:** Composite: LMS + per-step normalization.
**Cost model:** O(N) per sample; one extra division.
**Real wall?** Yes — step-size bound becomes signal-independent (0 < μ < 2); tracking ability improved for nonstationary inputs.
**Cross-domain wiring:** Ml-training (Adam = NLMS-like with second-moment normalization), statistics-probability (rank-1 covariance update).
**Notes:** Default choice in modern adaptive equalization and acoustic echo cancellation.

### leaky-lms (cross-domain alias: `leaky-update`, `L2-regularized-LMS`)
**Domain:** Signal Processing / RF
**Definition:** w[n+1] = (1−μγ)·w[n] + μ·e[n]·x[n]. Adds weight decay γ to prevent drift in absence of persistent excitation.
**Atom or composite:** Atom: LMS + L2 regularization.
**Cost model:** O(N) per sample.
**Real wall?** Yes — leak biases steady-state estimate but improves robustness to finite precision and ill-conditioned inputs.
**Cross-domain wiring:** Ml-training (weight decay = leaky update), control-numerical-opt (regularized least squares).
**Notes:** Prevents weight blow-up under signal silence. Choose γ ≪ μ·λ_min.

### sign-lms (cross-domain alias: `sign-error`, `clipped-LMS`, `pilot-LMS`)
**Domain:** Signal Processing / RF
**Definition:** w[n+1] = w[n] + μ·sign(e[n])·x[n]. Replace multiplication by error with sign — simpler hardware.
**Atom or composite:** Atom: LMS with sign nonlinearity.
**Cost model:** O(N) shifts/adds per sample (no multiplies for sign×x).
**Real wall?** Yes — slower convergence than LMS but robust to impulsive errors.
**Cross-domain wiring:** Ml-training (signSGD for byzantine-robust training), information-theory-coding (1-bit quantized updates).
**Notes:** Used in low-power adaptive filters; analyzed via Kushner-style ODE methods.

### affine-projection-algorithm (cross-domain alias: `APA`, `K-step-LMS`)
**Domain:** Signal Processing / RF
**Definition:** Use last K input vectors X = [x[n], …, x[n−K+1]]; update w[n+1] = w[n] + μ·X·(XᵀX + δI)⁻¹·e where e is error vector.
**Atom or composite:** Composite: K-step block update with regularized inverse.
**Cost model:** O(K²·N + K³) per sample.
**Real wall?** Yes — APA(K) is intermediate between LMS (K=1) and RLS (K=∞). Larger K → faster convergence on colored input but more computation.
**Cross-domain wiring:** Linear-algebra-matrix (rank-K updates), ml-training (mini-batch SGD).
**Notes:** Excellent for acoustic echo cancellation with speech inputs; fast variants reduce per-sample cost.

### rls-adaptive (cross-domain alias: `recursive-least-squares`, `exponential-window-LS`)
**Domain:** Signal Processing / RF
**Definition:** Minimize Σ λ^(n−k)·|d[k] − w[n]ᵀx[k]|². Update via P[n] = (1/λ)·(P[n−1] − k[n]·x[n]ᵀ·P[n−1]) with gain k[n] = P[n−1]x[n]/(λ + x[n]ᵀ P[n−1] x[n]).
**Atom or composite:** Composite: matrix inverse Lemma update per sample.
**Cost model:** O(N²) per sample.
**Real wall?** Yes — converges in ~2N samples but quadratic cost; numerically unstable due to finite-precision P-matrix drift.
**Cross-domain wiring:** Linear-algebra-matrix (Sherman-Morrison update), ml-training (online ridge regression).
**Notes:** Forgetting factor λ ∈ (0.95, 1) trades tracking vs steady-state error.

### fast-rls (cross-domain alias: `FAEST`, `FTRLS`, `O(N)-RLS`)
**Domain:** Signal Processing / RF
**Definition:** RLS exploiting Toeplitz/shift-invariant structure to achieve O(N) per sample using prediction-error filters and forward/backward gain coupling.
**Atom or composite:** Composite: cascade of forward predictor → backward predictor → joint-process update.
**Cost model:** O(N) per sample (5N-10N MACs typical).
**Real wall?** Yes — numerical instability is a chronic problem; needs periodic resets or stabilization.
**Cross-domain wiring:** Linear-algebra-matrix (shift-invariance = displacement structure), retrieval-search (online recursive nearest-neighbor in sliding window).
**Notes:** Cioffi-Kailath FAEST 1984. Used when adaptive filter length N is large and per-sample cost matters.

### lattice-rls (cross-domain alias: `lattice-LS`, `Schur-RLS`)
**Domain:** Signal Processing / RF
**Definition:** RLS in lattice (reflection-coefficient) form. Each stage adapts a single reflection coefficient via order-recursion.
**Atom or composite:** Composite: order- and time-recursive update.
**Cost model:** O(N) per sample.
**Real wall?** Yes — far more numerically robust than direct-form fast RLS; reflection coefficients are bounded.
**Cross-domain wiring:** Linear-algebra-matrix (LU vs Cholesky structural choice), information-theory-coding (PARCOR coefficients).
**Notes:** Friedlander 1982. Method of choice when robustness matters.

### qr-rls (cross-domain alias: `QR-decomposition-RLS`, `square-root-RLS`)
**Domain:** Signal Processing / RF
**Definition:** RLS via QR decomposition of weighted data matrix using Givens rotations. Avoids explicit P-matrix and its instability.
**Atom or composite:** Composite: Givens-rotation update of R-factor.
**Cost model:** O(N²) per sample, but can be parallelized in systolic arrays.
**Real wall?** Yes — numerically guaranteed positive-definite; preferred for fixed-point hardware.
**Cross-domain wiring:** Linear-algebra-matrix (QR factorization), control-numerical-opt (square-root Kalman filtering).
**Notes:** Maps naturally to systolic array architectures (McWhirter array).

### kalman-filter-signal (cross-domain alias: `optimal-recursive`, `state-space-Kalman`)
**Domain:** Signal Processing / RF
**Definition:** Predict x̂[n|n−1] = A·x̂[n−1|n−1]; update via Kalman gain K = P_pred·Cᵀ·(C·P_pred·Cᵀ + R)⁻¹. Optimal linear estimator under Gaussian noise.
**Atom or composite:** Composite: predict + update cycle with covariance propagation.
**Cost model:** O(n³) per step for n-state system (Riccati update).
**Real wall?** Yes — optimal only under linear-Gaussian assumption; covariance can lose positive definiteness in finite precision.
**Cross-domain wiring:** Control-numerical-opt (LQG control = Kalman+LQR), ml-training (Bayesian filtering), statistics-probability (conjugate Gaussian update).
**Notes:** Kalman 1960. Foundation of estimation theory. Used in tracking, navigation, channel estimation.

### extended-kalman-filter (cross-domain alias: `EKF`, `linearized-Kalman`)
**Domain:** Signal Processing / RF
**Definition:** Apply Kalman recursions to first-order Taylor-linearized nonlinear dynamics: F = ∂f/∂x|x̂, H = ∂h/∂x|x̂.
**Atom or composite:** Composite: linearize → Kalman step.
**Cost model:** O(n³) per step + Jacobian evaluation.
**Real wall?** Yes — linearization error makes EKF diverge for strongly nonlinear systems; covariance is no longer truly second-order accurate.
**Cross-domain wiring:** Control-numerical-opt (nonlinear filtering), ml-training (online Newton on linearized loss).
**Notes:** Most-used filter in inertial navigation and robotic SLAM despite suboptimality.

### unscented-kalman-filter (cross-domain alias: `UKF`, `sigma-point-Kalman`)
**Domain:** Signal Processing / RF
**Definition:** Propagate 2n+1 sigma points through nonlinearity, reconstruct mean/covariance via weighted samples. Avoids Jacobian computation.
**Atom or composite:** Composite: generate sigma points → propagate → recombine.
**Cost model:** O(n³) per step (Cholesky of covariance) + 2n+1 nonlinear evaluations.
**Real wall?** Yes — accurate to 3rd-order Taylor for Gaussian; fails for multimodal posteriors.
**Cross-domain wiring:** Statistics-probability (deterministic sampling vs Monte Carlo), ml-training (deterministic variational inference).
**Notes:** Julier-Uhlmann 1997. Often superior to EKF for nonlinear sensor fusion.

### particle-filter (cross-domain alias: `sequential-Monte-Carlo`, `SMC`, `bootstrap-filter`)
**Domain:** Signal Processing / RF
**Definition:** Represent posterior p(x[n]|y[1:n]) by N weighted particles {x_i, w_i}. Resample with replacement when effective sample size drops.
**Atom or composite:** Composite: propose → weight by likelihood → resample.
**Cost model:** O(N_p · n) per step for N_p particles and n-dim state.
**Real wall?** Yes — particle degeneracy in high dimensions; curse of dimensionality limits scaling beyond ~10 state dims.
**Cross-domain wiring:** Statistics-probability (importance sampling, MCMC), ml-training (sequential Monte Carlo VI).
**Notes:** Gordon-Salmond-Smith 1993. Handles arbitrary non-Gaussian noise; foundation of nonlinear tracking.

### ensemble-kalman-filter (cross-domain alias: `EnKF`, `Monte-Carlo-Kalman`)
**Domain:** Signal Processing / RF
**Definition:** Use ensemble {x_i} of N samples to estimate Kalman covariance: P̂ = (1/(N−1))·Σ(x_i − x̄)(x_i − x̄)ᵀ. Avoids storing full covariance.
**Atom or composite:** Composite: forecast ensemble → analysis with sample covariance.
**Cost model:** O(n·N) for n-dim state and N-member ensemble (typically N ≪ n).
**Real wall?** Yes — finite N introduces sampling noise; requires localization and inflation to prevent collapse in high-dim systems.
**Cross-domain wiring:** Statistics-probability (subspace Bayesian update), ml-training (mini-batch covariance estimation).
**Notes:** Evensen 1994. Standard in weather forecasting and large-scale geophysical data assimilation.

### h-infinity-filter (cross-domain alias: `H∞-filter`, `worst-case-filter`, `minimax-Kalman`)
**Domain:** Signal Processing / RF
**Definition:** Minimize worst-case energy gain from disturbances to estimation error: minₖ max_w ‖e‖²/(‖w‖²+‖v‖²) ≤ γ². Game-theoretic alternative to Kalman.
**Atom or composite:** Composite: Riccati-like recursion with sign-indefinite term.
**Cost model:** O(n³) per step.
**Real wall?** Yes — γ must satisfy existence conditions; trade-off between performance and robustness.
**Cross-domain wiring:** Control-numerical-opt (H∞ control), ml-training (adversarial robustness).
**Notes:** Robust to unknown noise statistics; preferred when noise model is uncertain.

### wiener-filter (cross-domain alias: `optimal-LMS-limit`, `MMSE-filter`)
**Domain:** Signal Processing / RF
**Definition:** FIR filter w* minimizing E[(d[n] − wᵀx[n])²]: solution w* = R_xx⁻¹·r_xd. Yields minimum-MSE estimate under stationarity.
**Atom or composite:** Atom: solve Toeplitz Wiener-Hopf equation.
**Cost model:** O(N²) Levinson-Durbin solve.
**Real wall?** Yes — requires stationarity and known second-order statistics; adaptive (LMS, RLS) variants estimate them online.
**Cross-domain wiring:** Linear-algebra-matrix (normal equations), statistics-probability (orthogonality principle).
**Notes:** Wiener 1949. Theoretical optimum for stationary inputs; adaptive filters converge to Wiener solution.

### wiener-hopf-equation (cross-domain alias: `normal-equations-DSP`, `Wiener-cross-correlation`)
**Domain:** Signal Processing / RF
**Definition:** R_xx·w = r_xd for finite-length FIR Wiener filter. R_xx is autocorrelation Toeplitz, r_xd is cross-correlation vector.
**Atom or composite:** Atom: linear system.
**Cost model:** O(N²) Levinson; O(N³) general solver.
**Real wall?** Yes — solution exists uniquely if R_xx is positive definite (excitation condition).
**Cross-domain wiring:** Ml-training (linear regression normal eqs), retrieval-search (linear classifier solution).
**Notes:** Wiener-Hopf 1931 (continuous causal version). Standard textbook formulation of MMSE FIR filter.

### lms-convergence-bounds (cross-domain alias: `step-size-criteria`, `MSE-misadjustment`)
**Domain:** Signal Processing / RF
**Definition:** LMS converges in mean if 0 < μ < 2/λ_max(R_xx) and in mean-square if 0 < μ < 2/(3·tr(R_xx)). Misadjustment M ≈ μ·tr(R_xx)/2.
**Atom or composite:** Analysis primitive (not algorithm).
**Cost model:** N/A.
**Real wall?** Yes — fundamental trade-off: small μ = slow tracking, large μ = high misadjustment.
**Cross-domain wiring:** Ml-training (SGD step-size theory), control-numerical-opt (Lyapunov stability of gradient methods).
**Notes:** Haykin's adaptive filter theory chapter 5. Tighter bounds via independence assumption.

### nlms-misadjustment-tradeoff (cross-domain alias: `excess-MSE`, `tracking-vs-noise`)
**Domain:** Signal Processing / RF
**Definition:** NLMS misadjustment M ≈ μ/(2 − μ); independent of input statistics. Speed-up over LMS for colored inputs proportional to condition number κ(R_xx).
**Atom or composite:** Analysis primitive.
**Cost model:** N/A.
**Real wall?** Yes — fundamental SNR vs convergence-rate trade.
**Cross-domain wiring:** Ml-training (adaptive learning rates), statistics-probability (estimator variance vs bias).
**Notes:** Slock 1993 derivation. Foundation for variable step-size LMS variants.

### variable-step-size-lms (cross-domain alias: `VSS-LMS`, `gradient-adaptive-step`)
**Domain:** Signal Processing / RF
**Definition:** μ[n+1] = α·μ[n] + γ·e[n]·e[n−1] (or similar). Adapt μ based on recent error correlation.
**Atom or composite:** Composite: LMS + step-size adaptation rule.
**Cost model:** O(N) + O(1) for step update.
**Real wall?** Yes — VSS heuristics introduce free parameters (α, γ, μ_max, μ_min) that themselves must be tuned.
**Cross-domain wiring:** Ml-training (Adam, RMSprop = VSS variants), control-numerical-opt (line search alternatives).
**Notes:** Kwong-Johnston 1992. Many variants exist; common in audio/biomedical filtering.

### block-lms (cross-domain alias: `BLMS`, `frequency-domain-LMS`)
**Domain:** Signal Processing / RF
**Definition:** Update once per block of L samples: w[n+L] = w[n] + (μ/L)·Σ_{k=0}^{L−1} e[n+k]·x[n+k]. Implemented via FFT for efficiency.
**Atom or composite:** Composite: gradient averaging over block → FFT-based convolution.
**Cost model:** O(N log N) per block of L samples (vs O(NL) sample-by-sample).
**Real wall?** Yes — block size L introduces delay = L; convergence behavior similar to LMS but coarser.
**Cross-domain wiring:** Ml-training (mini-batch SGD), graphics-rendering-lod (block-based filtering for GPU efficiency).
**Notes:** Frequency-domain block LMS (Ferrara 1980) is standard for long echo cancellers.

### subband-adaptive-filter (cross-domain alias: `SAF`, `polyphase-LMS`)
**Domain:** Signal Processing / RF
**Definition:** Split input into K subbands → run shorter LMS in each subband → recombine. Decorrelates input across bands.
**Atom or composite:** Composite: analysis filterbank → per-band LMS → synthesis filterbank.
**Cost model:** Roughly O(N/K) per sample plus filterbank cost.
**Real wall?** Yes — aliasing between subbands creates fundamental error floor; perfect-reconstruction filterbanks help.
**Cross-domain wiring:** Graphics-rendering-lod (multi-scale decomposition), ml-training (multi-task learning analogy).
**Notes:** Particularly effective for colored inputs where LMS suffers from eigenvalue spread.

### multi-delay-adaptive-filter (cross-domain alias: `MDF`, `partitioned-block-LMS`)
**Domain:** Signal Processing / RF
**Definition:** Partition long FIR into B short FIR sections; each updated independently with shared error signal. Reduces FFT-block latency.
**Atom or composite:** Composite: B parallel block-LMS instances on partitioned filter.
**Cost model:** O(N log(N/B)) per sample; latency = N/B not N.
**Real wall?** Yes — reduces algorithmic delay but does not change convergence-vs-misadjustment trade-off.
**Cross-domain wiring:** Graphics-rendering-lod (block-tiled rendering), retrieval-search (sharded index lookup).
**Notes:** Soo & Pang 1990. Standard in commercial echo cancellers (frame size = N/B).

### adaptive-line-enhancer (cross-domain alias: `ALE`, `narrowband-extractor`)
**Domain:** Signal Processing / RF
**Definition:** Adaptive filter with delayed input as desired signal: d[n] = x[n], reference = x[n−Δ]. Decorrelates broadband noise from periodic signal.
**Atom or composite:** Composite: delay + LMS predicting current from past.
**Cost model:** O(N) per sample.
**Real wall?** Yes — works only when signal autocorrelation extends beyond Δ but noise autocorrelation does not.
**Cross-domain wiring:** Statistics-probability (correlation-based detection), retrieval-search (template extraction from noise).
**Notes:** Widrow et al. 1975. Classic application for sinusoid extraction from broadband noise.

### blind-equalization-cma (cross-domain alias: `CMA`, `Godard-equalizer`, `constant-modulus`)
**Domain:** Signal Processing / RF
**Definition:** Adapt equalizer to minimize |‖y‖² − R|² where R is constant target modulus. No training sequence needed.
**Atom or composite:** Composite: nonlinear cost → stochastic gradient update.
**Cost model:** O(N) per sample.
**Real wall?** Yes — local minima; converges to phase-ambiguous solution; struggles with non-CM constellations like QAM.
**Cross-domain wiring:** Ml-training (self-supervised loss), information-theory-coding (channel inversion).
**Notes:** Godard 1980, Treichler-Agee 1983. Foundation of blind channel equalization in communications.

---

## Multirate Signal Processing

### decimation (cross-domain alias: `downsample`, `M-fold-decimate`)
**Domain:** Signal Processing / RF
**Definition:** y[n] = x[Mn]; reduces sample rate by integer factor M. Must be preceded by anti-alias LPF with cutoff π/M.
**Atom or composite:** Composite: lowpass filter → downsample.
**Cost model:** O(N/M) output samples; LPF dominates.
**Real wall?** Yes — Nyquist criterion: input bandwidth must be ≤ π/M to avoid aliasing.
**Cross-domain wiring:** Graphics-rendering-lod (mipmap downsample), retrieval-search (coarse-to-fine indexing).
**Notes:** Polyphase decomposition makes implementation efficient by computing only retained output samples.

### interpolation-upsample (cross-domain alias: `upsample`, `L-fold-expand`, `zero-stuff-interp`)
**Domain:** Signal Processing / RF
**Definition:** Insert L−1 zeros between samples → lowpass filter with gain L and cutoff π/L. Result: y[Ln] = x[n], interpolated in between.
**Atom or composite:** Composite: zero-insertion → LPF.
**Cost model:** O(LN) outputs from N inputs.
**Real wall?** Yes — spectral images at k·(2π/L) must be filtered; ideal sinc interp is infinite-tap.
**Cross-domain wiring:** Graphics-rendering-lod (image upscaling), retrieval-search (resolution refinement).
**Notes:** Polyphase interpolator avoids multiplying by inserted zeros.

### rational-rate-conversion (cross-domain alias: `L/M-resample`, `fractional-resample`)
**Domain:** Signal Processing / RF
**Definition:** Convert rate by L/M: upsample by L → LPF at min(π/L, π/M) → decimate by M. Implementable as single polyphase filter.
**Atom or composite:** Composite: upsample → filter → downsample (with merged filter).
**Cost model:** O(N · max(L,M) · K) for length-K filter naive; O(N · K) with polyphase.
**Real wall?** Yes — gcd(L,M)=1 must hold; very large LCM means many polyphase branches.
**Cross-domain wiring:** Graphics-rendering-lod (arbitrary scaling), retrieval-search (cross-rate index merge).
**Notes:** Standard for audio sample-rate conversion (44.1 → 48 kHz uses L/M = 160/147).

### polyphase-decomposition (cross-domain alias: `polyphase-FIR`, `Type-1-polyphase`)
**Domain:** Signal Processing / RF
**Definition:** Decompose filter H(z) = Σ_{k=0}^{M−1} z⁻ᵏ·H_k(zᴹ) where H_k(z) = Σ_m h[mM+k]·z⁻ᵐ.
**Atom or composite:** Atom: index-strided rearrangement.
**Cost model:** O(N/M) MACs per output for decimator (vs O(N) naively).
**Real wall?** Yes — exact mathematical identity; the savings come from moving operations past resamplers (Noble identities).
**Cross-domain wiring:** Linear-algebra-matrix (block-diagonal restructuring), graphics-rendering-lod (block-tile processing).
**Notes:** Vaidyanathan's textbook. Foundation of efficient multirate implementations.

### noble-identities (cross-domain alias: `multirate-equivalence`, `resample-commute`)
**Domain:** Signal Processing / RF
**Definition:** H(zᴹ) followed by ↓M ⟺ ↓M followed by H(z). Similarly ↑L followed by H(zᴸ) ⟺ H(z) followed by ↑L.
**Atom or composite:** Atom: structural rewriting rule.
**Cost model:** Enables polyphase savings.
**Real wall?** No — purely algebraic identity in z-domain.
**Cross-domain wiring:** Graphics-rendering-lod (LOD swap rules), retrieval-search (filter-pushdown optimization in databases).
**Notes:** Crochiere-Rabiner 1981. Generalized to multidimensional via sampling matrices.

### perfect-reconstruction-filterbank (cross-domain alias: `PRFB`, `lossless-analysis-synthesis`)
**Domain:** Signal Processing / RF
**Definition:** Analysis filters {H_k} and synthesis filters {F_k} such that Σ F_k·H_k = z⁻ᴺ (pure delay). Splits and rejoins signal without loss.
**Atom or composite:** Composite: analysis bank + synthesis bank.
**Cost model:** O(N·M) for M-band filterbank with length-N filters; polyphase reduces this.
**Real wall?** Yes — only finite-tap PR filterbanks of length ≥ 2M exist; aliasing-canceling and amplitude-restoring constraints must be satisfied.
**Cross-domain wiring:** Linear-algebra-matrix (paraunitary matrices), information-theory-coding (subband coding).
**Notes:** Vetterli 1989. Foundation of MPEG audio (32-band PR filterbank) and wavelet transforms.

### qmf-filterbank (cross-domain alias: `quadrature-mirror-filter`, `Esteban-Galand`)
**Domain:** Signal Processing / RF
**Definition:** 2-channel filterbank with H_1(z) = H_0(−z), F_0(z) = H_0(z), F_1(z) = −H_0(−z). Cancels aliasing exactly; amplitude distortion remains.
**Atom or composite:** Composite: complementary lowpass/highpass pair.
**Cost model:** O(N) per filter pair.
**Real wall?** Yes — exact PR requires Smith-Barnwell solution (orthogonal QMF) which constrains filter design.
**Cross-domain wiring:** Information-theory-coding (subband coding for compression), graphics-rendering-lod (wavelet pyramids).
**Notes:** Esteban-Galand 1977. Antecedent of orthogonal wavelets (Daubechies' construction).

### cmfb-filterbank (cross-domain alias: `cosine-modulated`, `MDCT-filterbank`)
**Domain:** Signal Processing / RF
**Definition:** M-band filterbank built from single prototype p[n] modulated by cosines: H_k[n] = p[n]·cos((2k+1)·(n−(N−1)/2)·π/(2M) + θ_k).
**Atom or composite:** Composite: prototype design + cosine modulation.
**Cost model:** O(N log M) via fast cosine transform.
**Real wall?** Yes — PR conditions are on prototype only; design reduces to lowpass design.
**Cross-domain wiring:** Information-theory-coding (MDCT in MP3/AAC), linear-algebra-matrix (DCT-IV structure).
**Notes:** Vaidyanathan 1990. Backbone of modern audio codecs.

### farrow-structure (cross-domain alias: `Farrow-interpolator`, `polynomial-fractional-delay`)
**Domain:** Signal Processing / RF
**Definition:** Implement fractional delay μ ∈ [0,1) via FIR coefficients that are polynomials in μ: h[n](μ) = Σ_k c_{n,k}·μᵏ. Output y(t) = Σ_n c_n(μ)·x[n_0 − n].
**Atom or composite:** Composite: M parallel FIRs combined by μ-polynomial.
**Cost model:** O(N·M) for length-N filters and degree-M polynomial.
**Real wall?** Yes — image rejection limited by polynomial degree; arbitrary μ enables continuous time delay.
**Cross-domain wiring:** Graphics-rendering-lod (subpixel resampling), retrieval-search (continuous-time index lookup).
**Notes:** Farrow 1988. Standard for asynchronous sample-rate conversion and timing recovery.

### cic-filter (cross-domain alias: `cascade-integrator-comb`, `Hogenauer`)
**Domain:** Signal Processing / RF
**Definition:** N integrators at high rate + downsample + N combs at low rate. Equivalent to (1 − z⁻ᴿᴹ)ᴺ / (1 − z⁻¹)ᴺ. No multipliers.
**Atom or composite:** Composite: integrators + decimate + combs.
**Cost model:** O(N) adds per sample regardless of decimation R.
**Real wall?** Yes — droop in passband requires compensating FIR; bit-width grows with N·log₂(RM).
**Cross-domain wiring:** Hardware (multiplierless DSP), graphics-rendering-lod (box-filter cascade approximating Gaussian).
**Notes:** Hogenauer 1981. Standard front-end for high-rate ADC decimation.

### halfband-filter (cross-domain alias: `half-band-FIR`, `complementary-FIR`)
**Domain:** Signal Processing / RF
**Definition:** FIR with cutoff π/2 and symmetric specs: |H(π−ω)| = 1 − |H(ω)|. Half the taps are zero (except central tap).
**Atom or composite:** Atom: structured FIR.
**Cost model:** ~N/2 nonzero MACs for length-N filter; ideal for 2× decimation/interpolation.
**Real wall?** Yes — applicable only for ratio-2 conversion; cascading multiple halfbands enables 2ᴺ rate change.
**Cross-domain wiring:** Graphics-rendering-lod (pyramidal scaling), retrieval-search (binary index splits).
**Notes:** Equiripple halfband designed by modified Parks-McClellan.

### fractional-resampling (cross-domain alias: `Lagrange-resample`, `B-spline-resample`)
**Domain:** Signal Processing / RF
**Definition:** Resample at arbitrary continuous offset using polynomial interpolation (Lagrange, B-spline) on neighborhood of input samples.
**Atom or composite:** Composite: polynomial interpolation kernel.
**Cost model:** O(K) per output sample for K-point kernel.
**Real wall?** Yes — kernel order trades passband ripple vs computational cost.
**Cross-domain wiring:** Graphics-rendering-lod (subtexel sampling), control-numerical-opt (numerical interpolation).
**Notes:** Lagrange interpolation = special case of Farrow structure.

### multirate-cascade (cross-domain alias: `cascaded-resampler`, `multistage-decimation`)
**Domain:** Signal Processing / RF
**Definition:** Decimate by large factor M as cascade M₁·M₂·…·M_L with simpler filters at each stage. Total taps reduced when M has many small factors.
**Atom or composite:** Composite: chained decimation stages.
**Cost model:** Often 5-10× cheaper than single-stage equivalent.
**Real wall?** Yes — only when M factorizes well; otherwise polyphase single stage is best.
**Cross-domain wiring:** Graphics-rendering-lod (mipmap chain), retrieval-search (hierarchical search).
**Notes:** Optimal decomposition by dynamic programming or Crochiere-Rabiner heuristic.

### polyphase-filterbank-channelizer (cross-domain alias: `PFB-channelizer`, `WOLA-channelizer`)
**Domain:** Signal Processing / RF
**Definition:** Combine polyphase prototype with M-point FFT to produce M frequency channels each decimated by M. Maximally decimated channelizer.
**Atom or composite:** Composite: polyphase commutator + FFT.
**Cost model:** O(N + M log M) per output block of M samples.
**Real wall?** Yes — adjacent-channel rejection set by prototype filter; FFT alone has high sidelobes.
**Cross-domain wiring:** Information-theory-coding (OFDMA uplink processing), retrieval-search (spectrum tiling for sensing).
**Notes:** Heart of SDR receivers, satellite payloads, and DAB/DVB receivers.

---

## Time-Frequency Analysis

### wigner-ville-distribution (cross-domain alias: `WVD`, `Wigner-Ville`, `quadratic-TF`)
**Domain:** Signal Processing / RF
**Definition:** W_x(t,ω) = ∫ x(t+τ/2)·x*(t−τ/2)·e⁻ʲωτ dτ. Bilinear time-frequency distribution with optimal time-frequency resolution.
**Atom or composite:** Atom: bilinear time-frequency integral.
**Cost model:** O(N²) computation; O(N²) memory for N samples.
**Real wall?** Yes — bilinear nature produces cross-terms between multi-component signals; not generally non-negative.
**Cross-domain wiring:** Linear-algebra-matrix (positive operator decomposition), statistics-probability (signal moment representation).
**Notes:** Wigner 1932 (quantum mechanics), Ville 1948 (signal processing). Foundation of Cohen's class.

### cohen-class-distribution (cross-domain alias: `Cohen-TF`, `kernel-TF`, `generalized-WVD`)
**Domain:** Signal Processing / RF
**Definition:** Family of TF distributions C_x(t,ω) = WVD ⋆⋆ φ(t,ω) where φ is a kernel. Different kernels trade cross-term suppression vs resolution.
**Atom or composite:** Composite: WVD + 2D smoothing kernel.
**Cost model:** O(N²) for filtered WVD.
**Real wall?** Yes — Heisenberg uncertainty bounds time-frequency concentration.
**Cross-domain wiring:** Graphics-rendering-lod (2D filtering), statistics-probability (joint density estimation).
**Notes:** Cohen 1966. Unifies spectrogram, WVD, Choi-Williams, etc.

### choi-williams-distribution (cross-domain alias: `CWD`, `exponential-kernel-TF`)
**Domain:** Signal Processing / RF
**Definition:** Cohen-class distribution with kernel φ(θ,τ) = exp(−θ²·τ²/σ). Suppresses cross-terms while preserving auto-term concentration.
**Atom or composite:** Composite: WVD + exponential 2D smoothing.
**Cost model:** O(N²·log N) via FFTs.
**Real wall?** Yes — σ parameter trades suppression vs resolution.
**Cross-domain wiring:** Statistics-probability (Gaussian smoothing of joint density).
**Notes:** Choi-Williams 1989. Standard for non-stationary signal analysis with multiple components.

### smoothed-pseudo-wigner (cross-domain alias: `SPWVD`, `windowed-WVD`)
**Domain:** Signal Processing / RF
**Definition:** Apply separable smoothing in time and frequency: SPWVD = WVD ⋆_t g(t) ⋆_ω h(ω). Two windows control cross-term suppression independently.
**Atom or composite:** Composite: WVD + time window + frequency window.
**Cost model:** O(N²) via short-time WVD.
**Real wall?** Yes — separable smoothing distorts non-axis-aligned features.
**Cross-domain wiring:** Graphics-rendering-lod (separable 2D blur), statistics-probability (kernel smoothing in 2D).
**Notes:** Most common form of Cohen-class distribution in practice.

### fractional-fourier-transform (cross-domain alias: `FrFT`, `α-rotation-FT`, `chirp-FT`)
**Domain:** Signal Processing / RF
**Definition:** F^α{x}(u) = ∫ K_α(u,t)·x(t)·dt where K_α is a chirp kernel; rotates time-frequency plane by angle α. F^1 = Fourier, F^0 = identity.
**Atom or composite:** Atom: angular-parametric transform.
**Cost model:** O(N log N) via fast discrete FrFT.
**Real wall?** No — purely linear unitary operator, but the choice of α requires knowledge of chirp rate.
**Cross-domain wiring:** Linear-algebra-matrix (rotation in time-frequency plane), graphics-rendering-lod (skewed sampling), ml-training (chirp-feature extraction).
**Notes:** Namias 1980, Almeida 1994. Optimal for chirp signals; used in optics and radar.

### s-transform-stockwell (cross-domain alias: `S-transform`, `Stockwell-transform`)
**Domain:** Signal Processing / RF
**Definition:** S(τ,f) = ∫ x(t)·(|f|/√(2π))·e⁻⁽ᵗ⁻τ⁾²·f²/²·e⁻ʲ²πft dt. STFT with frequency-dependent Gaussian window width.
**Atom or composite:** Composite: STFT with adaptive window.
**Cost model:** O(N² log N) via FFTs.
**Real wall?** Yes — invertibility relies on absolute-integrability conditions; oscillations introduced for sharp transients.
**Cross-domain wiring:** Statistics-probability (adaptive bandwidth kernel), graphics-rendering-lod (multi-scale local Fourier).
**Notes:** Stockwell 1996. Used in geophysics and biomedical signal analysis.

### constant-q-transform (cross-domain alias: `CQT`, `log-frequency-spectrogram`)
**Domain:** Signal Processing / RF
**Definition:** Frequency-domain transform with bins whose Q = f/Δf is constant; geometrically-spaced frequencies. Mimics musical scales.
**Atom or composite:** Composite: filterbank with logarithmic spacing.
**Cost model:** O(N log N) via Brown's efficient algorithm.
**Real wall?** Yes — non-invertible without regularization; bin overlap larger at low frequencies.
**Cross-domain wiring:** Ml-training (music information retrieval features), retrieval-search (audio fingerprinting features).
**Notes:** Brown 1991. Standard for pitch- and chroma-based music analysis.

### hilbert-huang-transform (cross-domain alias: `HHT`, `EMD-Hilbert`)
**Domain:** Signal Processing / RF
**Definition:** Decompose signal into intrinsic mode functions (IMFs) via EMD → Hilbert transform of each IMF → instantaneous frequency.
**Atom or composite:** Composite: EMD + Hilbert spectrum.
**Cost model:** O(N²) for EMD sifting + O(N) per IMF Hilbert.
**Real wall?** Yes — EMD is empirical/algorithmic, not mathematically rigorous; sensitive to noise and endpoint effects.
**Cross-domain wiring:** Statistics-probability (data-driven basis), ml-training (adaptive feature extraction).
**Notes:** Huang et al. 1998. Adaptive alternative to wavelet/Fourier for non-stationary, nonlinear signals.

### empirical-mode-decomposition (cross-domain alias: `EMD`, `IMF-sifting`)
**Domain:** Signal Processing / RF
**Definition:** Iteratively extract IMFs: find local maxima/minima → spline envelopes → subtract mean → repeat until IMF criteria satisfied.
**Atom or composite:** Composite: sifting iteration.
**Cost model:** O(N log N) per IMF (envelope splines); O(N²) total for K IMFs.
**Real wall?** Yes — mode mixing (one mode appearing in multiple IMFs); no theoretical convergence proof.
**Cross-domain wiring:** Statistics-probability (data-driven multi-scale decomposition), ml-training (signal-adaptive basis).
**Notes:** Huang 1998. Foundation of HHT. Stoppages criterion (Cauchy-type) determines IMF count.

### ensemble-emd (cross-domain alias: `EEMD`, `noise-assisted-EMD`)
**Domain:** Signal Processing / RF
**Definition:** Run EMD on x + white-noise multiple times → average IMFs across trials. Mitigates mode mixing.
**Atom or composite:** Composite: noise injection + EMD ensemble averaging.
**Cost model:** O(K) × EMD cost for K trials.
**Real wall?** Yes — added noise must be small enough not to dominate signal but large enough to populate dyadic scales.
**Cross-domain wiring:** Statistics-probability (jittered estimation, bagging), ml-training (noise-augmented training).
**Notes:** Wu-Huang 2009. Standard EMD variant for nonstationary geophysical/biomedical signals.

### variational-mode-decomposition (cross-domain alias: `VMD`, `Wiener-filter-decomp`)
**Domain:** Signal Processing / RF
**Definition:** Decompose into K narrowband modes simultaneously by solving variational problem: min Σ ‖∂_t[(δ(t)+j/πt) ∗ u_k(t)]·e⁻ʲωₖt‖² subject to Σ u_k = x.
**Atom or composite:** Composite: ADMM solution of constrained variational problem.
**Cost model:** O(KN log N) per iteration; converges in tens of iterations.
**Real wall?** Yes — K (number of modes) is hyperparameter; initialization affects center-frequency convergence.
**Cross-domain wiring:** Control-numerical-opt (ADMM splitting), ml-training (variational autoencoder analogy).
**Notes:** Dragomiretskiy-Zosso 2014. More principled than EMD; mathematically founded.

### reassignment-method (cross-domain alias: `reassign-TF`, `Auger-Flandrin`)
**Domain:** Signal Processing / RF
**Definition:** Move TF distribution energy to centroid of local energy distribution: t̂(t,ω) = t + Im(X_th(t,ω)·X*(t,ω)/|X(t,ω)|²).
**Atom or composite:** Composite: standard TF + centroid relocation.
**Cost model:** O(N log N) STFT × 3 (extra derivative STFTs).
**Real wall?** Yes — sharpens but does not improve fundamental Heisenberg bound; sensitive to noise.
**Cross-domain wiring:** Graphics-rendering-lod (image sharpening), statistics-probability (mean-shift relocation).
**Notes:** Auger-Flandrin 1995. Used for high-resolution component extraction in non-stationary signals.

### synchrosqueezing (cross-domain alias: `SST`, `Daubechies-synchrosqueeze`)
**Domain:** Signal Processing / RF
**Definition:** Reassignment confined to frequency axis only → invertible TF representation that sharpens around instantaneous frequencies.
**Atom or composite:** Composite: continuous wavelet/STFT + frequency reassignment.
**Cost model:** O(N log N) via FFT-based wavelet computation.
**Real wall?** Yes — invertibility requires careful kernel choice; mode separation requires components to be spectrally separated.
**Cross-domain wiring:** Ml-training (sharpened features), graphics-rendering-lod (super-resolution).
**Notes:** Daubechies-Lu-Wu 2011. State of the art for mode extraction with reconstruction.

### gabor-frame (cross-domain alias: `STFT-frame`, `Gabor-expansion`)
**Domain:** Signal Processing / RF
**Definition:** {g_{m,n}(t) = g(t−nT)·e^{j2πmFt}} forms a frame if energy bounds A‖x‖² ≤ Σ|⟨x,g_{m,n}⟩|² ≤ B‖x‖² hold. Stable analysis/synthesis.
**Atom or composite:** Atom: structured time-frequency frame.
**Cost model:** O(N log N) frame transform.
**Real wall?** Yes — Balian-Low theorem: orthonormal Gabor basis cannot have both g and ĝ well-localized.
**Cross-domain wiring:** Linear-algebra-matrix (frame theory), retrieval-search (overcomplete dictionaries).
**Notes:** Gabor 1946. Generalized by frame theory (Daubechies-Grossmann-Meyer).

### chirp-z-transform (cross-domain alias: `CZT`, `Bluestein`, `zoom-FT`)
**Domain:** Signal Processing / RF
**Definition:** Evaluate z-transform along arbitrary contour z_k = A·Wᵏ. Generalizes DFT (which uses unit circle uniform spacing).
**Atom or composite:** Composite: chirp multiply → convolution → chirp multiply (Bluestein's algorithm).
**Cost model:** O(N log N) via Bluestein.
**Real wall?** No — exact for chosen contour but only at chosen sample points.
**Cross-domain wiring:** Linear-algebra-matrix (generalized eigenvalue problem), retrieval-search (arbitrary-grid spectrum).
**Notes:** Used for zoom-FFT to resolve fine frequency detail in narrow bands.

### modulated-lapped-transform (cross-domain alias: `MLT`, `lapped-orthogonal-transform`)
**Domain:** Signal Processing / RF
**Definition:** Cosine-modulated lapped orthogonal transform with overlap-add reconstruction. Essentially the MDCT used in audio codecs.
**Atom or composite:** Composite: window + DCT-IV variant + overlap-add.
**Cost model:** O(N log N) per window of N samples.
**Real wall?** Yes — TDAC (time-domain aliasing cancellation) is the structural constraint that yields orthogonality.
**Cross-domain wiring:** Information-theory-coding (audio codecs MP3/AAC), graphics-rendering-lod (lapped image transforms).
**Notes:** Malvar 1990. Heart of perceptual audio codecs.

---

## Wavelets (Advanced)

### daubechies-wavelet (cross-domain alias: `dbN`, `compact-orthogonal-wavelet`)
**Domain:** Signal Processing / RF
**Definition:** Family of orthogonal wavelets with compact support 2N−1, N vanishing moments. Constructed from minimum-phase factorization of half-band filter.
**Atom or composite:** Composite: half-band filter design + spectral factorization.
**Cost model:** O(N) per level; O(N log N) total tree.
**Real wall?** Yes — Daubechies' impossibility: cannot have orthogonality, compact support, symmetry, and smoothness all simultaneously (Daubechies are asymmetric).
**Cross-domain wiring:** Linear-algebra-matrix (orthonormal basis), information-theory-coding (sparse representation).
**Notes:** Daubechies 1988. Foundation of practical orthogonal wavelets. db4, db6 common.

### symlet-wavelet (cross-domain alias: `Symlet`, `near-symmetric-Daubechies`)
**Domain:** Signal Processing / RF
**Definition:** Modified Daubechies wavelets chosen to maximize symmetry. Same vanishing moments and support length as dbN but more symmetric.
**Atom or composite:** Composite: alternative spectral factorization root choice.
**Cost model:** O(N) per level.
**Real wall?** Yes — strict symmetry impossible for orthogonal compact wavelets (only Haar is symmetric).
**Cross-domain wiring:** Graphics-rendering-lod (less phase distortion in image processing).
**Notes:** "Least asymmetric" Daubechies variant; preferred for image compression where phase matters.

### coiflet-wavelet (cross-domain alias: `Coiflet`, `coifN`, `vanishing-moments-scaling`)
**Domain:** Signal Processing / RF
**Definition:** Wavelet family where both wavelet ψ and scaling φ have N vanishing moments. Better for numerical analysis applications.
**Atom or composite:** Composite: Daubechies-style construction with scaling-function constraint.
**Cost model:** O(N) per level; longer support than dbN.
**Real wall?** Yes — extra vanishing moments for φ requires longer support length.
**Cross-domain wiring:** Control-numerical-opt (numerical integration), graphics-rendering-lod (zero-order term suppression).
**Notes:** Coifman 1989. Used when scaling-function moments must vanish (e.g., quadrature applications).

### biorthogonal-wavelet (cross-domain alias: `CDF-wavelet`, `bior`, `dual-basis-wavelet`)
**Domain:** Signal Processing / RF
**Definition:** Two dual bases {ψ_{j,k}} and {ψ̃_{j,k}} with ⟨ψ_{j,k}, ψ̃_{j',k'}⟩ = δ_{jj'}·δ_{kk'}. Allows symmetric (linear-phase) wavelets.
**Atom or composite:** Composite: analysis filter + synthesis filter that satisfy PR.
**Cost model:** O(N) per level.
**Real wall?** Yes — orthogonality is relaxed; energy is not preserved between coefficient and signal domains.
**Cross-domain wiring:** Linear-algebra-matrix (dual frame), graphics-rendering-lod (JPEG2000 uses CDF 9/7).
**Notes:** Cohen-Daubechies-Feauveau (CDF) 9/7 is the JPEG2000 standard.

### wavelet-packet (cross-domain alias: `WP`, `tree-structured-wavelet`)
**Domain:** Signal Processing / RF
**Definition:** Apply wavelet decomposition to both low- and high-pass branches → tree of subbands at all dyadic frequencies/times.
**Atom or composite:** Composite: tree of QMF filterbanks.
**Cost model:** O(N log N) full tree; best-basis selection adds O(N log N).
**Real wall?** Yes — best-basis algorithm (Coifman-Wickerhauser) finds optimal partition in O(N log N).
**Cross-domain wiring:** Information-theory-coding (entropy-coded subband compression), retrieval-search (multi-scale tree search).
**Notes:** Better frequency localization than wavelet at cost of fixed time-frequency cells.

### lifting-scheme (cross-domain alias: `lifting-wavelet`, `Sweldens`)
**Domain:** Signal Processing / RF
**Definition:** Build wavelet by alternating predict and update steps: d[n] = x[2n+1] − P(x_even); s[n] = x[2n] + U(d). In-place, integer-to-integer mappable.
**Atom or composite:** Composite: predict + update lifting steps.
**Cost model:** O(N) per level; ~half the work of standard wavelet implementation.
**Real wall?** No — any FIR wavelet has a lifting factorization (Daubechies-Sweldens theorem).
**Cross-domain wiring:** Information-theory-coding (lossless integer wavelets for JPEG2000), graphics-rendering-lod (in-place pyramid construction).
**Notes:** Sweldens 1995. Standard for hardware-efficient and lossless wavelet implementations.

### dual-tree-complex-wavelet (cross-domain alias: `DT-CWT`, `Kingsbury-wavelet`)
**Domain:** Signal Processing / RF
**Definition:** Two parallel real wavelet trees whose outputs form approximate Hilbert pair → near shift-invariant, directionally selective.
**Atom or composite:** Composite: two wavelet trees with quarter-sample delay.
**Cost model:** 2× cost of standard wavelet; O(N log N) total.
**Real wall?** Yes — perfect shift invariance impossible for critically sampled wavelets (aliasing).
**Cross-domain wiring:** Graphics-rendering-lod (directional image features), ml-training (shift-invariant features).
**Notes:** Kingsbury 1998. 6-direction selectivity in 2D; standard for image denoising and texture analysis.

### curvelet-transform (cross-domain alias: `curvelet`, `multiscale-directional`)
**Domain:** Signal Processing / RF
**Definition:** Anisotropic scaling (width ≈ length²) plus rotation: optimal for representing curve-like singularities in 2D.
**Atom or composite:** Composite: bandpass filtering + angular tiling + ridgelet-like representation per band.
**Cost model:** O(N² log N) for N×N image.
**Real wall?** Yes — optimal m-term approximation rate for cartoon images (functions smooth except along curves).
**Cross-domain wiring:** Graphics-rendering-lod (edge-preserving compression), retrieval-search (curve-feature retrieval).
**Notes:** Candès-Donoho 2000. Outperforms wavelets on edge-rich images.

### contourlet-transform (cross-domain alias: `contourlet`, `pyramidal-directional-filterbank`)
**Domain:** Signal Processing / RF
**Definition:** Discrete domain alternative to curvelets: Laplacian pyramid + directional filterbank at each scale.
**Atom or composite:** Composite: Laplacian pyramid + DFB.
**Cost model:** O(N²) for image.
**Real wall?** Yes — discrete directional filtering causes aliasing without nonsubsampled variants.
**Cross-domain wiring:** Graphics-rendering-lod (directional features), ml-training (sparse coding dictionaries).
**Notes:** Do-Vetterli 2005. More implementation-friendly than curvelet; nonsubsampled version (NSCT) is shift-invariant.

### shearlet-transform (cross-domain alias: `shearlet`, `composite-dilation-wavelet`)
**Domain:** Signal Processing / RF
**Definition:** Shearlet generators ψ_{a,s,t}(x) = a⁻³ᐟ⁴·ψ(A_a·S_s·(x−t)) where A_a is anisotropic dilation and S_s is shear matrix.
**Atom or composite:** Composite: parabolic scaling + shear + translation.
**Cost model:** O(N²·log N) for N×N image.
**Real wall?** Yes — provides optimally sparse representation for cartoon-like images with discrete-domain implementation.
**Cross-domain wiring:** Graphics-rendering-lod (multi-direction edge representation), ml-training (directional dictionary learning).
**Notes:** Labate-Lim-Kutyniok 2005. Cleaner discrete implementation than curvelets.

### ridgelet-transform (cross-domain alias: `ridgelet`, `Radon-wavelet`)
**Domain:** Signal Processing / RF
**Definition:** Ridgelet = wavelet of Radon transform. Sparse for piecewise smooth functions along lines.
**Atom or composite:** Composite: Radon transform + 1D wavelet.
**Cost model:** O(N²) for N×N image (dominated by Radon).
**Real wall?** Yes — discrete Radon transform is non-trivial (Gridding, slant-stack).
**Cross-domain wiring:** Linear-algebra-matrix (Radon = projection-slice), retrieval-search (line-feature dictionary).
**Notes:** Candès 1998. Building block of curvelets; preferred for line-rich images.

### dyadic-wavelet-transform (cross-domain alias: `DWT-undecimated`, `Mallat-Zhong`)
**Domain:** Signal Processing / RF
**Definition:** Wavelet transform without decimation between scales — shift-invariant. Aka stationary wavelet transform (SWT) or "à trous" algorithm.
**Atom or composite:** Composite: filter at each scale with hole-stuffing (à trous).
**Cost model:** O(N log N) total but L-times redundant for L levels.
**Real wall?** Yes — overcomplete representation; aliasing absent.
**Cross-domain wiring:** Graphics-rendering-lod (shift-invariant pyramids), retrieval-search (shift-invariant features).
**Notes:** Holschneider-Tchamitchian 1989. Preferred for denoising (TI-denoising) and feature extraction.

### lifting-integer-wavelet (cross-domain alias: `int-to-int-wavelet`, `IDWT-lossless`)
**Domain:** Signal Processing / RF
**Definition:** Lifting steps with rounding produce reversible integer→integer mapping. Enables lossless wavelet compression.
**Atom or composite:** Composite: lifting + rounding before storing.
**Cost model:** O(N) per level.
**Real wall?** Yes — rounding introduces small nonlinearity but reversibility is exact.
**Cross-domain wiring:** Information-theory-coding (JPEG2000 reversible mode), retrieval-search (lossless index encoding).
**Notes:** Calderbank-Daubechies-Sweldens 1998. Used wherever exact reconstruction is required.

---

## Spectral Estimation

### welch-periodogram (cross-domain alias: `Welch`, `WOSA`, `averaged-periodogram`)
**Domain:** Signal Processing / RF
**Definition:** Average modified periodograms of K overlapping windowed segments: P̂(ω) = (1/K)·Σ |W(ω)·X_k(ω)|².
**Atom or composite:** Composite: segment → window → FFT → magnitude² → average.
**Cost model:** O(K·N log N) for K segments of N samples.
**Real wall?** Yes — variance reduction at cost of frequency resolution; bias-variance tradeoff via window.
**Cross-domain wiring:** Statistics-probability (sample averaging for variance reduction), ml-training (bagging analogy).
**Notes:** Welch 1967. Default spectral estimator in scientific computing.

### bartlett-periodogram (cross-domain alias: `Bartlett`, `non-overlap-average`)
**Domain:** Signal Processing / RF
**Definition:** Average periodograms of K non-overlapping length-N segments. Special case of Welch with no overlap and rectangular window.
**Atom or composite:** Composite: non-overlapping → FFT → average.
**Cost model:** O(K·N log N).
**Real wall?** Yes — non-overlapping segments waste signal; rectangular window has high sidelobes.
**Cross-domain wiring:** Statistics-probability (independent estimator averaging).
**Notes:** Bartlett 1948. Predecessor of Welch; rarely used today.

### blackman-tukey-spectrum (cross-domain alias: `BT-spectrum`, `windowed-correlogram`)
**Domain:** Signal Processing / RF
**Definition:** Estimate autocorrelation R̂(k) → window with w[k] → FFT to get spectrum estimate. P̂(ω) = Σ R̂(k)·w[k]·e⁻ʲωᵏ.
**Atom or composite:** Composite: autocorrelation → window → FFT.
**Cost model:** O(N log N) via FFT-based autocorrelation.
**Real wall?** Yes — autocorrelation lag window controls bias-variance; max lag must be ≪ N.
**Cross-domain wiring:** Statistics-probability (kernel density estimation), ml-training (covariance estimation).
**Notes:** Blackman-Tukey 1958. Foundational nonparametric spectral estimator.

### multitaper-thomson (cross-domain alias: `MTM`, `Slepian-spectrum`, `DPSS-spectrum`)
**Domain:** Signal Processing / RF
**Definition:** Average K periodograms using K DPSS (Slepian) tapers: P̂(ω) = (1/K)·Σ |X_k(ω)|², where X_k is FFT of x times k-th DPSS.
**Atom or composite:** Composite: K orthogonal tapers → K FFTs → average.
**Cost model:** O(K·N log N) per spectrum; K = 2NW−1 for time-bandwidth product NW.
**Real wall?** Yes — Slepian tapers maximize energy concentration in W; K limits frequency resolution.
**Cross-domain wiring:** Linear-algebra-matrix (prolate eigenfunctions), statistics-probability (variance-reducing orthogonal estimators).
**Notes:** Thomson 1982. Best-in-class for spectra with both broadband and line components.

### music-algorithm (cross-domain alias: `MUSIC`, `multiple-signal-classification`)
**Domain:** Signal Processing / RF
**Definition:** Eigendecomposition of covariance R = U_s·Λ_s·U_sᴴ + U_n·Λ_n·U_nᴴ → MUSIC spectrum P(ω) = 1/‖U_nᴴ·a(ω)‖². Peaks at signal frequencies/DOAs.
**Atom or composite:** Composite: eigendecomposition + pseudo-spectrum sweep.
**Cost model:** O(N³) for eigendecomposition; O(M·G) for spectrum sweep.
**Real wall?** Yes — requires knowing signal subspace dimension; coherent signals require spatial smoothing.
**Cross-domain wiring:** Linear-algebra-matrix (subspace methods), retrieval-search (low-rank + projection).
**Notes:** Schmidt 1979. Foundation of high-resolution spectral estimation and DOA.

### root-music (cross-domain alias: `root-MUSIC`, `polynomial-MUSIC`)
**Domain:** Signal Processing / RF
**Definition:** Replace MUSIC spectrum sweep with polynomial rooting: form q(z) = aᴴ(z)·U_n·U_nᴴ·a(z), find roots closest to unit circle.
**Atom or composite:** Composite: subspace + polynomial root finding.
**Cost model:** O(N³) eigendecomposition + O(M³) for root finding.
**Real wall?** Yes — only applicable to uniform linear arrays / uniform sampling.
**Cross-domain wiring:** Linear-algebra-matrix (Vandermonde structure), retrieval-search (root-based search).
**Notes:** Barabell 1983. Higher resolution than spectrum-based MUSIC; rooted methods are more accurate.

### esprit-algorithm (cross-domain alias: `ESPRIT`, `rotational-invariance-DOA`)
**Domain:** Signal Processing / RF
**Definition:** Exploit shift invariance of array geometry: U_s,1 = U_s,2·Ψ. Frequencies/DOAs are eigenvalues of Ψ.
**Atom or composite:** Composite: SVD + generalized eigenvalue.
**Cost model:** O(N³) for SVD + O(M³) for eigenvalue.
**Real wall?** Yes — requires array with translational invariance (two identical subarrays).
**Cross-domain wiring:** Linear-algebra-matrix (generalized eigenvalue), control-numerical-opt (rotational invariance principle).
**Notes:** Roy-Kailath 1989. Often outperforms MUSIC; lower computational cost.

### min-norm-spectrum (cross-domain alias: `min-norm`, `Kumaresan-Tufts`)
**Domain:** Signal Processing / RF
**Definition:** Choose vector d with minimum norm in noise subspace such that first element is 1 → P(ω) = 1/|aᴴ(ω)·d|².
**Atom or composite:** Composite: subspace projection + minimum-norm extraction.
**Cost model:** O(N³) eigendecomposition.
**Real wall?** Yes — sensitive to subspace dimension; better than MUSIC for some short-data cases.
**Cross-domain wiring:** Linear-algebra-matrix (minimum-norm solution to underdetermined system).
**Notes:** Kumaresan-Tufts 1983. Alternative high-resolution spectral estimator.

### pisarenko-harmonic (cross-domain alias: `Pisarenko`, `eigenvalue-frequency`)
**Domain:** Signal Processing / RF
**Definition:** For p sinusoids in white noise: smallest eigenvector of (p+1)×(p+1) autocorrelation matrix has roots at signal frequencies.
**Atom or composite:** Composite: eigendecomposition + polynomial rooting.
**Cost model:** O(p³).
**Real wall?** Yes — assumes exactly known number of sinusoids; very sensitive to noise model assumption.
**Cross-domain wiring:** Linear-algebra-matrix (smallest-eigenvector method).
**Notes:** Pisarenko 1973. Historical precursor to MUSIC; superseded by it but conceptually important.

### capon-mvdr (cross-domain alias: `Capon`, `MVDR-spectrum`, `min-variance-spectrum`)
**Domain:** Signal Processing / RF
**Definition:** P_Capon(ω) = 1/(aᴴ(ω)·R⁻¹·a(ω)). Minimum-variance distortionless response to direction ω.
**Atom or composite:** Composite: covariance inverse + quadratic form.
**Cost model:** O(N³) for R⁻¹; O(N²) per frequency point.
**Real wall?** Yes — sensitive to covariance estimation; diagonal loading needed for robustness.
**Cross-domain wiring:** Linear-algebra-matrix (constrained quadratic min), retrieval-search (matched-filter with whitening).
**Notes:** Capon 1969. Foundation of adaptive beamforming.

### quinn-fernandes-estimator (cross-domain alias: `Quinn-Fernandes`, `iterative-frequency`)
**Domain:** Signal Processing / RF
**Definition:** Iterative refinement of single-tone frequency via FFT-interpolated parabolic estimator with correction term.
**Atom or composite:** Composite: FFT + parabolic interpolation + correction.
**Cost model:** O(N log N) FFT + O(K) iterations of O(1) refinement.
**Real wall?** Yes — converges to ML estimate for single tone; assumes single sinusoid.
**Cross-domain wiring:** Control-numerical-opt (iterative refinement of estimators).
**Notes:** Quinn-Fernandes 1991. Sub-bin frequency accuracy from FFT.

### jacobsen-frequency-estimator (cross-domain alias: `Jacobsen`, `parabolic-FFT-interp`)
**Domain:** Signal Processing / RF
**Definition:** Estimate sinusoid frequency by parabolic interpolation across 3 FFT bins around peak: δ = Re((X[k−1]−X[k+1])/(2X[k]−X[k−1]−X[k+1])).
**Atom or composite:** Atom: 3-point spectral interpolation.
**Cost model:** O(1) per estimate after FFT.
**Real wall?** Yes — biased for low SNR; Jacobsen-Kootsookos derived bias correction.
**Cross-domain wiring:** Graphics-rendering-lod (subpixel peak localization), retrieval-search (sub-bin position estimation).
**Notes:** Standard technique in tone-detection and instrument tuning.

### macleod-estimator (cross-domain alias: `Macleod`, `weighted-spectral-interpolation`)
**Domain:** Signal Processing / RF
**Definition:** Optimal interpolator for the 3-bin parabolic frequency-estimation problem for windowed signals.
**Atom or composite:** Atom: weighted three-bin formula.
**Cost model:** O(1) per estimate.
**Real wall?** Yes — minimizes interpolation variance for given window.
**Cross-domain wiring:** Statistics-probability (minimum-variance estimator), retrieval-search (peak position refinement).
**Notes:** Macleod 1998. Improvement over Quinn-Fernandes for short data.

### parametric-spectral-estimation (cross-domain alias: `AR-spectrum`, `model-based-spectrum`)
**Domain:** Signal Processing / RF
**Definition:** Fit AR/ARMA model to data → spectrum from model coefficients. Higher resolution than nonparametric for short data.
**Atom or composite:** Composite: model estimation + spectrum computation.
**Cost model:** O(p²) Levinson + O(p) per frequency.
**Real wall?** Yes — model-order selection (AIC/MDL) critical; mismodel induces spurious peaks.
**Cross-domain wiring:** Statistics-probability (model-based inference), ml-training (parametric vs nonparametric).
**Notes:** Standard high-resolution alternative to FFT-based methods.

### aic-mdl-order-selection (cross-domain alias: `AIC`, `MDL`, `BIC`, `model-order-criteria`)
**Domain:** Signal Processing / RF
**Definition:** AIC(p) = N·log(σ²_p) + 2p; MDL(p) = N·log(σ²_p) + p·log(N). Choose p minimizing the criterion.
**Atom or composite:** Atom: cost-function evaluation.
**Cost model:** O(1) per order after model fit.
**Real wall?** Yes — AIC overestimates order asymptotically; MDL/BIC are consistent but conservative.
**Cross-domain wiring:** Statistics-probability (model selection theory), ml-training (regularization).
**Notes:** Akaike 1974, Rissanen 1978. Standard for AR/ARMA model selection.

### cyclostationary-analysis (cross-domain alias: `spectral-correlation`, `Gardner-cyclic`)
**Domain:** Signal Processing / RF
**Definition:** Signals whose statistics are periodic: R(t+T, τ) = R(t, τ). Spectral correlation function S(α, f) = Fourier transform of cyclic autocorrelation R(α, τ).
**Atom or composite:** Composite: cyclic autocorrelation → 2D Fourier transform.
**Cost model:** O(N²) for time-smoothed cyclic periodogram.
**Real wall?** Yes — cyclic frequencies α must match modulation/symbol rates; useful for signal recognition.
**Cross-domain wiring:** Statistics-probability (non-stationary processes), retrieval-search (modulation classification features).
**Notes:** Gardner 1990. Foundation of spectrum sensing in cognitive radio and modulation classification.

---

## Array Processing & Beamforming

### delay-sum-beamform (cross-domain alias: `DSB`, `conventional-beamformer`)
**Domain:** Signal Processing / RF
**Definition:** y(t) = Σ_m x_m(t − τ_m); compensate per-sensor delays to steer beam in look direction. Equivalent to spatial matched filter.
**Atom or composite:** Atom: spatial matched-filter sum.
**Cost model:** O(M) per output sample for M sensors.
**Real wall?** Yes — fixed beam pattern; sidelobes determined by array geometry (uniform → sinc-like).
**Cross-domain wiring:** Linear-algebra-matrix (vector inner product), graphics-rendering-lod (sum-pooling).
**Notes:** Simplest beamformer; foundation of all spatial filtering. SNR gain = M.

### mvdr-beamform (cross-domain alias: `MVDR`, `Capon-beamform`, `minimum-variance-distortionless`)
**Domain:** Signal Processing / RF
**Definition:** w_MVDR = R⁻¹·a(θ)/(aᴴ(θ)·R⁻¹·a(θ)). Minimize output power subject to unit gain in look direction.
**Atom or composite:** Composite: covariance inverse + constraint.
**Cost model:** O(M³) for R⁻¹, O(M²) per direction.
**Real wall?** Yes — sensitive to steering-vector errors; diagonal loading R + γI improves robustness.
**Cross-domain wiring:** Linear-algebra-matrix (constrained quadratic programming), control-numerical-opt (Lagrange multipliers).
**Notes:** Capon 1969. Standard adaptive beamformer; foundation of MUSIC/Capon spectrum.

### lcmv-beamform (cross-domain alias: `LCMV`, `linearly-constrained-min-var`)
**Domain:** Signal Processing / RF
**Definition:** Minimize wᴴRw subject to Cᴴw = f. Multiple linear constraints (e.g., null in interference direction, gain in look direction).
**Atom or composite:** Composite: MVDR with multiple constraints.
**Cost model:** O(M³) for R⁻¹; O(M·K²) for K constraints.
**Real wall?** Yes — degrees of freedom = M − K; over-constraining destroys SNR gain.
**Cross-domain wiring:** Linear-algebra-matrix (constrained least squares), control-numerical-opt (KKT conditions).
**Notes:** Frost 1972 (constrained LMS). Generalization of MVDR for robust adaptive arrays.

### gsc-beamform (cross-domain alias: `GSC`, `generalized-sidelobe-canceller`, `Griffiths-Jim`)
**Domain:** Signal Processing / RF
**Definition:** Decompose LCMV beamformer into fixed beamformer + blocking matrix + adaptive sidelobe canceller. Implements LCMV in unconstrained form.
**Atom or composite:** Composite: fixed beamformer parallel with constraint-blocking adaptive path.
**Cost model:** Per-sample: O(M) for FBF + O(M−K) for adaptive.
**Real wall?** Yes — blocking matrix imperfection causes "signal leakage" → desired signal cancellation.
**Cross-domain wiring:** Linear-algebra-matrix (null-space decomposition), ml-training (orthogonal projection + LMS).
**Notes:** Griffiths-Jim 1982. Most common implementation of constrained adaptive beamforming.

### null-steering (cross-domain alias: `null-placement`, `interference-cancellation-spatial`)
**Domain:** Signal Processing / RF
**Definition:** Place beamformer nulls at known interferer DOAs: w = (I − a_I·a_Iᴴ/(a_Iᴴa_I))·a_d.
**Atom or composite:** Composite: projection onto orthogonal complement of interferers.
**Cost model:** O(M·K) for K interferers.
**Real wall?** Yes — number of nulls ≤ M − 1 (degrees of freedom limit).
**Cross-domain wiring:** Linear-algebra-matrix (orthogonal projection), retrieval-search (negative selection).
**Notes:** Standard non-adaptive interference suppression when interferer DOAs are known.

### dolph-chebyshev-array (cross-domain alias: `Dolph-Chebyshev-weighting`, `equiripple-sidelobe-array`)
**Domain:** Signal Processing / RF
**Definition:** Choose array weights so that array factor is Chebyshev polynomial → equiripple sidelobes at chosen level → minimum mainlobe width.
**Atom or composite:** Atom: deterministic taper from Chebyshev polynomial.
**Cost model:** O(M) precompute.
**Real wall?** Yes — optimal in Chebyshev sense for fixed sidelobe level.
**Cross-domain wiring:** Linear-algebra-matrix (Chebyshev expansion), retrieval-search (selectivity-margin trade).
**Notes:** Dolph 1946. Array-design analog of Dolph-Chebyshev window.

### taylor-weighting (cross-domain alias: `Taylor-array`, `near-uniform-sidelobe`)
**Domain:** Signal Processing / RF
**Definition:** Modified Dolph-Chebyshev with finite-distance sidelobe decay: only first n̄ sidelobes equal, beyond which they decay.
**Atom or composite:** Composite: modified Chebyshev polynomial roots.
**Cost model:** O(M).
**Real wall?** Yes — trade between mainlobe width and pattern decay beyond n̄-th sidelobe.
**Cross-domain wiring:** Graphics-rendering-lod (apodization for finite arrays), control-numerical-opt (constrained polynomial design).
**Notes:** Taylor 1955. Standard in radar/antenna design where far-out sidelobes can be relaxed.

### frost-beamformer (cross-domain alias: `Frost-LMS`, `constrained-LMS-array`)
**Domain:** Signal Processing / RF
**Definition:** LMS-style adaptive update projected onto constraint surface Cᴴw = f at each step.
**Atom or composite:** Composite: LMS + constraint projection.
**Cost model:** O(M·L) per sample for M sensors and L taps.
**Real wall?** Yes — accumulation errors require periodic constraint reprojection.
**Cross-domain wiring:** Control-numerical-opt (projected gradient method), ml-training (constrained SGD).
**Notes:** Frost 1972. Real-time adaptive beamforming with arbitrary linear constraints.

### howells-applebaum (cross-domain alias: `Applebaum-array`, `sidelobe-canceller-SNR`)
**Domain:** Signal Processing / RF
**Definition:** Maximize SNR by w = μ·R⁻¹·s where s is signal steering vector. Howells-Applebaum loop computes this adaptively.
**Atom or composite:** Composite: covariance inverse + steering vector.
**Cost model:** O(M³) for inverse; O(M²) per output.
**Real wall?** Yes — assumes perfect interference plus noise covariance; sample matrix inversion requires K > 2M snapshots.
**Cross-domain wiring:** Linear-algebra-matrix (max-SNR generalized eigenvector), control-numerical-opt (Rayleigh quotient).
**Notes:** Howells 1965, Applebaum 1976. Original adaptive array.

### spatial-smoothing (cross-domain alias: `forward-backward-averaging`, `coherent-source-decorr`)
**Domain:** Signal Processing / RF
**Definition:** Subarray-average covariance matrices to decorrelate coherent signals: R_FB = (1/L)·Σ R_l where R_l is subarray l's covariance.
**Atom or composite:** Composite: subarray decomposition + averaging.
**Cost model:** O(L·M²).
**Real wall?** Yes — reduces effective array aperture from M to M−L+1.
**Cross-domain wiring:** Linear-algebra-matrix (Toeplitz averaging), statistics-probability (decorrelation via averaging).
**Notes:** Evans 1981. Standard preprocessing for MUSIC/ESPRIT under multipath/coherent sources.

### ml-doa (cross-domain alias: `ML-DOA`, `maximum-likelihood-array`)
**Domain:** Signal Processing / RF
**Definition:** Maximize p(X|θ) over DOA parameters: θ̂ = argmax_θ tr(P_A(θ)·R̂). Optimal under Gaussian assumption.
**Atom or composite:** Composite: nonlinear multidimensional optimization.
**Cost model:** O(M³) per parameter evaluation × search complexity.
**Real wall?** Yes — non-convex; needs initial guess from MUSIC/ESPRIT. Cramer-Rao bound is achievable.
**Cross-domain wiring:** Ml-training (likelihood maximization), control-numerical-opt (nonlinear least squares).
**Notes:** Stoica-Nehorai 1989. Statistically efficient (achieves CRB) but expensive; benchmark for comparison.

### mode-doa (cross-domain alias: `MODE`, `weighted-subspace-fitting`)
**Domain:** Signal Processing / RF
**Definition:** Asymptotically ML method: fit signal subspace using weighted subspace-fit cost. Computationally cheaper than full ML.
**Atom or composite:** Composite: subspace fitting + Newton iteration.
**Cost model:** O(M³) eigendecomposition + O(K) iterations.
**Real wall?** Yes — local minima possible; asymptotic efficiency requires large sample size.
**Cross-domain wiring:** Linear-algebra-matrix (subspace fitting), ml-training (matrix factorization).
**Notes:** Stoica-Sharman 1990. Practical alternative to ML-DOA.

### gcc-phat (cross-domain alias: `GCC-PHAT`, `phase-transform-correlation`, `TDOA`)
**Domain:** Signal Processing / RF
**Definition:** Cross-correlation in frequency domain with magnitude whitening: R̂(τ) = ∫ X_1·X_2*/|X_1·X_2| ·e^{jωτ} dω.
**Atom or composite:** Composite: cross-power spectrum + phase normalization + IDFT.
**Cost model:** O(N log N).
**Real wall?** Yes — phase-only whitening is sensitive to noise at low SNR; mainlobe sharpened at cost of noise enhancement.
**Cross-domain wiring:** Information-theory-coding (mutual information localization), retrieval-search (template matching with phase-only).
**Notes:** Knapp-Carter 1976. Standard for acoustic source localization and TDOA estimation.

### monopulse-tracking (cross-domain alias: `monopulse`, `sum-difference-array`)
**Domain:** Signal Processing / RF
**Definition:** Two-beam (sum Σ, difference Δ) angle measurement: angle error ε ∝ Δ/Σ. Single-pulse angle estimation.
**Atom or composite:** Composite: two beamformers + ratio.
**Cost model:** O(M) per pulse.
**Real wall?** Yes — ratio Δ/Σ is sensitive to amplitude and phase matching of channels.
**Cross-domain wiring:** Control-numerical-opt (gradient sensing), retrieval-search (continuous interpolation between hypotheses).
**Notes:** Used in radar tracking, missile seeker, antenna pointing.

### array-calibration (cross-domain alias: `array-cal`, `manifold-correction`)
**Domain:** Signal Processing / RF
**Definition:** Estimate per-element gain/phase errors from known reference sources, correct steering vector: a_cal(θ) = G·a(θ) with G diagonal complex.
**Atom or composite:** Composite: reference measurements + least-squares fit.
**Cost model:** O(M²·K) for K calibration sources.
**Real wall?** Yes — calibration errors propagate to DOA bias; mutual coupling further complicates.
**Cross-domain wiring:** Statistics-probability (sensor calibration), control-numerical-opt (model error compensation).
**Notes:** Vital for high-resolution methods (MUSIC); even 1° phase error degrades resolution significantly.

---

## MIMO Communications

### mimo-capacity (cross-domain alias: `MIMO-Shannon`, `Foschini-capacity`)
**Domain:** Signal Processing / RF
**Definition:** C = log₂ det(I + (ρ/N_t)·HHᴴ) bits/s/Hz where H is N_r × N_t channel matrix, ρ is SNR.
**Atom or composite:** Atom: log-det formula.
**Cost model:** O(N³) for det via Cholesky.
**Real wall?** Yes — fundamental Shannon limit; achievable with Gaussian inputs and full CSI at TX.
**Cross-domain wiring:** Information-theory-coding (Shannon capacity), linear-algebra-matrix (Hermitian determinant).
**Notes:** Telatar 1995, Foschini 1996. Capacity scales linearly with min(N_t, N_r).

### svd-precoding (cross-domain alias: `SVD-MIMO`, `eigen-beamforming`)
**Domain:** Signal Processing / RF
**Definition:** H = U·Σ·Vᴴ. Precode by V, post-filter by Uᴴ → parallel scalar channels with gains σ_i.
**Atom or composite:** Composite: SVD + water-filling power allocation.
**Cost model:** O(min(N_r, N_t)·N_r·N_t) for SVD.
**Real wall?** Yes — requires full CSI at TX (CSIT); fast-fading limits feedback rate.
**Cross-domain wiring:** Linear-algebra-matrix (SVD), information-theory-coding (parallel Gaussian channels).
**Notes:** Optimal linear precoding for known channel; benchmark for finite-rate feedback schemes.

### water-filling (cross-domain alias: `water-pouring`, `WF-power-allocation`)
**Domain:** Signal Processing / RF
**Definition:** Allocate power: p_i = max(0, μ − N_0/|h_i|²), Σ p_i = P. Higher power on better subchannels.
**Atom or composite:** Atom: KKT solution to capacity maximization.
**Cost model:** O(N log N) sort + bisection on water level μ.
**Real wall?** Yes — optimal under per-channel Gaussian noise; gain over uniform is small at high SNR.
**Cross-domain wiring:** Control-numerical-opt (KKT conditions), information-theory-coding (parallel channel capacity).
**Notes:** Shannon-Gallager. Generalized to MIMO via SVD parallel channels.

### v-blast (cross-domain alias: `V-BLAST`, `vertical-BLAST`, `layered-MIMO`)
**Domain:** Signal Processing / RF
**Definition:** Spatially multiplex N_t streams; receiver detects sequentially with successive interference cancellation, ordered by SNR.
**Atom or composite:** Composite: nulling + cancellation + ordering.
**Cost model:** O(N_t⁴) for ordered detection per symbol time.
**Real wall?** Yes — error propagation through cancellation steps; ordering reduces but doesn't eliminate it.
**Cross-domain wiring:** Information-theory-coding (successive interference cancellation), retrieval-search (greedy ordering).
**Notes:** Foschini 1996, Wolniansky 1998. Practical realization of MIMO multiplexing gains.

### d-blast (cross-domain alias: `D-BLAST`, `diagonal-BLAST`)
**Domain:** Signal Processing / RF
**Definition:** Spatial-temporal coding across antennas: each codeword spans diagonals through space-time. Closer to MIMO capacity than V-BLAST.
**Atom or composite:** Composite: layered coding + interference cancellation.
**Cost model:** Comparable to V-BLAST with longer codewords.
**Real wall?** Yes — start-up loss; only asymptotically capacity-achieving.
**Cross-domain wiring:** Information-theory-coding (interleaved coding across MIMO dimensions).
**Notes:** Foschini 1996. Theoretical foundation; rarely implemented exactly.

### zf-precoding (cross-domain alias: `ZF-precoding`, `channel-inversion`)
**Domain:** Signal Processing / RF
**Definition:** Precode by Hᴴ(HHᴴ)⁻¹ to invert channel: ŷ = x + ñ. Removes inter-stream interference at cost of noise enhancement.
**Atom or composite:** Composite: pseudoinverse of channel.
**Cost model:** O(N_t·N_r²) per CSI update.
**Real wall?** Yes — fails when channel is ill-conditioned; needs regularization.
**Cross-domain wiring:** Linear-algebra-matrix (pseudoinverse), ml-training (ridgeless regression).
**Notes:** Linear precoding baseline; suboptimal but simple.

### mmse-precoding (cross-domain alias: `regularized-ZF`, `MMSE-MIMO`)
**Domain:** Signal Processing / RF
**Definition:** Precode by Hᴴ(HHᴴ + (N_0/P)·I)⁻¹. Trades interference suppression against noise enhancement.
**Atom or composite:** Composite: regularized pseudoinverse.
**Cost model:** O(N_t·N_r²).
**Real wall?** Yes — regularization parameter must match SNR; mismatched parameter degrades performance.
**Cross-domain wiring:** Ml-training (ridge regression with optimal λ), linear-algebra-matrix (Tikhonov regularization).
**Notes:** Joham-Utschick 2002. Standard linear precoder; outperforms ZF at low SNR.

### dirty-paper-coding (cross-domain alias: `DPC`, `Costa-coding`)
**Domain:** Signal Processing / RF
**Definition:** When TX knows interference s in advance, code u = x + α·s such that p(y|u) is independent of s. Capacity = Gaussian capacity without interference.
**Atom or composite:** Atom: information-theoretic construction.
**Cost model:** Practical implementations (lattice-coded DPC) cost O(2^n) for high-rate.
**Real wall?** Yes — practical DPC achieves only fraction of theoretical capacity.
**Cross-domain wiring:** Information-theory-coding (precoding for known interference), ml-training (control-with-disturbance-preview).
**Notes:** Costa 1983. Theoretical optimum for MIMO broadcast channel.

### tomlinson-harashima (cross-domain alias: `TH-precoding`, `modulo-precoding`)
**Domain:** Signal Processing / RF
**Definition:** Pre-cancel ISI/inter-stream interference at TX using modulo arithmetic to prevent power explosion. Practical DPC approximation.
**Atom or composite:** Composite: feedback cancellation + modulo nonlinearity.
**Cost model:** O(N²) per symbol.
**Real wall?** Yes — modulo loss (small) + shaping loss at low SNR.
**Cross-domain wiring:** Control-numerical-opt (nonlinear precoder design), information-theory-coding (lattice coding).
**Notes:** Tomlinson 1971, Harashima-Miyakawa 1972. Standard nonlinear precoder; close to capacity in MIMO BC.

### vector-perturbation-precoding (cross-domain alias: `VP-precoding`, `lattice-perturbation`)
**Domain:** Signal Processing / RF
**Definition:** Add integer-lattice perturbation l to symbols x such that ‖H⁻¹(x+τ·l)‖ is minimized. Reduces transmit power versus ZF.
**Atom or composite:** Composite: ZF + lattice search.
**Cost model:** O(N^N) naive; O(N³) with sphere decoder.
**Real wall?** Yes — sphere-decode search is NP-hard worst-case; tractable for typical channels.
**Cross-domain wiring:** Linear-algebra-matrix (lattice reduction), retrieval-search (lattice search).
**Notes:** Hochwald-Peel-Swindlehurst 2005. Best linear-ish precoder for known channel.

### massive-mimo (cross-domain alias: `mMIMO`, `large-scale-MIMO`)
**Domain:** Signal Processing / RF
**Definition:** N_t ≫ K users. Channel hardening: hᴴh/N_t → 1 as N_t → ∞; simple linear precoding (MRT, ZF) becomes near-optimal.
**Atom or composite:** Atom: asymptotic regime.
**Cost model:** Linear precoding becomes O(N_t·K) instead of O(N_t³).
**Real wall?** Yes — pilot contamination (reuse) limits achievable rate; calibration of large arrays is non-trivial.
**Cross-domain wiring:** Statistics-probability (concentration of measure), linear-algebra-matrix (random matrix theory).
**Notes:** Marzetta 2010. 5G mMIMO uses 64-256 antennas at base station.

### hybrid-beamforming (cross-domain alias: `hybrid-precoding`, `analog-digital-MIMO`)
**Domain:** Signal Processing / RF
**Definition:** Decompose precoder F = F_RF·F_BB where F_RF is constant-modulus (phase shifters) and F_BB is full-precision digital.
**Atom or composite:** Composite: constrained matrix factorization.
**Cost model:** Iterative optimization with constant-modulus constraint; ~O(N²K) per iter.
**Real wall?** Yes — hardware constraint: number of RF chains << number of antennas; performance gap vs full digital.
**Cross-domain wiring:** Linear-algebra-matrix (constrained factorization), control-numerical-opt (manifold optimization).
**Notes:** Heath et al. 2014. Standard for mmWave 5G/6G systems where digital chains are expensive.

### mmwave-channel-estimation (cross-domain alias: `mmWave-CSI`, `sparse-mmWave`)
**Domain:** Signal Processing / RF
**Definition:** Exploit angular sparsity of mmWave channel: H = Σ_l α_l·a_R(φ_l)·a_Tᴴ(θ_l). Solve via compressed sensing.
**Atom or composite:** Composite: structured-CS recovery.
**Cost model:** O(N·N_path) via OMP.
**Real wall?** Yes — beam misalignment costs huge SNR; tracking is critical at vehicular speeds.
**Cross-domain wiring:** Compressed-sensing (sparse recovery), retrieval-search (angular dictionary search).
**Notes:** Alkhateeb et al. 2014. Foundation of mmWave beam management in 5G NR.

### codebook-precoding (cross-domain alias: `limited-feedback-precoding`, `Grassmannian-codebook`)
**Domain:** Signal Processing / RF
**Definition:** UE picks best precoder from finite codebook Q = {F_1, …, F_K} and feeds back log₂K-bit index. Codebook designed via Grassmannian packing.
**Atom or composite:** Composite: quantization on Grassmannian manifold.
**Cost model:** O(K) search per CSI; codebook design is offline.
**Real wall?** Yes — finite feedback creates quantization-loss vs capacity scaling roughly as (B/K)·log B.
**Cross-domain wiring:** Linear-algebra-matrix (Grassmannian packings), information-theory-coding (rate-distortion of CSIT).
**Notes:** Standard in LTE/5G CSI feedback (Type I/II codebooks).

### beamspace-mimo (cross-domain alias: `beam-space`, `DFT-beam-MIMO`)
**Domain:** Signal Processing / RF
**Definition:** Transform from antenna-space to beam-space via DFT: H_beam = U_DFT·H. mmWave channels are sparse in beam-space.
**Atom or composite:** Atom: DFT transformation of channel matrix.
**Cost model:** O(N log N) per transform.
**Real wall?** Yes — only sparse if channel has few dominant beams; otherwise no reduction.
**Cross-domain wiring:** Linear-algebra-matrix (DFT basis transform), compressed-sensing (sparse representation).
**Notes:** Sayeed 2002. Foundation of beam-selection-based mmWave processing.

---

## Radar Signal Processing

### pulse-compression-lfm (cross-domain alias: `LFM-chirp`, `linear-FM`, `chirp-radar`)
**Domain:** Signal Processing / RF
**Definition:** Transmit chirp s(t) = exp(jπ·K·t²); receiver matched filter compresses pulse to width 1/B. Range resolution = c/(2B) independent of pulse length.
**Atom or composite:** Composite: chirp generation + matched filter.
**Cost model:** O(N log N) via FFT-based matched filter.
**Real wall?** Yes — time-bandwidth product TB = pulse-compression gain; range sidelobes intrinsic without windowing.
**Cross-domain wiring:** Linear-algebra-matrix (chirp basis = generalized Fourier), information-theory-coding (LFM = spread-spectrum analog).
**Notes:** Klauder 1960. Foundation of modern radar; SAR uses LFM extensively.

### barker-code (cross-domain alias: `Barker-sequence`, `binary-pulse-compression`)
**Domain:** Signal Processing / RF
**Definition:** Binary ±1 codes with autocorrelation sidelobes ≤ 1. Lengths 2, 3, 4, 5, 7, 11, 13; no longer Barker codes are known.
**Atom or composite:** Atom: deterministic sequence.
**Cost model:** O(N) matched filter.
**Real wall?** Yes — no Barker code beyond length 13 (conjecture, unproven for general lengths).
**Cross-domain wiring:** Information-theory-coding (good autocorrelation sequences), retrieval-search (template-matching codes).
**Notes:** Barker 1953. Used when integer-multiple Doppler tolerance matters.

### costas-code (cross-domain alias: `Costas-array`, `frequency-hop-code`)
**Domain:** Signal Processing / RF
**Definition:** Frequency-hopping pulse where each chip uses unique frequency from {f_1, …, f_N}; permutation chosen so 2D autocorrelation is thumbtack.
**Atom or composite:** Composite: permutation matrix + tone generation.
**Cost model:** O(N) matched filter per Doppler bin.
**Real wall?** Yes — no general construction for large N; combinatorial search.
**Cross-domain wiring:** Information-theory-coding (low-correlation sequences), retrieval-search (sparse 2D code matching).
**Notes:** Costas 1984. Doppler-tolerant pulse compression for moving targets.

### frank-code (cross-domain alias: `Frank-polyphase`, `quadratic-phase-code`)
**Domain:** Signal Processing / RF
**Definition:** N²-length polyphase code with chip phases φ_{i,j} = 2π·(i·j)/N. Lowest peak sidelobe among classical polyphase codes.
**Atom or composite:** Atom: deterministic polyphase sequence.
**Cost model:** O(N²) matched filter.
**Real wall?** Yes — peak sidelobe level grows ~log(N)/N; doppler-sensitive.
**Cross-domain wiring:** Linear-algebra-matrix (Vandermonde-like phases), information-theory-coding (polyphase pulse compression).
**Notes:** Frank 1963. Foundation of polyphase pulse-compression family (P1, P2, P3, P4 derived).

### p3-p4-codes (cross-domain alias: `P3-polyphase`, `P4-polyphase`)
**Domain:** Signal Processing / RF
**Definition:** Discretized chirp phase codes: P3 chip phases φ_k = π·(k²)/N. P4 derived to be Doppler-tolerant.
**Atom or composite:** Atom: discrete-chirp polyphase code.
**Cost model:** O(N) matched filter.
**Real wall?** Yes — discretization losses; better Doppler tolerance than Frank.
**Cross-domain wiring:** Linear-algebra-matrix (sampled chirp), information-theory-coding (polyphase modulation).
**Notes:** Kretschmer 1983. Practical alternative to LFM with similar properties.

### welti-code (cross-domain alias: `Welti-sequence`, `complementary-pair`)
**Domain:** Signal Processing / RF
**Definition:** Pair of binary sequences (A, B) whose autocorrelations sum to perfect impulse: R_A + R_B = 2N·δ.
**Atom or composite:** Composite: paired sequences.
**Cost model:** O(N) per sequence.
**Real wall?** Yes — must transmit both A and B in temporal succession; range-aliased if echoes overlap.
**Cross-domain wiring:** Information-theory-coding (complementary sequences = orthogonal codes), retrieval-search (zero-sidelobe template matching).
**Notes:** Welti 1960. Generalized as Golay complementary pairs.

### nlfm-chirp (cross-domain alias: `NLFM`, `nonlinear-FM`, `taylor-weighted-chirp`)
**Domain:** Signal Processing / RF
**Definition:** Chirp with nonlinear instantaneous frequency tailored so receiver doesn't need amplitude window — built-in low sidelobes.
**Atom or composite:** Composite: stationary-phase-method design of f(t).
**Cost model:** Same as LFM at receiver.
**Real wall?** Yes — Doppler tolerance is worse than LFM; design via stationary-phase principle.
**Cross-domain wiring:** Information-theory-coding (built-in window in transmit waveform), control-numerical-opt (waveform design optimization).
**Notes:** Cook 1964. Used when receiver-side windowing's SNR loss must be avoided.

### matched-filter-radar (cross-domain alias: `MF-radar`, `correlator-receiver`)
**Domain:** Signal Processing / RF
**Definition:** Receiver impulse response h(t) = s*(−t) where s is transmit signal. Maximizes SNR at sampling instant.
**Atom or composite:** Atom: time-reversed conjugate filter.
**Cost model:** O(N log N) via FFT-based convolution.
**Real wall?** Yes — optimal under white noise; colored noise requires pre-whitening.
**Cross-domain wiring:** Linear-algebra-matrix (inner product), retrieval-search (cross-correlation matching).
**Notes:** Foundation of radar/sonar. Equivalent to dot product with signal template.

### ca-cfar (cross-domain alias: `cell-averaging-CFAR`, `CA-CFAR`)
**Domain:** Signal Processing / RF
**Definition:** Threshold T = α·μ̂ where μ̂ = (1/M)·Σ reference cells. Adaptive to homogeneous-noise level.
**Atom or composite:** Composite: sliding-window mean + multiplier.
**Cost model:** O(M) per range cell.
**Real wall?** Yes — assumes homogeneous noise across reference cells; fails near clutter edges.
**Cross-domain wiring:** Statistics-probability (sample mean), retrieval-search (adaptive threshold).
**Notes:** Finn-Johnson 1968. Standard radar detector; CFAR loss ~M/M+1 for finite M.

### go-cfar (cross-domain alias: `greatest-of-CFAR`, `GO-CFAR`)
**Domain:** Signal Processing / RF
**Definition:** T = α·max(μ_lead, μ_trail). Robust to single-edge clutter step.
**Atom or composite:** Composite: max of leading and trailing cell averages.
**Cost model:** O(M).
**Real wall?** Yes — over-thresholds in homogeneous areas; worse for isolated targets.
**Cross-domain wiring:** Statistics-probability (robust statistics), retrieval-search (max-pooling threshold).
**Notes:** Hansen-Sawyers 1980. Variant for clutter-edge environments.

### so-cfar (cross-domain alias: `smallest-of-CFAR`, `SO-CFAR`)
**Domain:** Signal Processing / RF
**Definition:** T = α·min(μ_lead, μ_trail). Resolves closely-spaced targets that CA-CFAR would mask.
**Atom or composite:** Composite: min of cell averages.
**Cost model:** O(M).
**Real wall?** Yes — high false-alarm rate near clutter edges; needs careful α.
**Cross-domain wiring:** Statistics-probability (robust min estimator), retrieval-search (selective threshold).
**Notes:** Trunk 1978. Variant for multi-target environments.

### os-cfar (cross-domain alias: `OS-CFAR`, `order-statistic-CFAR`)
**Domain:** Signal Processing / RF
**Definition:** T = α·x_(k) where x_(k) is k-th order statistic of M reference cells. Robust to outliers/interferers.
**Atom or composite:** Composite: order-statistic estimation.
**Cost model:** O(M log M) for sort.
**Real wall?** Yes — k-choice trades robustness vs CFAR loss; k ≈ 3M/4 typical.
**Cross-domain wiring:** Statistics-probability (order statistics), retrieval-search (top-k thresholding).
**Notes:** Rohling 1983. Best general-purpose CFAR for mixed environments.

### range-doppler-map (cross-domain alias: `R-D-map`, `range-velocity-image`)
**Domain:** Signal Processing / RF
**Definition:** 2D output: range from fast-time matched-filter, Doppler from slow-time FFT across pulses. Targets appear as peaks in (range, velocity).
**Atom or composite:** Composite: fast-time MF + slow-time DFT.
**Cost model:** O(M·N·log N) for M pulses, N range bins.
**Real wall?** Yes — Doppler resolution = 1/(M·PRI); fundamental tradeoff with dwell time.
**Cross-domain wiring:** Graphics-rendering-lod (2D image formation), retrieval-search (2D peak detection).
**Notes:** Standard processing for pulse-Doppler radar.

### mti-filter (cross-domain alias: `MTI`, `moving-target-indicator`, `clutter-rejection`)
**Domain:** Signal Processing / RF
**Definition:** Pulse-to-pulse subtraction (1−z⁻¹) cancels stationary clutter; cascade improves rejection. Single-pulse MTI = (1, −1) FIR across pulses.
**Atom or composite:** Atom: differentiator across slow-time.
**Cost model:** O(M) per range cell.
**Real wall?** Yes — blind speeds at integer multiples of PRF; staggered PRF mitigates.
**Cross-domain wiring:** Graphics-rendering-lod (background subtraction), retrieval-search (temporal differencing).
**Notes:** Standard ground-clutter rejection technique.

### mtd-processing (cross-domain alias: `MTD`, `moving-target-detection`)
**Domain:** Signal Processing / RF
**Definition:** Filterbank of Doppler filters (typically DFT) across pulse train → separates targets by velocity. Generalizes MTI to multi-band.
**Atom or composite:** Composite: slow-time DFT.
**Cost model:** O(M log M) per range cell.
**Real wall?** Yes — Doppler ambiguity at PRF; resolution = 1/(M·PRI).
**Cross-domain wiring:** Information-theory-coding (Doppler channels), retrieval-search (velocity-band filtering).
**Notes:** Heart of pulse-Doppler radar.

### stap-processing (cross-domain alias: `STAP`, `space-time-adaptive`)
**Domain:** Signal Processing / RF
**Definition:** Joint adaptive filter across antenna elements and pulses: w = R_uu⁻¹·s. Suppresses clutter that is range-Doppler-coupled.
**Atom or composite:** Composite: spatial-temporal covariance + adaptive weight.
**Cost model:** O((MK)³) for M pulses, K antennas; reduced-rank STAP O((MK)² · r).
**Real wall?** Yes — sample support requirement K·M·2 snapshots, often unavailable.
**Cross-domain wiring:** Linear-algebra-matrix (block-Toeplitz covariance), ml-training (high-dim covariance estimation).
**Notes:** Klemm 1998 textbook. Critical for airborne ground-MTI radar.

### sar-imaging (cross-domain alias: `SAR`, `synthetic-aperture-radar`)
**Domain:** Signal Processing / RF
**Definition:** Combine sequential radar echoes along flight path to synthesize large aperture → fine cross-range resolution = D/2 where D is antenna size.
**Atom or composite:** Composite: range compression + azimuth compression.
**Cost model:** O(N² log N) for range-Doppler algorithm.
**Real wall?** Yes — motion-compensation errors degrade focus; processing assumes known platform trajectory.
**Cross-domain wiring:** Graphics-rendering-lod (image formation from raw data), retrieval-search (aperture synthesis = parallel-projection MRI).
**Notes:** Wiley 1951, Cutrona et al. 1960s. Fundamental remote-sensing technique.

### isar-imaging (cross-domain alias: `ISAR`, `inverse-SAR`)
**Domain:** Signal Processing / RF
**Definition:** Target motion (rotation) instead of platform motion provides synthetic aperture. Image of non-cooperative moving target.
**Atom or composite:** Composite: motion compensation + 2D Fourier-based image formation.
**Cost model:** O(N² log N).
**Real wall?** Yes — target rotation must be estimated accurately; small rotation → poor cross-range resolution.
**Cross-domain wiring:** Graphics-rendering-lod (image from motion parallax), retrieval-search (target identification).
**Notes:** Used for ship classification, satellite imaging.

### range-migration-omega-k (cross-domain alias: `ω-k-algorithm`, `Stolt-interpolation-SAR`)
**Domain:** Signal Processing / RF
**Definition:** SAR image formation via 2D FFT + Stolt frequency-axis interpolation. Exact match to wave-equation propagation.
**Atom or composite:** Composite: 2D FFT + Stolt warp.
**Cost model:** O(N² log N) + O(N²) for interpolation.
**Real wall?** Yes — works for straight-line trajectories; bistatic and curvilinear paths need back-projection.
**Cross-domain wiring:** Graphics-rendering-lod (frequency-domain warping), control-numerical-opt (wave-equation solver).
**Notes:** Cafforio-Prati-Rocca 1991. Standard for spaceborne SAR.

### back-projection-sar (cross-domain alias: `BP-SAR`, `time-domain-SAR`)
**Domain:** Signal Processing / RF
**Definition:** For each image pixel, coherently sum range-compressed echoes from all aperture positions. Time-domain, geometrically exact.
**Atom or composite:** Composite: per-pixel coherent integration.
**Cost model:** O(N³) naive; O(N²·log N) with fast factorized back-projection.
**Real wall?** Yes — computationally expensive but handles arbitrary trajectories; fast BP achieves comparable cost to ω-k.
**Cross-domain wiring:** Graphics-rendering-lod (radiosity-like coherent integration), retrieval-search (per-pixel accumulation).
**Notes:** Ulander-Hellsten 2003 fast factorized BP. Preferred for UAV/airborne SAR.

### polarimetric-radar (cross-domain alias: `PolSAR`, `polarization-radar`)
**Domain:** Signal Processing / RF
**Definition:** Measure full 2×2 scattering matrix S = [[S_HH, S_HV],[S_VH, S_VV]] for each pixel. Enables material discrimination.
**Atom or composite:** Composite: dual-polarization transmit + receive.
**Cost model:** Adds 4× data; processing per pixel O(1).
**Real wall?** Yes — calibration requires known polarimetric reference targets; ionospheric Faraday rotation affects L-band PolSAR.
**Cross-domain wiring:** Linear-algebra-matrix (Pauli/Sinclair decomposition), ml-training (polarimetric feature classifiers).
**Notes:** Cloude-Pottier 1996 decomposition. Used in Earth observation, target classification.

### passive-sonar-beamforming (cross-domain alias: `passive-sonar`, `acoustic-array`)
**Domain:** Signal Processing / RF
**Definition:** Beamforming on hydrophone arrays in passive (listen-only) mode → direction-of-arrival of underwater acoustic sources.
**Atom or composite:** Composite: spatial filter on hydrophone array.
**Cost model:** O(M·K) per output for M sensors, K beams.
**Real wall?** Yes — sound-speed profile variations cause beam wander; long arrays for low-frequency sources.
**Cross-domain wiring:** Statistics-probability (signal detection in noise), retrieval-search (acoustic source location).
**Notes:** Used in towed-array sonar, sonobuoys, naval surveillance.

### tdoa-localization (cross-domain alias: `TDOA`, `time-diff-of-arrival`)
**Domain:** Signal Processing / RF
**Definition:** Estimate position from differences in arrival time across multiple receivers. Each TDOA defines a hyperbola; intersection gives position.
**Atom or composite:** Composite: GCC-PHAT correlation + hyperbolic intersection.
**Cost model:** O(N log N) per TDOA + O(K) intersections.
**Real wall?** Yes — geometric dilution of precision (GDOP) limits accuracy; multipath causes outliers.
**Cross-domain wiring:** Statistics-probability (multilateration), retrieval-search (intersection of constraint sets).
**Notes:** Foundation of GPS, emitter localization, indoor positioning.

---

## Synchronization & PLLs

### costas-loop (cross-domain alias: `Costas-PLL`, `coherent-demod-loop`)
**Domain:** Signal Processing / RF
**Definition:** Suppressed-carrier coherent demodulator: I·Q multiplier provides phase error to VCO control. Locks to BPSK/QPSK carrier despite π-ambiguity.
**Atom or composite:** Composite: I/Q mixer + LPF + loop filter + VCO.
**Cost model:** O(1) per sample (NCO + multiplies).
**Real wall?** Yes — π/2 ambiguity for QPSK; cycle-slip events at low SNR.
**Cross-domain wiring:** Control-numerical-opt (closed-loop feedback), ml-training (online phase estimation).
**Notes:** Costas 1956. Standard for BPSK/QPSK coherent receivers.

### decision-directed-carrier (cross-domain alias: `DD-carrier`, `decision-feedback-PLL`)
**Domain:** Signal Processing / RF
**Definition:** Phase error: ε = Im(y[n]·â*[n]) using detected symbol â. Tracks carrier phase after acquisition.
**Atom or composite:** Composite: detector + phase error + loop filter.
**Cost model:** O(1) per symbol.
**Real wall?** Yes — error propagation if symbol decisions are wrong (catastrophic at low SNR).
**Cross-domain wiring:** Control-numerical-opt (model-predictive correction), ml-training (self-training pseudo-labels).
**Notes:** Used in QAM receivers where Costas loop won't lock; needs initial pilot acquisition.

### mueller-muller-timing (cross-domain alias: `M&M-timing`, `Mueller-Muller-recovery`)
**Domain:** Signal Processing / RF
**Definition:** Symbol-timing error: e = â[n−1]·y[n] − â[n]·y[n−1]. Baud-rate (1 sample/symbol) timing recovery.
**Atom or composite:** Composite: decision feedback + timing-error detector.
**Cost model:** O(1) per symbol.
**Real wall?** Yes — works only after decisions are reliable; not self-starting.
**Cross-domain wiring:** Control-numerical-opt (online resampling), retrieval-search (template-position estimation).
**Notes:** Mueller-Muller 1976. Default symbol-rate timing recovery.

### gardner-timing (cross-domain alias: `Gardner-TED`, `2-sample-timing`)
**Domain:** Signal Processing / RF
**Definition:** Timing-error: e = Re(y[k−1/2]·(y*[k] − y*[k−1])). Works on 2 samples/symbol; doesn't need decisions.
**Atom or composite:** Composite: 2-sample-per-symbol TED + loop filter.
**Cost model:** O(1) per symbol.
**Real wall?** Yes — needs 2 samples/symbol; tone at twice symbol rate not required.
**Cross-domain wiring:** Control-numerical-opt (sub-symbol-rate timing).
**Notes:** Gardner 1986. Default modulation-independent symbol timing.

### early-late-gate (cross-domain alias: `early-late-TED`, `discriminator-timing`)
**Domain:** Signal Processing / RF
**Definition:** Compute matched-filter outputs at t − τ/2 and t + τ/2; error = |y_late|² − |y_early|². Drives loop to align.
**Atom or composite:** Composite: dual matched filters + difference.
**Cost model:** O(N) per symbol for 2× matched-filter operations.
**Real wall?** Yes — gate width τ must match expected pulse shape; non-zero offset under noise.
**Cross-domain wiring:** Control-numerical-opt (numerical gradient), retrieval-search (centroid estimation).
**Notes:** Used in GPS code-tracking loops, spread-spectrum receivers.

### schmidl-cox-sync (cross-domain alias: `Schmidl-Cox`, `OFDM-preamble-sync`)
**Domain:** Signal Processing / RF
**Definition:** Use preamble of two identical halves; correlate halves: M(d) = |Σ y*[d+k]·y[d+k+L]|². Peak indicates symbol start.
**Atom or composite:** Composite: half-symbol correlation + peak detection.
**Cost model:** O(N) sliding correlation.
**Real wall?** Yes — coarse plateau in M(d) limits timing precision; fine timing requires additional processing.
**Cross-domain wiring:** Retrieval-search (template matching), information-theory-coding (preamble-based sync).
**Notes:** Schmidl-Cox 1997. Foundation of OFDM synchronization (WiFi, LTE preamble).

### type-2-pll (cross-domain alias: `type-II-PLL`, `2nd-order-PLL`)
**Domain:** Signal Processing / RF
**Definition:** Loop filter with one integrator → zero steady-state error to phase ramp (frequency). Transfer H(s) = (2ζω_n·s + ω_n²)/(s² + 2ζω_n·s + ω_n²).
**Atom or composite:** Composite: PD + PI filter + VCO.
**Cost model:** O(1) per sample.
**Real wall?** Yes — stability constrained by phase margin; loop bandwidth vs noise tradeoff.
**Cross-domain wiring:** Control-numerical-opt (PI controller, internal-model principle).
**Notes:** Standard frequency-locked loop. ζ ≈ 0.707 critically damped.

### type-3-pll (cross-domain alias: `type-III-PLL`, `Doppler-tracking-PLL`)
**Domain:** Signal Processing / RF
**Definition:** Two integrators → zero steady-state error to phase acceleration (e.g., Doppler rate). H(s) has 3 poles at origin from open-loop.
**Atom or composite:** Composite: PD + double-integrator filter + VCO.
**Cost model:** O(1) per sample.
**Real wall?** Yes — stability margin shrinks; sensitive to filter parameter mismatch.
**Cross-domain wiring:** Control-numerical-opt (3rd-order servo), aerospace (missile guidance loops).
**Notes:** Used in GPS receivers for high-dynamics, GNSS jamming environments.

### charge-pump-pll (cross-domain alias: `CP-PLL`, `phase-frequency-detector-PLL`)
**Domain:** Signal Processing / RF
**Definition:** Combines phase-frequency detector (PFD) with charge pump to drive loop filter — wide capture range and lock-detect signals.
**Atom or composite:** Composite: PFD + CP + loop filter + VCO.
**Cost model:** Analog-hardware primitive; control loop runs at ref frequency.
**Real wall?** Yes — CP mismatch and reference spurs limit phase noise floor.
**Cross-domain wiring:** Control-numerical-opt (mixed-signal control), hardware-design.
**Notes:** Default frequency-synthesis PLL in mixed-signal ICs.

### adpll (cross-domain alias: `all-digital-PLL`, `ADPLL`)
**Domain:** Signal Processing / RF
**Definition:** PLL with digitally controlled oscillator (DCO) and time-to-digital converter (TDC) — all-digital implementation.
**Atom or composite:** Composite: TDC + digital loop filter + DCO.
**Cost model:** TDC resolution limited by gate-delay quantization.
**Real wall?** Yes — TDC quantization noise dominates phase noise; gear-shifting bandwidth helps lock acquisition.
**Cross-domain wiring:** Control-numerical-opt (digital servo), information-theory-coding (quantization-induced jitter).
**Notes:** Staszewski 2003. Dominant architecture for cellular ICs (e.g., DRP).

### fractional-n-synthesis (cross-domain alias: `frac-N-PLL`, `sigma-delta-fractional-N`)
**Domain:** Signal Processing / RF
**Definition:** Achieve fractional division ratio N + α by dithering integer divider; sigma-delta modulator shapes resulting quantization noise.
**Atom or composite:** Composite: integer-N PLL + ΣΔ-modulated divider.
**Cost model:** ΣΔ modulator runs at reference rate.
**Real wall?** Yes — ΣΔ noise folds back through nonlinear PFD/CP; spurs at fractional frequencies.
**Cross-domain wiring:** Information-theory-coding (noise shaping), control-numerical-opt (sigma-delta in control loops).
**Notes:** Riley-Copeland-Kwasniewski 1993. Standard for fine-resolution synthesizers.

### foe-coarse (cross-domain alias: `FOE`, `frequency-offset-estimator`)
**Domain:** Signal Processing / RF
**Definition:** Estimate carrier frequency offset Δf from received signal using FFT peak, m-th power method (raise to m), or pilot-based estimation.
**Atom or composite:** Composite: nonlinearity + spectrum peak detection.
**Cost model:** O(N log N) FFT.
**Real wall?** Yes — m-th power suffers SNR loss factor m; pilots cost spectrum overhead.
**Cross-domain wiring:** Statistics-probability (frequency estimation in noise), retrieval-search (spectral peak detection).
**Notes:** Heart of acquisition stage in coherent receivers.

---

## Coding (Advanced)

### ldpc-tanner-graph (cross-domain alias: `LDPC-bipartite`, `Tanner-graph-decoder`)
**Domain:** Signal Processing / RF
**Definition:** LDPC parity-check matrix H represented as bipartite graph of variable nodes and check nodes. Sparse H enables efficient iterative decoding.
**Atom or composite:** Atom: graph structure.
**Cost model:** O(E) memory for E edges; near-linear in code length.
**Real wall?** Yes — performance depends on graph girth, degree distribution; cycles degrade convergence.
**Cross-domain wiring:** Retrieval-search (sparse graph search), ml-training (graph neural network analog).
**Notes:** Tanner 1981. Foundation of all LDPC decoders.

### sum-product-decoding (cross-domain alias: `belief-propagation`, `BP-decoder`, `flooding-decoder`)
**Domain:** Signal Processing / RF
**Definition:** Iterative message passing on Tanner graph: variable nodes sum LLRs from check messages; check nodes apply tanh nonlinearity.
**Atom or composite:** Composite: VN update + CN update + iteration.
**Cost model:** O(E·iters); typically 10-50 iterations.
**Real wall?** Yes — sum-product is exact only on tree; cycles cause loopy BP suboptimality.
**Cross-domain wiring:** Ml-training (graphical model inference), retrieval-search (factor-graph propagation).
**Notes:** Gallager 1962, rediscovered by MacKay 1996. Capacity-approaching with long codes.

### min-sum-decoding (cross-domain alias: `MS-decoder`, `min-sum-BP`)
**Domain:** Signal Processing / RF
**Definition:** Replace tanh in sum-product with sign × min approximation: cn out = sign-product · min(|m_v|). Reduces hardware complexity.
**Atom or composite:** Composite: sign-min check node update.
**Cost model:** O(E·iters); no transcendentals.
**Real wall?** Yes — performance loss vs sum-product (~0.5 dB) but compensable with offset/scaling.
**Cross-domain wiring:** Ml-training (max-product / max-sum belief propagation), control-numerical-opt (logarithmic numerics).
**Notes:** Fossorier 1999. Standard for hardware LDPC decoders.

### offset-min-sum (cross-domain alias: `OMS-decoder`, `offset-MS`)
**Domain:** Signal Processing / RF
**Definition:** Min-sum with offset correction at check nodes: cn out = sign · max(min − β, 0). Recovers most of sum-product gain.
**Atom or composite:** Composite: min-sum + offset.
**Cost model:** Same as min-sum + small constant.
**Real wall?** Yes — offset β must be tuned to code/SNR.
**Cross-domain wiring:** Ml-training (calibrated soft predictions), information-theory-coding (LLR clipping).
**Notes:** Chen-Fossorier 2002. Within 0.1 dB of sum-product in many regimes.

### layered-ldpc-decoding (cross-domain alias: `layered-BP`, `shuffled-decoder`)
**Domain:** Signal Processing / RF
**Definition:** Process check-node rows sequentially, updating variable-node estimates between rows. Converges ~2× faster than flooding.
**Atom or composite:** Composite: row-sequential message passing.
**Cost model:** O(E·iters), fewer iterations needed.
**Real wall?** Yes — ordering matters; parallelizable in blocks but not fully.
**Cross-domain wiring:** Control-numerical-opt (Gauss-Seidel vs Jacobi iteration), retrieval-search (incremental update).
**Notes:** Mansour-Shanbhag 2003. Default in modern LDPC implementations (5G NR uses layered).

### polar-code-construction (cross-domain alias: `Arikan-polarization`, `polar-encoder`)
**Domain:** Signal Processing / RF
**Definition:** Recursive transform G_N = G_2^⊗log N applied to bits creates channels with polarized capacities (→0 or →1). Send info on high-capacity channels.
**Atom or composite:** Composite: log N levels of Arikan kernels.
**Cost model:** O(N log N) encoding.
**Real wall?** Yes — provably capacity-achieving for binary-input symmetric channels; finite-length performance needs CRC-aided SCL.
**Cross-domain wiring:** Information-theory-coding (Shannon limit achieving), linear-algebra-matrix (Kronecker product structure).
**Notes:** Arikan 2009. 5G control-channel code (PBCH, PDCCH).

### sc-decoder-polar (cross-domain alias: `SC-decoder`, `successive-cancellation`)
**Domain:** Signal Processing / RF
**Definition:** Decode bits sequentially: û_i = decision(L_i(y, û_{1:i−1})) using log-likelihood ratios computed by recursive butterfly.
**Atom or composite:** Composite: recursive LLR butterfly + decision.
**Cost model:** O(N log N).
**Real wall?** Yes — error propagation through û_{1:i−1}; SCL needed for short-length codes.
**Cross-domain wiring:** Ml-training (autoregressive decoding), retrieval-search (sequential decision).
**Notes:** Arikan 2009. Capacity-achieving asymptotically but underperforms LDPC at short blocklength.

### scl-decoder-polar (cross-domain alias: `SCL-decoder`, `polar-list-decoder`)
**Domain:** Signal Processing / RF
**Definition:** Maintain L candidate paths; at each bit, expand each by both 0/1 and keep best L paths by path metric. CRC-aided SCL picks CRC-valid path.
**Atom or composite:** Composite: SC + path expansion + pruning.
**Cost model:** O(L·N log N).
**Real wall?** Yes — closes BLER gap to ML at moderate L; CRC overhead small.
**Cross-domain wiring:** Retrieval-search (beam search), ml-training (top-K sampling).
**Notes:** Tal-Vardy 2015. Default 5G polar decoder.

### turbo-code (cross-domain alias: `turbo-PCCC`, `parallel-concatenated`)
**Domain:** Signal Processing / RF
**Definition:** Parallel concatenation of two recursive convolutional codes via interleaver. Iterative MAP decoding exchanges extrinsic LLRs.
**Atom or composite:** Composite: two encoders + interleaver + iterative decoder.
**Cost model:** O(2^v · N) per BCJR pass; typically 6-10 iterations.
**Real wall?** Yes — within 0.5 dB of Shannon limit at long block, but error floor from poor interleavers.
**Cross-domain wiring:** Information-theory-coding (capacity-approaching), ml-training (iterative inference).
**Notes:** Berrou-Glavieux 1993. UMTS, LTE turbo decoder.

### bcjr-decoder (cross-domain alias: `BCJR`, `MAP-decoder`, `forward-backward-decoder`)
**Domain:** Signal Processing / RF
**Definition:** Maximum a posteriori symbol-by-symbol decoder using forward (α) and backward (β) trellis recursions: P(s|y) ∝ α(s)·γ·β(s).
**Atom or composite:** Composite: forward + backward recursion + combine.
**Cost model:** O(2^v · N) for memory-v code, length-N block.
**Real wall?** Yes — log-MAP (log-domain BCJR) needed for numerical stability; max-log-MAP simplifies further.
**Cross-domain wiring:** Statistics-probability (HMM forward-backward), ml-training (CRF inference).
**Notes:** Bahl-Cocke-Jelinek-Raviv 1974. Foundation of soft-decoding convolutional codes; heart of turbo.

### log-map-decoder (cross-domain alias: `log-MAP`, `Jacobian-logarithm-BCJR`)
**Domain:** Signal Processing / RF
**Definition:** BCJR in log-domain using log-sum-exp: log(eˣ + eʸ) = max(x,y) + log(1+e^{−|x−y|}). Numerically stable.
**Atom or composite:** Composite: BCJR + Jacobian-log trick.
**Cost model:** Same as BCJR; LUT for correction term.
**Real wall?** Yes — exact log-MAP needs the correction; max-log-MAP omits it with ~0.4 dB loss.
**Cross-domain wiring:** Ml-training (log-sum-exp in softmax), statistics-probability (numerical stability).
**Notes:** Robertson-Villebrun-Hoeher 1995. Default trellis decoder implementation.

### tcm-ungerboeck (cross-domain alias: `trellis-coded-modulation`, `Ungerboeck-mapping`)
**Domain:** Signal Processing / RF
**Definition:** Combine convolutional code with set-partitioned constellation labeling to maximize free Euclidean distance.
**Atom or composite:** Composite: convolutional encoder + set-partitioning mapper.
**Cost model:** Viterbi decoder O(2^v · N) on Euclidean metric.
**Real wall?** Yes — TCM gain bounded by ~6 dB; outperformed by capacity-approaching codes today.
**Cross-domain wiring:** Information-theory-coding (joint code-modulation design), linear-algebra-matrix (lattice partitioning).
**Notes:** Ungerboeck 1982. Standard for voiceband modems (V.32, V.34).

### set-partitioning (cross-domain alias: `Ungerboeck-partition`, `nested-bit-labeling`)
**Domain:** Signal Processing / RF
**Definition:** Recursively split constellation into subsets such that intra-subset distance increases at each level. Used to map TCM coded bits to higher-priority subset selection.
**Atom or composite:** Atom: combinatorial labeling rule.
**Cost model:** Precomputed offline.
**Real wall?** Yes — only well-defined for symmetric constellations (QAM, PSK).
**Cross-domain wiring:** Linear-algebra-matrix (lattice cosets), information-theory-coding (TCM mapping).
**Notes:** Ungerboeck 1982. Foundation of TCM and lattice-coded modulation.

### bicm (cross-domain alias: `bit-interleaved-coded-modulation`, `BICM`)
**Domain:** Signal Processing / RF
**Definition:** Encode → bit-interleave → map to constellation. At RX: demap to LLR per bit → de-interleave → decode. Simpler than TCM, competitive performance.
**Atom or composite:** Composite: code + interleaver + mapping.
**Cost model:** Encoder + mapper standard; demapper O(2^m) per symbol for 2^m-ary constellation.
**Real wall?** Yes — capacity gap vs joint TCM at low SNR; closed by BICM-ID.
**Cross-domain wiring:** Information-theory-coding (bit-level metric vs symbol-level), retrieval-search (interleaved random access).
**Notes:** Caire-Taricco-Biglieri 1998. Default for modern systems (WiFi, LTE, 5G).

### bicm-id (cross-domain alias: `BICM-ID`, `iterative-BICM`)
**Domain:** Signal Processing / RF
**Definition:** Iterative BICM: decoder produces extrinsic LLRs → soft demapper uses them as a priori for next iteration.
**Atom or composite:** Composite: BICM + iteration between decoder and demapper.
**Cost model:** N_iter × (decoder + demapper) cost.
**Real wall?** Yes — gain depends on labeling (anti-Gray better than Gray for BICM-ID).
**Cross-domain wiring:** Ml-training (iterative inference loops), information-theory-coding (turbo principle).
**Notes:** Li-Ritcey 1998. Used when BICM-only gap to capacity unacceptable.

### lattice-coding (cross-domain alias: `lattice-codes`, `Voronoi-region-shaping`)
**Domain:** Signal Processing / RF
**Definition:** Codewords are lattice points; decoding = closest lattice point search. Optimal lattices (Leech, E8) approach Shannon shaping gain.
**Atom or composite:** Composite: lattice construction + closest-point search (CVP).
**Cost model:** Sphere decoder O(N!) worst case; O(N²) typical.
**Real wall?** Yes — CVP is NP-hard general; tractable for structured lattices.
**Cross-domain wiring:** Linear-algebra-matrix (Gram matrix, LLL reduction), retrieval-search (nearest-lattice-point).
**Notes:** Conway-Sloane textbook. Used in capacity-approaching coded modulation.

---

## Quantization & Source Coding

### uniform-quantization (cross-domain alias: `linear-quantizer`, `mid-tread`, `mid-rise`)
**Domain:** Signal Processing / RF
**Definition:** Q(x) = Δ·round(x/Δ). Quantization noise = x − Q(x). Uniformly distributed ∈ [−Δ/2, Δ/2] when x is "smooth".
**Atom or composite:** Atom: rounding to lattice.
**Cost model:** O(1) per sample.
**Real wall?** Yes — SQNR = 6.02·B + 1.76 dB for B-bit; granular noise dominates above threshold.
**Cross-domain wiring:** Information-theory-coding (rate-distortion), graphics-rendering-lod (color quantization).
**Notes:** Foundation of all ADCs. Mid-tread includes zero in output; mid-rise has zero on edge.

### lloyd-max-quantizer (cross-domain alias: `Lloyd-Max`, `optimal-scalar-quantizer`)
**Domain:** Signal Processing / RF
**Definition:** Iteratively find quantization levels {y_k} and decision boundaries {x_k} that minimize E[(X − Q(X))²] for given source p(x).
**Atom or composite:** Composite: alternating optimization of boundaries and centroids.
**Cost model:** Iterative; converges in tens of iterations to local optimum.
**Real wall?** Yes — local optima for non-log-concave densities; global optimum only for Gaussian (analytic).
**Cross-domain wiring:** Ml-training (k-means clustering = Lloyd's algorithm in higher dim), statistics-probability (centroid clustering).
**Notes:** Lloyd 1957, Max 1960. Foundation of optimal quantization theory.

### vector-quantization (cross-domain alias: `VQ`, `block-quantization`)
**Domain:** Signal Processing / RF
**Definition:** Map vectors to codewords from finite codebook: Q(x) = argmin_c ‖x − c‖. Achieves shaping/space-filling gains over scalar quantization.
**Atom or composite:** Composite: codebook + nearest-neighbor search.
**Cost model:** O(N·K) per vector for K codewords; tree-VQ reduces this.
**Real wall?** Yes — codebook search dominates; storage scales as 2^(N·R) for rate R per dim.
**Cross-domain wiring:** Retrieval-search (vector nearest-neighbor), ml-training (centroid-based representation, VQ-VAE).
**Notes:** Linde-Buzo-Gray 1980. Foundation of speech codebook techniques.

### lbg-algorithm (cross-domain alias: `LBG`, `generalized-Lloyd`, `splitting-VQ`)
**Domain:** Signal Processing / RF
**Definition:** Train VQ codebook by iterative refinement: assign each training vector → recompute centroids → split high-distortion cells.
**Atom or composite:** Composite: k-means + splitting.
**Cost model:** O(N·K·iters) for N training vectors, K codewords.
**Real wall?** Yes — depends on training data sufficiency; codebook design separate from runtime.
**Cross-domain wiring:** Ml-training (k-means++, codebook learning), retrieval-search (clustering for index).
**Notes:** Linde-Buzo-Gray 1980. Standard offline VQ training.

### tree-structured-vq (cross-domain alias: `TSVQ`, `hierarchical-VQ`)
**Domain:** Signal Processing / RF
**Definition:** VQ search via binary tree: at each node, choose subtree containing closer centroid. O(log K) search vs O(K) full search.
**Atom or composite:** Composite: hierarchical codebook + tree traversal.
**Cost model:** O(log K) per vector but slightly suboptimal vs full VQ.
**Real wall?** Yes — greedy tree decisions cause suboptimality; multi-stage VQ alternative.
**Cross-domain wiring:** Retrieval-search (hierarchical NN indexes), ml-training (decision tree quantization).
**Notes:** Used in speech codecs where search complexity must be bounded.

### lattice-vq (cross-domain alias: `lattice-quantizer`, `regular-VQ`)
**Domain:** Signal Processing / RF
**Definition:** Codebook = points of a lattice (Z^N, D_4, E_8, Leech). Fast quantization via lattice-specific algorithms; structured + dense.
**Atom or composite:** Composite: lattice + nearest-point algorithm.
**Cost model:** O(N log N) or O(N) for special lattices.
**Real wall?** Yes — lattice shape determines space-filling efficiency; Leech achieves best in dim 24.
**Cross-domain wiring:** Linear-algebra-matrix (lattice point counting), retrieval-search (lattice search).
**Notes:** Conway-Sloane 1982. Compromise between structure (fast search) and performance.

### trellis-coded-quantization (cross-domain alias: `TCQ`, `Marcellin-Fischer`)
**Domain:** Signal Processing / RF
**Definition:** TCM applied to quantization: Viterbi search through trellis where branches are Voronoi cells of a fine lattice.
**Atom or composite:** Composite: trellis + cell decoder.
**Cost model:** O(N·2^v) Viterbi.
**Real wall?** Yes — bounded by lattice quantizer gain; less rate flexibility than VQ.
**Cross-domain wiring:** Information-theory-coding (TCM-quantization duality), retrieval-search (Viterbi search).
**Notes:** Marcellin-Fischer 1990. Achieves shaping + granular gains close to optimal.

### dpcm (cross-domain alias: `differential-PCM`, `predictive-quantizer`)
**Domain:** Signal Processing / RF
**Definition:** Quantize prediction residual r[n] = x[n] − x̂[n]; reduces bit rate by exploiting signal correlation. x[n] reconstructed by adding back prediction.
**Atom or composite:** Composite: predictor + quantizer + feedback loop.
**Cost model:** O(P) per sample for P-tap predictor.
**Real wall?** Yes — open-loop predictor in DPCM has stability/bias; closed-loop (reconstructed signal) avoids drift but couples errors.
**Cross-domain wiring:** Information-theory-coding (rate reduction via prediction), ml-training (autoregressive coding).
**Notes:** Cutler 1952. Foundation of all predictive quantization.

### adpcm (cross-domain alias: `adaptive-DPCM`, `G.726`)
**Domain:** Signal Processing / RF
**Definition:** DPCM with both predictor and quantizer step size adapted online to signal statistics.
**Atom or composite:** Composite: adaptive predictor + adaptive scalar quantizer.
**Cost model:** O(P) per sample.
**Real wall?** Yes — adaptation lag during nonstationary input.
**Cross-domain wiring:** Ml-training (online adaptation), control-numerical-opt (gradient adaptation).
**Notes:** G.721 → G.726 ITU codec. 32 kbit/s voice; widely deployed.

### sigma-delta-1st-order (cross-domain alias: `ΣΔ-1st`, `1-bit-modulator`)
**Domain:** Signal Processing / RF
**Definition:** Loop with integrator and 1-bit quantizer: y[n] = sign(integrator). Quantization noise shaped to high frequency by integrator.
**Atom or composite:** Composite: integrator + 1-bit quantizer + DAC feedback.
**Cost model:** O(1) per sample at oversampled rate.
**Real wall?** Yes — 1st-order ΣΔ provides 9 dB/oct noise rejection; need high oversampling for high resolution.
**Cross-domain wiring:** Control-numerical-opt (closed-loop quantization), information-theory-coding (noise shaping).
**Notes:** Cutler 1960 patent. Used in 1-bit DACs and oversampled ADCs.

### sigma-delta-higher-order (cross-domain alias: `higher-order-ΣΔ`, `multi-loop-ΣΔ`)
**Domain:** Signal Processing / RF
**Definition:** L-th order ΣΔ has L cascaded integrators → noise transfer function (1−z⁻¹)^L → 6L+3 dB/octave rejection.
**Atom or composite:** Composite: L integrators + feedback loop.
**Cost model:** O(L) per sample.
**Real wall?** Yes — L > 2 has stability issues; coefficient scaling required.
**Cross-domain wiring:** Control-numerical-opt (higher-order servo), information-theory-coding (aggressive noise shaping).
**Notes:** Candy-Temes 1991. Foundation of modern audio ADC/DAC (16-24 bit at ~1 MHz).

### mash-modulator (cross-domain alias: `MASH-ΣΔ`, `multi-stage-noise-shaping`)
**Domain:** Signal Processing / RF
**Definition:** Cascade of stable lower-order ΣΔ stages with digital cancellation of stage-1 noise. Achieves higher-order noise shaping with guaranteed stability.
**Atom or composite:** Composite: cascade of 1st/2nd-order ΣΔ + digital error cancellation.
**Cost model:** O(L) per sample for L-stage MASH.
**Real wall?** Yes — analog coefficient matching critical for digital cancellation effectiveness.
**Cross-domain wiring:** Control-numerical-opt (stage decomposition), information-theory-coding (multi-stage noise shaping).
**Notes:** Matsuya et al. 1987. Common in high-resolution audio ADCs.

### noise-shaping (cross-domain alias: `error-shaping`, `dither-shape`)
**Domain:** Signal Processing / RF
**Definition:** Filter quantization error with NTF(z) such that in-band noise is minimized: ε_shaped[n] = ε[n] ∗ NTF.
**Atom or composite:** Atom: error-feedback filter.
**Cost model:** O(N) for N-tap NTF.
**Real wall?** Yes — Gerzon-Craven theorem bounds total noise power; can only redistribute, not remove.
**Cross-domain wiring:** Information-theory-coding (rate-distortion via noise allocation), graphics-rendering-lod (perceptual quantization).
**Notes:** Foundation of high-resolution audio (Apogee UV22, Pacific Microsonics).

### dithering (cross-domain alias: `dither`, `randomized-quantizer`)
**Domain:** Signal Processing / RF
**Definition:** Add small random signal before quantization to decorrelate quantization noise from signal. Rectangular, triangular, or Gaussian dither pdf.
**Atom or composite:** Atom: random injection + quantize.
**Cost model:** O(1) per sample.
**Real wall?** Yes — TPDF (triangular) dither makes 2nd moment of error independent of signal, eliminating distortion.
**Cross-domain wiring:** Graphics-rendering-lod (image dithering for low bit-depth display), statistics-probability (randomized rounding).
**Notes:** Schuchman 1964. Standard in audio mastering; converts distortion to white noise.

### companding (cross-domain alias: `μ-law`, `A-law`, `non-uniform-quantization`)
**Domain:** Signal Processing / RF
**Definition:** Nonlinear quantizer where low-amplitude samples get finer steps. μ-law: y = sign(x)·log(1+μ|x|)/log(1+μ).
**Atom or composite:** Composite: nonlinear map + uniform quantizer + inverse map.
**Cost model:** O(1) per sample.
**Real wall?** Yes — μ ≈ 255 (US), A = 87.6 (Europe) approximate optimum for speech.
**Cross-domain wiring:** Statistics-probability (variance-stabilizing transform), ml-training (log-domain quantization).
**Notes:** Smith 1957. Foundation of voice telephony (G.711).

### huffman-coding (cross-domain alias: `prefix-code`, `Huffman-tree`)
**Domain:** Signal Processing / RF
**Definition:** Variable-length prefix code; symbols with higher probability get shorter codewords. Optimal among prefix codes for known distribution.
**Atom or composite:** Composite: greedy tree construction.
**Cost model:** O(N log N) for sorted construction.
**Real wall?** Yes — within 1 bit of entropy; optimal for known iid distribution.
**Cross-domain wiring:** Information-theory-coding (entropy coding), retrieval-search (prefix-tree decoding).
**Notes:** Huffman 1952. Standard for fixed-distribution entropy coding (JPEG, DEFLATE).

### arithmetic-coding (cross-domain alias: `arithmetic-coder`, `range-coder`)
**Domain:** Signal Processing / RF
**Definition:** Encode sequence as fractional number in interval [0,1) iteratively subdivided by symbol probabilities. Approaches entropy to fractional bits.
**Atom or composite:** Composite: interval subdivision + bit emission.
**Cost model:** O(N) per N symbols.
**Real wall?** Yes — finite-precision arithmetic causes "carry" issues; resolved by renormalization.
**Cross-domain wiring:** Information-theory-coding (near-entropy coding), retrieval-search (probabilistic indexing).
**Notes:** Rissanen-Pasco 1976. Used in JPEG2000, H.264 CABAC.

### range-coding (cross-domain alias: `byte-aligned-arithmetic`, `RC`)
**Domain:** Signal Processing / RF
**Definition:** Byte-oriented version of arithmetic coding; uses integer arithmetic with renormalization at byte boundaries.
**Atom or composite:** Composite: arithmetic coder + byte renormalization.
**Cost model:** O(N), faster than bit-oriented arithmetic.
**Real wall?** Yes — small efficiency loss vs ideal arithmetic; trades for implementation simplicity.
**Cross-domain wiring:** Information-theory-coding (entropy coding), retrieval-search (compressed index).
**Notes:** Subbotin 1979, Martin 1979. Used in modern compressors (LZMA, zstd).

---

## Speech & Audio

### lpc-analysis (cross-domain alias: `linear-predictive-coding`, `LPC`, `inverse-filter`)
**Domain:** Signal Processing / RF
**Definition:** Estimate AR model for short speech frame: minimize Σ |x[n] − Σ a_k·x[n−k]|². Coefficients model vocal tract.
**Atom or composite:** Composite: autocorrelation + Levinson-Durbin.
**Cost model:** O(N·p + p²) per frame for length-N, order p.
**Real wall?** Yes — assumes all-pole vocal tract; nasals and fricatives violate this.
**Cross-domain wiring:** Statistics-probability (AR modeling), ml-training (linear prediction features).
**Notes:** Atal-Hanauer 1971. Foundation of all speech coders (LPC-10, CELP, G.729).

### line-spectral-pairs (cross-domain alias: `LSP`, `LSF`, `line-spectral-frequencies`)
**Domain:** Signal Processing / RF
**Definition:** Decompose LPC polynomial A(z) = (P(z) + Q(z))/2 where P, Q have zeros on unit circle at "line spectral frequencies".
**Atom or composite:** Composite: polynomial decomposition + root finding.
**Cost model:** O(p²) root-finding via Chebyshev polynomial expansion.
**Real wall?** Yes — LSPs more robust to quantization than direct LPC coefficients; bounded ordering preserves stability.
**Cross-domain wiring:** Linear-algebra-matrix (polynomial roots), information-theory-coding (vector-quantize LSPs).
**Notes:** Itakura 1975. Standard for transmitting LPC parameters in speech codecs.

### formant-estimation (cross-domain alias: `formant-tracker`, `LPC-formants`)
**Domain:** Signal Processing / RF
**Definition:** Find LPC polynomial roots inside unit circle; angles = formant frequencies, distance-from-circle = bandwidths.
**Atom or composite:** Composite: LPC + polynomial root finding.
**Cost model:** O(p²) per frame.
**Real wall?** Yes — root-pole assignment to formants is heuristic; track-switching in continuous speech.
**Cross-domain wiring:** Linear-algebra-matrix (polynomial roots), retrieval-search (peak tracking).
**Notes:** Foundation of formant-based speech analysis and synthesis.

### autocorrelation-pitch (cross-domain alias: `ACF-pitch`, `autocorrelation-F0`)
**Domain:** Signal Processing / RF
**Definition:** Pitch period = lag τ maximizing autocorrelation R(τ) for τ ∈ [τ_min, τ_max].
**Atom or composite:** Composite: autocorrelation + peak detection.
**Cost model:** O(N log N) via FFT.
**Real wall?** Yes — octave errors common (peak at 2τ or τ/2); pre-filtering and dynamic programming help.
**Cross-domain wiring:** Statistics-probability (correlation lag), retrieval-search (template-period matching).
**Notes:** Simplest pitch detector. Modern variants: YIN, RAPT, PYIN.

### amdf-pitch (cross-domain alias: `AMDF`, `average-magnitude-difference`)
**Domain:** Signal Processing / RF
**Definition:** D(τ) = Σ |x[n] − x[n+τ]|; pitch at minimum. Cheap alternative to autocorrelation.
**Atom or composite:** Composite: difference function + minimum.
**Cost model:** O(N · τ_max).
**Real wall?** Yes — same octave-ambiguity issues; less robust than autocorrelation for noisy speech.
**Cross-domain wiring:** Statistics-probability (L1 distance), retrieval-search (L1-template matching).
**Notes:** Ross et al. 1974. Common in low-power pitch detectors.

### yin-pitch (cross-domain alias: `YIN`, `cumulative-mean-norm`)
**Domain:** Signal Processing / RF
**Definition:** Cumulative mean normalized difference: d'(τ) = d(τ)/((1/τ)·Σ_{k=1}^τ d(k)); find first dip below threshold.
**Atom or composite:** Composite: AMDF-like difference + normalization + parabolic refinement.
**Cost model:** O(N · τ_max).
**Real wall?** Yes — robust against octave errors but voicing-decision and noise still challenging.
**Cross-domain wiring:** Statistics-probability (cumulative normalization), ml-training (signal-feature extraction).
**Notes:** de Cheveigné-Kawahara 2002. Default modern pitch detector in MIR.

### pyin-pitch (cross-domain alias: `pYIN`, `probabilistic-YIN`)
**Domain:** Signal Processing / RF
**Definition:** Probabilistic YIN: multiple thresholds → HMM-based voicing/pitch tracking across frames.
**Atom or composite:** Composite: YIN candidates + HMM smoothing.
**Cost model:** O(N · τ_max + K) per frame, plus HMM decode.
**Real wall?** Yes — HMM smoothing handles brief errors but requires good transition model.
**Cross-domain wiring:** Statistics-probability (HMM tracking), ml-training (probabilistic feature extraction).
**Notes:** Mauch-Dixon 2014. Industry-standard pitch tracker in librosa.

### cepstral-analysis (cross-domain alias: `cepstrum`, `quefrency-domain`)
**Domain:** Signal Processing / RF
**Definition:** Cepstrum c[n] = IDFT(log|DFT(x[n])|). Separates source (excitation) from filter (vocal tract).
**Atom or composite:** Composite: DFT → log magnitude → IDFT.
**Cost model:** O(N log N).
**Real wall?** Yes — log near zero is unstable; complex cepstrum unwraps phase carefully.
**Cross-domain wiring:** Statistics-probability (homomorphic separation), retrieval-search (cepstral fingerprint).
**Notes:** Bogert-Healy-Tukey 1963. Reverses convolution into addition.

### mfcc (cross-domain alias: `mel-frequency-cepstral-coefficients`, `MFCC`)
**Domain:** Signal Processing / RF
**Definition:** STFT → mel filter bank → log → DCT-II → keep first 12-20 coefficients. Compact perceptual features.
**Atom or composite:** Composite: STFT + mel filterbank + log + DCT.
**Cost model:** O(N log N).
**Real wall?** Yes — fixed mel scale; phase information discarded.
**Cross-domain wiring:** Ml-training (speech recognition features), retrieval-search (audio fingerprinting).
**Notes:** Davis-Mermelstein 1980. Foundation feature for ASR (replaced now by learned features).

### mfcc-deltas (cross-domain alias: `Δ-MFCC`, `dynamic-MFCC`)
**Domain:** Signal Processing / RF
**Definition:** First and second time-derivatives of MFCC across frames: Δ_t = (Σ_k k·(c_{t+k} − c_{t−k}))/(2 Σ_k k²).
**Atom or composite:** Atom: regression-based time derivative.
**Cost model:** O(K·D) per frame for window K, dimension D.
**Real wall?** Yes — captures local dynamics; saturates beyond 2nd derivative.
**Cross-domain wiring:** Ml-training (temporal features), control-numerical-opt (numerical differentiation).
**Notes:** Furui 1986. Standard 39-dim feature: 13 MFCC + 13 Δ + 13 ΔΔ.

### celp-coder (cross-domain alias: `CELP`, `code-excited-LP`)
**Domain:** Signal Processing / RF
**Definition:** Excite LPC synthesis filter with codebook entries → search codebook for best perceptual match (analysis-by-synthesis).
**Atom or composite:** Composite: LPC + adaptive codebook (pitch) + fixed codebook + perceptual weighting.
**Cost model:** Codebook search dominates; O(K·N) per frame for K codewords.
**Real wall?** Yes — codebook size vs search complexity trade; perceptual weighting filter is critical.
**Cross-domain wiring:** Information-theory-coding (analysis-by-synthesis), retrieval-search (best-match codebook).
**Notes:** Atal-Schroeder 1985. Foundation of G.729, AMR, AMR-WB.

### mdct-codec (cross-domain alias: `MDCT`, `modified-DCT`)
**Domain:** Signal Processing / RF
**Definition:** Lapped 50%-overlap DCT-IV with TDAC for perfect reconstruction. y_k = Σ x_n·cos(π(n+1/2+N/2)(k+1/2)/N).
**Atom or composite:** Composite: window + DCT-IV + overlap-add.
**Cost model:** O(N log N) per block via fast DCT.
**Real wall?** Yes — block boundaries create TDAC constraint; window must satisfy princen-bradley condition.
**Cross-domain wiring:** Information-theory-coding (transform coding), graphics-rendering-lod (lapped transforms).
**Notes:** Princen-Bradley 1986. Heart of MP3, AAC, Vorbis, Opus transform layer.

### psychoacoustic-masking (cross-domain alias: `masking-model`, `bark-masking`)
**Domain:** Signal Processing / RF
**Definition:** Compute masking threshold per Bark band: spreading function + tonality factor → bit-allocation for transform coder.
**Atom or composite:** Composite: tonality detection + spreading function + masking threshold.
**Cost model:** O(N) per frame after spectrum.
**Real wall?** Yes — masking model is approximate; complex audio may exceed model accuracy.
**Cross-domain wiring:** Statistics-probability (perceptual likelihood), ml-training (rate-distortion with perceptual loss).
**Notes:** Foundation of all perceptual audio codecs (MP3, AAC, Opus).

---

## Compressed Sensing

### sparse-recovery (cross-domain alias: `CS-recovery`, `sparse-inverse-problem`)
**Domain:** Signal Processing / RF
**Definition:** Recover sparse x from y = Φx + n where Φ is M×N with M ≪ N. Solve via L1 minimization: min ‖x‖_1 s.t. ‖y − Φx‖_2 ≤ ε.
**Atom or composite:** Composite: sparse formulation + convex solver.
**Cost model:** O(MN log N) via fast solvers; O(N³) interior-point.
**Real wall?** Yes — recovery guaranteed under RIP; without RIP, NP-hard sparse approximation.
**Cross-domain wiring:** Linear-algebra-matrix (underdetermined systems), ml-training (Lasso regression).
**Notes:** Donoho 2004, Candès-Romberg-Tao 2006. Foundation of compressed sensing.

### restricted-isometry-property (cross-domain alias: `RIP`, `near-isometry`)
**Domain:** Signal Processing / RF
**Definition:** Φ satisfies (k, δ_k)-RIP if (1−δ_k)‖x‖² ≤ ‖Φx‖² ≤ (1+δ_k)‖x‖² for all k-sparse x.
**Atom or composite:** Atom: structural property.
**Cost model:** Verifying RIP is NP-hard; random matrices satisfy it whp.
**Real wall?** Yes — RIP is the canonical sufficient condition for sparse recovery; coherence is computable proxy.
**Cross-domain wiring:** Linear-algebra-matrix (concentration of measure), retrieval-search (incoherence in dictionaries).
**Notes:** Candès-Tao 2005. Random Gaussian/Bernoulli matrices satisfy RIP with overwhelming probability for M ≳ k·log(N/k).

### basis-pursuit (cross-domain alias: `BP`, `L1-minimization`)
**Domain:** Signal Processing / RF
**Definition:** min ‖x‖_1 subject to Φx = y. Convex problem solvable via LP.
**Atom or composite:** Atom: convex L1 minimization.
**Cost model:** O(N³) interior-point LP.
**Real wall?** Yes — L1 is convex relaxation of L0; exact recovery when RIP holds and k is below threshold.
**Cross-domain wiring:** Linear-algebra-matrix (LP duality), ml-training (Lasso = noisy BP).
**Notes:** Chen-Donoho-Saunders 1998. Foundational convex relaxation in compressed sensing.

### basis-pursuit-denoising (cross-domain alias: `BPDN`, `noisy-Lasso`)
**Domain:** Signal Processing / RF
**Definition:** min ‖x‖_1 s.t. ‖Φx − y‖_2 ≤ ε. Robust to noise. Lagrangian form: min ‖Φx − y‖_2² + λ‖x‖_1.
**Atom or composite:** Composite: BP + noise constraint.
**Cost model:** Various: ISTA O(MN/k), ADMM O(MN), interior-point O(N³).
**Real wall?** Yes — λ trades sparsity vs fit; cross-validation needed.
**Cross-domain wiring:** Ml-training (Lasso/elastic-net), control-numerical-opt (proximal methods).
**Notes:** Default formulation for sparse recovery with noise; Lasso (Tibshirani 1996) is statistical equivalent.

### omp-algorithm (cross-domain alias: `OMP`, `orthogonal-matching-pursuit`)
**Domain:** Signal Processing / RF
**Definition:** Greedy: at each step pick column most correlated with residual, project signal onto chosen columns, update residual.
**Atom or composite:** Composite: greedy column selection + LS projection.
**Cost model:** O(k·MN) for k-sparse signal.
**Real wall?** Yes — greedy choices may select wrong columns under high coherence.
**Cross-domain wiring:** Retrieval-search (greedy template selection), ml-training (forward stagewise regression).
**Notes:** Pati-Rezaiifar-Krishnaprasad 1993. Standard greedy sparse recovery.

### cosamp-algorithm (cross-domain alias: `CoSaMP`, `compressive-sampling-MP`)
**Domain:** Signal Processing / RF
**Definition:** Pick 2k largest correlations, solve LS on union with current support, prune to top k. Provable RIP-based recovery.
**Atom or composite:** Composite: support identification + LS + pruning.
**Cost model:** O(MN) per iteration; O(log(‖x‖/ε)) iterations.
**Real wall?** Yes — within constant of optimal sparse approximation under RIP.
**Cross-domain wiring:** Retrieval-search (top-k selection), control-numerical-opt (iterative refinement).
**Notes:** Needell-Tropp 2009. Combines provable guarantees with practical efficiency.

### iht-algorithm (cross-domain alias: `IHT`, `iterative-hard-thresholding`)
**Domain:** Signal Processing / RF
**Definition:** x_{t+1} = H_k(x_t + μ·Φᵀ(y − Φx_t)) where H_k zeros all but top-k entries.
**Atom or composite:** Composite: gradient step + hard thresholding.
**Cost model:** O(MN) per iteration.
**Real wall?** Yes — converges if μ·‖Φ‖² < 2 and RIP holds; otherwise can diverge.
**Cross-domain wiring:** Ml-training (proximal gradient with non-convex L0), control-numerical-opt (projected gradient).
**Notes:** Blumensath-Davies 2009. Simple, fast, with RIP-based guarantees.

### amp-algorithm (cross-domain alias: `AMP`, `approximate-message-passing`)
**Domain:** Signal Processing / RF
**Definition:** Iterative algorithm with Onsager correction term: x_{t+1} = η(x_t + Φᵀ z_t; λ); z_t = y − Φx_t + (z_{t−1}/M)·‖η'‖.
**Atom or composite:** Composite: thresholded gradient + Onsager term.
**Cost model:** O(MN) per iter; converges in O(1) iterations.
**Real wall?** Yes — works for iid Gaussian Φ; correlated matrices break state evolution.
**Cross-domain wiring:** Statistics-probability (state evolution analysis), ml-training (denoising-step interpretation).
**Notes:** Donoho-Maleki-Montanari 2009. Provably optimal Bayes-MMSE for Gaussian sensing matrices.

### vamp-algorithm (cross-domain alias: `VAMP`, `vector-AMP`)
**Domain:** Signal Processing / RF
**Definition:** AMP variant tolerating arbitrary right-rotationally-invariant Φ via SVD-based pre-rotation.
**Atom or composite:** Composite: pre-rotate + AMP-like iteration.
**Cost model:** O(MN) per iter + SVD precomputation O(min(M,N)²·max(M,N)).
**Real wall?** Yes — extends AMP's reach but right-rotational invariance still restrictive.
**Cross-domain wiring:** Linear-algebra-matrix (SVD-based preconditioning), ml-training (variational inference).
**Notes:** Rangan-Schniter-Fletcher 2017. Resolves AMP's correlation-matrix sensitivity.

### damp-denoising-amp (cross-domain alias: `D-AMP`, `denoising-AMP`)
**Domain:** Signal Processing / RF
**Definition:** Replace AMP's soft-thresholding with arbitrary denoiser (BM3D, deep network). Image-domain compressed sensing.
**Atom or composite:** Composite: gradient + arbitrary denoiser + Onsager.
**Cost model:** O(MN + denoiser cost) per iter.
**Real wall?** Yes — Onsager term requires Monte-Carlo divergence estimate of denoiser.
**Cross-domain wiring:** Ml-training (deep denoiser priors), retrieval-search (learned dictionaries).
**Notes:** Metzler-Maleki-Baraniuk 2016. Foundation of "plug-and-play" image reconstruction.

### deep-unrolling-cs (cross-domain alias: `LISTA`, `unrolled-CS`)
**Domain:** Signal Processing / RF
**Definition:** Replace fixed iterative-soft-thresholding (ISTA) with trainable neural network: each "layer" is one ISTA step with learnable parameters.
**Atom or composite:** Composite: K-layer unrolled algorithm + learning.
**Cost model:** Training O(N_iter); inference K-layer NN.
**Real wall?** Yes — needs labeled training data; generalization beyond training distribution unclear.
**Cross-domain wiring:** Ml-training (learning iterative algorithms), control-numerical-opt (learned solvers).
**Notes:** Gregor-LeCun 2010. Foundational technique for algorithm-inspired networks.

---

## RF Hardware Models

### iq-imbalance (cross-domain alias: `I/Q-mismatch`, `gain-phase-imbalance`)
**Domain:** Signal Processing / RF
**Definition:** I/Q paths have gain α and phase φ mismatch → image rejection ratio IRR = (1+α²−2α·cos φ)/(1+α²+2α·cos φ).
**Atom or composite:** Atom: hardware-induced linear distortion.
**Cost model:** O(1) per sample for correction matrix.
**Real wall?** Yes — temperature-dependent; requires periodic recalibration. Image creates ghost copy of desired signal.
**Cross-domain wiring:** Linear-algebra-matrix (2×2 correction matrix), control-numerical-opt (adaptive calibration loop).
**Notes:** Standard correction: estimate α, φ from pilot or blind methods (Schenk 2008).

### dc-offset (cross-domain alias: `DC-bias`, `LO-leakage`)
**Domain:** Signal Processing / RF
**Definition:** Direct-conversion receivers exhibit DC offset from LO self-mixing and amplifier mismatch. Spectrum spike at zero IF.
**Atom or composite:** Atom: low-frequency interference.
**Cost model:** High-pass filter or per-block mean subtraction.
**Real wall?** Yes — high-pass destroys low-frequency information; AC coupling has finite blocking time.
**Cross-domain wiring:** Statistics-probability (mean estimation), ml-training (zero-mean preprocessing).
**Notes:** Mitigated by mean subtraction or analog DC servo. Critical for direct-conversion (zero-IF) receivers.

### phase-noise (cross-domain alias: `PN`, `LO-jitter`, `Lorentzian-spectrum`)
**Domain:** Signal Processing / RF
**Definition:** Oscillator phase fluctuation θ(t) → spectrum spreading around carrier. Power spectral density typically Lorentzian (1/f²) + flicker (1/f).
**Atom or composite:** Atom: stochastic process on oscillator phase.
**Cost model:** Simulation via random walk integration.
**Real wall?** Yes — Leeson's equation bounds achievable PN given oscillator Q-factor and power; fundamental physical limit.
**Cross-domain wiring:** Statistics-probability (stochastic process), control-numerical-opt (PLL noise shaping).
**Notes:** Leeson 1966. PN limits OFDM ICI, coherent demodulation, mmWave/RF capacity.

### jitter-clock (cross-domain alias: `aperture-jitter`, `sample-clock-jitter`)
**Domain:** Signal Processing / RF
**Definition:** Sampling instant uncertainty τ_jitter → noise power proportional to (2πf·σ_τ)² for input frequency f.
**Atom or composite:** Atom: timing noise.
**Cost model:** Bounds SNR independent of bit count beyond jitter floor.
**Real wall?** Yes — SNR_jitter = 1/(2πf·σ_τ)²; for 1 ps RMS jitter at 1 GHz, SNR ~44 dB ceiling.
**Cross-domain wiring:** Control-numerical-opt (clock-source PLL design), information-theory-coding (timing-noise channel).
**Notes:** Critical in high-speed ADCs; limits high-frequency input SFDR.

### third-order-intercept (cross-domain alias: `IP3`, `IIP3`, `intermodulation`)
**Domain:** Signal Processing / RF
**Definition:** Theoretical input level where 3rd-order intermod equals fundamental. IM3 ∝ A³; fundamental ∝ A. Lines cross at IP3.
**Atom or composite:** Atom: nonlinearity figure of merit.
**Cost model:** Two-tone test measurement.
**Real wall?** Yes — fundamental tradeoff with gain, power, linearity. Linearity-power scaling product is physical limit.
**Cross-domain wiring:** Information-theory-coding (dynamic-range = signal-distortion ratio), control-numerical-opt (DPD compensation).
**Notes:** Standard linearity metric for amplifiers, mixers, RF chains.

### one-db-compression (cross-domain alias: `1dB-CP`, `P1dB`, `gain-saturation`)
**Domain:** Signal Processing / RF
**Definition:** Input power where small-signal gain drops by 1 dB. Onset of nonlinear saturation.
**Atom or composite:** Atom: gain compression point.
**Cost model:** Measurement primitive.
**Real wall?** Yes — typically P1dB ≈ IP3 − 10 dB; defines linear operation upper limit.
**Cross-domain wiring:** Control-numerical-opt (PA backoff), information-theory-coding (PAPR-driven backoff).
**Notes:** Standard PA datasheet figure; PAPR of OFDM forces operation 6-10 dB below P1dB.

### evm-error-vector (cross-domain alias: `EVM`, `error-vector-magnitude`)
**Domain:** Signal Processing / RF
**Definition:** EVM = ‖r − s‖/‖s‖ where r is received symbol, s is reference. Captures total impairment magnitude.
**Atom or composite:** Atom: signal quality metric.
**Cost model:** O(1) per symbol.
**Real wall?** Yes — combines noise, phase, IQ, distortion into one metric. Direct conversion to BER for given constellation.
**Cross-domain wiring:** Statistics-probability (RMS deviation), ml-training (reconstruction error).
**Notes:** Industry standard for transmitter quality (3GPP, IEEE specs).

### aclr-leakage (cross-domain alias: `ACLR`, `ACPR`, `adjacent-channel-leakage`)
**Domain:** Signal Processing / RF
**Definition:** Ratio of power in adjacent channel to in-channel power: ACLR = P_adj/P_in_channel. Driven by PA nonlinearity.
**Atom or composite:** Atom: power-ratio metric.
**Cost model:** Bandpass integration over adjacent and in-channel bands.
**Real wall?** Yes — direct measure of spectral regrowth; regulatory limits (45 dBc typical for cellular).
**Cross-domain wiring:** Information-theory-coding (out-of-band emissions), control-numerical-opt (DPD design objective).
**Notes:** Primary 3GPP transmitter test; drives DPD complexity.

### papr-reduction (cross-domain alias: `crest-factor-reduction`, `PAPR`)
**Domain:** Signal Processing / RF
**Definition:** PAPR = max|x[n]|² / E[|x[n]|²]. Reduction methods: clipping+filter, SLM, PTS, ACE, tone reservation.
**Atom or composite:** Composite: signal-processing block before PA.
**Cost model:** Method-dependent; iterative clip+filter is O(N log N) per iter.
**Real wall?** Yes — PAPR reduction trades EVM, BER, spectral mask, computational cost.
**Cross-domain wiring:** Information-theory-coding (signal-shaping codes), control-numerical-opt (constrained optimization on signal).
**Notes:** Essential for OFDM/SC-FDMA. 6-10 dB PAPR typical for OFDM.

### digital-predistortion (cross-domain alias: `DPD`, `predistortion`)
**Domain:** Signal Processing / RF
**Definition:** Apply inverse nonlinearity D before PA so that PA(D(x)) ≈ G·x. Linearizes PA at high efficiency.
**Atom or composite:** Composite: PA model + inverse + adaptation.
**Cost model:** Memory-polynomial DPD: O(P·M) per sample.
**Real wall?** Yes — PA changes with temperature, aging; requires continuous adaptation. Memory effects require nonlinear-with-memory model.
**Cross-domain wiring:** Ml-training (inverse model learning), control-numerical-opt (RLS-based adaptation).
**Notes:** Heart of efficient base-station PAs in LTE/5G.

### volterra-pa-model (cross-domain alias: `Volterra-series`, `PA-nonlinear`)
**Domain:** Signal Processing / RF
**Definition:** y[n] = Σ_p Σ_{k1,…,kp} h_p[k_1,…,k_p]·Π x[n−k_i]. Full nonlinear-with-memory expansion of PA.
**Atom or composite:** Composite: multi-dimensional convolution.
**Cost model:** O(M^p) coefficients for memory M, order p.
**Real wall?** Yes — exponential parameter count; truncated/structured versions needed.
**Cross-domain wiring:** Linear-algebra-matrix (Volterra kernels), ml-training (universal nonlinear approximator).
**Notes:** Foundation of PA behavioral modeling; impractical at full generality.

### memory-polynomial (cross-domain alias: `MP-DPD`, `memory-poly-PA-model`)
**Domain:** Signal Processing / RF
**Definition:** Restricted Volterra: y[n] = Σ_m Σ_p a_{m,p}·x[n−m]·|x[n−m]|^(p−1). Diagonal sub-kernels only.
**Atom or composite:** Composite: tapped-delay-line + per-tap polynomial.
**Cost model:** O(P·M) parameters; LS fit O((PM)³).
**Real wall?** Yes — captures memory + nonlinearity tradeoff; insufficient for strong cross-memory effects.
**Cross-domain wiring:** Linear-algebra-matrix (basis-function regression), ml-training (polynomial features).
**Notes:** Ding-Zhou-Morgan 2004. Standard DPD model in industry.

### generalized-memory-polynomial (cross-domain alias: `GMP`, `Morgan-DPD`)
**Domain:** Signal Processing / RF
**Definition:** MP extended with leading + lagging envelope terms: includes x[n−m]·|x[n−m−l]|^p basis functions.
**Atom or composite:** Composite: extended basis functions.
**Cost model:** O(P·M_a·M_b·M_c) parameters.
**Real wall?** Yes — covers cross-memory effects MP misses; parameter explosion at high orders.
**Cross-domain wiring:** Ml-training (interaction features), linear-algebra-matrix (extended basis regression).
**Notes:** Morgan et al. 2006. Industry-standard DPD basis for modern PAs.

### neural-dpd (cross-domain alias: `NN-DPD`, `neural-predistorter`)
**Domain:** Signal Processing / RF
**Definition:** Replace polynomial DPD basis with neural network. Better captures complex nonlinearities at cost of training data and complexity.
**Atom or composite:** Composite: NN + LMS/RLS-style adaptation or backprop.
**Cost model:** NN inference per sample; offline training cost.
**Real wall?** Yes — overfitting / generalization to unseen signal statistics; quantization-friendly NN architectures needed for hardware.
**Cross-domain wiring:** Ml-training (deep regression), control-numerical-opt (data-driven inverse model).
**Notes:** Emerging area; potential for >dB ACLR improvement vs GMP.

### crest-factor-reduction (cross-domain alias: `CFR`, `clip-and-filter`)
**Domain:** Signal Processing / RF
**Definition:** Clip OFDM signal peaks → filter to suppress out-of-band; iterate. Reduces PAPR pre-PA without DPD.
**Atom or composite:** Composite: clipping + band-pass filtering, iterated.
**Cost model:** O(N log N) per iteration.
**Real wall?** Yes — EVM degradation traded for PAPR reduction; regulatory mask limits clipping aggressiveness.
**Cross-domain wiring:** Information-theory-coding (signal-power shaping), control-numerical-opt (constrained projection).
**Notes:** Standard pre-DPD CFR in base stations.

### slm-papr (cross-domain alias: `selected-mapping`, `SLM`)
**Domain:** Signal Processing / RF
**Definition:** Multiply data with M random phase sequences; transmit the one with lowest PAPR. Side-info index conveys chosen sequence.
**Atom or composite:** Composite: M-fold modulation + selection.
**Cost model:** O(M·N log N) per symbol.
**Real wall?** Yes — side-info overhead (log₂ M bits) and M-fold IFFT.
**Cross-domain wiring:** Statistics-probability (order statistic of PAPR), retrieval-search (best-candidate selection).
**Notes:** Bauml-Fischer-Huber 1996. Standard PAPR-reduction technique.

### pts-papr (cross-domain alias: `partial-transmit-sequence`, `PTS`)
**Domain:** Signal Processing / RF
**Definition:** Partition subcarriers into V subblocks; multiply each by phase factor; optimize phases to minimize PAPR.
**Atom or composite:** Composite: block partition + phase search.
**Cost model:** O(W^V · N log N) for W candidate phases per block.
**Real wall?** Yes — exponential phase search; suboptimal via cyclic/binary phase sets.
**Cross-domain wiring:** Information-theory-coding (joint signal-design), control-numerical-opt (combinatorial search).
**Notes:** Müller-Huber 1997. Higher gain than SLM at higher complexity.

---

## Channel Estimation & Equalization

### zero-forcing-equalizer (cross-domain alias: `ZF-equalizer`, `channel-inverse`)
**Domain:** Signal Processing / RF
**Definition:** w_ZF = 1/H(ω). Removes ISI perfectly in absence of noise but amplifies noise where |H(ω)| is small.
**Atom or composite:** Atom: channel-inverse filter.
**Cost model:** O(N) per sample for FIR equalizer.
**Real wall?** Yes — noise enhancement at spectral nulls; not applicable when H has zeros on/outside unit circle.
**Cross-domain wiring:** Linear-algebra-matrix (matrix inverse), ml-training (pseudoinverse regression).
**Notes:** Baseline equalizer; almost always outperformed by MMSE.

### mmse-equalizer (cross-domain alias: `MMSE-equalizer`, `Wiener-equalizer`)
**Domain:** Signal Processing / RF
**Definition:** w_MMSE = H*/(|H|² + N_0/E_s). Optimal linear equalizer balancing ISI and noise.
**Atom or composite:** Atom: Wiener filter on channel-symbol joint statistics.
**Cost model:** O(N) per sample.
**Real wall?** Yes — requires N_0/E_s knowledge; mismatched SNR degrades performance.
**Cross-domain wiring:** Statistics-probability (Wiener filter), ml-training (ridge regression equivalence).
**Notes:** Standard linear equalizer in modern receivers.

### decision-feedback-equalizer (cross-domain alias: `DFE`, `nonlinear-equalizer`)
**Domain:** Signal Processing / RF
**Definition:** Feedforward filter + decision device + feedback filter on decisions. Cancels post-cursor ISI without noise enhancement.
**Atom or composite:** Composite: FFE + decision device + FBF.
**Cost model:** O(N_f + N_b) per symbol.
**Real wall?** Yes — error propagation if decisions are wrong; severe at low SNR.
**Cross-domain wiring:** Control-numerical-opt (feedback cancellation), ml-training (autoregressive decoding).
**Notes:** Standard for high-SNR equalization (DSL, coherent fiber, copper Ethernet).

### mlse-viterbi (cross-domain alias: `MLSE`, `Viterbi-equalizer`)
**Domain:** Signal Processing / RF
**Definition:** Maximum-likelihood sequence estimator: search Viterbi trellis where states are recent symbol history, branches weighted by Euclidean distance.
**Atom or composite:** Composite: trellis + Viterbi algorithm.
**Cost model:** O(M^L · N) for M-ary symbols, channel length L.
**Real wall?** Yes — exponential in channel length; reduced-state DFSE alternatives.
**Cross-domain wiring:** Retrieval-search (Viterbi shortest path), ml-training (CRF inference).
**Notes:** Forney 1972. Optimal for known channel; basis for soft-output extension to LDPC/turbo.

### turbo-equalization (cross-domain alias: `turbo-EQ`, `iterative-equalize-decode`)
**Domain:** Signal Processing / RF
**Definition:** Iterate between MAP equalizer (BCJR or SOVA on channel trellis) and MAP decoder (BCJR on code trellis) via interleaver.
**Atom or composite:** Composite: BCJR equalizer + BCJR decoder + iteration.
**Cost model:** O(N · (M^L + 2^v)) per iter.
**Real wall?** Yes — convergence depends on EXIT chart; closes gap to capacity over ISI channels.
**Cross-domain wiring:** Ml-training (iterative inference), information-theory-coding (turbo principle on ISI).
**Notes:** Douillard et al. 1995. Closes ~3 dB vs equalize-then-decode at moderate iterations.

### ls-channel-estimation (cross-domain alias: `LS-CE`, `pilot-based-CE`)
**Domain:** Signal Processing / RF
**Definition:** With pilots X_p, Y_p: Ĥ = Y_p./X_p (per-subcarrier division). Simple unbiased estimator.
**Atom or composite:** Atom: element-wise division.
**Cost model:** O(N_p) per OFDM symbol.
**Real wall?** Yes — noise unmitigated; needs interpolation across non-pilot subcarriers.
**Cross-domain wiring:** Linear-algebra-matrix (least squares), statistics-probability (unbiased estimator).
**Notes:** Default OFDM CE; refined via MMSE/Kalman.

### mmse-channel-estimation (cross-domain alias: `MMSE-CE`, `Wiener-CE`)
**Domain:** Signal Processing / RF
**Definition:** Ĥ = R_HH·(R_HH + σ²·I)⁻¹·Ĥ_LS. Uses channel correlation prior for noise suppression.
**Atom or composite:** Composite: covariance prior + Wiener filter.
**Cost model:** O(N²) per OFDM symbol; O(N) with structured R_HH.
**Real wall?** Yes — requires knowledge of R_HH; mismatched prior degrades performance.
**Cross-domain wiring:** Statistics-probability (Bayesian estimation), ml-training (prior-informed regression).
**Notes:** Substantial gain over LS-CE at low SNR.

### kalman-channel-tracking (cross-domain alias: `KCE`, `Kalman-CE`)
**Domain:** Signal Processing / RF
**Definition:** Treat channel taps as state evolving via AR model: h_{n+1} = A·h_n + w_n. Kalman recursions track time-varying channel.
**Atom or composite:** Composite: AR channel model + Kalman filter.
**Cost model:** O(L³) per pilot/symbol for L-tap channel.
**Real wall?** Yes — requires AR model parameters (Doppler-dependent); model mismatch costs tracking accuracy.
**Cross-domain wiring:** Control-numerical-opt (Kalman tracking), statistics-probability (time-varying state).
**Notes:** Used in high-mobility scenarios (vehicular, aeronautical).

### blind-channel-estimation (cross-domain alias: `blind-CE`, `subspace-CE`)
**Domain:** Signal Processing / RF
**Definition:** Estimate H without pilots, exploiting signal structure (cyclostationarity, subspace, constant modulus).
**Atom or composite:** Composite: signal-structure exploitation + subspace methods.
**Cost model:** O(N³) for eigendecomposition.
**Real wall?** Yes — pilot-saved overhead vs slower convergence and identifiability ambiguities.
**Cross-domain wiring:** Linear-algebra-matrix (subspace decomposition), retrieval-search (self-similar pattern recovery).
**Notes:** Tong-Xu-Kailath 1994. Useful for spectrally efficient links and pilot-contaminated systems.

### semi-blind-channel (cross-domain alias: `semi-blind-CE`)
**Domain:** Signal Processing / RF
**Definition:** Combine sparse pilots with blind structural priors. Lower pilot overhead than pilot-only; faster convergence than fully blind.
**Atom or composite:** Composite: pilot-based estimate + blind refinement.
**Cost model:** Hybrid of LS-CE and blind methods.
**Real wall?** Yes — design trade-off between pilot overhead and blind convergence rate.
**Cross-domain wiring:** Ml-training (semi-supervised learning analogy).
**Notes:** Common in modern systems where pilot overhead is squeezed.

### pilot-interpolation (cross-domain alias: `2D-channel-interp`, `Wiener-2D-interp`)
**Domain:** Signal Processing / RF
**Definition:** Estimate channel at non-pilot subcarriers/symbols via 2D interpolation across (frequency, time). Linear, spline, or Wiener-optimal.
**Atom or composite:** Composite: 2D interpolation operator.
**Cost model:** O(N_d) for linear; O(N_d·K) for kernel methods.
**Real wall?** Yes — pilot density set by Nyquist on Doppler/delay spread.
**Cross-domain wiring:** Graphics-rendering-lod (2D upsampling), statistics-probability (kriging/Gaussian process).
**Notes:** Standard processing for LTE/5G OFDM channels.

---

## Diversity Combining

### selection-combining (cross-domain alias: `SC-diversity`, `select-best-branch`)
**Domain:** Signal Processing / RF
**Definition:** Use only the branch with highest instantaneous SNR. Simple but suboptimal vs MRC.
**Atom or composite:** Composite: SNR estimation + branch switching.
**Cost model:** O(M) per symbol for M branches.
**Real wall?** Yes — gain over single branch saturates: E[γ] = γ̄·Σ(1/k); slow growth.
**Cross-domain wiring:** Retrieval-search (max-selection), statistics-probability (order statistic).
**Notes:** Easiest diversity scheme; baseline for comparison.

### equal-gain-combining (cross-domain alias: `EGC`, `unit-weighted-combining`)
**Domain:** Signal Processing / RF
**Definition:** y = Σ exp(−jφ_i)·y_i (co-phase only, no amplitude weighting). Simpler than MRC; loses ~1 dB.
**Atom or composite:** Composite: phase compensation + sum.
**Cost model:** O(M).
**Real wall?** Yes — requires phase estimation per branch; sensitive to estimation errors.
**Cross-domain wiring:** Linear-algebra-matrix (unit-norm weighting), statistics-probability (phase-aligned averaging).
**Notes:** Used in low-complexity receivers.

### maximum-ratio-combining (cross-domain alias: `MRC`, `optimal-combining`)
**Domain:** Signal Processing / RF
**Definition:** w_i = h_i*/N_0 → SNR_out = Σ |h_i|²/N_0. Optimal under AWGN; achieves full diversity order M.
**Atom or composite:** Atom: weighted-sum with conjugate channel.
**Cost model:** O(M).
**Real wall?** Yes — requires accurate channel knowledge; mismatched weights degrade gain.
**Cross-domain wiring:** Linear-algebra-matrix (matched filter), statistics-probability (Bayes-optimal combining).
**Notes:** Brennan 1959. Theoretical optimum for SIMO diversity.

### switched-diversity (cross-domain alias: `SwSC`, `switch-and-stay`)
**Domain:** Signal Processing / RF
**Definition:** Use current branch until SNR drops below threshold; switch to next. Avoids continuous monitoring.
**Atom or composite:** Composite: SNR estimator + threshold + switching logic.
**Cost model:** O(1) per symbol (single branch).
**Real wall?** Yes — performance bounded by selection combining; switching transient costs.
**Cross-domain wiring:** Control-numerical-opt (hysteresis switching).
**Notes:** Used when continuous multi-branch processing is too expensive.

---

## 5G / 6G Specifics

### nr-numerology (cross-domain alias: `5G-NR-subcarriers`, `scalable-OFDM-μ`)
**Domain:** Signal Processing / RF
**Definition:** 5G NR supports subcarrier spacing 15·2^μ kHz for μ ∈ {0,…,4}. Slot durations scale inversely.
**Atom or composite:** Atom: parametric OFDM configuration.
**Cost model:** Same FFT size; varying symbol duration.
**Real wall?** Yes — μ trades latency (shorter slots) vs phase-noise tolerance (wider subcarriers).
**Cross-domain wiring:** Information-theory-coding (latency-throughput tradeoffs), control-numerical-opt (resource allocation).
**Notes:** Foundation of 5G NR; mmWave bands use μ ∈ {3, 4} (120/240 kHz).

### mini-slot (cross-domain alias: `non-slot-based`, `5G-low-latency`)
**Domain:** Signal Processing / RF
**Definition:** Transmission of 2/4/7 OFDM symbols rather than full slot (14). Enables URLLC < 1 ms latency.
**Atom or composite:** Atom: subframe granularity.
**Cost model:** Higher per-symbol overhead due to control signaling per mini-slot.
**Real wall?** Yes — mini-slots have less code averaging; small block-error tradeoffs.
**Cross-domain wiring:** Control-numerical-opt (low-latency control loops), information-theory-coding (URLLC reliability).
**Notes:** Standard 5G URLLC mechanism for sub-ms latency.

### beam-tracking (cross-domain alias: `beam-tracking-5G`, `mmWave-beam-mgmt`)
**Domain:** Signal Processing / RF
**Definition:** Periodically update analog beam to follow UE in mmWave. Combines coarse beam-sweep with fine beam-refinement (CSI-RS).
**Atom or composite:** Composite: beam-sweep + measurement + beam-update.
**Cost model:** O(N_beams) sweep time per UE.
**Real wall?** Yes — beam-coherence time at mmWave is short (~ms); blockage causes sudden loss.
**Cross-domain wiring:** Retrieval-search (sequential beam-narrowing), control-numerical-opt (closed-loop pointing).
**Notes:** Critical for mmWave 5G/6G performance; failure causes outages.

### reconfigurable-intelligent-surface (cross-domain alias: `RIS`, `IRS`, `smart-surface`)
**Domain:** Signal Processing / RF
**Definition:** Passive metasurface with reconfigurable elements; each reflects with controllable phase. Creates configurable propagation channel.
**Atom or composite:** Atom: passive beamforming layer.
**Cost model:** No active processing; phase control bit count limits resolution.
**Real wall?** Yes — passive elements have hardware limits on phase resolution; channel estimation challenging through RIS.
**Cross-domain wiring:** Control-numerical-opt (passive control), linear-algebra-matrix (cascaded channel estimation).
**Notes:** Direnzo et al. 2020. Emerging 6G technology for coverage and capacity enhancement.

### oam-multiplexing (cross-domain alias: `orbital-angular-momentum`, `OAM-mode`)
**Domain:** Signal Processing / RF
**Definition:** Use vortex EM modes (e^{jℓφ}) as orthogonal channels. Theoretical capacity gain in LoS scenarios.
**Atom or composite:** Atom: spatial mode division.
**Cost model:** O(M) for M-mode multiplexing.
**Real wall?** Yes — modes are subset of MIMO eigenmodes; pure-LoS only. Non-orthogonality at receiver in NLoS.
**Cross-domain wiring:** Linear-algebra-matrix (orthogonal basis), information-theory-coding (LoS-MIMO subspace).
**Notes:** Edfors-Johansson 2012. Experimental for fixed wireless backhaul.

---

## Cognitive Radio & Sensing

### energy-detection-sensing (cross-domain alias: `ED`, `radiometer`, `energy-CFAR`)
**Domain:** Signal Processing / RF
**Definition:** Compute E = Σ |y[n]|² over N samples; compare to threshold derived from noise floor and false-alarm rate.
**Atom or composite:** Atom: power integration + threshold.
**Cost model:** O(N) per decision.
**Real wall?** Yes — SNR wall: cannot detect below noise-uncertainty-determined SNR irrespective of N.
**Cross-domain wiring:** Statistics-probability (Neyman-Pearson detection), retrieval-search (signal-presence binary classifier).
**Notes:** Foundation of spectrum sensing; suffers from noise-power uncertainty.

### matched-filter-sensing (cross-domain alias: `MF-sensing`, `known-signal-detection`)
**Domain:** Signal Processing / RF
**Definition:** Coherent matched filter for known primary signal. Optimal Neyman-Pearson detector under AWGN.
**Atom or composite:** Atom: correlator + threshold.
**Cost model:** O(N) per decision.
**Real wall?** Yes — requires sync and signal knowledge; impractical for many spectrum-sensing use cases.
**Cross-domain wiring:** Statistics-probability (likelihood ratio test).
**Notes:** Theoretical upper bound for spectrum-sensing performance.

### cyclostationary-sensing (cross-domain alias: `cyclic-feature-sensing`, `SCD-detection`)
**Domain:** Signal Processing / RF
**Definition:** Detect signal by presence of cyclic features (modulation/symbol-rate spectral lines). Robust to noise-uncertainty.
**Atom or composite:** Composite: spectral correlation density + peak detection.
**Cost model:** O(N²).
**Real wall?** Yes — works only if signal has known cyclic frequencies; pure white noise has no cyclic features.
**Cross-domain wiring:** Statistics-probability (cyclostationarity), retrieval-search (modulation-fingerprint detection).
**Notes:** Gardner 1988. Standard for primary-user detection in cognitive radio.

---

## Source Separation & Other Advanced

### fast-ica (cross-domain alias: `FastICA`, `Hyvarinen-ICA`)
**Domain:** Signal Processing / RF
**Definition:** Find unmixing matrix W such that components of Wx are maximally non-Gaussian. Uses fixed-point iteration on contrast function.
**Atom or composite:** Composite: whitening + fixed-point iteration.
**Cost model:** O(N²·K) for K-source N-sample.
**Real wall?** Yes — requires non-Gaussian sources; cannot separate Gaussian mixtures; underdetermined case needs sparsity.
**Cross-domain wiring:** Ml-training (representation learning), statistics-probability (independence test).
**Notes:** Hyvarinen 1999. Standard ICA algorithm; faster than gradient methods.

### jade-ica (cross-domain alias: `JADE`, `joint-approx-diag-eigenmatrix`)
**Domain:** Signal Processing / RF
**Definition:** Diagonalize fourth-order cumulant tensor via joint approximate diagonalization of multiple cumulant matrices.
**Atom or composite:** Composite: tensor computation + joint diagonalization.
**Cost model:** O(N⁴) for cumulant computation; O(K²) joint diag.
**Real wall?** Yes — sensitive to outliers (cumulant-based); requires sufficient samples.
**Cross-domain wiring:** Linear-algebra-matrix (joint diagonalization), ml-training (higher-order moment learning).
**Notes:** Cardoso-Souloumiac 1993. Foundational alternative to FastICA.

### iva-source-separation (cross-domain alias: `IVA`, `independent-vector-analysis`)
**Domain:** Signal Processing / RF
**Definition:** Multi-channel ICA on STFT bins with dependency modeling within sources across frequency. Resolves frequency-domain permutation ambiguity.
**Atom or composite:** Composite: STFT + per-bin ICA + cross-bin dependency.
**Cost model:** O(B·N·K²) for B bins.
**Real wall?** Yes — better than per-bin ICA for convolutive mixtures (e.g., speech separation in rooms).
**Cross-domain wiring:** Ml-training (multi-view learning), statistics-probability (joint independence).
**Notes:** Kim et al. 2007. Standard for blind speech separation in real rooms.

### kurtosis-detection (cross-domain alias: `4th-moment`, `non-Gaussianity`)
**Domain:** Signal Processing / RF
**Definition:** κ = E[x⁴]/E[x²]² − 3. Non-zero indicates non-Gaussian distribution; signs distinguish super-/sub-Gaussian.
**Atom or composite:** Atom: 4th-order normalized moment.
**Cost model:** O(N).
**Real wall?** Yes — sensitive to outliers; variance grows with N⁻¹ slowly.
**Cross-domain wiring:** Statistics-probability (higher-order moments), ml-training (peakedness features).
**Notes:** Used in ICA, signal/noise discrimination, modulation classification.

### higher-order-statistics (cross-domain alias: `HOS`, `cumulants-bispectrum`)
**Domain:** Signal Processing / RF
**Definition:** 3rd-/4th-order cumulants and their Fourier transforms (bispectrum, trispectrum). Capture non-Gaussian and nonlinear properties.
**Atom or composite:** Composite: multi-dimensional moments + Fourier.
**Cost model:** O(N²) bispectrum; O(N^k) for k-th cumulant.
**Real wall?** Yes — high estimator variance; needs large N for reliable estimates.
**Cross-domain wiring:** Statistics-probability (cumulant theory), ml-training (nonlinear feature extraction).
**Notes:** Nikias-Petropulu textbook. Foundation of nonlinear signal analysis.

### bispectrum (cross-domain alias: `3rd-order-spectrum`, `phase-coupling`)
**Domain:** Signal Processing / RF
**Definition:** B(ω_1, ω_2) = E[X(ω_1)·X(ω_2)·X*(ω_1+ω_2)]. Detects quadratic phase coupling between frequencies.
**Atom or composite:** Composite: triple-product spectrum.
**Cost model:** O(N²) computation; O(N²) storage.
**Real wall?** Yes — high variance estimator; needs averaging or parametric models for reliable detection.
**Cross-domain wiring:** Statistics-probability (3rd-order spectra), retrieval-search (frequency-coupling fingerprinting).
**Notes:** Used in biomedical (EEG coupling), oceanography (wave interactions), modulation classification.

---

## Generalized Bus & Composition Notes

### filterbank-as-bus (cross-domain alias: `subband-bus`, `multi-rate-bus`)
**Domain:** Signal Processing / RF
**Definition:** Treat M-band filterbank as a parallel bus: each band is a stream; on-ramps add new sources, off-ramps consume.
**Atom or composite:** Composite: analysis filterbank + per-band processing + synthesis.
**Cost model:** O(N log M) per sample with polyphase + FFT.
**Real wall?** No — composition primitive (Jesse's spiderweb bus pattern applied to DSP).
**Cross-domain wiring:** Spiderweb-bus pattern, retrieval-search (multi-band routing), graphics-rendering-lod (multi-scale composition).
**Notes:** Aligns DSP with Jesse's bus doctrine — bands are parallel highway lanes.

### dsp-graph-execution (cross-domain alias: `signal-flow-graph`, `dataflow-DSP`)
**Domain:** Signal Processing / RF
**Definition:** Represent DSP as directed graph; nodes are atomic operators (filters, transforms, decimators), edges carry streams.
**Atom or composite:** Composite: graph + scheduling.
**Cost model:** Dictated by atomic-operator costs and graph topology.
**Real wall?** No — composition pattern.
**Cross-domain wiring:** Spiderweb-bus (parallel execution lanes), retrieval-search (dataflow query plans), ml-training (computation graphs).
**Notes:** Foundation of GNU Radio, MATLAB Simulink, LabVIEW. The doctrine in code: composition over cracking the engine.

### blockfloat-dsp (cross-domain alias: `block-floating-point`, `BFP-DSP`)
**Domain:** Signal Processing / RF
**Definition:** Per-block shared exponent + per-sample mantissa. Balances fixed-point speed with floating-point dynamic range.
**Atom or composite:** Composite: scaling + fixed-point ops.
**Cost model:** O(1) per sample; block-level exponent update.
**Real wall?** Yes — block-size choice trades dynamic-range tracking vs scaling overhead.
**Cross-domain wiring:** Information-theory-coding (efficient numerical representation), ml-training (mixed-precision training).
**Notes:** Standard in fixed-point DSP processors; resurgent in low-power ML inference.

### overlap-save-convolution (cross-domain alias: `OLS`, `overlap-save`)
**Domain:** Signal Processing / RF
**Definition:** FFT-based fast convolution: block input into overlapping segments → FFT × filter → IFFT → discard transient.
**Atom or composite:** Composite: block + FFT + IFFT + assembly.
**Cost model:** O(L·log L) per output sample for block size L; ~50× faster than direct convolution for long filters.
**Real wall?** Yes — block boundary handling determines correctness; overlap = filter length − 1.
**Cross-domain wiring:** Graphics-rendering-lod (block-tile FFTs), retrieval-search (fast batch correlation).
**Notes:** Stockham 1966. Default fast-convolution algorithm.

### overlap-add-convolution (cross-domain alias: `OLA`, `overlap-add`)
**Domain:** Signal Processing / RF
**Definition:** Block input into non-overlapping segments → convolve each with filter → add overlapping tails.
**Atom or composite:** Composite: block + FFT + IFFT + accumulation.
**Cost model:** Same as overlap-save.
**Real wall?** Yes — needs more memory for accumulation; preferred when input arrives in well-defined blocks.
**Cross-domain wiring:** Graphics-rendering-lod (overlap-add image filtering), ml-training (block-based gradient accumulation).
**Notes:** OLA vs OLS choice is implementation preference; both compute same result.

*Last updated: 2026-06-25 (added 250 primitives covering advanced filter design, adaptive filters, multirate, time-frequency, wavelets, spectral estimation, array processing, MIMO, radar, synchronization, coding, quantization, speech/audio, compressed sensing, RF hardware, channel estimation, diversity, 5G/6G, source separation)*
*Source doctrine: The Painted Fence — Jesse*

