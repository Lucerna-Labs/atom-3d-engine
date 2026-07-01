# Photonics & Optics — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## Geometric Optics

### ray-equation (cross-domain alias: `ray-trace`, `eikonal-ray`, `optical-path-ray`)
**Domain:** Photonics / Optics
**Definition:** d/ds[n(r)·dr/ds] = ∇n(r); rays are characteristics of the eikonal equation |∇S|² = n².
**Atom or composite:** Atom (the fundamental geometric-optics primitive).
**Cost model:** O(N) per ray-step; integration cost scales with sample count along path.
**Real wall?** Yes — geometric optics breaks down when λ ~ feature size; diffraction takes over.
**Cross-domain wiring:** Same characteristic-curve formalism as physics-diffusion (Hamilton-Jacobi), graphics-rendering-lod (path tracing), signal-processing-rf (geometric-theory-of-diffraction).
**Notes:** Born & Wolf §3.2. Ray = energy-flow direction in the short-wave limit.

### fermat-principle (cross-domain alias: `least-time`, `stationary-OPL`, `variational-ray`)
**Domain:** Photonics / Optics
**Definition:** Light follows the path P that makes the optical path length ∫n(r)ds stationary: δ∫n ds = 0.
**Atom or composite:** Atom (variational principle generating ray-equation).
**Cost model:** Variational solve = root-finding in path space; O(M) candidate paths.
**Real wall?** No — it's a principle, not a computation.
**Cross-domain wiring:** Mirror of physics-classical-mech (least action), control-numerical-opt (shooting methods), graphics-rendering-lod (Metropolis light transport sampling stationary paths).
**Notes:** Generalizes Hero's reflection-principle; basis for caustics, Maupertuis.

### snell-law (cross-domain alias: `refraction-law`, `descartes-law`, `interface-bend`)
**Domain:** Photonics / Optics
**Definition:** n₁ sin θ₁ = n₂ sin θ₂; vector form: n₁(k̂₁×n̂) = n₂(k̂₂×n̂) with n̂ the surface normal.
**Atom or composite:** Atom (boundary primitive).
**Cost model:** O(1) per interface; vector form is two cross products and a normalize.
**Real wall?** Yes — total internal reflection above θ_c = arcsin(n₂/n₁) blocks transmission, an interface-physics wall.
**Cross-domain wiring:** Same form as electromagnetics-antennas (Fresnel) and signal-processing-rf (Snell-for-acoustics in layered media); networking analogue: routing-cost gradient at interface.
**Notes:** Born & Wolf §1.5. Reduces to Fermat's principle at flat interface.

### snell-vector-3d (cross-domain alias: `vector-snell`, `3d-refraction`, `ray-bend`)
**Domain:** Photonics / Optics
**Definition:** t̂ = (n₁/n₂)î + (n₁/n₂ cos θᵢ − cos θₜ)n̂, with cos θₜ = √(1−(n₁/n₂)² sin²θᵢ).
**Atom or composite:** Composite (Snell scalar + normal-component reconstruction).
**Cost model:** O(1) per ray; single sqrt + dot products.
**Real wall?** Yes — when discriminant negative, total internal reflection; no transmitted ray.
**Cross-domain wiring:** Used directly by graphics-rendering-lod refractive shaders; analog in seismic ray tracing (signal-processing-rf).
**Notes:** Glassner §5; the form ray tracers actually implement.

### thin-lens-equation (cross-domain alias: `lensmaker-thin`, `1/f-rule`, `imaging-eq`)
**Domain:** Photonics / Optics
**Definition:** 1/s + 1/s′ = 1/f, where s = object distance, s′ = image distance, f = focal length.
**Atom or composite:** Composite (paraxial limit of full refraction at two surfaces).
**Cost model:** O(1) algebra; gives image location and magnification m = −s′/s.
**Real wall?** No — but breaks down off-axis (aberrations) and at finite thickness.
**Cross-domain wiring:** Same harmonic-conjugate algebra as electromagnetics-antennas focal-feed geometry, signal-processing-rf time-bandwidth focusing.
**Notes:** Hecht §6.2; sign convention varies (Cartesian vs real-is-positive).

### thick-lens-equation (cross-domain alias: `lensmaker`, `R1-R2-d-formula`)
**Domain:** Photonics / Optics
**Definition:** 1/f = (n−1)[1/R₁ − 1/R₂ + (n−1)d/(nR₁R₂)] — full lensmaker.
**Atom or composite:** Composite (two refractions + translation through glass).
**Cost model:** O(1); used during lens design.
**Real wall?** No.
**Cross-domain wiring:** Decomposes to product of ABCD matrices — see abcd-matrix; relates to linear-algebra-matrix (small matrix products).
**Notes:** Born & Wolf §4.4; principal-plane locations derived from this.

### abcd-matrix (cross-domain alias: `ray-matrix`, `system-matrix`, `paraxial-matrix`)
**Domain:** Photonics / Optics
**Definition:** [y′; θ′] = [A B; C D][y; θ]; cascading elements = matrix multiplication.
**Atom or composite:** Composite (algebra over atomic interfaces: refraction, translation, mirror).
**Cost model:** O(N) 2×2 matrix products for N surfaces.
**Real wall?** No — paraxial only; fails for large θ.
**Cross-domain wiring:** Identical algebra in linear-algebra-matrix; quantum-computing 2×2 unitary cascading; control-numerical-opt state-space.
**Notes:** Siegman §15; determinant = n₁/n₂ tracks index change.

### paraxial-approximation (cross-domain alias: `small-angle`, `linearize-rays`, `gaussian-optics`)
**Domain:** Photonics / Optics
**Definition:** sin θ ≈ θ, cos θ ≈ 1, tan θ ≈ θ — linearize all geometric optics around the optical axis.
**Atom or composite:** Atom (the linearization underlying ABCD and lensmaker).
**Cost model:** Free; collapses transcendentals to linear ops.
**Real wall?** No — accuracy bounded by θ³/6 (next Seidel term).
**Cross-domain wiring:** Same role as small-signal linearization in control-numerical-opt; first-order expansion in physics-diffusion perturbation theory.
**Notes:** First-order Taylor of full ray-trig; aberrations are the higher-order remainder.

### aberration-spherical (cross-domain alias: `seidel-S1`, `radial-defocus`)
**Domain:** Photonics / Optics
**Definition:** Wavefront error proportional to ρ⁴: W = a₀₄₀ ρ⁴ — outer rays focus closer than paraxial.
**Atom or composite:** Composite (third-order departure from paraxial).
**Cost model:** Evaluated as one Zernike term; correction needs aspheric or doublet.
**Real wall?** No — corrected by aspherics; lambda-scale residual remains.
**Cross-domain wiring:** Same 4th-order term shows up in graphics-rendering-lod bokeh shape; control-numerical-opt nonlinearity expansion.
**Notes:** Born & Wolf §5.3; the dominant aberration of single spherical lens.

### aberration-coma (cross-domain alias: `seidel-S2`, `off-axis-fan`, `comet-tail`)
**Domain:** Photonics / Optics
**Definition:** W = a₁₃₁ ρ³ cos φ — off-axis points form comet-shaped images.
**Atom or composite:** Composite (third-order Seidel term, antisymmetric in field).
**Cost model:** O(1) per ray; corrected by stop position or doublet.
**Real wall?** No — Abbe sine condition eliminates coma for axial point.
**Cross-domain wiring:** Phase-error term identical to electromagnetics-antennas off-axis array pattern; signal-processing-rf phased-array squint.
**Notes:** Hecht §6.3.2; "coma-free" systems satisfy y′/sin θ = const.

### aberration-astigmatism (cross-domain alias: `seidel-S3`, `tangential-sagittal`)
**Domain:** Photonics / Optics
**Definition:** W = a₂₂₂ ρ² cos² φ — tangential and sagittal foci separate axially.
**Atom or composite:** Composite (third-order, quadratic in field and aperture).
**Cost model:** Two focal surfaces instead of one; metric = midpoint blur.
**Real wall?** No — anastigmat designs (Petzval) correct it.
**Cross-domain wiring:** Same orthogonal-axis foci show in plasma physics (X-ray mirrors), graphics-rendering-lod elliptical filtering.
**Notes:** Born & Wolf §5.3.

### aberration-field-curvature (cross-domain alias: `seidel-S4`, `petzval`)
**Domain:** Photonics / Optics
**Definition:** Best image surface curves: 1/R_p = Σ(1/(n_iₐ f_i)) — Petzval sum.
**Atom or composite:** Composite (sum over surfaces; flat field requires Σ = 0).
**Cost model:** O(N) over surfaces; design constraint.
**Real wall?** Yes — fundamentally curved field for single positive lens; flattener required.
**Cross-domain wiring:** Same as graphics-rendering-lod sensor-curvature emulation; appears in MRI gradient-coil fields (physics-diffusion).
**Notes:** Hecht §6.3.4.

### aberration-distortion (cross-domain alias: `seidel-S5`, `barrel-pincushion`)
**Domain:** Photonics / Optics
**Definition:** W ∝ ρ cos φ · h³ — image height changes nonlinearly with field; barrel (negative) or pincushion (positive).
**Atom or composite:** Composite (geometric mapping error, no blur).
**Cost model:** Post-correctable in software; field-dependent only.
**Real wall?** No — pure remap, no information loss.
**Cross-domain wiring:** Image undistortion is the inverse of graphics-rendering-lod lens-shading; same algebra as networking topology-distortion.
**Notes:** Hecht §6.3.5; cheap to remove computationally.

### aberration-chromatic (cross-domain alias: `axial-color`, `lateral-color`, `dispersion-blur`)
**Domain:** Photonics / Optics
**Definition:** f(λ) varies with index n(λ); axial color = different focal planes per λ; lateral color = different magnification per λ.
**Atom or composite:** Composite (dispersion + imaging law).
**Cost model:** Achromat design = solving Σ(φ_i/V_i) = 0 across glasses.
**Real wall?** Yes — material dispersion is a real-material wall; corrected only over discrete λ.
**Cross-domain wiring:** Same dispersion algebra in signal-processing-rf (group-velocity dispersion), fiber-optics chromatic-dispersion; graphics-rendering-lod spectral rendering.
**Notes:** Abbe V-number quantifies dispersion; apo correction = 3 λ matched.

### seidel-aberrations (cross-domain alias: `third-order`, `primary-aberrations`)
**Domain:** Photonics / Optics
**Definition:** Five third-order monochromatic aberrations: S1–S5 (spherical, coma, astigmatism, field-curvature, distortion), summed over surfaces.
**Atom or composite:** Composite (five-tuple from a surface-by-surface sum).
**Cost model:** O(N_surfaces); foundation of classical lens design.
**Real wall?** No — but coupled: minimizing one often grows another.
**Cross-domain wiring:** Same kind of orthogonal-error decomposition as PCA basis in linear-algebra-matrix; trade-offs analogous to control-numerical-opt Pareto fronts.
**Notes:** Born & Wolf §5.3.

### zernike-polynomial (cross-domain alias: `Zernike-basis`, `orthogonal-aperture-polys`)
**Domain:** Photonics / Optics
**Definition:** Z_n^m(ρ,φ) = R_n^m(ρ) · cos(mφ) or sin(mφ); orthogonal over unit disk: ∫Z_iZ_j = δ_ij π/(n+1).
**Atom or composite:** Composite (basis decomposition of wavefront).
**Cost model:** O(K) coefficients; FFT-like fits with discrete-orthogonal samples.
**Real wall?** No.
**Cross-domain wiring:** Disk analog of spherical harmonics (graphics-rendering-lod, electromagnetics-antennas); orthogonal-basis like Walsh in signal-processing-rf.
**Notes:** Noll indexing standard; first 9 cover piston-tilt-defocus-astig-coma-spherical.

### wavefront-aberration-W (cross-domain alias: `OPD-map`, `phase-error`, `W(ρ,φ)`)
**Domain:** Photonics / Optics
**Definition:** W(ρ,φ) = optical-path-difference between actual wavefront and ideal sphere referenced to exit pupil.
**Atom or composite:** Composite (sum of Zernike or Seidel terms).
**Cost model:** Sample at pupil grid; FFT for PSF.
**Real wall?** Yes — Maréchal criterion: RMS W ≤ λ/14 for "diffraction-limited."
**Cross-domain wiring:** Same role as channel-response in signal-processing-rf; phase-aberration in electromagnetics-antennas array calibration.
**Notes:** Born & Wolf §9.

### imaging-system (cross-domain alias: `lens-system`, `optical-relay`)
**Domain:** Photonics / Optics
**Definition:** A cascade of refracting/reflecting surfaces mapping object plane → image plane with magnification m and aberrations.
**Atom or composite:** Composite (sequence of refractions, translations, stops).
**Cost model:** ABCD product + ray-aberration sum.
**Real wall?** Yes — étendue (n²A·Ω) conservation limits brightness.
**Cross-domain wiring:** Conserves étendue analogous to phase-space volume (physics-classical), information-theory-coding channel capacity.
**Notes:** Smith §3.

### focal-length (cross-domain alias: `f`, `EFL`, `power=1/f`)
**Domain:** Photonics / Optics
**Definition:** Distance from rear principal plane to image of object at infinity. Power φ = 1/f.
**Atom or composite:** Atom (defining scalar of an imaging element).
**Cost model:** O(1); core design parameter.
**Real wall?** No.
**Cross-domain wiring:** Analog of beam-steering coefficient in electromagnetics-antennas; gain in control-numerical-opt.
**Notes:** Hecht §5.2.

### principal-planes (cross-domain alias: `H1-H2`, `unit-mag-planes`)
**Domain:** Photonics / Optics
**Definition:** Conjugate planes of unit transverse magnification; thick-lens equation uses distances measured from them.
**Atom or composite:** Composite (derived from ABCD trace and powers).
**Cost model:** O(1) after ABCD known.
**Real wall?** No.
**Cross-domain wiring:** Analogous to characteristic impedance reference planes in signal-processing-rf.
**Notes:** Born & Wolf §4.3.

### nodal-points (cross-domain alias: `N1-N2`, `unit-angular-mag`)
**Domain:** Photonics / Optics
**Definition:** Conjugate axial points with unit angular magnification: ray through N1 emerges parallel through N2.
**Atom or composite:** Composite.
**Cost model:** O(1).
**Real wall?** No.
**Cross-domain wiring:** Conjugate-rotation pivot — analog to pivot of stereographic projection (graphics-rendering-lod).
**Notes:** Coincides with principal points if object/image media identical.

### pupil-aperture-stop (cross-domain alias: `aperture-stop`, `entrance-pupil`, `exit-pupil`)
**Domain:** Photonics / Optics
**Definition:** Aperture stop limits axial bundle; entrance/exit pupils are its images in object/image space.
**Atom or composite:** Composite (image of physical stop through preceding/following elements).
**Cost model:** ABCD-trace of stop edges.
**Real wall?** Yes — pupil size sets diffraction limit (λ/D).
**Cross-domain wiring:** Same role as antenna aperture in electromagnetics-antennas; window-function in signal-processing-rf.
**Notes:** Saleh & Teich §4.4.

### field-stop (cross-domain alias: `FOV-stop`, `image-frame`)
**Domain:** Photonics / Optics
**Definition:** The stop that limits the field-of-view (image extent), distinct from aperture stop.
**Atom or composite:** Atom (geometric boundary).
**Cost model:** O(1) clip.
**Real wall?** No.
**Cross-domain wiring:** Identical to sensor crop in graphics-rendering-lod; gating in signal-processing-rf.
**Notes:** Smith §6.

### vignetting (cross-domain alias: `cos⁴-falloff`, `mechanical-vignette`)
**Domain:** Photonics / Optics
**Definition:** Reduction of irradiance away from optical axis: natural cos⁴θ + mechanical clipping by non-pupil elements.
**Atom or composite:** Composite (geometric + photometric).
**Cost model:** Per-pixel multiplicative correction.
**Real wall?** Partial — cos⁴ is physical; mechanical is design-correctable.
**Cross-domain wiring:** Analog to antenna pattern roll-off (electromagnetics-antennas); spatial-window in signal-processing-rf.
**Notes:** Kingslake §6.

### depth-of-field (cross-domain alias: `DoF`, `near-far-acceptable-sharpness`)
**Domain:** Photonics / Optics
**Definition:** Range of object distances over which blur ≤ circle of confusion c: DoF ≈ 2Nc·s²/f² for small c.
**Atom or composite:** Composite (derived from defocus blur + acceptable-blur threshold).
**Cost model:** O(1).
**Real wall?** Yes — fundamental diffraction-blur floor when stopped down: c_diff ≈ 2.44 λ N.
**Cross-domain wiring:** Mirrors depth-of-focus in graphics-rendering-lod bokeh; coherence-length analog in signal-processing-rf.
**Notes:** Hecht §6.

### depth-of-focus (cross-domain alias: `DoFo`, `image-side-DoF`)
**Domain:** Photonics / Optics
**Definition:** Image-side counterpart: range of sensor positions giving acceptable blur, ≈ ±2λN² (diffraction).
**Atom or composite:** Composite.
**Cost model:** O(1).
**Real wall?** Yes — Rayleigh limit fundamental.
**Cross-domain wiring:** Same algebra as Rayleigh range in Gaussian-beam optics.
**Notes:** Born & Wolf §8.

### mtf (cross-domain alias: `modulation-transfer-function`, `optical-OTF-magnitude`)
**Domain:** Photonics / Optics
**Definition:** MTF(f) = |OTF(f)| = |F{PSF}|(f) — contrast transfer as a function of spatial frequency f (cycles/mm).
**Atom or composite:** Composite (Fourier magnitude of PSF).
**Cost model:** One 2D FFT of PSF.
**Real wall?** Yes — diffraction-cutoff f_c = 1/(λN) is a hard band-limit.
**Cross-domain wiring:** Identical algebra as signal-processing-rf channel magnitude response; spatial-frequency analog of frequency response.
**Notes:** Goodman §6.

### psf (cross-domain alias: `point-spread`, `impulse-response-optical`)
**Domain:** Photonics / Optics
**Definition:** Image of a point source; diffraction-limited: |F{P(ξ,η)·exp(ikW)}|² where P is pupil function.
**Atom or composite:** Composite (Fourier transform squared of pupil function).
**Cost model:** One pupil-FFT.
**Real wall?** Yes — Airy pattern is the diffraction-limited floor.
**Cross-domain wiring:** Optical "impulse response" — convolution kernel like signal-processing-rf FIR; image-formation kernel.
**Notes:** Goodman §6; Born & Wolf §9.

### strehl-ratio (cross-domain alias: `peak-intensity-ratio`, `S`)
**Domain:** Photonics / Optics
**Definition:** S = PSF_peak(actual) / PSF_peak(ideal) ≈ exp(−(2π σ_W/λ)²) for small σ_W.
**Atom or composite:** Composite (figure of merit from W-RMS).
**Cost model:** Scalar from RMS wavefront.
**Real wall?** Yes — S=1 is the diffraction-limited ceiling.
**Cross-domain wiring:** Same exponential degradation form as Debye-Waller in physics-diffusion, decoherence factor in quantum-computing.
**Notes:** Maréchal S ≥ 0.8 ↔ RMS W ≤ λ/14.

---

## Wave Optics & Diffraction

### helmholtz-equation (cross-domain alias: `scalar-wave-eq`, `(∇²+k²)U=0`)
**Domain:** Photonics / Optics
**Definition:** (∇² + k²)U(r) = 0; time-independent monochromatic scalar wave equation, k = nω/c.
**Atom or composite:** Atom (defining equation of monochromatic optics).
**Cost model:** O(N) FFT-based BPM or O(N³) full 3D solves.
**Real wall?** Yes — full vector Maxwell required when polarization couples.
**Cross-domain wiring:** Same operator as electromagnetics-antennas (scalar approximation), physics-diffusion (steady-state Schrödinger after Wick rotation).
**Notes:** Born & Wolf §1.4.

### kirchhoff-fresnel-integral (cross-domain alias: `Fresnel-Kirchhoff`, `Huygens-Fresnel`)
**Domain:** Photonics / Optics
**Definition:** U(P) = (1/iλ) ∬ U(Q) (e^{ikr}/r) K(χ) dA, where K(χ) is the obliquity factor (1+cos χ)/2.
**Atom or composite:** Composite (Huygens secondary sources + Kirchhoff boundary).
**Cost model:** O(N²) brute; O(N log N) via angular spectrum.
**Real wall?** No — scalar approximation, fine for most diffraction.
**Cross-domain wiring:** Same Green's-function integral as electromagnetics-antennas physical-optics PO; analog of path-integral in physics-quantum.
**Notes:** Goodman §3.

### rayleigh-sommerfeld (cross-domain alias: `RS-diffraction`, `exact-scalar-diffraction`)
**Domain:** Photonics / Optics
**Definition:** U(P) = (1/iλ) ∬ U(Q)(e^{ikr}/r) cos(n,r) dA; resolves Kirchhoff's boundary inconsistency.
**Atom or composite:** Composite (variant of Kirchhoff using only U on aperture).
**Cost model:** O(N log N) with FFT-based convolution.
**Real wall?** No — accurate for λ-scale apertures.
**Cross-domain wiring:** Identical kernel structure to electromagnetics-antennas surface-currents formulation.
**Notes:** Goodman §3.5.

### angular-spectrum (cross-domain alias: `AS-propagation`, `plane-wave-decomposition`)
**Domain:** Photonics / Optics
**Definition:** U(x,y,z) = F⁻¹{F{U(x,y,0)} · exp(ikz√(1−(λf_x)²−(λf_y)²))}; propagate via FFT in spatial-frequency.
**Atom or composite:** Composite (FFT · transfer-function · iFFT).
**Cost model:** O(N log N).
**Real wall?** No — evanescent components (|f|>1/λ) decay exponentially.
**Cross-domain wiring:** Same k-space propagator as signal-processing-rf time-evolution; physics-diffusion spectral-method.
**Notes:** Goodman §3.10; the workhorse of digital holography.

### fresnel-diffraction (cross-domain alias: `near-field-diffraction`, `quadratic-phase`)
**Domain:** Photonics / Optics
**Definition:** U(x,y,z) = (e^{ikz}/iλz) ∬ U(ξ,η,0) exp(ik[(x−ξ)²+(y−η)²]/2z) dξdη.
**Atom or composite:** Composite (paraxial limit of Kirchhoff; quadratic-phase kernel).
**Cost model:** Convolution by quadratic-phase chirp; O(N log N) via FFT.
**Real wall?** No — paraxial; breaks for high NA.
**Cross-domain wiring:** Same chirp-kernel as signal-processing-rf chirp-z transform; ml-training Fresnel-zone-plate-as-mask.
**Notes:** Goodman §4.

### fraunhofer-diffraction (cross-domain alias: `far-field`, `FFT-of-aperture`)
**Domain:** Photonics / Optics
**Definition:** U(x,y) ∝ F{U(ξ,η,0)} evaluated at f_x = x/(λz), f_y = y/(λz); far-field = Fourier transform of aperture.
**Atom or composite:** Composite (limit z → ∞ of Fresnel).
**Cost model:** Single FFT.
**Real wall?** No.
**Cross-domain wiring:** Identical to electromagnetics-antennas far-field pattern = FT of aperture distribution; signal-processing-rf spectrum-via-FFT.
**Notes:** Goodman §4.3.

### bpm (cross-domain alias: `beam-propagation-method`, `split-step-fourier-optical`)
**Domain:** Photonics / Optics
**Definition:** Alternately apply free-space propagation (angular spectrum) and index-modulation phase: U_{n+1} = e^{ikΔn·Δz} · P_{Δz}[U_n].
**Atom or composite:** Composite (split-operator: kinetic + potential).
**Cost model:** O(N log N) per step.
**Real wall?** Yes — paraxial wide-angle correction needed for high-NA; vector BPM needed for strong polarization coupling.
**Cross-domain wiring:** Direct analog of physics-diffusion split-step Schrödinger; quantum-computing Trotter step.
**Notes:** Saleh & Teich §22.

### split-step-fourier-optical (cross-domain alias: `SSF`, `optical-NLSE-solver`)
**Domain:** Photonics / Optics
**Definition:** Solves NLSE: i∂A/∂z = −(β₂/2)∂²A/∂t² + γ|A|²A by alternating dispersion (k-space) and nonlinear phase (real-space).
**Atom or composite:** Composite (Trotter for dispersion + Kerr).
**Cost model:** O(N log N) per Δz; convergence O(Δz²).
**Real wall?** No — symmetric Strang splitting improves to O(Δz³).
**Cross-domain wiring:** Same algorithm as physics-diffusion split-operator GP equation; signal-processing-rf nonlinear-fiber simulation.
**Notes:** Agrawal §2.4.

### paraxial-wave-equation (cross-domain alias: `slowly-varying`, `2ik∂_z+∇⊥²`)
**Domain:** Photonics / Optics
**Definition:** 2ik ∂U/∂z + ∇⊥²U = 0 — drop ∂²U/∂z² assuming envelope varies slowly compared to e^{ikz}.
**Atom or composite:** Composite (slowly-varying-envelope approximation of Helmholtz).
**Cost model:** Parabolic PDE; O(N log N) via FFT.
**Real wall?** No — accuracy ~ (NA)²; fails at high NA.
**Cross-domain wiring:** Mathematically identical to Schrödinger equation (physics-quantum), heat equation with imaginary time (physics-diffusion).
**Notes:** Siegman §16.

### gaussian-beam (cross-domain alias: `TEM00`, `fundamental-mode`)
**Domain:** Photonics / Optics
**Definition:** U(r,z) = (w₀/w(z)) exp(−r²/w²(z)) exp(i[kz+kr²/(2R(z))−ψ(z)]) — exact solution of paraxial wave eq.
**Atom or composite:** Atom (fundamental laser mode).
**Cost model:** Analytic; closed-form propagation via complex-q.
**Real wall?** Yes — Rayleigh range z_R = πw₀²/λ limits how tightly focused a beam can stay collimated.
**Cross-domain wiring:** Same form as quantum-computing coherent state of harmonic oscillator; signal-processing-rf Gaussian pulse envelope.
**Notes:** Siegman §17.

### gaussian-beam-waist (cross-domain alias: `w0`, `beam-waist`, `focused-spot`)
**Domain:** Photonics / Optics
**Definition:** w₀ = minimum 1/e² radius of Gaussian beam; relates to NA via w₀ ≈ λ/(π NA).
**Atom or composite:** Atom (defining parameter of Gaussian).
**Cost model:** O(1).
**Real wall?** Yes — diffraction-limited minimum waist ≈ λ/2 (NA=1).
**Cross-domain wiring:** Equivalent to time-domain pulse width in signal-processing-rf (∆t·∆ω ≥ 1/2).
**Notes:** Siegman §17.

### rayleigh-range (cross-domain alias: `z_R`, `confocal-parameter/2`)
**Domain:** Photonics / Optics
**Definition:** z_R = πw₀²/λ; distance over which Gaussian beam area doubles. Confocal parameter b = 2z_R.
**Atom or composite:** Composite (w₀² · π/λ).
**Real wall?** Yes — fundamental focus-vs-collimation trade-off.
**Cross-domain wiring:** Optical analog of coherence length in signal-processing-rf; "Sommerfeld–Brillouin" zone in propagating waves.
**Notes:** Siegman §17.

### q-parameter (cross-domain alias: `complex-beam-parameter`, `1/q=1/R−iλ/(πw²)`)
**Domain:** Photonics / Optics
**Definition:** q(z) = z − z₀ + i z_R; encodes both R(z) and w(z); transforms by q′ = (Aq+B)/(Cq+D).
**Atom or composite:** Composite (complex encoding of Gaussian).
**Cost model:** O(N) ABCD-product on complex scalar.
**Real wall?** No.
**Cross-domain wiring:** Same Möbius transformation as control-numerical-opt bilinear, electromagnetics-antennas Smith chart.
**Notes:** Siegman §15.5.

### abcd-law-gaussian (cross-domain alias: `Kogelnik-law`, `q-transform`)
**Domain:** Photonics / Optics
**Definition:** q₂ = (A q₁ + B)/(C q₁ + D); Gaussian beam propagates through any ABCD system by Möbius transform on q.
**Atom or composite:** Composite (Möbius + ABCD).
**Real wall?** No.
**Cross-domain wiring:** Möbius group SL(2,ℂ) — same as quantum-computing single-qubit gates on Bloch sphere, complex-analysis conformal maps.
**Notes:** Kogelnik & Li 1966; foundation of laser-cavity design.

### hermite-gauss (cross-domain alias: `HG-modes`, `TEM_mn`, `rectangular-modes`)
**Domain:** Photonics / Optics
**Definition:** HG_mn(x,y,z) = H_m(√2 x/w)·H_n(√2 y/w)·exp(−(x²+y²)/w²)·phase, with Hermite polynomials H_m.
**Atom or composite:** Composite (basis of paraxial wave equation in Cartesian).
**Cost model:** O(N) coefficients per mode.
**Real wall?** No.
**Cross-domain wiring:** Same Hermite functions as harmonic-oscillator eigenstates (quantum-computing), Hermite features in ml-training.
**Notes:** Siegman §16; orthonormal complete basis.

### laguerre-gauss (cross-domain alias: `LG-modes`, `cylindrical-modes`, `OAM-modes`)
**Domain:** Photonics / Optics
**Definition:** LG_pl(r,φ,z) ∝ r^|l| L_p^|l|(2r²/w²) exp(−r²/w²) e^{ilφ} — carries orbital angular momentum lℏ per photon.
**Atom or composite:** Composite (cylindrical paraxial basis).
**Real wall?** No — but OAM mode-purity limited by alignment.
**Cross-domain wiring:** Same Laguerre polynomials in radial-3D quantum harmonic oscillator; OAM analog of quantum-computing spin angular momentum.
**Notes:** Allen et al. 1992; key for OAM communications.

### oam (cross-domain alias: `orbital-angular-momentum`, `vortex-beam`, `topological-charge`)
**Domain:** Photonics / Optics
**Definition:** Photon carries Lz = lℏ when wavefront has e^{ilφ} azimuthal phase; topological charge l ∈ ℤ.
**Atom or composite:** Atom (quantum number of light field).
**Cost model:** Mode-sort via spiral-phase plate or hologram; O(N) decoding.
**Real wall?** Yes — atmospheric turbulence scrambles OAM mode purity.
**Cross-domain wiring:** Topological-invariant analog in quantum-computing (anyonic charge), graphics-rendering-lod winding number.
**Notes:** Allen 1992; basis for OAM multiplexing in networking.

### bessel-beam (cross-domain alias: `non-diffracting-beam`, `J0-beam`)
**Domain:** Photonics / Optics
**Definition:** U(r,φ,z) = J_l(k_r r) e^{ilφ} e^{ik_z z}; exact solution of Helmholtz with non-spreading transverse profile.
**Atom or composite:** Atom (eigenfunction of Helmholtz in cylinder).
**Cost model:** Generated by axicon or ring-mask + lens.
**Real wall?** Yes — infinite-energy ideal; finite Bessel-Gauss has finite propagation range.
**Cross-domain wiring:** Same Bessel functions as electromagnetics-antennas cylindrical-waveguide modes; signal-processing-rf chirped pulses.
**Notes:** Durnin 1987.

### airy-beam (cross-domain alias: `self-accelerating`, `non-diffracting-parabolic`)
**Domain:** Photonics / Optics
**Definition:** Airy-function beam U ∝ Ai(s) follows parabolic trajectory while resisting diffraction over finite range.
**Atom or composite:** Composite (cubic-phase mask + FT).
**Real wall?** Yes — finite energy limits self-healing range.
**Cross-domain wiring:** Same Airy function appears in physics-quantum (linear-potential eigenstates), causality in retrieval-search (rare).
**Notes:** Siviloglou 2007.

### vortex-beam (cross-domain alias: `phase-singularity`, `donut-mode`)
**Domain:** Photonics / Optics
**Definition:** Beam with on-axis phase singularity (zero amplitude); azimuthal phase e^{ilφ} winds 2πl around the axis.
**Atom or composite:** Composite (spiral phase mask on Gaussian).
**Real wall?** No.
**Cross-domain wiring:** Topological defect — same as vortex in superfluid/Bose-Einstein (physics-quantum), dislocation in graphics-rendering-lod.
**Notes:** Berry & Nye 1974.

### structured-light (cross-domain alias: `tailored-beam`, `engineered-modes`)
**Domain:** Photonics / Optics
**Definition:** General term: beams with spatially shaped amplitude, phase, and/or polarization (vector beams, OAM superpositions, flat-top).
**Atom or composite:** Composite (SLM/DMD/metasurface modulation of input mode).
**Cost model:** SLM bandwidth × refresh rate.
**Real wall?** Yes — space-bandwidth product bounded by SLM pixel count.
**Cross-domain wiring:** Same as ml-training learned spatial filters; signal-processing-rf arbitrary-waveform generation.
**Notes:** Forbes 2021 review.

---

## Polarization

### jones-vector (cross-domain alias: `polarization-state`, `2-spinor`)
**Domain:** Photonics / Optics
**Definition:** E = [E_x; E_y]^T ∈ ℂ²; describes fully polarized light. Normalized: |E_x|² + |E_y|² = 1.
**Atom or composite:** Atom (state vector of polarization).
**Cost model:** O(1); 2-complex scalars per ray.
**Real wall?** Yes — only describes fully polarized light; partially polarized needs Stokes.
**Cross-domain wiring:** Mathematically identical to quantum-computing qubit-state; spinor analog in physics-quantum (electron spin).
**Notes:** Jones 1941; Hecht §8.

### jones-matrix (cross-domain alias: `2x2-pol-matrix`, `polarization-operator`)
**Domain:** Photonics / Optics
**Definition:** 2×2 complex matrix mapping input to output Jones vector: E_out = J·E_in.
**Atom or composite:** Composite (product of element matrices).
**Cost model:** O(N) 2×2 matrix products for N elements.
**Real wall?** No — only valid for coherent fully-polarized light.
**Cross-domain wiring:** Same as quantum-computing single-qubit gate matrix; linear-algebra-matrix 2×2 ops.
**Notes:** Hecht §8.13.

### linear-polarizer-jones (cross-domain alias: `polarizer-matrix`, `extinction-axis`)
**Domain:** Photonics / Optics
**Definition:** J = [cos²θ, sin θ cos θ; sin θ cos θ, sin²θ] — projects onto axis at angle θ.
**Atom or composite:** Atom (projection operator in polarization space).
**Cost model:** O(1).
**Real wall?** Yes — ideal extinction never achieved; real polarizers have finite extinction ratio.
**Cross-domain wiring:** Projection operator (rank-1) — same as quantum-computing measurement-projector.
**Notes:** Malus's law: I = I₀ cos²θ.

### waveplate-jones (cross-domain alias: `retarder`, `phase-plate`)
**Domain:** Photonics / Optics
**Definition:** J = R(−θ)·diag(e^{iδ/2}, e^{−iδ/2})·R(θ); introduces retardance δ between fast/slow axes.
**Atom or composite:** Composite (rotation·diagonal·rotation).
**Cost model:** O(1).
**Real wall?** Yes — chromatic: δ(λ) varies, true achromatic plates need compound design.
**Cross-domain wiring:** Same SU(2) rotation as quantum-computing R_x/R_y/R_z; analog of group-delay element in signal-processing-rf.
**Notes:** δ = π/2 → QWP, δ = π → HWP.

### polarization-rotator (cross-domain alias: `optical-rotator`, `Faraday-rotator`)
**Domain:** Photonics / Optics
**Definition:** J = [cos α, −sin α; sin α, cos α] — rotates polarization by angle α (e.g., via Faraday or sugar).
**Atom or composite:** Atom.
**Cost model:** O(1).
**Real wall?** Yes — Faraday rotation is non-reciprocal, exploited in isolators; bound by Verdet constant.
**Cross-domain wiring:** SO(2) rotation — same form as 2D rotation in computational-geometry, graphics-rendering-lod.
**Notes:** Saleh & Teich §6.

### stokes-vector (cross-domain alias: `S0-S1-S2-S3`, `Stokes-params`)
**Domain:** Photonics / Optics
**Definition:** S = (S₀,S₁,S₂,S₃) with S₀=I, S₁=I_H−I_V, S₂=I_+45−I_−45, S₃=I_R−I_L; works for partial polarization.
**Atom or composite:** Composite (intensity moments).
**Cost model:** O(1); measured by 4 intensity values.
**Real wall?** Yes — degree of polarization p = √(S₁²+S₂²+S₃²)/S₀ ≤ 1.
**Cross-domain wiring:** Same as quantum-computing Bloch vector for mixed states; statistics-probability moment-vector.
**Notes:** Born & Wolf §1.4; vector lives on/inside Poincaré sphere.

### poincare-sphere (cross-domain alias: `polarization-sphere`, `pol-Bloch`)
**Domain:** Photonics / Optics
**Definition:** Unit sphere in (S₁,S₂,S₃) space; pure states on surface, mixed inside. Antipodal points are orthogonal polarizations.
**Atom or composite:** Atom (geometric representation).
**Cost model:** O(1) geometric ops.
**Real wall?** No.
**Cross-domain wiring:** Identical to quantum-computing Bloch sphere; SU(2)/U(1) topology.
**Notes:** Hecht §8.13.5.

### mueller-matrix (cross-domain alias: `4x4-pol-matrix`, `Stokes-operator`)
**Domain:** Photonics / Optics
**Definition:** 4×4 real matrix M: S_out = M·S_in; handles polarized + unpolarized + depolarization.
**Atom or composite:** Composite (real matrix from coherency average).
**Cost model:** O(N) 4×4 products.
**Real wall?** Yes — physical M must satisfy positivity (M maps Stokes-physical to Stokes-physical).
**Cross-domain wiring:** Analog of quantum-computing quantum channel super-operator; statistics-probability transition matrix.
**Notes:** Chipman 2005.

### partial-polarization (cross-domain alias: `degree-of-polarization`, `coherency-matrix`)
**Domain:** Photonics / Optics
**Definition:** Coherency matrix J_ij = ⟨E_i E_j*⟩; p = √(1−4 det J / tr²J).
**Atom or composite:** Composite (statistical average of Jones outer product).
**Cost model:** O(1) on 2×2 Hermitian matrix.
**Real wall?** Yes — natural light has p ≈ 0; lasers p ≈ 1.
**Cross-domain wiring:** Density matrix in quantum-computing; covariance matrix in statistics-probability.
**Notes:** Born & Wolf §10.

### polarization-aberration (cross-domain alias: `pol-aberration`, `pol-MTF`)
**Domain:** Photonics / Optics
**Definition:** Pupil-dependent polarization changes (diattenuation, retardance) introduced by coatings and oblique reflections.
**Atom or composite:** Composite (Jones-pupil function).
**Cost model:** Per-pixel Jones matrix at pupil; O(N²).
**Real wall?** Yes — limits contrast in high-NA imaging (lithography, microscopy).
**Cross-domain wiring:** Pol analog of wavefront-aberration; coupled-mode dispersion in signal-processing-rf.
**Notes:** Chipman 2018.

### depolarization (cross-domain alias: `pol-randomization`, `Mueller-depol`)
**Domain:** Photonics / Optics
**Definition:** Reduction of degree of polarization due to ensemble averaging (scattering, multimode integration).
**Atom or composite:** Composite (Mueller-matrix component that shrinks Poincaré radius).
**Cost model:** Scales with sampling diversity.
**Real wall?** Yes — irreversible in classical optics; quantum-state-erasure analog.
**Cross-domain wiring:** Same as quantum-computing decoherence; statistics-probability ensemble-averaging.
**Notes:** Lu-Chipman decomposition.

---

## Coherence

### temporal-coherence (cross-domain alias: `time-coherence`, `Δν-coherence`)
**Domain:** Photonics / Optics
**Definition:** Ability of light at times t and t+τ to interfere; characterized by g(1)(τ) = ⟨E*(t)E(t+τ)⟩/⟨|E|²⟩.
**Atom or composite:** Composite (autocorrelation of field).
**Cost model:** O(N log N) for autocorrelation via Wiener-Khinchin.
**Real wall?** Yes — bandwidth Δν caps τ_c ≈ 1/Δν.
**Cross-domain wiring:** Same autocorrelation in signal-processing-rf coherence-time; statistics-probability stationary-process correlation.
**Notes:** Mandel & Wolf §4.

### spatial-coherence (cross-domain alias: `transverse-coherence`, `coherence-area`)
**Domain:** Photonics / Optics
**Definition:** Field correlation across transverse positions: Γ(r₁,r₂) = ⟨E*(r₁)E(r₂)⟩; coherence area A_c.
**Atom or composite:** Composite (transverse cross-correlation).
**Cost model:** 2D FFT-based correlation.
**Real wall?** Yes — set by source angular extent: A_c ≈ (λ/θ_s)².
**Cross-domain wiring:** Analog of channel-coherence in signal-processing-rf MIMO; ml-training kernel-similarity.
**Notes:** Mandel & Wolf §4.3.

### coherence-length (cross-domain alias: `Lc`, `cτ_c`)
**Domain:** Photonics / Optics
**Definition:** L_c = c·τ_c ≈ c/Δν ≈ λ²/Δλ; path-length difference over which fringes remain visible.
**Atom or composite:** Composite.
**Real wall?** Yes — bound by spectral linewidth.
**Cross-domain wiring:** Same scaling λ²/Δλ as time-bandwidth product in signal-processing-rf.
**Notes:** Hecht §12.

### coherence-time (cross-domain alias: `τ_c`, `1/Δν`)
**Domain:** Photonics / Optics
**Definition:** τ_c = 1/Δν (Fourier-limited); time over which the field correlates with itself.
**Atom or composite:** Composite.
**Real wall?** Yes — laser linewidth Δν determines τ_c.
**Cross-domain wiring:** Same as quantum-computing T₂ (coherence time of qubit).
**Notes:** Mandel & Wolf §3.

### mutual-coherence-function (cross-domain alias: `Γ12(τ)`, `cross-coherence`)
**Domain:** Photonics / Optics
**Definition:** Γ₁₂(τ) = ⟨E*(r₁,t)E(r₂,t+τ)⟩; full second-order coherence function.
**Atom or composite:** Composite (joint spacetime correlation).
**Cost model:** Measured by Young's two-pinhole + delay.
**Real wall?** No — fundamental object.
**Cross-domain wiring:** Same as signal-processing-rf cross-correlation function; statistics-probability cross-covariance.
**Notes:** Wolf 1955; basis for coherence theory.

### van-cittert-zernike (cross-domain alias: `vC-Z-theorem`, `FT-of-source-→coherence`)
**Domain:** Photonics / Optics
**Definition:** Far-field mutual coherence ∝ Fourier transform of source intensity distribution.
**Atom or composite:** Composite theorem (FT relation).
**Cost model:** O(N log N) FFT.
**Real wall?** No — exact in far-field paraxial.
**Cross-domain wiring:** Identical theorem as Wiener-Khinchin in signal-processing-rf; underlies aperture-synthesis radio astronomy.
**Notes:** Goodman §5; basis of VLBI imaging.

### wiener-khinchin-optical (cross-domain alias: `W-K-optical`, `spectral-density=FT(autocorr)`)
**Domain:** Photonics / Optics
**Definition:** Power spectral density S(ν) = F{g(1)(τ)} — spectrum is FT of field autocorrelation.
**Atom or composite:** Composite (FT pair).
**Cost model:** One FFT.
**Real wall?** No.
**Cross-domain wiring:** Same theorem as signal-processing-rf Wiener-Khinchin; statistics-probability spectral density of stationary process.
**Notes:** Mandel & Wolf §2.

### hbt-correlation (cross-domain alias: `Hanbury-Brown-Twiss`, `intensity-correlation`)
**Domain:** Photonics / Optics
**Definition:** g(2)(τ) = ⟨I(t)I(t+τ)⟩/⟨I⟩²; measures intensity correlations and photon statistics.
**Atom or composite:** Composite (fourth-order coherence).
**Cost model:** Coincidence detection; rate ∝ ⟨I⟩².
**Real wall?** Yes — distinguishes classical (g²(0)≥1) vs nonclassical (g²(0)<1) light.
**Cross-domain wiring:** Same algebra as variance/covariance estimators in statistics-probability.
**Notes:** Hanbury Brown & Twiss 1956.

### g1-correlation (cross-domain alias: `first-order-coherence`, `field-correlation`)
**Domain:** Photonics / Optics
**Definition:** g(1)(τ) = ⟨E*(t)E(t+τ)⟩/⟨|E|²⟩ — normalized first-order coherence; sets fringe visibility.
**Atom or composite:** Composite.
**Cost model:** Measured by Michelson interferometer.
**Real wall?** Yes — |g(1)| ≤ 1.
**Cross-domain wiring:** Same as signal-processing-rf normalized autocorrelation.
**Notes:** Loudon §3.

### g2-correlation (cross-domain alias: `second-order-coherence`, `intensity-correlation`)
**Domain:** Photonics / Optics
**Definition:** g(2)(τ) = ⟨I(t)I(t+τ)⟩/⟨I⟩² — distinguishes thermal (g²(0)=2), coherent (g²(0)=1), antibunched (g²(0)<1).
**Atom or composite:** Composite (normalized intensity correlation).
**Real wall?** Yes — g²(0)≥1 classical bound; violating it proves quantum source.
**Cross-domain wiring:** Same as 4-point correlator in statistics-probability; quantum-computing photon-statistics witness.
**Notes:** Glauber 1963.

### photon-bunching (cross-domain alias: `thermal-bunching`, `g²(0)>1`)
**Domain:** Photonics / Optics
**Definition:** g²(0) > 1 — photons arrive in clusters; characteristic of thermal/chaotic light.
**Atom or composite:** Composite (statistical feature of intensity).
**Real wall?** Yes — classical light cannot achieve g²(0) < 1.
**Cross-domain wiring:** Same as super-Poissonian counting in statistics-probability.
**Notes:** Loudon §6.

### photon-antibunching (cross-domain alias: `g²(0)<1`, `sub-poissonian`)
**Domain:** Photonics / Optics
**Definition:** g²(0) < 1 — photons arrive more regularly than Poisson; signature of single-photon source.
**Atom or composite:** Composite (nonclassical photon statistics).
**Real wall?** Yes — fundamentally requires quantization of light.
**Cross-domain wiring:** Quantum signature; analog of single-particle fermionic statistics (physics-quantum).
**Notes:** Kimble et al. 1977.

---

## Interferometry

### michelson-interferometer (cross-domain alias: `Michelson`, `two-arm-interferometer`)
**Domain:** Photonics / Optics
**Definition:** Beam-splitter sends light along two arms with mirrors; recombined beams interfere with phase 2k(L₁−L₂).
**Atom or composite:** Composite (BS + mirrors + recombination).
**Cost model:** O(1) optical path; scan one arm to record interferogram.
**Real wall?** Yes — vibration & thermal drift limit sensitivity.
**Cross-domain wiring:** Same delay-and-add architecture as signal-processing-rf correlator; quantum-computing interferometric phase estimation.
**Notes:** Michelson 1881; LIGO scale.

### mach-zehnder (cross-domain alias: `MZ-interferometer`, `dual-path`)
**Domain:** Photonics / Optics
**Definition:** Two beam-splitters with two paths between them; outputs depend on phase difference Δφ between arms.
**Atom or composite:** Composite (BS-arm-BS).
**Cost model:** O(1).
**Real wall?** Yes — path-length stability sets minimum detectable Δφ.
**Cross-domain wiring:** Direct optical analog of quantum-computing two-mode interference gate; networking duplexed channel.
**Notes:** Zehnder 1891; backbone of integrated-photonic modulators.

### sagnac-interferometer (cross-domain alias: `ring-interferometer`, `rotation-sensor`)
**Domain:** Photonics / Optics
**Definition:** Counter-propagating beams in a closed loop; rotation Ω induces phase shift Δφ = 8πAΩ/(λc).
**Atom or composite:** Composite (common-path topology).
**Real wall?** Yes — quantum shot-noise limits fiber-gyro sensitivity (~10⁻⁹ rad/s).
**Cross-domain wiring:** Same rotation-sensing principle as quantum-computing matter-wave gyros; networking common-path noise rejection.
**Notes:** Sagnac 1913; basis of fiber-optic gyros.

### fabry-perot (cross-domain alias: `FP-cavity`, `etalon`, `finesse-cavity`)
**Domain:** Photonics / Optics
**Definition:** Two parallel partially-reflecting mirrors with high reflectivity R; transmission T(ν) is Airy function with finesse F = π√R/(1−R).
**Atom or composite:** Composite (multi-beam interference; geometric series).
**Cost model:** O(1) closed-form; numerical for arbitrary spectra.
**Real wall?** Yes — mirror loss caps finesse F < ~10⁶ practical.
**Cross-domain wiring:** Resonator analog of signal-processing-rf IIR bandpass filter; quantum-computing optical cavity (CQED).
**Notes:** Born & Wolf §7.6.

### twyman-green (cross-domain alias: `TG-interferometer`, `lens-test`)
**Domain:** Photonics / Optics
**Definition:** Michelson variant with collimated input; one arm contains the test optic, fringes encode its wavefront aberration.
**Atom or composite:** Composite (Michelson + reference flat + test optic).
**Cost model:** One CCD frame; aberration extracted via fringe analysis.
**Real wall?** Yes — requires λ-stable reference flat.
**Cross-domain wiring:** Same direct-measurement paradigm as signal-processing-rf vector-network analyzer.
**Notes:** Malacara §2.

### common-path-interferometer (cross-domain alias: `CP-IF`, `point-diffraction`)
**Domain:** Photonics / Optics
**Definition:** Both interfering beams traverse the same path; intrinsically immune to common-mode vibration.
**Atom or composite:** Composite (shared path + small lateral split).
**Cost model:** O(1).
**Real wall?** No — measurement floor set by detector noise.
**Cross-domain wiring:** Common-mode rejection analog of signal-processing-rf differential amplifier.
**Notes:** Smartt 1975 point-diffraction.

### white-light-interferometer (cross-domain alias: `WLI`, `coherence-scanning`)
**Domain:** Photonics / Optics
**Definition:** Broadband source with short L_c gives narrow envelope in interferogram; envelope peak locates zero-OPD with sub-nm resolution.
**Atom or composite:** Composite (broadband source + scan).
**Cost model:** Scan + envelope-detection per pixel; O(N) per height.
**Real wall?** Yes — axial resolution ∝ L_c.
**Cross-domain wiring:** Same envelope-detection principle as signal-processing-rf time-of-flight ranging.
**Notes:** de Groot 2015.

### ftir (cross-domain alias: `Fourier-transform-IR`, `Michelson-spectrometer`)
**Domain:** Photonics / Optics
**Definition:** Michelson with moving mirror records interferogram I(τ); spectrum S(ν) = F{I(τ)}.
**Atom or composite:** Composite (interferogram + FFT).
**Cost model:** One FFT per scan; multiplex (Fellgett) advantage.
**Real wall?** Yes — finite scan length limits spectral resolution.
**Cross-domain wiring:** Direct application of Wiener-Khinchin; same as time-domain spectroscopy in signal-processing-rf.
**Notes:** Griffiths & de Haseth 2007.

### heterodyne-detection (cross-domain alias: `optical-heterodyne`, `beat-detection`)
**Domain:** Photonics / Optics
**Definition:** Mix signal with local-oscillator at ν_LO; photodetector outputs beat at |ν_s − ν_LO|; carries amplitude and phase of signal.
**Atom or composite:** Composite (mix + low-pass).
**Cost model:** Photodiode bandwidth-limited.
**Real wall?** Yes — shot-noise-limited sensitivity = LO-amplified single-photon detection.
**Cross-domain wiring:** Identical to signal-processing-rf superheterodyne receiver; quantum-computing homodyne quadrature measurement.
**Notes:** Yariv §11.

### homodyne-detection (cross-domain alias: `optical-homodyne`, `quadrature-measure`)
**Domain:** Photonics / Optics
**Definition:** Like heterodyne but ν_LO = ν_s; output is DC carrying phase-sensitive quadrature; balanced detection cancels LO noise.
**Atom or composite:** Composite (BS + dual PD + subtract).
**Cost model:** Balanced detector bandwidth.
**Real wall?** Yes — quantum-limited at vacuum noise (3 dB squeezing improves).
**Cross-domain wiring:** Same as signal-processing-rf I/Q demodulation; quantum-computing quadrature tomography.
**Notes:** Yuen & Chan 1983.

### lock-in-detection (cross-domain alias: `phase-sensitive-detection`, `narrowband-rec`)
**Domain:** Photonics / Optics
**Definition:** Multiply detected signal by reference cos(ω_r t + φ); low-pass filter extracts component at ω_r.
**Atom or composite:** Composite (multiply + LPF).
**Cost model:** O(N) per sample.
**Real wall?** Yes — recovers signal from 100 dB of broadband noise.
**Cross-domain wiring:** Direct lift from signal-processing-rf synchronous detection; ml-training Fourier-feature extraction.
**Notes:** Stanford SR830 datasheet.

### oct-td (cross-domain alias: `time-domain-OCT`, `scan-OCT`)
**Domain:** Photonics / Optics
**Definition:** Low-coherence interferometer scans reference arm; axial profile = depth scan via interferogram envelope.
**Atom or composite:** Composite (WLI + lateral scan).
**Cost model:** Scan time per A-line; slow.
**Real wall?** Yes — speed limited by mechanical scan.
**Cross-domain wiring:** Analog of pulse-echo ultrasound (signal-processing-rf).
**Notes:** Huang 1991.

### oct-sd (cross-domain alias: `spectral-domain-OCT`, `Fourier-domain-OCT`)
**Domain:** Photonics / Optics
**Definition:** Stationary reference + broadband source; spectrometer records I(k); IFT yields depth profile in single shot.
**Atom or composite:** Composite (broadband + spectrometer + IFFT).
**Cost model:** One FFT per A-line; ~100× faster than TD.
**Real wall?** Yes — sensitivity roll-off vs depth from spectrometer finite resolution.
**Cross-domain wiring:** Same FFT-spectroscopy paradigm as FTIR; range-FFT in signal-processing-rf FMCW radar.
**Notes:** Fercher 1995.

### oct-ss (cross-domain alias: `swept-source-OCT`, `SS-OCT`)
**Domain:** Photonics / Optics
**Definition:** Rapidly tuned laser (sweep) + single detector; FFT of time signal = depth profile.
**Atom or composite:** Composite (FMCW analog in optics).
**Cost model:** Sweep rate sets A-line rate (MHz achievable).
**Real wall?** Yes — sweep linearity and laser coherence cap depth range.
**Cross-domain wiring:** Direct optical analog of signal-processing-rf FMCW radar; range = beat-frequency / chirp-rate.
**Notes:** Choma 2003.

### ligo-interferometer (cross-domain alias: `gravitational-wave-IF`, `dual-recycled-Michelson`)
**Domain:** Photonics / Optics
**Definition:** Power- and signal-recycled Michelson with Fabry-Perot arms; measures strain h = ΔL/L ~ 10⁻²¹.
**Atom or composite:** Composite (Michelson + FP arms + recycling cavities + squeezed light).
**Cost model:** km-scale; cryogenic test masses, ultra-high vacuum.
**Real wall?** Yes — quantum shot noise + radiation pressure; addressed by frequency-dependent squeezing.
**Cross-domain wiring:** Combines every coherence/quantum primitive; same SNR-vs-bandwidth trade as signal-processing-rf matched filter.
**Notes:** LIGO Collab. 2016.

---

## Holography

### inline-hologram (cross-domain alias: `Gabor-hologram`, `axial-hologram`)
**Domain:** Photonics / Optics
**Definition:** Reference wave and object wave share the same axis; interference fringes recorded on detector.
**Atom or composite:** Composite (transparent object + coherent illum + sensor).
**Cost model:** Reconstruction by digital back-propagation: O(N log N).
**Real wall?** Yes — twin-image ambiguity along axis.
**Cross-domain wiring:** Same single-arm phase recovery as iterative-phase-retrieval (ml-training, retrieval-search).
**Notes:** Gabor 1948.

### off-axis-hologram (cross-domain alias: `Leith-Upatnieks`, `tilted-reference`)
**Domain:** Photonics / Optics
**Definition:** Reference at angle θ_R to object; fringe carrier separates virtual and real images in Fourier space.
**Atom or composite:** Composite (carrier-modulated hologram).
**Cost model:** Single FFT to isolate sideband.
**Real wall?** Yes — sensor pixel pitch must resolve carrier fringes.
**Cross-domain wiring:** Same carrier-isolation as signal-processing-rf SSB modulation.
**Notes:** Leith & Upatnieks 1962.

### reflection-hologram (cross-domain alias: `Denisyuk-hologram`, `volume-reflection`)
**Domain:** Photonics / Optics
**Definition:** Object and reference beams from opposite sides; Bragg-selective volume hologram, viewable in white light.
**Atom or composite:** Composite (volume Bragg grating recorded by interference).
**Real wall?** Yes — recording emulsion shrinks; Bragg λ may shift.
**Cross-domain wiring:** Same Bragg selectivity as electromagnetics-antennas Bragg reflector; X-ray crystallography.
**Notes:** Denisyuk 1962.

### transmission-hologram (cross-domain alias: `Leith-style`, `thin-hologram`)
**Domain:** Photonics / Optics
**Definition:** Object & reference on same side; viewed in transmission with reference-like beam to reconstruct virtual image.
**Atom or composite:** Composite (thin or thick recording medium).
**Cost model:** Recording exposure + chemical/photopolymer processing.
**Real wall?** Yes — needs coherent reference for reconstruction.
**Cross-domain wiring:** Same as transmission Bragg grating; networking spatial-mode demultiplexing.
**Notes:** Hariharan 2002.

### volume-hologram (cross-domain alias: `thick-hologram`, `Bragg-volume-grating`)
**Domain:** Photonics / Optics
**Definition:** Hologram thickness >> λ; angle-multiplexing and wavelength-multiplexing via Bragg selectivity.
**Atom or composite:** Composite (Kogelnik coupled-wave theory governs diffraction efficiency η = sin²(πΔn d/(λ cos θ))).
**Cost model:** Many holograms stacked in same volume.
**Real wall?** Yes — material Δn limits efficiency × multiplexing product.
**Cross-domain wiring:** Same Bragg-multiplexing as networking WDM; storage density analog to information-theory-coding.
**Notes:** Kogelnik 1969.

### cgh (cross-domain alias: `computer-generated-hologram`, `digital-mask`)
**Domain:** Photonics / Optics
**Definition:** Numerically computed phase/amplitude pattern that, when illuminated, reconstructs a desired wavefront.
**Atom or composite:** Composite (iterative phase retrieval: Gerchberg-Saxton, Fienup).
**Cost model:** O(N² log N) iterative FFTs.
**Real wall?** Yes — discretization speckle; SLM bit-depth and pixel pitch.
**Cross-domain wiring:** Same algorithm class as ml-training inverse problems; signal-processing-rf phase-retrieval.
**Notes:** Brown & Lohmann 1966.

### digital-holography (cross-domain alias: `DH`, `numerical-reconstruction`)
**Domain:** Photonics / Optics
**Definition:** Record hologram on CCD/CMOS; reconstruct complex field numerically via back-propagation.
**Atom or composite:** Composite (sensor capture + angular-spectrum back-prop).
**Cost model:** One FFT per depth.
**Real wall?** Yes — sensor SBP (space-bandwidth product) caps resolution and FOV.
**Cross-domain wiring:** Same compressed-sensing flavor as ml-training; signal-processing-rf SAR imaging.
**Notes:** Schnars & Jueptner 2002.

### holographic-storage (cross-domain alias: `HDS`, `volume-storage`)
**Domain:** Photonics / Optics
**Definition:** Data pages stored as angle-multiplexed volume holograms; read via Bragg-selective reference.
**Atom or composite:** Composite (volume hologram + page-mode storage).
**Cost model:** Tb/cm³ theoretical density.
**Real wall?** Yes — material noise & shrinkage; commercial uptake limited.
**Cross-domain wiring:** Same parallel readout as ml-training optical processors; database-streaming-sketching content-addressable lookup.
**Notes:** Coufal et al. 2000.

### hoe (cross-domain alias: `holographic-optical-element`, `HOE`)
**Domain:** Photonics / Optics
**Definition:** Hologram designed to perform an optical function (lens, beam-splitter, combiner) — recorded interference pattern of desired wavefronts.
**Atom or composite:** Composite (specialized hologram).
**Real wall?** Yes — chromatic and angular sensitivity of Bragg structure.
**Cross-domain wiring:** Functional analog of metasurfaces; signal-processing-rf matched-filter.
**Notes:** Close 1973; common in HUDs and AR.

---

## Diffraction Gratings

### ruled-grating (cross-domain alias: `mechanical-grating`, `surface-grating`)
**Domain:** Photonics / Optics
**Definition:** Periodic grooves of spacing d; diffraction orders at sin θ_m = sin θ_i + mλ/d.
**Atom or composite:** Atom (1D periodic structure).
**Cost model:** Diamond-tool ruling slow; replicas cast.
**Real wall?** Yes — ghost orders from periodicity errors (Rowland ghosts).
**Cross-domain wiring:** Same as electromagnetics-antennas linear array; signal-processing-rf FIR comb.
**Notes:** Loewen & Popov 1997.

### blazed-grating (cross-domain alias: `echelette`, `littrow-blaze`)
**Domain:** Photonics / Optics
**Definition:** Sawtooth grooves tilted at blaze angle θ_B to direct most energy into one chosen order.
**Atom or composite:** Composite (ruled grating + groove shape).
**Cost model:** Efficiency >80% in blaze order.
**Real wall?** Yes — blaze condition holds only over a narrow λ band.
**Cross-domain wiring:** Same as electromagnetics-antennas phased-array tilted-beam steering.
**Notes:** Wood 1910.

### transmission-grating (cross-domain alias: `T-grating`, `binary-grating`)
**Domain:** Photonics / Optics
**Definition:** Phase or amplitude grating used in transmission; same equation as ruled grating.
**Atom or composite:** Composite.
**Cost model:** Etched in dielectric; lithography-fabricated.
**Real wall?** Yes — diffraction efficiency depends on duty cycle and depth.
**Cross-domain wiring:** Same kind of phase-mask as cgh.
**Notes:** Petit 1980.

### reflection-grating (cross-domain alias: `R-grating`, `Echelle-large-d`)
**Domain:** Photonics / Optics
**Definition:** Grating used in reflection; widely used in monochromators and spectrometers.
**Atom or composite:** Composite.
**Real wall?** Yes — coating reflectivity dictates throughput.
**Cross-domain wiring:** Reflective phased array (electromagnetics-antennas).
**Notes:** Loewen §1.

### holographic-grating (cross-domain alias: `interference-grating`, `low-stray-light`)
**Domain:** Photonics / Optics
**Definition:** Grating recorded by interference of two laser beams in photoresist; sinusoidal profile, low stray light.
**Atom or composite:** Composite.
**Real wall?** Yes — lower efficiency than blazed unless ion-etched into triangular profile.
**Cross-domain wiring:** Recording analog of metasurface lithography.
**Notes:** Rudolph & Schmahl 1967.

### echelle-grating (cross-domain alias: `coarse-blaze`, `high-order`)
**Domain:** Photonics / Optics
**Definition:** Coarsely ruled grating used at high diffraction order m (10–100); high dispersion in a compact size.
**Atom or composite:** Composite (large-d + cross-disperser).
**Real wall?** Yes — order overlap requires cross-disperser.
**Cross-domain wiring:** Same as electromagnetics-antennas high-order Floquet modes.
**Notes:** Harrison 1949; used in échelle spectrographs.

### volume-bragg-grating (cross-domain alias: `VBG`, `PTR-glass-grating`)
**Domain:** Photonics / Optics
**Definition:** Refractive-index modulation within a transparent volume; high-efficiency Bragg-selective filtering.
**Atom or composite:** Composite (Kogelnik regime).
**Real wall?** Yes — angle/wavelength acceptance ∝ 1/thickness.
**Cross-domain wiring:** Same as fiber Bragg grating; X-ray Bragg reflection in physics-diffusion.
**Notes:** Efimov 2004.

### fiber-bragg-grating (cross-domain alias: `FBG`, `in-fiber-mirror`)
**Domain:** Photonics / Optics
**Definition:** UV-written periodic index modulation in fiber core; reflects λ_B = 2 n_eff Λ; narrow notch reflector.
**Atom or composite:** Composite (UV exposure + phase mask).
**Real wall?** Yes — temperature & strain sensitivity (used as sensor).
**Cross-domain wiring:** Same as networking optical add-drop multiplex; signal-processing-rf bandstop filter.
**Notes:** Hill & Meltz 1997.

### photonic-crystal-grating (cross-domain alias: `PhC-grating`, `2D-3D-periodic`)
**Domain:** Photonics / Optics
**Definition:** Periodic dielectric structure with 1D/2D/3D periodicity producing photonic bandgaps; engineered diffraction.
**Atom or composite:** Composite (periodic-medium Bloch modes).
**Real wall?** Yes — bandgap formation requires Δn ≳ 2:1 contrast for full 3D gap.
**Cross-domain wiring:** Same as quantum-computing periodic potential bandgap; electromagnetics-antennas EBG surfaces.
**Notes:** Joannopoulos 2008.

---

## Fourier Optics

### 4f-system (cross-domain alias: `4f-relay`, `Fourier-correlator`)
**Domain:** Photonics / Optics
**Definition:** Two lenses spaced by 2f with Fourier plane at midpoint; performs FT (lens 1) and inverse FT (lens 2).
**Atom or composite:** Composite (two FT lenses, three planes).
**Cost model:** Hardware FFT — speed-of-light Fourier transform.
**Real wall?** Yes — finite aperture sets max spatial frequency.
**Cross-domain wiring:** Analog of ml-training learnable FT layer; signal-processing-rf optical FFT.
**Notes:** Goodman §6.

### spatial-filter (cross-domain alias: `Fourier-filter`, `pinhole-filter`)
**Domain:** Photonics / Optics
**Definition:** Mask placed at Fourier plane to attenuate selected spatial frequencies; modifies image spectrum.
**Atom or composite:** Composite (mask in Fourier plane).
**Cost model:** Designed once; static.
**Real wall?** No.
**Cross-domain wiring:** Direct optical analog of signal-processing-rf FIR filter; ml-training convolution kernel.
**Notes:** Goodman §8.

### low-pass-filter-optical (cross-domain alias: `optical-LPF`, `defocus-blur`)
**Domain:** Photonics / Optics
**Definition:** Small aperture at Fourier plane passes only low spatial frequencies; smooths image.
**Atom or composite:** Composite (small-radius pinhole).
**Cost model:** O(1).
**Real wall?** No.
**Cross-domain wiring:** Same as signal-processing-rf low-pass; ml-training Gaussian blur.
**Notes:** Goodman §8.

### high-pass-filter-optical (cross-domain alias: `optical-HPF`, `edge-enhancement`)
**Domain:** Photonics / Optics
**Definition:** Opaque center at Fourier plane blocks DC and low frequencies; enhances edges.
**Atom or composite:** Composite (annular stop or dark-spot).
**Cost model:** O(1).
**Real wall?** No.
**Cross-domain wiring:** Same as signal-processing-rf HPF; ml-training Laplacian filter.
**Notes:** Goodman §8.

### phase-contrast-zernike (cross-domain alias: `Zernike-PC`, `quarter-wave-dot`)
**Domain:** Photonics / Optics
**Definition:** Place λ/4 phase plate at Fourier-plane DC spot; converts phase variations in object to intensity contrast.
**Atom or composite:** Composite (quarter-wave dot in 4f).
**Real wall?** Yes — only for weak-phase objects.
**Cross-domain wiring:** Same as signal-processing-rf I/Q conversion via 90° phase shifter.
**Notes:** Zernike 1942 Nobel; key for transparent biological samples.

### dark-field-imaging (cross-domain alias: `DF-microscopy`, `oblique-illumination`)
**Domain:** Photonics / Optics
**Definition:** Direct (undiffracted) light blocked at Fourier plane; only scattered light reaches detector — high contrast for scatterers.
**Atom or composite:** Composite (annular illumination + central stop).
**Real wall?** Yes — signal scales as N_scatterer.
**Cross-domain wiring:** Same as signal-processing-rf null-steering; rejecting direct path.
**Notes:** Hecht §13.

### schlieren-imaging (cross-domain alias: `knife-edge-imaging`, `density-visualization`)
**Domain:** Photonics / Optics
**Definition:** Knife-edge at Fourier plane blocks half the diffraction spectrum; image intensity ∝ ∇n in object.
**Atom or composite:** Composite (knife-edge + 4f).
**Real wall?** Yes — sensitivity vs dynamic range trade.
**Cross-domain wiring:** Same as signal-processing-rf single-sideband; physics-diffusion shock-wave visualization.
**Notes:** Settles 2001.

### shack-hartmann (cross-domain alias: `SHWS`, `lenslet-array-sensor`)
**Domain:** Photonics / Optics
**Definition:** Lenslet array samples wavefront; per-spot centroid shift ∝ local wavefront tilt; integrate to get W(x,y).
**Atom or composite:** Composite (lenslet array + camera + centroid algorithm).
**Cost model:** O(N) centroids + Zernike fit.
**Real wall?** Yes — sample spacing = lenslet pitch sets spatial resolution.
**Cross-domain wiring:** Same gradient-sampling as ml-training finite-difference; computational-geometry mesh-tilt.
**Notes:** Shack & Platt 1971; basis of adaptive optics.

### shadowgraph (cross-domain alias: `shadowgraphy`, `direct-shadow`)
**Domain:** Photonics / Optics
**Definition:** Image directly proportional to ∇²n in test region (second derivative of refractive index).
**Atom or composite:** Composite (point source + screen).
**Real wall?** Yes — quantitative recovery requires schlieren or BOS (background-oriented schlieren).
**Cross-domain wiring:** Same as physics-diffusion Laplacian visualization.
**Notes:** Settles 2001.

### foucault-knife-edge (cross-domain alias: `Foucault-test`, `optic-figure-test`)
**Domain:** Photonics / Optics
**Definition:** Knife-edge at focus reveals wavefront errors as shadows on illuminated mirror.
**Atom or composite:** Composite (knife-edge near focus).
**Real wall?** Yes — qualitative; needs Hartmann test for quantitative.
**Cross-domain wiring:** Same edge-detection principle as graphics-rendering-lod gradient detector.
**Notes:** Foucault 1859; classic telescope-mirror test.

---

## Nonlinear Optics

### chi2-shg (cross-domain alias: `second-harmonic-generation`, `SHG`, `frequency-doubling`)
**Domain:** Photonics / Optics
**Definition:** P^(2)(2ω) = ε₀ χ⁽²⁾ E²(ω); two photons at ω → one photon at 2ω in non-centrosymmetric crystal.
**Atom or composite:** Composite (χ⁽²⁾ nonlinear polarization → wave generation).
**Cost model:** Conversion efficiency η ∝ |χ⁽²⁾|² L² I (phase-matched).
**Real wall?** Yes — phase matching Δk = 2k(ω) − k(2ω) = 0 required.
**Cross-domain wiring:** Same parametric mixing as signal-processing-rf mixer harmonics; quantum-computing two-photon down-conversion (inverse).
**Notes:** Boyd §2.

### chi2-sfg (cross-domain alias: `sum-frequency-generation`, `SFG`)
**Domain:** Photonics / Optics
**Definition:** ω₁ + ω₂ → ω₃ in χ⁽²⁾ medium; momentum: k₁ + k₂ = k₃.
**Atom or composite:** Composite (3-wave mixing).
**Real wall?** Yes — phase matching.
**Cross-domain wiring:** Same as signal-processing-rf upconversion mixer.
**Notes:** Boyd §2.

### chi2-dfg (cross-domain alias: `difference-frequency-generation`, `DFG`)
**Domain:** Photonics / Optics
**Definition:** ω₁ − ω₂ → ω₃; produces idler at ω₁ − ω₂; basis of mid-IR sources.
**Atom or composite:** Composite (3-wave mixing).
**Real wall?** Yes — phase matching.
**Cross-domain wiring:** Same as signal-processing-rf downconversion mixer.
**Notes:** Boyd §2.

### opo (cross-domain alias: `optical-parametric-oscillator`, `OPO`)
**Domain:** Photonics / Optics
**Definition:** Parametric gain (χ⁽²⁾) inside resonator; signal+idler oscillate when round-trip gain > loss; tunable.
**Atom or composite:** Composite (OPA inside cavity).
**Cost model:** Threshold pump intensity; gain bandwidth.
**Real wall?** Yes — pump threshold and damage threshold of crystal.
**Cross-domain wiring:** Same as quantum-computing parametric amplifier (squeezed light); signal-processing-rf parametric oscillator.
**Notes:** Giordmaine & Miller 1965.

### opa (cross-domain alias: `optical-parametric-amplifier`, `OPA`)
**Domain:** Photonics / Optics
**Definition:** Signal amplified by transferring energy from pump via χ⁽²⁾; gain g = 2π χ⁽²⁾ E_p L/(λ n).
**Atom or composite:** Composite (single-pass parametric gain).
**Real wall?** Yes — phase matching limits gain bandwidth.
**Cross-domain wiring:** Quantum-limited amplifier; analog to quantum-computing two-mode squeezing.
**Notes:** Boyd §2.

### spdc (cross-domain alias: `spontaneous-parametric-down-conversion`, `PDC`)
**Domain:** Photonics / Optics
**Definition:** Pump photon → signal+idler pair via χ⁽²⁾, conservation ω_p = ω_s + ω_i, k_p = k_s + k_i.
**Atom or composite:** Composite (quantum-vacuum-seeded OPA).
**Real wall?** Yes — pair-generation rate × heralding efficiency.
**Cross-domain wiring:** Core source for quantum-computing entangled-photon pairs.
**Notes:** Burnham & Weinberg 1970.

### chi3-thg (cross-domain alias: `third-harmonic-generation`, `THG`)
**Domain:** Photonics / Optics
**Definition:** P⁽³⁾(3ω) = ε₀ χ⁽³⁾ E³(ω); 3 ω → ω in centrosymmetric medium.
**Atom or composite:** Composite (four-wave mixing degenerate).
**Real wall?** Yes — typically lower efficiency than cascaded χ⁽²⁾ doublings.
**Cross-domain wiring:** Same as signal-processing-rf 3rd-harmonic distortion.
**Notes:** Boyd §4.

### spm (cross-domain alias: `self-phase-modulation`, `SPM`, `Kerr-self`)
**Domain:** Photonics / Optics
**Definition:** Intensity-dependent index n = n₀ + n₂ I induces self-imposed phase φ_NL = γ P L; spectrally broadens pulse.
**Atom or composite:** Composite (Kerr index + pulse propagation).
**Real wall?** Yes — eventually wavebreaking; soliton balance with anomalous GVD.
**Cross-domain wiring:** Same as signal-processing-rf AM-to-PM distortion.
**Notes:** Agrawal §4.

### xpm (cross-domain alias: `cross-phase-modulation`, `XPM`)
**Domain:** Photonics / Optics
**Definition:** Phase of one wave modulated by intensity of another co-propagating wave: φ_NL = 2γ P_other L.
**Atom or composite:** Composite (Kerr + two-color).
**Real wall?** Yes — limits WDM channel spacing.
**Cross-domain wiring:** Source of crosstalk in networking optical channels.
**Notes:** Agrawal §7.

### fwm (cross-domain alias: `four-wave-mixing`, `FWM`)
**Domain:** Photonics / Optics
**Definition:** ω₁ + ω₂ = ω₃ + ω₄ via χ⁽³⁾; basis of fiber parametric amplifiers and frequency combs.
**Atom or composite:** Composite (4-wave nonlinear mixing).
**Real wall?** Yes — phase-matching across 4 frequencies.
**Cross-domain wiring:** Same as signal-processing-rf intermodulation distortion (3rd-order).
**Notes:** Boyd §4.

### raman-scattering (cross-domain alias: `SRS`, `Stokes-anti-Stokes`)
**Domain:** Photonics / Optics
**Definition:** Inelastic scattering shifts photon by vibrational quantum ω_v; Stokes at ω−ω_v gains gain g_R I L.
**Atom or composite:** Composite (χ⁽³⁾ with material resonance).
**Real wall?** Yes — Raman gain threshold (e.g., ~16 dBm in km of SMF).
**Cross-domain wiring:** Same Raman process used in physics-diffusion vibrational spectroscopy; signal-processing-rf phonon-coupling.
**Notes:** Boyd §10.

### brillouin-scattering (cross-domain alias: `SBS`, `acoustic-scattering`)
**Domain:** Photonics / Optics
**Definition:** Light scattered from acoustic phonons; narrow GHz Stokes shift; very high gain in fibers.
**Atom or composite:** Composite (electrostriction-driven χ⁽³⁾).
**Real wall?** Yes — SBS threshold limits CW power in long fibers.
**Cross-domain wiring:** Same as signal-processing-rf acoustooptic interaction.
**Notes:** Boyd §9.

### phase-matching-birefringent (cross-domain alias: `angle-tuning`, `BPM`)
**Domain:** Photonics / Optics
**Definition:** Use birefringence n_o(λ) vs n_e(λ,θ) to satisfy Δk = 0 by tuning crystal angle.
**Atom or composite:** Composite.
**Real wall?** Yes — walk-off limits interaction length.
**Cross-domain wiring:** Same kind of dispersion-engineering as signal-processing-rf delay-line matching.
**Notes:** Boyd §2.3.

### qpm (cross-domain alias: `quasi-phase-matching`, `periodic-poling`, `PPLN`)
**Domain:** Photonics / Optics
**Definition:** Reverse χ⁽²⁾ every coherence length Λ_c via domain reversal; effective phase-match Δk_eff = Δk − 2π/Λ = 0.
**Atom or composite:** Composite (periodic poling + 3-wave mixing).
**Real wall?** Yes — poling period accuracy; aperiodic chirped poling for broadband.
**Cross-domain wiring:** Same as signal-processing-rf periodic structures providing reciprocal-lattice phase.
**Notes:** Armstrong et al. 1962.

### self-focusing (cross-domain alias: `Kerr-self-focus`, `optical-Townes-collapse`)
**Domain:** Photonics / Optics
**Definition:** n = n₀ + n₂ I causes intense beam to converge; critical power P_cr ≈ 3.77 λ²/(8π n₀ n₂).
**Atom or composite:** Composite (Kerr + diffraction balance).
**Real wall?** Yes — leads to filamentation and damage above P_cr.
**Cross-domain wiring:** Same as physics-diffusion nonlinear Schrödinger collapse.
**Notes:** Boyd §7.

### kerr-lens (cross-domain alias: `KL`, `intensity-lens`)
**Domain:** Photonics / Optics
**Definition:** Self-focusing acts as intensity-dependent lens — basis of Kerr-lens mode locking.
**Atom or composite:** Composite (Kerr nonlinearity + Gaussian beam).
**Real wall?** Yes — must avoid catastrophic collapse.
**Cross-domain wiring:** Same as quantum-computing optical Kerr gate.
**Notes:** Spence 1991.

### temporal-soliton (cross-domain alias: `optical-soliton`, `NLSE-soliton`)
**Domain:** Photonics / Optics
**Definition:** Balanced anomalous GVD + SPM yields shape-preserving pulse: A(z,t) = sech(t/τ) e^{iγPz/2}.
**Atom or composite:** Composite (NLSE eigenstate).
**Real wall?** No — but loss disrupts soliton, requires Raman amplification.
**Cross-domain wiring:** Same as physics-diffusion solitons (KdV, NLSE); signal-processing-rf shape-preserving pulses.
**Notes:** Hasegawa & Tappert 1973.

### spatial-soliton (cross-domain alias: `self-trapped-beam`, `2D-soliton`)
**Domain:** Photonics / Optics
**Definition:** Balanced diffraction + self-focusing produces shape-preserving beam in space; stable in (1+1)D.
**Atom or composite:** Composite (Kerr + diffraction).
**Real wall?** Yes — 2D Kerr solitons are unstable (Townes profile); stable in saturable nonlinearity.
**Cross-domain wiring:** Same as physics-diffusion nonlinear-Schrödinger stationary solutions.
**Notes:** Chiao et al. 1964.

### supercontinuum (cross-domain alias: `SC-generation`, `white-light-laser`)
**Domain:** Photonics / Optics
**Definition:** Ultrabroadband (often >octave) coherent spectrum from intense pulse in nonlinear fiber via SPM+FWM+Raman+soliton fission.
**Atom or composite:** Composite (multiple χ⁽³⁾ effects).
**Cost model:** Photonic-crystal fiber pumped near ZDW.
**Real wall?** Yes — coherence degrades for long pulses (modulation instability).
**Cross-domain wiring:** Same as signal-processing-rf wideband chirp; basis of frequency combs.
**Notes:** Ranka 2000.

### optical-bistability (cross-domain alias: `OB`, `Kerr-cavity-hysteresis`)
**Domain:** Photonics / Optics
**Definition:** Nonlinear cavity has two stable transmission states for the same input — basis of optical logic.
**Atom or composite:** Composite (Kerr + Fabry-Perot feedback).
**Real wall?** Yes — switching power × speed limited by photon-cavity lifetime.
**Cross-domain wiring:** Same as control-numerical-opt hysteresis; ml-training bistable neurons.
**Notes:** Gibbs 1985.

### parametric-amplification (cross-domain alias: `PA`, `χ⁽²⁾-amp`)
**Domain:** Photonics / Optics
**Definition:** Coherent amplification of signal at ω_s using pump at ω_p ≈ 2ω_s; gain G ≈ cosh²(gL).
**Atom or composite:** Composite (OPA).
**Real wall?** Yes — quantum noise added on idler arm; phase-sensitive PA can be noiseless.
**Cross-domain wiring:** Same as quantum-computing parametric amplifier in superconducting circuits.
**Notes:** Hansryd 2002.

---

## Lasers

### population-inversion (cross-domain alias: `N2>N1`, `negative-temperature`)
**Domain:** Photonics / Optics
**Definition:** Upper-state population N₂ exceeds lower-state N₁; required for net stimulated emission gain.
**Atom or composite:** Atom (foundational laser condition).
**Cost model:** Pumping rate vs spontaneous decay.
**Real wall?** Yes — impossible in pure 2-level system in steady state (must be ≥ 3 levels).
**Cross-domain wiring:** Same negative-Boltzmann distribution concept in physics-quantum spin systems.
**Notes:** Schawlow & Townes 1958.

### stimulated-emission (cross-domain alias: `Einstein-B`, `coherent-emission`)
**Domain:** Photonics / Optics
**Definition:** Photon induces atomic transition emitting identical photon; rate ∝ B₂₁ N₂ ρ(ν).
**Atom or composite:** Atom (one of the three Einstein rate processes).
**Real wall?** Yes — Einstein's relation A/B = 8πhν³/c³ ties emission to absorption.
**Cross-domain wiring:** Quantum process underlying lasers, masers; analog of resonant signal amplification.
**Notes:** Einstein 1917.

### gain-medium (cross-domain alias: `active-medium`, `amplifier`)
**Domain:** Photonics / Optics
**Definition:** Material with population inversion producing optical gain g(ν) = σ(ν)(N₂ − (g₂/g₁)N₁).
**Atom or composite:** Composite (medium + pumping scheme).
**Cost model:** Gain × length product; pump efficiency.
**Real wall?** Yes — saturation: g(I) = g₀/(1+I/I_sat).
**Cross-domain wiring:** Analog of signal-processing-rf amplifier with compression.
**Notes:** Siegman §1.

### optical-resonator (cross-domain alias: `laser-cavity`, `Fabry-Perot-cavity`)
**Domain:** Photonics / Optics
**Definition:** Two (or more) mirrors providing feedback; supports discrete modes at ν_q = qc/(2nL).
**Atom or composite:** Composite (mirror geometry + propagation).
**Real wall?** Yes — diffraction loss; stability criterion 0 ≤ g₁g₂ ≤ 1 with g_i = 1 − L/R_i.
**Cross-domain wiring:** Same as signal-processing-rf resonator (Q-factor); quantum-computing cavity-QED.
**Notes:** Siegman §19.

### rate-equations (cross-domain alias: `laser-rate-eqs`, `Statz-DeMars`)
**Domain:** Photonics / Optics
**Definition:** Coupled ODEs for population and photon number: dN/dt = R_p − N/τ − cσφN; dφ/dt = (cσN − γ)φ.
**Atom or composite:** Composite (ODE system).
**Cost model:** O(N) per timestep; stiff for relaxation oscillations.
**Real wall?** Yes — only valid for slowly varying envelopes; misses coherent dynamics.
**Cross-domain wiring:** Same form as predator-prey (Lotka-Volterra) in biology-bioinformatics.
**Notes:** Siegman §13.

### four-level-system (cross-domain alias: `4-level-laser`, `Nd:YAG-scheme`)
**Domain:** Photonics / Optics
**Definition:** Pump → level 3, fast decay to 2, lasing 2→1, fast decay 1→0; inversion easy because N₁ ≈ 0.
**Atom or composite:** Composite (4 energy levels + rates).
**Real wall?** Yes — pump quantum-defect heat load.
**Cross-domain wiring:** Same population-pumping logic in maser physics.
**Notes:** Schawlow & Townes; Nd³⁺ is canonical example.

### three-level-system (cross-domain alias: `3-level-laser`, `ruby-scheme`)
**Domain:** Photonics / Optics
**Definition:** Pump 1→3, fast 3→2, lasing 2→1; ground-state recovery slow → high threshold.
**Atom or composite:** Composite.
**Real wall?** Yes — half population must be pumped before threshold.
**Cross-domain wiring:** Less efficient than 4-level; same Boltzmann-pumping concept.
**Notes:** Maiman 1960 ruby laser.

### lasing-threshold (cross-domain alias: `Pth`, `gain=loss`)
**Domain:** Photonics / Optics
**Definition:** Gain per round trip equals total loss; above threshold N₂ clamps and output rises linearly with pump.
**Atom or composite:** Composite (gain-loss balance).
**Real wall?** Yes — pump efficiency × beam quality affects achievable threshold.
**Cross-domain wiring:** Same as control-numerical-opt phase-transition onset; statistics-probability bifurcation.
**Notes:** Siegman §13.

### slope-efficiency (cross-domain alias: `η_s`, `dP_out/dP_in`)
**Domain:** Photonics / Optics
**Definition:** Slope of output vs pump above threshold: η_s ≈ η_pump × η_quantum × η_mode × η_extraction.
**Atom or composite:** Composite.
**Real wall?** Yes — capped by quantum defect (ν_lase/ν_pump).
**Cross-domain wiring:** Same as signal-processing-rf amplifier efficiency.
**Notes:** Koechner §3.

### longitudinal-mode (cross-domain alias: `axial-mode`, `comb-line`)
**Domain:** Photonics / Optics
**Definition:** Standing-wave mode along resonator axis at ν_q = qc/(2nL); FSR = c/(2nL).
**Atom or composite:** Atom (mode index q).
**Real wall?** Yes — gain bandwidth × FSR sets number of oscillating modes.
**Cross-domain wiring:** Same as signal-processing-rf cavity modes; quantum-computing harmonic-oscillator levels.
**Notes:** Siegman §11.

### transverse-mode (cross-domain alias: `TEM_mn`, `transverse-pattern`)
**Domain:** Photonics / Optics
**Definition:** HG/LG mode of resonator transverse to axis; mode losses select mode hierarchy.
**Atom or composite:** Composite (Hermite-Gauss basis).
**Real wall?** Yes — TEM00 single-mode operation requires aperture or stable confocal cavity.
**Cross-domain wiring:** Same orthogonal-mode set as fiber-optic modes.
**Notes:** Siegman §17.

### q-switching (cross-domain alias: `Q-switch`, `giant-pulse`)
**Domain:** Photonics / Optics
**Definition:** Modulate cavity Q to store inversion then release in ns pulse; peak power ≫ CW.
**Atom or composite:** Composite (modulator + gain).
**Cost model:** Repetition rate × pulse energy; passive Q-switch uses saturable absorber.
**Real wall?** Yes — minimum pulse length ~ cavity round-trip.
**Cross-domain wiring:** Same as signal-processing-rf energy-storage-then-release pulsed transmit.
**Notes:** McClung & Hellwarth 1962.

### mode-locking-active (cross-domain alias: `AML`, `synchronous-modulation`)
**Domain:** Photonics / Optics
**Definition:** Loss modulator driven at cavity FSR locks longitudinal modes in phase → ps pulses.
**Atom or composite:** Composite (cavity + active modulator).
**Real wall?** Yes — modulator bandwidth limits min pulse width.
**Cross-domain wiring:** Same as signal-processing-rf injection locking.
**Notes:** Siegman §27.

### mode-locking-passive (cross-domain alias: `PML`, `SAM-mode-lock`)
**Domain:** Photonics / Optics
**Definition:** Self-amplitude modulator (saturable absorber) favors high-intensity short pulse over CW.
**Atom or composite:** Composite (SAM + dispersion management).
**Real wall?** Yes — bandwidth limits min pulse to ~few cycles.
**Cross-domain wiring:** Same self-organization principle as signal-processing-rf bistable pulsers.
**Notes:** Keller §4.

### klm (cross-domain alias: `Kerr-lens-mode-lock`, `Ti:Sapph-mode-lock`)
**Domain:** Photonics / Optics
**Definition:** Self-focusing in gain medium creates intensity-dependent aperture loss favoring short pulses.
**Atom or composite:** Composite (Kerr lensing + hard/soft aperture).
**Real wall?** Yes — sub-5-fs achievable in Ti:Sapphire.
**Cross-domain wiring:** Same Kerr-lens dynamics as spatial soliton; nonlinear feedback in control-numerical-opt.
**Notes:** Spence 1991.

### sesam (cross-domain alias: `semiconductor-saturable-absorber-mirror`)
**Domain:** Photonics / Optics
**Definition:** Multilayer Bragg + semiconductor absorber; saturable reflectivity passively starts and stabilizes mode-locking.
**Atom or composite:** Composite (DBR + QW absorber).
**Real wall?** Yes — recovery time and damage threshold tradeoff.
**Cross-domain wiring:** Same intensity-dependent loss as Kerr-aperture.
**Notes:** Keller 1996.

### frequency-comb (cross-domain alias: `optical-comb`, `f_ceo+nf_rep`)
**Domain:** Photonics / Optics
**Definition:** Train of phase-locked pulses → spectrum is comb of lines at ν_n = f_ceo + n f_rep.
**Atom or composite:** Composite (mode-locked laser + CEO stabilization).
**Real wall?** Yes — needs octave-spanning spectrum for f-2f self-referencing.
**Cross-domain wiring:** Same as signal-processing-rf reference comb; quantum-computing optical clock synthesis.
**Notes:** Hänsch & Hall Nobel 2005.

### optical-frequency-synthesis (cross-domain alias: `OFS`, `cw-from-comb`)
**Domain:** Photonics / Optics
**Definition:** Phase-lock CW laser to a chosen comb tooth; transfers comb stability to arbitrary optical frequency.
**Atom or composite:** Composite (phase lock + comb).
**Real wall?** Yes — comb tooth linewidth.
**Cross-domain wiring:** Same as networking timing distribution; signal-processing-rf PLL.
**Notes:** Diddams 2010.

---

## Laser Types

### hene-laser (cross-domain alias: `HeNe`, `633nm-gas-laser`)
**Domain:** Photonics / Optics
**Definition:** Helium-neon discharge; 632.8 nm red line is the iconic transition (3s₂→2p₄).
**Atom or composite:** Composite (gas discharge + Brewster windows + cavity).
**Real wall?** Yes — power capped ~ tens of mW; replaced by diode lasers.
**Cross-domain wiring:** Same kind of plasma-discharge as physics-diffusion gas-discharge models.
**Notes:** Javan 1961.

### argon-ion-laser (cross-domain alias: `Ar+`, `ion-laser`)
**Domain:** Photonics / Optics
**Definition:** Ionized argon plasma; multiple visible lines (488, 514 nm); high power but very inefficient.
**Atom or composite:** Composite (plasma + magnetic confinement).
**Real wall?** Yes — kW input for W output; mostly retired.
**Cross-domain wiring:** Power-budget extreme example; same plasma physics in physics-diffusion fusion devices.
**Notes:** Bridges 1964.

### co2-laser (cross-domain alias: `CO2`, `10.6um-laser`)
**Domain:** Photonics / Optics
**Definition:** Vibrational-rotational transition in CO₂; 10.6 μm; multi-kW industrial workhorse.
**Atom or composite:** Composite (molecular gas + RF/DC discharge).
**Real wall?** Yes — needs ZnSe optics; mid-IR safety.
**Cross-domain wiring:** Vibrational-mode lasing — same kind of mode as physics-diffusion molecular dynamics.
**Notes:** Patel 1964.

### ndyag-laser (cross-domain alias: `Nd:YAG`, `1064nm`)
**Domain:** Photonics / Optics
**Definition:** Neodymium-doped yttrium aluminum garnet; 1064 nm, four-level scheme.
**Atom or composite:** Composite (Nd³⁺ ion + YAG host).
**Real wall?** Yes — thermal lensing at high power.
**Cross-domain wiring:** Workhorse for SHG to 532 nm green; same ion-host system in EDFA.
**Notes:** Geusic 1964.

### nd-yvo4 (cross-domain alias: `Nd:Vanadate`, `Nd:YVO4`)
**Domain:** Photonics / Optics
**Definition:** Nd-doped yttrium orthovanadate; higher absorption cross-section than YAG → diode-pumped efficiency.
**Atom or composite:** Composite.
**Real wall?** Yes — lower thermal conductivity than YAG.
**Cross-domain wiring:** Material-engineering tradeoff cf. material selection in physics-quantum.
**Notes:** Common in DPSS green lasers.

### ti-sapphire (cross-domain alias: `Ti:Sapph`, `tunable-fs-laser`)
**Domain:** Photonics / Optics
**Definition:** Ti³⁺:Al₂O₃; ultrabroad gain (650–1100 nm) supports few-fs pulses.
**Atom or composite:** Composite (vibronic gain medium).
**Real wall?** Yes — needs blue/green pump; dispersion management for short pulses.
**Cross-domain wiring:** Broadband gain enables full frequency comb; same broadband-amp concept as signal-processing-rf TWT.
**Notes:** Moulton 1986.

### er-fiber-laser (cross-domain alias: `Er:fiber`, `1550nm-fiber`)
**Domain:** Photonics / Optics
**Definition:** Erbium-doped fiber laser; 1550 nm telecom band; pumped at 980 or 1480 nm.
**Atom or composite:** Composite (doped fiber + cavity).
**Real wall?** Yes — ASE noise; gain narrowing.
**Cross-domain wiring:** Same gain as EDFA; underpins networking long-haul optical links.
**Notes:** Mears 1987.

### yb-fiber-laser (cross-domain alias: `Yb:fiber`, `1030nm-high-power`)
**Domain:** Photonics / Optics
**Definition:** Yb-doped fiber; high efficiency (>80%), high power (>10 kW CW), 1030 nm.
**Atom or composite:** Composite (large-mode-area fiber + cladding pump).
**Real wall?** Yes — mode instability (TMI) limits CW power.
**Cross-domain wiring:** Industrial cutting workhorse; same scaling as physics-diffusion thermal management.
**Notes:** Limpert 2007.

### dye-laser (cross-domain alias: `organic-dye-laser`, `tunable-cw-dye`)
**Domain:** Photonics / Optics
**Definition:** Organic dye in solvent provides broadband tunable gain (visible); largely replaced by Ti:Sapph + OPO.
**Atom or composite:** Composite (dye flow cell + pump).
**Real wall?** Yes — dye photobleaching.
**Cross-domain wiring:** Tunability concept identical to Ti:Sapph; chemistry-engineering analog.
**Notes:** Sorokin 1966.

### semiconductor-fp-laser (cross-domain alias: `FP-LD`, `cleaved-facet-LD`)
**Domain:** Photonics / Optics
**Definition:** Semiconductor laser with cleaved-facet Fabry-Perot cavity; multi-mode unless filtered.
**Atom or composite:** Composite (active region + waveguide + facet mirrors).
**Real wall?** Yes — chirp and broad linewidth.
**Cross-domain wiring:** Cheapest coherent source; powers networking short-reach optics.
**Notes:** Hayashi 1970.

### dfb-laser (cross-domain alias: `distributed-feedback`, `DFB-LD`)
**Domain:** Photonics / Optics
**Definition:** Integrated grating along active region selects single λ; single-mode, narrow linewidth.
**Atom or composite:** Composite (grating + gain).
**Real wall?** Yes — temperature tuning ~ 0.1 nm/°C.
**Cross-domain wiring:** Same Bragg-selectivity as FBG; key to networking DWDM.
**Notes:** Kogelnik 1971.

### dbr-laser (cross-domain alias: `distributed-Bragg-reflector`)
**Domain:** Photonics / Optics
**Definition:** Bragg mirror at one or both ends of gain section — single-mode with tunable Bragg section.
**Atom or composite:** Composite (gain + DBR mirrors).
**Real wall?** Yes — mode hops at section-current boundaries.
**Cross-domain wiring:** Tunable laser source for networking widely-tunable transceivers.
**Notes:** Coldren 2012.

### vcsel (cross-domain alias: `vertical-cavity-surface-emitting-laser`)
**Domain:** Photonics / Optics
**Definition:** Vertical resonator with DBR mirrors top/bottom; circular beam, low threshold, wafer-scale.
**Atom or composite:** Composite (epitaxial DBRs + QW gain).
**Real wall?** Yes — short cavity limits gain; needs >99.5% DBR.
**Cross-domain wiring:** Same wafer-scale economics as ml-training GPU chip yields.
**Notes:** Iga 1979.

### qcl (cross-domain alias: `quantum-cascade-laser`, `intersubband-laser`)
**Domain:** Photonics / Optics
**Definition:** Stack of QW intersubband transitions; mid-IR to THz; one electron emits many photons.
**Atom or composite:** Composite (engineered band structure).
**Real wall?** Yes — needs cryogenic for THz; high voltage for mid-IR.
**Cross-domain wiring:** Same engineered-band approach as quantum-computing topological circuits.
**Notes:** Faist 1994.

### qd-laser (cross-domain alias: `quantum-dot-laser`, `0D-laser`)
**Domain:** Photonics / Optics
**Definition:** Self-assembled QDs in gain region; δ-like DOS → low threshold, low chirp, temperature-stable.
**Atom or composite:** Composite (Stranski-Krastanov QD layer).
**Real wall?** Yes — inhomogeneous broadening of QD ensemble.
**Cross-domain wiring:** Same 0D quantization as quantum-computing dot qubits.
**Notes:** Arakawa 1982.

### fel (cross-domain alias: `free-electron-laser`, `undulator-laser`)
**Domain:** Photonics / Optics
**Definition:** Relativistic electrons through periodic magnet undulator radiate at λ ∝ λ_u/(2γ²); tunable from THz to X-ray.
**Atom or composite:** Composite (relativistic e-beam + undulator + cavity or SASE).
**Cost model:** Accelerator-scale facility.
**Real wall?** Yes — emittance and energy spread limit gain.
**Cross-domain wiring:** Same kind of synchrotron radiation as electromagnetics-antennas.
**Notes:** Madey 1971.

### xfel (cross-domain alias: `X-ray-free-electron-laser`, `LCLS-class`)
**Domain:** Photonics / Optics
**Definition:** Self-amplified spontaneous emission FEL at sub-nm wavelengths; fs hard X-ray pulses.
**Atom or composite:** Composite (km-scale linac + long undulator).
**Real wall?** Yes — facility scale; SASE has temporal jitter.
**Cross-domain wiring:** Enables single-molecule diffractive imaging (biology-bioinformatics); ultrafast atomic physics.
**Notes:** Emma 2010 LCLS first lasing.

---

## Detectors

### pin-photodiode (cross-domain alias: `PIN-PD`, `linear-photodetector`)
**Domain:** Photonics / Optics
**Definition:** p-i-n junction with wide intrinsic region; photogenerated carriers swept by E-field; responsivity R = ηqλ/(hc).
**Atom or composite:** Atom (semiconductor photodetector).
**Cost model:** Bandwidth × area trade; capacitance C = εA/d.
**Real wall?** Yes — quantum efficiency ≤ 1; shot noise floor.
**Cross-domain wiring:** Same R-C detector model as signal-processing-rf diode detector.
**Notes:** Saleh & Teich §17.

### apd (cross-domain alias: `avalanche-photodiode`, `gain-photodiode`)
**Domain:** Photonics / Optics
**Definition:** Reverse-biased PD with impact-ionization gain M; detects weak signals at cost of excess noise factor F.
**Atom or composite:** Composite (PIN + avalanche region).
**Real wall?** Yes — gain-bandwidth product; excess-noise F(M).
**Cross-domain wiring:** Same as signal-processing-rf low-noise amp before detection.
**Notes:** McIntyre 1966.

### spad (cross-domain alias: `single-photon-avalanche-diode`, `Geiger-PD`)
**Domain:** Photonics / Optics
**Definition:** APD biased above breakdown; single photon triggers macroscopic current pulse; quenched and reset.
**Atom or composite:** Composite (APD + quench circuit).
**Real wall?** Yes — dead time, dark counts, afterpulsing.
**Cross-domain wiring:** Same digital-counting principle as physics-diffusion Geiger counter.
**Notes:** Cova 1996.

### pmt (cross-domain alias: `photomultiplier-tube`, `dynode-multiplier`)
**Domain:** Photonics / Optics
**Definition:** Photoemissive cathode + dynode chain; gain 10⁶–10⁸ via secondary emission.
**Atom or composite:** Composite (photocathode + dynodes + anode).
**Real wall?** Yes — magnetic-field sensitive; fragile.
**Cross-domain wiring:** Same electron multiplication as physics-quantum cascade processes.
**Notes:** Iams 1936.

### ccd (cross-domain alias: `charge-coupled-device`, `bucket-brigade-sensor`)
**Domain:** Photonics / Optics
**Definition:** Array of MOS capacitors; photogenerated charge transferred bucket-brigade to readout; high QE, low noise.
**Atom or composite:** Composite (pixel array + charge transfer + ADC).
**Real wall?** Yes — slow readout; charge-transfer inefficiency.
**Cross-domain wiring:** Same shift-register topology as digital memory; classic sensor in graphics-rendering-lod.
**Notes:** Boyle & Smith 1969 Nobel.

### cmos-image-sensor (cross-domain alias: `CIS`, `active-pixel-sensor`)
**Domain:** Photonics / Optics
**Definition:** Each pixel has its own readout transistor; parallel readout, low power, integrates with logic.
**Atom or composite:** Composite (pixel + col-amp + ADC).
**Real wall?** Yes — fixed-pattern noise; rolling shutter artifacts (mitigated by global shutter).
**Cross-domain wiring:** Standard sensor for ml-training, graphics-rendering-lod.
**Notes:** Fossum 1993.

### emccd (cross-domain alias: `electron-multiplying-CCD`, `L3-CCD`)
**Domain:** Photonics / Optics
**Definition:** CCD with on-chip avalanche register; sub-electron read noise; for single-photon imaging.
**Atom or composite:** Composite (CCD + multiplication register).
**Real wall?** Yes — clock-induced charge; excess-noise factor √2.
**Cross-domain wiring:** Single-photon imaging analog of SPAD arrays.
**Notes:** Jerram 2001.

### scmos (cross-domain alias: `scientific-CMOS`, `sCMOS`)
**Domain:** Photonics / Optics
**Definition:** CMOS sensor with low read noise (~1 e⁻ RMS), high QE, large array, high frame rate.
**Atom or composite:** Composite (correlated-double-sampling + dual-gain ADC).
**Real wall?** Yes — row noise; nonlinearity at saturation.
**Cross-domain wiring:** Replacing EMCCD in life-sciences imaging.
**Notes:** Andor/Hamamatsu datasheets.

### iccd (cross-domain alias: `intensified-CCD`, `gated-imager`)
**Domain:** Photonics / Optics
**Definition:** Image intensifier (photocathode + MCP + phosphor) coupled to CCD; ns-gating for time-resolved imaging.
**Atom or composite:** Composite (intensifier + CCD).
**Real wall?** Yes — limited dynamic range; MCP ageing.
**Cross-domain wiring:** Time-gated detection analog of signal-processing-rf range-gating.
**Notes:** Photonis tech notes.

### bolometer (cross-domain alias: `thermal-detector`, `Boltz-IR`)
**Domain:** Photonics / Optics
**Definition:** Detector where absorbed radiation raises temperature; resistance change read out (TES, semiconductor).
**Atom or composite:** Composite (absorber + thermistor + thermal bath).
**Real wall?** Yes — NEP set by thermal noise √(4kT²G).
**Cross-domain wiring:** Same heat-balance analysis as physics-diffusion thermal transport.
**Notes:** Richards 1994.

### microbolometer (cross-domain alias: `µ-bol`, `uncooled-IR`)
**Domain:** Photonics / Optics
**Definition:** MEMS pixel array of thermistors; 8–14 µm thermal imaging without cryogenic cooling.
**Atom or composite:** Composite (suspended absorber + thermistor).
**Real wall?** Yes — NETD ~50 mK limited by 1/f and JN.
**Cross-domain wiring:** Same MEMS thermal design as bioMEMS sensors.
**Notes:** FLIR sensors.

### pyroelectric (cross-domain alias: `pyro-detector`, `dT/dt-detector`)
**Domain:** Photonics / Optics
**Definition:** Pyroelectric crystal generates current ∝ dT/dt; AC detection — needs chopped illumination.
**Atom or composite:** Composite (pyro crystal + transimpedance amp).
**Real wall?** Yes — Curie-temperature limit.
**Cross-domain wiring:** Differential thermal detection analog of signal-processing-rf AC-coupled.
**Notes:** Putley 1980.

### thermopile (cross-domain alias: `Seebeck-detector`, `power-meter-detector`)
**Domain:** Photonics / Optics
**Definition:** Series-connected thermocouples; Seebeck voltage ∝ ΔT ∝ absorbed power.
**Atom or composite:** Composite (junction array + absorber).
**Real wall?** Yes — slow (ms-s response) but flat spectral.
**Cross-domain wiring:** Used as absolute laser power standard; same as physics-diffusion thermoelectric.
**Notes:** Coblentz 1913.

### mct (cross-domain alias: `HgCdTe`, `MCT-IR-detector`)
**Domain:** Photonics / Optics
**Definition:** Mercury-cadmium-telluride photodetector; tunable cutoff (3–14 µm) by composition.
**Atom or composite:** Composite (band-gap engineered PD).
**Real wall?** Yes — requires cryogenic cooling for low dark current.
**Cross-domain wiring:** Same engineered bandgap idea as quantum-computing band engineering.
**Notes:** Lawson 1959.

### snspd (cross-domain alias: `superconducting-nanowire-SPD`, `SNSPD`)
**Domain:** Photonics / Optics
**Definition:** Current-biased superconducting nanowire; absorbed photon creates resistive hot-spot → voltage pulse.
**Atom or composite:** Composite (NbN nanowire + bias circuit).
**Real wall?** Yes — needs ~2 K cryostat; high efficiency (>95%).
**Cross-domain wiring:** Key detector for quantum-computing photonic quantum links.
**Notes:** Gol'tsman 2001.

### tes (cross-domain alias: `transition-edge-sensor`, `TES`)
**Domain:** Photonics / Optics
**Definition:** Superconductor biased on its sharp R(T) transition; energy of single photon raises T → measurable ΔR.
**Atom or composite:** Composite (TES + SQUID readout).
**Real wall?** Yes — sub-K operation; slow recovery vs SNSPD.
**Cross-domain wiring:** Energy-resolving photon counting; analog in physics-quantum X-ray calorimetry.
**Notes:** Irwin 1995.

---

## Fiber Optics

### step-index-fiber (cross-domain alias: `SIF`, `core-cladding-step`)
**Domain:** Photonics / Optics
**Definition:** Core with constant n₁ surrounded by cladding n₂ < n₁; guided modes via total internal reflection.
**Atom or composite:** Composite (cylindrical waveguide).
**Cost model:** Modes from Bessel-function dispersion equations.
**Real wall?** Yes — V = (2πa/λ)√(n₁²−n₂²); V<2.405 = single mode.
**Cross-domain wiring:** Same kind of guided-wave eigenmodes as electromagnetics-antennas rectangular waveguide; quantum-computing well states.
**Notes:** Snyder & Love §12.

### graded-index-fiber (cross-domain alias: `GRIN`, `parabolic-profile`)
**Domain:** Photonics / Optics
**Definition:** n(r) = n₁√(1 − 2Δ(r/a)^α); parabolic profile (α=2) equalizes mode group velocities.
**Atom or composite:** Composite (radial-profile + paraxial WG).
**Real wall?** Yes — only ideal profile fully equalizes; chromatic remains.
**Cross-domain wiring:** Same as physics-diffusion GRIN-lens analog; computational-geometry parabolic-focus.
**Notes:** Saleh & Teich §9.

### smf (cross-domain alias: `single-mode-fiber`, `SMF-28`)
**Domain:** Photonics / Optics
**Definition:** V<2.405 ensures only LP01 mode; standard telecom fiber, 8.2 µm core.
**Atom or composite:** Atom (one guided mode).
**Real wall?** Yes — 0.2 dB/km loss at 1550 nm; chromatic dispersion 17 ps/(nm·km).
**Cross-domain wiring:** Backbone of networking; same single-mode constraint as quantum-computing channels.
**Notes:** ITU-T G.652.

### mmf (cross-domain alias: `multi-mode-fiber`, `MMF`)
**Domain:** Photonics / Optics
**Definition:** Large core (50/62.5 µm) supports many modes; cheap LEDs and VCSELs as sources.
**Atom or composite:** Composite (many modes propagating).
**Real wall?** Yes — modal dispersion limits BWxL product.
**Cross-domain wiring:** Used in short-reach datacenter networking; modal-dispersion analog of multipath in signal-processing-rf.
**Notes:** OM3/OM4/OM5 standards.

### chromatic-dispersion (cross-domain alias: `CD`, `GVD-fiber`)
**Domain:** Photonics / Optics
**Definition:** D = −(λ/c) d²n/dλ² ps/(nm·km); group delay varies with λ; broadens pulses.
**Atom or composite:** Composite (material + waveguide dispersion).
**Real wall?** Yes — limits high-bit-rate transmission distance.
**Cross-domain wiring:** Same as signal-processing-rf group-delay distortion.
**Notes:** Agrawal §1.2.

### pmd (cross-domain alias: `polarization-mode-dispersion`, `PMD`)
**Domain:** Photonics / Optics
**Definition:** Random birefringence in fiber causes DGD between orthogonal polarizations; ⟨ΔT⟩ ∝ √L.
**Atom or composite:** Composite (concatenated birefringent segments).
**Real wall?** Yes — limits 40+ Gbps systems; mitigated by coherent DSP.
**Cross-domain wiring:** Same random-walk algebra as statistics-probability Brownian motion.
**Notes:** Agrawal §6.

### rayleigh-attenuation (cross-domain alias: `Rayleigh-loss`, `1/λ⁴-scattering`)
**Domain:** Photonics / Optics
**Definition:** α_R ∝ 1/λ⁴; intrinsic scattering by density fluctuations frozen in glass.
**Atom or composite:** Atom (fundamental loss term).
**Real wall?** Yes — sets ~0.15 dB/km floor at 1550 nm in silica.
**Cross-domain wiring:** Same Rayleigh-cross-section as electromagnetics-antennas blue-sky scattering.
**Notes:** Lord Rayleigh 1871.

### oh-absorption (cross-domain alias: `water-peak`, `1383nm-loss`)
**Domain:** Photonics / Optics
**Definition:** Vibrational overtones of OH⁻ ions in fiber cause loss peaks at 1244, 1383 nm.
**Atom or composite:** Composite (impurity absorption).
**Real wall?** Yes — modern ULL fiber removes the 1383 peak ("zero-water-peak").
**Cross-domain wiring:** Same as molecular-absorption spectroscopy.
**Notes:** ITU-T G.652.D.

### ir-absorption (cross-domain alias: `multiphonon-loss`)
**Domain:** Photonics / Optics
**Definition:** Si-O lattice vibration absorption rises rapidly beyond ~1700 nm.
**Atom or composite:** Atom (material vibrational tail).
**Real wall?** Yes — caps silica fiber to <1.7 µm; ZBLAN/chalcogenide for mid-IR.
**Cross-domain wiring:** Same lattice-vibration tail as physics-diffusion phonon scattering.
**Notes:** Tonucci 1995.

### pcf (cross-domain alias: `photonic-crystal-fiber`, `holey-fiber`)
**Domain:** Photonics / Optics
**Definition:** Microstructured fiber with air holes; tailorable dispersion, nonlinearity, mode area.
**Atom or composite:** Composite (PhC cladding + solid/hollow core).
**Cost model:** Stack-and-draw fabrication.
**Real wall?** Yes — confinement loss for hollow-core.
**Cross-domain wiring:** Same bandgap engineering as photonic-crystal-grating.
**Notes:** Russell 2003.

### hollow-core-fiber (cross-domain alias: `HCF`, `air-core-PCF`)
**Domain:** Photonics / Optics
**Definition:** Light guided in air via photonic bandgap or antiresonance; very low nonlinearity, latency 30% lower than glass.
**Atom or composite:** Composite (PhC cladding + air core).
**Real wall?** Yes — surface-scattering loss; current ~0.1 dB/km.
**Cross-domain wiring:** Same as quantum-computing low-loss photonic interconnects.
**Notes:** Cregan 1999.

### edfa (cross-domain alias: `erbium-doped-fiber-amplifier`)
**Domain:** Photonics / Optics
**Definition:** Er³⁺-doped fiber pumped at 980/1480 nm provides ~20 dB gain in C-band; quantum-limited noise figure 3 dB.
**Atom or composite:** Composite (doped fiber + WDM + pump diode).
**Real wall?** Yes — gain flatness; ASE accumulates with chain length.
**Cross-domain wiring:** Critical for networking long-haul DWDM; signal-processing-rf gain block.
**Notes:** Mears 1987; Desurvire 1987.

### raman-amplifier (cross-domain alias: `distributed-Raman`, `DRA`)
**Domain:** Photonics / Optics
**Definition:** Pump Stokes-shifts to signal; gain distributed along transmission fiber → better OSNR.
**Atom or composite:** Composite (high-power pump + SRS gain).
**Real wall?** Yes — high pump power; double Rayleigh backscatter noise.
**Cross-domain wiring:** Same Raman gain as in molecular spectroscopy.
**Notes:** Islam 2002.

### long-period-grating (cross-domain alias: `LPG`, `mode-coupling-grating`)
**Domain:** Photonics / Optics
**Definition:** Periodic perturbation with Λ ~ 100s µm couples core to cladding modes; broadband loss notches.
**Atom or composite:** Composite (UV/CO₂/electric-arc induced).
**Real wall?** Yes — temperature/strain sensitive (used as sensor).
**Cross-domain wiring:** Same coupled-mode theory as FBG but for co-propagating modes.
**Notes:** Vengsarkar 1996.

### pm-fiber (cross-domain alias: `polarization-maintaining-fiber`, `PANDA`)
**Domain:** Photonics / Optics
**Definition:** Stress rods induce birefringence so two orthogonal polarizations propagate without coupling.
**Atom or composite:** Composite (stress-induced birefringence).
**Real wall?** Yes — extinction ratio ~25 dB; degrades with bending.
**Cross-domain wiring:** Polarization preservation key for quantum-computing photonic states.
**Notes:** Noda 1986.

### fiber-coupler (cross-domain alias: `fused-coupler`, `2x2-BS`)
**Domain:** Photonics / Optics
**Definition:** Two fibers fused so evanescent fields couple — variable splitting ratio by pull length.
**Atom or composite:** Composite (coupled-mode tapered fibers).
**Cost model:** Excess loss <0.1 dB.
**Real wall?** Yes — wavelength-dependent split.
**Cross-domain wiring:** Same coupled-mode physics as integrated directional coupler.
**Notes:** Snyder & Love §29.

### fiber-splice (cross-domain alias: `fusion-splice`, `arc-splice`)
**Domain:** Photonics / Optics
**Definition:** Two fibers fused with electric arc, forming permanent low-loss (<0.05 dB) joint.
**Atom or composite:** Composite (alignment + arc + tension).
**Real wall?** Yes — mode-field mismatch loss for dissimilar fibers.
**Cross-domain wiring:** Joining primitive — analog of weld/solder in physical systems.
**Notes:** Fujikura/Sumitomo tech notes.

### fc-pc-connector (cross-domain alias: `FC/PC`, `screw-on-connector`)
**Domain:** Photonics / Optics
**Definition:** Threaded ferrule connector with physical contact (flat polish); back-reflection ~ −40 dB.
**Atom or composite:** Composite (ferrule + alignment sleeve).
**Real wall?** Yes — limited reflection performance; replaced by APC for high-perf.
**Cross-domain wiring:** Mechanical mating standard — analog of RF connector in signal-processing-rf.
**Notes:** Telcordia GR-326.

### fc-apc-connector (cross-domain alias: `FC/APC`, `8°-polish`)
**Domain:** Photonics / Optics
**Definition:** Same as FC/PC but ferrule polished at 8°; back-reflection < −60 dB.
**Atom or composite:** Composite.
**Real wall?** Yes — requires angle-matched mate.
**Cross-domain wiring:** Angled-polish trick — same as RF mismatch dump.
**Notes:** EIA-604-30.

### sc-connector (cross-domain alias: `SC`, `push-pull-connector`)
**Domain:** Photonics / Optics
**Definition:** Square push-pull connector; data-center standard before LC.
**Atom or composite:** Composite.
**Cross-domain wiring:** Connector ecosystem analog to USB-family in networking hardware.
**Notes:** IEC 61754-4.

### lc-connector (cross-domain alias: `LC`, `small-form-factor-connector`)
**Domain:** Photonics / Optics
**Definition:** Half-size SC connector; 1.25 mm ferrule; dominant in QSFP/SFP transceivers.
**Atom or composite:** Composite.
**Real wall?** Yes — ferrule size limits insertion-loss budget.
**Cross-domain wiring:** Mass-deployment connector in networking optical front panels.
**Notes:** IEC 61754-20.

### st-connector (cross-domain alias: `ST`, `bayonet-connector`)
**Domain:** Photonics / Optics
**Definition:** Bayonet-style 2.5 mm ferrule connector; legacy multimode networks.
**Atom or composite:** Composite.
**Real wall?** No.
**Cross-domain wiring:** Same legacy-vs-modern as BNC in signal-processing-rf.
**Notes:** IEC 61754-2.

---

## Photonic Devices

### directional-coupler (cross-domain alias: `DC`, `2x2-evanescent-coupler`)
**Domain:** Photonics / Optics
**Definition:** Two parallel waveguides with overlapping evanescent fields; power oscillates with length: P_cross = sin²(κL).
**Atom or composite:** Composite (coupled-mode theory).
**Real wall?** Yes — κ wavelength dependent.
**Cross-domain wiring:** Same coupled-LC behavior as signal-processing-rf microstrip coupler; quantum-computing beam-splitter equivalent.
**Notes:** Yariv §13.

### mmi (cross-domain alias: `multi-mode-interference`, `self-imaging`)
**Domain:** Photonics / Optics
**Definition:** Wide multimode waveguide produces N-fold self-images by Talbot-effect; compact 1×N or N×N splitter.
**Atom or composite:** Composite (multimode WG + Talbot).
**Cost model:** Length L_π ∝ W²/λ.
**Real wall?** Yes — fabrication width tolerance.
**Cross-domain wiring:** Same Talbot self-imaging as Fresnel diffraction.
**Notes:** Soldano & Pennings 1995.

### awg (cross-domain alias: `arrayed-waveguide-grating`, `phasar`)
**Domain:** Photonics / Optics
**Definition:** Array of waveguides of incrementing length acts as integrated grating; demultiplexer/multiplexer for DWDM.
**Atom or composite:** Composite (input/output FPRs + array).
**Real wall?** Yes — phase errors limit crosstalk.
**Cross-domain wiring:** Same algebra as electromagnetics-antennas planar phased array.
**Notes:** Smit 1996.

### ring-resonator (cross-domain alias: `ring-cavity`, `microring`)
**Domain:** Photonics / Optics
**Definition:** Closed waveguide loop with FSR = c/(n_g·L); resonances when phase = 2πq; coupled to bus by DC.
**Atom or composite:** Composite (ring + bus + coupling).
**Cost model:** Q × FSR product.
**Real wall?** Yes — bend-loss vs footprint.
**Cross-domain wiring:** Same as signal-processing-rf high-Q resonator; quantum-computing cavity-QED ring.
**Notes:** Bogaerts 2012.

### mzi-modulator (cross-domain alias: `MZM`, `Mach-Zehnder-modulator`)
**Domain:** Photonics / Optics
**Definition:** Electro-optic phase shifters in MZI arms; voltage modulates phase → amplitude at output.
**Atom or composite:** Composite (MZI + EO phase shifters).
**Real wall?** Yes — V_π·L tradeoff; chirp.
**Cross-domain wiring:** Workhorse of networking coherent transceivers; same as signal-processing-rf I/Q modulator.
**Notes:** LiNbO₃ modulators canonical.

### eo-pockels (cross-domain alias: `Pockels-cell`, `electro-optic-modulator`)
**Domain:** Photonics / Optics
**Definition:** Linear EO effect: Δn = n³ r E / 2 in non-centrosymmetric crystal; voltage-controlled retardance.
**Atom or composite:** Atom (χ⁽²⁾ in DC limit).
**Real wall?** Yes — V_π ∝ d/(L·r); piezoelectric resonances limit bandwidth.
**Cross-domain wiring:** Same as quantum-computing fast-switching of light; signal-processing-rf phase modulator.
**Notes:** Boyd §11.

### aom (cross-domain alias: `acousto-optic-modulator`, `Bragg-cell`)
**Domain:** Photonics / Optics
**Definition:** RF acoustic wave creates moving Bragg grating in crystal; diffracts light with Doppler shift = RF.
**Atom or composite:** Composite (transducer + crystal).
**Real wall?** Yes — rise time ≈ beam transit time across acoustic wave.
**Cross-domain wiring:** Tied directly to signal-processing-rf — RF in, optics out.
**Notes:** Klein-Cook parameter.

### lcd-pol (cross-domain alias: `liquid-crystal-display`, `LCD-light-valve`)
**Domain:** Photonics / Optics
**Definition:** Polarized light passes liquid-crystal layer whose orientation controls retardance; addresses each pixel.
**Atom or composite:** Composite (LC + pol + TFT).
**Real wall?** Yes — switching speed ms; contrast limited by polarizer.
**Cross-domain wiring:** Display analog of SLM; ml-training optical computing.
**Notes:** TFT-LCD standards.

### lcos (cross-domain alias: `liquid-crystal-on-silicon`, `reflective-LC-SLM`)
**Domain:** Photonics / Optics
**Definition:** LC over CMOS backplane; phase-only SLM with µm pixels at HD resolution.
**Atom or composite:** Composite (LC + Si backplane + reflection).
**Real wall?** Yes — ~kHz update; 8-bit phase quantization.
**Cross-domain wiring:** Hardware substrate for CGH, optical-tweezers.
**Notes:** Hamamatsu/Holoeye LCoS modules.

### dmd (cross-domain alias: `digital-micromirror-device`, `DLP-chip`)
**Domain:** Photonics / Optics
**Definition:** MEMS array of tilting mirrors; binary amplitude modulator at kHz frame rates.
**Atom or composite:** Composite (micromirrors + Yoke + memory).
**Real wall?** Yes — binary; gray levels via PWM.
**Cross-domain wiring:** Same workhorse as ml-training optical neural net binary masks.
**Notes:** Hornbeck 1996.

### slm (cross-domain alias: `spatial-light-modulator`, `programmable-mask`)
**Domain:** Photonics / Optics
**Definition:** Pixel-addressable amplitude/phase modulator (LCoS, DMD, MEMS); programs wavefront.
**Atom or composite:** Composite (pixel array + driver).
**Cost model:** Pixel × refresh-rate.
**Real wall?** Yes — space-bandwidth product fixed by pixel count.
**Cross-domain wiring:** Programmable optical kernel — analog of FPGA in signal-processing-rf.
**Notes:** Workhorse of holography, AO, optical tweezers.

---

## Integrated Photonics

### silicon-photonics (cross-domain alias: `SiPh`, `SOI-photonics`)
**Domain:** Photonics / Optics
**Definition:** Photonic devices in Si on insulator wafers; index contrast 3.48/1.45 enables µm-scale bends.
**Atom or composite:** Composite (Si waveguide + Ge detector + heater).
**Real wall?** Yes — no light emission natively (indirect bandgap); hybrid laser integration.
**Cross-domain wiring:** Reuses CMOS foundry processes — ml-training accelerator integration; networking 400G/800G transceivers.
**Notes:** Soref 2006.

### silicon-nitride (cross-domain alias: `SiN`, `low-loss-photonics`)
**Domain:** Photonics / Optics
**Definition:** Si₃N₄ waveguides; broadband transparency (visible–NIR), ultralow loss (<0.1 dB/m).
**Atom or composite:** Composite (SiN + SiO₂ cladding).
**Real wall?** Yes — lower index contrast → larger bends.
**Cross-domain wiring:** Key for quantum-computing photonic interconnect; LIGO-grade resonators.
**Notes:** Bauters 2011.

### tfln (cross-domain alias: `thin-film-lithium-niobate`, `LNOI`)
**Domain:** Photonics / Optics
**Definition:** Sub-µm LiNbO₃ films on insulator; sharp bends and strong χ⁽²⁾ in same platform.
**Atom or composite:** Composite (smart-cut + dry etch).
**Real wall?** Yes — sidewall roughness; etching slow.
**Cross-domain wiring:** Combines QPM + high-speed MZM; powers quantum-computing photonic entanglement sources.
**Notes:** Zhang 2017.

### inp-photonics (cross-domain alias: `InP-PIC`, `monolithic-laser-platform`)
**Domain:** Photonics / Optics
**Definition:** InP wafers integrate lasers, modulators, detectors monolithically at 1550 nm.
**Atom or composite:** Composite (multi-quantum-well structures + WG).
**Real wall?** Yes — costlier than Si; smaller wafers.
**Cross-domain wiring:** Standard for networking long-haul transceivers; coherent transmitters.
**Notes:** Smit 2014.

### polymer-waveguide (cross-domain alias: `polymer-PIC`, `low-cost-WG`)
**Domain:** Photonics / Optics
**Definition:** Polymer (e.g., SU-8, OrmoComp) low-index waveguides; cheap, flexible, but lossy.
**Atom or composite:** Composite (polymer + cladding).
**Real wall?** Yes — thermal sensitivity; aging.
**Cross-domain wiring:** Used for board-level optical interconnects.
**Notes:** Eldada 2001.

### foundry-pdk (cross-domain alias: `process-design-kit`, `PDK`)
**Domain:** Photonics / Optics
**Definition:** Library of qualified components (WGs, splitters, modulators, detectors) provided by foundry for chip design.
**Atom or composite:** Composite (mask layouts + simulation models).
**Real wall?** Yes — proprietary; constrained by foundry process.
**Cross-domain wiring:** Direct analog of CMOS PDK in ml-training accelerator design.
**Notes:** IMEC/AIM Photonics PDKs.

### edge-coupler (cross-domain alias: `inverted-taper`, `spot-size-converter`)
**Domain:** Photonics / Optics
**Definition:** Tapered waveguide expands mode to match fiber MFD at chip edge; coupling loss ~1 dB.
**Atom or composite:** Composite (taper + cladding mode).
**Real wall?** Yes — polarization-sensitive; alignment-critical.
**Cross-domain wiring:** Bridge between waveguide and free-space/fiber world.
**Notes:** Almeida 2003.

### grating-coupler (cross-domain alias: `surface-grating-coupler`, `vertical-coupler`)
**Domain:** Photonics / Optics
**Definition:** Grating on chip diffracts light vertically into fiber; convenient wafer-level probing.
**Atom or composite:** Composite (grating period × duty cycle).
**Real wall?** Yes — narrowband (~30 nm); polarization-sensitive.
**Cross-domain wiring:** Same Bragg-coupling as VBG, fiber Bragg grating.
**Notes:** Taillaert 2004.

### wdm (cross-domain alias: `wavelength-division-multiplexing`)
**Domain:** Photonics / Optics
**Definition:** Multiple wavelengths share one fiber; each channel carries independent data.
**Atom or composite:** Composite (mux + demux + parallel transceivers).
**Cost model:** Spectral efficiency × channel count.
**Real wall?** Yes — fiber dispersion + nonlinearity (Kerr) limits ultimate capacity (Shannon-Kerr).
**Cross-domain wiring:** Direct optical analog of signal-processing-rf FDM; networking backbone.
**Notes:** ITU-T G.694.

### dwdm (cross-domain alias: `dense-WDM`, `100GHz-grid`)
**Domain:** Photonics / Optics
**Definition:** WDM on 50/100 GHz ITU grid in C/L bands; ~80–160 channels per fiber.
**Atom or composite:** Composite (DFB array + AWG + EDFA chain).
**Real wall?** Yes — channel spacing × symbol rate constrained by Kerr.
**Cross-domain wiring:** Same as cellular sub-channelization in signal-processing-rf.
**Notes:** ITU-T G.694.1.

### cwdm (cross-domain alias: `coarse-WDM`, `20nm-grid`)
**Domain:** Photonics / Optics
**Definition:** WDM on 20 nm spacing (1271–1611 nm); uses uncooled DFBs.
**Atom or composite:** Composite.
**Real wall?** Yes — chromatic dispersion across wide band.
**Cross-domain wiring:** Low-cost WDM in datacenter optics.
**Notes:** ITU-T G.694.2.

### pic (cross-domain alias: `photonic-integrated-circuit`)
**Domain:** Photonics / Optics
**Definition:** Multiple optical functions integrated on a single chip; analog of electronic IC.
**Atom or composite:** Composite (many photonic primitives integrated).
**Cost model:** Wafer-scale fab economics.
**Real wall?** Yes — yield × thermal management.
**Cross-domain wiring:** Hardware foundation for ml-training photonic accelerators, quantum-computing photonic processors.
**Notes:** Smit 2019.

---

## Quantum Optics

### photon-fock-state (cross-domain alias: `|n⟩`, `number-state`)
**Domain:** Photonics / Optics
**Definition:** Eigenstate of photon number operator: â†â|n⟩ = n|n⟩; minimum-uncertainty in n, maximum in phase.
**Atom or composite:** Atom (basis state of bosonic mode).
**Real wall?** Yes — large-n Fock states fragile to loss.
**Cross-domain wiring:** Same as quantum-computing bosonic-code basis; physics-quantum harmonic-oscillator eigenstate.
**Notes:** Loudon §4.

### coherent-state-optical (cross-domain alias: `|α⟩`, `Glauber-state`)
**Domain:** Photonics / Optics
**Definition:** Eigenstate of annihilation operator: â|α⟩ = α|α⟩; classical-like quantum state of light from lasers.
**Atom or composite:** Atom (displaced vacuum).
**Real wall?** No — closest quantum state to a classical field.
**Cross-domain wiring:** Same as Gaussian state in quantum-computing CV; reference state for homodyne.
**Notes:** Glauber 1963.

### squeezed-state-optical (cross-domain alias: `squeezed-vacuum`, `quadrature-squeezed`)
**Domain:** Photonics / Optics
**Definition:** Gaussian state with quadrature variance reduced below vacuum at cost of orthogonal quadrature.
**Atom or composite:** Composite (squeezing operator S(r) on vacuum).
**Real wall?** Yes — finite squeezing in dB; loss kills squeezing exponentially.
**Cross-domain wiring:** Used in LIGO; same as quantum-computing CV squeezing for fault tolerance.
**Notes:** Caves 1981.

### wigner-function-optical (cross-domain alias: `W(x,p)`, `phase-space-rep`)
**Domain:** Photonics / Optics
**Definition:** Quasi-probability distribution in (x,p) optical phase space; negative regions ↔ nonclassicality.
**Atom or composite:** Composite (Wigner transform of density matrix).
**Cost model:** Reconstructed by homodyne tomography.
**Real wall?** Yes — negative-valued for nonclassical states; classical states always W ≥ 0.
**Cross-domain wiring:** Same Wigner function as physics-quantum; signal-processing-rf time-frequency distribution.
**Notes:** Smithey 1993.

### sfwm (cross-domain alias: `spontaneous-four-wave-mixing`, `SFWM`)
**Domain:** Photonics / Optics
**Definition:** χ⁽³⁾ process: 2 pump photons → signal+idler pair; used in Si/SiN waveguides where no χ⁽²⁾.
**Atom or composite:** Composite (4-wave + vacuum seed).
**Real wall?** Yes — Raman noise in glass fibers contaminates signal.
**Cross-domain wiring:** Source for quantum-computing on-chip entanglement.
**Notes:** Lin 2006.

### heralded-single-photon (cross-domain alias: `HSP`, `heralded-photon`)
**Domain:** Photonics / Optics
**Definition:** Detection of one photon in PDC/SFWM heralds presence of its twin; non-deterministic single-photon source.
**Atom or composite:** Composite (PDC + herald detector + gating).
**Real wall?** Yes — heralding efficiency × multiphoton contamination at high pump.
**Cross-domain wiring:** Core resource for quantum-computing linear-optical QC.
**Notes:** Hong & Mandel 1986.

### photon-pair-source (cross-domain alias: `entangled-pair-source`, `PPS`)
**Domain:** Photonics / Optics
**Definition:** Source producing time/polarization/energy-entangled photon pairs (PDC, SFWM, QDs).
**Atom or composite:** Composite (nonlinear medium + spectral/polarization filtering).
**Cost model:** Pair generation rate × heralding efficiency × indistinguishability.
**Real wall?** Yes — tradeoff between brightness and purity.
**Cross-domain wiring:** Backbone of quantum-computing networks and QKD.
**Notes:** Kwiat 1995.

### hom-effect (cross-domain alias: `Hong-Ou-Mandel`, `two-photon-dip`)
**Domain:** Photonics / Optics
**Definition:** Two identical photons entering a 50:50 BS exit together (bunching) — coincidence count dips to zero.
**Atom or composite:** Composite (BS + indistinguishable photons).
**Real wall?** Yes — depth of dip = photon indistinguishability ≤ 1.
**Cross-domain wiring:** Test of bosonic statistics; quantum-computing photonic gate building block.
**Notes:** Hong, Ou, Mandel 1987.

### two-photon-interference (cross-domain alias: `2PI`, `nonclassical-interference`)
**Domain:** Photonics / Optics
**Definition:** Interference of two-photon probability amplitudes; visibility > 50% violates classical bound.
**Atom or composite:** Composite (multi-path two-photon amplitude).
**Real wall?** Yes — distinguishability degrades visibility.
**Cross-domain wiring:** Same multi-particle interference as quantum-computing boson sampling.
**Notes:** Mandel 1999.

### polarization-entanglement (cross-domain alias: `Bell-pair-photonic`)
**Domain:** Photonics / Optics
**Definition:** State (|HH⟩+|VV⟩)/√2 in two photons; measured by polarizer pairs.
**Atom or composite:** Composite (Type-II PDC or post-selection).
**Real wall?** Yes — birefringent walk-off; PMD limits transport.
**Cross-domain wiring:** Same Bell-state as quantum-computing teleportation resource.
**Notes:** Kwiat 1995.

### time-bin-entanglement (cross-domain alias: `time-bin-Bell`, `Franson-IF`)
**Domain:** Photonics / Optics
**Definition:** Encoding in early/late time bins: (|EE⟩+|LL⟩)/√2; robust over fibers.
**Atom or composite:** Composite (PDC + unbalanced MZI + delay).
**Real wall?** Yes — coherence between bins limited by MZI stability.
**Cross-domain wiring:** Same encoding as quantum-computing chronologically encoded states; favored for long-distance QKD.
**Notes:** Brendel 1999.

### oam-entanglement (cross-domain alias: `OAM-Bell`)
**Domain:** Photonics / Optics
**Definition:** Entanglement in OAM degree of freedom: Σ c_l |l,−l⟩; high-dim states accessible.
**Atom or composite:** Composite (PDC + OAM-mode sorter).
**Real wall?** Yes — atmospheric scattering scrambles OAM modes.
**Cross-domain wiring:** High-dimensional quantum-computing communication; analog of OAM-multiplexed networking.
**Notes:** Mair 2001.

### qst-optical (cross-domain alias: `quantum-state-tomography`, `optical-tomography`)
**Domain:** Photonics / Optics
**Definition:** Reconstruct ρ from measurements in informationally-complete basis (e.g., Pauli for qubit, MUBs for qudits).
**Atom or composite:** Composite (measurements + maximum-likelihood reconstruction).
**Cost model:** Measurements ~ d² for d-dim state.
**Real wall?** Yes — finite samples; statistical error.
**Cross-domain wiring:** Same algorithm as quantum-computing process tomography; statistics-probability MLE.
**Notes:** James 2001.

### homodyne-tomography (cross-domain alias: `quadrature-tomography`)
**Domain:** Photonics / Optics
**Definition:** Histogram of homodyne-measured quadratures at varying LO phase → reconstruct Wigner function via inverse Radon.
**Atom or composite:** Composite (homodyne + Radon inversion).
**Cost model:** N_quadrature × N_phase samples.
**Real wall?** Yes — finite efficiency degrades reconstructed W.
**Cross-domain wiring:** Same Radon transform as graphics-rendering-lod CT; signal-processing-rf inverse problems.
**Notes:** Smithey 1993.

---

## Plasmonics

### spp (cross-domain alias: `surface-plasmon-polariton`, `SPP`)
**Domain:** Photonics / Optics
**Definition:** Coupled surface mode of EM field and electron oscillation at metal-dielectric interface; k_spp > k_0.
**Atom or composite:** Composite (Maxwell + Drude metal).
**Real wall?** Yes — Ohmic loss in metal exponentially decays propagation (~µm in visible).
**Cross-domain wiring:** Same surface-wave physics as electromagnetics-antennas Zenneck waves.
**Notes:** Maier 2007.

### lspr (cross-domain alias: `localized-surface-plasmon-resonance`)
**Domain:** Photonics / Optics
**Definition:** Resonance of conduction electrons in metallic nanoparticle; geometry-dependent λ_res; large near-field enhancement.
**Atom or composite:** Composite (Mie/quasi-static analysis).
**Real wall?** Yes — radiative + Ohmic damping caps Q ~ 10–100.
**Cross-domain wiring:** Same dipole-resonance as electromagnetics-antennas dielectric resonator; sensor in chemistry.
**Notes:** Bohren & Huffman 1983.

### mie-resonance (cross-domain alias: `Mie-scattering`, `dielectric-sphere-resonance`)
**Domain:** Photonics / Optics
**Definition:** Exact analytic solution for EM scattering by sphere; resonances at size-dependent λ.
**Atom or composite:** Composite (spherical harmonic series).
**Cost model:** Truncated series; O(ka).
**Real wall?** No — exact within Maxwell assumptions.
**Cross-domain wiring:** Same as electromagnetics-antennas radome scattering; basis for dielectric metasurfaces.
**Notes:** Mie 1908.

### near-field-optics (cross-domain alias: `NSOM`, `evanescent-imaging`)
**Domain:** Photonics / Optics
**Definition:** Imaging using evanescent (near-field) components beyond diffraction limit; tip-sample distance ≪ λ.
**Atom or composite:** Composite (nano-tip + scan).
**Real wall?** Yes — exponential decay z ~ λ/10; mechanical scan slow.
**Cross-domain wiring:** Same evanescent coupling as integrated optics; analog of STM in physics-quantum.
**Notes:** Pohl 1984.

### plasmonic-waveguide (cross-domain alias: `metal-WG`, `SPP-guide`)
**Domain:** Photonics / Optics
**Definition:** Waveguide using SPPs (stripe, channel, MIM); sub-diffraction mode confinement.
**Atom or composite:** Composite (metal-dielectric structure).
**Real wall?** Yes — propagation length × mode confinement is fundamentally constrained.
**Cross-domain wiring:** Same trade-off as electromagnetics-antennas leaky waveguide.
**Notes:** Maier §3.

### mim-waveguide (cross-domain alias: `metal-insulator-metal`, `MIM`)
**Domain:** Photonics / Optics
**Definition:** Plasmonic slot waveguide between two metal films; deep sub-λ confinement, high loss.
**Atom or composite:** Composite (gap-plasmon mode).
**Real wall?** Yes — propagation ~ µm.
**Cross-domain wiring:** Used in plasmonic nanocavities; analog of electromagnetics-antennas parallel-plate WG.
**Notes:** Bozhevolnyi 2008.

### prism-spp-coupling (cross-domain alias: `Kretschmann`, `Otto-coupling`)
**Domain:** Photonics / Optics
**Definition:** Coupling free-space light to SPP via prism providing momentum match; ATR configuration.
**Atom or composite:** Composite (prism + thin metal film).
**Real wall?** Yes — only at one angle/λ.
**Cross-domain wiring:** Same momentum-matching as electromagnetics-antennas leaky-wave excitation.
**Notes:** Kretschmann 1971.

### grating-spp-coupling (cross-domain alias: `grating-coupler-SPP`)
**Domain:** Photonics / Optics
**Definition:** Periodic grating provides reciprocal-lattice momentum for free-space-to-SPP coupling: k_∥ = k_0 sin θ + mG.
**Atom or composite:** Composite (metal grating).
**Real wall?** Yes — bandwidth × efficiency.
**Cross-domain wiring:** Same as photonic-crystal grating coupling.
**Notes:** Maier §3.4.

### nanoantenna (cross-domain alias: `optical-antenna`, `plasmonic-antenna`)
**Domain:** Photonics / Optics
**Definition:** Metallic nanostructure that couples free-space radiation to nanoscale near fields; analog of RF antenna at optical.
**Atom or composite:** Composite (resonant metal element).
**Real wall?** Yes — Ohmic loss caps efficiency.
**Cross-domain wiring:** Direct optical analog of electromagnetics-antennas dipole/Yagi-Uda; ml-training optical interconnect.
**Notes:** Bharadwaj 2009.

### sers (cross-domain alias: `surface-enhanced-Raman`, `plasmonic-SERS`)
**Domain:** Photonics / Optics
**Definition:** Raman signal enhanced by 10⁶–10¹¹ at plasmonic hot spots due to field enhancement |E|⁴ scaling.
**Atom or composite:** Composite (Raman scattering + LSPR).
**Real wall?** Yes — single-molecule SERS achievable; spot-to-spot variability.
**Cross-domain wiring:** Sensing analog of biology-bioinformatics single-molecule detection.
**Notes:** Fleischmann 1974.

---

## Metamaterials and Metasurfaces

### negative-index (cross-domain alias: `NIM`, `n<0-medium`)
**Domain:** Photonics / Optics
**Definition:** Material with simultaneous ε<0 and µ<0 → n<0; reversed Snell's law, backward phase velocity.
**Atom or composite:** Composite (engineered unit cell + sub-λ periodicity).
**Real wall?** Yes — losses fundamental in resonant designs.
**Cross-domain wiring:** Same engineered-medium concept as electromagnetics-antennas FSS.
**Notes:** Veselago 1968; Pendry 2000.

### hyperbolic-metamaterial (cross-domain alias: `HMM`, `indefinite-medium`)
**Domain:** Photonics / Optics
**Definition:** Anisotropic medium with ε_∥·ε_⊥ < 0 → hyperbolic iso-frequency surface; supports very large k modes.
**Atom or composite:** Composite (metal-dielectric multilayer or nanowire array).
**Real wall?** Yes — non-locality at large k; loss.
**Cross-domain wiring:** Same anisotropic-tensor algebra as electromagnetics-antennas uniaxial substrates.
**Notes:** Smolyaninov 2013.

### metalens (cross-domain alias: `flat-lens`, `Pancharatnam-Berry-lens`)
**Domain:** Photonics / Optics
**Definition:** Flat array of sub-wavelength nanoposts imparting position-dependent phase to focus light; t hickness ~ λ.
**Atom or composite:** Composite (phase map + nanoscatterers).
**Cost model:** Lithography limited.
**Real wall?** Yes — chromatic aberration severe in single-element metalens.
**Cross-domain wiring:** Same as electromagnetics-antennas reflectarray; planar replacement of refractive lens.
**Notes:** Khorasaninejad 2016.

### pancharatnam-berry-phase (cross-domain alias: `PB-phase`, `geometric-phase`)
**Domain:** Photonics / Optics
**Definition:** Phase Φ = 2θ imparted by rotated half-wave plate (or nanopost); independent of dynamic phase.
**Atom or composite:** Atom (geometric phase from polarization path on Poincaré).
**Real wall?** No.
**Cross-domain wiring:** Same Berry phase as quantum-computing geometric gates; physics-quantum adiabatic phase.
**Notes:** Pancharatnam 1956; Berry 1984.

### metasurface-polarization (cross-domain alias: `meta-polarizer`, `Jones-metasurface`)
**Domain:** Photonics / Optics
**Definition:** Metasurface with anisotropic unit cells implementing arbitrary Jones matrix at each pixel.
**Atom or composite:** Composite (anisotropic nanopost array).
**Real wall?** Yes — bandwidth limited; design complexity grows quadratically.
**Cross-domain wiring:** Same as polarization-aberration correction; quantum-computing photonic gate.
**Notes:** Arbabi 2015.

### holographic-metasurface (cross-domain alias: `meta-hologram`)
**Domain:** Photonics / Optics
**Definition:** Metasurface with phase map = CGH for desired far-field; flat, high-efficiency hologram.
**Atom or composite:** Composite (metasurface + Gerchberg-Saxton design).
**Real wall?** Yes — pixel pitch limits angular range; chromatic.
**Cross-domain wiring:** Replaces SLM in static applications; ml-training optical neural net.
**Notes:** Zheng 2015.

### achromatic-metalens (cross-domain alias: `broadband-metalens`)
**Domain:** Photonics / Optics
**Definition:** Metalens engineered for constant focal length across visible by dispersion-engineered nano-elements.
**Atom or composite:** Composite (broadband Pancharatnam-Berry + dispersive nanopost).
**Real wall?** Yes — efficiency × bandwidth × NA tradeoff.
**Cross-domain wiring:** Same broadband-matching as electromagnetics-antennas ultra-wideband design.
**Notes:** Wang 2018.

---

## Imaging and Microscopy

### confocal-microscopy (cross-domain alias: `confocal`, `pinhole-rejection`)
**Domain:** Photonics / Optics
**Definition:** Pinhole at conjugate image plane rejects out-of-focus light; 3D sectioning at diffraction limit.
**Atom or composite:** Composite (scan + pinhole + PMT).
**Real wall?** Yes — Abbe diffraction limit Δx = 0.61 λ/NA.
**Cross-domain wiring:** Same spatial-filter as 4f system; signal-processing-rf gated detection.
**Notes:** Minsky 1957.

### sted (cross-domain alias: `stimulated-emission-depletion`, `Hell-STED`)
**Domain:** Photonics / Optics
**Definition:** Donut-shaped STED beam depletes excited state outside center → sub-diffraction PSF, resolution ~ λ/(2 NA √(1+I/I_s)).
**Atom or composite:** Composite (excitation + depletion + scan).
**Real wall?** Yes — photodamage from intense depletion beam.
**Cross-domain wiring:** Same nonlinear-narrowing as nonlinear electromagnetics; quantum-computing dark-state preparation.
**Notes:** Hell 1994 Nobel.

### storm-palm (cross-domain alias: `single-molecule-localization-microscopy`, `SMLM`)
**Domain:** Photonics / Optics
**Definition:** Stochastically activate sparse subsets of fluorophores; localize centroids → super-resolved image.
**Atom or composite:** Composite (blinking + centroid fit + accumulate).
**Real wall?** Yes — localization precision ~ σ/√N photons.
**Cross-domain wiring:** Same centroiding algorithm as Shack-Hartmann; statistics-probability MLE.
**Notes:** Betzig 2006 Nobel.

### minflux (cross-domain alias: `MINFLUX`, `min-emission-flux-localization`)
**Domain:** Photonics / Optics
**Definition:** Localize single emitter using a structured donut beam at multiple positions; nm precision with ~100 photons.
**Atom or composite:** Composite (donut + MLE).
**Real wall?** Yes — pointing accuracy limit; reaches molecular scale.
**Cross-domain wiring:** Information-efficient localization — same as ml-training active-learning.
**Notes:** Balzarotti 2017.

### spim (cross-domain alias: `light-sheet-microscopy`, `SPIM`)
**Domain:** Photonics / Optics
**Definition:** Illuminate single plane with thin light sheet; orthogonal detection — fast 3D, low photo-damage.
**Atom or composite:** Composite (cylindrical lens + camera + scan).
**Real wall?** Yes — sheet thickness limits axial resolution.
**Cross-domain wiring:** Tomographic acquisition similar to graphics-rendering-lod slice-renderer.
**Notes:** Huisken 2004.

### two-photon-microscopy (cross-domain alias: `2PEF`, `multiphoton-microscopy`)
**Domain:** Photonics / Optics
**Definition:** Excitation by two-photon absorption (∝ I²); intrinsic sectioning, deep tissue penetration in NIR.
**Atom or composite:** Composite (fs pulse + tight focus).
**Real wall?** Yes — limited by scattering at depth; photon budget.
**Cross-domain wiring:** Same nonlinear excitation as nonlinear optics; signal-processing-rf squared envelope detection.
**Notes:** Denk 1990.

### multiphoton-microscopy (cross-domain alias: `3PEF`, `n-photon-imaging`)
**Domain:** Photonics / Optics
**Definition:** 3+ photon absorption; deeper-than-2P penetration (lower scattering at longer λ).
**Atom or composite:** Composite (high-intensity NIR pulse + tight focus).
**Real wall?** Yes — even more demanding photon flux.
**Cross-domain wiring:** Higher-order nonlinear cross section; same I^n scaling.
**Notes:** Horton 2013.

### shg-imaging (cross-domain alias: `second-harmonic-microscopy`)
**Domain:** Photonics / Optics
**Definition:** Image based on intrinsic χ⁽²⁾ contrast (collagen, microtubules); label-free.
**Atom or composite:** Composite (SHG + microscopy).
**Real wall?** Yes — non-centrosymmetric structures required.
**Cross-domain wiring:** Same χ⁽²⁾ physics as chi2-shg; nonlinear optics in life sciences.
**Notes:** Campagnola 2003.

### cars (cross-domain alias: `coherent-anti-Stokes-Raman`)
**Domain:** Photonics / Optics
**Definition:** Four-wave mixing produces anti-Stokes signal at 2ω_p − ω_s when ω_p − ω_s matches a vibrational mode.
**Atom or composite:** Composite (χ⁽³⁾ + dual-color pump).
**Real wall?** Yes — non-resonant background limits contrast.
**Cross-domain wiring:** Same vibrational fingerprint as Raman in spectroscopy.
**Notes:** Zumbusch 1999.

### flim (cross-domain alias: `fluorescence-lifetime-imaging`)
**Domain:** Photonics / Optics
**Definition:** Image based on excited-state lifetime τ, not intensity; reveals microenvironment, FRET.
**Atom or composite:** Composite (pulsed source + time-resolved detector + per-pixel fit).
**Real wall?** Yes — photon-count limited; complex fit.
**Cross-domain wiring:** Same exponential-decay fitting as physics-diffusion lifetime decay; ml-training inverse-problem.
**Notes:** Lakowicz §4.

### frap (cross-domain alias: `fluorescence-recovery-after-photobleaching`)
**Domain:** Photonics / Optics
**Definition:** Photobleach a region, observe diffusion-driven fluorescence recovery; extract diffusion coefficient.
**Atom or composite:** Composite (bleach + time series + diffusion fit).
**Real wall?** Yes — assumes 2D diffusion; complex geometries break model.
**Cross-domain wiring:** Same diffusion-coefficient extraction as physics-diffusion tracer.
**Notes:** Axelrod 1976.

### fret (cross-domain alias: `Förster-resonance-energy-transfer`)
**Domain:** Photonics / Optics
**Definition:** Non-radiative dipole-dipole energy transfer between fluorophores; rate ∝ 1/R⁶; molecular ruler 1–10 nm.
**Atom or composite:** Composite (dipole-dipole interaction).
**Real wall?** Yes — only sensitive in narrow R range.
**Cross-domain wiring:** Same dipole-dipole as physics-quantum atomic interactions; biology-bioinformatics protein FRET.
**Notes:** Förster 1948.

### sim (cross-domain alias: `structured-illumination-microscopy`)
**Domain:** Photonics / Optics
**Definition:** Project sinusoidal pattern onto sample; high-frequency information aliased into passband, then reconstructed → 2× resolution.
**Atom or composite:** Composite (illumination grating + algorithmic reconstruction).
**Real wall?** Yes — 2× factor; nonlinear SIM goes higher.
**Cross-domain wiring:** Same as signal-processing-rf bandpass downconversion + computational super-res.
**Notes:** Gustafsson 2000.

### expansion-microscopy (cross-domain alias: `ExM`)
**Domain:** Photonics / Optics
**Definition:** Embed sample in swellable polymer; physically expand sample 4×−20× → effective resolution beyond diffraction limit.
**Atom or composite:** Composite (sample preparation + conventional microscopy).
**Real wall?** No — limited by isotropy of expansion and label retention.
**Cross-domain wiring:** Physical transform — analog of computational-geometry mesh upsampling.
**Notes:** Chen 2015.

### adaptive-optics-im (cross-domain alias: `AO-microscopy`)
**Domain:** Photonics / Optics
**Definition:** Apply AO loop to correct aberrations in microscopy (especially deep tissue); restore diffraction-limited PSF.
**Atom or composite:** Composite (WFS + DM + closed loop).
**Real wall?** Yes — aberrations evolve with depth; finite DM actuators.
**Cross-domain wiring:** Same closed-loop control as adaptive-optics for astronomy.
**Notes:** Booth 2014.

---

## Adaptive Optics

### deformable-mirror (cross-domain alias: `DM`, `AO-mirror`)
**Domain:** Photonics / Optics
**Definition:** Mirror with array of actuators that locally deform surface to correct wavefront errors.
**Atom or composite:** Composite (continuous facesheet + actuator array).
**Cost model:** N actuators × control bandwidth.
**Real wall?** Yes — fitting error ∝ (d_actuator/r₀)^5/3.
**Cross-domain wiring:** Same actuator-grid control as control-numerical-opt distributed actuation.
**Notes:** Hardy 1998.

### wavefront-sensor-sh (cross-domain alias: `Shack-Hartmann-WFS`, `SH-WFS`)
**Domain:** Photonics / Optics
**Definition:** Lenslet array + centroiding sensor — measures local slopes of wavefront across pupil.
**Atom or composite:** Composite (lenslet + camera + centroid).
**Real wall?** Yes — finite lenslet count caps spatial resolution.
**Cross-domain wiring:** Same gradient-sampling as ml-training finite-difference gradient.
**Notes:** Platt & Shack 2001.

### wavefront-sensor-pyramid (cross-domain alias: `pyramid-WFS`)
**Domain:** Photonics / Optics
**Definition:** Refractive pyramid at focal plane splits wavefront into 4 pupil images; modulated for linearity.
**Atom or composite:** Composite (pyramid prism + pupil imaging).
**Real wall?** Yes — non-linear at large aberrations.
**Cross-domain wiring:** Foucault-test-array; same as quadrant-detector in signal-processing-rf.
**Notes:** Ragazzoni 1996.

### wavefront-sensor-curvature (cross-domain alias: `curvature-WFS`)
**Domain:** Photonics / Optics
**Definition:** Compare in-focus and out-of-focus images; ∇²W ∝ I_+ − I_−.
**Atom or composite:** Composite (defocus + Laplacian extract).
**Real wall?** Yes — sensitivity falls at zero curvature.
**Cross-domain wiring:** Same kind of focus-stack as light-field cameras.
**Notes:** Roddier 1988.

### strehl-metric (cross-domain alias: `S-based-control`)
**Domain:** Photonics / Optics
**Definition:** Use Strehl ratio as figure-of-merit in AO control loop; maximize via gradient or zonal control.
**Atom or composite:** Composite (PSF peak + reference).
**Real wall?** Yes — non-convex landscape for large aberrations.
**Cross-domain wiring:** Same scalar-cost optimization as control-numerical-opt.
**Notes:** Roddier 1999.

### ao-closed-loop (cross-domain alias: `AO-feedback`)
**Domain:** Photonics / Optics
**Definition:** WFS → reconstructor → DM → optical path → WFS; loop closes at kHz to track turbulence.
**Atom or composite:** Composite (control loop).
**Cost model:** Latency + sample rate vs turbulence timescale.
**Real wall?** Yes — finite control bandwidth (~kHz) vs atmospheric coherence time τ₀.
**Cross-domain wiring:** Same MIMO loop as control-numerical-opt; signal-processing-rf phased-array calibration.
**Notes:** Roddier 1999.

---

## Spectroscopy

### absorption-spectroscopy (cross-domain alias: `absorption-spec`, `Beer-Lambert`)
**Domain:** Photonics / Optics
**Definition:** Measure transmittance T(λ) = exp(−α(λ)L); extract concentration via Beer-Lambert α = ε c.
**Atom or composite:** Composite (broadband source + sample + spectrometer).
**Real wall?** Yes — limited by source noise and detector NEP.
**Cross-domain wiring:** Same Beer-Lambert as physics-diffusion radiative transfer.
**Notes:** Demtröder §6.

### emission-spectroscopy (cross-domain alias: `emission-spec`)
**Domain:** Photonics / Optics
**Definition:** Measure radiated light spectrum from excited atoms/molecules; identifies via line spectra.
**Atom or composite:** Composite (excitation + dispersive readout).
**Real wall?** Yes — Doppler/pressure broadening sets resolution.
**Cross-domain wiring:** Same Boltzmann line-strength as physics-quantum.
**Notes:** Kuhn 1969.

### fluorescence-spectroscopy (cross-domain alias: `PL-spec`)
**Domain:** Photonics / Optics
**Definition:** Excite at λ_ex, measure emission spectrum (Stokes-shifted); identifies fluorophores.
**Atom or composite:** Composite (excitation + emission filter + spectrometer).
**Real wall?** Yes — quantum yield × photobleaching.
**Cross-domain wiring:** Same kind of stimulated/spontaneous interplay as laser dynamics.
**Notes:** Lakowicz §1.

### raman-spectroscopy (cross-domain alias: `Raman-spec`)
**Domain:** Photonics / Optics
**Definition:** Inelastic scattering at ω − Ω_vib; vibrational fingerprint.
**Atom or composite:** Composite (laser + notch filter + spectrometer).
**Real wall?** Yes — cross section 10⁻³⁰ cm²; enhanced by SERS.
**Cross-domain wiring:** Same Raman process as fiber Raman amplifier; chemistry/biology fingerprinting.
**Notes:** Long 2002.

### brillouin-spectroscopy (cross-domain alias: `Brillouin-spec`)
**Domain:** Photonics / Optics
**Definition:** Inelastic scattering by acoustic phonons; GHz shifts; reports elastic moduli.
**Atom or composite:** Composite (laser + Tandem Fabry-Perot).
**Real wall?** Yes — narrow shift requires GHz spectrometers.
**Cross-domain wiring:** Same as biology-bioinformatics tissue stiffness mapping; signal-processing-rf acoustic Doppler.
**Notes:** Sandercock 1982.

### dispersive-spectrometer (cross-domain alias: `grating-spectrometer`)
**Domain:** Photonics / Optics
**Definition:** Slit + grating + detector array; resolving power R = mN.
**Atom or composite:** Composite (slit + dispersion + sensor).
**Cost model:** Tradeoff between R, throughput, and bandwidth.
**Real wall?** Yes — slit width × NA limit etendue.
**Cross-domain wiring:** Same as signal-processing-rf filter bank.
**Notes:** Czerny-Turner config.

### monochromator (cross-domain alias: `single-λ-selector`)
**Domain:** Photonics / Optics
**Definition:** Dispersive spectrometer with output slit selecting narrow band; tuned by rotating grating.
**Atom or composite:** Composite (spectrometer + output slit).
**Real wall?** Yes — slit width vs throughput.
**Cross-domain wiring:** Tunable bandpass filter — same role as signal-processing-rf tunable filter.
**Notes:** Demtröder §4.

### hyperspectral-imaging (cross-domain alias: `HSI`)
**Domain:** Photonics / Optics
**Definition:** Image cube (x, y, λ); each pixel has full spectrum.
**Atom or composite:** Composite (push-broom scan or tunable filter or snapshot).
**Cost model:** Data rate ∝ N_x × N_y × N_λ.
**Real wall?** Yes — light-budget vs noise vs spatial resolution.
**Cross-domain wiring:** Same data structure as ml-training spectral tensors; remote sensing in graphics-rendering-lod.
**Notes:** Goetz 1985.

---

## Lidar

### lidar-tof (cross-domain alias: `TOF-lidar`, `direct-time-of-flight`)
**Domain:** Photonics / Optics
**Definition:** Pulse laser, measure round-trip time t → range R = ct/2; ns timing → cm precision.
**Atom or composite:** Composite (pulsed laser + APD/SPAD + TDC).
**Real wall?** Yes — speed-of-light timing demands fast electronics.
**Cross-domain wiring:** Same as signal-processing-rf radar TOF; sonar acoustic TOF.
**Notes:** Wandinger 2005.

### lidar-amcw (cross-domain alias: `amplitude-modulated-CW-lidar`)
**Domain:** Photonics / Optics
**Definition:** Modulate laser amplitude at f_m; measure phase shift between transmit and receive → R = c·Δφ/(4π f_m).
**Atom or composite:** Composite (CW laser + AM modulator + phase-detector).
**Real wall?** Yes — range ambiguity at c/(2 f_m).
**Cross-domain wiring:** Same as signal-processing-rf phase-coded radar.
**Notes:** Kingston 1995.

### lidar-fmcw (cross-domain alias: `FMCW-lidar`, `coherent-chirp-lidar`)
**Domain:** Photonics / Optics
**Definition:** Chirped laser + coherent detection; beat frequency f_b = (B/T)·(2R/c); also velocity from Doppler.
**Atom or composite:** Composite (linear-chirp laser + coherent receiver).
**Real wall?** Yes — chirp linearity and laser coherence cap range × resolution.
**Cross-domain wiring:** Direct optical implementation of signal-processing-rf FMCW radar.
**Notes:** Behroozpour 2017.

### coherent-lidar (cross-domain alias: `Doppler-lidar`)
**Domain:** Photonics / Optics
**Definition:** Heterodyne detection of backscatter for Doppler velocity measurement.
**Atom or composite:** Composite (CW laser + heterodyne + spectrum analyzer).
**Real wall?** Yes — atmospheric coherence; speckle.
**Cross-domain wiring:** Same as signal-processing-rf Doppler radar.
**Notes:** Henderson 1991.

### sp-lidar (cross-domain alias: `single-photon-lidar`, `SPAD-array-lidar`)
**Domain:** Photonics / Optics
**Definition:** SPAD arrays + ns gating; sensitive to single-photon returns; eye-safe with low average power.
**Atom or composite:** Composite (SPAD array + gating + photon counting).
**Real wall?** Yes — pile-up at high return rates; dark count.
**Cross-domain wiring:** Same TCSPC principle as biological imaging FLIM.
**Notes:** Halimi 2017.

### flash-lidar (cross-domain alias: `staring-lidar`)
**Domain:** Photonics / Optics
**Definition:** Single pulse illuminates entire FOV; 2D TOF camera reads range per pixel.
**Atom or composite:** Composite (high-energy pulse + 2D TOF sensor).
**Real wall?** Yes — eye-safety limits at λ<1.4 µm; range × eye-safe-energy.
**Cross-domain wiring:** Same as ml-training depth-camera fusion; analog of radar imaging.
**Notes:** Stettner 2008.

### mems-lidar (cross-domain alias: `MEMS-scanning-lidar`)
**Domain:** Photonics / Optics
**Definition:** MEMS mirrors scan the laser across FOV; compact, no macroscopic moving parts.
**Atom or composite:** Composite (MEMS mirror + lidar engine).
**Real wall?** Yes — limited aperture and scan range.
**Cross-domain wiring:** Same scanning paradigm as galvo systems; signal-processing-rf phased-array steering.
**Notes:** Wang 2020.

### opa-lidar (cross-domain alias: `optical-phased-array-lidar`)
**Domain:** Photonics / Optics
**Definition:** Electronically steered beam from chip-scale array of antennas with controlled phase; no mechanical parts.
**Atom or composite:** Composite (phased waveguide array + thermo/EO phase shifters).
**Real wall?** Yes — antenna count × emitter efficiency limits range × FOV.
**Cross-domain wiring:** Direct optical analog of electromagnetics-antennas phased-array.
**Notes:** Sun 2013.

---

## Free-Space Optical Communication

### pat (cross-domain alias: `pointing-acquisition-tracking`, `PAT`)
**Domain:** Photonics / Optics
**Definition:** Closed-loop control system that points, acquires, and tracks a remote terminal for FSO link.
**Atom or composite:** Composite (gimbal + WFS-like + control).
**Real wall?** Yes — pointing jitter must be ≪ beam divergence.
**Cross-domain wiring:** Same as control-numerical-opt servo loop; satellite tracking.
**Notes:** Hemmati 2006.

### atmospheric-cn2 (cross-domain alias: `Cn²`, `turbulence-strength`)
**Domain:** Photonics / Optics
**Definition:** Structure constant of refractive-index fluctuations; sets seeing and scintillation strength.
**Atom or composite:** Atom (turbulence parameter).
**Real wall?** Yes — fundamental atmospheric variability.
**Cross-domain wiring:** Same Kolmogorov turbulence as physics-diffusion fluid dynamics.
**Notes:** Andrews 2005.

### scintillation (cross-domain alias: `intensity-fluctuation`, `Rytov-variance`)
**Domain:** Photonics / Optics
**Definition:** Intensity fluctuations from atmospheric turbulence; σ_I² = exp(σ_R²) − 1.
**Atom or composite:** Composite (log-normal in weak turbulence).
**Real wall?** Yes — degrades FSO link BER.
**Cross-domain wiring:** Same as signal-processing-rf fading channel.
**Notes:** Andrews 2005.

### beam-wander (cross-domain alias: `centroid-jitter`)
**Domain:** Photonics / Optics
**Definition:** Random angular displacement of beam centroid due to turbulence-tilt; degrades pointing.
**Atom or composite:** Composite (turbulence tilt mode).
**Real wall?** Yes — fundamental atmospheric tilt component.
**Cross-domain wiring:** Same as control-numerical-opt LOS jitter; can be mitigated by tip-tilt AO.
**Notes:** Fante 1975.

### free-space-qkd (cross-domain alias: `FS-QKD`)
**Domain:** Photonics / Optics
**Definition:** Quantum key distribution over free-space link (satellite, terrestrial); BB84/E91.
**Atom or composite:** Composite (single-photon source + tracking + WCP/MDI protocol).
**Real wall?** Yes — turbulence + losses limit secure key rate.
**Cross-domain wiring:** Same as quantum-computing key distribution; networking secure-key channel.
**Notes:** Liao 2017 (Micius).

---

## Other Photonics Primitives

### optical-tweezers-single (cross-domain alias: `OT`, `single-beam-trap`)
**Domain:** Photonics / Optics
**Definition:** Single tightly focused beam creates gradient force on dielectric particle; traps in 3D.
**Atom or composite:** Composite (high-NA objective + gradient force).
**Cost model:** Force ~ pN/(mW·µm).
**Real wall?** Yes — heating from absorption limits power on biological samples.
**Cross-domain wiring:** Same gradient-force as physics-diffusion dielectrophoresis; biology-bioinformatics single-molecule.
**Notes:** Ashkin 1986 Nobel.

### dual-trap-tweezers (cross-domain alias: `dual-beam-OT`, `DNA-stretching-OT`)
**Domain:** Photonics / Optics
**Definition:** Two independent optical traps; useful for measuring single-molecule force-extension curves.
**Atom or composite:** Composite (split-beam OT + position sensor).
**Real wall?** Yes — force sensitivity limited by Brownian noise.
**Cross-domain wiring:** Same as biology-bioinformatics DNA mechanics; statistics-probability Langevin dynamics.
**Notes:** Smith 1996.

### holographic-tweezers (cross-domain alias: `HOT`, `SLM-trap-array`)
**Domain:** Photonics / Optics
**Definition:** SLM displays phase hologram producing multiple controllable traps in parallel.
**Atom or composite:** Composite (SLM + CGH + objective).
**Real wall?** Yes — SLM efficiency × cross-talk between traps.
**Cross-domain wiring:** Same Fourier-plane control as 4f system; ml-training parallel computation.
**Notes:** Grier 2003.

### radiation-pressure (cross-domain alias: `light-momentum-force`)
**Domain:** Photonics / Optics
**Definition:** Force F = P/c (absorbing surface) or 2P/c (reflecting); momentum transfer from photons.
**Atom or composite:** Atom (photon momentum).
**Real wall?** Yes — limited by available laser power.
**Cross-domain wiring:** Same momentum-flux as electromagnetics-antennas Poynting vector; LIGO test-mass back-action.
**Notes:** Lebedev 1901.

### abraham-minkowski (cross-domain alias: `light-momentum-controversy`)
**Domain:** Photonics / Optics
**Definition:** Two formulations of photon momentum in media: Minkowski p = nℏk vs Abraham p = ℏk/n. Resolved by context.
**Atom or composite:** Composite (canonical vs kinetic momentum).
**Real wall?** No — both correct in appropriate contexts.
**Cross-domain wiring:** Same canonical/kinetic distinction as physics-quantum gauge momentum.
**Notes:** Barnett 2010.

### photon-counting (cross-domain alias: `PC-detection`, `digital-photon-counting`)
**Domain:** Photonics / Optics
**Definition:** Discriminator on PMT/SPAD/SNSPD pulses; integer photon count per gate.
**Atom or composite:** Composite (detector + discriminator + counter).
**Cost model:** Count rate × dead time.
**Real wall?** Yes — dead time saturates at high flux.
**Cross-domain wiring:** Same Poisson counting as statistics-probability rare events; signal-processing-rf event-driven sensing.
**Notes:** Demtröder §6.

### tcspc (cross-domain alias: `time-correlated-single-photon-counting`)
**Domain:** Photonics / Optics
**Definition:** Histogram photon arrival times relative to laser sync; ps resolution; basis of FLIM.
**Atom or composite:** Composite (SPAD + TAC + MCA).
**Cost model:** Counts × bins × pile-up correction.
**Real wall?** Yes — pile-up at >1 count/pulse distorts decay.
**Cross-domain wiring:** Same as signal-processing-rf time-of-arrival histogramming; statistics-probability arrival-time distributions.
**Notes:** O'Connor & Phillips 1984.

### speckle (cross-domain alias: `coherent-speckle`, `random-interference`)
**Domain:** Photonics / Optics
**Definition:** Granular intensity pattern from coherent illumination of rough surface; statistics: Rayleigh in amplitude, exponential in intensity.
**Atom or composite:** Composite (random superposition of many phasors).
**Real wall?** Yes — fundamental for coherent imaging; mitigated by spatial/temporal incoherence.
**Cross-domain wiring:** Same as signal-processing-rf multipath; statistics-probability random-walk in complex plane.
**Notes:** Goodman 2007.

### homo-pol-mode-dispersion (cross-domain alias: `1st-order-PMD-vector`)
**Domain:** Photonics / Optics
**Definition:** Polarization-mode dispersion vector Ω: dT/dω parameterizes 1st-order PMD; |Ω| = DGD.
**Atom or composite:** Composite (frequency-dependent Stokes evolution).
**Real wall?** Yes — random concatenation gives ⟨DGD⟩ ∝ √L.
**Cross-domain wiring:** Same SU(2) algebra as quantum-computing single-qubit gates.
**Notes:** Gordon & Kogelnik 2000.

### nlse (cross-domain alias: `nonlinear-Schrödinger-equation-optical`)
**Domain:** Photonics / Optics
**Definition:** i ∂A/∂z = (β₂/2) ∂²A/∂T² − γ|A|²A + iα/2·A; full envelope model in fibers.
**Atom or composite:** Composite (dispersion + nonlinearity + loss).
**Cost model:** Solved by split-step Fourier.
**Real wall?** Yes — Raman, higher-order dispersion add corrections.
**Cross-domain wiring:** Same NLSE as physics-diffusion Bose-Einstein condensate (Gross-Pitaevskii).
**Notes:** Agrawal §2.

### eye-safety-limit (cross-domain alias: `MPE`, `maximum-permissible-exposure`)
**Domain:** Photonics / Optics
**Definition:** Wavelength- and exposure-dependent intensity ceiling protecting retina/cornea (ANSI Z136).
**Atom or composite:** Composite (engineering safety standard).
**Real wall?** Yes — strict regulatory limit; key constraint for FSO/lidar design.
**Cross-domain wiring:** Same kind of regulatory threshold as RF-exposure limits in signal-processing-rf.
**Notes:** ANSI Z136.1.

### abbe-diffraction-limit (cross-domain alias: `Abbe-limit`, `λ/(2NA)`)
**Domain:** Photonics / Optics
**Definition:** Minimum resolvable distance Δx = λ/(2 NA) for incoherent imaging through aperture of NA.
**Atom or composite:** Atom (fundamental resolution limit).
**Real wall?** Yes — only super-resolution (STED, SMLM, near-field) defeats it.
**Cross-domain wiring:** Same Fourier-cutoff as signal-processing-rf Nyquist limit.
**Notes:** Abbe 1873.

### etendue (cross-domain alias: `optical-throughput`, `n²A·Ω`)
**Domain:** Photonics / Optics
**Definition:** A·Ω·n² conserved through optical system; sets brightness × area product upper bound.
**Atom or composite:** Atom (phase-space invariant).
**Real wall?** Yes — fundamentally invariant in lossless system; concentration limited.
**Cross-domain wiring:** Same as Liouville theorem in physics-classical; information-theory-coding channel capacity (radiance ↔ bits).
**Notes:** Welford & Winston 1989.

### rayleigh-criterion (cross-domain alias: `Rayleigh-resolution`, `1.22λ/D`)
**Domain:** Photonics / Optics
**Definition:** Two point sources resolvable when central maximum of one falls on first null of the other: θ = 1.22 λ/D.
**Atom or composite:** Atom (resolution criterion).
**Real wall?** Yes — diffraction-based.
**Cross-domain wiring:** Same as signal-processing-rf bandwidth-resolution; standard in astronomy.
**Notes:** Rayleigh 1879.

### sparrow-criterion (cross-domain alias: `Sparrow-resolution`)
**Domain:** Photonics / Optics
**Definition:** Stricter than Rayleigh: two sources resolved when second derivative of combined intensity = 0 at midpoint.
**Atom or composite:** Composite (numerical criterion).
**Real wall?** Yes — informational limit < Rayleigh.
**Cross-domain wiring:** Same as signal-processing-rf super-resolution metric.
**Notes:** Sparrow 1916.

### dispersion-relation-optical (cross-domain alias: `ω(k)`, `n(λ)`)
**Domain:** Photonics / Optics
**Definition:** Relationship k(ω) = n(ω) ω/c; group velocity v_g = dω/dk.
**Atom or composite:** Atom (material property).
**Real wall?** Yes — Kramers-Kronig couples n and absorption.
**Cross-domain wiring:** Same as physics-diffusion wave dispersion; electromagnetics-antennas waveguide dispersion.
**Notes:** Jackson §7.

### kramers-kronig (cross-domain alias: `K-K-relation`, `causality-Hilbert`)
**Domain:** Photonics / Optics
**Definition:** Re[χ(ω)] and Im[χ(ω)] are Hilbert-transform pairs by causality.
**Atom or composite:** Composite (causality principle in Fourier).
**Real wall?** No — exact for linear causal systems.
**Cross-domain wiring:** Same Hilbert relation in signal-processing-rf analytic signals; control-numerical-opt minimum-phase networks.
**Notes:** Toll 1956.

### group-velocity-dispersion (cross-domain alias: `GVD`, `β₂`)
**Domain:** Photonics / Optics
**Definition:** β₂ = d²β/dω²; pulse broadening rate per unit length per unit bandwidth.
**Atom or composite:** Atom (second-order dispersion coefficient).
**Real wall?** Yes — must be managed for short pulses.
**Cross-domain wiring:** Same as signal-processing-rf chirp from dispersive channel.
**Notes:** Agrawal §1.

### zero-dispersion-wavelength (cross-domain alias: `ZDW`, `λ_0-GVD=0`)
**Domain:** Photonics / Optics
**Definition:** Wavelength where β₂ = 0; pumping near ZDW enables phase-matched FWM and supercontinuum.
**Atom or composite:** Atom (special λ of waveguide).
**Real wall?** Yes — material + waveguide dispersion combine.
**Cross-domain wiring:** Operating point for nonlinear pulse propagation; analog of zero-IF in signal-processing-rf.
**Notes:** Agrawal §1.

### slow-light (cross-domain alias: `v_g≪c`, `EIT-slow-light`)
**Domain:** Photonics / Optics
**Definition:** Engineered v_g much less than c via narrow resonance (EIT, photonic crystal); time-bandwidth tradeoff.
**Atom or composite:** Composite (sharp dispersion).
**Real wall?** Yes — bandwidth-delay product upper-bounded.
**Cross-domain wiring:** Same as quantum-computing photonic memory; signal-processing-rf delay line.
**Notes:** Boyd 2009.

### fast-light (cross-domain alias: `v_g>c`, `superluminal-group`)
**Domain:** Photonics / Optics
**Definition:** Group velocity > c (or negative) in anomalous-dispersion region; no information faster than c.
**Atom or composite:** Composite (anomalous dispersion).
**Real wall?** Yes — front velocity is c; group velocity is not information speed.
**Cross-domain wiring:** Same kind of anomalous group delay in signal-processing-rf filters.
**Notes:** Boyd 2009.

### thermo-optic-effect (cross-domain alias: `dn/dT`)
**Domain:** Photonics / Optics
**Definition:** Refractive index varies with temperature dn/dT (~ 10⁻⁵/K silica, ~ 10⁻⁴/K Si).
**Atom or composite:** Atom (material coefficient).
**Real wall?** Yes — fundamental thermal drift; used as heater tuning.
**Cross-domain wiring:** Same as physics-diffusion temperature-dependent material parameter.
**Notes:** Cocorullo 1992.

### electro-absorption (cross-domain alias: `Franz-Keldysh`, `QCSE`)
**Domain:** Photonics / Optics
**Definition:** Voltage-induced shift of absorption edge in semiconductor (FK in bulk, QCSE in QW); basis of EA modulators.
**Atom or composite:** Composite (field + semiconductor band structure).
**Real wall?** Yes — narrowband; chirp control.
**Cross-domain wiring:** Same field-effect as physics-quantum Stark; used in datacenter networking modulators.
**Notes:** Wood 1988.

### photoelastic-effect (cross-domain alias: `stress-optic`)
**Domain:** Photonics / Optics
**Definition:** Strain induces birefringence Δn = C σ; allows stress visualization and PMD analysis.
**Atom or composite:** Atom (material tensor C).
**Real wall?** No.
**Cross-domain wiring:** Same as physics-diffusion stress-strain coupling.
**Notes:** Brewster 1815.

### magneto-optic-effect (cross-domain alias: `Faraday-effect`, `Kerr-MO`)
**Domain:** Photonics / Optics
**Definition:** Magnetic field rotates polarization (Faraday: transmission, Kerr: reflection); θ = V·B·L.
**Atom or composite:** Atom (Verdet constant V).
**Real wall?** Yes — non-reciprocal; basis of isolators.
**Cross-domain wiring:** Same time-reversal-breaking as electromagnetics-antennas non-reciprocal devices.
**Notes:** Faraday 1845.

### isolator (cross-domain alias: `optical-diode`)
**Domain:** Photonics / Optics
**Definition:** Non-reciprocal Faraday rotator + polarizers; blocks back-reflection but passes forward.
**Atom or composite:** Composite (Faraday + polarizer pair).
**Real wall?** Yes — non-reciprocity requires magnetic field, breaking time-reversal.
**Cross-domain wiring:** Same one-way device concept as signal-processing-rf circulator.
**Notes:** Yariv §6.

### circulator-optical (cross-domain alias: `optical-circulator`)
**Domain:** Photonics / Optics
**Definition:** 3-port (or 4-port) non-reciprocal device routing input cyclically: 1→2, 2→3, etc.
**Atom or composite:** Composite (Faraday rotator + PBS).
**Real wall?** Yes — bandwidth × isolation tradeoff.
**Cross-domain wiring:** Same topology as electromagnetics-antennas RF circulator.
**Notes:** Iwamura 1979.

### narrow-linewidth-laser (cross-domain alias: `Schawlow-Townes-limit`)
**Domain:** Photonics / Optics
**Definition:** Minimum linewidth Δν_ST = πhν(Δν_c)²/P; fundamental quantum-phase-noise limit.
**Atom or composite:** Atom (quantum-limit).
**Real wall?** Yes — set by spontaneous emission into the lasing mode.
**Cross-domain wiring:** Same as quantum-computing phase noise; signal-processing-rf oscillator phase noise.
**Notes:** Schawlow & Townes 1958.

### shot-noise (cross-domain alias: `Poisson-noise-optical`)
**Domain:** Photonics / Optics
**Definition:** σ_n² = ⟨n⟩ photon-counting fluctuation; sets fundamental SNR = √⟨n⟩ for coherent state.
**Atom or composite:** Atom (quantum noise floor).
**Real wall?** Yes — beaten only by squeezed light.
**Cross-domain wiring:** Same as statistics-probability Poisson statistics; signal-processing-rf detector noise.
**Notes:** Schottky 1918.

### rin (cross-domain alias: `relative-intensity-noise`)
**Domain:** Photonics / Optics
**Definition:** RIN(f) = S_P(f)/⟨P⟩²; relative power-spectral density of intensity fluctuations from laser source.
**Atom or composite:** Composite (intensity-noise PSD normalized).
**Real wall?** Yes — sets minimum detectable AM modulation.
**Cross-domain wiring:** Same as signal-processing-rf amplitude noise floor.
**Notes:** Yariv §11.

### lambertian-source (cross-domain alias: `cosine-emitter`, `diffuse-source`)
**Domain:** Photonics / Optics
**Definition:** Intensity I(θ) = I₀ cos θ; radiance is angle-independent.
**Atom or composite:** Atom (canonical diffuse-emission model).
**Real wall?** No — idealization.
**Cross-domain wiring:** Same model as graphics-rendering-lod diffuse BRDF.
**Notes:** Lambert 1760.

### radiance (cross-domain alias: `L`, `W/(m²·sr)`)
**Domain:** Photonics / Optics
**Definition:** Power per unit area per unit solid angle; invariant along ray in lossless system.
**Atom or composite:** Atom (radiometric quantity).
**Real wall?** Yes — étendue-conservation fundamental.
**Cross-domain wiring:** Same as graphics-rendering-lod radiometry foundation.
**Notes:** McCluney 1994.

### irradiance (cross-domain alias: `E`, `W/m²`)
**Domain:** Photonics / Optics
**Definition:** Power per unit area at a surface; E = ∫L cos θ dΩ.
**Atom or composite:** Composite (radiance integrated over hemisphere).
**Real wall?** No.
**Cross-domain wiring:** Same as graphics-rendering-lod illuminance/irradiance.
**Notes:** McCluney 1994.

### luminous-flux (cross-domain alias: `lumen`, `photometric-power`)
**Domain:** Photonics / Optics
**Definition:** Photometric quantity weighted by V(λ); lumens; 683 lm/W at 555 nm peak.
**Atom or composite:** Composite (radiant flux + V(λ)).
**Real wall?** Yes — defined per CIE standard observer.
**Cross-domain wiring:** Same kind of human-perceptual weighting as graphics-rendering-lod tone-mapping.
**Notes:** CIE 1931.

### cri (cross-domain alias: `color-rendering-index`)
**Domain:** Photonics / Optics
**Definition:** Measure of how a light source renders 8 standard test colors vs reference; max 100.
**Atom or composite:** Composite (spectral metric).
**Real wall?** Yes — limited by source spectrum.
**Cross-domain wiring:** Same as graphics-rendering-lod color-fidelity metric.
**Notes:** CIE 13.3.

### cct (cross-domain alias: `correlated-color-temperature`)
**Domain:** Photonics / Optics
**Definition:** Temperature of blackbody whose chromaticity is closest to the source; e.g., 2700K warm white, 6500K daylight.
**Atom or composite:** Composite (CIE chromaticity distance).
**Real wall?** No.
**Cross-domain wiring:** Same as graphics-rendering-lod white-point in color science.
**Notes:** CIE 15:2004.

---


