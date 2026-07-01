# Astrophysics & Cosmology — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## 1. General Relativity (Operational)

### einstein-field-equations-op (cross-domain alias: `efe`, `gravity-pde`, `curvature=stress`)
**Domain:** Astrophysics / Cosmology
**Definition:** G_μν + Λg_μν = (8πG/c⁴) T_μν — curvature sourced by stress-energy; 10 coupled nonlinear PDEs for the metric g_μν.
**Atom or composite:** Composite (Ricci tensor, scalar, metric, stress-energy).
**Cost model:** Symbolic: O(d⁴) Christoffel terms; numeric: depends on solver (BSSN/ADM).
**Real wall?** Yes — nonlinear hyperbolic-elliptic coupling forces constraint preservation.
**Cross-domain wiring:** Maps to physics-diffusion (relaxation methods), control-numerical-opt (constraint projection), ml-training (auto-diff Christoffels). Tensor algebra shares index machinery with electromagnetics-antennas (covariant Maxwell).
**Notes:** Einstein (1915); modern operational form in Misner-Thorne-Wheeler; numerical formulations in Baumgarte-Shapiro.

### schwarzschild-solution (cross-domain alias: `schwarzschild`, `static-vacuum`, `r_s`)
**Domain:** Astrophysics / Cosmology
**Definition:** Unique spherically symmetric vacuum solution: ds² = −(1−r_s/r)c²dt² + (1−r_s/r)⁻¹dr² + r²dΩ²; r_s = 2GM/c².
**Atom or composite:** Atom (canonical exact solution).
**Cost model:** Closed-form geodesics via effective potential; orbit integration O(N) steps.
**Real wall?** Yes — coordinate singularity at r_s; true singularity at r=0.
**Cross-domain wiring:** Geodesic equation is an ODE shared with control-numerical-opt (symplectic integrators). Effective potential reuses statistics-probability (1D landscape sampling).
**Notes:** Schwarzschild (1916); Birkhoff theorem ensures uniqueness.

### kerr-solution (cross-domain alias: `kerr`, `rotating-bh`, `boyer-lindquist`)
**Domain:** Astrophysics / Cosmology
**Definition:** Axisymmetric stationary vacuum solution for rotating BH parametrized by mass M and spin a=J/Mc; metric expressed in Boyer-Lindquist coordinates.
**Atom or composite:** Atom.
**Cost model:** Closed-form; geodesic integration uses Carter constant for separation.
**Real wall?** Yes — frame dragging, ergosphere, inner Cauchy horizon instability.
**Cross-domain wiring:** Hamilton-Jacobi separability connects to signal-processing-rf (separable filter banks). Ray tracing reuses photonics-optics machinery.
**Notes:** Kerr (1963); Carter constant (1968) gives the fourth integral.

### reissner-nordstrom (cross-domain alias: `rn-bh`, `charged-bh`)
**Domain:** Astrophysics / Cosmology
**Definition:** Spherically symmetric charged BH: f(r) = 1 − 2GM/(rc²) + GQ²/(4πε₀r²c⁴); two horizons r_± when |Q|<M.
**Atom or composite:** Atom.
**Cost model:** Algebraic horizon roots; geodesics in effective potential.
**Real wall?** Yes — extremal limit Q=M sets a no-go for further charge.
**Cross-domain wiring:** Charge term couples to electromagnetics-antennas (Maxwell stress-energy). Effective potential analysis mirrors photonics-optics waveguide modes.
**Notes:** Reissner (1916), Nordström (1918); third-law-style extremality bounds.

### kerr-newman (cross-domain alias: `kn-bh`, `charged-rotating-bh`)
**Domain:** Astrophysics / Cosmology
**Definition:** Most general electrovacuum stationary BH: mass M, spin a, charge Q. Metric extends Kerr with Q² shifting Δ.
**Atom or composite:** Composite (Kerr × charge).
**Cost model:** Same as Kerr plus EM-field tracing.
**Real wall?** Yes — astrophysical BHs are essentially neutral; KN is mostly theoretical.
**Cross-domain wiring:** Combines Kerr geodesic machinery with electromagnetics-antennas Maxwell tensor in curved space.
**Notes:** Newman et al. (1965); no-hair theorem covers this family.

### tov-equation (cross-domain alias: `tov`, `relativistic-hydrostatic`, `ns-structure`)
**Domain:** Astrophysics / Cosmology
**Definition:** dp/dr = −G[ρ+p/c²][m(r)+4πr³p/c²] / [r²(1−2Gm/rc²)] — relativistic generalization of stellar hydrostatic equilibrium.
**Atom or composite:** Composite (mass continuity + relativistic pressure balance).
**Cost model:** ODE integration O(N) given EOS; mass-radius curve sweep over central density.
**Real wall?** Yes — equation of state above nuclear density is uncertain.
**Cross-domain wiring:** Shooting/integration ODE pattern is reused in control-numerical-opt; mass-radius relation maps to statistics-probability inference of NS EOS.
**Notes:** Tolman (1939), Oppenheimer-Volkoff (1939); modern review Lattimer (2012).

### isco (cross-domain alias: `innermost-stable-circular-orbit`, `r_isco`)
**Domain:** Astrophysics / Cosmology
**Definition:** Smallest radius at which circular timelike geodesics are stable; Schwarzschild: r_ISCO=6GM/c²; Kerr prograde a=1: r_ISCO=GM/c².
**Atom or composite:** Atom (effective-potential inflection).
**Cost model:** Algebraic root from V_eff''(r)=0.
**Real wall?** Yes — sets inner edge of thin accretion disks and efficiency η.
**Cross-domain wiring:** Efficiency η connects to photonics-optics radiative bookkeeping; spin measurement via ISCO maps to statistics-probability MCMC.
**Notes:** Bardeen, Press, Teukolsky (1972).

### photon-sphere (cross-domain alias: `light-ring`, `r_ph`)
**Domain:** Astrophysics / Cosmology
**Definition:** Radius of unstable null circular geodesic; Schwarzschild: r=3GM/c²; sets BH shadow size in EHT images.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — Lyapunov instability controls ringdown.
**Cross-domain wiring:** Shadow imaging shares deconvolution machinery with signal-processing-rf (VLBI CLEAN). Lyapunov exponent connects to control-numerical-opt stability theory.
**Notes:** Bardeen (1973); EHT (M87, Sgr A*) imaged shadows.

### ergosphere (cross-domain alias: `ergoregion`, `frame-drag-zone`)
**Domain:** Astrophysics / Cosmology
**Definition:** Region outside Kerr event horizon where g_tt>0; static observers impossible; co-rotation forced.
**Atom or composite:** Atom (metric-component sign change).
**Cost model:** Algebraic boundary surface.
**Real wall?** Yes — defines where energy extraction is possible.
**Cross-domain wiring:** Frame dragging maps to electromagnetics-antennas (gravitomagnetic analogy); rotational energy budget connects to control-numerical-opt (extremum extraction).
**Notes:** Penrose (1969).

### penrose-process (cross-domain alias: `rotational-extraction`)
**Domain:** Astrophysics / Cosmology
**Definition:** Mechanism to extract rotational energy by splitting a particle in the ergosphere so one fragment carries negative energy into the horizon.
**Atom or composite:** Composite (ergosphere + 4-momentum split).
**Cost model:** Algebraic energetics; efficiency ≤ 20.7%.
**Real wall?** Yes — requires arrangement of fragments; mostly theoretical, BZ dominates astrophysically.
**Cross-domain wiring:** Energy-budget bookkeeping reused in control-numerical-opt (Pareto extraction). Negative-energy state maps to photonics-optics squeezed-state intuition.
**Notes:** Penrose (1969); superradiance is the wave analog.

### blandford-znajek (cross-domain alias: `bz-mechanism`, `bh-jet-engine`)
**Domain:** Astrophysics / Cosmology
**Definition:** EM extraction of Kerr rotational energy via force-free magnetosphere threading the horizon; power P_BZ ∝ a²B²M².
**Atom or composite:** Composite (Kerr + force-free MHD).
**Cost model:** Force-free / GRMHD simulation cost.
**Real wall?** Yes — efficiency depends on magnetic flux saturation (MAD state).
**Cross-domain wiring:** Force-free electrodynamics shares structure with electromagnetics-antennas; jet launching couples to plasma & MHD primitives downstream.
**Notes:** Blandford & Znajek (1977); modern GRMHD: Tchekhovskoy, McKinney.

### bh-thermodynamics (cross-domain alias: `four-laws-bh`)
**Domain:** Astrophysics / Cosmology
**Definition:** Four laws: κ constant on horizon (0th); dM = (κ/8π)dA + ΩdJ + ΦdQ (1st); δA ≥ 0 (2nd); κ→0 unattainable (3rd).
**Atom or composite:** Composite.
**Cost model:** Algebraic.
**Real wall?** Yes — area theorem and 2nd law generalized to GSL.
**Cross-domain wiring:** Direct analog to statistics-probability thermodynamic ensembles; entropy bookkeeping connects to ml-training (info-theoretic bounds).
**Notes:** Bardeen, Carter, Hawking (1973); Bekenstein (1973).

### hawking-radiation (cross-domain alias: `bh-evaporation`)
**Domain:** Astrophysics / Cosmology
**Definition:** Thermal emission from BH horizon due to QFT in curved spacetime; spectrum nearly blackbody at Hawking temperature.
**Atom or composite:** Composite (QFT vacuum + horizon).
**Cost model:** Bogoliubov-coefficient calculation; greybody factors.
**Real wall?** Yes — astrophysical BHs colder than CMB; lifetime ∝ M³.
**Cross-domain wiring:** Bogoliubov transformations reuse signal-processing-rf machinery (mode mixing); spectrum analysis shares photonics-optics blackbody tooling.
**Notes:** Hawking (1974, 1975).

### hawking-temperature (cross-domain alias: `T_H`, `surface-gravity-temp`)
**Domain:** Astrophysics / Cosmology
**Definition:** T_H = ℏκ/(2πk_B c); Schwarzschild: T_H = ℏc³/(8πGMk_B) ≈ 6×10⁻⁸ K (M_⊙/M).
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — vanishingly small for stellar BHs.
**Cross-domain wiring:** Connects to surface gravity κ — same Killing-vector machinery as ergosphere/horizon analysis.
**Notes:** Hawking (1975).

### bekenstein-hawking-entropy (cross-domain alias: `s_bh`, `area-entropy`)
**Domain:** Astrophysics / Cosmology
**Definition:** S_BH = k_B A / (4ℓ_P²) — entropy equals one-quarter horizon area in Planck units.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — holographic bound is the strongest known limit on info per area.
**Cross-domain wiring:** Holographic principle underwrites ml-training (info-theoretic capacity bounds) and statistics-probability (max-entropy framework).
**Notes:** Bekenstein (1972), Hawking (1974); 't Hooft, Susskind holography.

### bh-information-paradox (cross-domain alias: `info-paradox`, `page-curve`)
**Domain:** Astrophysics / Cosmology
**Definition:** Apparent loss of unitarity when BHs evaporate into thermal radiation; Page curve gives expected entropy of Hawking radiation.
**Atom or composite:** Composite (Hawking radiation + unitarity constraint).
**Cost model:** Replica-trick island calculations in semiclassical gravity.
**Real wall?** Open problem; islands/QES proposals restore Page curve.
**Cross-domain wiring:** Connects to ml-training (info-theoretic capacity), statistics-probability (entropy estimators), and quantum (entanglement entropy).
**Notes:** Hawking (1976); Page (1993); Almheiri et al. (2019).

### no-hair-theorem (cross-domain alias: `nht`, `bh-uniqueness`)
**Domain:** Astrophysics / Cosmology
**Definition:** Stationary axisymmetric electrovacuum BHs are uniquely characterized by (M, J, Q) — all other multipoles fixed.
**Atom or composite:** Atom (uniqueness statement).
**Cost model:** N/A (theorem).
**Real wall?** Yes — testable via ringdown spectroscopy.
**Cross-domain wiring:** Parametrized post-Kerr tests connect to statistics-probability (model selection) and signal-processing-rf (QNM mode fitting).
**Notes:** Israel (1967), Carter (1971), Robinson (1975).

### geodesic-equation (cross-domain alias: `parallel-transport`, `christoffel-ode`)
**Domain:** Astrophysics / Cosmology
**Definition:** d²x^μ/dτ² + Γ^μ_αβ dx^α/dτ dx^β/dτ = 0; ODE for free-fall in curved spacetime.
**Atom or composite:** Atom.
**Cost model:** ODE integration; symplectic recommended.
**Real wall?** No; standard ODE machinery.
**Cross-domain wiring:** Same ODE structure as control-numerical-opt (constrained Hamiltonian flow); integrator choice shared with photonics-optics ray tracing.
**Notes:** Einstein (1915); modern integrators: Press et al., MTW.

### killing-vector (cross-domain alias: `killing-field`, `symmetry-generator`)
**Domain:** Astrophysics / Cosmology
**Definition:** Vector field ξ satisfying ∇_(μ ξ_ν)=0; generates spacetime isometry and conserved quantity p_μ ξ^μ.
**Atom or composite:** Atom.
**Cost model:** Algebraic / Lie derivative computation.
**Real wall?** No.
**Cross-domain wiring:** Noether-symmetry pattern shared with control-numerical-opt conserved quantities; symplectic integrators leverage Killing vectors.
**Notes:** Killing (1892); MTW ch. 25.

## 2. Numerical Relativity

### bssn-formulation (cross-domain alias: `bssn`, `conformal-3+1`)
**Domain:** Astrophysics / Cosmology
**Definition:** Baumgarte-Shapiro-Shibata-Nakamura conformal reformulation of ADM with conformal factor χ, traceless extrinsic curvature Ã_ij, conformal connection Γ̃^i.
**Atom or composite:** Composite (ADM + conformal split + Γ̃-evolution).
**Cost model:** O(N³) per timestep on 3D grid; standard for binary BH inspirals.
**Real wall?** Yes — constraint violations grow without damping terms.
**Cross-domain wiring:** PDE structure shared with physics-diffusion (hyperbolic flux solvers); constraint damping is control-numerical-opt feedback.
**Notes:** Nakamura, Oohara, Kojima (1987); Shibata-Nakamura (1995); Baumgarte-Shapiro (1998).

### adm-3+1-split (cross-domain alias: `adm`, `lapse-shift`)
**Domain:** Astrophysics / Cosmology
**Definition:** Decompose 4-metric into spatial 3-metric γ_ij, lapse α, shift β^i; evolve γ_ij and extrinsic curvature K_ij with Hamiltonian and momentum constraints.
**Atom or composite:** Composite.
**Cost model:** Standard for NR; constraints are elliptic.
**Real wall?** Yes — raw ADM is weakly hyperbolic; BSSN/Z4c fix it.
**Cross-domain wiring:** Constraint-evolution split mirrors control-numerical-opt DAE solvers; lapse-shift gauge choice is a coordinate optimization.
**Notes:** Arnowitt, Deser, Misner (1962); York (1979).

### harmonic-slicing (cross-domain alias: `1+log-slicing`, `gauge-choice`)
**Domain:** Astrophysics / Cosmology
**Definition:** Choice of lapse: ∂_t α = −2αK (1+log) or □x^μ=0 (harmonic); avoids singularities, prevents grid stretching.
**Atom or composite:** Atom (gauge ODE).
**Cost model:** Local algebraic update.
**Real wall?** Yes — bad gauges crash NR codes.
**Cross-domain wiring:** Gauge fixing mirrors control-numerical-opt regularization choices; signal-processing-rf phase-locking analogy.
**Notes:** Bona-Massó (1995); Alcubierre review.

### moving-puncture (cross-domain alias: `puncture-method`)
**Domain:** Astrophysics / Cosmology
**Definition:** BH evolution scheme where punctures (singularities) are co-moved by shift vector without excision, enabled by 1+log lapse and Gamma-driver shift.
**Atom or composite:** Composite (BSSN + 1+log + Gamma-driver).
**Cost model:** Standard for binary BH; AMR essential.
**Real wall?** Yes — required for stable BBH inspirals.
**Cross-domain wiring:** Co-moving frame trick reused in physics-diffusion (Lagrangian/ALE methods).
**Notes:** Campanelli et al. (2006); Baker et al. (2006).

### characteristic-method-nr (cross-domain alias: `null-cone-evolution`, `ccm`)
**Domain:** Astrophysics / Cosmology
**Definition:** Evolve Einstein equations on outgoing null cones; well-suited for wave-zone GW extraction at future null infinity.
**Atom or composite:** Composite.
**Cost model:** O(N²) per slice on null grid.
**Real wall?** Yes — caustics limit interior coverage.
**Cross-domain wiring:** Null evolution mirrors signal-processing-rf method-of-characteristics; future infinity is the propagation-zone analog.
**Notes:** Bondi (1962); Winicour reviews.

### berger-oliger-amr (cross-domain alias: `amr`, `block-amr`)
**Domain:** Astrophysics / Cosmology
**Definition:** Adaptive mesh refinement using nested rectangular patches at varying resolutions; refinement triggered by error/criterion.
**Atom or composite:** Composite (mesh hierarchy + flux correction).
**Cost model:** Memory/computation savings of 10⁴–10⁶ vs uniform.
**Real wall?** No — algorithmic.
**Cross-domain wiring:** Multi-scale hierarchy reused in ml-training (multi-resolution), signal-processing-rf (wavelet packets), photonics-optics (multigrid).
**Notes:** Berger-Oliger (1984); Berger-Colella (1989).

### excision (cross-domain alias: `bh-excision`, `inner-bc`)
**Domain:** Astrophysics / Cosmology
**Definition:** Remove BH interior from computational domain by excising a region inside the horizon; relies on causal disconnection.
**Atom or composite:** Atom (boundary technique).
**Cost model:** Reduces stiffness; complex book-keeping.
**Real wall?** Yes — must track moving horizons.
**Cross-domain wiring:** Domain-removal pattern appears in control-numerical-opt (active-set methods).
**Notes:** Unruh (1984); Seidel-Suen.

### constraint-damping (cross-domain alias: `z4c-damping`, `gundlach-damping`)
**Domain:** Astrophysics / Cosmology
**Definition:** Add terms proportional to constraint violations to drive them exponentially to zero during evolution; Z4/Z4c formulation.
**Atom or composite:** Composite (Z4 + damping coefficients).
**Cost model:** Cheap; algebraic source terms.
**Real wall?** Yes — without damping, constraints blow up.
**Cross-domain wiring:** Feedback-stabilization mirrors control-numerical-opt PI/PID controllers; reuses statistics-probability error-correction language.
**Notes:** Gundlach et al. (2005); Bona, Bernuzzi, Hilditch.

### psi4-extraction (cross-domain alias: `weyl-scalar-extraction`, `ψ4`)
**Domain:** Astrophysics / Cosmology
**Definition:** Newman-Penrose scalar ψ₄ encodes outgoing GW radiation in vacuum; h_+ − ih_× = ∫∫ψ₄ dt dt' at infinity.
**Atom or composite:** Composite (NP tetrad + double integration).
**Cost model:** Surface integration on extraction sphere; FFI in frequency domain to avoid drift.
**Real wall?** Yes — extraction radius finite; extrapolation needed.
**Cross-domain wiring:** Double-time integration shares signal-processing-rf high-pass-filter trick (fixed-frequency integration).
**Notes:** Newman-Penrose (1962); Reisswig-Pollney FFI (2011).

### cauchy-characteristic-extraction (cross-domain alias: `cce`, `null-extraction`)
**Domain:** Astrophysics / Cosmology
**Definition:** Combine 3+1 interior evolution with characteristic exterior evolution to extract GW at future null infinity unambiguously.
**Atom or composite:** Composite (Cauchy + characteristic + match).
**Cost model:** Adds null-grid evolution; produces gauge-invariant strain.
**Real wall?** Yes — most accurate extraction method.
**Cross-domain wiring:** Matching interior/exterior solvers parallels electromagnetics-antennas absorbing boundary methods.
**Notes:** Bishop, Gomez, Lehner, Winicour; SpECTRE, SXS catalogs.

### eob-waveform (cross-domain alias: `eob`, `effective-one-body`)
**Domain:** Astrophysics / Cosmology
**Definition:** Map two-body PN dynamics onto effective particle in deformed Kerr background; resums PN expansion; calibrated to NR.
**Atom or composite:** Composite (PN + deformation + NR calibration).
**Cost model:** Fast (~ms) per waveform; suitable for parameter estimation.
**Real wall?** Yes — extrapolation outside calibration range degrades accuracy.
**Cross-domain wiring:** Resummation strategy is a control-numerical-opt model-reduction pattern; calibration is ml-training surrogate methodology.
**Notes:** Buonanno-Damour (1999, 2000); SEOBNR family.

### imrphenom (cross-domain alias: `phenomenological-waveform`, `phenom`)
**Domain:** Astrophysics / Cosmology
**Definition:** Frequency-domain phenomenological waveform models (PhenomD/HM/X) joining PN inspiral, NR-fit merger, and ringdown analytic continuation.
**Atom or composite:** Composite (PN + NR fit + Lorentzian ringdown).
**Cost model:** O(N_f) frequency-domain evaluation; very fast.
**Real wall?** Yes — fit residuals dominate at high SNR.
**Cross-domain wiring:** Piecewise model joining shared with signal-processing-rf filter design; surrogate fitting is ml-training territory.
**Notes:** Ajith et al. (2007); Khan et al.; Pratten et al.

### seobnr (cross-domain alias: `seobnrv4`, `seobnrv5`)
**Domain:** Astrophysics / Cosmology
**Definition:** Spinning EOB-NR family of time-domain waveforms; latest SEOBNRv5 includes higher modes and aligned-spin precession.
**Atom or composite:** Composite (EOB + spin + NR + ringdown).
**Cost model:** ODE integration of EOB Hamiltonian; ~10ms/waveform.
**Real wall?** Yes — precession adds dimensionality to calibration.
**Cross-domain wiring:** Hamiltonian integration shared with control-numerical-opt; ringdown attachment uses signal-processing-rf templates.
**Notes:** Pan et al.; Bohé et al. (2017); LIGO LAL.

### numerical-relativity-pipeline (cross-domain alias: `nr-pipeline`)
**Domain:** Astrophysics / Cosmology
**Definition:** End-to-end stack: initial-data solver (BSSN/Z4c constraints) → evolution → horizon finder → waveform extraction → post-processing/catalog.
**Atom or composite:** Composite (every subsystem above).
**Cost model:** 10⁵–10⁶ CPU-hours per BBH simulation.
**Real wall?** Yes — wall-clock months for long inspirals.
**Cross-domain wiring:** Pipeline orchestration mirrors ml-training experiment-tracking and signal-processing-rf processing chains.
**Notes:** SXS, RIT, GATech, Frankfurt; SpEC, Einstein Toolkit, GRChombo.

## 3. Gravitational Waves

### linearized-gr (cross-domain alias: `weak-field-gr`, `h_μν`)
**Domain:** Astrophysics / Cosmology
**Definition:** g_μν = η_μν + h_μν with |h|≪1; linearized Einstein equations □h̄_μν = −(16πG/c⁴)T_μν in Lorenz gauge.
**Atom or composite:** Atom.
**Cost model:** Linear wave equation; FFT-friendly.
**Real wall?** No — but breaks down near strong-field sources.
**Cross-domain wiring:** Same wave operator as electromagnetics-antennas (Maxwell in Lorenz gauge); reuses photonics-optics propagation tools.
**Notes:** Einstein (1916); MTW ch. 18.

### gw-polarizations (cross-domain alias: `plus-cross`, `tt-modes`)
**Domain:** Astrophysics / Cosmology
**Definition:** Two physical GW polarizations h_+, h_× in TT gauge; transverse to propagation, traceless.
**Atom or composite:** Atom.
**Cost model:** N/A.
**Real wall?** Yes — alternative-theory tests look for extra polarizations.
**Cross-domain wiring:** Direct analog to photonics-optics Stokes parameters; detection uses signal-processing-rf polarization channels.
**Notes:** Eardley et al. (1973) classification.

### gw-strain (cross-domain alias: `h(t)`, `dimensionless-strain`)
**Domain:** Astrophysics / Cosmology
**Definition:** Dimensionless metric perturbation amplitude h = ΔL/L observable as length change in interferometer arms.
**Atom or composite:** Atom.
**Cost model:** Time-series; sample rate ≥ 4 kHz.
**Real wall?** Yes — detector noise floor 10⁻²³ /√Hz.
**Cross-domain wiring:** Time-series h(t) is a signal-processing-rf primary observable; whitening uses statistics-probability noise models.
**Notes:** LIGO Sensitivity Working Group.

### tt-gauge (cross-domain alias: `transverse-traceless`)
**Domain:** Astrophysics / Cosmology
**Definition:** Gauge in which h_μν is transverse (k^μ h_μν=0) and traceless (η^μν h_μν=0); only physical DOF remain.
**Atom or composite:** Atom.
**Cost model:** Algebraic projection.
**Real wall?** No — pure gauge choice.
**Cross-domain wiring:** Projection-onto-physical-modes pattern mirrors signal-processing-rf basis projection.
**Notes:** MTW ch. 35.

### quadrupole-formula (cross-domain alias: `mass-quadrupole-radiation`)
**Domain:** Astrophysics / Cosmology
**Definition:** Leading GW emission: h_ij^TT = (2G/rc⁴) Q̈_ij^TT; luminosity L_GW = (G/5c⁵)⟨Q⃛_ij Q⃛^ij⟩.
**Atom or composite:** Atom.
**Cost model:** Algebraic from second mass moment.
**Real wall?** Yes — fails when v/c is not small (need PN).
**Cross-domain wiring:** Multipole expansion shared with electromagnetics-antennas (electric/magnetic multipoles); spherical-harmonic decomposition reused.
**Notes:** Einstein (1918); Hulse-Taylor verified to 0.1%.

### post-newtonian-expansion (cross-domain alias: `pn`, `v/c-expansion`)
**Domain:** Astrophysics / Cosmology
**Definition:** Expansion in (v/c)² around Newtonian limit; nPN means corrections of order v^(2n+2)/c^(2n+2) beyond quadrupole.
**Atom or composite:** Composite.
**Cost model:** Algebraic per order; complexity grows fast (4.5PN known).
**Real wall?** Yes — convergence breaks at strong field.
**Cross-domain wiring:** Series acceleration uses statistics-probability Padé/resummation; perturbative expansion mirrors quantum loop expansions.
**Notes:** Blanchet review; Damour, Iyer, Will.

### ringdown (cross-domain alias: `qnm-ringdown`, `bh-bell`)
**Domain:** Astrophysics / Cosmology
**Definition:** Damped oscillations of post-merger BH characterized by complex quasi-normal mode frequencies ω_lmn determined by (M, a).
**Atom or composite:** Composite (Teukolsky modes + amplitudes).
**Cost model:** Continued-fraction Leaver method for ω_lmn; O(1).
**Real wall?** Yes — early ringdown vs. nonlinear regime boundary debated.
**Cross-domain wiring:** Damped sinusoid fitting reuses signal-processing-rf Prony/matrix-pencil methods.
**Notes:** Vishveshwara (1970); Leaver (1985); ringdown spectroscopy proposals.

### lyapunov-instability-photon (cross-domain alias: `light-ring-lyapunov`)
**Domain:** Astrophysics / Cosmology
**Definition:** Eikonal QNM frequencies relate to light-ring orbital frequency Ω and Lyapunov exponent λ: ω ≈ lΩ − i(n+½)λ.
**Atom or composite:** Composite (geodesic + perturbation).
**Cost model:** Algebraic in eikonal limit.
**Real wall?** Yes — eikonal correspondence breaks at low l.
**Cross-domain wiring:** Lyapunov language directly maps to control-numerical-opt stability theory.
**Notes:** Cardoso, Miranda, Berti, Witek, Zanchin (2009).

### gw-parameter-estimation (cross-domain alias: `gw-pe`, `bilby-lalinference`)
**Domain:** Astrophysics / Cosmology
**Definition:** Bayesian inference of source parameters (M_c, q, χ_1, χ_2, distance, sky location, inclination) from h(t) using waveform model.
**Atom or composite:** Composite (likelihood + sampler + waveform).
**Cost model:** 10⁶–10⁸ waveform evaluations per event.
**Real wall?** Yes — sampler cost; ROM/heterodyning amortize.
**Cross-domain wiring:** Identical machinery to statistics-probability Bayesian posterior estimation; ml-training surrogate models accelerate.
**Notes:** Veitch et al.; Bilby; LALInference; RIFT.

### mcmc-for-gw (cross-domain alias: `mcmc-pe`, `emcee-gw`)
**Domain:** Astrophysics / Cosmology
**Definition:** Metropolis-Hastings or parallel tempered chains exploring GW posterior; jumps tuned for ~10-100 parameter spaces.
**Atom or composite:** Composite.
**Cost model:** 10⁶–10⁸ samples; days on cluster per event.
**Real wall?** Yes — multimodal posteriors stall chains.
**Cross-domain wiring:** Same MCMC primitives as statistics-probability; tempering shared across ml-training.
**Notes:** van der Sluys et al.; Christensen-Meyer review.

### nested-sampling-gw (cross-domain alias: `multinest-gw`, `dynesty-gw`)
**Domain:** Astrophysics / Cosmology
**Definition:** Nested sampling estimates Bayesian evidence and posterior simultaneously by shrinking iso-likelihood contours.
**Atom or composite:** Composite.
**Cost model:** O(N_live × N_iter); competitive with MCMC for evidence.
**Real wall?** Yes — efficient prior sampling required.
**Cross-domain wiring:** Direct port from statistics-probability; same library powers cosmology inference.
**Notes:** Skilling (2004); Feroz; Higson.

### matched-filtering-gw (cross-domain alias: `mf-search`, `inspiral-search`)
**Domain:** Astrophysics / Cosmology
**Definition:** Optimal linear detection statistic ρ = ⟨d|h⟩ / √⟨h|h⟩; template bank covers parameter space.
**Atom or composite:** Composite (template + noise PSD + inner product).
**Cost model:** O(N_template × N_segment) FFTs; GPU-friendly.
**Real wall?** Yes — template-bank size scales with mismatch budget.
**Cross-domain wiring:** Identical to signal-processing-rf matched filtering; PSD estimation shared with statistics-probability.
**Notes:** Wainstein-Zubakov (1962); Allen, Anderson, Brady, Brown.

### ligo-virgo-kagra (cross-domain alias: `lvk-network`, `gw-network`)
**Domain:** Astrophysics / Cosmology
**Definition:** Global second-generation interferometric GW detector network; 4-km/3-km arms; 10⁻²³ /√Hz strain sensitivity.
**Atom or composite:** Composite (instrument + pipeline + catalog).
**Cost model:** O3/O4 catalogs (~100 events) require pipeline 24/7 operation.
**Real wall?** Yes — quantum, thermal, seismic noise floors.
**Cross-domain wiring:** Interferometry shared with photonics-optics; control loops with control-numerical-opt; noise modeling with statistics-probability.
**Notes:** GWTC catalogs.

### lisa (cross-domain alias: `space-gw`, `mhz-band`)
**Domain:** Astrophysics / Cosmology
**Definition:** Laser Interferometer Space Antenna; 2.5 Gm arms, 0.1 mHz–0.1 Hz band, target massive BH binaries, EMRIs, galactic binaries.
**Atom or composite:** Composite.
**Cost model:** TDI (time-delay interferometry) algorithm; data 2030s.
**Real wall?** Yes — confusion noise from galactic WD binaries.
**Cross-domain wiring:** TDI is signal-processing-rf data combination; foreground subtraction = stats sub-pop modeling.
**Notes:** ESA L3 mission; Amaro-Seoane et al. red book.

### pta (cross-domain alias: `pulsar-timing-array`)
**Domain:** Astrophysics / Cosmology
**Definition:** Array of millisecond pulsars whose timing residuals are cross-correlated to detect nanohertz GW; Hellings-Downs curve as smoking gun.
**Atom or composite:** Composite.
**Cost model:** Multi-decade timing campaigns.
**Real wall?** Yes — ISM noise, jitter, intrinsic spin noise.
**Cross-domain wiring:** Cross-correlation across array is signal-processing-rf beamforming analog; Hellings-Downs is a quadrupole signature.
**Notes:** Sazhin (1978); Hellings-Downs (1983); NANOGrav 15yr (2023).

### nanohertz-gw-background (cross-domain alias: `nhz-sgwb`, `smbhb-background`)
**Domain:** Astrophysics / Cosmology
**Definition:** Stochastic GW background at 1–100 nHz, plausibly from inspiraling supermassive BH binaries; spectral index −2/3 prediction.
**Atom or composite:** Composite.
**Cost model:** PTA cross-correlation analysis.
**Real wall?** Yes — common-spectrum vs. true HD correlation distinguishability.
**Cross-domain wiring:** Stochastic spectrum estimation is statistics-probability cross-spectral density.
**Notes:** Phinney (2001); NANOGrav, EPTA, PPTA, IPTA.

### stochastic-gw-background (cross-domain alias: `sgwb`, `omega_gw`)
**Domain:** Astrophysics / Cosmology
**Definition:** Energy density per logarithmic frequency Ω_GW(f) = (1/ρ_c) dρ_GW/d ln f; sources: cosmological + astrophysical.
**Atom or composite:** Composite.
**Cost model:** Cross-correlation between detector pairs; long-integration.
**Real wall?** Yes — overlap reduction functions, correlated noise.
**Cross-domain wiring:** Energy spectrum bookkeeping reuses signal-processing-rf PSD definitions.
**Notes:** Allen-Romano (1999); Christensen review.

## 4. Cosmology Framework

### flrw-metric (cross-domain alias: `flrw`, `friedmann-metric`)
**Domain:** Astrophysics / Cosmology
**Definition:** ds² = −c²dt² + a(t)²[dr²/(1−kr²) + r²dΩ²]; homogeneous isotropic spacetime, scale factor a(t), curvature k∈{−1,0,+1}.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — assumes cosmological principle; not exact in real universe.
**Cross-domain wiring:** Conformal mapping shared with photonics-optics waveguide rescaling; perturbations on FLRW are the standard cosmology background.
**Notes:** Friedmann (1922), Lemaître (1927), Robertson-Walker (1935-36).

### cosmological-principle (cross-domain alias: `homogeneity-isotropy`)
**Domain:** Astrophysics / Cosmology
**Definition:** Universe is statistically homogeneous and isotropic on large (≳100 Mpc) scales.
**Atom or composite:** Atom (axiom).
**Cost model:** Tested via N-point statistics.
**Real wall?** Statistical; large-scale anomalies challenge it.
**Cross-domain wiring:** Stationarity assumption is statistics-probability ergodic-theorem analog.
**Notes:** Milne (1933); modern tests with SDSS, DES.

### friedmann-equations (cross-domain alias: `friedmann-eq`, `h²-eq`)
**Domain:** Astrophysics / Cosmology
**Definition:** H² = (8πG/3)ρ − kc²/a² + Λc²/3 ; ä/a = −(4πG/3)(ρ + 3p/c²) + Λc²/3.
**Atom or composite:** Composite (FLRW + Einstein eq + perfect fluid).
**Cost model:** ODE in a(t); algebraic given EOS.
**Real wall?** Yes — flatness, horizon, monopole problems → inflation.
**Cross-domain wiring:** ODE form shared with control-numerical-opt; energy-density bookkeeping is statistics-probability accounting.
**Notes:** Friedmann (1922).

### raychaudhuri-eq (cross-domain alias: `expansion-evolution`)
**Domain:** Astrophysics / Cosmology
**Definition:** dθ/dτ = −θ²/3 − σ_μν σ^μν + ω_μν ω^μν − R_μν u^μ u^ν; evolution of geodesic congruence expansion θ.
**Atom or composite:** Atom.
**Cost model:** ODE along congruence.
**Real wall?** Yes — singularity theorems via focusing.
**Cross-domain wiring:** Same focusing logic as photonics-optics ray bundles; entropy-of-congruence connects to statistics-probability.
**Notes:** Raychaudhuri (1955); Hawking-Penrose theorems.

### conformal-time (cross-domain alias: `η-time`, `τ-conformal`)
**Domain:** Astrophysics / Cosmology
**Definition:** η = ∫dt/a(t); light cones become 45° lines in (η,χ) coordinates.
**Atom or composite:** Atom.
**Cost model:** Integral.
**Real wall?** No.
**Cross-domain wiring:** Standard time-rescaling reused in signal-processing-rf (time warping) and photonics-optics (optical-path time).
**Notes:** Standard in inflation and CMB physics.

### comoving-distance (cross-domain alias: `χ-comoving`, `d_c`)
**Domain:** Astrophysics / Cosmology
**Definition:** D_C(z) = c ∫₀^z dz'/H(z'); distance unaffected by Hubble expansion.
**Atom or composite:** Atom.
**Cost model:** 1D integral.
**Real wall?** No.
**Cross-domain wiring:** Used as the spatial coordinate in N-body sims; same integral pattern as control-numerical-opt path-length.
**Notes:** Hogg (1999) distance review.

### luminosity-distance (cross-domain alias: `d_l`, `flux-distance`)
**Domain:** Astrophysics / Cosmology
**Definition:** D_L(z) = (1+z) D_C(z); relates observed flux to intrinsic luminosity: F = L/(4πD_L²).
**Atom or composite:** Composite (D_C + redshift dilation).
**Real wall?** Yes — Etherington reciprocity relates to D_A in vacuum.
**Cross-domain wiring:** Inverse-square law shared with photonics-optics; SNe Ia standard candles drive H₀ tension analysis.
**Notes:** Hogg (1999); Etherington (1933).

### angular-diameter-distance (cross-domain alias: `d_a`, `da-distance`)
**Domain:** Astrophysics / Cosmology
**Definition:** D_A(z) = D_C(z)/(1+z); converts angular size to physical size; D_L = (1+z)² D_A.
**Atom or composite:** Composite.
**Real wall?** Yes — central to BAO and CMB acoustic scale.
**Cross-domain wiring:** Direct usage in lensing primitives; small-angle photonics-optics imaging machinery.
**Notes:** Etherington reciprocity.

### redshift (cross-domain alias: `z`, `cosmological-redshift`)
**Domain:** Astrophysics / Cosmology
**Definition:** 1+z = a(t_obs)/a(t_emit) = λ_obs/λ_emit; stretching of wavelength with expansion.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — combines cosmological + Doppler + gravitational contributions.
**Cross-domain wiring:** Doppler shift shared with signal-processing-rf; gravitational redshift connects back to GR (clock-rate ratio).
**Notes:** Slipher (1917); Hubble (1929).

### deceleration-parameter (cross-domain alias: `q0`, `q_deceleration`)
**Domain:** Astrophysics / Cosmology
**Definition:** q ≡ −aä/ȧ²; q₀ < 0 signals acceleration; in ΛCDM q₀ ≈ −0.55.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — observed q<0 implies dark energy.
**Cross-domain wiring:** Second-derivative diagnostic shared with control-numerical-opt acceleration detection.
**Notes:** Riess et al. (1998), Perlmutter et al. (1999).

### jerk-parameter (cross-domain alias: `j0`, `cosmographic-jerk`)
**Domain:** Astrophysics / Cosmology
**Definition:** j ≡ ⃛a / aH³; constant j=1 in ΛCDM; deviations test alternative DE.
**Atom or composite:** Atom (third-derivative diagnostic).
**Cost model:** Requires high-z SNe + BAO.
**Real wall?** Yes — currently weakly constrained.
**Cross-domain wiring:** Higher-derivative diagnostics shared with control-numerical-opt jerk-minimal trajectories.
**Notes:** Visser cosmography series.

### lcdm-parameters (cross-domain alias: `λcdm-6par`, `planck-6par`)
**Domain:** Astrophysics / Cosmology
**Definition:** Six-parameter concordance model: Ω_b h², Ω_c h², 100θ_*, τ, A_s, n_s.
**Atom or composite:** Composite.
**Cost model:** Inference cost dominated by Boltzmann + likelihood.
**Real wall?** Yes — H₀ and σ₈ tensions challenge concordance.
**Cross-domain wiring:** Hyperparameter set is statistics-probability prior space; sampler shared with ml-training.
**Notes:** Planck 2018; ACT, SPT.

### dark-energy-eos (cross-domain alias: `w(z)`, `de-equation-of-state`)
**Domain:** Astrophysics / Cosmology
**Definition:** Equation of state w = p/ρ for dark energy; Λ ⇒ w=−1; CPL parametrization w(a)=w_0+w_a(1−a).
**Atom or composite:** Composite (CPL or quintessence model).
**Cost model:** Adds 1–2 params to inference.
**Real wall?** Yes — DESI 2024 hints at evolving w.
**Cross-domain wiring:** Time-varying constitutive parameter; analog to time-varying impedance in electromagnetics-antennas.
**Notes:** Chevallier-Polarski (2001); Linder (2003); DESI Y1 2024.

### desi-boss-bao (cross-domain alias: `bao-survey`, `desi-ruler`)
**Domain:** Astrophysics / Cosmology
**Definition:** Spectroscopic survey measurement of BAO peak in galaxy 2pt function gives standard ruler r_d, constraining D_A(z), H(z).
**Atom or composite:** Composite (galaxy catalog + 2pt + template).
**Cost model:** ~10⁶–10⁷ galaxies; covariance estimation expensive.
**Real wall?** Yes — non-linear smearing biases peak position by ~few Mpc.
**Cross-domain wiring:** Peak-finding in correlation function reuses signal-processing-rf peak detection.
**Notes:** Eisenstein et al. (2005); BOSS, eBOSS, DESI.

### cmb-anisotropy (cross-domain alias: `cmb-ΔT/T`)
**Domain:** Astrophysics / Cosmology
**Definition:** Temperature fluctuations ΔT/T ~ 10⁻⁵ across sky, decomposed into spherical harmonics a_lm with power spectrum C_l.
**Atom or composite:** Composite.
**Cost model:** HEALPix spherical-harmonic transform O(N^(3/2)).
**Real wall?** Yes — cosmic variance limits low-l precision.
**Cross-domain wiring:** Spherical-harmonic decomposition shared with electromagnetics-antennas (antenna patterns), photonics-optics (Zernike-like modes).
**Notes:** Penzias-Wilson (1965); COBE, WMAP, Planck.

### sachs-wolfe (cross-domain alias: `sw-effect`)
**Domain:** Astrophysics / Cosmology
**Definition:** ΔT/T = (1/3) Ψ at last scattering due to gravitational potential differences; dominates large-scale CMB.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — combined with intrinsic adiabatic + Doppler at recombination.
**Cross-domain wiring:** Potential-to-temperature mapping is photonics-optics gravitational redshift analog.
**Notes:** Sachs & Wolfe (1967).

### integrated-sachs-wolfe (cross-domain alias: `isw`, `late-isw`)
**Domain:** Astrophysics / Cosmology
**Definition:** ΔT/T = 2∫dη ∂Ψ/∂η along line of sight; nonzero when potentials evolve (Λ-dominated era, curvature).
**Atom or composite:** Composite (line-of-sight integral).
**Cost model:** Boltzmann hierarchy or LOS integration.
**Real wall?** Yes — cross-correlation with LSS dark-energy probe.
**Cross-domain wiring:** Line-of-sight integration shared with photonics-optics (Radon transform).
**Notes:** Crittenden-Turok (1996); Boughn-Crittenden detections.

### doppler-peak-cmb (cross-domain alias: `velocity-cmb`)
**Domain:** Astrophysics / Cosmology
**Definition:** Photon-baryon fluid velocity contributes ΔT/T = n̂·v_b at last scattering; π/2 phase-shifted from density peaks.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — contributes to acoustic peak structure.
**Cross-domain wiring:** Standard Doppler effect shared with signal-processing-rf; phase relation analogous to LC-circuit resonance.
**Notes:** Hu-Sugiyama review.

### cmb-polarization-e-b (cross-domain alias: `e-modes-b-modes`)
**Domain:** Astrophysics / Cosmology
**Definition:** CMB Stokes Q,U decompose into curl-free E-modes (scalar perturbations) and divergence-free B-modes (tensor / lensing).
**Atom or composite:** Composite.
**Cost model:** Spin-2 spherical-harmonic transform.
**Real wall?** Yes — primordial B-modes signal inflation; foreground dust dominates.
**Cross-domain wiring:** Polarization decomposition shared with electromagnetics-antennas (TE/TM), photonics-optics (Stokes).
**Notes:** Kamionkowski et al., Seljak-Zaldarriaga (1997).

### cmb-power-spectrum-cl (cross-domain alias: `c_l`, `tt-te-ee-bb`)
**Domain:** Astrophysics / Cosmology
**Definition:** Angular power spectrum C_l = ⟨|a_lm|²⟩; primary observable for ΛCDM inference.
**Atom or composite:** Composite (a_lm + variance estimator).
**Cost model:** Pseudo-Cl O(l_max³); MASTER deconvolution.
**Real wall?** Yes — cosmic variance, mask-coupling, beam.
**Cross-domain wiring:** Periodogram analog of signal-processing-rf PSD; window/mask deconvolution is signal-processing-rf staple.
**Notes:** Hivon et al. (2002) MASTER.

### acoustic-peaks (cross-domain alias: `cmb-peaks`)
**Domain:** Astrophysics / Cosmology
**Definition:** Series of peaks at l ≈ nπ D_A / r_s in CMB C_l from baryon-photon acoustic oscillations.
**Atom or composite:** Composite.
**Cost model:** Boltzmann code (CAMB/CLASS).
**Real wall?** Yes — first peak constrains Ω_total; ratio constrains Ω_b.
**Cross-domain wiring:** Resonance structure shared with signal-processing-rf cavity modes.
**Notes:** Hu & White (1996); Doroshkevich, Zel'dovich, Sunyaev.

### silk-damping (cross-domain alias: `diffusion-damping`)
**Domain:** Astrophysics / Cosmology
**Definition:** Exponential damping of small-scale CMB anisotropies due to photon diffusion through recombining baryons; damping scale ~ √(λ_C η).
**Atom or composite:** Atom.
**Cost model:** Algebraic / Boltzmann.
**Real wall?** Yes — sets high-l cutoff of CMB.
**Cross-domain wiring:** Diffusion damping is physics-diffusion exponential attenuation; analog to skin depth in electromagnetics-antennas.
**Notes:** Silk (1968).

### reionization-tau (cross-domain alias: `τ-reion`, `optical-depth-reion`)
**Domain:** Astrophysics / Cosmology
**Definition:** CMB photon scattering optical depth from reionization τ ≈ 0.054 (Planck); damps high-l TT, sources large-l EE.
**Atom or composite:** Atom.
**Cost model:** Single parameter in CAMB/CLASS.
**Real wall?** Yes — degenerate with A_s in CMB-only inference.
**Cross-domain wiring:** Optical-depth bookkeeping shared with photonics-optics and signal-processing-rf attenuation models.
**Notes:** Planck 2018 reionization paper.

## 5. Inflation

### slow-roll-inflation (cross-domain alias: `slow-roll`, `sr-inflation`)
**Domain:** Astrophysics / Cosmology
**Definition:** Quasi-de Sitter expansion driven by scalar inflaton rolling slowly down potential V(φ); φ̈ ≪ 3Hφ̇.
**Atom or composite:** Composite (scalar field + Hubble friction).
**Cost model:** ODE in (φ,φ̇,a); algebraic in SR limit.
**Real wall?** Yes — sets prediction n_s ≠ 1, small r.
**Cross-domain wiring:** Overdamped scalar ODE shared with control-numerical-opt (gradient descent with momentum).
**Notes:** Guth (1981), Linde, Albrecht-Steinhardt.

### slow-roll-epsilon (cross-domain alias: `ε-sr`, `kinetic-sr`)
**Domain:** Astrophysics / Cosmology
**Definition:** ε = (M_P²/2)(V'/V)² ≪ 1; first SR parameter; ends inflation when ε≈1.
**Atom or composite:** Atom.
**Cost model:** Algebraic from V(φ).
**Real wall?** Yes — controls tensor amplitude r ≈ 16ε.
**Cross-domain wiring:** Gradient-magnitude diagnostic shared with ml-training learning-rate analysis.
**Notes:** Liddle-Parsons-Barrow (1994).

### slow-roll-eta (cross-domain alias: `η-sr`, `curvature-sr`)
**Domain:** Astrophysics / Cosmology
**Definition:** η = M_P² V''/V; controls scalar tilt n_s − 1 ≈ 2η − 6ε.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — observed n_s ≈ 0.965.
**Cross-domain wiring:** Curvature-of-loss analog in ml-training; Hessian diagnostic.
**Notes:** Standard SR formalism.

### e-folds (cross-domain alias: `n-efolds`, `n_e`)
**Domain:** Astrophysics / Cosmology
**Definition:** N = ln(a_end/a) = ∫H dt ≈ ∫(V/V')dφ/M_P²; need N ≳ 50–60 to solve horizon/flatness.
**Atom or composite:** Atom.
**Cost model:** Integral.
**Real wall?** Yes — observable scales exit horizon at N ≈ 50–60.
**Cross-domain wiring:** Log-scale unit shared with signal-processing-rf (decade), ml-training (log-loss).
**Notes:** Liddle-Lyth review.

### scalar-perturbation-spectrum (cross-domain alias: `p_ζ`, `curvature-power`)
**Domain:** Astrophysics / Cosmology
**Definition:** P_ζ(k) = (H²/8π²M_P²ε)|_(k=aH); amplitude A_s ≈ 2.1×10⁻⁹, tilt n_s.
**Atom or composite:** Composite (SR + mode quantization).
**Cost model:** Mode-by-mode ODE integration in numerical pipelines.
**Real wall?** Yes — observed by Planck/ACT/SPT.
**Cross-domain wiring:** Vacuum-mode quantization is signal-processing-rf white-noise-driven oscillator.
**Notes:** Mukhanov-Sasaki equation.

### tensor-to-scalar-ratio (cross-domain alias: `r-ratio`)
**Domain:** Astrophysics / Cosmology
**Definition:** r ≡ P_T/P_ζ ≈ 16ε; sets primordial GW amplitude; r<0.036 (BICEP/Keck 2021).
**Atom or composite:** Atom.
**Cost model:** Single parameter in inflation fits.
**Real wall?** Yes — bounds inflation scale V^(1/4) < 1.6×10¹⁶ GeV.
**Cross-domain wiring:** Tensor-to-scalar power ratio shared with electromagnetics-antennas polarization power partitioning.
**Notes:** Lyth bound; BK18.

### spectral-index (cross-domain alias: `n_s`, `tilt`)
**Domain:** Astrophysics / Cosmology
**Definition:** n_s − 1 ≡ d ln P_ζ / d ln k ≈ 2η − 6ε; red tilt confirms inflation.
**Atom or composite:** Atom.
**Cost model:** Inference parameter.
**Real wall?** Yes — n_s = 0.9649 ± 0.0042 (Planck 2018).
**Cross-domain wiring:** Power-law spectral index reuses signal-processing-rf 1/f-noise framework.
**Notes:** Planck 2018.

### spectral-running (cross-domain alias: `α_s`, `running-of-running`)
**Domain:** Astrophysics / Cosmology
**Definition:** α_s = dn_s/d ln k; predicted small in single-field SR; large α_s would falsify minimal models.
**Atom or composite:** Atom.
**Cost model:** Adds 1 parameter.
**Real wall?** Yes — currently consistent with zero.
**Cross-domain wiring:** Higher-order spectral diagnostics shared with signal-processing-rf (bispectrum slope).
**Notes:** Kosowsky-Turner (1995).

### non-gaussianity (cross-domain alias: `f_nl`, `bispectrum-amplitude`)
**Domain:** Astrophysics / Cosmology
**Definition:** Φ = Φ_G + f_NL(Φ_G² − ⟨Φ_G²⟩) parametrizes deviation from Gaussian primordial perturbations; bispectrum amplitude.
**Atom or composite:** Composite.
**Cost model:** Bispectrum estimator O(N^(3/2)) per ℓ.
**Real wall?** Yes — Planck f_NL ~ 0±5 strongly constrains multi-field.
**Cross-domain wiring:** Higher-order statistics shared with signal-processing-rf bispectrum, ml-training generative-model evaluation.
**Notes:** Komatsu-Spergel (2001); Planck NG 2019.

### fnl-equilateral (cross-domain alias: `f_nl-eq`)
**Domain:** Astrophysics / Cosmology
**Definition:** Bispectrum shape peaking at k₁≈k₂≈k₃; signals derivative interactions / DBI inflation.
**Atom or composite:** Atom (template).
**Cost model:** Same as NG estimator.
**Real wall?** Yes — Planck f_NL^eq = −26 ± 47.
**Cross-domain wiring:** Template-matching in 3D k-space shared with signal-processing-rf 3D filter banks.
**Notes:** Creminelli et al. (2006).

### fnl-orthogonal (cross-domain alias: `f_nl-orth`)
**Domain:** Astrophysics / Cosmology
**Definition:** Orthogonal-to-equilateral shape; arises in EFT of inflation with specific operator combinations.
**Atom or composite:** Atom.
**Cost model:** Same as NG estimator.
**Real wall?** Yes — distinguishes inflation models.
**Cross-domain wiring:** Same orthonormal-template machinery as signal-processing-rf basis sets.
**Notes:** Senatore-Smith-Zaldarriaga.

### fnl-local (cross-domain alias: `f_nl-loc`)
**Domain:** Astrophysics / Cosmology
**Definition:** Squeezed-limit bispectrum amplitude; nonzero local f_NL falsifies single-field inflation (Maldacena consistency).
**Atom or composite:** Atom.
**Cost model:** Squeezed-bispectrum estimator.
**Real wall?** Yes — Planck f_NL^loc = −0.9 ± 5.1.
**Cross-domain wiring:** Squeezed-limit theorem analog to fluctuation-dissipation in statistics-probability.
**Notes:** Maldacena (2002); single-field consistency.

### g_nl (cross-domain alias: `trispectrum-amplitude`)
**Domain:** Astrophysics / Cosmology
**Definition:** Cubic non-Gaussianity coefficient; trispectrum-level test of multi-field inflation.
**Atom or composite:** Atom.
**Cost model:** Trispectrum estimator O(N²) per ℓ.
**Real wall?** Yes — much weaker constraints than f_NL.
**Cross-domain wiring:** Fourth-order cumulants shared with signal-processing-rf higher-order spectra.
**Notes:** Okamoto-Hu, Smith et al.

### reheating (cross-domain alias: `inflaton-decay`, `reheat-T_rh`)
**Domain:** Astrophysics / Cosmology
**Definition:** Conversion of inflaton energy into SM particles after slow-roll ends; reheating temperature T_rh sets BBN initial conditions.
**Atom or composite:** Composite (decay rates + thermalization).
**Cost model:** Boltzmann equations for particle species.
**Real wall?** Yes — T_rh > few MeV for BBN; upper bound from gravitino.
**Cross-domain wiring:** Reaction-network ODE pattern reused in physics-diffusion combustion.
**Notes:** Kofman-Linde-Starobinsky.

### preheating (cross-domain alias: `parametric-resonance`)
**Domain:** Astrophysics / Cosmology
**Definition:** Non-perturbative explosive energy transfer via parametric resonance during inflaton oscillations.
**Atom or composite:** Composite.
**Cost model:** Floquet analysis; lattice simulations.
**Real wall?** Yes — produces GW background, can seed defects.
**Cross-domain wiring:** Floquet/Mathieu equation shared with photonics-optics (parametric amplifiers) and control-numerical-opt resonant control.
**Notes:** Kofman-Linde-Starobinsky (1994).

### eternal-inflation (cross-domain alias: `stochastic-inflation`)
**Domain:** Astrophysics / Cosmology
**Definition:** Regions where quantum fluctuations dominate classical roll keep inflating forever; produces self-reproducing universe.
**Atom or composite:** Composite (Langevin SDE for φ).
**Cost model:** Stochastic PDE.
**Real wall?** Open theoretical issue (measure problem).
**Cross-domain wiring:** Langevin formalism direct from statistics-probability; same SDE machinery as physics-diffusion.
**Notes:** Linde (1986); Starobinsky stochastic inflation.

### multiverse-measure (cross-domain alias: `measure-problem`)
**Domain:** Astrophysics / Cosmology
**Definition:** Choice of probability measure across pocket universes in eternal inflation needed to make predictions.
**Atom or composite:** Composite.
**Cost model:** Theoretical; many proposals.
**Real wall?** Yes — measure ambiguity hampers predictions.
**Cross-domain wiring:** Measure-on-paths shared with statistics-probability path integrals.
**Notes:** Vilenkin, Linde, Bousso, Susskind.

## 6. Perturbation Theory & Structure Formation

### linear-perturbation-theory (cross-domain alias: `lpt`, `linear-cosmology`)
**Domain:** Astrophysics / Cosmology
**Definition:** Treat density contrast δ = δρ/ρ as small; modes evolve independently per Fourier component.
**Atom or composite:** Atom.
**Cost model:** ODE per mode; fast.
**Real wall?** Yes — breaks at δ ~ 1.
**Cross-domain wiring:** Same linear-PDE machinery as signal-processing-rf transfer functions.
**Notes:** Lifshitz (1946), Bardeen (1980).

### growth-factor (cross-domain alias: `d(a)`, `linear-growth`)
**Domain:** Astrophysics / Cosmology
**Definition:** D(a): scale-independent function multiplying initial δ; in EdS D=a; in ΛCDM suppressed by Λ.
**Atom or composite:** Atom.
**Cost model:** 1D ODE; fitting formula D ∝ a g(a).
**Real wall?** No.
**Cross-domain wiring:** Same time-evolution-of-mode pattern as signal-processing-rf channel gain.
**Notes:** Heath (1977); Carroll-Press-Turner formula.

### meszaros-effect (cross-domain alias: `radiation-suppression`)
**Domain:** Astrophysics / Cosmology
**Definition:** Suppression of sub-horizon DM growth during radiation domination; growth logarithmic instead of linear in a.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — imprinted on transfer function below k_eq.
**Cross-domain wiring:** Stagnant-growth phase mirrors physics-diffusion damped regime.
**Notes:** Mészáros (1974).

### transfer-function (cross-domain alias: `t(k)`, `k-mode-shape`)
**Domain:** Astrophysics / Cosmology
**Definition:** T(k) relates primordial P_ζ to linear matter P(k): P(k) = (2π²/k³)(k/k_*)^(n_s−1) T²(k) A_s.
**Atom or composite:** Composite.
**Cost model:** Boltzmann code O(modes × time).
**Real wall?** No.
**Cross-domain wiring:** Direct analog to signal-processing-rf transfer function; freq response of cosmology.
**Notes:** Sugiyama; Eisenstein-Hu fitting formula.

### bbks-transfer (cross-domain alias: `bbks`, `bbks-formula`)
**Domain:** Astrophysics / Cosmology
**Definition:** Bardeen-Bond-Kaiser-Szalay analytic fit to CDM transfer function in absence of baryons.
**Atom or composite:** Atom.
**Cost model:** Closed-form algebraic.
**Real wall?** No — superseded by Eisenstein-Hu when baryons matter.
**Cross-domain wiring:** Curve fitting analog; same role as Padé approximants.
**Notes:** Bardeen, Bond, Kaiser, Szalay (1986).

### eisenstein-hu (cross-domain alias: `eh-formula`)
**Domain:** Astrophysics / Cosmology
**Definition:** Analytic fit to baryon-CDM transfer function including BAO wiggles; matches Boltzmann codes to 5%.
**Atom or composite:** Atom.
**Cost model:** Closed-form.
**Real wall?** No.
**Cross-domain wiring:** Empirical fit pattern shared with ml-training surrogate models.
**Notes:** Eisenstein & Hu (1998).

### press-schechter (cross-domain alias: `ps-mass-function`)
**Domain:** Astrophysics / Cosmology
**Definition:** n(M)dM = (ρ̄/M)|d ln σ/d ln M| (2/π)^(1/2) (δ_c/σ) exp(−δ_c²/2σ²) dM/M.
**Atom or composite:** Composite (variance σ(M) + threshold δ_c).
**Cost model:** Closed-form given σ(M).
**Real wall?** Yes — underpredicts massive haloes.
**Cross-domain wiring:** Threshold-crossing statistics shared with statistics-probability extreme-value theory.
**Notes:** Press & Schechter (1974).

### sheth-tormen (cross-domain alias: `st-mass-function`)
**Domain:** Astrophysics / Cosmology
**Definition:** Ellipsoidal-collapse refinement of PS mass function; better match to N-body sims.
**Atom or composite:** Composite.
**Cost model:** Closed-form.
**Real wall?** Yes — empirical parameters tuned to sims.
**Cross-domain wiring:** Empirical correction pattern shared with statistics-probability bias-correction.
**Notes:** Sheth & Tormen (1999); Sheth, Mo, Tormen (2001).

### halo-mass-function (cross-domain alias: `hmf`, `dn/dm`)
**Domain:** Astrophysics / Cosmology
**Definition:** Differential comoving number density of dark-matter halos as function of mass; cornerstone observable for cluster cosmology.
**Atom or composite:** Composite.
**Cost model:** PS/ST/Tinker fits or N-body calibration.
**Real wall?** Yes — sensitive to σ₈, Ω_m.
**Cross-domain wiring:** Counting statistic shared with statistics-probability event-counting; cluster counts = ml-training feature engineering for cosmology.
**Notes:** Tinker et al. (2008).

### spherical-collapse (cross-domain alias: `sc-model`, `top-hat-collapse`)
**Domain:** Astrophysics / Cosmology
**Definition:** Toy model: uniform-density sphere overdensity grows linearly until turnaround, then collapses; δ_c ≈ 1.686.
**Atom or composite:** Composite.
**Cost model:** ODE.
**Real wall?** Yes — gives threshold for halo formation.
**Cross-domain wiring:** Self-gravitating sphere shared with stellar-collapse physics; threshold crossing connects to control-numerical-opt switching.
**Notes:** Gunn-Gott (1972).

### ellipsoidal-collapse (cross-domain alias: `ec-collapse`)
**Domain:** Astrophysics / Cosmology
**Definition:** Anisotropic collapse with three axes collapsing at different times; threshold depends on σ.
**Atom or composite:** Composite.
**Cost model:** ODE on 3 axes.
**Real wall?** Yes — basis for ST mass function.
**Cross-domain wiring:** Zel'dovich pancake / first-axis collapse shared with photonics-optics caustics.
**Notes:** Bond-Myers; Sheth-Mo-Tormen.

### halo-bias (cross-domain alias: `b(m)`, `linear-bias`)
**Domain:** Astrophysics / Cosmology
**Definition:** Linear bias b(M): δ_h = b(M)δ_m on large scales; massive halos cluster more strongly than matter.
**Atom or composite:** Composite.
**Cost model:** Algebraic / N-body fit.
**Real wall?** Yes — nonlinear bias important at small scales.
**Cross-domain wiring:** Source-amplification factor analog to signal-processing-rf gain.
**Notes:** Mo-White (1996); Tinker bias.

### peak-background-split (cross-domain alias: `pbs`, `peak-split`)
**Domain:** Astrophysics / Cosmology
**Definition:** Long-wavelength modes modulate threshold crossings, yielding scale-dependent bias derivation from mass function.
**Atom or composite:** Composite.
**Cost model:** Algebraic given HMF.
**Real wall?** Yes — predicts NG-induced scale-dependent bias.
**Cross-domain wiring:** Threshold-modulation pattern shared with signal-processing-rf adaptive thresholds.
**Notes:** Bardeen-Bond-Kaiser-Szalay; Cole-Kaiser.

### halo-model (cross-domain alias: `1h-2h-decomposition`)
**Domain:** Astrophysics / Cosmology
**Definition:** Decompose matter power spectrum into 1-halo (intra-halo) + 2-halo (inter-halo) terms; uses HMF + halo profile + bias.
**Atom or composite:** Composite.
**Cost model:** Numerical integration; ms-level.
**Real wall?** Yes — transition regime k ~ 0.1 Mpc⁻¹ poorly modeled.
**Cross-domain wiring:** Convolution decomposition shared with signal-processing-rf clutter+target models.
**Notes:** Cooray & Sheth (2002) review.

### hod (cross-domain alias: `halo-occupation`, `n_cen-n_sat`)
**Domain:** Astrophysics / Cosmology
**Definition:** P(N | M_h): probability of N galaxies in halo of mass M_h; central + satellite split.
**Atom or composite:** Composite.
**Cost model:** Inference parameters per survey.
**Real wall?** Yes — assembly bias breaks pure-mass HOD.
**Cross-domain wiring:** Mixture model in statistics-probability; latent-variable structure shared with ml-training.
**Notes:** Berlind-Weinberg (2002); Zheng et al.

### redshift-space-distortions (cross-domain alias: `rsd`, `velocity-distortion`)
**Domain:** Astrophysics / Cosmology
**Definition:** Peculiar velocities distort galaxy positions in z-space along LOS, breaking isotropy; encodes growth rate fσ_8.
**Atom or composite:** Composite.
**Cost model:** Multipole decomposition of ξ(s,μ) or P(k,μ).
**Real wall?** Yes — FoG nonlinearity hampers small-scale RSD.
**Cross-domain wiring:** Velocity-mapped distortion mirrors signal-processing-rf Doppler imaging.
**Notes:** Kaiser (1987); Hamilton (1998).

### kaiser-effect (cross-domain alias: `linear-rsd`)
**Domain:** Astrophysics / Cosmology
**Definition:** Large-scale coherent infall toward overdensities boosts P(k,μ) by (1+βμ²)² with β = f/b.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — exact only in linear regime.
**Cross-domain wiring:** Angular gain pattern shared with electromagnetics-antennas array factor.
**Notes:** Kaiser (1987).

### fingers-of-god (cross-domain alias: `fog`, `virial-rsd`)
**Domain:** Astrophysics / Cosmology
**Definition:** Velocity dispersion within virialized halos elongates clusters along LOS in z-space; suppresses small-scale P(k).
**Atom or composite:** Atom.
**Cost model:** Lorentzian or Gaussian damping model.
**Real wall?** Yes — limits scales usable for cosmology.
**Cross-domain wiring:** Velocity-PDF convolution shared with signal-processing-rf Doppler broadening.
**Notes:** Jackson (1972); Peacock-Dodds.

### bao-standard-ruler (cross-domain alias: `bao-ruler`, `r_d`)
**Domain:** Astrophysics / Cosmology
**Definition:** Comoving sound horizon at drag epoch r_d ≈ 147 Mpc imprinted on galaxy 2pt function; standard ruler.
**Atom or composite:** Composite.
**Cost model:** Template fit to ξ(r) or P(k).
**Real wall?** Yes — non-linear damping shifts peak by ~0.5%.
**Cross-domain wiring:** Reference-scale calibration mirrors photonics-optics interferometric fringe spacing.
**Notes:** Eisenstein et al. (2005).

## 7. Dark Matter

### cold-dark-matter (cross-domain alias: `cdm`)
**Domain:** Astrophysics / Cosmology
**Definition:** Non-relativistic, collisionless, non-baryonic DM with negligible primordial velocity; structure forms hierarchically.
**Atom or composite:** Atom (model class).
**Cost model:** Default for N-body simulations.
**Real wall?** Yes — small-scale tensions (cusp-core, missing satellites) under tension.
**Cross-domain wiring:** Particle-mesh discretization of collisionless Vlasov shared with control-numerical-opt PDE methods.
**Notes:** Peebles, Davis-Efstathiou-Frenk-White.

### warm-dark-matter (cross-domain alias: `wdm`)
**Domain:** Astrophysics / Cosmology
**Definition:** DM with non-negligible primordial velocity (e.g., keV sterile ν); suppresses small-scale power below free-streaming length.
**Atom or composite:** Atom.
**Cost model:** Transfer-function cutoff.
**Real wall?** Yes — Ly-α forest constrains m_WDM > ~3 keV.
**Cross-domain wiring:** Free-streaming cutoff mirrors signal-processing-rf low-pass filter.
**Notes:** Bode-Ostriker-Turok.

### hot-dark-matter (cross-domain alias: `hdm`, `ν-dm`)
**Domain:** Astrophysics / Cosmology
**Definition:** Relativistic DM (e.g., light neutrinos) at decoupling; erases small-scale structure.
**Atom or composite:** Atom.
**Cost model:** Linear theory + N-body for neutrinos.
**Real wall?** Yes — HDM cannot be dominant DM; Σm_ν < 0.12 eV.
**Cross-domain wiring:** Relativistic species treatment in physics-diffusion (radiative transfer).
**Notes:** Bond-Szalay (1983).

### axion-dm (cross-domain alias: `qcd-axion`, `cold-axion`)
**Domain:** Astrophysics / Cosmology
**Definition:** Light pseudo-scalar from PQ symmetry breaking; misalignment + topological mechanisms produce cold-DM-like component.
**Atom or composite:** Composite.
**Cost model:** Lattice / cosmological perturbation pipeline.
**Real wall?** Yes — m_a ~ μeV–meV target experiments.
**Cross-domain wiring:** Cavity/haloscope searches shared with electromagnetics-antennas RF resonator design.
**Notes:** Peccei-Quinn (1977); Weinberg, Wilczek.

### fuzzy-dm (cross-domain alias: `ultralight-dm`, `ψdm`)
**Domain:** Astrophysics / Cosmology
**Definition:** Ultra-light boson (m ~ 10⁻²² eV) with de Broglie wavelength ~kpc; suppresses small-scale structure via wave nature.
**Atom or composite:** Composite (Schrödinger-Poisson system).
**Cost model:** Pseudo-spectral PDE solver.
**Real wall?** Yes — tension with Lyα and UFD.
**Cross-domain wiring:** Schrödinger-Poisson shares numerics with photonics-optics nonlinear Schrödinger.
**Notes:** Hu-Barkana-Gruzinov (2000).

### sidm (cross-domain alias: `self-interacting-dm`)
**Domain:** Astrophysics / Cosmology
**Definition:** DM with non-gravitational self-scattering cross-section σ/m ~ 0.1–10 cm²/g; flattens halo cores.
**Atom or composite:** Composite.
**Cost model:** N-body + Monte-Carlo scattering.
**Real wall?** Yes — clusters bound σ/m at high velocities.
**Cross-domain wiring:** MC scattering kernel shared with physics-diffusion neutron transport.
**Notes:** Spergel-Steinhardt (2000).

### primordial-bh-dm (cross-domain alias: `pbh-dm`)
**Domain:** Astrophysics / Cosmology
**Definition:** Hypothesis that all/part of DM is in primordial BHs formed in early universe; mass windows constrained by lensing, GW, CMB.
**Atom or composite:** Composite.
**Cost model:** Multi-probe inference.
**Real wall?** Yes — only asteroid-mass window remains open.
**Cross-domain wiring:** Microlensing surveys are signal-processing-rf transient detection.
**Notes:** Hawking (1971); Carr review.

### mond (cross-domain alias: `mond-modification`)
**Domain:** Astrophysics / Cosmology
**Definition:** Modified Newtonian dynamics replaces Newton's law below acceleration a_0 ≈ 1.2×10⁻¹⁰ m/s² to explain rotation curves without DM.
**Atom or composite:** Composite.
**Cost model:** Nonlinear Poisson-like equation.
**Real wall?** Yes — fails on cluster scales; needs relativistic extension.
**Cross-domain wiring:** Nonlinear constitutive law analog in photonics-optics nonlinear media.
**Notes:** Milgrom (1983).

### teves (cross-domain alias: `teves-theory`)
**Domain:** Astrophysics / Cosmology
**Definition:** Tensor-Vector-Scalar relativistic extension of MOND by Bekenstein (2004); supplies cosmological framework.
**Atom or composite:** Composite.
**Cost model:** Multi-field PDE.
**Real wall?** Yes — bullet cluster, GW170817 disfavor.
**Cross-domain wiring:** Multi-field gravity tooling shared with quintessence in cosmology pipelines.
**Notes:** Bekenstein (2004); Skordis post-GW170817.

### dm-direct-detection (cross-domain alias: `dd-rate`, `nuclear-recoil`)
**Domain:** Astrophysics / Cosmology
**Definition:** Event rate dR/dE_R = (ρ_χ/m_χ)∫f(v)(dσ/dE_R)v dv; nuclear/electron recoil signal.
**Atom or composite:** Composite (halo model + cross-section + detector).
**Cost model:** Integral over velocity distribution.
**Real wall?** Yes — solar/atmospheric ν floor.
**Cross-domain wiring:** Recoil energy spectrometry shared with signal-processing-rf pulse-height analysis.
**Notes:** Lewin-Smith (1996); Goodman-Witten.

### dm-indirect-detection (cross-domain alias: `χχ→γγ`, `indirect-search`)
**Domain:** Astrophysics / Cosmology
**Definition:** Detection of DM annihilation/decay products: γ-rays, ν, e±, p̄; rate ∝ ρ_DM² ⟨σv⟩.
**Atom or composite:** Composite.
**Cost model:** J-factor integrals over targets.
**Real wall?** Yes — astrophysical backgrounds dominate.
**Cross-domain wiring:** J-factor LOS integration is photonics-optics radiative transfer.
**Notes:** Bergström review; Fermi-LAT dSph limits.

### nfw-profile (cross-domain alias: `nfw`, `r_s-nfw`)
**Domain:** Astrophysics / Cosmology
**Definition:** ρ(r) = ρ_s / [(r/r_s)(1+r/r_s)²]; universal CDM halo profile from N-body sims.
**Atom or composite:** Atom.
**Cost model:** Closed-form.
**Real wall?** Yes — cusp at small r vs. observed cores.
**Cross-domain wiring:** Universal-profile fitting pattern reused across photonics-optics PSF models.
**Notes:** Navarro-Frenk-White (1997).

### einasto-profile (cross-domain alias: `einasto`)
**Domain:** Astrophysics / Cosmology
**Definition:** ρ(r) ∝ exp[−(2/α)((r/r_s)^α − 1)]; better fit than NFW for high-resolution sims.
**Atom or composite:** Atom.
**Cost model:** Closed-form.
**Real wall?** No.
**Cross-domain wiring:** Stretched-exponential shared with photonics-optics super-Gaussian beams.
**Notes:** Einasto (1965); Navarro et al. (2010).

### burkert-profile (cross-domain alias: `burkert-core`)
**Domain:** Astrophysics / Cosmology
**Definition:** ρ(r) = ρ_0 r_0³ / [(r+r_0)(r²+r_0²)]; cored profile fitting dwarf galaxy rotation curves.
**Atom or composite:** Atom.
**Cost model:** Closed-form.
**Real wall?** Yes — empirical; tied to core-cusp tension.
**Cross-domain wiring:** Cored profile = regularized inverse-power-law, common pattern in signal-processing-rf.
**Notes:** Burkert (1995).

### isothermal-halo (cross-domain alias: `siscore`, `iso-profile`)
**Domain:** Astrophysics / Cosmology
**Definition:** ρ(r) ∝ 1/r²; produces flat rotation curve; idealized model.
**Atom or composite:** Atom.
**Cost model:** Closed-form.
**Real wall?** Yes — diverges at large r, not a true equilibrium.
**Cross-domain wiring:** 1/r² potential shared with electromagnetics-antennas wire-current fields.
**Notes:** Standard galactic dynamics text.

### hernquist-profile (cross-domain alias: `hernquist`)
**Domain:** Astrophysics / Cosmology
**Definition:** ρ(r) = (M/2π)(a/r)/(r+a)³; analytic profile with finite total mass; matches de Vaucouleurs surface brightness.
**Atom or composite:** Atom.
**Cost model:** Closed-form distribution function known.
**Real wall?** No.
**Cross-domain wiring:** Closed-form DF pattern reused in stellar dynamics primitives.
**Notes:** Hernquist (1990).

## 8. N-body & Hydrodynamics Simulations

### barnes-hut-tree (cross-domain alias: `bh-tree`, `tree-code`)
**Domain:** Astrophysics / Cosmology
**Definition:** Hierarchical octree where distant cell groups are approximated by multipole; force evaluation O(N log N) at fixed accuracy.
**Atom or composite:** Composite.
**Cost model:** O(N log N).
**Real wall?** Yes — opening angle θ controls accuracy.
**Cross-domain wiring:** Spatial-tree approximation shared with ml-training KD-trees, signal-processing-rf hierarchical clustering.
**Notes:** Barnes & Hut (1986).

### particle-mesh (cross-domain alias: `pm-solver`)
**Domain:** Astrophysics / Cosmology
**Definition:** Mass assigned to grid via CIC/TSC, Poisson solved via FFT, forces interpolated to particles; O(N + N_g log N_g).
**Atom or composite:** Composite.
**Cost model:** O(N + N_g³ log N_g).
**Real wall?** Yes — force resolution limited to grid scale.
**Cross-domain wiring:** Same FFT-Poisson core as electromagnetics-antennas PIC, photonics-optics BPM.
**Notes:** Hockney-Eastwood (1981).

### p3m (cross-domain alias: `particle-particle-particle-mesh`)
**Domain:** Astrophysics / Cosmology
**Definition:** Hybrid PM for long-range + direct PP for short-range; combines O(N log N) global + O(N_pp) local.
**Atom or composite:** Composite (PM + PP).
**Cost model:** Better than naive PM for small-scale resolution.
**Real wall?** Yes — load balancing in PP.
**Cross-domain wiring:** Multi-scale decomposition shared with Ewald summation in physics-diffusion.
**Notes:** Hockney-Eastwood; Couchman P³M.

### tree-pm (cross-domain alias: `treepm`, `gadget-treepm`)
**Domain:** Astrophysics / Cosmology
**Definition:** PM for long-range, tree for short-range; standard in modern cosmological N-body codes.
**Atom or composite:** Composite.
**Cost model:** O(N log N) with smaller constant than pure tree.
**Real wall?** Yes — clean separation requires Ewald-like window.
**Cross-domain wiring:** Windowed-spectral + local solver pattern reused in signal-processing-rf STFT.
**Notes:** Bagla (2002); GADGET-2/4.

### fmm-gravity (cross-domain alias: `fast-multipole`)
**Domain:** Astrophysics / Cosmology
**Definition:** O(N) force evaluation via multipole expansions of source cells + local expansions at target cells.
**Atom or composite:** Composite.
**Cost model:** O(N) theoretical; higher constants than tree.
**Real wall?** Yes — implementation complexity, memory.
**Cross-domain wiring:** Same FMM library as electromagnetics-antennas Helmholtz/MoM.
**Notes:** Greengard-Rokhlin (1987); Bonsai, ChaNGa.

### fft-poisson (cross-domain alias: `spectral-poisson`)
**Domain:** Astrophysics / Cosmology
**Definition:** Solve ∇²Φ = 4πGρ via FFT(ρ)/k² in periodic box; O(N log N).
**Atom or composite:** Composite.
**Cost model:** FFT dominates.
**Real wall?** Yes — periodic BC; aliasing.
**Cross-domain wiring:** Spectral Poisson reused in physics-diffusion, electromagnetics-antennas.
**Notes:** Hockney-Eastwood.

### multigrid-poisson (cross-domain alias: `mg-poisson`)
**Domain:** Astrophysics / Cosmology
**Definition:** V-cycle smoother+restrict+prolong; O(N) Poisson solve for non-periodic / AMR domains.
**Atom or composite:** Composite.
**Cost model:** O(N) per V-cycle; constant ~10.
**Real wall?** Yes — coarse-grid coupling on AMR.
**Cross-domain wiring:** Multigrid shared across PDE primitives in physics-diffusion, electromagnetics-antennas, photonics-optics.
**Notes:** Brandt (1977); Briggs-Henson-McCormick.

### berger-colella-amr (cross-domain alias: `bc-amr`, `flux-corrected-amr`)
**Domain:** Astrophysics / Cosmology
**Definition:** Conservative AMR for hyperbolic systems; coarse-fine flux correction maintains conservation.
**Atom or composite:** Composite.
**Cost model:** AMR overhead ~30%; >10⁴ savings vs uniform.
**Real wall?** Yes — load balancing.
**Cross-domain wiring:** Block-structured grid library shared with all PDE domains.
**Notes:** Berger & Colella (1989).

### sph (cross-domain alias: `smoothed-particle-hydro`)
**Domain:** Astrophysics / Cosmology
**Definition:** Lagrangian hydrodynamics via kernel-smoothed particles; A(r) = Σ_j m_j A_j W(|r−r_j|, h)/ρ_j.
**Atom or composite:** Composite.
**Cost model:** O(N N_neigh) per timestep.
**Real wall?** Yes — surface tension artifacts, weak instability resolution.
**Cross-domain wiring:** Kernel density estimation shared with statistics-probability; meshless methods cross to ml-training mesh-free networks.
**Notes:** Lucy (1977); Gingold-Monaghan (1977).

### moving-mesh (cross-domain alias: `arepo`, `voronoi-hydro`)
**Domain:** Astrophysics / Cosmology
**Definition:** Finite-volume hydro on Voronoi mesh that moves with the flow; Galilean-invariant, adaptive resolution.
**Atom or composite:** Composite.
**Cost model:** Voronoi reconstruction + flux solve.
**Real wall?** Yes — mesh regularization needed.
**Cross-domain wiring:** Voronoi tessellation reused in ml-training (k-NN classifiers), signal-processing-rf coverage maps.
**Notes:** Springel (2010) AREPO.

### voronoi-tessellation (cross-domain alias: `voronoi`, `voronoi-cells`)
**Domain:** Astrophysics / Cosmology
**Definition:** Decomposition of space into cells V_i = {x : |x−p_i| ≤ |x−p_j| ∀j}; foundation for moving-mesh and density estimators.
**Atom or composite:** Atom.
**Cost model:** O(N log N) construction.
**Real wall?** No.
**Cross-domain wiring:** Same primitive as ml-training nearest-neighbor partition.
**Notes:** Voronoi (1908); CGAL implementations.

### muscl-scheme (cross-domain alias: `muscl`, `monotone-upwind`)
**Domain:** Astrophysics / Cosmology
**Definition:** Second-order finite-volume scheme with slope-limited reconstruction; underpins many hydro codes.
**Atom or composite:** Composite.
**Cost model:** Constant-factor more than first-order.
**Real wall?** Yes — TVD limiters reduce to first-order at extrema.
**Cross-domain wiring:** TVD machinery shared with control-numerical-opt flux limiters.
**Notes:** van Leer (1979).

### godunov-method (cross-domain alias: `godunov-fv`)
**Domain:** Astrophysics / Cosmology
**Definition:** Finite-volume update using exact/approximate Riemann solution at cell interfaces; foundation of modern hydro.
**Atom or composite:** Composite.
**Cost model:** Riemann solve per face.
**Real wall?** Yes — first-order accurate alone.
**Cross-domain wiring:** Riemann solvers reused in physics-diffusion (multiphase), electromagnetics-antennas (Maxwell-FV).
**Notes:** Godunov (1959); Toro book.

### hllc-riemann (cross-domain alias: `hllc-solver`)
**Domain:** Astrophysics / Cosmology
**Definition:** Approximate Riemann solver with intermediate contact wave; preserves contact discontinuities, fast.
**Atom or composite:** Composite.
**Cost model:** Algebraic per interface.
**Real wall?** Yes — fails for strong rarefactions occasionally.
**Cross-domain wiring:** Approximate-flux method reused in physics-diffusion HLL family.
**Notes:** Toro-Spruce-Speares (1994).

### ramses-code (cross-domain alias: `ramses`)
**Domain:** Astrophysics / Cosmology
**Definition:** AMR Eulerian cosmological code with MUSCL hydro and tree-PM gravity; widely used for galaxy formation.
**Atom or composite:** Composite (toolchain).
**Cost model:** 10⁵–10⁷ CPU-hours per zoom.
**Real wall?** Yes — subgrid baryonics dominate uncertainty.
**Cross-domain wiring:** Workflow orchestration reused across all simulation domains.
**Notes:** Teyssier (2002).

### gadget4 (cross-domain alias: `gadget`)
**Domain:** Astrophysics / Cosmology
**Definition:** Latest tree-PM + SPH (and FoF/Subfind) code from Springel group; basis for IllustrisTNG, EAGLE-style runs.
**Atom or composite:** Composite.
**Cost model:** Petascale runs.
**Real wall?** Yes — memory per particle.
**Cross-domain wiring:** Domain decomposition / Peano-Hilbert ordering reused in HPC frameworks.
**Notes:** Springel et al. (2021).

### bonsai-gpu (cross-domain alias: `bonsai-nbody`)
**Domain:** Astrophysics / Cosmology
**Definition:** GPU-resident tree N-body code; full traversal on GPU.
**Atom or composite:** Composite.
**Cost model:** 10–100× speedup vs CPU.
**Real wall?** Yes — memory hierarchy hidden by traversal.
**Cross-domain wiring:** GPU traversal patterns shared with ml-training BVH/raytracing kernels.
**Notes:** Bédorf, Gaburov, Portegies Zwart.

### gizmo-code (cross-domain alias: `gizmo`, `mfm-mfv`)
**Domain:** Astrophysics / Cosmology
**Definition:** Meshless finite-mass/volume hydro built on GADGET infrastructure; better than classic SPH for shear/contact.
**Atom or composite:** Composite.
**Cost model:** Similar to SPH.
**Real wall?** Yes — meshless methods still active research.
**Cross-domain wiring:** Meshless methods bridge to ml-training neural-mesh-free PDE solvers.
**Notes:** Hopkins (2015).

## 9. Stellar Physics

### hr-diagram (cross-domain alias: `hertzsprung-russell`)
**Domain:** Astrophysics / Cosmology
**Definition:** L vs. T_eff (or color-magnitude) plot of stars; main sequence, giants, dwarfs as evolutionary tracks.
**Atom or composite:** Composite (observational + evolutionary).
**Cost model:** Population-synthesis pipeline.
**Real wall?** Yes — interpretation requires distance, extinction.
**Cross-domain wiring:** 2D feature space is ml-training classification playground.
**Notes:** Hertzsprung (1911), Russell (1913).

### stellar-structure-equations (cross-domain alias: `sse`, `4-coupled-odes`)
**Domain:** Astrophysics / Cosmology
**Definition:** Four ODEs: mass continuity, hydrostatic balance, energy conservation, radiative/convective transfer; in r or m_r.
**Atom or composite:** Composite.
**Cost model:** Henyey relaxation O(N×iter).
**Real wall?** Yes — boundary conditions (atmosphere matching).
**Cross-domain wiring:** Stiff BVP machinery shared with control-numerical-opt.
**Notes:** Henyey et al.; Kippenhahn-Weigert.

### lane-emden (cross-domain alias: `polytrope-eq`)
**Domain:** Astrophysics / Cosmology
**Definition:** Dimensionless ODE θ'' + (2/ξ)θ' + θ^n = 0 for polytropic star P=Kρ^(1+1/n).
**Atom or composite:** Atom.
**Cost model:** 1D ODE; analytic for n=0,1,5.
**Real wall?** No — useful approximation.
**Cross-domain wiring:** Same self-gravity-pressure balance shared with photonics-optics self-focusing.
**Notes:** Lane (1869); Emden (1907).

### polytropes (cross-domain alias: `n-polytrope`)
**Domain:** Astrophysics / Cosmology
**Definition:** Stars with P=Kρ^(1+1/n); n=3/2 for non-relativistic degenerate, n=3 for ultra-relativistic / radiation.
**Atom or composite:** Atom.
**Cost model:** Algebraic given Lane-Emden.
**Real wall?** Yes — toy model.
**Cross-domain wiring:** Power-law EOS shared with photonics-optics power-law media.
**Notes:** Chandrasekhar (1939).

### hydrostatic-equilibrium (cross-domain alias: `dp/dr-gravity`)
**Domain:** Astrophysics / Cosmology
**Definition:** dP/dr = −Gm(r)ρ/r²; force balance in non-relativistic star.
**Atom or composite:** Atom.
**Cost model:** ODE.
**Real wall?** Yes — central pressure diverges relativistically (TOV needed).
**Cross-domain wiring:** Force-balance ODE shared with control-numerical-opt static equilibria.
**Notes:** Eddington (1926).

### radiative-transfer-eddington (cross-domain alias: `eddington-approximation`)
**Domain:** Astrophysics / Cosmology
**Definition:** Closure J = 3K, H = (1/3)dJ/dτ; reduces RT moment hierarchy.
**Atom or composite:** Atom.
**Cost model:** Algebraic closure.
**Real wall?** Yes — fails in optically thin transitions.
**Cross-domain wiring:** Moment-closure pattern reused in physics-diffusion (P_N method), photonics-optics radiative cooling.
**Notes:** Eddington (1926).

### gray-atmosphere (cross-domain alias: `gray-rt`)
**Domain:** Astrophysics / Cosmology
**Definition:** Frequency-independent opacity approximation; T(τ) = T_eff(¾(τ+⅔))^(1/4).
**Atom or composite:** Atom.
**Cost model:** Analytic.
**Real wall?** Yes — real atmospheres have lines.
**Cross-domain wiring:** Effective-medium approximation reused in photonics-optics.
**Notes:** Eddington-Schwarzschild.

### nlte-rt (cross-domain alias: `non-lte`, `level-population-rt`)
**Domain:** Astrophysics / Cosmology
**Definition:** Solve rate equations for level populations together with RT; departures from Saha-Boltzmann.
**Atom or composite:** Composite.
**Cost model:** Coupled nonlinear iterations (ALI).
**Real wall?** Yes — chromospheres, hot stars need NLTE.
**Cross-domain wiring:** Rate-equation + transport coupling shared with physics-diffusion plasma kinetics.
**Notes:** Mihalas; Rybicki-Hummer ALI.

### schwarzschild-criterion (cross-domain alias: `convection-criterion`)
**Domain:** Astrophysics / Cosmology
**Definition:** Layer unstable to convection if (d ln T / d ln P)_rad > (d ln T / d ln P)_ad.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — onset only; nonlinear convection beyond.
**Cross-domain wiring:** Stability-criterion pattern reused in control-numerical-opt parameter envelopes.
**Notes:** Schwarzschild (1906); Ledoux variant.

### mixing-length-theory (cross-domain alias: `mlt`)
**Domain:** Astrophysics / Cosmology
**Definition:** Convection modeled as parcels traveling mixing length ℓ before mixing; gives convective flux F_conv.
**Atom or composite:** Composite.
**Cost model:** Algebraic per layer.
**Real wall?** Yes — calibrated parameter; superseded by 3D RHD locally.
**Cross-domain wiring:** Eddy-turnover model shared with physics-diffusion turbulence closures.
**Notes:** Böhm-Vitense (1958).

### pp-chain (cross-domain alias: `pp-fusion`)
**Domain:** Astrophysics / Cosmology
**Definition:** Sequence of proton-proton fusion reactions powering low-mass stars; 4 ¹H → ⁴He releasing 26.7 MeV.
**Atom or composite:** Composite (reaction network).
**Cost model:** Network ODE.
**Real wall?** Yes — temperature dependence ε ∝ T^4.
**Cross-domain wiring:** Reaction network shared with chemistry/electrochem; cross-domain to nuclear-physics.
**Notes:** Bethe-Critchfield (1938).

### cno-cycle (cross-domain alias: `cno-bicycle`)
**Domain:** Astrophysics / Cosmology
**Definition:** Catalytic cycle using C,N,O nuclei to fuse 4 ¹H → ⁴He; dominates above 17 MK; ε ∝ T^17.
**Atom or composite:** Composite.
**Cost model:** Network ODE.
**Real wall?** Yes — extreme T dependence drives convective cores.
**Cross-domain wiring:** Catalytic cycle pattern reused in chemistry electro-catalysis.
**Notes:** Bethe (1939), von Weizsäcker.

### triple-alpha (cross-domain alias: `3α-process`)
**Domain:** Astrophysics / Cosmology
**Definition:** 3 ⁴He → ¹²C via metastable ⁸Be; rate ∝ ρ²T^40; powers core-helium-burning.
**Atom or composite:** Composite.
**Cost model:** Network.
**Real wall?** Yes — Hoyle-state resonance crucial.
**Cross-domain wiring:** Resonance-mediated rate pattern reused in cavity QED.
**Notes:** Hoyle (1954) prediction.

### s-process (cross-domain alias: `slow-neutron-capture`)
**Domain:** Astrophysics / Cosmology
**Definition:** Neutron captures on slow timescale (τ_β < τ_n) in AGB stars; produces nuclei along β-stability valley.
**Atom or composite:** Composite (network).
**Cost model:** Network ODE with hundreds of isotopes.
**Real wall?** Yes — neutron source (¹³C(α,n)¹⁶O) uncertain.
**Cross-domain wiring:** Sparse linear systems shared with statistics-probability sparse-recovery.
**Notes:** B²FH (1957).

### r-process (cross-domain alias: `rapid-neutron-capture`)
**Domain:** Astrophysics / Cosmology
**Definition:** Rapid neutron captures producing heavy elements (A>140); sites: NS mergers (kilonova GW170817), supernovae.
**Atom or composite:** Composite.
**Cost model:** Reaction network 1000s isotopes.
**Real wall?** Yes — neutron-rich exotic nuclear masses poorly known.
**Cross-domain wiring:** Multi-messenger constraint integrates GW + EM.
**Notes:** B²FH; Kasen et al. kilonova.

### rp-process (cross-domain alias: `rapid-proton-capture`)
**Domain:** Astrophysics / Cosmology
**Definition:** Proton captures in H-rich, hot environments (X-ray bursts); pushes flow toward proton drip line.
**Atom or composite:** Composite.
**Cost model:** Network with proton-rich nuclei.
**Real wall?** Yes — endpoint near A~100.
**Cross-domain wiring:** Proton-rich network shared with rare-isotope physics.
**Notes:** Wallace-Woosley (1981).

### p-process (cross-domain alias: `gamma-process`)
**Domain:** Astrophysics / Cosmology
**Definition:** Photodisintegration of seeds (γ,n) in core-collapse SN producing proton-rich isotopes inaccessible to s/r.
**Atom or composite:** Composite.
**Cost model:** Network.
**Real wall?** Yes — accurate s-seed input required.
**Cross-domain wiring:** Photodisintegration cross-sections shared with photonics-optics photonuclear data.
**Notes:** Woosley-Howard (1978).

### red-giant-branch (cross-domain alias: `rgb`)
**Domain:** Astrophysics / Cosmology
**Definition:** Post-MS phase with H-shell burning, expanding envelope, He core; ends at He flash.
**Atom or composite:** Composite.
**Cost model:** Stellar-evolution code (MESA).
**Real wall?** Yes — Hayashi limit, dredge-ups.
**Cross-domain wiring:** Shell-source instability shared with combustion deflagration models.
**Notes:** Iben review.

### agb (cross-domain alias: `asymptotic-giant-branch`)
**Domain:** Astrophysics / Cosmology
**Definition:** Double-shell-burning phase with thermal pulses; major s-process site; ends in planetary nebula.
**Atom or composite:** Composite.
**Cost model:** Long-time evolution; mass loss critical.
**Real wall?** Yes — mass-loss prescription is uncertain.
**Cross-domain wiring:** Dust-driven winds shared with photonics-optics radiation pressure.
**Notes:** Karakas-Lattanzio review.

### planetary-nebula (cross-domain alias: `pn`)
**Domain:** Astrophysics / Cosmology
**Definition:** Ionized AGB-ejected envelope around a hot WD core; rich nebular emission lines.
**Atom or composite:** Composite.
**Cost model:** Photoionization code (Cloudy).
**Real wall?** Yes — morphological asymmetries unexplained.
**Cross-domain wiring:** Photoionization equilibrium shared with chemistry plasma.
**Notes:** Kwok review.

### white-dwarf (cross-domain alias: `wd`)
**Domain:** Astrophysics / Cosmology
**Definition:** Electron-degenerate remnant of low/intermediate-mass stars; mass-radius R ∝ M^(−1/3); cooling timescale Gyr.
**Atom or composite:** Composite.
**Cost model:** Cooling track ODE.
**Real wall?** Yes — Chandrasekhar limit.
**Cross-domain wiring:** Fermi-degenerate EOS shared with condensed-matter primitives.
**Notes:** Chandrasekhar (1931); Mestel cooling.

### chandrasekhar-limit (cross-domain alias: `m_ch`)
**Domain:** Astrophysics / Cosmology
**Definition:** Maximum WD mass M_Ch ≈ 1.4 M_⊙ above which electron degeneracy can't support star; sets SN Ia trigger.
**Atom or composite:** Atom.
**Cost model:** Algebraic from polytropic n=3.
**Real wall?** Yes — rotating/magnetic WDs slightly exceed.
**Cross-domain wiring:** Critical-load concept shared with control-numerical-opt buckling.
**Notes:** Chandrasekhar (1931).

### neutron-star (cross-domain alias: `ns`)
**Domain:** Astrophysics / Cosmology
**Definition:** Compact remnant ρ~10¹⁴ g/cc, R~10 km, M~1.4–2 M_⊙; neutron degeneracy + nuclear repulsion.
**Atom or composite:** Composite.
**Cost model:** TOV + EOS.
**Real wall?** Yes — TOV mass limit ~2.2 M_⊙.
**Cross-domain wiring:** Dense-matter EOS bridges to nuclear-physics, condensed-matter lattice-QCD.
**Notes:** Baade-Zwicky (1934); Demorest 2010 mass.

### magnetar (cross-domain alias: `sgr-axp`)
**Domain:** Astrophysics / Cosmology
**Definition:** NS with surface field B ~10¹⁴–10¹⁵ G; powers SGR/AXP via magnetic decay.
**Atom or composite:** Composite.
**Cost model:** Force-free magnetosphere modeling.
**Real wall?** Yes — origin of fields debated (dynamo vs. fossil).
**Cross-domain wiring:** Strong-field QED tooling shared with photonics-optics vacuum birefringence.
**Notes:** Duncan-Thompson (1992).

### pulsar (cross-domain alias: `radio-pulsar`)
**Domain:** Astrophysics / Cosmology
**Definition:** Rotating NS emitting beamed radio pulses at spin period; powered by rotational KE loss Ė = IΩΩ̇.
**Atom or composite:** Composite.
**Cost model:** Timing residuals analyzed with TEMPO/PINT.
**Real wall?** Yes — emission mechanism still uncertain.
**Cross-domain wiring:** Timing analysis shared with signal-processing-rf phase-locked loops.
**Notes:** Hewish-Bell (1968).

### supernova-ia (cross-domain alias: `sn-ia`, `wd-explosion`)
**Domain:** Astrophysics / Cosmology
**Definition:** Thermonuclear runaway in WD reaching M_Ch via accretion or merger; standardizable candle (M_B ≈ −19.3).
**Atom or composite:** Composite.
**Cost model:** Hydrodynamic + nuclear network sims.
**Real wall?** Yes — single-degenerate vs. double-degenerate debate.
**Cross-domain wiring:** Light-curve standardization is ml-training feature engineering.
**Notes:** Hoyle-Fowler (1960); Phillips relation.

### supernova-ii (cross-domain alias: `cc-sn`, `core-collapse-h`)
**Domain:** Astrophysics / Cosmology
**Definition:** Hydrogen-rich core-collapse SN from massive star (>8 M_⊙); leaves NS or BH; light curves with plateau (IIP).
**Atom or composite:** Composite.
**Cost model:** Multi-D rad-hydro neutrino transport.
**Real wall?** Yes — explosion mechanism partially solved (neutrino-driven).
**Cross-domain wiring:** Neutrino transport shared with physics-diffusion radiative transport.
**Notes:** Bethe-Wilson (1985); Janka review.

### supernova-ibc (cross-domain alias: `stripped-cc-sn`)
**Domain:** Astrophysics / Cosmology
**Definition:** Core-collapse SN with H (Ib) and He (Ic) envelopes stripped by winds/binaries.
**Atom or composite:** Composite.
**Cost model:** Same as Type II.
**Real wall?** Yes — progenitor identification rare.
**Cross-domain wiring:** Binary stripping → mass-transfer codes connect to stellar dynamics primitives.
**Notes:** Filippenko (1997) classification.

### sn-light-curve-modeling (cross-domain alias: `lc-modeling`, `arnett-model`)
**Domain:** Astrophysics / Cosmology
**Definition:** Model bolometric LC via radioactive decay (⁵⁶Ni→⁵⁶Co→⁵⁶Fe) + diffusion approximation in expanding ejecta.
**Atom or composite:** Composite.
**Cost model:** Analytic Arnett; numerical via STELLA/Sedona.
**Real wall?** Yes — non-LTE and asymmetry effects matter.
**Cross-domain wiring:** Radioactive-decay driven diffusion shared with physics-diffusion.
**Notes:** Arnett (1982).

### kilonova (cross-domain alias: `kn-merger-glow`)
**Domain:** Astrophysics / Cosmology
**Definition:** Optical/IR transient from NS-NS or NS-BH merger powered by r-process radioactive heating; GW170817/AT2017gfo.
**Atom or composite:** Composite.
**Cost model:** Radiative transfer with billions of lines.
**Real wall?** Yes — opacity of lanthanides huge and uncertain.
**Cross-domain wiring:** Multi-messenger pipeline (GW + EM) integrated end-to-end.
**Notes:** Metzger review; Kasen et al.

### x-ray-binary (cross-domain alias: `xrb`)
**Domain:** Astrophysics / Cosmology
**Definition:** Compact object accreting from companion; HMXB (massive donor) and LMXB (low-mass donor) classes; emits X-rays.
**Atom or composite:** Composite.
**Cost model:** Accretion modeling.
**Real wall?** Yes — accretion physics + wind feedback.
**Cross-domain wiring:** Spectral modeling shared with photonics-optics line-formation.
**Notes:** Shakura-Sunyaev disk.

### grb-long (cross-domain alias: `long-grb`, `collapsar`)
**Domain:** Astrophysics / Cosmology
**Definition:** Gamma-ray burst with duration T_90>2 s from collapsar of rapidly rotating massive star; ultra-relativistic jet.
**Atom or composite:** Composite.
**Cost model:** Relativistic jet simulation.
**Real wall?** Yes — jet launching and dissipation mechanisms.
**Cross-domain wiring:** Relativistic jets shared with AGN primitives downstream.
**Notes:** Woosley (1993); MacFadyen-Woosley.

### grb-short (cross-domain alias: `short-grb`, `ns-ns-grb`)
**Domain:** Astrophysics / Cosmology
**Definition:** GRB with T_90 < 2 s from compact-object merger; co-incident with GW170817.
**Atom or composite:** Composite.
**Cost model:** Relativistic outflow + EM model.
**Real wall?** Yes — off-axis afterglow geometry.
**Cross-domain wiring:** Multi-messenger detection couples GW and γ-ray pipelines.
**Notes:** Eichler et al. (1989); GW170817/GRB170817A.

## 10. Stellar Dynamics

### virial-theorem-stellar (cross-domain alias: `2t+w=0`)
**Domain:** Astrophysics / Cosmology
**Definition:** For bound self-gravitating equilibrium: 2T + W = 0; relates kinetic and potential energy averages.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — assumes equilibrium.
**Cross-domain wiring:** Variational identity reused in control-numerical-opt; analog to virial in thermodynamics.
**Notes:** Clausius (1870); Chandrasekhar.

### violent-relaxation (cross-domain alias: `lynden-bell`)
**Domain:** Astrophysics / Cosmology
**Definition:** Rapid phase mixing in time-varying potential during galaxy formation; leads to quasi-equilibrium independent of initial conditions.
**Atom or composite:** Composite.
**Cost model:** N-body fast (few crossing times).
**Real wall?** Yes — does not produce thermal Boltzmann DF.
**Cross-domain wiring:** Phase-mixing analog of statistics-probability ergodicity.
**Notes:** Lynden-Bell (1967).

### two-body-relaxation (cross-domain alias: `t_relax`)
**Domain:** Astrophysics / Cosmology
**Definition:** t_relax ≈ 0.1N/ln N × t_cross; timescale for accumulated 2-body encounters to redistribute energies.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — distinguishes collisional (clusters) vs collisionless (galaxies) systems.
**Cross-domain wiring:** Diffusion-in-energy concept shared with physics-diffusion stochastic transport.
**Notes:** Chandrasekhar (1942); BT08.

### dynamical-friction (cross-domain alias: `df-chandrasekhar`)
**Domain:** Astrophysics / Cosmology
**Definition:** Drag on massive object moving through stellar background due to gravitational wake; F_DF ∝ ρ M² / v².
**Atom or composite:** Composite.
**Cost model:** Algebraic with Coulomb log.
**Real wall?** Yes — drives sinking of satellites/SMBHs.
**Cross-domain wiring:** Polarization-wake analog to electromagnetics-antennas plasma drag.
**Notes:** Chandrasekhar (1943).

### mass-segregation (cross-domain alias: `mass-stratification`)
**Domain:** Astrophysics / Cosmology
**Definition:** Equipartition drives heavy stars to sink to cluster core via 2-body relaxation; observed in globular clusters.
**Atom or composite:** Composite.
**Cost model:** N-body with stellar masses.
**Real wall?** Yes — incomplete equipartition.
**Cross-domain wiring:** Sorting-by-mass analog to control-numerical-opt sorting networks.
**Notes:** Spitzer instability (1969).

### core-collapse-cluster (cross-domain alias: `gravothermal-collapse`)
**Domain:** Astrophysics / Cosmology
**Definition:** Runaway contraction of cluster core driven by negative heat capacity; halted by binary heating.
**Atom or composite:** Composite.
**Cost model:** N-body or Fokker-Planck.
**Real wall?** Yes — negative specific heat is fundamental.
**Cross-domain wiring:** Negative heat capacity shared with statistics-probability non-extensive thermodynamics.
**Notes:** Antonov, Lynden-Bell-Wood (1968).

### gravothermal-catastrophe (cross-domain alias: `antonov-instability`)
**Domain:** Astrophysics / Cosmology
**Definition:** Instability of self-gravitating isothermal sphere when central density > 709× boundary density.
**Atom or composite:** Atom.
**Cost model:** Algebraic stability criterion.
**Real wall?** Yes — onset of core collapse.
**Cross-domain wiring:** Eigenvalue instability pattern shared with control-numerical-opt.
**Notes:** Antonov (1962); Lynden-Bell-Wood (1968).

### fokker-planck-stellar (cross-domain alias: `fp-stellar`)
**Domain:** Astrophysics / Cosmology
**Definition:** Evolution of DF f(E,L,t) under diffusion + drift coefficients from 2-body encounters; valid in collisional regime.
**Atom or composite:** Composite.
**Cost model:** 2D PDE in (E,L).
**Real wall?** Yes — assumes locality, isotropy.
**Cross-domain wiring:** Fokker-Planck shared with physics-diffusion, statistics-probability, ml-training (gradient noise).
**Notes:** Cohn (1979).

### jeans-equations (cross-domain alias: `jeans-stellar`)
**Domain:** Astrophysics / Cosmology
**Definition:** Moments of collisionless Boltzmann; gives velocity-dispersion-tensor balance with potential gradient.
**Atom or composite:** Composite.
**Cost model:** Algebraic given DF/anisotropy.
**Real wall?** Yes — anisotropy-mass degeneracy.
**Cross-domain wiring:** Moment hierarchy shared with physics-diffusion fluid moments.
**Notes:** Jeans (1915); BT08.

### schwarzschild-orbit-modeling (cross-domain alias: `orbit-superposition`)
**Domain:** Astrophysics / Cosmology
**Definition:** Build galaxy DF as weighted sum of integrable orbits in fixed potential; constraints from kinematics.
**Atom or composite:** Composite.
**Cost model:** QP / NNLS over orbit library.
**Real wall?** Yes — non-uniqueness without proper-motion data.
**Cross-domain wiring:** Quadratic programming = control-numerical-opt; orbit library is signal-processing-rf basis.
**Notes:** Schwarzschild (1979).

### distribution-functions-stellar (cross-domain alias: `df-stellar`, `f(E,L)`)
**Domain:** Astrophysics / Cosmology
**Definition:** f(x,v) phase-space density of stars; integrates to ρ(x); for integrable systems f=f(integrals).
**Atom or composite:** Atom.
**Cost model:** Sampling / analytic.
**Real wall?** No.
**Cross-domain wiring:** Phase-space PDF shared with quantum/statistics-probability.
**Notes:** Eddington formula; BT08.

### action-angle-stellar (cross-domain alias: `j_r-j_z-j_φ`)
**Domain:** Astrophysics / Cosmology
**Definition:** Canonical action variables labeling integrable orbits; angles increase linearly with time.
**Atom or composite:** Atom.
**Cost model:** Stäckel fits / torus mapping.
**Real wall?** Yes — non-integrable potentials lack global actions.
**Cross-domain wiring:** Action-angle reused in control-numerical-opt KAM analyses.
**Notes:** Binney-Tremaine; Binney (2012) torus mapping.

### epicycle-approximation (cross-domain alias: `epicycle`)
**Domain:** Astrophysics / Cosmology
**Definition:** Near-circular orbit linearized to harmonic radial+vertical oscillations around guiding center.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — fails for eccentric orbits.
**Cross-domain wiring:** Linearized oscillator around equilibrium shared with control-numerical-opt small-signal analysis.
**Notes:** Lindblad (1958); BT08.

### density-wave-theory (cross-domain alias: `lin-shu`)
**Domain:** Astrophysics / Cosmology
**Definition:** Spiral arms as quasi-stationary density waves in disk; Lin-Shu dispersion relation links Ω_p, k, σ.
**Atom or composite:** Composite.
**Cost model:** Linear stability of disk.
**Real wall?** Yes — long-lived vs. transient debate.
**Cross-domain wiring:** Linear wave theory shared with photonics-optics, electromagnetics-antennas.
**Notes:** Lin-Shu (1964).

### spiral-structure (cross-domain alias: `spiral-arms`)
**Domain:** Astrophysics / Cosmology
**Definition:** Logarithmic spirals in disk galaxies; pitch angle, arm count, m-fold symmetry; tracers: SF regions, gas.
**Atom or composite:** Composite.
**Cost model:** Image decomposition.
**Real wall?** Yes — origin is multi-mechanism.
**Cross-domain wiring:** Pattern recognition = ml-training feature extraction.
**Notes:** Sellwood reviews.

### bar-formation (cross-domain alias: `bar-instability`)
**Domain:** Astrophysics / Cosmology
**Definition:** m=2 perturbation grows nonlinearly in cold disk; produces stellar bar in ~half of spirals.
**Atom or composite:** Composite.
**Cost model:** N-body simulation.
**Real wall?** Yes — gas content modulates bar lifetime.
**Cross-domain wiring:** Instability + saturation pattern shared with photonics-optics nonlinear modes.
**Notes:** Hohl (1971).

### tidal-stripping (cross-domain alias: `tidal-stripping-cluster`)
**Domain:** Astrophysics / Cosmology
**Definition:** Mass loss from satellite as material outside tidal radius is removed by host gravity.
**Atom or composite:** Composite.
**Cost model:** N-body / semi-analytic.
**Real wall?** Yes — sets satellite mass functions.
**Cross-domain wiring:** Tidal-shear concept shared with photonics-optics shear forces.
**Notes:** King (1962) profile.

### tidal-radius (cross-domain alias: `roche-radius`)
**Domain:** Astrophysics / Cosmology
**Definition:** r_t ≈ a (m/3M)^(1/3); distance beyond which satellite material is stripped.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — depends on host triaxiality.
**Cross-domain wiring:** Force-balance radius shared with control-numerical-opt feasibility boundaries.
**Notes:** Jacobi/Roche (1881).

### dynamical-heating (cross-domain alias: `dyn-heating`)
**Domain:** Astrophysics / Cosmology
**Definition:** Increase in disk velocity dispersion due to scattering by GMCs, spirals, mergers; thickens disk over time.
**Atom or composite:** Composite.
**Cost model:** Diffusion in action space.
**Real wall?** Yes — drives age-velocity-dispersion relation.
**Cross-domain wiring:** Diffusion-in-action shared with stochastic-PDE in physics-diffusion.
**Notes:** Spitzer-Schwarzschild; Sellwood-Binney.

## 11. Galactic & Extragalactic

### hubble-sequence (cross-domain alias: `tuning-fork`)
**Domain:** Astrophysics / Cosmology
**Definition:** Morphological classification: ellipticals (E0–E7), lenticulars (S0), spirals (Sa–Sd), barred (SB), irregular.
**Atom or composite:** Atom (taxonomy).
**Cost model:** Visual / ML classification.
**Real wall?** Yes — physical drivers (merger history, mass).
**Cross-domain wiring:** Morphology classification is ml-training image classification staple.
**Notes:** Hubble (1926); de Vaucouleurs extensions.

### galaxy-scaling-relations (cross-domain alias: `scaling-laws`)
**Domain:** Astrophysics / Cosmology
**Definition:** Empirical correlations among global properties (size-mass, luminosity-velocity); reveal regulation by gravity + feedback.
**Atom or composite:** Composite.
**Cost model:** Linear regression.
**Real wall?** Yes — intrinsic scatter encodes physics.
**Cross-domain wiring:** Empirical scaling laws shared with statistics-probability hierarchical Bayes.
**Notes:** Kormendy-Bender.

### tully-fisher (cross-domain alias: `tf-relation`)
**Domain:** Astrophysics / Cosmology
**Definition:** L ∝ v_rot^4 for spirals; baryonic version M_b ∝ v^4 extends across 10 dex.
**Atom or composite:** Atom.
**Cost model:** Linear regression.
**Real wall?** Yes — used as distance indicator.
**Cross-domain wiring:** Direct distance ladder coupling to H₀ inference.
**Notes:** Tully-Fisher (1977); McGaugh BTFR.

### fundamental-plane (cross-domain alias: `fp-ellipticals`)
**Domain:** Astrophysics / Cosmology
**Definition:** Ellipticals lie on plane log R_e = a log σ + b ⟨μ_e⟩ + c; tight relation linking structure & kinematics.
**Atom or composite:** Composite.
**Cost model:** 3-parameter regression.
**Real wall?** Yes — tilt indicates non-homology / IMF variation.
**Cross-domain wiring:** Manifold-learning pattern; analog to ml-training dimensionality reduction.
**Notes:** Djorgovski-Davis; Dressler et al. (1987).

### m-sigma (cross-domain alias: `bh-bulge-relation`)
**Domain:** Astrophysics / Cosmology
**Definition:** M_BH ∝ σ^4 for SMBH at galaxy center; tight ~0.3 dex scatter.
**Atom or composite:** Atom.
**Cost model:** Regression.
**Real wall?** Yes — co-evolution mechanism unclear.
**Cross-domain wiring:** Tight correlation suggests feedback-loop signature analog in control-numerical-opt.
**Notes:** Ferrarese-Merritt; Gebhardt (2000).

### galaxy-halo-connection (cross-domain alias: `shm`, `stellar-halo-mass`)
**Domain:** Astrophysics / Cosmology
**Definition:** M_* − M_h relation linking baryonic galaxy mass to host DM halo mass via abundance matching / HOD.
**Atom or composite:** Composite.
**Cost model:** Statistical fit / HOD.
**Real wall?** Yes — degeneracies with scatter and assembly bias.
**Cross-domain wiring:** Latent-variable matching shared with ml-training assignment problems.
**Notes:** Behroozi-Wechsler-Conroy.

### agn-unification (cross-domain alias: `obscured-vs-type1`)
**Domain:** Astrophysics / Cosmology
**Definition:** Different AGN classes (Seyfert 1, 2, quasar, blazar, radio galaxy) explained by orientation w.r.t. dusty torus.
**Atom or composite:** Composite.
**Cost model:** Conceptual / SED modeling.
**Real wall?** Yes — luminosity-dependent obscuration (receding torus).
**Cross-domain wiring:** Geometry-driven appearance shared with photonics-optics anisotropic scattering.
**Notes:** Antonucci (1993); Urry-Padovani (1995).

### blazar (cross-domain alias: `bl-lac-fsrq`)
**Domain:** Astrophysics / Cosmology
**Definition:** AGN with relativistic jet aligned near LOS; double-humped SED (synchrotron + IC).
**Atom or composite:** Composite.
**Cost model:** SED + variability modeling.
**Real wall?** Yes — beaming amplification dominant.
**Cross-domain wiring:** Relativistic beaming math shared with signal-processing-rf Doppler shifts at large β.
**Notes:** Padovani review.

### radio-galaxy (cross-domain alias: `fr-i-fr-ii`)
**Domain:** Astrophysics / Cosmology
**Definition:** AGN with extended kpc-Mpc radio lobes; FR-I (edge-darkened) vs FR-II (edge-brightened, hot spots).
**Atom or composite:** Composite.
**Cost model:** Radio imaging + spectral aging.
**Real wall?** Yes — jet stability vs. ambient pressure.
**Cross-domain wiring:** Radio morphology = electromagnetics-antennas array-image inversion.
**Notes:** Fanaroff-Riley (1974).

### quasar-luminosity-function (cross-domain alias: `qlf`, `phi(l,z)`)
**Domain:** Astrophysics / Cosmology
**Definition:** Comoving number density of quasars as function of luminosity and redshift; double power law (PLE/PDE).
**Atom or composite:** Composite.
**Cost model:** Maximum likelihood on surveys.
**Real wall?** Yes — incompleteness at faint end.
**Cross-domain wiring:** Survival/likelihood fits shared with statistics-probability.
**Notes:** Schmidt (1968); Boyle et al.

### eddington-luminosity (cross-domain alias: `l_edd`)
**Domain:** Astrophysics / Cosmology
**Definition:** L_Edd = 4πGMm_p c/σ_T ≈ 1.26×10³⁸ erg/s (M/M_⊙); balance of radiation pressure and gravity.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — limits accretion rate (modulo super-Eddington flows).
**Cross-domain wiring:** Radiation pressure shared with photonics-optics; same force balance.
**Notes:** Eddington (1926).

### shakura-sunyaev-disk (cross-domain alias: `thin-disk`, `α-disk`)
**Domain:** Astrophysics / Cosmology
**Definition:** Geometrically thin, optically thick accretion disk with viscosity ν=αc_s H; multicolor blackbody spectrum.
**Atom or composite:** Composite.
**Cost model:** 1D radial equations.
**Real wall?** Yes — α prescription empirical.
**Cross-domain wiring:** Effective-viscosity model shared with physics-diffusion turbulent closures.
**Notes:** Shakura-Sunyaev (1973).

### slim-disk (cross-domain alias: `super-eddington-disk`)
**Domain:** Astrophysics / Cosmology
**Definition:** Accretion disk where radial advection becomes important at near-/super-Eddington rates; H/R ~ 1.
**Atom or composite:** Composite.
**Cost model:** 1D radial; needs GR.
**Real wall?** Yes — radiation pressure dominates.
**Cross-domain wiring:** Advection-dominated flows shared with physics-diffusion convection-dominated regimes.
**Notes:** Abramowicz et al. (1988).

### adaf (cross-domain alias: `riaf`, `advection-dominated-accretion`)
**Domain:** Astrophysics / Cosmology
**Definition:** Hot, low-density, two-temperature accretion at sub-Eddington rates; advection carries energy into BH.
**Atom or composite:** Composite.
**Cost model:** Self-similar / GRMHD.
**Real wall?** Yes — Sgr A* prototype.
**Cross-domain wiring:** Two-temperature plasma shared with plasma primitives downstream.
**Notes:** Narayan-Yi (1994).

### blandford-payne (cross-domain alias: `bp-disk-wind`)
**Domain:** Astrophysics / Cosmology
**Definition:** MHD launching of disk wind/jet by magnetic field threading rotating disk; angle <60° from disk plane required.
**Atom or composite:** Composite.
**Cost model:** Force-free / MHD simulation.
**Real wall?** Yes — competes with Blandford-Znajek for jet power.
**Cross-domain wiring:** Magnetic-field-line wind launching shared with stellar wind primitives.
**Notes:** Blandford-Payne (1982).

### relativistic-jet (cross-domain alias: `agn-jet`)
**Domain:** Astrophysics / Cosmology
**Definition:** Collimated outflow with bulk Lorentz factor Γ ~10–50; powered by central engine; observable from radio to TeV.
**Atom or composite:** Composite.
**Cost model:** RMHD simulation.
**Real wall?** Yes — composition (e±/baryon) debated.
**Cross-domain wiring:** Collimation physics shared with plasma confinement; spectral modeling reuses photonics-optics.
**Notes:** Begelman-Blandford-Rees review.

### faraday-rotation (cross-domain alias: `rm-rotation-measure`)
**Domain:** Astrophysics / Cosmology
**Definition:** Rotation of polarization angle Δχ = RM λ²; RM ∝ ∫n_e B_∥ dl; probes magnetic field along LOS.
**Atom or composite:** Atom.
**Cost model:** Spectropolarimetry.
**Real wall?** Yes — depolarization at high RM.
**Cross-domain wiring:** Same effect treated in electromagnetics-antennas, photonics-optics; cosmic application here.
**Notes:** Burn (1966); Brentjens-de Bruyn RM synthesis.

### synchrotron-self-absorption (cross-domain alias: `ssa`)
**Domain:** Astrophysics / Cosmology
**Definition:** At low frequencies synchrotron source becomes optically thick to its own radiation; spectral turnover ∝ ν^(5/2).
**Atom or composite:** Atom.
**Cost model:** RT solve.
**Real wall?** Yes — sets brightness-temperature limit (~10¹² K).
**Cross-domain wiring:** Optical-depth threshold shared with photonics-optics blackbody saturation.
**Notes:** Slish (1963).

### inverse-compton (cross-domain alias: `ic-scattering`)
**Domain:** Astrophysics / Cosmology
**Definition:** Up-scattering of low-energy photons by relativistic electrons; second hump in blazar SED.
**Atom or composite:** Atom.
**Cost model:** Convolution of electron distribution with photon field.
**Real wall?** Yes — Klein-Nishina regime at high energies.
**Cross-domain wiring:** Same scattering kernel as photonics-optics Compton imaging.
**Notes:** Compton; Rybicki-Lightman.

### sed-modeling (cross-domain alias: `agn-sed`, `mwl-sed`)
**Domain:** Astrophysics / Cosmology
**Definition:** Multi-wavelength spectral-energy distribution fit with synchrotron + IC + thermal + dust components.
**Atom or composite:** Composite.
**Cost model:** MCMC / least-squares.
**Real wall?** Yes — many degeneracies; need broadband coverage.
**Cross-domain wiring:** SED fitting is statistics-probability MCMC; templates reuse signal-processing-rf basis methods.
**Notes:** da Cunha MAGPHYS; Boquien CIGALE.

### dust-extinction-calzetti (cross-domain alias: `calzetti-law`)
**Domain:** Astrophysics / Cosmology
**Definition:** Empirical attenuation law for starburst galaxies: k(λ) with E(B−V)_stars = 0.44 E(B−V)_gas.
**Atom or composite:** Atom.
**Cost model:** Algebraic per λ.
**Real wall?** Yes — varies by galaxy.
**Cross-domain wiring:** Wavelength-dependent attenuation shared with photonics-optics absorption spectra.
**Notes:** Calzetti et al. (2000).

### cardelli-extinction (cross-domain alias: `ccm-law`, `r_v-law`)
**Domain:** Astrophysics / Cosmology
**Definition:** Galactic extinction A(λ)/A_V parametrized by R_V = A_V/E(B−V); R_V=3.1 standard ISM.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** No.
**Cross-domain wiring:** Standard reddening curve reused in dust-modeling pipelines.
**Notes:** Cardelli-Clayton-Mathis (1989).

## 12. Plasma, MHD & Cosmic Rays

### ideal-mhd (cross-domain alias: `imhd`)
**Domain:** Astrophysics / Cosmology
**Definition:** Single-fluid MHD with infinite conductivity: ∂B/∂t = ∇×(v×B); flux freezing.
**Atom or composite:** Composite.
**Cost model:** Hyperbolic PDE.
**Real wall?** Yes — breaks at current sheets (reconnection).
**Cross-domain wiring:** Conservation-law solver shared with hydro Riemann/HLL methods.
**Notes:** Alfvén (1942).

### resistive-mhd (cross-domain alias: `rmhd`, `finite-η`)
**Domain:** Astrophysics / Cosmology
**Definition:** MHD with finite resistivity η; adds diffusion of B: ∂B/∂t = ∇×(v×B) + η∇²B.
**Atom or composite:** Composite.
**Cost model:** Stiffer; implicit/IMEX preferred.
**Real wall?** Yes — Lundquist number S = LV_A/η.
**Cross-domain wiring:** Diffusion of B is physics-diffusion; same Laplacian operator.
**Notes:** Goedbloed-Poedts.

### ambipolar-diffusion (cross-domain alias: `ad-mhd`)
**Domain:** Astrophysics / Cosmology
**Definition:** Slow drift of B with ions through neutrals in partially ionized gas; relevant to molecular clouds, protoplanetary disks.
**Atom or composite:** Composite.
**Cost model:** Stiff source term.
**Real wall?** Yes — sets star-formation timescale in clouds.
**Cross-domain wiring:** Two-species drift shared with semiconductor physics.
**Notes:** Mestel-Spitzer (1956).

### hall-mhd (cross-domain alias: `hall-term`)
**Domain:** Astrophysics / Cosmology
**Definition:** Generalized Ohm's law including J×B term; relevant at ion inertial scale d_i; allows whistlers.
**Atom or composite:** Composite.
**Cost model:** Dispersive; stiff explicit.
**Real wall?** Yes — needed for fast reconnection.
**Cross-domain wiring:** Dispersive PDE shared with photonics-optics dispersive media.
**Notes:** Hall (1879); modern reconnection.

### sweet-parker-reconnection (cross-domain alias: `sp-reconnection`)
**Domain:** Astrophysics / Cosmology
**Definition:** Reconnection rate v_rec/v_A ≈ S^(−1/2); too slow for solar flares.
**Atom or composite:** Composite.
**Cost model:** Algebraic scaling.
**Real wall?** Yes — astrophysical S enormous (10¹⁴), rate negligible.
**Cross-domain wiring:** Diffusive boundary-layer scaling shared with physics-diffusion Prandtl layers.
**Notes:** Sweet (1958), Parker (1957).

### petschek-reconnection (cross-domain alias: `petschek`)
**Domain:** Astrophysics / Cosmology
**Definition:** Reconnection model with X-line + slow shocks; v_rec/v_A ~ 1/log S; faster than Sweet-Parker.
**Atom or composite:** Composite.
**Cost model:** Algebraic scaling.
**Real wall?** Yes — requires localized resistivity.
**Cross-domain wiring:** Same shock-mediated reconnection used in solar/space physics.
**Notes:** Petschek (1964).

### plasmoid-instability (cross-domain alias: `plasmoid-cascade`)
**Domain:** Astrophysics / Cosmology
**Definition:** Tearing of long current sheets into plasmoid chain when S > 10⁴; reconnection rate becomes ~0.01 v_A independent of S.
**Atom or composite:** Composite.
**Cost model:** High-resolution MHD sims.
**Real wall?** Yes — bridges Sweet-Parker / Petschek.
**Cross-domain wiring:** Cascade-of-scales shared with turbulence primitives.
**Notes:** Loureiro et al. (2007); Bhattacharjee.

### kolmogorov-cascade (cross-domain alias: `k41`, `e(k)∝k^-5/3`)
**Domain:** Astrophysics / Cosmology
**Definition:** Hydrodynamic turbulence energy spectrum E(k) ∝ k^(−5/3) in inertial range.
**Atom or composite:** Atom.
**Cost model:** Spectral diagnostic.
**Real wall?** Yes — universal in 3D HD.
**Cross-domain wiring:** Power-law spectrum shared with signal-processing-rf 1/f-α scalings.
**Notes:** Kolmogorov (1941).

### iroshnikov-kraichnan (cross-domain alias: `ik-spectrum`, `e(k)∝k^-3/2`)
**Domain:** Astrophysics / Cosmology
**Definition:** Isotropic MHD turbulence prediction E(k) ∝ k^(−3/2); from Alfvén-wave interactions.
**Atom or composite:** Atom.
**Cost model:** Spectral diagnostic.
**Real wall?** Yes — superseded by anisotropic GS95.
**Cross-domain wiring:** Spectral-index measurement shared with signal-processing-rf.
**Notes:** Iroshnikov (1963), Kraichnan (1965).

### goldreich-sridhar (cross-domain alias: `gs95`, `critical-balance`)
**Domain:** Astrophysics / Cosmology
**Definition:** Anisotropic MHD turbulence: critical balance k_∥ v_A ~ k_⊥ v_⊥; E(k_⊥) ∝ k_⊥^(−5/3).
**Atom or composite:** Composite.
**Cost model:** Spectral diagnostic.
**Real wall?** Yes — verified in solar wind.
**Cross-domain wiring:** Anisotropic spectra shared with signal-processing-rf direction-finding spectra.
**Notes:** Goldreich-Sridhar (1995).

### ism-phases (cross-domain alias: `cnm-wnm-him`)
**Domain:** Astrophysics / Cosmology
**Definition:** Interstellar medium thermal equilibrium produces multiple stable phases: cold neutral (CNM), warm neutral (WNM), hot ionized (HIM).
**Atom or composite:** Composite.
**Cost model:** Cooling-curve equilibria.
**Real wall?** Yes — pressure equilibrium maintained.
**Cross-domain wiring:** Multi-phase equilibrium shared with combustion / chemistry-equilibrium primitives.
**Notes:** McKee-Ostriker (1977); Wolfire et al.

### supernova-remnant (cross-domain alias: `snr`)
**Domain:** Astrophysics / Cosmology
**Definition:** Shocked ejecta + ISM phases: free expansion → Sedov-Taylor → snowplow → fade-out; cosmic-ray acceleration.
**Atom or composite:** Composite.
**Cost model:** Sedov-Taylor self-similar / hydro simulation.
**Real wall?** Yes — diffusive shock acceleration central.
**Cross-domain wiring:** Self-similar blast wave shared with physics-diffusion explosion problems.
**Notes:** Sedov (1959), Taylor (1950).

### rankine-hugoniot (cross-domain alias: `rh-jump`)
**Domain:** Astrophysics / Cosmology
**Definition:** Jump conditions across shock from mass, momentum, energy conservation; strong shock limit ρ_2/ρ_1=4 (γ=5/3).
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — assumes inviscid steady shock.
**Cross-domain wiring:** Conservation-law jumps shared with hydro Riemann solvers.
**Notes:** Rankine (1870), Hugoniot (1887).

### oblique-shock (cross-domain alias: `oblique-mhd-shock`)
**Domain:** Astrophysics / Cosmology
**Definition:** Shock with B at angle to normal; fast/slow/intermediate MHD shock families per Mach number.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — solution sometimes non-unique.
**Cross-domain wiring:** Mode-dependent shock jumps shared with electromagnetics-antennas wave-mode coupling at boundaries.
**Notes:** Goedbloed-Poedts.

### perpendicular-shock (cross-domain alias: `quasi-perp-shock`)
**Domain:** Astrophysics / Cosmology
**Definition:** Shock with B perpendicular to normal; efficient particle acceleration via SDA.
**Atom or composite:** Atom.
**Cost model:** PIC simulations to study.
**Real wall?** Yes — distinguishes acceleration channels.
**Cross-domain wiring:** PIC simulation tooling shared with electromagnetics-antennas plasma kinetics.
**Notes:** Burgess review.

### parallel-shock (cross-domain alias: `quasi-par-shock`)
**Domain:** Astrophysics / Cosmology
**Definition:** Shock with B parallel to normal; first-order Fermi (DSA) efficient.
**Atom or composite:** Atom.
**Cost model:** Hybrid simulations.
**Real wall?** Yes — injection mechanism still active research.
**Cross-domain wiring:** Diffusive shock acceleration shared with cosmic-ray transport.
**Notes:** Bell (1978); Blandford-Ostriker.

### cosmic-ray-diffusion (cross-domain alias: `cr-diffusion`)
**Domain:** Astrophysics / Cosmology
**Definition:** CR transport in galactic B field modeled as diffusion D(E) ∝ E^δ with δ ~0.3–0.6.
**Atom or composite:** Composite.
**Cost model:** Diffusion PDE; GALPROP/DRAGON.
**Real wall?** Yes — uncertainty in halo size, sources.
**Cross-domain wiring:** Diffusion-advection PDE shared with physics-diffusion; energy-dependent diffusion analog of dispersion.
**Notes:** Berezinsky et al.

### cr-knee (cross-domain alias: `knee-spectrum`)
**Domain:** Astrophysics / Cosmology
**Definition:** Steepening of CR all-particle spectrum at ~3×10¹⁵ eV; likely transition between galactic sources or rigidity cutoff.
**Atom or composite:** Atom.
**Cost model:** Spectral measurement.
**Real wall?** Yes — composition change accompanies.
**Cross-domain wiring:** Spectral-break diagnostic shared with signal-processing-rf.
**Notes:** KASCADE, IceTop measurements.

### cr-ankle (cross-domain alias: `ankle-spectrum`)
**Domain:** Astrophysics / Cosmology
**Definition:** Flattening of CR spectrum at ~5×10¹⁸ eV; possible galactic-to-extragalactic transition or proton dip.
**Atom or composite:** Atom.
**Cost model:** Spectral measurement.
**Real wall?** Yes — source composition debated.
**Cross-domain wiring:** Same spectral-break analysis tooling.
**Notes:** Auger, Telescope Array.

### gzk-cutoff (cross-domain alias: `gzk`)
**Domain:** Astrophysics / Cosmology
**Definition:** Energy cutoff ~5×10¹⁹ eV from proton-CMB photopion interaction; horizon ~50 Mpc for UHECRs.
**Atom or composite:** Atom.
**Cost model:** Algebraic from cross-section.
**Real wall?** Yes — confirmed by Auger.
**Cross-domain wiring:** Photonuclear cross-section bridges to photonics-optics atomic data.
**Notes:** Greisen, Zatsepin, Kuzmin (1966).

### cr-anisotropy (cross-domain alias: `cr-dipole`)
**Domain:** Astrophysics / Cosmology
**Definition:** Small (10⁻³) dipole anisotropy of CR arrival directions; constrains source distribution + diffusion.
**Atom or composite:** Atom.
**Cost model:** Likelihood analysis on event maps.
**Real wall?** Yes — multipoles diluted by magnetic deflection.
**Cross-domain wiring:** Spherical-harmonic anisotropy analysis shared with CMB pipeline.
**Notes:** Auger above 8 EeV detection.

### cr-sources (cross-domain alias: `snr-agn-grb-cr`)
**Domain:** Astrophysics / Cosmology
**Definition:** Candidate accelerators: SNRs (galactic), AGN jets, GRBs, starburst galaxies, tidal disruption events (extragalactic).
**Atom or composite:** Composite.
**Cost model:** Population synthesis.
**Real wall?** Yes — IceCube ν-blazar association points to AGN.
**Cross-domain wiring:** Source identification = multi-messenger correlation; statistics-probability hypothesis testing.
**Notes:** Hillas plot (1984).

### galprop-dragon-picard (cross-domain alias: `cr-propagation-codes`)
**Domain:** Astrophysics / Cosmology
**Definition:** Numerical CR propagation codes solving diffusion-loss equation on galaxy grid; fit to secondary/primary ratios.
**Atom or composite:** Composite (toolchain).
**Cost model:** PDE solve hours-days.
**Real wall?** Yes — degeneracies in diffusion + halo size.
**Cross-domain wiring:** PDE solver + Bayesian inference combination shared across pipelines.
**Notes:** Strong-Moskalenko; Evoli et al.

### heliospheric-modulation (cross-domain alias: `solar-mod`)
**Domain:** Astrophysics / Cosmology
**Definition:** Suppression of low-energy galactic CR flux at Earth due to solar wind / heliospheric magnetic field; force-field approximation.
**Atom or composite:** Composite.
**Cost model:** 1D Parker eq / 3D MHD.
**Real wall?** Yes — varies with solar cycle.
**Cross-domain wiring:** Convection-diffusion shared with atmospheric science.
**Notes:** Parker (1965); Gleeson-Axford.

## 13. Multi-Messenger Astronomy & Observational

### gw-em-multimessenger (cross-domain alias: `gw+em`)
**Domain:** Astrophysics / Cosmology
**Definition:** Joint GW + EM detection providing host identification, distance ladder, EOS constraints; GW170817 archetype.
**Atom or composite:** Composite (pipeline).
**Cost model:** Real-time alert + follow-up campaigns.
**Real wall?** Yes — sky-localization area drives follow-up cost.
**Cross-domain wiring:** Coincidence-detection statistics shared with signal-processing-rf event matching; statistics-probability cross-correlation.
**Notes:** GW170817 / AT2017gfo / GRB170817A.

### gw-neutrino-multimessenger (cross-domain alias: `gw+ν`)
**Domain:** Astrophysics / Cosmology
**Definition:** Joint GW + high-energy neutrino searches around mergers/accretion events; very deep limits so far.
**Atom or composite:** Composite.
**Cost model:** Time-windowed coincidence.
**Real wall?** Yes — backgrounds dominate.
**Cross-domain wiring:** Cross-detector time correlation shared with PTA cross-correlation pipelines.
**Notes:** ANTARES, IceCube collaborations.

### icecube-neutrino (cross-domain alias: `icecube`)
**Domain:** Astrophysics / Cosmology
**Definition:** km³ Cherenkov detector in Antarctic ice; detects astrophysical ν of TeV–PeV; first source association TXS 0506+056.
**Atom or composite:** Composite (instrument + pipeline).
**Cost model:** Real-time alerts + offline catalog.
**Real wall?** Yes — atmospheric muon background.
**Cross-domain wiring:** Cherenkov-light reconstruction shared with photonics-optics imaging.
**Notes:** Aartsen et al.; IceCube-Gen2 plans.

### high-energy-neutrinos (cross-domain alias: `astro-ν`)
**Domain:** Astrophysics / Cosmology
**Definition:** Astrophysical ν above 100 TeV; spectrum E^(−2.5); origins blazars, TDEs, starburst.
**Atom or composite:** Composite.
**Cost model:** ν-flux measurement.
**Real wall?** Yes — flavor ratios distinguish source physics.
**Cross-domain wiring:** Particle physics interaction with photonics-optics calibration data.
**Notes:** Waxman-Bahcall bound.

### uhecr (cross-domain alias: `ultra-high-energy-cr`)
**Domain:** Astrophysics / Cosmology
**Definition:** Cosmic rays above 10¹⁸ eV; mostly extragalactic; composition transitions to heavier nuclei at high E.
**Atom or composite:** Composite.
**Cost model:** Auger surface + fluorescence detector arrays.
**Real wall?** Yes — magnetic deflection complicates source finding.
**Cross-domain wiring:** Extensive air shower reconstruction shared with high-energy detector primitives.
**Notes:** Auger, Telescope Array.

### optical-ir-followup (cross-domain alias: `ztf-decam-followup`)
**Domain:** Astrophysics / Cosmology
**Definition:** Wide-field optical/IR imaging campaigns triggered by alerts (GW, ν, GRB) to find EM counterparts.
**Atom or composite:** Composite.
**Cost model:** Tiling + image differencing pipelines.
**Real wall?** Yes — kilonova fade timescale ~ days.
**Cross-domain wiring:** Difference imaging shared with signal-processing-rf change detection.
**Notes:** ZTF, DECam, LSST.

### too-observing (cross-domain alias: `target-of-opportunity`)
**Domain:** Astrophysics / Cosmology
**Definition:** Interruptive observation triggered by transient alerts; uses queue-scheduled facilities.
**Atom or composite:** Composite.
**Cost model:** Operations and scheduling overhead.
**Real wall?** Yes — slew times, telescope availability.
**Cross-domain wiring:** Scheduling = control-numerical-opt resource allocation.
**Notes:** Standard astronomical practice.

### gw170817-event (cross-domain alias: `emgw170817`)
**Domain:** Astrophysics / Cosmology
**Definition:** First NS-NS merger detected in GW + γ-rays + optical + radio; established multi-messenger astronomy; constrained EOS, H₀.
**Atom or composite:** Composite.
**Cost model:** End-to-end pipeline triggered.
**Real wall?** Yes — single event; statistics limited.
**Cross-domain wiring:** Demonstrates all multi-messenger primitives integrated.
**Notes:** Abbott et al. 2017.

### photometry (cross-domain alias: `aperture-photom`)
**Domain:** Astrophysics / Cosmology
**Definition:** Flux measurement of point/extended sources via aperture or PSF fitting; calibrated to magnitude system.
**Atom or composite:** Composite.
**Cost model:** O(N_src) PSF fits.
**Real wall?** Yes — crowding, sky subtraction, PSF variation.
**Cross-domain wiring:** PSF deconvolution shared with photonics-optics, signal-processing-rf.
**Notes:** SExtractor, DAOPHOT pipelines.

### astrometry-gaia (cross-domain alias: `gaia-edr3`, `gaia-dr3`)
**Domain:** Astrophysics / Cosmology
**Definition:** All-sky stellar position/parallax/proper-motion catalog from ESA Gaia; ~1 billion sources, μas precision.
**Atom or composite:** Composite.
**Cost model:** Mission processing PB-scale.
**Real wall?** Yes — systematics in faint/binary stars.
**Cross-domain wiring:** Iterative global astrometric solution = control-numerical-opt least-squares; cross-match to ml-training entity resolution.
**Notes:** Gaia Collaboration (2022) DR3.

### spectroscopy-slit (cross-domain alias: `long-slit-spec`)
**Domain:** Astrophysics / Cosmology
**Definition:** Disperses light through narrow slit; cross-dispersed spectrum yields velocity and composition.
**Atom or composite:** Composite.
**Cost model:** O(N_pix) per exposure.
**Real wall?** Yes — slit losses, alignment.
**Cross-domain wiring:** Dispersion + wavelength solution shared with photonics-optics spectrographs.
**Notes:** Standard astronomy practice.

### ifu-spectroscopy (cross-domain alias: `ifu`, `integral-field-spec`)
**Domain:** Astrophysics / Cosmology
**Definition:** Spatially-resolved spectroscopy via lenslet/fiber bundle; produces 3D data cube (x,y,λ).
**Atom or composite:** Composite.
**Cost model:** 3D data; large file sizes.
**Real wall?** Yes — fiber/lenslet calibration.
**Cross-domain wiring:** Data cubes mirror photonics-optics hyperspectral imaging.
**Notes:** MUSE, KMOS, SAMI.

### multi-object-spectroscopy (cross-domain alias: `mos`)
**Domain:** Astrophysics / Cosmology
**Definition:** Simultaneous spectra of hundreds of targets via fibers or slit masks; enables redshift surveys.
**Atom or composite:** Composite.
**Cost model:** Mass production; fiber positioner robotics.
**Real wall?** Yes — fiber collisions, target placement.
**Cross-domain wiring:** Combinatorial fiber allocation = control-numerical-opt assignment.
**Notes:** DESI, 4MOST, PFS.

### echelle-spectroscopy (cross-domain alias: `echelle`, `high-r-spec`)
**Domain:** Astrophysics / Cosmology
**Definition:** High-resolution spectrograph using cross-dispersed echelle grating; R = 50,000–200,000.
**Atom or composite:** Composite.
**Cost model:** Long exposures; bright targets.
**Real wall?** Yes — order-merging, calibration.
**Cross-domain wiring:** Cross-dispersion grating design shared with photonics-optics.
**Notes:** HARPS, ESPRESSO, EXPRES.

### vlbi (cross-domain alias: `very-long-baseline`)
**Domain:** Astrophysics / Cosmology
**Definition:** Radio interferometry with continental/global baselines; μas angular resolution; underpins EHT.
**Atom or composite:** Composite.
**Cost model:** Cross-correlation petabyte-scale.
**Real wall?** Yes — atmospheric phase, sparse uv-coverage.
**Cross-domain wiring:** Same correlator-FFT machinery as signal-processing-rf.
**Notes:** EHT collaboration; Goddard VLBI.

### gravity-interferometer (cross-domain alias: `gravity-vlti`)
**Domain:** Astrophysics / Cosmology
**Definition:** Near-IR interferometric instrument at VLTI achieving 50 μas astrometry; observed S2 orbiting Sgr A*.
**Atom or composite:** Composite.
**Cost model:** Beam-combination + fringe tracking.
**Real wall?** Yes — atmospheric turbulence; needs AO.
**Cross-domain wiring:** Beam-combining and fringe tracking shared with photonics-optics interferometry.
**Notes:** GRAVITY Collaboration (2018) Sgr A* GR test.

### polarimetry (cross-domain alias: `pol-astro`)
**Domain:** Astrophysics / Cosmology
**Definition:** Measurement of Stokes (I,Q,U,V) for astronomical sources; probes magnetic fields, dust, synchrotron.
**Atom or composite:** Composite.
**Cost model:** Calibration-heavy.
**Real wall?** Yes — instrumental polarization.
**Cross-domain wiring:** Stokes-parameter machinery shared with photonics-optics, electromagnetics-antennas.
**Notes:** Tinbergen polarimetry text.

### time-domain-astronomy (cross-domain alias: `tda`)
**Domain:** Astrophysics / Cosmology
**Definition:** Repeated wide-field observation to find transients, variables, periodic signals; LSST scales it up.
**Atom or composite:** Composite.
**Cost model:** PB/night raw; real-time pipeline.
**Real wall?** Yes — false-positive rates with image artifacts.
**Cross-domain wiring:** Change detection = signal-processing-rf novelty detection; classification = ml-training.
**Notes:** ZTF, ATLAS; LSST.

### transient-pipeline (cross-domain alias: `transient-broker`)
**Domain:** Astrophysics / Cosmology
**Definition:** Pipeline scanning subtracted images for candidates, applying ML classification, vetting, and alert distribution.
**Atom or composite:** Composite.
**Cost model:** Real-time low-latency stream processing.
**Real wall?** Yes — purity vs. completeness trade-off.
**Cross-domain wiring:** Stream processing + ML inference uses ml-training infrastructure directly.
**Notes:** ANTARES, ALERCE, FINK.

## 14. Cosmological Data Analysis & Lensing

### power-spectrum-estimation (cross-domain alias: `pkl-est`)
**Domain:** Astrophysics / Cosmology
**Definition:** Estimating P(k) or C_l from masked, partial-sky data with noise; pseudo-Cl, QML, optimal estimators.
**Atom or composite:** Composite.
**Cost model:** O(l_max³) for pseudo-Cl.
**Real wall?** Yes — mask coupling matrix expensive.
**Cross-domain wiring:** PSD estimation shared with signal-processing-rf periodograms.
**Notes:** Tegmark QML; Hivon MASTER.

### pseudo-cl (cross-domain alias: `pcl`)
**Domain:** Astrophysics / Cosmology
**Definition:** Direct spherical-harmonic estimate on masked map; biased by mask leakage; deconvolved by mode-coupling matrix.
**Atom or composite:** Composite.
**Cost model:** O(N^(3/2)) per spectrum.
**Real wall?** Yes — coupling matrix inversion can be unstable.
**Cross-domain wiring:** Windowing bias correction shared with signal-processing-rf periodogram leakage.
**Notes:** Hivon et al. (2002); Wandelt-Hivon-Górski.

### master-algorithm (cross-domain alias: `master-decoupling`)
**Domain:** Astrophysics / Cosmology
**Definition:** Deconvolution of mask-induced mode coupling via inversion of M_ll'; produces unbiased pseudo-Cl.
**Atom or composite:** Composite.
**Cost model:** Matrix invert O(l_max³).
**Real wall?** Yes — singular for narrow masks.
**Cross-domain wiring:** Inverse-problem deconvolution shared with signal-processing-rf.
**Notes:** Hivon et al. (2002).

### namaster (cross-domain alias: `namaster-lib`)
**Domain:** Astrophysics / Cosmology
**Definition:** Practical implementation of MASTER for scalar, spin, polarized fields with bandpower binning.
**Atom or composite:** Composite (library).
**Cost model:** Python wrapper around C; fast.
**Real wall?** No.
**Cross-domain wiring:** Library architecture reusable as a signal-processing-rf wrapper.
**Notes:** Alonso et al. (2019).

### gaussian-likelihood (cross-domain alias: `g-likelihood`)
**Domain:** Astrophysics / Cosmology
**Definition:** −2 ln L = (d−μ)^T C⁻¹ (d−μ) + log det C; default in cosmology likelihood codes.
**Atom or composite:** Atom.
**Cost model:** O(N³) covariance solve.
**Real wall?** Yes — covariance estimation dominates.
**Cross-domain wiring:** Same likelihood as statistics-probability multivariate Gaussian; analogous to ml-training MSE loss.
**Notes:** Standard.

### lognormal-likelihood (cross-domain alias: `lognormal-lh`)
**Domain:** Astrophysics / Cosmology
**Definition:** Approximate non-Gaussian PDF for cosmological fields; transform field → ln(1+δ) before Gaussian.
**Atom or composite:** Composite.
**Cost model:** As Gaussian + transform.
**Real wall?** Yes — non-Gaussianity at low-l requires better.
**Cross-domain wiring:** Variance-stabilizing transform shared with statistics-probability Box-Cox.
**Notes:** Coles-Jones (1991).

### bayesian-cosmology (cross-domain alias: `cosmomc-cobaya`)
**Domain:** Astrophysics / Cosmology
**Definition:** Bayesian parameter inference using Boltzmann code (CAMB/CLASS) + samplers (CosmoMC, Cobaya).
**Atom or composite:** Composite.
**Cost model:** 10⁵–10⁶ likelihood evals.
**Real wall?** Yes — likelihood eval is bottleneck.
**Cross-domain wiring:** Same inference framework as ml-training Bayesian deep learning.
**Notes:** Lewis-Bridle; Torrado-Lewis Cobaya.

### polychord (cross-domain alias: `polychord-ns`)
**Domain:** Astrophysics / Cosmology
**Definition:** Slice-sampling-based nested sampler for high-dimensional cosmology problems with evidence estimation.
**Atom or composite:** Composite.
**Cost model:** O(N_live N_dim²) per iteration.
**Real wall?** Yes — high-dim posteriors stress samplers.
**Cross-domain wiring:** Slice sampling shared with statistics-probability MCMC kernels.
**Notes:** Handley, Hobson, Lasenby (2015).

### fisher-information (cross-domain alias: `fisher-matrix`)
**Domain:** Astrophysics / Cosmology
**Definition:** F_ij = ⟨∂_i ln L ∂_j ln L⟩; gives parameter-covariance forecast (Cramér-Rao).
**Atom or composite:** Atom.
**Cost model:** Numerical derivatives + likelihood eval.
**Real wall?** Yes — only valid in Gaussian limit.
**Cross-domain wiring:** Same Fisher info used in ml-training optimization, statistics-probability bounds.
**Notes:** Tegmark-Taylor-Heavens (1997).

### parameter-forecasting (cross-domain alias: `forecast`)
**Domain:** Astrophysics / Cosmology
**Definition:** Predict parameter errors for future surveys using Fisher / MCMC mocks; informs experiment design.
**Atom or composite:** Composite.
**Cost model:** Fisher fast; full mocks expensive.
**Real wall?** Yes — assumes Gaussian/known model.
**Cross-domain wiring:** Same forecasting framework as ml-training learning-curve projection.
**Notes:** Albrecht et al. DETF.

### mcmc-for-cosmology (cross-domain alias: `cosmo-mcmc`)
**Domain:** Astrophysics / Cosmology
**Definition:** Metropolis-Hastings sampling of (Ω_m, Ω_b h², n_s, A_s, τ, h, w_0, w_a, …) posteriors.
**Atom or composite:** Composite.
**Cost model:** 10⁴–10⁶ samples; days on cluster.
**Real wall?** Yes — chain convergence (R−1 < 0.01).
**Cross-domain wiring:** Adopted directly from statistics-probability MCMC.
**Notes:** Lewis-Bridle CosmoMC.

### simulation-based-inference (cross-domain alias: `sbi`, `lfi`)
**Domain:** Astrophysics / Cosmology
**Definition:** Inference using forward simulators rather than tractable likelihood; neural-network density estimators (NPE, NLE, NRE).
**Atom or composite:** Composite.
**Cost model:** Sim cost dominates.
**Real wall?** Yes — simulation budget.
**Cross-domain wiring:** Direct ml-training territory (normalizing flows).
**Notes:** Cranmer et al. (2020); SBI package.

### field-level-inference (cross-domain alias: `flu`)
**Domain:** Astrophysics / Cosmology
**Definition:** Bayesian inference on entire 3D matter field rather than summary statistics; exploits maximum information.
**Atom or composite:** Composite.
**Cost model:** Very expensive HMC over millions of voxels.
**Real wall?** Yes — gradient through forward model.
**Cross-domain wiring:** Differentiable simulators bridge to ml-training auto-diff.
**Notes:** BORG (Jasche-Wandelt).

### gravitational-lensing-strong (cross-domain alias: `strong-lensing`)
**Domain:** Astrophysics / Cosmology
**Definition:** Multiple images / arcs of background source from massive foreground lens; geometry yields H₀ via time delays.
**Atom or composite:** Composite.
**Cost model:** Lens-mass reconstruction.
**Real wall?** Yes — mass-sheet degeneracy.
**Cross-domain wiring:** Inverse problem identical to electromagnetics-antennas inverse scattering.
**Notes:** Refsdal (1964); Suyu H0LiCOW.

### gravitational-lensing-weak (cross-domain alias: `cosmic-shear`)
**Domain:** Astrophysics / Cosmology
**Definition:** Statistical distortion of background galaxies by foreground LSS; measures matter power spectrum.
**Atom or composite:** Composite.
**Cost model:** Catalog of galaxy shapes; 2pt statistics.
**Real wall?** Yes — shape-measurement systematics.
**Cross-domain wiring:** Shape measurement is ml-training computer vision; correlation = signal-processing-rf.
**Notes:** Kaiser-Squires (1993); DES Y3.

### microlensing (cross-domain alias: `microlensing-survey`)
**Domain:** Astrophysics / Cosmology
**Definition:** Brightening of background star by stellar lens; probes compact dark objects, exoplanets.
**Atom or composite:** Composite.
**Cost model:** Long-time photometry.
**Real wall?** Yes — degeneracies in light-curve modeling.
**Cross-domain wiring:** Transient time-series analysis shared with signal-processing-rf.
**Notes:** Paczynski (1986); OGLE, MOA, KMTNet.

### thin-lens-approx (cross-domain alias: `tla`)
**Domain:** Astrophysics / Cosmology
**Definition:** Deflection α evaluated at lens plane; valid when lens depth ≪ distances.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — breaks for line-of-sight structures.
**Cross-domain wiring:** Thin-lens identical to photonics-optics geometric optics thin lens.
**Notes:** SEF (Schneider-Ehlers-Falco) text.

### einstein-ring (cross-domain alias: `e-ring`, `θ_e`)
**Domain:** Astrophysics / Cosmology
**Definition:** θ_E = √(4GM D_ls / c² D_l D_s); ring image of source perfectly aligned with point lens.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — rarely perfect alignment.
**Cross-domain wiring:** Caustic geometry shared with photonics-optics aberration patterns.
**Notes:** Einstein (1936).

### time-delay-h0 (cross-domain alias: `h0licow`, `tdcosmo`)
**Domain:** Astrophysics / Cosmology
**Definition:** H₀ from time delays between strong-lens images: Δt ∝ (1/H₀)D_Δt; independent of distance ladder.
**Atom or composite:** Composite.
**Cost model:** Years of monitoring + modeling.
**Real wall?** Yes — mass-sheet degeneracy.
**Cross-domain wiring:** Light-curve cross-correlation = signal-processing-rf delay estimation.
**Notes:** Refsdal (1964); H0LiCOW; TDCOSMO.

### cosmic-shear (cross-domain alias: `ξ+ξ-`, `2pt-shear`)
**Domain:** Astrophysics / Cosmology
**Definition:** 2pt function of galaxy ellipticities; measures projected matter power spectrum.
**Atom or composite:** Composite.
**Cost model:** O(N²) brute force; tree codes faster.
**Real wall?** Yes — intrinsic alignments contaminate.
**Cross-domain wiring:** Correlation function analysis shared with statistics-probability.
**Notes:** Bartelmann-Schneider review.

### kids-survey (cross-domain alias: `kids`)
**Domain:** Astrophysics / Cosmology
**Definition:** Kilo-Degree Survey: optical/NIR weak lensing survey; ~1000 deg²; constrained S₈.
**Atom or composite:** Composite.
**Cost model:** Cosmology pipeline.
**Real wall?** Yes — S₈ tension with Planck.
**Cross-domain wiring:** Same shear pipeline as DES, Euclid.
**Notes:** Heymans et al. KiDS-1000.

### des-survey (cross-domain alias: `des`)
**Domain:** Astrophysics / Cosmology
**Definition:** Dark Energy Survey: 5000 deg² optical survey; weak lensing + galaxy clustering + clusters + SNe Ia.
**Atom or composite:** Composite.
**Cost model:** End-to-end cosmology pipeline.
**Real wall?** Yes — photo-z systematics.
**Cross-domain wiring:** Joint 3x2pt analysis = ml-training multi-task inference.
**Notes:** DES Y3 papers.

### euclid-forecast (cross-domain alias: `euclid-l3`)
**Domain:** Astrophysics / Cosmology
**Definition:** ESA Euclid mission survey: 15,000 deg² imaging + slitless spectroscopy; precision DE constraints.
**Atom or composite:** Composite.
**Cost model:** Petabyte data + complex pipeline.
**Real wall?** Yes — systematics dominate.
**Cross-domain wiring:** Slitless spec + photo shared with photonics-optics dispersive optics.
**Notes:** Laureijs et al.; launched 2023.

### kaiser-squires-mass-recon (cross-domain alias: `ks-recon`)
**Domain:** Astrophysics / Cosmology
**Definition:** Maps shear field → convergence κ via 2D Wiener / inverse Laplacian; foundation of mass mapping.
**Atom or composite:** Composite.
**Cost model:** FFT-based; O(N log N).
**Real wall?** Yes — finite-field boundary leakage.
**Cross-domain wiring:** Inverse-Laplacian recovery shared with electromagnetics-antennas E-mode reconstruction.
**Notes:** Kaiser-Squires (1993).

### aperture-mass (cross-domain alias: `m_ap`)
**Domain:** Astrophysics / Cosmology
**Definition:** Compensated filter on shear field giving signal-to-noise per aperture for cluster/structure detection.
**Atom or composite:** Atom (matched filter).
**Cost model:** Convolutions.
**Real wall?** No.
**Cross-domain wiring:** Compensated filter = signal-processing-rf wavelet detector.
**Notes:** Schneider (1996).

### b-modes-weak-lensing (cross-domain alias: `wl-b`)
**Domain:** Astrophysics / Cosmology
**Definition:** Curl-mode shear; nonzero indicates systematics or non-LSS sources (intrinsic alignments).
**Atom or composite:** Atom.
**Cost model:** E/B decomposition.
**Real wall?** Yes — sanity check for any WL survey.
**Cross-domain wiring:** Direct analog to CMB B-modes; same machinery.
**Notes:** Crittenden et al. (2002).

## 15. Reionization, 21cm, Compact Objects, SZ, GW Detector Noise

### twenty-one-cm-cosmology (cross-domain alias: `21cm`)
**Domain:** Astrophysics / Cosmology
**Definition:** Hyperfine HI line at λ=21 cm probes neutral gas through cosmic dawn, reionization, dark ages.
**Atom or composite:** Composite.
**Cost model:** Radio surveys; foreground subtraction expensive.
**Real wall?** Yes — galactic synchrotron foreground 10⁴× signal.
**Cross-domain wiring:** Foreground subtraction = signal-processing-rf source separation.
**Notes:** Furlanetto-Oh-Briggs review.

### brightness-temperature-21cm (cross-domain alias: `δ_t_b`)
**Domain:** Astrophysics / Cosmology
**Definition:** δT_b ≈ 27 mK x_HI (1+δ) (T_S−T_γ)/T_S [(1+z)/10]^(1/2) ...; differential brightness against CMB.
**Atom or composite:** Composite.
**Cost model:** Radiative transfer.
**Real wall?** Yes — sign depends on T_S vs T_γ.
**Cross-domain wiring:** Brightness-temp formalism shared with electromagnetics-antennas radiometry.
**Notes:** Madau-Meiksin-Rees (1997).

### spin-temperature (cross-domain alias: `t_s`)
**Domain:** Astrophysics / Cosmology
**Definition:** Population ratio of HI hyperfine levels: n_1/n_0 = 3 exp(−T_*/T_S); coupled to T_γ and T_K via collisions and Wouthuysen-Field.
**Atom or composite:** Composite.
**Cost model:** Algebraic.
**Real wall?** Yes — sets sign and amplitude of 21cm signal.
**Cross-domain wiring:** Two-level-system population shared with photonics-optics laser atomic physics.
**Notes:** Field (1958).

### wouthuysen-field (cross-domain alias: `wf-coupling`)
**Domain:** Astrophysics / Cosmology
**Definition:** Ly-α photons mix HI hyperfine levels via Ly-α scattering, coupling T_S to T_K; central to cosmic dawn.
**Atom or composite:** Composite.
**Cost model:** Algebraic / Monte Carlo.
**Real wall?** Yes — first stars determine onset.
**Cross-domain wiring:** Resonant scattering coupling shared with photonics-optics.
**Notes:** Wouthuysen (1952), Field (1958).

### x-ray-heating-21cm (cross-domain alias: `x-heat`)
**Domain:** Astrophysics / Cosmology
**Definition:** X-rays from first BHs/HMXBs heat IGM neutral hydrogen, transitioning 21cm from absorption to emission.
**Atom or composite:** Composite.
**Cost model:** Coupled photon transport.
**Real wall?** Yes — X-ray source spectra uncertain.
**Cross-domain wiring:** Long-mean-free-path photon transport mirrors physics-diffusion.
**Notes:** Pritchard-Furlanetto review.

### lya-coupling (cross-domain alias: `lya-pumping`)
**Domain:** Astrophysics / Cosmology
**Definition:** First-stars Ly-α emission saturates Wouthuysen-Field, locking T_S to T_K; ignites cosmic dawn 21cm absorption.
**Atom or composite:** Composite.
**Cost model:** RT + UV background.
**Real wall?** Yes — pop-III star spectra uncertain.
**Cross-domain wiring:** Effectiveness depends on photonics-optics resonant scattering.
**Notes:** Madau et al.

### edges-experiment (cross-domain alias: `edges`)
**Domain:** Astrophysics / Cosmology
**Definition:** Global 21cm absorption experiment reporting 78 MHz feature; controversial extra-cold IGM signal at z~17.
**Atom or composite:** Composite.
**Cost model:** Antenna + radiometer.
**Real wall?** Yes — systematics debated.
**Cross-domain wiring:** Calibrated radiometry = electromagnetics-antennas absolute radiometer technique.
**Notes:** Bowman et al. (2018).

### hera-21cm (cross-domain alias: `hera`)
**Domain:** Astrophysics / Cosmology
**Definition:** Hydrogen Epoch of Reionization Array: redundant short-baseline interferometer to measure 21cm power spectrum.
**Atom or composite:** Composite.
**Cost model:** Cross-correlation pipeline.
**Real wall?** Yes — foreground avoidance via delay spectrum.
**Cross-domain wiring:** Delay-spectrum technique shared with signal-processing-rf chirp filters.
**Notes:** DeBoer et al.

### ska-21cm-forecast (cross-domain alias: `ska`)
**Domain:** Astrophysics / Cosmology
**Definition:** Square Kilometre Array: km²-collecting-area radio telescope; tomographic 21cm imaging of EoR.
**Atom or composite:** Composite.
**Cost model:** Exabyte data rates; HPC pipeline.
**Real wall?** Yes — calibration at ν below 200 MHz.
**Cross-domain wiring:** Wide-field synthesis imaging shared with electromagnetics-antennas, signal-processing-rf.
**Notes:** SKAO project office.

### pulsar-timing-glitch (cross-domain alias: `pulsar-glitch`)
**Domain:** Astrophysics / Cosmology
**Definition:** Sudden spin-up of pulsar Δν/ν ~10⁻⁶ followed by exponential recovery; probes NS interior superfluid.
**Atom or composite:** Composite.
**Cost model:** Timing residual analysis.
**Real wall?** Yes — superfluid microphysics unsolved.
**Cross-domain wiring:** Step detection in time series = signal-processing-rf changepoint detection.
**Notes:** Anderson-Itoh (1975).

### pulsar-magnetosphere (cross-domain alias: `psr-magnetosphere`)
**Domain:** Astrophysics / Cosmology
**Definition:** Co-rotating plasma magnetosphere with light cylinder; force-free / kinetic models predict emission geometry.
**Atom or composite:** Composite.
**Cost model:** GR-FFE or PIC simulations.
**Real wall?** Yes — emission location debated.
**Cross-domain wiring:** Force-free electrodynamics shared with BZ mechanism.
**Notes:** Goldreich-Julian (1969); Spitkovsky.

### millisecond-pulsar (cross-domain alias: `msp`)
**Domain:** Astrophysics / Cosmology
**Definition:** Recycled pulsars with P < 30 ms spun up by accretion; basis for PTA cosmology.
**Atom or composite:** Composite.
**Cost model:** Long-baseline timing.
**Real wall?** Yes — timing noise floor sets PTA sensitivity.
**Cross-domain wiring:** Atomic-clock-like stability shared with photonics-optics frequency standards.
**Notes:** Backer et al. (1982).

### x-ray-pulsar (cross-domain alias: `accretion-pulsar`)
**Domain:** Astrophysics / Cosmology
**Definition:** NS in HMXB channeling accretion to magnetic poles; X-ray pulses at spin period; cyclotron features in spectrum.
**Atom or composite:** Composite.
**Cost model:** Spectral-timing analysis.
**Real wall?** Yes — accretion-torque modeling complex.
**Cross-domain wiring:** Cyclotron resonance shared with photonics-optics magnetic-field diagnostics.
**Notes:** NICER, NuSTAR observations.

### fast-radio-burst (cross-domain alias: `frb`)
**Domain:** Astrophysics / Cosmology
**Definition:** ms-duration radio transients with extragalactic DM; some repeat; sources include magnetars (SGR 1935+2154).
**Atom or composite:** Composite.
**Cost model:** Real-time radio searches.
**Real wall?** Yes — coherent emission mechanism unclear.
**Cross-domain wiring:** DM-based cosmography uses DM-z relation; signal-processing-rf dedispersion staple.
**Notes:** Lorimer et al. (2007); CHIME/FRB.

### pulsar-wind-nebula (cross-domain alias: `pwn`)
**Domain:** Astrophysics / Cosmology
**Definition:** Synchrotron-emitting nebula powered by relativistic pulsar wind; Crab Nebula archetype.
**Atom or composite:** Composite.
**Cost model:** MHD + radiative modeling.
**Real wall?** Yes — σ-problem (low magnetization at termination shock).
**Cross-domain wiring:** Synchrotron SED shared with AGN modeling.
**Notes:** Rees-Gunn (1974); Kennel-Coroniti.

### magnetar-burst (cross-domain alias: `sgr-burst`)
**Domain:** Astrophysics / Cosmology
**Definition:** Soft gamma repeater (SGR) outbursts and giant flares from magnetic-field rearrangement in magnetar crust.
**Atom or composite:** Composite.
**Cost model:** X/γ-ray timing.
**Real wall?** Yes — trigger mechanism unsolved.
**Cross-domain wiring:** Magnetar-FRB link is hot multi-messenger topic.
**Notes:** Thompson-Duncan; SGR 1806−20 (2004).

### thermal-sz (cross-domain alias: `tsz`)
**Domain:** Astrophysics / Cosmology
**Definition:** Inverse-Compton up-scattering of CMB photons by hot cluster electrons; ΔT/T characteristic ν-dependence (null at 217 GHz).
**Atom or composite:** Atom.
**Cost model:** Algebraic spectral shape.
**Real wall?** Yes — distinguishes from CMB temperature anisotropy by ν-dependence.
**Cross-domain wiring:** Compton scattering spectrum shared with photonics-optics.
**Notes:** Sunyaev-Zel'dovich (1970, 1972).

### kinetic-sz (cross-domain alias: `ksz`)
**Domain:** Astrophysics / Cosmology
**Definition:** Doppler shift due to cluster bulk velocity along LOS; spectral signature identical to CMB.
**Atom or composite:** Atom.
**Cost model:** Velocity reconstruction needed.
**Real wall?** Yes — degenerate with primordial CMB without velocity tracer.
**Cross-domain wiring:** Doppler shift in scattering shared with signal-processing-rf clutter.
**Notes:** Sunyaev-Zel'dovich (1980).

### polarization-sz (cross-domain alias: `psz`)
**Domain:** Astrophysics / Cosmology
**Definition:** Polarized SZ signal from cluster transverse velocity + quadrupole CMB at cluster; smallest SZ effect.
**Atom or composite:** Atom.
**Cost model:** Very small; future surveys.
**Real wall?** Yes — orders below tSZ.
**Cross-domain wiring:** Polarization scattering shared with photonics-optics.
**Notes:** Sazonov-Sunyaev (1999).

### cmb-e-modes (cross-domain alias: `cmb-e`)
**Domain:** Astrophysics / Cosmology
**Definition:** Curl-free CMB polarization from scalar density perturbations + Thomson scattering at last scattering.
**Atom or composite:** Composite.
**Cost model:** Spin-2 SHT.
**Real wall?** Yes — fully exploited by Planck/ACT/SPT.
**Cross-domain wiring:** Same as weak-lensing E-modes; helicity decomposition shared.
**Notes:** Zaldarriaga-Seljak (1997).

### cmb-b-modes (cross-domain alias: `cmb-b`, `primordial-b`)
**Domain:** Astrophysics / Cosmology
**Definition:** Divergence-free CMB polarization; primary from inflationary tensors + lensing of E into B.
**Atom or composite:** Composite.
**Cost model:** Spin-2 SHT.
**Real wall?** Yes — instrumental polarization, dust dominate; r<0.036.
**Cross-domain wiring:** Delensing pipelines bridge CMB and LSS.
**Notes:** Kamionkowski et al.; BICEP/Keck.

### gw-detector-noise-quantum (cross-domain alias: `shot-noise-gw`)
**Domain:** Astrophysics / Cosmology
**Definition:** Photon shot noise / radiation-pressure noise; sets sensitivity at high/low frequencies.
**Atom or composite:** Composite.
**Cost model:** N/A (detector property).
**Real wall?** Yes — squeezed light reduces.
**Cross-domain wiring:** Standard quantum limit shared with photonics-optics; squeezing tech imported.
**Notes:** Caves (1981).

### gw-detector-noise-seismic (cross-domain alias: `seismic-noise`)
**Domain:** Astrophysics / Cosmology
**Definition:** Ground motion 10⁻⁷ m/√Hz below 10 Hz; isolated by multi-stage pendulum + active seismic.
**Atom or composite:** Composite.
**Real wall?** Yes — sets low-frequency cutoff.
**Cross-domain wiring:** Active vibration isolation = control-numerical-opt feedback control.
**Notes:** LIGO seismic isolation papers.

### gw-detector-noise-thermal (cross-domain alias: `thermal-noise-gw`)
**Domain:** Astrophysics / Cosmology
**Definition:** Brownian motion of mirror coatings/suspensions; dominant noise 50–200 Hz.
**Atom or composite:** Composite.
**Cost model:** Material-loss model.
**Real wall?** Yes — coating loss material limit.
**Cross-domain wiring:** Fluctuation-dissipation theorem = statistics-probability.
**Notes:** Saulson (1990).

### kagra-cryogenic (cross-domain alias: `kagra`)
**Domain:** Astrophysics / Cosmology
**Definition:** Underground GW interferometer with sapphire mirrors cooled to 20 K; reduces thermal noise.
**Atom or composite:** Composite.
**Cost model:** Cryogenic engineering overhead.
**Real wall?** Yes — cryo cooling complicates suspension.
**Cross-domain wiring:** Cryo-optics shared with photonics-optics low-noise photodetection.
**Notes:** Akutsu et al.

### ringdown-spectroscopy (cross-domain alias: `bh-spec`, `qnm-spec`)
**Domain:** Astrophysics / Cosmology
**Definition:** Identify multiple QNMs in BH ringdown; test no-hair theorem from frequency-damping ratios.
**Atom or composite:** Composite.
**Cost model:** Matched filtering with QNM templates.
**Real wall?** Yes — SNR per mode limited at current detectors.
**Cross-domain wiring:** Mode decomposition = signal-processing-rf Prony / matrix-pencil.
**Notes:** Dreyer et al. (2004); Berti review.

### no-hair-tests (cross-domain alias: `nht-test`)
**Domain:** Astrophysics / Cosmology
**Definition:** Bayesian tests of consistency between dominant + subdominant QNM frequencies as predicted by Kerr (M,a).
**Atom or composite:** Composite.
**Cost model:** Bayesian model selection.
**Real wall?** Yes — SNR per subdominant mode.
**Cross-domain wiring:** Model selection shared with statistics-probability.
**Notes:** Isi et al. GW150914 ringdown test.

### event-horizon-telescope (cross-domain alias: `eht`)
**Domain:** Astrophysics / Cosmology
**Definition:** Global mm-VLBI array imaging Sgr A* and M87* event-horizon-scale ring.
**Atom or composite:** Composite.
**Cost model:** Petabyte raw → correlator → CLEAN/RML imaging.
**Real wall?** Yes — sparse uv-coverage; atmosphere; needs regularization.
**Cross-domain wiring:** RML imaging shared with ml-training compressed sensing.
**Notes:** EHT Collaboration M87* (2019), Sgr A* (2022).

### vlbi-techniques (cross-domain alias: `vlbi-pipeline`)
**Domain:** Astrophysics / Cosmology
**Definition:** Cross-correlation of widely separated antennas with atomic-clock timestamps; produces visibilities V(u,v).
**Atom or composite:** Composite.
**Cost model:** Correlator FLOPs ∝ N_bl × T.
**Real wall?** Yes — fringe finding, clock stability.
**Cross-domain wiring:** Direct application of signal-processing-rf correlation; clocks from photonics-optics frequency combs.
**Notes:** Thompson-Moran-Swenson interferometry book.

### cmb-stage4 (cross-domain alias: `cmb-s4`)
**Domain:** Astrophysics / Cosmology
**Definition:** Next-generation ground-based CMB experiment targeting r<0.001, ν_eff, m_ν, dark-energy phenomenology.
**Atom or composite:** Composite.
**Cost model:** 500k detectors; petabyte data; HPC analysis.
**Real wall?** Yes — atmospheric, foreground, calibration.
**Cross-domain wiring:** Pipeline orchestration shared with all surveys; component separation = ml-training source separation.
**Notes:** Abazajian et al. CMB-S4 Science Book.

### litebird-mission (cross-domain alias: `litebird`)
**Domain:** Astrophysics / Cosmology
**Definition:** JAXA satellite mission targeting CMB B-modes at large angular scales (l<200); r<0.001 target.
**Atom or composite:** Composite.
**Cost model:** Cryogenic satellite instrument.
**Real wall?** Yes — dust + synchrotron foregrounds.
**Cross-domain wiring:** Foreground component separation = ml-training ICA / Bayesian.
**Notes:** Hazumi et al.

### baryogenesis (cross-domain alias: `bg-asymmetry`)
**Domain:** Astrophysics / Cosmology
**Definition:** Generation of net baryon asymmetry η_B ~ 6×10⁻¹⁰; requires Sakharov conditions.
**Atom or composite:** Composite (BSM model).
**Cost model:** Theoretical.
**Real wall?** Yes — SM electroweak baryogenesis insufficient.
**Cross-domain wiring:** Out-of-equilibrium dynamics shared with statistics-probability nonequilibrium thermodynamics.
**Notes:** Sakharov (1967); Cohen-Kaplan-Nelson.

### bbn-predictions (cross-domain alias: `bbn`, `primordial-abundances`)
**Domain:** Astrophysics / Cosmology
**Definition:** Big Bang Nucleosynthesis predicts D/H, He, Li abundances from Ω_b h² and N_eff; matches observations except lithium.
**Atom or composite:** Composite.
**Cost model:** Nuclear network ODE.
**Real wall?** Yes — lithium problem unresolved.
**Cross-domain wiring:** Same nuclear-network ODE solver as stellar nucleosynthesis primitives.
**Notes:** Wagoner; Steigman review.

### cosmography (cross-domain alias: `model-independent-cosmo`)
**Domain:** Astrophysics / Cosmology
**Definition:** Taylor-expand a(t) around today: H₀, q₀, j₀, s₀; fits SNe + BAO + cosmic chronometers model-agnostically.
**Atom or composite:** Composite.
**Cost model:** Polynomial regression with priors.
**Real wall?** Yes — Taylor series convergence at high z.
**Cross-domain wiring:** Polynomial expansion shared with signal-processing-rf Taylor approximation.
**Notes:** Visser series.

### h0-tension (cross-domain alias: `hubble-tension`)
**Domain:** Astrophysics / Cosmology
**Definition:** ~5σ discrepancy between local (SH0ES SNe Ia + Cepheids) and CMB (Planck) H₀ measurements.
**Atom or composite:** Composite (observational).
**Cost model:** Inference comparison.
**Real wall?** Yes — new physics or systematics; debate ongoing.
**Cross-domain wiring:** Cross-experiment model comparison shared with statistics-probability tension metrics.
**Notes:** Riess et al.; Planck 2018; Verde-Treu-Riess review.

### s8-tension (cross-domain alias: `σ8-tension`)
**Domain:** Astrophysics / Cosmology
**Definition:** 2–3σ disagreement between weak lensing S₈ ≡ σ₈(Ω_m/0.3)^0.5 and Planck CMB extrapolation.
**Atom or composite:** Composite.
**Cost model:** Inference comparison.
**Real wall?** Yes — likely systematics or new physics.
**Cross-domain wiring:** Same comparison framework as H₀ tension.
**Notes:** Heymans et al. KiDS; DES Y3.

### neff-relativistic-species (cross-domain alias: `n_eff`)
**Domain:** Astrophysics / Cosmology
**Definition:** Effective number of relativistic degrees of freedom; SM predicts 3.046; CMB measures 2.99±0.17 (Planck).
**Atom or composite:** Atom.
**Cost model:** Inference parameter.
**Real wall?** Yes — sensitive to dark radiation.
**Cross-domain wiring:** Counting DoF shared with statistics-probability partition functions.
**Notes:** Mangano et al.

### sigma8-amplitude (cross-domain alias: `σ8`)
**Domain:** Astrophysics / Cosmology
**Definition:** RMS matter fluctuation in 8 h⁻¹ Mpc spheres; benchmark of late-time structure amplitude.
**Atom or composite:** Atom.
**Cost model:** Linear/non-linear computation.
**Real wall?** Yes — handle on late-time vs. early-time growth.
**Cross-domain wiring:** Variance summary statistic shared with signal-processing-rf RMS metrics.
**Notes:** Davis-Peebles; modern Planck/DES.

### nuisance-parameters (cross-domain alias: `marginalized-nuisance`)
**Domain:** Astrophysics / Cosmology
**Definition:** Parameters describing systematics (e.g., bias b, photo-z shifts, magnification) marginalized in cosmology inference.
**Atom or composite:** Composite.
**Cost model:** Adds dimensions to MCMC.
**Real wall?** Yes — strong nuisance priors needed.
**Cross-domain wiring:** Nuisance marginalization shared with statistics-probability hierarchical models.
**Notes:** Standard in DES/KiDS analyses.

### emulator-cosmology (cross-domain alias: `cosmoemulators`)
**Domain:** Astrophysics / Cosmology
**Definition:** Neural-network / Gaussian-process surrogates for Boltzmann code or N-body P(k); μs evals replace minutes.
**Atom or composite:** Composite.
**Cost model:** Training cost amortized.
**Real wall?** Yes — extrapolation outside training box.
**Cross-domain wiring:** Direct ml-training surrogate modeling.
**Notes:** CosmoPower, EuclidEmulator, BACCO.

### differentiable-cosmology (cross-domain alias: `jax-cosmo`)
**Domain:** Astrophysics / Cosmology
**Definition:** Cosmology pipelines implemented in JAX/PyTorch for auto-diff; enables HMC, gradient-based inference, ML coupling.
**Atom or composite:** Composite.
**Cost model:** Up-front rewrite; runtime competitive.
**Real wall?** No — increasingly mainstream.
**Cross-domain wiring:** Auto-diff is ml-training foundation; field-level inference relies on this.
**Notes:** jax-cosmo (Campagne et al.).

### fast-pt (cross-domain alias: `fastpt`)
**Domain:** Astrophysics / Cosmology
**Definition:** FFTLog-based fast evaluation of higher-order perturbation theory integrals (1-loop, RSD, IA).
**Atom or composite:** Composite.
**Cost model:** O(N log N) per kernel.
**Real wall?** No.
**Cross-domain wiring:** FFTLog shared with signal-processing-rf logarithmic-radix transforms.
**Notes:** McEwen et al. (2016).

### eft-of-lss (cross-domain alias: `eft-lss`)
**Domain:** Astrophysics / Cosmology
**Definition:** Effective field theory of LSS: integrate out short modes, expand in derivatives + counterterms; controlled non-linear cosmology.
**Atom or composite:** Composite.
**Cost model:** Several free counterterms.
**Real wall?** Yes — counterterm fitting required.
**Cross-domain wiring:** EFT methodology shared with quantum effective theories.
**Notes:** Baumann et al.; Carrasco-Hertzberg-Senatore.

### cosmic-variance (cross-domain alias: `cv-limit`)
**Domain:** Astrophysics / Cosmology
**Definition:** Irreducible Gaussian-field variance from having one universe; var(C_l) = 2C_l²/(2l+1)f_sky.
**Atom or composite:** Atom.
**Cost model:** Algebraic.
**Real wall?** Yes — fundamental at low ℓ.
**Cross-domain wiring:** Sample variance shared with statistics-probability sampling theory.
**Notes:** Knox (1995).

### sound-horizon (cross-domain alias: `r_s-sound`)
**Domain:** Astrophysics / Cosmology
**Definition:** r_s(z) = ∫_z^∞ c_s/H dz'; comoving distance sound waves traveled by epoch z; r_d at drag epoch is BAO ruler.
**Atom or composite:** Atom.
**Cost model:** 1D integral.
**Real wall?** Yes — sensitive to N_eff, Ω_b.
**Cross-domain wiring:** Sound-wave kinematics shared with signal-processing-rf wave propagation.
**Notes:** Hu-Sugiyama; Eisenstein-Hu fit.

### drag-epoch (cross-domain alias: `z_drag`)
**Domain:** Astrophysics / Cosmology
**Definition:** Redshift z_d ≈ 1060 at which baryons release from photon drag; BAO peak imprinted at r_s(z_d).
**Atom or composite:** Atom.
**Cost model:** Algebraic / Boltzmann code.
**Real wall?** Yes — calibrates BAO ruler.
**Cross-domain wiring:** Drag coefficient analog in physics-diffusion two-fluid drag.
**Notes:** Hu-Sugiyama.

### last-scattering-surface (cross-domain alias: `lss-cmb`)
**Domain:** Astrophysics / Cosmology
**Definition:** Redshift z* ≈ 1090 of CMB photon last Thomson scatter; defines the visible "wall" of CMB.
**Atom or composite:** Atom.
**Cost model:** Algebraic from recombination history.
**Real wall?** Yes — finite thickness ~80 Mpc.
**Cross-domain wiring:** Visibility-function pattern shared with photonics-optics scattering depths.
**Notes:** Peebles (1968); RECFAST.

### recombination (cross-domain alias: `recombination-history`)
**Domain:** Astrophysics / Cosmology
**Definition:** Capture of electrons by protons reducing free-e⁻ density; standard tools RECFAST/CosmoRec/HyRec.
**Atom or composite:** Composite.
**Cost model:** ODE for x_e(z).
**Real wall?** Yes — needs sub-permille accuracy.
**Cross-domain wiring:** Two-photon decay rates shared with photonics-optics atomic physics.
**Notes:** Peebles, Zel'dovich-Kurt-Sunyaev; Chluba-Sunyaev HyRec.

### compton-y-parameter (cross-domain alias: `y-param`)
**Domain:** Astrophysics / Cosmology
**Definition:** y = ∫(k_B T_e/m_e c²) σ_T n_e dl; integrated electron pressure on LOS; SZ amplitude.
**Atom or composite:** Atom.
**Cost model:** LOS integral.
**Real wall?** Yes — sensitive to cluster pressure profile.
**Cross-domain wiring:** Pressure-line-integral shared with hot-cluster X-ray modeling.
**Notes:** Sunyaev-Zel'dovich.

### sz-cluster-survey (cross-domain alias: `sz-cluster-counts`)
**Domain:** Astrophysics / Cosmology
**Definition:** Surveys (Planck/ACT/SPT) detect clusters via SZ decrement; counts dN/dz constrain σ₈, Ω_m.
**Atom or composite:** Composite.
**Cost model:** Matched-filter cluster detection.
**Real wall?** Yes — mass calibration via WL.
**Cross-domain wiring:** Counts cosmology shared with statistics-probability Poisson likelihoods.
**Notes:** Planck SZ catalog; ACT, SPT clusters.

### hubble-deep-field (cross-domain alias: `hdf-hudf`)
**Domain:** Astrophysics / Cosmology
**Definition:** Deep pencil-beam HST imaging revealing very high-z galaxies; foundation of galaxy-evolution studies.
**Atom or composite:** Composite (observational).
**Cost model:** Long integration; ~100 orbits.
**Real wall?** Yes — cosmic variance at small fields.
**Cross-domain wiring:** Deep image co-addition uses signal-processing-rf stacking; photo-z is ml-training regression.
**Notes:** Williams et al. (1996, 2004).

### jwst-deep-survey (cross-domain alias: `jwst-ceers-jades`)
**Domain:** Astrophysics / Cosmology
**Definition:** JWST/NIRCam+NIRSpec deep imaging+spectroscopy revealing z>10 galaxies and reionization-era populations.
**Atom or composite:** Composite.
**Cost model:** Cycle-level observing.
**Real wall?** Yes — confusion, photo-z catastrophic failures.
**Cross-domain wiring:** NIR imaging chain shared with photonics-optics; ML classification central.
**Notes:** Finkelstein, Eisenstein, Curtis-Lake teams.

### cosmic-chronometers (cross-domain alias: `dz-cc`)
**Domain:** Astrophysics / Cosmology
**Definition:** Differential age dt/dz of passively evolving galaxies yields H(z) = −(1+z)⁻¹ dz/dt; model-independent.
**Atom or composite:** Composite.
**Real wall?** Yes — assumes synchronous evolution.
**Cross-domain wiring:** Differential measurement pattern shared with control-numerical-opt finite-difference estimates.
**Notes:** Jimenez-Loeb (2002); Moresco compilations.

### standard-siren (cross-domain alias: `gw-h0`)
**Domain:** Astrophysics / Cosmology
**Definition:** GW source provides absolute distance; combined with EM redshift gives H₀ independent of distance ladder.
**Atom or composite:** Composite.
**Cost model:** Catalog growth O(events).
**Real wall?** Yes — sub-percent H₀ needs ~50–100 events.
**Cross-domain wiring:** Distance-redshift inference shared with statistics-probability hierarchical Bayesian models.
**Notes:** Schutz (1986); GW170817 first measurement.

### bao-reconstruction (cross-domain alias: `bao-recon`)
**Domain:** Astrophysics / Cosmology
**Definition:** Reverse displacement field to undo nonlinear BAO smearing; sharpens BAO peak in 2pt function.
**Atom or composite:** Composite (Zel'dovich + density estimation).
**Cost model:** FFT-based; O(N log N).
**Real wall?** Yes — depends on accurate density field.
**Cross-domain wiring:** Inverse-Lagrangian step shared with signal-processing-rf inverse-filter design.
**Notes:** Eisenstein et al. (2007).

### void-statistics (cross-domain alias: `cosmic-voids`)
**Domain:** Astrophysics / Cosmology
**Definition:** Underdense regions in galaxy distribution; void counts and stacked profiles constrain DE, modified gravity.
**Atom or composite:** Composite.
**Cost model:** Void finder (VIDE, ZOBOV) + statistics.
**Real wall?** Yes — definition-dependent.
**Cross-domain wiring:** Watershed segmentation shared with ml-training image processing.
**Notes:** Sutter; Hamaus reviews.

### lyman-alpha-forest (cross-domain alias: `lyaf`)
**Domain:** Astrophysics / Cosmology
**Definition:** Lyα absorption lines in quasar spectra trace IGM neutral hydrogen at 2<z<4; probes small-scale P(k).
**Atom or composite:** Composite.
**Cost model:** Spectral analysis + 1D P(k).
**Real wall?** Yes — IGM thermodynamics, mean flux.
**Cross-domain wiring:** Spectral feature extraction shared with signal-processing-rf line detection.
**Notes:** Croft et al.; eBOSS Lyα BAO.

### dust-foreground-cmb (cross-domain alias: `dust-bicep`)
**Domain:** Astrophysics / Cosmology
**Definition:** Polarized thermal dust emission dominates CMB B-modes at high frequencies; modeled as modified blackbody.
**Atom or composite:** Composite.
**Cost model:** Multi-frequency component separation.
**Real wall?** Yes — sets primary obstacle to inflation B-modes.
**Cross-domain wiring:** Component separation is signal-processing-rf source separation; spectral fitting is ml-training feature regression.
**Notes:** Planck Intermediate XXX.

### synchrotron-foreground-cmb (cross-domain alias: `sync-fg`)
**Domain:** Astrophysics / Cosmology
**Definition:** Galactic synchrotron emission contributes polarization at low ν; spectral index β_s ≈ −3.
**Atom or composite:** Composite.
**Cost model:** Multi-frequency separation.
**Real wall?** Yes — frequency-dependent foreground.
**Cross-domain wiring:** Same ICA / parametric separation as dust.
**Notes:** Page et al. WMAP; Planck.

### atmospheric-noise-cmb (cross-domain alias: `atm-noise`)
**Domain:** Astrophysics / Cosmology
**Definition:** Water-vapor fluctuations produce correlated 1/f noise in ground-based CMB observations; mitigated by scanning strategy.
**Atom or composite:** Composite.
**Cost model:** Time-domain filtering.
**Real wall?** Yes — fundamentally limits ground-based low-l.
**Cross-domain wiring:** Correlated-noise modeling shared with signal-processing-rf 1/f-noise compensation.
**Notes:** Tegmark mapmaking.

### map-making-cmb (cross-domain alias: `cmb-mapmaker`)
**Domain:** Astrophysics / Cosmology
**Definition:** Convert scanning TOD to sky map via maximum-likelihood / destriping; handles 1/f noise correlations.
**Atom or composite:** Composite.
**Cost model:** Large linear system; PCG.
**Real wall?** Yes — billions of TOD samples.
**Cross-domain wiring:** Same PCG framework as control-numerical-opt large-scale linear systems.
**Notes:** Wright; Tegmark; Sutton.

### healpix (cross-domain alias: `healpix-pixelization`)
**Domain:** Astrophysics / Cosmology
**Definition:** Hierarchical Equal-Area iso-Latitude Pixelization of the sphere; basis for CMB & full-sky analysis.
**Atom or composite:** Composite.
**Cost model:** Iso-area pixels; fast SHT O(N^(3/2)).
**Real wall?** No.
**Cross-domain wiring:** Sphere tessellation shared with photonics-optics spherical-fiber arrangements.
**Notes:** Górski et al. (2005).

### cmb-lensing (cross-domain alias: `phi-reconstruction`)
**Domain:** Astrophysics / Cosmology
**Definition:** Reconstruction of LSS gravitational potential φ from CMB E/B mode mixing; standard cosmology probe.
**Atom or composite:** Composite.
**Cost model:** Quadratic estimator O(l_max⁴) naive; fast variants.
**Real wall?** Yes — N_0 bias subtraction.
**Cross-domain wiring:** Quadratic estimator pattern shared with signal-processing-rf bispectrum techniques.
**Notes:** Hu-Okamoto (2002); Planck lensing.

### delensing-cmb (cross-domain alias: `cmb-delens`)
**Domain:** Astrophysics / Cosmology
**Definition:** Remove lensing-induced B-modes using reconstructed φ + observed E; reduces B variance to expose primordial signal.
**Atom or composite:** Composite.
**Cost model:** Iterative procedure.
**Real wall?** Yes — needed for r<0.001.
**Cross-domain wiring:** Iterative inverse problem shared with control-numerical-opt EM-style algorithms.
**Notes:** Carron-Lewis; Planck PR4.

### desi-y1-bao (cross-domain alias: `desi-y1`)
**Domain:** Astrophysics / Cosmology
**Definition:** DESI Year-1 BAO measurement hinting at evolving dark energy w_a ≠ 0 at ~2.6σ.
**Atom or composite:** Composite.
**Cost model:** Full pipeline.
**Real wall?** Yes — currently 2–3σ; awaiting Y3/Y5.
**Cross-domain wiring:** Tension assessment shared with statistics-probability.
**Notes:** DESI Collaboration (2024).

