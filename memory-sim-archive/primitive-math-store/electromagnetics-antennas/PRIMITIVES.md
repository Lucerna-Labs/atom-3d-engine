# Electromagnetics & Antennas — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## Maxwell's Equations & Potentials

### maxwell-eq-differential (cross-domain alias: `curl-div-system`, `field-pde`, `em-master-eq`)
**Domain:** Electromagnetics / Antennas
**Definition:** ∇·D = ρ, ∇·B = 0, ∇×E = −∂B/∂t, ∇×H = J + ∂D/∂t. Local PDE form of all classical EM.
**Atom or composite:** Atom — irreducible field law.
**Cost model:** O(N) per evaluation on a grid; coupled hyperbolic system.
**Real wall?** Yes — speed-of-light causality, Lorentz invariance.
**Cross-domain wiring:** physics-diffusion (parabolic limit at low freq), signal-processing-rf (wave equation root), quantum-computing (gauge field analog), control-numerical-opt (constrained PDE).
**Notes:** Maxwell 1865; modern form due to Heaviside & Hertz.

### maxwell-eq-integral (cross-domain alias: `gauss-faraday-ampere`, `flux-circulation`, `macro-em`)
**Domain:** Electromagnetics / Antennas
**Definition:** ∮D·dA = Q_enc, ∮B·dA = 0, ∮E·dl = −dΦ_B/dt, ∮H·dl = I_enc + dΦ_D/dt.
**Atom or composite:** Atom — dual to differential form via Stokes/divergence theorems.
**Cost model:** Surface/contour integration; natural for measurement.
**Real wall?** Yes — same physics as differential, integrated.
**Cross-domain wiring:** computational-geometry (surface integration), control-numerical-opt (finite-volume schemes), networking (lumped-circuit limit).
**Notes:** Gauss, Faraday, Ampère pre-Maxwell; unified by Maxwell.

### constitutive-relations (cross-domain alias: `material-tensors`, `D-eps-E`, `B-mu-H`)
**Domain:** Electromagnetics / Antennas
**Definition:** D = εE + P, B = μH − μ₀M, J = σE. Links fields to material response.
**Atom or composite:** Composite — combines field with material tensors (ε, μ, σ).
**Cost model:** Tensor multiplication; can be nonlinear, dispersive, anisotropic.
**Real wall?** No — abstraction over microscopic Lorentz model.
**Cross-domain wiring:** physics-diffusion (transport coefficients), statistics-probability (Onsager reciprocity), linear-algebra-matrix (tensor eigenstructure).
**Notes:** Jackson §6; Landau-Lifshitz ECM.

### em-boundary-conditions (cross-domain alias: `interface-jump`, `tangential-normal-match`, `pec-pmc-bc`)
**Domain:** Electromagnetics / Antennas
**Definition:** n̂×(E₁−E₂) = 0, n̂×(H₁−H₂) = J_s, n̂·(D₁−D₂) = ρ_s, n̂·(B₁−B₂) = 0.
**Atom or composite:** Atom — derived directly from integral Maxwell at pillbox/loop.
**Cost model:** Local interface evaluation; key to FEM/MoM matching.
**Real wall?** Yes — interface continuity is geometric.
**Cross-domain wiring:** formal-verification (interface invariants), control-numerical-opt (jump conditions in DG schemes), graphics-rendering-lod (material boundaries).
**Notes:** Balanis Ch. 1; standard PEC/PMC idealizations.

### displacement-current (cross-domain alias: `dD-dt`, `vacuum-current`, `maxwell-correction`)
**Domain:** Electromagnetics / Antennas
**Definition:** J_d = ∂D/∂t. Term added by Maxwell to make Ampère's law consistent with charge conservation.
**Atom or composite:** Atom — the term that makes light possible.
**Cost model:** Differential operation; essential for radiation.
**Real wall?** Yes — without it, no EM waves.
**Cross-domain wiring:** physics-diffusion (analog: relaxation current), signal-processing-rf (radiating term), networking (capacitive coupling).
**Notes:** Maxwell 1861; closed the system, predicted c = 1/√(μ₀ε₀).

### coulomb-gauge (cross-domain alias: `transverse-gauge`, `divA-zero`, `radiation-gauge`)
**Domain:** Electromagnetics / Antennas
**Definition:** ∇·A = 0. Decouples scalar potential (instantaneous Coulomb) from vector potential (radiative).
**Atom or composite:** Composite — gauge choice on (φ, A).
**Cost model:** Imposes Poisson solve for φ; convenient for quasi-static.
**Real wall?** No — gauge freedom is unphysical.
**Cross-domain wiring:** quantum-computing (QED gauge), control-numerical-opt (constraint preservation), formal-verification (gauge-invariant observables).
**Notes:** Jackson §6.3.

### lorenz-gauge (cross-domain alias: `relativistic-gauge`, `divA-plus-dtphi`, `wave-eq-gauge`)
**Domain:** Electromagnetics / Antennas
**Definition:** ∇·A + (1/c²)∂φ/∂t = 0. Makes (φ, A) each satisfy decoupled wave equations.
**Atom or composite:** Atom — Lorentz-covariant choice.
**Cost model:** Symmetric wave-eq solves for φ and A separately.
**Real wall?** No — gauge choice.
**Cross-domain wiring:** quantum-computing (manifestly covariant QED), signal-processing-rf (separate scalar/vector wave channels).
**Notes:** Ludvig Lorenz 1867 (not H. A. Lorentz); see Jackson §6.3.

### temporal-gauge (cross-domain alias: `weyl-gauge`, `phi-zero`, `hamiltonian-gauge`)
**Domain:** Electromagnetics / Antennas
**Definition:** φ = 0. Sets scalar potential to zero; E = −∂A/∂t alone.
**Atom or composite:** Atom — algebraic gauge fix.
**Cost model:** Convenient in Hamiltonian/canonical formulation.
**Real wall?** No — gauge choice.
**Cross-domain wiring:** quantum-computing (canonical EM quantization), control-numerical-opt (symplectic integrators).
**Notes:** Used in lattice gauge theory and PIC simulations.

### retarded-potentials (cross-domain alias: `causal-greens-function`, `t-minus-r-over-c`, `delayed-source`)
**Domain:** Electromagnetics / Antennas
**Definition:** φ(r,t) = (1/4πε₀)∫ρ(r',t−|r−r'|/c)/|r−r'| d³r'; analogous for A.
**Atom or composite:** Composite — Green's function convolution with source.
**Cost model:** O(N_obs · N_src) brute force; MLFMM accelerates.
**Real wall?** Yes — causality: future cannot affect past.
**Cross-domain wiring:** signal-processing-rf (causal convolution), retrieval-search (kernel methods), physics-diffusion (heat-eq Green's function analog).
**Notes:** Foundation of antenna theory; Jackson §6.5.

### advanced-potentials (cross-domain alias: `anti-causal-greens`, `t-plus-r-over-c`, `time-reversed`)
**Domain:** Electromagnetics / Antennas
**Definition:** Same as retarded but with t+|r−r'|/c; corresponds to incoming waves.
**Atom or composite:** Atom — time-reversed Green's function.
**Cost model:** Same as retarded but acausal in observer frame.
**Real wall?** No — mathematically valid; unphysical alone but used in absorber theory.
**Cross-domain wiring:** signal-processing-rf (matched-filter time reversal), quantum-computing (Wheeler-Feynman absorber theory).
**Notes:** Wheeler-Feynman 1945; used in time-reversal mirrors.

### lienard-wiechert-potentials (cross-domain alias: `point-charge-radiation`, `moving-source-fields`, `LW-potentials`)
**Domain:** Electromagnetics / Antennas
**Definition:** φ = q/(4πε₀)·1/(R−v·R/c)|_ret; A = (v/c²)φ. Exact for arbitrary point-charge trajectory.
**Atom or composite:** Composite — retarded potential specialized to delta-source.
**Cost model:** Closed-form once retarded time is solved (implicit eq).
**Real wall?** Yes — relativistic causality.
**Cross-domain wiring:** physics-diffusion (moving heat source analog), signal-processing-rf (Doppler+aberration), quantum-computing (synchrotron radiation).
**Notes:** Liénard 1898, Wiechert 1900; Jackson §14.

### hertz-potentials (cross-domain alias: `single-vector-potential`, `Pi-electric-magnetic`, `polarization-potential`)
**Domain:** Electromagnetics / Antennas
**Definition:** Π_e, Π_m vectors with E and H derived as second derivatives; reduces to one vector per polarization.
**Atom or composite:** Composite — superpotential of (φ, A).
**Cost model:** Often simplifies waveguide/cavity problems to scalar Helmholtz.
**Real wall?** No — mathematical convenience.
**Cross-domain wiring:** linear-algebra-matrix (operator factorization), formal-verification (reduced state-space).
**Notes:** Hertz 1888–1889; Stratton Ch. 1.

### debye-potentials (cross-domain alias: `scalar-spherical-potential`, `mie-scalar`, `transverse-decomp`)
**Domain:** Electromagnetics / Antennas
**Definition:** Two scalar potentials in spherical geometry; generate TE and TM parts of Maxwell.
**Atom or composite:** Composite — spherical specialization of Hertz potentials.
**Cost model:** Scalar Helmholtz in r, θ, φ; series in spherical harmonics.
**Real wall?** No — chosen for symmetry.
**Cross-domain wiring:** quantum-computing (angular-momentum eigenstates), graphics-rendering-lod (spherical harmonic lighting).
**Notes:** Debye 1909; foundational for Mie scattering.

### dyadic-greens-function (cross-domain alias: `tensor-kernel`, `Ḡ-of-r-r-prime`, `em-impulse-response`)
**Domain:** Electromagnetics / Antennas
**Definition:** Ḡ(r,r') such that E(r) = jωμ ∫Ḡ(r,r')·J(r') dV'. Tensor impulse response of Maxwell.
**Atom or composite:** Composite — bilinear kernel with polarization tensor structure.
**Cost model:** Singular at r=r'; principal-value/excluded-region treatment needed.
**Real wall?** No — operator inverse.
**Cross-domain wiring:** linear-algebra-matrix (operator inverse), signal-processing-rf (MIMO channel kernel), retrieval-search (Mercer kernel).
**Notes:** Tai 1971 "Dyadic Green's Functions"; foundation of MoM.

### poynting-theorem (cross-domain alias: `em-power-flow`, `ExH-flux`, `energy-conservation`)
**Domain:** Electromagnetics / Antennas
**Definition:** S = E×H, ∂u/∂t + ∇·S = −J·E. Local statement of EM energy conservation.
**Atom or composite:** Atom — conservation law derived from Maxwell.
**Cost model:** Pointwise vector product; surface integration for power.
**Real wall?** Yes — energy conservation.
**Cross-domain wiring:** statistics-probability (information flow analog), networking (link-budget power), control-numerical-opt (passivity proof).
**Notes:** Poynting 1884.

### em-stress-energy-tensor (cross-domain alias: `maxwell-stress`, `Tij-em`, `radiation-pressure`)
**Domain:** Electromagnetics / Antennas
**Definition:** T_ij = ε₀(E_iE_j − ½δ_ij|E|²) + (1/μ₀)(B_iB_j − ½δ_ij|B|²).
**Atom or composite:** Composite — momentum/stress per Maxwell field.
**Cost model:** Surface integration gives force on enclosed bodies.
**Real wall?** Yes — momentum conservation.
**Cross-domain wiring:** physics-diffusion (stress tensor analog), control-numerical-opt (force computation in optimization).
**Notes:** Maxwell 1873; basis of optical tweezers.

### duality-principle (cross-domain alias: `e-h-swap`, `magnetic-current`, `em-duality`)
**Domain:** Electromagnetics / Antennas
**Definition:** Maxwell invariant under E→H, H→−E, ε↔μ, J↔M (magnetic current).
**Atom or composite:** Atom — symmetry of equations.
**Cost model:** Free — gives PMC/slot-antenna analogs of PEC/wire results.
**Real wall?** No — formal symmetry; magnetic monopoles unobserved.
**Cross-domain wiring:** quantum-computing (electromagnetic duality, S-duality), type-theory-programming-languages (categorical duality).
**Notes:** Babinet's principle uses this; Balanis §7.

### reciprocity-theorem (cross-domain alias: `lorentz-reciprocity`, `tx-rx-swap`, `em-symmetry`)
**Domain:** Electromagnetics / Antennas
**Definition:** ∫(E_a·J_b − E_b·J_a)dV = 0 in reciprocal media. Tx/Rx pattern identity.
**Atom or composite:** Atom — symmetry of dyadic Green's function.
**Cost model:** Free invariant; halves measurement effort.
**Real wall?** Yes in reciprocal media; broken by ferrites, plasmas with B.
**Cross-domain wiring:** signal-processing-rf (TDD channel reciprocity), networking (massive MIMO calibration), formal-verification (symmetry invariants).
**Notes:** Lorentz 1896; foundational for antenna measurement.

### equivalence-principle-em (cross-domain alias: `huygens-equivalence`, `surface-currents`, `love-equivalence`)
**Domain:** Electromagnetics / Antennas
**Definition:** Fields outside a closed surface replicated by equivalent J_s = n̂×H, M_s = −n̂×E on the surface.
**Atom or composite:** Composite — surface-source representation.
**Cost model:** Reduces 3D problem to 2D surface integral.
**Real wall?** No — mathematical equivalence.
**Cross-domain wiring:** computational-geometry (surface representations), retrieval-search (boundary embeddings).
**Notes:** Love 1901; foundation of MoM-SIE methods.

### babinet-principle (cross-domain alias: `complementary-screen`, `slot-wire-duality`, `aperture-complement`)
**Domain:** Electromagnetics / Antennas
**Definition:** Z_screen · Z_complement = η²/4 for complementary planar structures.
**Atom or composite:** Composite — duality applied to planar apertures.
**Cost model:** Free — pairs known wire solutions with slot solutions.
**Real wall?** No — exact for infinite planar screens only.
**Cross-domain wiring:** computational-geometry (complement operation), graphics-rendering-lod (silhouette duality).
**Notes:** Babinet 1837 (optical); Booker 1946 (EM extension).

---

## EM Waves & Propagation

### plane-wave-uniform (cross-domain alias: `monochromatic-wave`, `exp-jkz`, `propagating-mode`)
**Domain:** Electromagnetics / Antennas
**Definition:** E(r,t) = E₀ exp(j(ωt − k·r)), with k·E₀ = 0 in free space.
**Atom or composite:** Atom — eigenmode of free-space Maxwell.
**Cost model:** Single complex amplitude per (k, polarization).
**Real wall?** No — idealization; finite beams composed of these.
**Cross-domain wiring:** signal-processing-rf (Fourier basis), linear-algebra-matrix (eigenfunctions), quantum-computing (photon mode).
**Notes:** Stratton Ch. 5.

### te-tm-tem-modes (cross-domain alias: `polarization-decomp`, `e-perp-h-perp`, `transverse-modes`)
**Domain:** Electromagnetics / Antennas
**Definition:** TE: E_z=0; TM: H_z=0; TEM: both transverse. Decomposes guided waves.
**Atom or composite:** Composite — directional decomposition w.r.t. propagation axis.
**Cost model:** Reduces vector problem to two scalar Helmholtz problems.
**Real wall?** No — basis choice.
**Cross-domain wiring:** linear-algebra-matrix (orthogonal decomposition), photonics-optics (waveguide modes).
**Notes:** Pozar §3; Collin "Field Theory of Guided Waves".

### dispersion-relation-medium (cross-domain alias: `k-omega-curve`, `n-of-omega`, `material-dispersion`)
**Domain:** Electromagnetics / Antennas
**Definition:** k² = ω²με(ω) with complex ε(ω) in lossy/dispersive media.
**Atom or composite:** Atom — links spatial and temporal frequencies in a medium.
**Cost model:** Pointwise lookup once ε(ω) known.
**Real wall?** Yes — causality enforces Kramers-Kronig on ε(ω).
**Cross-domain wiring:** signal-processing-rf (phase distortion), control-numerical-opt (PDE stability), photonics-optics (group-velocity dispersion).
**Notes:** Jackson §7.5.

### skin-depth (cross-domain alias: `delta-conductor`, `1-over-alpha`, `field-penetration`)
**Domain:** Electromagnetics / Antennas
**Definition:** δ = √(2/(ωμσ)) — 1/e field penetration into a good conductor.
**Atom or composite:** Atom — conductor wave property.
**Cost model:** Sets surface-impedance approximation; mesh refinement needed in FDTD.
**Real wall?** Yes — physical attenuation.
**Cross-domain wiring:** physics-diffusion (parabolic limit of Maxwell in conductor), networking (cable losses), control-numerical-opt (boundary layers).
**Notes:** 66 μm in Cu at 1 MHz; 2 μm at 1 GHz.

### wave-impedance (cross-domain alias: `eta-medium`, `E-over-H`, `intrinsic-impedance`)
**Domain:** Electromagnetics / Antennas
**Definition:** η = √(μ/ε); η₀ = 376.73 Ω in free space.
**Atom or composite:** Atom — ratio of transverse fields in plane wave.
**Cost model:** Trivial scalar; defines matching to transmission lines.
**Real wall?** Yes — fundamental constant of medium.
**Cross-domain wiring:** networking (Z₀ analog), signal-processing-rf (impedance matching), control-numerical-opt (PML matching).
**Notes:** Jackson §7.

### group-velocity (cross-domain alias: `vg-domega-dk`, `energy-transport-vel`, `packet-velocity`)
**Domain:** Electromagnetics / Antennas
**Definition:** v_g = dω/dk; speed of envelope of narrowband wave packet.
**Atom or composite:** Atom — derivative of dispersion relation.
**Cost model:** Numerical differentiation of ω(k); cheap.
**Real wall?** Yes — bounded by c for energy/signal in vacuum; can exceed c in dispersive media for phase, never information.
**Cross-domain wiring:** signal-processing-rf (envelope detection), photonics-optics (chromatic dispersion).
**Notes:** Rayleigh 1881; Brillouin's "Wave Propagation and Group Velocity".

### phase-velocity (cross-domain alias: `vp-omega-over-k`, `wavefront-vel`, `crest-speed`)
**Domain:** Electromagnetics / Antennas
**Definition:** v_p = ω/k. Can exceed c without violating causality (no information).
**Atom or composite:** Atom — ratio in dispersion.
**Cost model:** Trivial.
**Real wall?** No — no information transmitted at v_p > c.
**Cross-domain wiring:** signal-processing-rf (carrier frequency), photonics-optics (refractive index n = c/v_p).
**Notes:** Sommerfeld-Brillouin debate (1907–1914) resolved this.

### sommerfeld-precursor (cross-domain alias: `front-precursor`, `early-arrival`, `causality-test`)
**Domain:** Electromagnetics / Antennas
**Definition:** Initial high-frequency, low-amplitude signal arriving at exactly t = z/c through a dispersive medium.
**Atom or composite:** Composite — saddle-point analysis of Fourier integral.
**Cost model:** Asymptotic expansion of integral; numerical via FFT.
**Real wall?** Yes — proof that signal front never exceeds c.
**Cross-domain wiring:** signal-processing-rf (pre-cursor in equalizer), formal-verification (causality proofs).
**Notes:** Sommerfeld 1914; Oughstun & Sherman monograph.

### brillouin-precursor (cross-domain alias: `late-precursor`, `low-freq-precursor`, `second-precursor`)
**Domain:** Electromagnetics / Antennas
**Definition:** Slowly-oscillating low-frequency signal following the Sommerfeld precursor; persists at depth.
**Atom or composite:** Composite — second saddle-point contribution.
**Cost model:** Asymptotic; experimental detection requires sub-ps gating.
**Real wall?** Yes — predicted by causality + Drude/Lorentz medium.
**Cross-domain wiring:** signal-processing-rf (deep-penetration imaging), biology-bioinformatics (ground-penetrating EM).
**Notes:** Brillouin 1914; first measured 1969.

### evanescent-wave (cross-domain alias: `decaying-mode`, `imaginary-k`, `near-field-tail`)
**Domain:** Electromagnetics / Antennas
**Definition:** Wave with imaginary k_z component: E ∝ exp(−κz)exp(jk_x x). Decays exponentially with no power flow.
**Atom or composite:** Atom — non-propagating Maxwell solution.
**Cost model:** Same Helmholtz solver; sets near-field range scale.
**Real wall?** Yes — solutions exist below cutoff; carry reactive power only.
**Cross-domain wiring:** photonics-optics (frustrated TIR), retrieval-search (super-resolution imaging), quantum-computing (tunneling analog).
**Notes:** Newton's "fits"; key to near-field microscopy.

### surface-wave (cross-domain alias: `zenneck-wave`, `bound-mode`, `interface-guided`)
**Domain:** Electromagnetics / Antennas
**Definition:** Wave bound to a planar interface, decaying exponentially in both transverse directions.
**Atom or composite:** Composite — TM mode at lossy-conductor or impedance interface.
**Cost model:** Solve complex dispersion det = 0; root tracking.
**Real wall?** Yes — exists when surface impedance has appropriate sign.
**Cross-domain wiring:** photonics-optics (surface plasmon polariton), networking (long-wave ground propagation).
**Notes:** Zenneck 1907; Sommerfeld 1909.

### goos-hanchen-shift (cross-domain alias: `lateral-tir-shift`, `beam-displacement`, `phase-shift-shift`)
**Domain:** Electromagnetics / Antennas
**Definition:** Lateral displacement of finite beam undergoing TIR; D = −∂φ/∂k_x.
**Atom or composite:** Composite — k-space derivative of reflection phase.
**Cost model:** Small (~λ); requires beam-shape analysis.
**Real wall?** Yes — predicted by stationary-phase analysis.
**Cross-domain wiring:** signal-processing-rf (group-delay analog), photonics-optics (sensor mechanism).
**Notes:** Goos & Hänchen 1947.

### polarization-states (cross-domain alias: `linear-circular-elliptical`, `jones-vector`, `stokes-params`)
**Domain:** Electromagnetics / Antennas
**Definition:** Direction of E vector vs time: linear, circular (LHCP/RHCP), elliptical; encoded by Jones vector or Stokes parameters.
**Atom or composite:** Composite — 2-vector or 4-Stokes representation.
**Cost model:** 2×2 Jones matrix multiplication; 4×4 Mueller for incoherent.
**Real wall?** No — basis description.
**Cross-domain wiring:** quantum-computing (qubit Bloch sphere analog), signal-processing-rf (dual-pol MIMO), photonics-optics (Stokes vectors).
**Notes:** Stokes 1852; Jones 1941.

### kramers-kronig-relations (cross-domain alias: `hilbert-transform-eps`, `causality-eps-omega`, `dispersion-relations`)
**Domain:** Electromagnetics / Antennas
**Definition:** Re ε(ω) and Im ε(ω) are Hilbert transforms of each other; consequence of causality.
**Atom or composite:** Atom — causality constraint on response function.
**Cost model:** Hilbert transform; FFT-accelerated.
**Real wall?** Yes — strict consequence of causality + linearity.
**Cross-domain wiring:** signal-processing-rf (Hilbert transform), statistics-probability (analytic signal), formal-verification (causality invariants).
**Notes:** Kramers 1927, Kronig 1926.

### drude-lorentz-model (cross-domain alias: `oscillator-model-eps`, `plasma-eps`, `dispersive-medium`)
**Domain:** Electromagnetics / Antennas
**Definition:** ε(ω) = ε_∞ + Σ ω_p²/(ω₀² − ω² − jγω). Lorentz oscillators; Drude when ω₀ → 0.
**Atom or composite:** Composite — sum of damped-oscillator responses.
**Cost model:** Few-parameter fit per material; ADE-FDTD implementations.
**Real wall?** Yes — Kramers-Kronig consistent if Im part positive.
**Cross-domain wiring:** statistics-probability (linear response), control-numerical-opt (state-space realization), photonics-optics (metal optics).
**Notes:** Drude 1900, Lorentz 1880.

### debye-relaxation (cross-domain alias: `single-pole-eps`, `polar-dielectric`, `water-relaxation`)
**Domain:** Electromagnetics / Antennas
**Definition:** ε(ω) = ε_∞ + (ε_s − ε_∞)/(1 + jωτ). One-pole relaxation; water peaks ~20 GHz.
**Atom or composite:** Composite — single-pole low-pass response.
**Cost model:** One ADE per pole in FDTD.
**Real wall?** Yes — molecular reorientation physics.
**Cross-domain wiring:** statistics-probability (autocorrelation), biology-bioinformatics (tissue dielectric).
**Notes:** Debye 1929 "Polar Molecules".

### atmospheric-attenuation (cross-domain alias: `itu-r-p676`, `gaseous-absorption`, `oxygen-water-bands`)
**Domain:** Electromagnetics / Antennas
**Definition:** dB/km from O₂ (60 GHz peak) and H₂O (22, 183 GHz peaks).
**Atom or composite:** Composite — sum of molecular line absorptions.
**Cost model:** Tabulated coefficients; HITRAN database for fine lines.
**Real wall?** Yes — physical absorption.
**Cross-domain wiring:** networking (millimeter-wave 5G link budget), signal-processing-rf (channel modeling).
**Notes:** ITU-R P.676; Liebe MPM model.

### rain-attenuation-itu (cross-domain alias: `itu-r-p838`, `specific-attenuation`, `k-alpha-coefficients`)
**Domain:** Electromagnetics / Antennas
**Definition:** γ_R = k · R^α dB/km, where R is rain rate (mm/h), k and α frequency/polarization-dependent.
**Atom or composite:** Composite — empirical fit to Mie scattering by droplet distributions.
**Cost model:** Lookup; combine with rain-cell statistics for outage.
**Real wall?** Yes — Mie scattering off finite droplets.
**Cross-domain wiring:** statistics-probability (rain-rate CDFs), networking (link-availability budgeting).
**Notes:** ITU-R P.838; foundational for Ka/Ku satellite links.

### faraday-rotation (cross-domain alias: `magneto-optic-rotation`, `gyrotropic-rotation`, `ionospheric-rotation`)
**Domain:** Electromagnetics / Antennas
**Definition:** Rotation of linear polarization in a magnetized plasma/material; θ = V·B·L (Verdet constant V).
**Atom or composite:** Composite — anisotropic ε tensor breaks degeneracy of LHCP/RHCP.
**Cost model:** Closed-form for uniform medium; transfer matrix for layers.
**Real wall?** Yes — broken time-reversal symmetry from B field.
**Cross-domain wiring:** photonics-optics (optical isolators), quantum-computing (Kerr effect), networking (transionospheric links).
**Notes:** Faraday 1845.

---

## Reflection, Transmission & Thin Films

### fresnel-coefficients (cross-domain alias: `r-s-r-p`, `parallel-perp-reflection`, `oblique-incidence`)
**Domain:** Electromagnetics / Antennas
**Definition:** r_s = (η₂cosθ_i − η₁cosθ_t)/(η₂cosθ_i + η₁cosθ_t); r_p dual; t = 1+r relations.
**Atom or composite:** Atom — closed-form interface scattering.
**Cost model:** Pointwise per (θ, polarization, frequency).
**Real wall?** No — assumes planar infinite interface.
**Cross-domain wiring:** graphics-rendering-lod (BRDF base), photonics-optics (AR coatings), networking (ground-reflection model).
**Notes:** Fresnel 1823.

### brewster-angle (cross-domain alias: `polarizing-angle`, `zero-rp`, `tan-inverse-n2-n1`)
**Domain:** Electromagnetics / Antennas
**Definition:** θ_B = arctan(n₂/n₁). At this angle r_p = 0; reflected light fully s-polarized.
**Atom or composite:** Atom — Fresnel zero condition.
**Cost model:** Trivial; design parameter for polarizers.
**Real wall?** Yes — exact for lossless dielectrics.
**Cross-domain wiring:** photonics-optics (laser windows), signal-processing-rf (polarization filters).
**Notes:** Brewster 1815.

### critical-angle-tir (cross-domain alias: `total-internal-reflection`, `theta-c`, `tir-onset`)
**Domain:** Electromagnetics / Antennas
**Definition:** θ_c = arcsin(n₂/n₁), n₁ > n₂. Above θ_c, |r|=1, evanescent wave on transmission side.
**Atom or composite:** Atom — Snell's law with imaginary θ_t.
**Cost model:** Trivial.
**Real wall?** Yes — geometric.
**Cross-domain wiring:** photonics-optics (fiber-optic confinement), networking (optical fibers).
**Notes:** Foundation of dielectric-waveguide confinement.

### transfer-matrix-method (cross-domain alias: `tmm-thin-film`, `abcd-stack`, `multilayer-stack`)
**Domain:** Electromagnetics / Antennas
**Definition:** Stack of layers → product of 2×2 matrices; total r, t from product.
**Atom or composite:** Composite — multiplicative ABCD across interfaces.
**Cost model:** O(N) per N layers; trivially parallelizable in frequency.
**Real wall?** No — requires planar, infinite layers.
**Cross-domain wiring:** linear-algebra-matrix (matrix product), photonics-optics (Bragg mirrors), quantum-computing (1D transfer matrices).
**Notes:** Abelès 1950; Born & Wolf §1.6.

### quarter-wave-ar-coating (cross-domain alias: `lambda-over-4-ar`, `single-layer-ar`, `nL-equals-sqrt`)
**Domain:** Electromagnetics / Antennas
**Definition:** n_L = √(n_s), thickness = λ/(4n_L). Zero reflection at center wavelength.
**Atom or composite:** Composite — TMM result for one layer.
**Cost model:** Single-layer; narrowband.
**Real wall?** No — quarterwave assumption.
**Cross-domain wiring:** photonics-optics (lens AR), signal-processing-rf (impedance transformer).
**Notes:** Lord Rayleigh 1886.

### bragg-reflector (cross-domain alias: `dbr`, `quarter-wave-stack`, `photonic-bandgap-1d`)
**Domain:** Electromagnetics / Antennas
**Definition:** Periodic stack of λ/4 layers (n_H, n_L). Reflection band centered at λ₀; bandwidth ∝ |n_H − n_L|/n_avg.
**Atom or composite:** Composite — N-period TMM.
**Cost model:** O(N) layers; eigenmode for infinite period.
**Real wall?** Yes — periodic structures forbid propagation in stopband.
**Cross-domain wiring:** photonics-optics (VCSELs, fiber Bragg gratings), quantum-computing (photonic crystal cavities).
**Notes:** Bragg 1913 (X-ray); 1D photonic crystal.

### chirped-bragg-grating (cross-domain alias: `linear-chirp-dbr`, `dispersion-compensator`, `broadband-mirror`)
**Domain:** Electromagnetics / Antennas
**Definition:** Bragg stack with spatially-varying period; reflects band of wavelengths.
**Atom or composite:** Composite — TMM with non-uniform layers.
**Cost model:** Same as TMM; optimization over chirp profile.
**Real wall?** No — design freedom.
**Cross-domain wiring:** photonics-optics (CPA pulse stretching), signal-processing-rf (analog dispersion).
**Notes:** Used in fiber-laser pulse compression.

### snell-law (cross-domain alias: `n1-sin-theta1`, `refraction-law`, `phase-matching`)
**Domain:** Electromagnetics / Antennas
**Definition:** n₁ sinθ₁ = n₂ sinθ₂. Consequence of tangential-k continuity at interface.
**Atom or composite:** Atom — momentum matching.
**Cost model:** Trivial.
**Real wall?** Yes — phase-matching geometric law.
**Cross-domain wiring:** photonics-optics (lens design), quantum-computing (phase matching in NL optics).
**Notes:** Ibn Sahl 984 CE; Snell 1621.

### geometrical-optics-limit (cross-domain alias: `ray-optics`, `eikonal-limit`, `lambda-to-zero`)
**Domain:** Electromagnetics / Antennas
**Definition:** λ → 0 limit of Maxwell; rays follow eikonal equation |∇S|² = n².
**Atom or composite:** Composite — WKB-like approximation.
**Cost model:** Ray-trace O(N_rays · N_bounces); much cheaper than full-wave.
**Real wall?** No — breaks down at edges/caustics.
**Cross-domain wiring:** graphics-rendering-lod (ray tracing), computational-geometry (ray intersections).
**Notes:** Hamilton 1830s; basis of SBR methods.

### eikonal-equation (cross-domain alias: `iconal`, `wavefront-equation`, `S-of-r`)
**Domain:** Electromagnetics / Antennas
**Definition:** |∇S(r)|² = n²(r). Hamilton-Jacobi form of high-frequency Maxwell.
**Atom or composite:** Atom — first-order nonlinear PDE.
**Cost model:** Fast-marching/sweeping O(N log N).
**Real wall?** No — high-frequency approximation.
**Cross-domain wiring:** control-numerical-opt (HJB equation), computational-geometry (distance fields), graphics-rendering-lod (signed distance functions).
**Notes:** Bruns 1895; Sommerfeld & Runge 1911.

---

## Waveguides & Transmission Lines

### rectangular-waveguide-modes (cross-domain alias: `tem-rect-wg`, `te-mn-tm-mn`, `wr-band`)
**Domain:** Electromagnetics / Antennas
**Definition:** TE_mn, TM_mn modes with cutoff f_c = (c/2)√((m/a)² + (n/b)²); TE₁₀ dominant.
**Atom or composite:** Composite — separable Helmholtz with PEC walls.
**Cost model:** Closed-form eigenmodes; sums of modes for arbitrary excitation.
**Real wall?** Yes — cutoff is physical.
**Cross-domain wiring:** linear-algebra-matrix (eigendecomposition), networking (microwave plumbing).
**Notes:** Pozar Ch. 3; WR-90 = X-band.

### circular-waveguide-modes (cross-domain alias: `te-tm-circular`, `bessel-modes`, `circ-wg`)
**Domain:** Electromagnetics / Antennas
**Definition:** TE_nm, TM_nm with cutoff from Bessel zeros; TE₁₁ dominant.
**Atom or composite:** Composite — separable Helmholtz in (ρ, φ).
**Cost model:** Bessel-function lookups; root tracking.
**Real wall?** Yes — physical cutoffs.
**Cross-domain wiring:** photonics-optics (optical fibers analog), quantum-computing (cylindrical cavities).
**Notes:** Collin Ch. 5.

### coaxial-line (cross-domain alias: `coax-cable`, `tem-coax`, `Z0-138-log-b-over-a`)
**Domain:** Electromagnetics / Antennas
**Definition:** Two concentric conductors; supports TEM; Z₀ = (η/2π) ln(b/a)/√ε_r.
**Atom or composite:** Composite — TEM mode in coaxial geometry.
**Cost model:** Closed-form; modeled as transmission line.
**Real wall?** Yes — no cutoff for TEM; higher modes above f_c.
**Cross-domain wiring:** networking (CATV, Ethernet over coax), signal-processing-rf (RG-58, etc.).
**Notes:** Heaviside 1880s.

### microstrip-line (cross-domain alias: `quasi-tem-strip`, `pcb-trace`, `eps-eff`)
**Domain:** Electromagnetics / Antennas
**Definition:** Conductor strip over ground plane on dielectric substrate; quasi-TEM mode.
**Atom or composite:** Composite — open dielectric structure; ε_eff between air and substrate.
**Cost model:** Closed-form approximations (Hammerstad-Jensen); full-wave for accuracy.
**Real wall?** No — quasi-static approximation degrades at high f.
**Cross-domain wiring:** networking (PCB), signal-processing-rf (RF front-ends).
**Notes:** Wheeler 1965; Pozar §3.8.

### stripline (cross-domain alias: `triplate`, `buried-strip`, `pure-tem-pcb`)
**Domain:** Electromagnetics / Antennas
**Definition:** Conductor between two ground planes embedded in homogeneous dielectric; pure TEM.
**Atom or composite:** Composite — closed homogeneous TEM line.
**Cost model:** Closed-form Z₀.
**Real wall?** Yes — TEM cutoff-free.
**Cross-domain wiring:** networking (high-speed digital), signal-processing-rf (low-radiation traces).
**Notes:** Barrett 1955.

### coplanar-waveguide (cross-domain alias: `cpw`, `signal-with-ground-cuts`, `uniplanar`)
**Domain:** Electromagnetics / Antennas
**Definition:** Center conductor with two side ground planes on same surface; quasi-TEM.
**Atom or composite:** Composite — three-conductor open structure.
**Cost model:** Conformal-mapping closed-form; full-wave for losses.
**Real wall?** No — depends on substrate thickness and gap.
**Cross-domain wiring:** signal-processing-rf (MMIC interconnect), quantum-computing (superconducting qubit lines).
**Notes:** Wen 1969; common in GaAs MMICs.

### substrate-integrated-waveguide (cross-domain alias: `siw`, `via-fence-wg`, `pcb-waveguide`)
**Domain:** Electromagnetics / Antennas
**Definition:** Rectangular waveguide realized in PCB with via fences as side walls.
**Atom or composite:** Composite — discrete-via approximation of continuous wall.
**Cost model:** Same as rect WG; via pitch < λ_g/5.
**Real wall?** Yes — leakage if via spacing too large.
**Cross-domain wiring:** networking (mmWave 5G modules), signal-processing-rf (low-cost waveguides).
**Notes:** Wu 1998–2003.

### dielectric-waveguide (cross-domain alias: `optical-fiber-analog`, `slab-wg`, `tir-confined`)
**Domain:** Electromagnetics / Antennas
**Definition:** Dielectric core surrounded by lower-index cladding; confines by TIR.
**Atom or composite:** Composite — eigenmode of inhomogeneous Helmholtz.
**Cost model:** Transcendental dispersion; mode-solver O(N log N).
**Real wall?** Yes — TIR sets confinement.
**Cross-domain wiring:** photonics-optics (fibers), quantum-computing (photonic circuits).
**Notes:** Snyder & Love.

### ridge-waveguide (cross-domain alias: `single-double-ridge-wg`, `lowered-cutoff`, `broadband-wg`)
**Domain:** Electromagnetics / Antennas
**Definition:** Rectangular WG with central conducting ridge; lowers TE₁₀ cutoff, broadens single-mode band.
**Atom or composite:** Composite — modified rect WG.
**Cost model:** Numerical eigensolve; tabulated.
**Real wall?** No — geometric design freedom.
**Cross-domain wiring:** signal-processing-rf (broadband filters).
**Notes:** Cohn 1947.

### telegrapher-equations (cross-domain alias: `tline-pde`, `RLGC-line`, `lumped-line`)
**Domain:** Electromagnetics / Antennas
**Definition:** ∂V/∂z = −(R + jωL)I, ∂I/∂z = −(G + jωC)V. 1D wave equation for line voltage/current.
**Atom or composite:** Atom — 1D limit of Maxwell on TEM line.
**Cost model:** Closed-form for uniform line; numerical for non-uniform.
**Real wall?** Yes — derived from Maxwell on TEM modes.
**Cross-domain wiring:** networking (signal integrity), control-numerical-opt (hyperbolic PDE).
**Notes:** Heaviside 1880s; named after Lord Kelvin's telegraph work.

### characteristic-impedance (cross-domain alias: `z0`, `sqrt-L-over-C`, `wave-impedance-line`)
**Domain:** Electromagnetics / Antennas
**Definition:** Z₀ = √((R + jωL)/(G + jωC)) → √(L/C) lossless.
**Atom or composite:** Atom — line property.
**Cost model:** Trivial once L, C known.
**Real wall?** Yes — physical line parameter.
**Cross-domain wiring:** signal-processing-rf (matching), networking (50/75 Ω standards).
**Notes:** Heaviside.

### reflection-coefficient-line (cross-domain alias: `Gamma`, `ZL-minus-Z0`, `s11`)
**Domain:** Electromagnetics / Antennas
**Definition:** Γ = (Z_L − Z₀)/(Z_L + Z₀). Voltage-wave reflection at impedance discontinuity.
**Atom or composite:** Atom — interface scattering ratio.
**Cost model:** Trivial complex arithmetic.
**Real wall?** Yes — exact for TEM lines.
**Cross-domain wiring:** signal-processing-rf (S-parameter S11), networking (return-loss budget).
**Notes:** Pozar §2.

### vswr (cross-domain alias: `voltage-swr`, `1-plus-mag-Gamma`, `mismatch-ratio`)
**Domain:** Electromagnetics / Antennas
**Definition:** VSWR = (1 + |Γ|)/(1 − |Γ|). Ratio of max to min envelope on mismatched line.
**Atom or composite:** Atom — derived from |Γ|.
**Cost model:** Trivial.
**Real wall?** Yes — observable on slotted line.
**Cross-domain wiring:** signal-processing-rf (matching network design), networking (antenna return loss).
**Notes:** Pozar §2.5.

### smith-chart (cross-domain alias: `gamma-plane`, `impedance-conformal-map`, `z-y-chart`)
**Domain:** Electromagnetics / Antennas
**Definition:** Conformal map of right-half-z-plane to unit disk in Γ. Resistance/reactance circles.
**Atom or composite:** Composite — Möbius transform of impedance.
**Cost model:** Pencil-and-paper; trivial computation.
**Real wall?** No — graphical convenience.
**Cross-domain wiring:** linear-algebra-matrix (Möbius), control-numerical-opt (Nyquist-like plot).
**Notes:** Smith 1939.

### quarter-wave-transformer (cross-domain alias: `lambda-over-4-match`, `sqrt-Z1-Z2`, `pi-band-narrow`)
**Domain:** Electromagnetics / Antennas
**Definition:** Single λ/4 section of impedance Z_T = √(Z_S · Z_L) matches real loads.
**Atom or composite:** Composite — TMM single-layer analog for transmission lines.
**Cost model:** Trivial; narrowband.
**Real wall?** No — geometric.
**Cross-domain wiring:** signal-processing-rf (matching), photonics-optics (AR coating analog).
**Notes:** Pozar §5.

### single-stub-matching (cross-domain alias: `shunt-stub`, `b-cancel`, `open-or-short-stub`)
**Domain:** Electromagnetics / Antennas
**Definition:** Open/short stub at distance d from load cancels imaginary part of Y.
**Atom or composite:** Composite — line + parallel reactance.
**Cost model:** Two parameters (d, l_stub); analytic.
**Real wall?** No — design choice.
**Cross-domain wiring:** signal-processing-rf (impedance match), networking (T-junction).
**Notes:** Pozar §5.2.

### double-stub-matching (cross-domain alias: `two-stub`, `forbidden-region`, `Y-chart-match`)
**Domain:** Electromagnetics / Antennas
**Definition:** Two stubs at fixed positions; tunes match without moving stub location.
**Atom or composite:** Composite — two-parameter network.
**Cost model:** Forbidden region for some loads.
**Real wall?** No — but topology constraint.
**Cross-domain wiring:** signal-processing-rf (adjustable match), control-numerical-opt (constrained 2-D matching).
**Notes:** Pozar §5.3.

### binomial-multisection-transformer (cross-domain alias: `maximally-flat-match`, `binomial-coeffs-Z`, `butterworth-match`)
**Domain:** Electromagnetics / Antennas
**Definition:** N-section transformer with reflection coefficients ∝ binomial; maximally flat |Γ| at f₀.
**Atom or composite:** Composite — TMM with binomial weighting.
**Cost model:** Closed-form section impedances.
**Real wall?** No — design choice (Butterworth analog).
**Cross-domain wiring:** signal-processing-rf (Butterworth filters), control-numerical-opt (maximally-flat polynomials).
**Notes:** Pozar §5.6.

### chebyshev-multisection-transformer (cross-domain alias: `equi-ripple-match`, `chebyshev-Tn`, `min-max-Gamma`)
**Domain:** Electromagnetics / Antennas
**Definition:** N-section equiripple match; optimal min-max |Γ| over band.
**Atom or composite:** Composite — TMM with Chebyshev coefficients.
**Cost model:** Closed-form via T_N polynomial.
**Real wall?** Yes — optimal in min-max sense.
**Cross-domain wiring:** signal-processing-rf (Chebyshev filters), control-numerical-opt (Remez algorithm).
**Notes:** Collin 1955.

### bode-fano-limit (cross-domain alias: `matching-bandwidth-bound`, `ln-1-over-Gamma`, `gain-bw-product-RC`)
**Domain:** Electromagnetics / Antennas
**Definition:** ∫ ln(1/|Γ|) dω ≤ π/(R C) for parallel-RC load. Fundamental match-bandwidth limit.
**Atom or composite:** Atom — integral causality constraint.
**Cost model:** Free; tells you how good a match can ever be.
**Real wall?** Yes — strict bound from Kramers-Kronig.
**Cross-domain wiring:** signal-processing-rf (impedance match), information-theory-coding (channel-capacity analog).
**Notes:** Bode 1945; Fano 1950.

### conjugate-matching (cross-domain alias: `ZL-equals-Zs-conj`, `max-power-transfer`, `image-impedance`)
**Domain:** Electromagnetics / Antennas
**Definition:** Maximum power transferred when Z_L = Z_S*.
**Atom or composite:** Atom — power-transfer theorem.
**Cost model:** Single-frequency design.
**Real wall?** Yes — derived from extremizing |I|²R_L.
**Cross-domain wiring:** signal-processing-rf (LNA input match), control-numerical-opt (optimal source/load).
**Notes:** Jacobi's law.

---

## Antenna Fundamentals

### radiation-pattern (cross-domain alias: `far-field-pattern`, `F-theta-phi`, `directional-gain-shape`)
**Domain:** Electromagnetics / Antennas
**Definition:** Angular distribution of far-field power: U(θ,φ) = |F(θ,φ)|².
**Atom or composite:** Composite — far-field Fourier transform of aperture current.
**Cost model:** O(N) samples in (θ, φ); FFT for periodic arrays.
**Real wall?** No — but governed by aperture-limit theorem.
**Cross-domain wiring:** signal-processing-rf (spatial spectrum), retrieval-search (kernel directionality).
**Notes:** Balanis Ch. 2.

### directivity (cross-domain alias: `D0`, `4pi-over-Omega-A`, `max-U-over-avg`)
**Domain:** Electromagnetics / Antennas
**Definition:** D₀ = 4π U_max / P_rad = 4π/Ω_A.
**Atom or composite:** Atom — pattern-shape figure of merit.
**Cost model:** Single number; computed by integrating pattern.
**Real wall?** Yes — limited by aperture for given size.
**Cross-domain wiring:** information-theory-coding (concentration), signal-processing-rf (array gain).
**Notes:** Balanis §2.

### gain-antenna (cross-domain alias: `realized-gain`, `D0-times-eta`, `IEEE-gain`)
**Domain:** Electromagnetics / Antennas
**Definition:** G = e_cd · D₀ (efficiency × directivity); realized G = G · (1−|Γ|²).
**Atom or composite:** Composite — directivity scaled by losses.
**Cost model:** Adds efficiency measurement.
**Real wall?** Yes — efficiency ≤ 1.
**Cross-domain wiring:** networking (link budget), signal-processing-rf (system gain).
**Notes:** Friis link uses G_t G_r.

### effective-aperture (cross-domain alias: `Ae`, `G-lambda2-over-4pi`, `capture-area`)
**Domain:** Electromagnetics / Antennas
**Definition:** A_e = G λ²/(4π). Equivalent capture area for receive.
**Atom or composite:** Atom — reciprocity-derived RX figure.
**Cost model:** Trivial.
**Real wall?** Yes — λ² scaling fundamental at long λ.
**Cross-domain wiring:** networking (RX power), retrieval-search (aperture diversity).
**Notes:** Schelkunoff 1952.

### friis-transmission (cross-domain alias: `link-equation`, `Pr-Pt-Gt-Gr`, `path-loss-formula`)
**Domain:** Electromagnetics / Antennas
**Definition:** P_r = P_t G_t G_r (λ/4πR)². Free-space link budget.
**Atom or composite:** Composite — directivity + free-space spreading.
**Cost model:** Trivial.
**Real wall?** Yes in free space; corrected by environment.
**Cross-domain wiring:** networking (radio links), control-numerical-opt (range optimization).
**Notes:** Friis 1946.

### beamwidth-hpbw (cross-domain alias: `3-db-beamwidth`, `half-power-bw`, `theta-hpbw`)
**Domain:** Electromagnetics / Antennas
**Definition:** Angular width between half-power points of main beam.
**Atom or composite:** Atom — pattern feature.
**Cost model:** Root-finding on pattern.
**Real wall?** Yes — limited by aperture/wavelength.
**Cross-domain wiring:** signal-processing-rf (resolution), retrieval-search (Rayleigh limit).
**Notes:** Approximation θ_HPBW ≈ kλ/D, k ≈ 1.

### sidelobe-level (cross-domain alias: `sll`, `peak-sidelobe`, `db-down`)
**Domain:** Electromagnetics / Antennas
**Definition:** Peak of strongest sidelobe in dB below main beam.
**Atom or composite:** Atom — pattern descriptor.
**Cost model:** Local maximum search.
**Real wall?** Set by aperture weighting; e.g. -13 dB uniform, -23 dB Hamming.
**Cross-domain wiring:** signal-processing-rf (FFT windowing), statistics-probability (leakage).
**Notes:** Balanis Ch. 6.

### polarization-axial-ratio (cross-domain alias: `ar-db`, `major-over-minor`, `cp-quality`)
**Domain:** Electromagnetics / Antennas
**Definition:** AR = E_major/E_minor for elliptical pol; 0 dB = pure CP, ∞ = linear.
**Atom or composite:** Atom — single number summarizing ellipse.
**Cost model:** Two field components → ratio.
**Real wall?** No — design quality metric.
**Cross-domain wiring:** signal-processing-rf (dual-pol systems), quantum-computing (qubit fidelity analog).
**Notes:** IEEE 145.

### radiation-resistance (cross-domain alias: `Rr`, `Prad-over-I0sq`, `aperture-loss-to-far-field`)
**Domain:** Electromagnetics / Antennas
**Definition:** R_r such that P_rad = ½|I₀|²R_r. R_r ≈ 80π²(L/λ)² for short dipole.
**Atom or composite:** Atom — derived from far-field integration.
**Cost model:** Integral over sphere; tabulated for canonical antennas.
**Real wall?** Yes — physical impedance.
**Cross-domain wiring:** signal-processing-rf (antenna matching), networking (RX impedance).
**Notes:** Balanis §4.

### near-field-regions (cross-domain alias: `reactive-fresnel-fraunhofer`, `0.62-sqrt-D3-lambda`, `2D2-over-lambda`)
**Domain:** Electromagnetics / Antennas
**Definition:** Reactive ≤ 0.62√(D³/λ); Fresnel 0.62√(D³/λ) ≤ r ≤ 2D²/λ; Fraunhofer ≥ 2D²/λ.
**Atom or composite:** Composite — phase-error criteria.
**Cost model:** Boundary calculations; trivial.
**Real wall?** Yes — λ/16 phase tolerance derivation.
**Cross-domain wiring:** signal-processing-rf (near-field measurement), retrieval-search (near vs far representation).
**Notes:** Balanis §2.

### antenna-factor (cross-domain alias: `AF`, `E-over-V`, `emc-receive-factor`)
**Domain:** Electromagnetics / Antennas
**Definition:** AF = E_inc / V_out. Used in EMI/EMC.
**Atom or composite:** Atom — calibration constant.
**Cost model:** Per-frequency calibration.
**Real wall?** Yes — physical RX response.
**Cross-domain wiring:** signal-processing-rf (spectrum analyzer cal), networking (compliance measurements).
**Notes:** CISPR standards.

### link-budget (cross-domain alias: `db-add-table`, `eirp-minus-pl-plus-gr`, `tx-to-rx-balance`)
**Domain:** Electromagnetics / Antennas
**Definition:** Sum (in dB) of gains/losses from TX to RX: EIRP − L_path − L_misc + G_r − N₀.
**Atom or composite:** Composite — additive ledger.
**Cost model:** Spreadsheet.
**Real wall?** Yes — encodes Shannon-limit through SNR.
**Cross-domain wiring:** networking (link engineering), information-theory-coding (SNR → capacity).
**Notes:** Standard satellite-systems chapter.

---

## Canonical Antennas

### infinitesimal-dipole (cross-domain alias: `hertzian-dipole`, `dl-current-element`, `point-radiator`)
**Domain:** Electromagnetics / Antennas
**Definition:** Current element I·dl ≪ λ. Pattern sin²θ; R_r = 80π²(dl/λ)².
**Atom or composite:** Atom — building block for all wire antennas.
**Cost model:** Closed-form fields.
**Real wall?** Yes — radiation impedance physics.
**Cross-domain wiring:** physics-diffusion (point source), signal-processing-rf (impulse response).
**Notes:** Hertz 1887.

### half-wave-dipole (cross-domain alias: `lambda-over-2-dipole`, `73-ohm`, `resonant-dipole`)
**Domain:** Electromagnetics / Antennas
**Definition:** Length = λ/2; R_in ≈ 73 + j42.5 Ω; reactance ≈ 0 at L ≈ 0.48λ.
**Atom or composite:** Composite — superposition of dipole elements with sinusoidal current.
**Cost model:** Closed-form pattern via integration.
**Real wall?** Yes — resonance at λ/2.
**Cross-domain wiring:** signal-processing-rf (matched 75 Ω TV input), networking (legacy broadcast).
**Notes:** Balanis §4.

### monopole-quarter-wave (cross-domain alias: `lambda-over-4-mono`, `36-5-ohm`, `image-theory`)
**Domain:** Electromagnetics / Antennas
**Definition:** λ/4 vertical over ground; image-theory equivalent to λ/2 dipole; R_in ≈ 36.5 Ω.
**Atom or composite:** Composite — dipole + ground image.
**Cost model:** Identical to dipole with half-space integration.
**Real wall?** Yes — image theory exact for PEC ground.
**Cross-domain wiring:** networking (AM/FM whips, GSM monopoles).
**Notes:** Balanis §4.

### small-loop-antenna (cross-domain alias: `magnetic-dipole`, `H-field-probe`, `circumf-lt-lambda-10`)
**Domain:** Electromagnetics / Antennas
**Definition:** Loop with C ≪ λ; magnetic-dipole pattern; R_r = 320π⁴(A/λ²)².
**Atom or composite:** Atom — magnetic dual of small electric dipole.
**Cost model:** Closed-form fields.
**Real wall?** Yes — low R_r → low efficiency without ferrite.
**Cross-domain wiring:** networking (NFC, RFID HF), signal-processing-rf (E/H probes).
**Notes:** Balanis §5.

### large-resonant-loop (cross-domain alias: `1-wavelength-loop`, `cobweb`, `quad-element`)
**Domain:** Electromagnetics / Antennas
**Definition:** Loop ≈ λ; standing-wave current; R_in ≈ 100–200 Ω; broadside pattern.
**Atom or composite:** Composite — sinusoidal-current superposition.
**Cost model:** Numerical for arbitrary shape; closed-form circular.
**Real wall?** Yes — resonance condition.
**Cross-domain wiring:** signal-processing-rf (cubical-quad arrays).
**Notes:** Used in HF directional arrays.

### folded-dipole (cross-domain alias: `300-ohm-feed`, `parallel-pair-dipole`, `tv-twin-lead`)
**Domain:** Electromagnetics / Antennas
**Definition:** Two parallel λ/2 conductors joined at ends; R_in ≈ 4×73 ≈ 280–300 Ω.
**Atom or composite:** Composite — two-conductor mode + radiation mode.
**Cost model:** Closed-form impedance transformation.
**Real wall?** No — design choice for impedance match.
**Cross-domain wiring:** signal-processing-rf (300 Ω twin-lead match), networking (legacy VHF TV).
**Notes:** Carter 1932.

### yagi-uda-array (cross-domain alias: `parasitic-array`, `director-reflector-driven`, `vhf-tv-antenna`)
**Domain:** Electromagnetics / Antennas
**Definition:** Driven element + reflector + directors; end-fire gain 7–15 dBi; narrowband.
**Atom or composite:** Composite — coupled-dipole array with parasitic excitation.
**Cost model:** MoM with N elements; design tables.
**Real wall?** Yes — element spacings tuned to phase delays.
**Cross-domain wiring:** signal-processing-rf (end-fire arrays), networking (HAM VHF/UHF).
**Notes:** Uda 1926, Yagi 1928.

### log-periodic-dipole-array (cross-domain alias: `lpda`, `frequency-independent`, `tau-sigma-design`)
**Domain:** Electromagnetics / Antennas
**Definition:** Geometrically scaled dipoles fed with phase reversal; constant-impedance, constant-pattern over decade BW.
**Atom or composite:** Composite — self-similar dipole array.
**Cost model:** Tabulated design; MoM verification.
**Real wall?** Yes — self-similarity gives frequency-independence within band edges.
**Cross-domain wiring:** signal-processing-rf (wideband measurement), computational-geometry (fractal/self-similar).
**Notes:** DuHamel 1957; Rumsey 1957.

### helical-antenna-axial (cross-domain alias: `axial-mode-helix`, `cp-helix`, `kraus-helix`)
**Domain:** Electromagnetics / Antennas
**Definition:** Helix with C ≈ λ; circularly-polarized end-fire; gain ≈ 10·log(N·S/λ).
**Atom or composite:** Composite — traveling-wave on helix.
**Cost model:** MoM/NEC.
**Real wall?** Yes — circumference must match λ.
**Cross-domain wiring:** signal-processing-rf (CP satcom uplink), quantum-computing (chiral structures).
**Notes:** Kraus 1947.

### helical-antenna-normal (cross-domain alias: `normal-mode-helix`, `short-helix`, `whip-replacement`)
**Domain:** Electromagnetics / Antennas
**Definition:** Helix with dimensions ≪ λ; broadside pattern; behaves as shortened dipole.
**Atom or composite:** Composite — coiled small antenna.
**Cost model:** Closed-form approximations.
**Real wall?** Yes — small-antenna Q limit (Chu).
**Cross-domain wiring:** networking (HT whips), signal-processing-rf (handheld radios).
**Notes:** Wheeler 1947.

### biconical-antenna (cross-domain alias: `bicone`, `infinite-bicone-Z0`, `tem-cone`)
**Domain:** Electromagnetics / Antennas
**Definition:** Two opposed cones fed at apex; supports spherical TEM; broadband.
**Atom or composite:** Composite — spherical-coordinate TEM mode.
**Cost model:** Closed-form for infinite cone; truncated bicone tabulated.
**Real wall?** Yes — TEM mode geometry.
**Cross-domain wiring:** signal-processing-rf (broadband measurement antennas, EMC).
**Notes:** Schelkunoff 1939.

### discone-antenna (cross-domain alias: `disc-cone`, `scanner-antenna`, `omnidirectional-broadband`)
**Domain:** Electromagnetics / Antennas
**Definition:** Disc + cone biconical variant; broadband (decade) omnidirectional vertical.
**Atom or composite:** Composite — truncated biconical.
**Cost model:** Design tables.
**Real wall?** Yes — cone half-angle sets impedance.
**Cross-domain wiring:** signal-processing-rf (scanners, EMC measurement).
**Notes:** Kandoian 1945.

### pyramidal-horn (cross-domain alias: `e-h-flared-horn`, `gain-horn`, `standard-gain`)
**Domain:** Electromagnetics / Antennas
**Definition:** Rectangular WG flared in both E and H planes; aperture-limited gain.
**Atom or composite:** Composite — aperture distribution from flared TE₁₀.
**Cost model:** Aperture integration; closed-form approximations.
**Real wall?** Yes — gain ≈ 6.4 A_p/λ² for optimum flare.
**Cross-domain wiring:** signal-processing-rf (calibration standards), networking (feed horns).
**Notes:** Balanis Ch. 13.

### conical-horn (cross-domain alias: `circular-horn`, `te11-horn`, `cp-feed-horn`)
**Domain:** Electromagnetics / Antennas
**Definition:** Circular WG flared into cone; supports CP with TE₁₁ + cross-pol terms.
**Atom or composite:** Composite — aperture from flared circular WG.
**Cost model:** Mode-matching at throat; aperture integration.
**Real wall?** Yes — flare angle sets phase error.
**Cross-domain wiring:** signal-processing-rf (satellite feeds).
**Notes:** Balanis §13.

### corrugated-horn (cross-domain alias: `hybrid-mode-horn`, `low-cross-pol-feed`, `radio-astronomy-horn`)
**Domain:** Electromagnetics / Antennas
**Definition:** Inner walls corrugated at λ/4 depth to enforce HE₁₁ hybrid mode; symmetric pattern, low cross-pol.
**Atom or composite:** Composite — anisotropic-impedance wall.
**Cost model:** Mode-matching; corrugation depth design.
**Real wall?** Yes — λ/4 wall reactance.
**Cross-domain wiring:** signal-processing-rf (radio-astronomy feeds), networking (deep-space DSN).
**Notes:** Clarricoats & Olver.

### e-plane-sectoral-horn (cross-domain alias: `e-flare`, `narrow-h-broad-e`, `sectoral-e`)
**Domain:** Electromagnetics / Antennas
**Definition:** Flare in E-plane only; fan-shaped pattern.
**Atom or composite:** Composite — 1D flare aperture.
**Cost model:** Closed-form aperture integration.
**Real wall?** Yes — phase error in flared dimension.
**Cross-domain wiring:** signal-processing-rf (slotted-line equipment).
**Notes:** Balanis §13.

### h-plane-sectoral-horn (cross-domain alias: `h-flare`, `wide-pattern-vertical`, `sectoral-h`)
**Domain:** Electromagnetics / Antennas
**Definition:** Flare in H-plane only.
**Atom or composite:** Composite — 1D flare aperture orthogonal to E-sectoral.
**Cost model:** Closed-form.
**Real wall?** Yes — quadratic phase error.
**Cross-domain wiring:** signal-processing-rf (broadcast).
**Notes:** Balanis §13.

### parabolic-reflector (cross-domain alias: `dish-antenna`, `prime-focus-dish`, `f-d-ratio`)
**Domain:** Electromagnetics / Antennas
**Definition:** Surface z = ρ²/(4f); collimates focal-point source to plane wave.
**Atom or composite:** Composite — GO + aperture-field method.
**Cost model:** PO surface integration; >λ² apertures.
**Real wall?** Yes — surface tolerance ≤ λ/16 RMS.
**Cross-domain wiring:** photonics-optics (optical telescopes), graphics-rendering-lod (catadioptrics).
**Notes:** Newton 1668 (telescope); Balanis Ch. 15.

### cassegrain-reflector (cross-domain alias: `dual-reflector`, `convex-hyperbolic-sub`, `folded-feed`)
**Domain:** Electromagnetics / Antennas
**Definition:** Parabolic main + convex hyperbolic sub-reflector; feed at vertex of main.
**Atom or composite:** Composite — two-reflector system.
**Cost model:** GO + PO; spillover & blockage analysis.
**Real wall?** Yes — surface and alignment tolerances.
**Cross-domain wiring:** photonics-optics (Cassegrain telescopes), networking (large earth-station antennas).
**Notes:** Cassegrain 1672.

### gregorian-reflector (cross-domain alias: `concave-elliptical-sub`, `gregory-dish`, `dual-reflector-gregorian`)
**Domain:** Electromagnetics / Antennas
**Definition:** Parabolic main + concave elliptical sub; feed beyond prime focus.
**Atom or composite:** Composite — two-reflector system.
**Cost model:** GO + PO.
**Real wall?** Yes — alignment + surface tolerance.
**Cross-domain wiring:** photonics-optics (Gregorian telescopes), networking (deep-space antennas).
**Notes:** Gregory 1663.

### offset-reflector (cross-domain alias: `offset-fed-dish`, `no-blockage`, `home-satellite`)
**Domain:** Electromagnetics / Antennas
**Definition:** Portion of paraboloid offset from axis to remove feed blockage.
**Atom or composite:** Composite — cropped paraboloid.
**Cost model:** Same PO; lower spillover.
**Real wall?** Yes — manufacturing tolerance.
**Cross-domain wiring:** networking (home VSAT dishes).
**Notes:** Used in DBS receive dishes.

### dielectric-lens-antenna (cross-domain alias: `eps-lens`, `n-of-r-lens`, `hyperbolic-lens`)
**Domain:** Electromagnetics / Antennas
**Definition:** Dielectric profile collimates feed-horn radiation; like optical lens.
**Atom or composite:** Composite — GO + dielectric.
**Cost model:** Ray-tracing; PO for diffraction.
**Real wall?** Yes — surface and homogeneity tolerances.
**Cross-domain wiring:** photonics-optics (optical lenses), networking (mmWave 5G).
**Notes:** Balanis §16.

### luneburg-lens (cross-domain alias: `graded-index-sphere`, `n-r-sqrt-2-minus-r2`, `360-deg-scan`)
**Domain:** Electromagnetics / Antennas
**Definition:** Sphere with n(r) = √(2 − (r/R)²); focuses plane wave to opposite surface.
**Atom or composite:** Composite — radial GRIN sphere.
**Cost model:** Geometric optics; multi-feed for multibeam.
**Real wall?** No — GRIN profile is a design choice; manufacturing-limited.
**Cross-domain wiring:** photonics-optics (GRIN lenses), networking (multibeam antennas).
**Notes:** Luneburg 1944.

### rotman-lens (cross-domain alias: `microwave-beamformer`, `true-time-delay-lens`, `passive-bff`)
**Domain:** Electromagnetics / Antennas
**Definition:** Parallel-plate region with multiple beam ports and array ports; gives true-time-delay beam steering.
**Atom or composite:** Composite — geometric path-length design.
**Cost model:** Closed-form Rotman equations.
**Real wall?** No — geometric design freedom.
**Cross-domain wiring:** signal-processing-rf (analog beamformer), networking (5G multibeam).
**Notes:** Rotman & Turner 1963.

### microstrip-patch-antenna (cross-domain alias: `patch`, `rectangular-mpa`, `pcb-antenna`)
**Domain:** Electromagnetics / Antennas
**Definition:** Conducting patch over ground; radiates as two-slot cavity; gain ~6 dBi.
**Atom or composite:** Composite — cavity model or full-wave.
**Cost model:** Closed-form cavity model; MoM for accuracy.
**Real wall?** Yes — narrow BW (~few %) from high Q.
**Cross-domain wiring:** networking (Wi-Fi, GPS receivers), signal-processing-rf (array elements).
**Notes:** Munson 1972; Balanis §14.

### slot-antenna (cross-domain alias: `babinet-slot`, `complementary-dipole`, `aperture-radiator`)
**Domain:** Electromagnetics / Antennas
**Definition:** Slot in conducting plane; pattern dual to dipole; Z_slot · Z_dip = η²/4.
**Atom or composite:** Composite — Babinet's principle applied.
**Cost model:** Dual of dipole calculation.
**Real wall?** Yes — Babinet exact for infinite plane.
**Cross-domain wiring:** networking (waveguide slot arrays).
**Notes:** Booker 1946.

### dielectric-resonator-antenna (cross-domain alias: `dra`, `eps-block-radiator`, `high-q-dielectric`)
**Domain:** Electromagnetics / Antennas
**Definition:** High-ε ceramic block resonator coupled to feed; radiates from modes leaking off the surface.
**Atom or composite:** Composite — dielectric mode + radiation.
**Cost model:** Full-wave; HFSS/CST.
**Real wall?** Yes — modal Q and bandwidth tradeoff.
**Cross-domain wiring:** signal-processing-rf (low-loss antennas), networking (UWB).
**Notes:** Long, McAllister & Shen 1983.

### vivaldi-antenna (cross-domain alias: `tsa`, `tapered-slot-antenna`, `exp-flared-slot`)
**Domain:** Electromagnetics / Antennas
**Definition:** Exponentially-flared slot on grounded dielectric; broadband end-fire.
**Atom or composite:** Composite — traveling-wave on tapered slot.
**Cost model:** Full-wave.
**Real wall?** Yes — exponential taper sets BW.
**Cross-domain wiring:** signal-processing-rf (UWB radar), retrieval-search (UWB sensors).
**Notes:** Gibson 1979.

### spiral-antenna (cross-domain alias: `archimedean-spiral`, `equiangular-spiral`, `frequency-independent`)
**Domain:** Electromagnetics / Antennas
**Definition:** Two-arm planar spiral; circularly polarized; decade-plus bandwidth.
**Atom or composite:** Composite — self-complementary structure.
**Cost model:** Full-wave; absorber cavity.
**Real wall?** Yes — self-complementarity gives BW.
**Cross-domain wiring:** signal-processing-rf (ESM/EW), retrieval-search (broadband DF).
**Notes:** DuHamel & Dyson 1957.

### leaky-wave-antenna (cross-domain alias: `lwa`, `slow-wave-leakage`, `freq-scanned-beam`)
**Domain:** Electromagnetics / Antennas
**Definition:** Waveguide with leakage along length; beam scans with frequency.
**Atom or composite:** Composite — periodic perturbation of guided mode.
**Cost model:** Floquet expansion; dispersion analysis.
**Real wall?** Yes — leakage rate sets beamwidth/efficiency.
**Cross-domain wiring:** signal-processing-rf (frequency-scanned radar), networking (mmWave beam steering).
**Notes:** Hines 1953.

### fractal-antenna-sierpinski (cross-domain alias: `sierpinski-multiband`, `self-similar-antenna`, `triangle-fractal`)
**Domain:** Electromagnetics / Antennas
**Definition:** Sierpinski gasket geometry; multiple log-spaced resonances.
**Atom or composite:** Composite — self-similar geometry.
**Cost model:** MoM; resonance scaling factor 2.
**Real wall?** Yes — geometric self-similarity sets resonance set.
**Cross-domain wiring:** computational-geometry (fractals), signal-processing-rf (multiband).
**Notes:** Puente 1996.

### fractal-antenna-koch (cross-domain alias: `koch-monopole`, `space-filling`, `meandered-fractal`)
**Domain:** Electromagnetics / Antennas
**Definition:** Koch curve lengthens resonator perimeter in finite area; lowers resonance.
**Atom or composite:** Composite — space-filling curve.
**Cost model:** MoM; iteration count = order.
**Real wall?** Yes — Chu small-antenna limit still applies.
**Cross-domain wiring:** computational-geometry (Koch curve), signal-processing-rf (electrically small).
**Notes:** Puente 1998.

### pifa-antenna (cross-domain alias: `planar-ifa`, `phone-antenna`, `shorted-patch`)
**Domain:** Electromagnetics / Antennas
**Definition:** Shorted microstrip patch; reduced size; low profile; multiband with parasitic strips.
**Atom or composite:** Composite — short-pin patch.
**Cost model:** Full-wave; tuned by short position.
**Real wall?** Yes — size–BW tradeoff (Chu).
**Cross-domain wiring:** networking (cellular handsets), signal-processing-rf (multiband).
**Notes:** Taga 1990.

### inverted-f-antenna (cross-domain alias: `ifa`, `wire-pifa`, `mobile-monopole-replacement`)
**Domain:** Electromagnetics / Antennas
**Definition:** Wire λ/4 monopole bent into "F" with shorted leg; matches ~50 Ω.
**Atom or composite:** Composite — bent monopole with parasitic short.
**Cost model:** NEC/MoM.
**Real wall?** Yes — Chu small-antenna limit.
**Cross-domain wiring:** networking (Wi-Fi, BT, Zigbee modules).
**Notes:** Wong 2003.

### chip-antenna (cross-domain alias: `surface-mount-antenna`, `ceramic-chip`, `low-pcb-footprint`)
**Domain:** Electromagnetics / Antennas
**Definition:** Ceramic SMT component with embedded radiator + matching; needs cleared ground.
**Atom or composite:** Composite — high-ε embedded structure.
**Cost model:** Vendor-supplied S-params.
**Real wall?** Yes — Chu small-antenna limit.
**Cross-domain wiring:** networking (BLE, IoT modules).
**Notes:** Common parts: Johanson, Pulse, AVX.

---

## Antenna Arrays & Beamforming

### array-factor (cross-domain alias: `AF-array`, `phased-sum`, `pattern-multiplication`)
**Domain:** Electromagnetics / Antennas
**Definition:** AF(θ,φ) = Σ a_n exp(j(k·r_n + α_n)). Pattern = element pattern × AF.
**Atom or composite:** Composite — discrete spatial-DFT-like sum.
**Cost model:** N complex sums per direction.
**Real wall?** No — geometric & weighting design.
**Cross-domain wiring:** signal-processing-rf (spatial DFT), linear-algebra-matrix (vector inner product).
**Notes:** Balanis Ch. 6.

### broadside-array (cross-domain alias: `0-deg-progressive-phase`, `peak-normal`, `transverse-firing`)
**Domain:** Electromagnetics / Antennas
**Definition:** All elements in phase; main beam normal to array axis.
**Atom or composite:** Composite — uniform-phase array.
**Cost model:** Single configuration.
**Real wall?** No — design choice.
**Cross-domain wiring:** signal-processing-rf (beamformer pointing).
**Notes:** Balanis §6.

### end-fire-array (cross-domain alias: `axial-firing`, `progressive-phase-equals-kd`, `yagi-like`)
**Domain:** Electromagnetics / Antennas
**Definition:** Progressive phase α = ±kd; beam along array axis.
**Atom or composite:** Composite — fully-phased uniform array.
**Cost model:** Single configuration.
**Real wall?** Yes — Hansen-Woodyard increases directivity at cost of SLL.
**Cross-domain wiring:** signal-processing-rf (Hansen-Woodyard condition).
**Notes:** Hansen-Woodyard 1938.

### planar-array (cross-domain alias: `2d-array`, `nx-ny-grid`, `rectangular-lattice`)
**Domain:** Electromagnetics / Antennas
**Definition:** Elements on rectangular lattice; pattern = AF_x · AF_y × element.
**Atom or composite:** Composite — outer product of two linear arrays.
**Cost model:** N_x × N_y complex sum.
**Real wall?** Yes — λ/2 spacing avoids grating lobes within visible space.
**Cross-domain wiring:** signal-processing-rf (2-D spatial FFT), linear-algebra-matrix (Kronecker product).
**Notes:** Balanis §6.10.

### conformal-array (cross-domain alias: `curved-array`, `body-conformal`, `non-planar-lattice`)
**Domain:** Electromagnetics / Antennas
**Definition:** Elements on curved surface (cylinder, sphere, airframe).
**Atom or composite:** Composite — generalized AF on non-Cartesian lattice.
**Cost model:** No closed-form; full-wave + element-pattern variation.
**Real wall?** Yes — element pattern varies across surface.
**Cross-domain wiring:** signal-processing-rf (aerospace radar), computational-geometry (surface lattices).
**Notes:** Hansen "Conformal Antenna Array Design".

### phased-array (cross-domain alias: `electronically-scanned`, `psa-aesa`, `phase-controlled-beam`)
**Domain:** Electromagnetics / Antennas
**Definition:** Element phases steer beam without mechanical motion.
**Atom or composite:** Composite — array + per-element phase shifter.
**Cost model:** N phase shifters; calibration matrix.
**Real wall?** Yes — scan loss (cos θ), grating lobes, beam-squint with frequency.
**Cross-domain wiring:** signal-processing-rf (radar beam scheduling), control-numerical-opt (beam search).
**Notes:** Skolnik "Radar Handbook".

### aesa (cross-domain alias: `active-esa`, `tr-module-per-element`, `solid-state-radar`)
**Domain:** Electromagnetics / Antennas
**Definition:** Each element has its own T/R module (amp + phase shifter); high duty cycle.
**Atom or composite:** Composite — distributed T/R + array.
**Cost model:** Scales linearly with N elements; cost-dominant in modern radar.
**Real wall?** Yes — power per element bounded by thermal/T/R limits.
**Cross-domain wiring:** signal-processing-rf (digital beamforming), networking (mmWave 5G AESA).
**Notes:** Brookner "Practical Phased Array Antenna Systems".

### butler-matrix (cross-domain alias: `n-by-n-bf`, `hybrid-coupler-matrix`, `fft-bff`)
**Domain:** Electromagnetics / Antennas
**Definition:** Lossless N×N (N=2^k) network of 90° hybrids; outputs N orthogonal beams.
**Atom or composite:** Composite — analog Cooley-Tukey FFT.
**Cost model:** N log₂ N hybrids.
**Real wall?** No — fixed beam set.
**Cross-domain wiring:** signal-processing-rf (analog FFT beamformer), linear-algebra-matrix (DFT).
**Notes:** Butler & Lowe 1961.

### blass-matrix (cross-domain alias: `delay-line-bff`, `coupler-tree`, `true-time-delay-net`)
**Domain:** Electromagnetics / Antennas
**Definition:** Resistive coupler/delay network producing M independent beams from N elements.
**Atom or composite:** Composite — true-time-delay analog network.
**Cost model:** N×M couplers; lossy.
**Real wall?** Yes — squint-free over wideband.
**Cross-domain wiring:** signal-processing-rf (wideband multibeam), networking (TTD arrays).
**Notes:** Blass 1960.

### digital-beamforming (cross-domain alias: `dbf`, `per-element-adc`, `software-beamformer`)
**Domain:** Electromagnetics / Antennas
**Definition:** Each element has independent RF chain + ADC/DAC; beams formed in DSP.
**Atom or composite:** Composite — array + per-element rcv chain.
**Cost model:** O(N) ADCs; very flexible.
**Real wall?** Yes — power & data-rate limited.
**Cross-domain wiring:** signal-processing-rf (MIMO), ml-training (RX-side learning).
**Notes:** Van Trees "Optimum Array Processing".

### hybrid-beamforming (cross-domain alias: `analog-plus-digital`, `subarray-bf`, `mmwave-bf`)
**Domain:** Electromagnetics / Antennas
**Definition:** Analog phase shifters per subarray + digital combining of M subarrays.
**Atom or composite:** Composite — two-tier beamformer.
**Cost model:** M ADCs (M ≪ N); reduces power vs full DBF.
**Real wall?** Yes — reduced flexibility vs DBF.
**Cross-domain wiring:** networking (5G mmWave), signal-processing-rf (massive MIMO).
**Notes:** Heath et al. 2016 review.

### true-time-delay-beamforming (cross-domain alias: `ttd-bf`, `wideband-no-squint`, `delay-line-bf`)
**Domain:** Electromagnetics / Antennas
**Definition:** True delays (not phase shifts) eliminate beam-squint over wideband.
**Atom or composite:** Composite — delay-line per element/subarray.
**Cost model:** Bulky for long delays; photonic TTD reduces size.
**Real wall?** Yes — needed for fractional BW >10%.
**Cross-domain wiring:** signal-processing-rf (wideband DBF), photonics-optics (photonic TTD).
**Notes:** Frigyes & Seeds 1995.

### subarray-architecture (cross-domain alias: `tile-array`, `subarray-thinning`, `hierarchy-bff`)
**Domain:** Electromagnetics / Antennas
**Definition:** Group elements into subarrays each with own phase shifter / delay.
**Atom or composite:** Composite — hierarchical AF.
**Cost model:** Reduces shifter count; introduces quantization sidelobes.
**Real wall?** Yes — sub-array grating lobes if too large.
**Cross-domain wiring:** signal-processing-rf (tiled radar), control-numerical-opt (clustering).
**Notes:** Mailloux "Phased Array Antenna Handbook".

### grating-lobes (cross-domain alias: `aliasing-lobes`, `d-gt-lambda-over-2`, `array-aliasing`)
**Domain:** Electromagnetics / Antennas
**Definition:** Additional main-beam-strength lobes when element spacing > λ/(1+|sinθ_0|).
**Atom or composite:** Atom — spatial-domain aliasing.
**Cost model:** Free — geometric.
**Real wall?** Yes — Nyquist-like sampling theorem in space.
**Cross-domain wiring:** signal-processing-rf (spatial aliasing), statistics-probability (sampling theory).
**Notes:** Balanis §6.

### dolph-chebyshev-weighting (cross-domain alias: `equiripple-sll`, `min-bw-given-sll`, `chebyshev-array`)
**Domain:** Electromagnetics / Antennas
**Definition:** Element amplitudes from Chebyshev polynomial; optimal min HPBW for given SLL.
**Atom or composite:** Composite — Chebyshev-polynomial coefficient design.
**Cost model:** Closed-form.
**Real wall?** Yes — optimal Chebyshev property.
**Cross-domain wiring:** signal-processing-rf (filter design), control-numerical-opt (Remez).
**Notes:** Dolph 1946.

### taylor-distribution (cross-domain alias: `taylor-n-bar`, `near-equiripple`, `controlled-sll`)
**Domain:** Electromagnetics / Antennas
**Definition:** Continuous aperture distribution giving Chebyshev-like SLL but realizable amplitude taper.
**Atom or composite:** Composite — modified-Chebyshev.
**Cost model:** Closed-form coefficients.
**Real wall?** Yes — near-optimal trade.
**Cross-domain wiring:** signal-processing-rf (low-sidelobe arrays), retrieval-search (windowing).
**Notes:** Taylor 1955.

### bayliss-distribution (cross-domain alias: `monopulse-bayliss`, `difference-pattern`, `radar-tracking`)
**Domain:** Electromagnetics / Antennas
**Definition:** Aperture distribution producing monopulse difference pattern with controlled SLL.
**Atom or composite:** Composite — odd-symmetric Taylor analog.
**Cost model:** Closed-form.
**Real wall?** Yes — radar tracking sensitivity limit.
**Cross-domain wiring:** signal-processing-rf (monopulse radar).
**Notes:** Bayliss 1968.

### schelkunoff-polynomial-method (cross-domain alias: `array-polynomial`, `z-plane-roots`, `null-placement`)
**Domain:** Electromagnetics / Antennas
**Definition:** Array factor as polynomial in z = exp(jψ); roots = pattern nulls.
**Atom or composite:** Composite — z-transform of array weights.
**Cost model:** Polynomial root manipulation.
**Real wall?** No — design via root placement.
**Cross-domain wiring:** signal-processing-rf (zero placement), control-numerical-opt (polynomial design).
**Notes:** Schelkunoff 1943.

### mutual-coupling (cross-domain alias: `Z-matrix-coupling`, `element-cross-talk`, `s12-array`)
**Domain:** Electromagnetics / Antennas
**Definition:** Currents on each element induce voltages on others; encoded in Z-matrix.
**Atom or composite:** Composite — full Z-matrix per array.
**Cost model:** O(N²) MoM evaluation; per-frequency.
**Real wall?** Yes — physical; affects active impedance.
**Cross-domain wiring:** linear-algebra-matrix (impedance matrix), signal-processing-rf (calibration).
**Notes:** Hannan 1964 "Element-Gain Paradox".

### active-impedance (cross-domain alias: `scan-impedance`, `port-Z-vs-scan`, `embedded-element`)
**Domain:** Electromagnetics / Antennas
**Definition:** Z_n^active = Σ_m Z_nm·(I_m/I_n); depends on scan angle.
**Atom or composite:** Composite — Z-matrix collapsed by phase progression.
**Cost model:** Single computation given Z-matrix.
**Real wall?** Yes — scan-blindness arises when Re Z → 0.
**Cross-domain wiring:** signal-processing-rf (scan-impedance match).
**Notes:** Hannan & Balfour 1965.

### array-calibration (cross-domain alias: `phase-amp-cal`, `embedded-pattern-cal`, `mcm-cal`)
**Domain:** Electromagnetics / Antennas
**Definition:** Per-element phase/amplitude correction to match design weights.
**Atom or composite:** Composite — per-element offset estimation.
**Cost model:** Anechoic chamber or built-in injection; O(N).
**Real wall?** Yes — manufacturing variation requires it.
**Cross-domain wiring:** signal-processing-rf (RF calibration), ml-training (data-driven cal).
**Notes:** Sorace 2001.

---

## Numerical Methods (Low Frequency / Full-Wave)

### fdtd-yee-cell (cross-domain alias: `staggered-grid-em`, `leapfrog-em`, `yee-1966`)
**Domain:** Electromagnetics / Antennas
**Definition:** E and H fields offset by ½ cell and ½ time step; central differences give 2nd-order accuracy.
**Atom or composite:** Composite — staggered-grid finite-difference scheme.
**Cost model:** O(N_cells · N_steps); per-cell flop count small.
**Real wall?** Yes — Courant limit cΔt ≤ Δx/√d.
**Cross-domain wiring:** physics-diffusion (staggered grids), control-numerical-opt (CFL stability), graphics-rendering-lod (volumetric).
**Notes:** Yee 1966.

### courant-friedrichs-lewy-em (cross-domain alias: `cfl-em`, `dt-bound-yee`, `stability-em`)
**Domain:** Electromagnetics / Antennas
**Definition:** cΔt ≤ 1/√((1/Δx)² + (1/Δy)² + (1/Δz)²). Necessary for FDTD stability.
**Atom or composite:** Atom — von Neumann stability condition.
**Cost model:** Free — bound on time step.
**Real wall?** Yes — explicit-scheme stability.
**Cross-domain wiring:** physics-diffusion (CFL), control-numerical-opt (explicit-scheme limit).
**Notes:** CFL 1928 paper; Taflove & Hagness Ch. 4.

### pml-perfectly-matched-layer (cross-domain alias: `berenger-pml`, `cpml`, `absorbing-layer`)
**Domain:** Electromagnetics / Antennas
**Definition:** Anisotropic-loss boundary that matches free-space impedance at all angles; absorbs outgoing waves.
**Atom or composite:** Composite — coordinate-stretched Maxwell.
**Cost model:** Adds ~10 cells per boundary; O(N) overhead.
**Real wall?** No — designed approximation; reflection ~−80 dB.
**Cross-domain wiring:** physics-diffusion (absorbing BC), control-numerical-opt (DtN map approximation).
**Notes:** Bérenger 1994; CPML by Roden & Gedney.

### mur-absorbing-bc (cross-domain alias: `mur-1st-order`, `one-way-wave-eq`, `engquist-majda`)
**Domain:** Electromagnetics / Antennas
**Definition:** First/second-order absorbing BC based on one-way wave equation.
**Atom or composite:** Composite — local difference operator at boundary.
**Cost model:** Cheap; reflection ~−40 dB.
**Real wall?** No — approximation; PML usually better.
**Cross-domain wiring:** physics-diffusion (open BCs), control-numerical-opt (radiation conditions).
**Notes:** Mur 1981.

### dispersive-fdtd-ade (cross-domain alias: `ade-fdtd`, `aux-diff-eq`, `lorentz-fdtd`)
**Domain:** Electromagnetics / Antennas
**Definition:** Add auxiliary ODE for polarization current to model frequency-dependent ε.
**Atom or composite:** Composite — Yee + ODE per pole.
**Cost model:** Extra state per pole per cell.
**Real wall?** No — fits arbitrary KK-consistent ε.
**Cross-domain wiring:** photonics-optics (metal in FDTD), control-numerical-opt (state-space).
**Notes:** Taflove Ch. 9.

### adi-fdtd (cross-domain alias: `unconditionally-stable-fdtd`, `alternating-direction`, `implicit-em`)
**Domain:** Electromagnetics / Antennas
**Definition:** Alternating-direction-implicit FDTD; unconditionally stable, no CFL.
**Atom or composite:** Composite — implicit time stepping in alternating axes.
**Cost model:** Tridiagonal solves; larger dt but per-step cost up.
**Real wall?** Yes — accuracy degrades at high dt.
**Cross-domain wiring:** control-numerical-opt (implicit schemes), linear-algebra-matrix (tridiagonal solvers).
**Notes:** Namiki 1999.

### subgridding-fdtd (cross-domain alias: `local-mesh-refinement`, `nested-grid-em`, `mr-fdtd`)
**Domain:** Electromagnetics / Antennas
**Definition:** Embedded fine grid regions inside coarse-grid FDTD.
**Atom or composite:** Composite — multi-resolution Yee.
**Cost model:** Saves cells where structure is small.
**Real wall?** Yes — interface reflections and instability if mishandled.
**Cross-domain wiring:** control-numerical-opt (AMR), graphics-rendering-lod (LOD).
**Notes:** Chevalier 1997.

### dgtd-discontinuous-galerkin (cross-domain alias: `dg-em`, `unstructured-em`, `hp-em`)
**Domain:** Electromagnetics / Antennas
**Definition:** Discontinuous Galerkin on unstructured meshes for time-domain Maxwell.
**Atom or composite:** Composite — FE basis on each element with flux at interfaces.
**Cost model:** Higher order = fewer elements; explicit time step.
**Real wall?** Yes — element-level CFL.
**Cross-domain wiring:** control-numerical-opt (DG schemes), computational-geometry (unstructured meshes).
**Notes:** Hesthaven & Warburton.

### method-of-moments-em (cross-domain alias: `mom-em`, `efie-pec`, `harrington-1968`)
**Domain:** Electromagnetics / Antennas
**Definition:** Express currents in basis functions; project Maxwell into matrix equation Z·I = V.
**Atom or composite:** Composite — Galerkin projection of EFIE/MFIE.
**Cost model:** O(N²) memory, O(N³) solve naive; MLFMM reduces.
**Real wall?** Yes — basis completeness.
**Cross-domain wiring:** linear-algebra-matrix (dense systems), retrieval-search (basis functions).
**Notes:** Harrington 1968.

### rwg-basis-functions (cross-domain alias: `rao-wilton-glisson`, `div-conforming-basis`, `edge-basis-surface`)
**Domain:** Electromagnetics / Antennas
**Definition:** Triangle-pair basis with continuous normal current across shared edge.
**Atom or composite:** Composite — div-conforming surface basis.
**Cost model:** Standard; ~6N triangles → ~N edges.
**Real wall?** Yes — div continuity required to avoid spurious modes.
**Cross-domain wiring:** computational-geometry (mesh), linear-algebra-matrix (sparsity pattern).
**Notes:** Rao, Wilton, Glisson 1982.

### efie-electric-field-integral-eq (cross-domain alias: `efie`, `t-pec-equation`, `tangential-E-zero`)
**Domain:** Electromagnetics / Antennas
**Definition:** Sets tangential E from current-induced + incident = 0 on PEC.
**Atom or composite:** Composite — boundary integral equation.
**Cost model:** Hypersingular kernel; preconditioning often needed.
**Real wall?** Yes — interior-resonance issue at closed bodies.
**Cross-domain wiring:** retrieval-search (kernel integral eq).
**Notes:** Harrington Ch. 4.

### mfie-magnetic-field-integral-eq (cross-domain alias: `mfie`, `tangential-H-jump`, `closed-body-mfie`)
**Domain:** Electromagnetics / Antennas
**Definition:** Uses tangential H jump = J_s on closed PEC.
**Atom or composite:** Composite — boundary integral equation.
**Cost model:** Better-conditioned than EFIE; closed bodies only.
**Real wall?** Yes — same interior-resonance.
**Cross-domain wiring:** linear-algebra-matrix (matrix conditioning).
**Notes:** Harrington Ch. 4.

### cfie-combined-field-integral-eq (cross-domain alias: `cfie`, `efie-plus-mfie`, `resonance-free-mom`)
**Domain:** Electromagnetics / Antennas
**Definition:** α·EFIE + (1−α)·η·MFIE; avoids interior-resonance for closed bodies.
**Atom or composite:** Composite — linear combination of EFIE/MFIE.
**Cost model:** Same as either; better conditioning.
**Real wall?** No — formulation choice; α ≈ 0.5 typical.
**Cross-domain wiring:** linear-algebra-matrix (regularization).
**Notes:** Mautz & Harrington 1979.

### pmchwt-formulation (cross-domain alias: `pmchwt`, `dielectric-body-bie`, `coupled-eq-set`)
**Domain:** Electromagnetics / Antennas
**Definition:** Coupled EFIE/MFIE for penetrable bodies with both electric and magnetic surface currents.
**Atom or composite:** Composite — system of integral equations.
**Cost model:** 2× DoFs vs PEC.
**Real wall?** Yes — accurate for dielectric scattering.
**Cross-domain wiring:** linear-algebra-matrix (block systems).
**Notes:** Poggio, Miller, Chang, Harrington, Wu, Tsai.

### mlfmm (cross-domain alias: `multilevel-fast-multipole`, `n-log-n-mom`, `tree-acceleration-em`)
**Domain:** Electromagnetics / Antennas
**Definition:** Tree-code acceleration of MoM matrix-vector products via translation operators in k-space.
**Atom or composite:** Composite — addition theorem + multilevel tree.
**Cost model:** O(N log N) per matrix-vector; iterative solve overall O(N log N).
**Real wall?** Yes — accuracy from truncation order L.
**Cross-domain wiring:** linear-algebra-matrix (fast matvec), retrieval-search (tree-codes), computational-geometry (octrees).
**Notes:** Song, Lu, Chew 1997.

### adaptive-integral-method (cross-domain alias: `aim`, `fft-mom`, `grid-projection-em`)
**Domain:** Electromagnetics / Antennas
**Definition:** Project sources onto uniform grid, do FFT convolution, project back.
**Atom or composite:** Composite — FFT-accelerated MoM.
**Cost model:** O(N log N) memory and matvec.
**Real wall?** Yes — projection error.
**Cross-domain wiring:** signal-processing-rf (FFT), linear-algebra-matrix (fast solvers).
**Notes:** Bleszynski et al. 1996.

### fem-em (cross-domain alias: `finite-element-em`, `volumetric-em`, `tetra-elements`)
**Domain:** Electromagnetics / Antennas
**Definition:** Variational form of Maxwell on tetrahedral mesh with edge elements.
**Atom or composite:** Composite — volumetric weighted-residual method.
**Cost model:** Sparse N×N system; iterative or direct.
**Real wall?** Yes — open-region requires PML/IE termination.
**Cross-domain wiring:** control-numerical-opt (FEM), computational-geometry (tet meshing).
**Notes:** Jin "Finite Element Method in Electromagnetics".

### nedelec-edge-elements (cross-domain alias: `curl-conforming`, `whitney-1-forms`, `vector-fem-basis`)
**Domain:** Electromagnetics / Antennas
**Definition:** Vector basis with tangential continuity (curl-conforming) on tetrahedra.
**Atom or composite:** Composite — div+curl-conforming basis.
**Cost model:** Higher DoFs but no spurious modes.
**Real wall?** Yes — required for vector-valued Maxwell.
**Cross-domain wiring:** type-theory-programming-languages (differential forms), computational-geometry (Whitney forms).
**Notes:** Nédélec 1980.

### tlm-method (cross-domain alias: `transmission-line-matrix`, `johns-1971`, `network-em`)
**Domain:** Electromagnetics / Antennas
**Definition:** Maxwell mapped onto network of transmission-line scatterers; field = voltage/current at nodes.
**Atom or composite:** Composite — discrete-network solver of Maxwell.
**Cost model:** Comparable to FDTD; explicit time stepping.
**Real wall?** Yes — scatterer locality.
**Cross-domain wiring:** networking (lumped networks), signal-processing-rf (filter banks).
**Notes:** P. B. Johns 1971.

### mmp-multiple-multipole (cross-domain alias: `mmp-method`, `expansion-around-sources`, `hafner-mmp`)
**Domain:** Electromagnetics / Antennas
**Definition:** Expand fields in multipole series around expansion points; match BCs by collocation.
**Atom or composite:** Composite — meshless multipole expansion.
**Cost model:** Smooth structures benefit; few thousand DoFs.
**Real wall?** Yes — multipole completeness in source-free regions.
**Cross-domain wiring:** linear-algebra-matrix (least-squares fit).
**Notes:** Hafner 1980s; MaX-1 code.

### bem-pec (cross-domain alias: `boundary-element-em`, `surface-mesh-mom`, `sie`)
**Domain:** Electromagnetics / Antennas
**Definition:** Boundary Element Method = MoM with surface integral equation on PEC.
**Atom or composite:** Composite — restated MoM.
**Cost model:** Same as MoM.
**Real wall?** Yes — only models surface unknowns.
**Cross-domain wiring:** computational-geometry (surface meshing).
**Notes:** Equivalent terminology in CEM.

### hybrid-fem-bem (cross-domain alias: `fem-mom-hybrid`, `volumetric-plus-boundary`, `truncated-fem`)
**Domain:** Electromagnetics / Antennas
**Definition:** FEM in inhomogeneous interior + BEM on truncation boundary for radiation.
**Atom or composite:** Composite — volume+surface coupling.
**Cost model:** FEM-sparse + dense-BEM block; MLFMM helps.
**Real wall?** Yes — open-region accuracy.
**Cross-domain wiring:** control-numerical-opt (DD methods), linear-algebra-matrix (block systems).
**Notes:** Yuan, Lynch & Strohbehn 1990.

### pinns-for-maxwell (cross-domain alias: `physics-informed-nn-em`, `nn-maxwell-solver`, `differentiable-em`)
**Domain:** Electromagnetics / Antennas
**Definition:** Neural network u_θ(r,t) trained to satisfy Maxwell residual + BCs.
**Atom or composite:** Composite — gradient-descent PDE residual minimization.
**Cost model:** GPU-hours per training; cheap inference.
**Real wall?** No — accuracy limited by NN expressivity & training.
**Cross-domain wiring:** ml-training (PINN), control-numerical-opt (variational solvers).
**Notes:** Raissi 2019; applications growing.

### fourier-neural-operator-em (cross-domain alias: `fno-em`, `spectral-neural-op`, `data-driven-em`)
**Domain:** Electromagnetics / Antennas
**Definition:** Learn mapping ε(r) → E(r) via Fourier-layer neural network.
**Atom or composite:** Composite — neural operator in spectral domain.
**Cost model:** Training expensive; inference very fast.
**Real wall?** No — generalization to unseen geometries limited.
**Cross-domain wiring:** ml-training (operator learning), signal-processing-rf (spectral methods).
**Notes:** Li et al. 2020.

### deeponet-em (cross-domain alias: `deeponet-maxwell`, `branch-trunk-em`, `operator-net`)
**Domain:** Electromagnetics / Antennas
**Definition:** Branch network encodes ε; trunk encodes (r); output = E(r).
**Atom or composite:** Composite — universal-approximation operator NN.
**Cost model:** Training expensive.
**Real wall?** No — universal-approximation theorem applies.
**Cross-domain wiring:** ml-training (operator nets), retrieval-search (functional learning).
**Notes:** Lu et al. 2021.

### gpu-fdtd (cross-domain alias: `cuda-fdtd`, `gpu-maxwell`, `parallel-yee`)
**Domain:** Electromagnetics / Antennas
**Definition:** Yee updates mapped to GPU kernels; 10–100× speedup vs CPU.
**Atom or composite:** Composite — Yee + CUDA.
**Cost model:** Memory-bandwidth bound; 100s of GFLOPs/s.
**Real wall?** Yes — bandwidth & VRAM size.
**Cross-domain wiring:** ml-training (GPU compute), graphics-rendering-lod (volumetric kernels).
**Notes:** Inman & Elsherbeni 2007.

---

## High-Frequency Asymptotic Methods

### physical-optics-po (cross-domain alias: `po-currents`, `2n-cross-H-inc`, `kirchhoff-approx`)
**Domain:** Electromagnetics / Antennas
**Definition:** J_s ≈ 2n̂×H_inc on lit surface, 0 in shadow. Radiated field by integration.
**Atom or composite:** Composite — high-frequency approximation to MoM.
**Cost model:** O(N_surf) integration per direction; no matrix.
**Real wall?** No — accurate when D ≫ λ and edge contributions small.
**Cross-domain wiring:** graphics-rendering-lod (ray-cast lighting), signal-processing-rf (RCS prediction).
**Notes:** Balanis Ch. 11.

### geometrical-theory-of-diffraction (cross-domain alias: `gtd`, `keller-1962`, `edge-diffraction`)
**Domain:** Electromagnetics / Antennas
**Definition:** Adds diffracted rays to GO at edges/tips/curved surfaces with diffraction coefficients D.
**Atom or composite:** Composite — GO + edge ray correction.
**Cost model:** Ray-traceable; per-edge coefficient lookup.
**Real wall?** Yes — singular at shadow boundaries; UTD fixes.
**Cross-domain wiring:** graphics-rendering-lod (silhouette shaders), computational-geometry (visibility).
**Notes:** Keller 1962.

### uniform-theory-of-diffraction (cross-domain alias: `utd`, `kouyoumjian-pathak`, `fresnel-transition`)
**Domain:** Electromagnetics / Antennas
**Definition:** UTD diffraction coefficients use Fresnel functions to remove GTD singularities.
**Atom or composite:** Composite — uniform asymptotic completion.
**Cost model:** Adds Fresnel evaluation per edge ray.
**Real wall?** No — uniform across boundaries.
**Cross-domain wiring:** signal-processing-rf (propagation), graphics-rendering-lod (rendering).
**Notes:** Kouyoumjian & Pathak 1974.

### physical-theory-of-diffraction (cross-domain alias: `ptd`, `ufimtsev-1962`, `non-uniform-currents`)
**Domain:** Electromagnetics / Antennas
**Definition:** Adds non-uniform edge currents to PO; recovers GTD high-frequency limit.
**Atom or composite:** Composite — PO + edge-correction currents.
**Cost model:** Edge integration added to PO.
**Real wall?** Yes — accuracy near edges.
**Cross-domain wiring:** signal-processing-rf (RCS), graphics-rendering-lod (edge shaders).
**Notes:** Ufimtsev 1962 (key to F-117 design).

### shooting-and-bouncing-rays (cross-domain alias: `sbr`, `ray-traced-em`, `multi-bounce`)
**Domain:** Electromagnetics / Antennas
**Definition:** Shoot dense ray bundle from source, follow reflections + diffractions, sum field contributions.
**Atom or composite:** Composite — GO + PO + UTD on rays.
**Cost model:** O(N_rays · bounces); GPU-friendly.
**Real wall?** Yes — ray density and aliasing.
**Cross-domain wiring:** graphics-rendering-lod (path tracing), signal-processing-rf (urban channel modeling).
**Notes:** Ling, Chou, Lee 1989.

### geometrical-optics (cross-domain alias: `go`, `ray-optics-em`, `lambda-zero-limit`)
**Domain:** Electromagnetics / Antennas
**Definition:** Rays travel along eikonal-equation paths; amplitude follows energy-conservation in tube.
**Atom or composite:** Atom — λ→0 limit.
**Cost model:** Closed-form for canonical reflectors.
**Real wall?** No — breaks down at caustics, edges.
**Cross-domain wiring:** graphics-rendering-lod (ray tracing), photonics-optics (lens design).
**Notes:** Hamilton.

---

## Propagation Models

### free-space-path-loss (cross-domain alias: `fspl`, `20-log-4pi-r-lambda`, `inverse-square`)
**Domain:** Electromagnetics / Antennas
**Definition:** L_fs = (4πR/λ)² ; in dB: 20 log(4πR/λ).
**Atom or composite:** Atom — inverse-square law in dB.
**Cost model:** Trivial.
**Real wall?** Yes — geometric spreading.
**Cross-domain wiring:** networking (link budget), information-theory-coding (range-vs-rate).
**Notes:** Friis 1946.

### two-ray-ground-reflection (cross-domain alias: `2-ray-model`, `direct-plus-ground`, `R-to-the-4`)
**Domain:** Electromagnetics / Antennas
**Definition:** Direct + ground-reflected ray; far-distance L ∝ R⁴ instead of R².
**Atom or composite:** Composite — two-ray interference.
**Cost model:** Closed-form for flat earth.
**Real wall?** Yes for grazing geometry over flat ground.
**Cross-domain wiring:** networking (cellular planning), signal-processing-rf (multipath).
**Notes:** Rappaport "Wireless Communications".

### knife-edge-diffraction (cross-domain alias: `fresnel-knife-edge`, `single-edge-loss`, `fresnel-integral`)
**Domain:** Electromagnetics / Antennas
**Definition:** Loss behind sharp obstacle from Fresnel integral; parametrized by ν.
**Atom or composite:** Composite — Kirchhoff diffraction over half-plane.
**Cost model:** Fresnel integral lookup.
**Real wall?** Yes — Fresnel zone geometry.
**Cross-domain wiring:** signal-processing-rf (terrain shadowing), networking (microwave LOS).
**Notes:** ITU-R P.526.

### bullington-method (cross-domain alias: `equivalent-knife-edge`, `single-replacement`, `multi-edge-merge`)
**Domain:** Electromagnetics / Antennas
**Definition:** Replace multiple obstacles with single equivalent knife-edge through ray-grazing intersection.
**Atom or composite:** Composite — simplification.
**Cost model:** Geometric construction.
**Real wall?** No — underestimates loss.
**Cross-domain wiring:** networking (link planning).
**Notes:** Bullington 1947.

### epstein-peterson-method (cross-domain alias: `sequential-knife-edge`, `multi-obstacle-loss`, `pairwise-edges`)
**Domain:** Electromagnetics / Antennas
**Definition:** Multi-edge loss as sum of pairwise knife-edge losses with intermediate Tx/Rx pairs.
**Atom or composite:** Composite — pairwise composition.
**Cost model:** O(N_edges).
**Real wall?** No — simpler than Vogler's exact solution.
**Cross-domain wiring:** networking (link planning).
**Notes:** Epstein & Peterson 1953.

### deygout-method (cross-domain alias: `dominant-edge`, `recursive-knife-edge`, `main-obstacle`)
**Domain:** Electromagnetics / Antennas
**Definition:** Identify dominant edge, then recurse on left/right segments.
**Atom or composite:** Composite — recursive algorithm.
**Cost model:** O(N log N).
**Real wall?** No — overestimates for many edges; ITU-R P.526 corrects.
**Cross-domain wiring:** networking (path-loss prediction).
**Notes:** Deygout 1966.

### itu-r-p526-diffraction (cross-domain alias: `p526`, `mixed-path-diffraction`, `itu-terrain`)
**Domain:** Electromagnetics / Antennas
**Definition:** ITU standard combining knife-edge, cylinder, smooth-earth diffraction.
**Atom or composite:** Composite — switching among models per regime.
**Cost model:** Tabulated.
**Real wall?** No — empirical/asymptotic blend.
**Cross-domain wiring:** networking (terrestrial planning).
**Notes:** ITU-R Rec. P.526-15.

### longley-rice-itm (cross-domain alias: `irregular-terrain-model`, `itm`, `longley-rice`)
**Domain:** Electromagnetics / Antennas
**Definition:** Statistical path-loss model over irregular terrain 20 MHz – 20 GHz; quantile predictions.
**Atom or composite:** Composite — diffraction + tropospheric scatter blend.
**Cost model:** Available as code; widely used.
**Real wall?** No — statistical; ±10 dB typical.
**Cross-domain wiring:** networking (broadcast coverage), statistics-probability (variability percentiles).
**Notes:** Longley & Rice 1968 NTIA report.

### okumura-hata-model (cross-domain alias: `hata`, `urban-cellular-pl`, `vhf-uhf-pl`)
**Domain:** Electromagnetics / Antennas
**Definition:** Empirical urban path-loss formula 150 MHz – 1500 MHz; correction factors for suburban/rural.
**Atom or composite:** Composite — empirical fit.
**Cost model:** Trivial.
**Real wall?** No — empirical; ±5–10 dB error.
**Cross-domain wiring:** networking (cellular planning).
**Notes:** Hata 1980, based on Okumura 1968.

### cost-231-hata (cross-domain alias: `cost231`, `pcs-extension`, `1500-2000-mhz`)
**Domain:** Electromagnetics / Antennas
**Definition:** Hata extension to 2 GHz; used for PCS/UMTS planning.
**Atom or composite:** Composite — Hata refit.
**Cost model:** Trivial.
**Real wall?** No — empirical.
**Cross-domain wiring:** networking (3G planning).
**Notes:** COST 231 report 1999.

### walfisch-ikegami (cross-domain alias: `cost231-wi`, `building-rooftop-diffraction`, `street-canyon`)
**Domain:** Electromagnetics / Antennas
**Definition:** Adds rooftop-to-street diffraction + multiscreen for urban microcells.
**Atom or composite:** Composite — Walfisch-Bertoni + Ikegami.
**Cost model:** Closed-form expressions.
**Real wall?** No — semi-empirical.
**Cross-domain wiring:** networking (urban microcell planning).
**Notes:** COST 231 Final Report.

### 3gpp-channel-models (cross-domain alias: `tr-38901`, `umi-uma-rma`, `5g-channel`)
**Domain:** Electromagnetics / Antennas
**Definition:** Tapped-delay-line + spatial models for UMi, UMa, RMa, InH; up to 100 GHz.
**Atom or composite:** Composite — statistical multipath models with cluster structure.
**Cost model:** Random realization per drop.
**Real wall?** No — fitted to measurement.
**Cross-domain wiring:** networking (5G/6G), signal-processing-rf (MIMO channels).
**Notes:** 3GPP TR 38.901.

### ieee-802-11-channel-models (cross-domain alias: `tgn-tgac-models`, `wifi-channel`, `indoor-mimo`)
**Domain:** Electromagnetics / Antennas
**Definition:** Models A-F for WLAN indoor multipath with cluster delay spreads.
**Atom or composite:** Composite — Saleh-Valenzuela cluster model fit.
**Cost model:** Tap generation per drop.
**Real wall?** No — fitted to measurement.
**Cross-domain wiring:** networking (Wi-Fi MIMO), signal-processing-rf (channel emulation).
**Notes:** IEEE 802.11n/ac models.

### saleh-valenzuela-model (cross-domain alias: `sv-model`, `cluster-tap-model`, `indoor-multipath`)
**Domain:** Electromagnetics / Antennas
**Definition:** Multipath as clusters of rays with double-exponential decay envelope.
**Atom or composite:** Composite — Poisson cluster + intra-cluster rays.
**Cost model:** Stochastic generation.
**Real wall?** No — model assumption.
**Cross-domain wiring:** statistics-probability (cluster processes), signal-processing-rf (channel models).
**Notes:** Saleh & Valenzuela 1987.

### ray-tracing-urban-canyon (cross-domain alias: `urban-ray-trace`, `building-database-rt`, `site-specific`)
**Domain:** Electromagnetics / Antennas
**Definition:** Site-specific propagation using building/terrain database, SBR/GO/UTD.
**Atom or composite:** Composite — large-scale SBR.
**Cost model:** Hours per scenario; GPU accelerates.
**Real wall?** Yes — accuracy bounded by database fidelity.
**Cross-domain wiring:** computational-geometry (BVH), graphics-rendering-lod (rendering pipeline).
**Notes:** Wireless InSite, Remcom, etc.

### tropospheric-scintillation (cross-domain alias: `clear-air-scintillation`, `refractive-index-fluctuation`, `itu-p618`)
**Domain:** Electromagnetics / Antennas
**Definition:** Rapid amplitude fluctuations from atmospheric turbulence; severe at low elevation.
**Atom or composite:** Composite — Rytov/log-normal model.
**Cost model:** Statistical CDF generation.
**Real wall?** Yes — Kolmogorov turbulence physics.
**Cross-domain wiring:** statistics-probability (log-normal stats), photonics-optics (atmospheric optics).
**Notes:** ITU-R P.618.

### multipath-fading-rayleigh (cross-domain alias: `rayleigh-fading`, `nlos-fading`, `complex-gaussian-channel`)
**Domain:** Electromagnetics / Antennas
**Definition:** Sum of many random-phase multipath → complex Gaussian → |h| Rayleigh.
**Atom or composite:** Composite — CLT applied to multipath.
**Cost model:** Trivial random sample.
**Real wall?** No — model assumption.
**Cross-domain wiring:** statistics-probability (Rayleigh distribution), signal-processing-rf (fading).
**Notes:** Lord Rayleigh 1880; Rappaport.

### multipath-fading-rician (cross-domain alias: `rician-k-factor`, `los-plus-scatter`, `k-equals-line-of-sight-power`)
**Domain:** Electromagnetics / Antennas
**Definition:** Strong LOS + scattered → Rician envelope; K = LOS/scatter power.
**Atom or composite:** Composite — non-central χ result.
**Cost model:** Trivial sample.
**Real wall?** No — model with K parameter.
**Cross-domain wiring:** statistics-probability (Rician), signal-processing-rf (LOS channels).
**Notes:** Rice 1944.

### nakagami-m-fading (cross-domain alias: `nakagami-m`, `gamma-envelope`, `m-parameter`)
**Domain:** Electromagnetics / Antennas
**Definition:** Generalized fading distribution with shape parameter m; m=1 → Rayleigh.
**Atom or composite:** Composite — flexible envelope model.
**Cost model:** Trivial sample.
**Real wall?** No — empirical fit.
**Cross-domain wiring:** statistics-probability (Gamma-distributed power), signal-processing-rf (channel modeling).
**Notes:** Nakagami 1960.

### log-normal-shadowing (cross-domain alias: `lognormal-shadow`, `sigma-db-fluctuation`, `large-scale-fading`)
**Domain:** Electromagnetics / Antennas
**Definition:** Mean path-loss × log-normal random variable; σ ≈ 4–10 dB.
**Atom or composite:** Composite — multiplicative log-normal model.
**Cost model:** Trivial sample.
**Real wall?** No — empirical.
**Cross-domain wiring:** networking (outage probabilities), statistics-probability (log-normal).
**Notes:** Rappaport Ch. 4.

---

## Scattering & Radar Cross Section

### radar-cross-section (cross-domain alias: `rcs`, `sigma-rcs`, `4-pi-r2-Es-over-Ei`)
**Domain:** Electromagnetics / Antennas
**Definition:** σ = lim_{R→∞} 4πR² |E_s|²/|E_i|² — equivalent isotropic-scatterer area.
**Atom or composite:** Atom — far-field scattering metric.
**Cost model:** Per direction + frequency + polarization.
**Real wall?** Yes — observable cross section.
**Cross-domain wiring:** signal-processing-rf (radar detection), retrieval-search (target identification).
**Notes:** Skolnik.

### monostatic-vs-bistatic-rcs (cross-domain alias: `back-scatter-rcs`, `forward-scatter`, `bistatic-angle`)
**Domain:** Electromagnetics / Antennas
**Definition:** Monostatic: TX = RX direction; bistatic: separate. Bistatic σ_b can be much larger near forward.
**Atom or composite:** Composite — RCS specialized by geometry.
**Cost model:** Same as RCS per direction pair.
**Real wall?** Yes — different scattering mechanisms.
**Cross-domain wiring:** networking (passive bistatic radar), signal-processing-rf (multistatic).
**Notes:** Knott "Radar Cross Section".

### rcs-shaping (cross-domain alias: `stealth-shaping`, `faceting`, `redirect-scatter`)
**Domain:** Electromagnetics / Antennas
**Definition:** Shape body so specular reflections avoid threat directions; planar facets, sharp edges.
**Atom or composite:** Composite — RCS reduction via GO redirection.
**Cost model:** Design tradeoff vs aerodynamics.
**Real wall?** Yes — physical reflection law.
**Cross-domain wiring:** computational-geometry (shape optimization), graphics-rendering-lod (silhouette).
**Notes:** F-117 (Ufimtsev PTD); F-22.

### rcs-reduction-materials (cross-domain alias: `radar-absorbing-material`, `ram`, `salisbury-screen`)
**Domain:** Electromagnetics / Antennas
**Definition:** Lossy or matched-impedance coatings absorb incident energy.
**Atom or composite:** Composite — material + geometry.
**Cost model:** Mass/cost vs absorption tradeoff.
**Real wall?** Yes — Rozanov bandwidth-thickness bound.
**Cross-domain wiring:** photonics-optics (anti-reflective coatings), signal-processing-rf (anechoic chambers).
**Notes:** Salisbury 1952; Jaumann.

### mie-scattering (cross-domain alias: `lorenz-mie`, `sphere-scattering`, `size-parameter-x`)
**Domain:** Electromagnetics / Antennas
**Definition:** Exact series solution for plane-wave scattering by sphere; series in Bessel & Legendre functions.
**Atom or composite:** Composite — separation of variables in sphere.
**Cost model:** N_max ≈ x + 4x^(1/3) terms.
**Real wall?** Yes — exact full-wave for sphere.
**Cross-domain wiring:** photonics-optics (atmospheric scattering), retrieval-search (particle sensing).
**Notes:** Mie 1908; Lorenz 1890.

### rayleigh-scattering (cross-domain alias: `small-particle-scatter`, `x-much-less-1`, `1-over-lambda4`)
**Domain:** Electromagnetics / Antennas
**Definition:** σ ∝ k⁴ a⁶; valid when ka ≪ 1; blue-sky law.
**Atom or composite:** Atom — Mie low-x limit.
**Cost model:** Trivial.
**Real wall?** Yes — dipole-radiation limit.
**Cross-domain wiring:** photonics-optics (sky color), signal-processing-rf (clutter from tiny scatterers).
**Notes:** Rayleigh 1871.

### born-approximation-em (cross-domain alias: `weak-scatter-born`, `first-order-volume-integral`, `linearized-inverse`)
**Domain:** Electromagnetics / Antennas
**Definition:** First-order solution where total field replaced by incident inside scatterer.
**Atom or composite:** Composite — linear approximation to scattering integral.
**Cost model:** One Fourier transform — fast inverse method.
**Real wall?** Yes when contrast small.
**Cross-domain wiring:** retrieval-search (inverse problems), biology-bioinformatics (medical imaging).
**Notes:** Born 1926.

### rytov-approximation (cross-domain alias: `rytov-phase`, `log-amplitude-rytov`, `weak-phase-scatterer`)
**Domain:** Electromagnetics / Antennas
**Definition:** Linearizes log of field rather than field itself; better for weakly-refractive media.
**Atom or composite:** Composite — phase-linearized scattering.
**Cost model:** Similar to Born; complementary regime.
**Real wall?** Yes — weak-perturbation assumption.
**Cross-domain wiring:** photonics-optics (turbulence), biology-bioinformatics (diffraction tomography).
**Notes:** Rytov 1937.

### inverse-scattering-problem (cross-domain alias: `is-em`, `target-reconstruction`, `Es-to-eps-of-r`)
**Domain:** Electromagnetics / Antennas
**Definition:** Recover ε(r) from scattered-field measurements; nonlinear, ill-posed.
**Atom or composite:** Composite — inverse problem.
**Cost model:** Iterative; regularization needed.
**Real wall?** Yes — ill-posedness fundamental.
**Cross-domain wiring:** retrieval-search (inverse problems), biology-bioinformatics (microwave imaging), ml-training (learned reconstruction).
**Notes:** Colton & Kress.

### target-identification-classification (cross-domain alias: `nctr`, `radar-id`, `feature-based-id`)
**Domain:** Electromagnetics / Antennas
**Definition:** Use RCS, HRR profile, micro-Doppler, polarimetric features for ATR.
**Atom or composite:** Composite — feature extraction + classifier.
**Cost model:** Real-time signal processing + ML.
**Real wall?** Yes — feature separability.
**Cross-domain wiring:** ml-training (classification), signal-processing-rf (HRR), retrieval-search (template matching).
**Notes:** Skolnik Ch. 9.

---

## EMC, Components & Effective-Media

### shielding-effectiveness (cross-domain alias: `se-db`, `e-h-attenuation`, `cage-rejection`)
**Domain:** Electromagnetics / Antennas
**Definition:** SE = 20 log(E_unshielded/E_shielded). Sum of absorption + reflection + multiple-reflection losses.
**Atom or composite:** Composite — Schelkunoff impedance method.
**Cost model:** Closed-form per planar shield.
**Real wall?** Yes — skin-depth + impedance mismatch.
**Cross-domain wiring:** signal-processing-rf (EMI), networking (cable shielding).
**Notes:** Schelkunoff 1938.

### bethe-coupling (cross-domain alias: `small-aperture-coupling`, `polarizability-coupling`, `slot-leakage`)
**Domain:** Electromagnetics / Antennas
**Definition:** Field through small aperture modeled by electric/magnetic dipole with aperture polarizabilities.
**Atom or composite:** Composite — small-aperture quasi-static.
**Cost model:** Closed-form polarizabilities.
**Real wall?** Yes — small-hole limit.
**Cross-domain wiring:** signal-processing-rf (coupler design), networking (shielded enclosures).
**Notes:** Bethe 1944.

### cavity-resonance (cross-domain alias: `enclosure-modes`, `te-tm-cavity`, `box-resonance`)
**Domain:** Electromagnetics / Antennas
**Definition:** Enclosed conducting cavity has discrete resonant modes f_mnp.
**Atom or composite:** Composite — eigenmodes of closed Helmholtz.
**Cost model:** Closed-form for rectangular/cylindrical.
**Real wall?** Yes — discrete spectrum.
**Cross-domain wiring:** signal-processing-rf (filters), quantum-computing (cavity QED).
**Notes:** Pozar Ch. 6.

### common-mode-vs-differential-mode (cross-domain alias: `cm-dm`, `balanced-unbalanced`, `eta-common-current`)
**Domain:** Electromagnetics / Antennas
**Definition:** Differential mode: equal and opposite currents; common mode: in-phase on both conductors.
**Atom or composite:** Composite — modal decomposition of multiconductor lines.
**Cost model:** Trivial matrix transform.
**Real wall?** Yes — CM radiation dominates EMI.
**Cross-domain wiring:** signal-processing-rf (baluns, chokes), networking (cable design).
**Notes:** Paul "Introduction to EMC".

### crosstalk (cross-domain alias: `near-end-far-end-xtalk`, `next-fext`, `coupling-loss`)
**Domain:** Electromagnetics / Antennas
**Definition:** Coupling between parallel traces via mutual L and C → NEXT/FEXT.
**Atom or composite:** Composite — multi-conductor TLM.
**Cost model:** RLGC matrix per line group.
**Real wall?** Yes — geometric coupling.
**Cross-domain wiring:** networking (PCB signal integrity), signal-processing-rf (channel isolation).
**Notes:** Paul Ch. 10.

### return-current-paths (cross-domain alias: `gnd-bounce`, `image-current`, `loop-area`)
**Domain:** Electromagnetics / Antennas
**Definition:** Return current takes path of least impedance — high-f follows signal trace via reference plane.
**Atom or composite:** Atom — Kirchhoff + skin-depth + inductive geometry.
**Cost model:** Determined by reference-plane geometry.
**Real wall?** Yes — physics; broken by gaps in plane.
**Cross-domain wiring:** networking (high-speed PCB), signal-processing-rf (EMI mitigation).
**Notes:** Johnson & Graham "High-Speed Digital Design".

### maxwell-garnett-mixing (cross-domain alias: `mg-effective-medium`, `inclusion-host`, `dilute-mixture`)
**Domain:** Electromagnetics / Antennas
**Definition:** ε_eff for dilute inclusions of ε_i in host ε_h: (ε_eff − ε_h)/(ε_eff + 2ε_h) = f(ε_i − ε_h)/(ε_i + 2ε_h).
**Atom or composite:** Composite — Clausius-Mossotti generalized.
**Cost model:** Closed-form.
**Real wall?** No — dilute limit assumption.
**Cross-domain wiring:** photonics-optics (composites), control-numerical-opt (homogenization).
**Notes:** Maxwell Garnett 1904.

### bruggeman-mixing (cross-domain alias: `symmetric-effective-medium`, `bruggeman-1935`, `volume-fraction-mixing`)
**Domain:** Electromagnetics / Antennas
**Definition:** Self-consistent effective medium: Σ f_i (ε_i − ε_eff)/(ε_i + 2ε_eff) = 0.
**Atom or composite:** Composite — symmetric mixing rule.
**Cost model:** Quadratic solve.
**Real wall?** No — symmetric assumption.
**Cross-domain wiring:** photonics-optics (porous media), biology-bioinformatics (tissue dielectric).
**Notes:** Bruggeman 1935.

---

## Microwave Components & Filters

### combline-filter (cross-domain alias: `combline-bpf`, `capacitive-loaded-resonator`, `cell-base-filter`)
**Domain:** Electromagnetics / Antennas
**Definition:** Coupled λ/4 resonators each loaded by capacitor; compact bandpass.
**Atom or composite:** Composite — coupled-resonator filter.
**Cost model:** Synthesis from low-pass prototype.
**Real wall?** Yes — Q limited by metallic losses.
**Cross-domain wiring:** signal-processing-rf (filter synthesis).
**Notes:** Matthaei, Young, Jones.

### interdigital-filter (cross-domain alias: `interdig-bpf`, `quarter-wave-resonator-bpf`, `printed-bpf`)
**Domain:** Electromagnetics / Antennas
**Definition:** Coupled λ/4 resonators alternating short/open ends; bandpass with wide stopband.
**Atom or composite:** Composite — coupled-resonator BPF.
**Cost model:** Tabulated synthesis.
**Real wall?** Yes — narrow band limited by structure size.
**Cross-domain wiring:** signal-processing-rf (RF filtering).
**Notes:** Matthaei et al.

### hairpin-filter (cross-domain alias: `hairpin-bpf`, `folded-half-wave`, `compact-printed-bpf`)
**Domain:** Electromagnetics / Antennas
**Definition:** λ/2 microstrip resonators folded into U-shape; saves area.
**Atom or composite:** Composite — folded edge-coupled BPF.
**Cost model:** Microstrip-MoM verification.
**Real wall?** Yes — radiation loss.
**Cross-domain wiring:** networking (PCB-mounted filters).
**Notes:** Cristal & Frankel 1972.

### edge-coupled-filter (cross-domain alias: `parallel-coupled-bpf`, `cohn-filter`, `printed-bpf`)
**Domain:** Electromagnetics / Antennas
**Definition:** λ/2 resonators with edge coupling; bandwidth set by coupling gap.
**Atom or composite:** Composite — coupled microstrip pairs.
**Cost model:** Closed-form initial design.
**Real wall?** Yes — minimum gap manufacturable.
**Cross-domain wiring:** signal-processing-rf (filter design).
**Notes:** Cohn 1958.

### branch-line-coupler (cross-domain alias: `90-deg-hybrid`, `quadrature-coupler`, `3-db-hybrid`)
**Domain:** Electromagnetics / Antennas
**Definition:** Four-port 3-dB 90° hybrid using λ/4 sections; outputs in quadrature.
**Atom or composite:** Composite — λ/4 ring coupler.
**Cost model:** Closed-form Z values.
**Real wall?** Yes — narrowband (~10%).
**Cross-domain wiring:** signal-processing-rf (IQ generation), networking (balanced amps).
**Notes:** Pozar §7.

### rat-race-coupler (cross-domain alias: `180-deg-hybrid`, `magic-t-print`, `ring-hybrid`)
**Domain:** Electromagnetics / Antennas
**Definition:** 3λ/2 ring with four ports gives 180° or 0° outputs.
**Atom or composite:** Composite — ring hybrid.
**Cost model:** Closed-form.
**Real wall?** Yes — narrowband.
**Cross-domain wiring:** signal-processing-rf (balanced mixer).
**Notes:** Pozar §7.

### lange-coupler (cross-domain alias: `interdigital-coupler`, `mmic-3-db-coupler`, `lange-1969`)
**Domain:** Electromagnetics / Antennas
**Definition:** Interdigitated coupled lines give 3-dB quadrature coupling over decade BW.
**Atom or composite:** Composite — broadside-coupled multi-line.
**Cost model:** Bond-wire or thin-film process.
**Real wall?** Yes — high frequencies, low Q.
**Cross-domain wiring:** signal-processing-rf (MMIC).
**Notes:** Lange 1969.

### wilkinson-divider (cross-domain alias: `n-way-wilkinson`, `equal-split-isolated`, `100-ohm-resistor`)
**Domain:** Electromagnetics / Antennas
**Definition:** λ/4 sections + isolation resistor; equal-split with port isolation.
**Atom or composite:** Composite — λ/4 + resistor.
**Cost model:** Closed-form.
**Real wall?** Yes — narrowband; multi-section for BW.
**Cross-domain wiring:** signal-processing-rf (balanced amps), networking (combiners).
**Notes:** Wilkinson 1960.

### circulator-ferrite (cross-domain alias: `3-port-circulator`, `gyrator`, `nonreciprocal-y-junction`)
**Domain:** Electromagnetics / Antennas
**Definition:** Three-port device with cyclic isolation: 1→2→3→1; uses biased ferrite.
**Atom or composite:** Composite — biased ferrite + Y-junction.
**Cost model:** Bulky magnet + ferrite; bandwidth limited.
**Real wall?** Yes — Faraday rotation in B field breaks reciprocity.
**Cross-domain wiring:** signal-processing-rf (T/R separation), quantum-computing (non-reciprocal links).
**Notes:** Hogan 1952.

### isolator (cross-domain alias: `two-port-isolator`, `terminated-circulator`, `reverse-block`)
**Domain:** Electromagnetics / Antennas
**Definition:** Circulator with port-3 terminated; passes forward, absorbs reverse.
**Atom or composite:** Composite — circulator + load.
**Cost model:** Same as circulator.
**Real wall?** Yes — same physics.
**Cross-domain wiring:** signal-processing-rf (oscillator protection).
**Notes:** Pozar §9.

### balun (cross-domain alias: `balanced-unbalanced-xfmr`, `marchand-balun`, `bazooka`)
**Domain:** Electromagnetics / Antennas
**Definition:** Converts unbalanced (coax) to balanced (dipole) currents; suppresses CM.
**Atom or composite:** Composite — distributed or lumped balun.
**Cost model:** Various: ferrite, λ/4 sleeve, Marchand.
**Real wall?** Yes — CM suppression bounded by balun design.
**Cross-domain wiring:** signal-processing-rf (mixer feeds), networking (dipole feeds).
**Notes:** Bazooka by Roberts 1953; Marchand 1944.

### phase-shifter-pin-diode (cross-domain alias: `pin-diode-ps`, `loaded-line-ps`, `switched-line-ps`)
**Domain:** Electromagnetics / Antennas
**Definition:** PIN diodes switch reactive loads or path lengths to achieve discrete phase states.
**Atom or composite:** Composite — switch + reactive elements.
**Cost model:** N-bit phase resolution; per-bit insertion loss.
**Real wall?** Yes — quantization error.
**Cross-domain wiring:** signal-processing-rf (phased arrays), networking (beamformers).
**Notes:** Pozar §10.

### phase-shifter-varactor (cross-domain alias: `varactor-ps`, `analog-phase-shifter`, `cv-tuned`)
**Domain:** Electromagnetics / Antennas
**Definition:** Varactor diode capacitance tunes resonator → continuous phase shift.
**Atom or composite:** Composite — voltage-tuned reactance.
**Cost model:** Continuous; Q limited by varactor.
**Real wall?** Yes — Q vs. tuning range tradeoff.
**Cross-domain wiring:** signal-processing-rf (VCO tuning).
**Notes:** Pozar §10.

### phase-shifter-mems (cross-domain alias: `mems-ps`, `rf-mems-switched-cap`, `low-loss-ps`)
**Domain:** Electromagnetics / Antennas
**Definition:** RF-MEMS switched capacitors give very low insertion loss phase states.
**Atom or composite:** Composite — micromechanical switch.
**Cost model:** Slow switching; high Q.
**Real wall?** Yes — switch cycle lifetime; packaging.
**Cross-domain wiring:** signal-processing-rf (low-loss beamformers).
**Notes:** Rebeiz 2003.

### saw-resonator (cross-domain alias: `surface-acoustic-wave`, `idt-saw`, `mhz-ghz-filter`)
**Domain:** Electromagnetics / Antennas
**Definition:** Interdigital transducers launch surface acoustic waves on piezo substrate; Q ≈ 10000.
**Atom or composite:** Composite — acoustic-EM transducer.
**Cost model:** Volume manufactured; up to a few GHz.
**Real wall?** Yes — limited by piezoelectric materials.
**Cross-domain wiring:** signal-processing-rf (RF filters), networking (handset IF filtering).
**Notes:** White & Voltmer 1965.

### baw-fbar-resonator (cross-domain alias: `bulk-acoustic-wave`, `fbar-film-bulk`, `5g-filter`)
**Domain:** Electromagnetics / Antennas
**Definition:** Thin piezo film between electrodes resonates at bulk thickness mode; GHz Q ~1000s.
**Atom or composite:** Composite — bulk acoustic resonator.
**Cost model:** Thin-film manufacturing.
**Real wall?** Yes — material acoustic loss.
**Cross-domain wiring:** networking (5G band filters), signal-processing-rf (sharp BPFs).
**Notes:** Avago/Broadcom FBAR.

### ferrite-yig-resonator (cross-domain alias: `yig-resonator`, `magnetically-tunable-bpf`, `gyromagnetic-resonance`)
**Domain:** Electromagnetics / Antennas
**Definition:** Yttrium-iron-garnet sphere in DC magnetic field resonates at ω = γ·B.
**Atom or composite:** Composite — gyromagnetic resonance + cavity coupling.
**Cost model:** Hours of tuning; expensive magnet.
**Real wall?** Yes — linear ω(B); set by gyromagnetic ratio.
**Cross-domain wiring:** signal-processing-rf (tunable filters/oscillators).
**Notes:** Pozar §10.

---

## Active Microwave Devices

### low-noise-amplifier (cross-domain alias: `lna`, `nf-minimization`, `gamma-opt-match`)
**Domain:** Electromagnetics / Antennas
**Definition:** Amplifier designed for minimum noise figure at fixed gain; input matched to Γ_opt, not Γ_S*.
**Atom or composite:** Composite — active 2-port + input-noise-match network.
**Cost model:** Sets RX sensitivity floor.
**Real wall?** Yes — Friis noise cascade; NF_min set by device technology.
**Cross-domain wiring:** signal-processing-rf (RX front-end), information-theory-coding (SNR floor).
**Notes:** Pozar §13.

### mixer-double-balanced (cross-domain alias: `dbm-mixer`, `gilbert-cell`, `lo-rejection`)
**Domain:** Electromagnetics / Antennas
**Definition:** Four diodes or Gilbert cell; rejects LO and even-order distortion.
**Atom or composite:** Composite — balanced diode/transistor ring.
**Cost model:** Conversion loss 6–8 dB (passive); active gives gain.
**Real wall?** Yes — IP3 vs LO power tradeoff.
**Cross-domain wiring:** signal-processing-rf (frequency conversion).
**Notes:** Maas "Microwave Mixers".

### frequency-multiplier (cross-domain alias: `varactor-multiplier`, `step-recovery-diode`, `harmonic-generator`)
**Domain:** Electromagnetics / Antennas
**Definition:** Nonlinear element generates Nf₀; filter selects desired harmonic.
**Atom or composite:** Composite — nonlinearity + filtering.
**Cost model:** Conversion loss increases with N.
**Real wall?** Yes — Page-Pantell bound on max efficiency.
**Cross-domain wiring:** signal-processing-rf (mmWave source), photonics-optics (frequency comb analog).
**Notes:** Pozar §13.

### voltage-controlled-oscillator (cross-domain alias: `vco`, `varactor-tuned-osc`, `phase-noise-leeson`)
**Domain:** Electromagnetics / Antennas
**Definition:** Oscillator whose frequency varies linearly with control voltage.
**Atom or composite:** Composite — amplifier + tunable resonator.
**Cost model:** Phase noise per Leeson model.
**Real wall?** Yes — phase noise bounded by Q · P_signal.
**Cross-domain wiring:** signal-processing-rf (PLL), control-numerical-opt (feedback).
**Notes:** Leeson 1966.

### dielectric-resonator-oscillator (cross-domain alias: `dro`, `high-q-osc`, `low-phase-noise`)
**Domain:** Electromagnetics / Antennas
**Definition:** Active oscillator stabilized by high-Q dielectric puck.
**Atom or composite:** Composite — active gain + DR resonator.
**Cost model:** Discrete frequency; low phase noise.
**Real wall?** Yes — DR Q sets phase noise floor.
**Cross-domain wiring:** signal-processing-rf (radar LO).
**Notes:** Khanna 1984.

### yig-oscillator (cross-domain alias: `yto`, `magnetically-tuned-vco`, `octave-vco`)
**Domain:** Electromagnetics / Antennas
**Definition:** YIG resonator tunes wide band by varying B field.
**Atom or composite:** Composite — YIG + active gain.
**Cost model:** Slow tuning; multi-octave.
**Real wall?** Yes — magnet hysteresis.
**Cross-domain wiring:** signal-processing-rf (test equipment).
**Notes:** Used in spectrum analyzers.

### power-amplifier-class-a (cross-domain alias: `class-a-pa`, `linear-pa`, `360-deg-conduction`)
**Domain:** Electromagnetics / Antennas
**Definition:** Bias for 360° conduction; ≤50 % theoretical efficiency; most linear.
**Atom or composite:** Composite — transistor + matching networks.
**Cost model:** Heat-limited.
**Real wall?** Yes — class-A efficiency bound.
**Cross-domain wiring:** signal-processing-rf (linear PAs for OFDM).
**Notes:** Cripps "RF Power Amplifiers".

### power-amplifier-class-b (cross-domain alias: `class-b-pa`, `push-pull`, `180-deg-conduction`)
**Domain:** Electromagnetics / Antennas
**Definition:** 180° conduction angle; up to 78.5 % efficiency; needs push-pull.
**Atom or composite:** Composite — biased transistor pair.
**Cost model:** Crossover distortion.
**Real wall?** Yes — efficiency-linearity tradeoff.
**Cross-domain wiring:** signal-processing-rf (audio + RF PAs).
**Notes:** Cripps.

### power-amplifier-class-c (cross-domain alias: `class-c-pa`, `narrowband-high-eff`, `cw-pa`)
**Domain:** Electromagnetics / Antennas
**Definition:** <180° conduction; >80 % efficiency; nonlinear, narrowband only.
**Atom or composite:** Composite — heavy biased transistor.
**Cost model:** Filters needed for harmonics.
**Real wall?** Yes — CW/FM only.
**Cross-domain wiring:** signal-processing-rf (FM broadcast).
**Notes:** Cripps.

### power-amplifier-class-e (cross-domain alias: `class-e-pa`, `switch-mode-pa`, `zero-voltage-switching`)
**Domain:** Electromagnetics / Antennas
**Definition:** Switch-mode PA with shunt capacitor + series tank; ZVS gives ~100 % theoretical efficiency.
**Atom or composite:** Composite — switching + reactive network.
**Cost model:** Narrowband; needs careful tuning.
**Real wall?** Yes — switching-time bound.
**Cross-domain wiring:** signal-processing-rf (wireless power), networking (efficient mobile PAs).
**Notes:** Sokal 1975.

### power-amplifier-class-f (cross-domain alias: `class-f-pa`, `harmonic-tuned-pa`, `square-wave-current`)
**Domain:** Electromagnetics / Antennas
**Definition:** Harmonic-tuned load presents short/open at even/odd harmonics; flat voltage waveform.
**Atom or composite:** Composite — harmonic-impedance tuning.
**Cost model:** Difficult to maintain over band.
**Real wall?** Yes — finite harmonic count limits η.
**Cross-domain wiring:** signal-processing-rf (high-efficiency mobile).
**Notes:** Raab 1997.

### doherty-amplifier (cross-domain alias: `doherty-pa`, `main-peaking-pa`, `back-off-efficient`)
**Domain:** Electromagnetics / Antennas
**Definition:** Main + peaking amp combined via λ/4; high efficiency over 6-dB back-off.
**Atom or composite:** Composite — load-modulation network.
**Cost model:** Bandwidth limited; difficult linearization.
**Real wall?** Yes — load-modulation physics.
**Cross-domain wiring:** networking (5G base stations).
**Notes:** Doherty 1936.

### outphasing-linc (cross-domain alias: `linc-pa`, `chireix-outphasing`, `constant-envelope-pair`)
**Domain:** Electromagnetics / Antennas
**Definition:** Decompose signal into two constant-envelope phasors, amplify with nonlinear PAs, combine.
**Atom or composite:** Composite — Chireix combiner.
**Cost model:** Efficient at back-off if combiner ideal.
**Real wall?** Yes — combiner non-ideality.
**Cross-domain wiring:** signal-processing-rf (envelope decomposition).
**Notes:** Chireix 1935; Cox 1974.

### envelope-tracking-pa (cross-domain alias: `et-pa`, `dynamic-bias-pa`, `digital-supply-modulator`)
**Domain:** Electromagnetics / Antennas
**Definition:** PA supply voltage tracks signal envelope; high efficiency over wide back-off.
**Atom or composite:** Composite — PA + DC-DC supply modulator.
**Cost model:** Modulator BW + efficiency tradeoff.
**Real wall?** Yes — supply BW bounds video signal.
**Cross-domain wiring:** networking (4G/5G handsets), control-numerical-opt (supply tracking).
**Notes:** Asbeck et al.

### gan-power-amplifier (cross-domain alias: `gan-pa`, `hemt-gan`, `wide-bandgap-pa`)
**Domain:** Electromagnetics / Antennas
**Definition:** GaN HEMT PAs: high V_breakdown, high f_T; ideal for high-power mmWave.
**Atom or composite:** Composite — device + matching + thermal.
**Cost model:** Thermal-density limited.
**Real wall?** Yes — Johnson's figure of merit.
**Cross-domain wiring:** signal-processing-rf (radar, 5G PA).
**Notes:** Mishra & Singh "Semiconductor Devices".

---

## Antenna Measurement

### anechoic-chamber (cross-domain alias: `absorber-lined-room`, `taper-foam-chamber`, `quiet-zone`)
**Domain:** Electromagnetics / Antennas
**Definition:** Room lined with pyramidal/wedge RF absorber for free-space measurements.
**Atom or composite:** Composite — absorber + room geometry.
**Cost model:** Capital intensive; size scales with λ.
**Real wall?** Yes — absorber reflectivity bounds quiet-zone level.
**Cross-domain wiring:** signal-processing-rf (EMC test), networking (handset OTA).
**Notes:** Hemming "Electromagnetic Anechoic Chambers".

### compact-antenna-test-range (cross-domain alias: `catr`, `compact-range`, `reflector-quiet-zone`)
**Domain:** Electromagnetics / Antennas
**Definition:** Offset parabolic reflector creates plane wave in small chamber.
**Atom or composite:** Composite — single-reflector collimator.
**Cost model:** High; serrated reflector for low ripple.
**Real wall?** Yes — quiet-zone size ≈ aperture/2.
**Cross-domain wiring:** signal-processing-rf (large antenna tests).
**Notes:** Johnson 1969.

### planar-near-field-scanning (cross-domain alias: `pnf-scan`, `flat-aperture-scan`, `nearfield-to-farfield`)
**Domain:** Electromagnetics / Antennas
**Definition:** Probe scans on plane in near field; FFT gives far field.
**Atom or composite:** Composite — sampling theorem + plane-wave decomposition.
**Cost model:** Number of samples grows with aperture/λ.
**Real wall?** Yes — finite scan plane truncates.
**Cross-domain wiring:** signal-processing-rf (FFT), retrieval-search (spectral decomposition).
**Notes:** Yaghjian 1986.

### cylindrical-near-field (cross-domain alias: `cnf-scan`, `cylindrical-modes-nf`, `bessel-expansion-nf`)
**Domain:** Electromagnetics / Antennas
**Definition:** Probe traverses cylinder; modal expansion in Hankel functions; broader pattern.
**Atom or composite:** Composite — cylindrical mode expansion.
**Cost model:** More samples than planar.
**Real wall?** Yes — endcap truncation.
**Cross-domain wiring:** signal-processing-rf (mode expansion).
**Notes:** Hansen ed.

### spherical-near-field (cross-domain alias: `snf-scan`, `spherical-modes-nf`, `wave-coefficient-method`)
**Domain:** Electromagnetics / Antennas
**Definition:** Full-sphere probe scan; spherical-wave expansion exact for AUT.
**Atom or composite:** Composite — spherical-harmonic expansion.
**Cost model:** Heavy; sample count ~ (kR)².
**Real wall?** Yes — exact within sphere if no truncation.
**Cross-domain wiring:** quantum-computing (spherical harmonics), graphics-rendering-lod (PRT).
**Notes:** Hansen ed.

### gain-substitution-method (cross-domain alias: `substitution-gain`, `standard-gain-cal`, `comparison-method`)
**Domain:** Electromagnetics / Antennas
**Definition:** Compare RX power of AUT to standard gain horn; G_AUT = G_std + (P_AUT − P_std)_dB.
**Atom or composite:** Composite — calibration via reference.
**Cost model:** Quick; requires known std-gain horn.
**Real wall?** No — accuracy ≈ ±0.5 dB.
**Cross-domain wiring:** signal-processing-rf (calibration).
**Notes:** IEEE Std 149.

### three-antenna-method (cross-domain alias: `3-antenna-gain`, `pairwise-link-gain`, `absolute-gain-cal`)
**Domain:** Electromagnetics / Antennas
**Definition:** Measure three pairs of antennas; solve linear system for absolute gains.
**Atom or composite:** Composite — overdetermined Friis-link inversion.
**Cost model:** Three measurements; algebra.
**Real wall?** No — accuracy ≈ ±0.5 dB.
**Cross-domain wiring:** linear-algebra-matrix (3-eq solve).
**Notes:** IEEE Std 149.

### vna-solt-calibration (cross-domain alias: `solt-cal`, `short-open-load-thru`, `one-port-vna-cal`)
**Domain:** Electromagnetics / Antennas
**Definition:** 4 known standards remove systematic errors from VNA measurements.
**Atom or composite:** Composite — 12-term error model inversion.
**Cost model:** Few minutes; software handles math.
**Real wall?** Yes — assumes ideal standards.
**Cross-domain wiring:** signal-processing-rf (S-parameters), linear-algebra-matrix (error matrix).
**Notes:** Schiek "Grundlagen der HF-Messtechnik".

### vna-trl-calibration (cross-domain alias: `trl-cal`, `thru-reflect-line`, `nist-trl`)
**Domain:** Electromagnetics / Antennas
**Definition:** Thru + Reflect + λ/4 line as standards; less reliance on perfect known loads.
**Atom or composite:** Composite — self-calibration via line.
**Cost model:** Best at high freq; band-limited.
**Real wall?** No — assumes line is good Z₀.
**Cross-domain wiring:** signal-processing-rf (on-wafer cal).
**Notes:** Engen & Hoer 1979.

### vna-lrm-calibration (cross-domain alias: `lrm-cal`, `line-reflect-match`, `wafer-prober-cal`)
**Domain:** Electromagnetics / Antennas
**Definition:** Line + Reflect + Match standards; well-suited for on-wafer probing.
**Atom or composite:** Composite — alternative to TRL.
**Cost model:** Practical at MMIC frequencies.
**Real wall?** No — match must be very good.
**Cross-domain wiring:** signal-processing-rf (MMIC measurement).
**Notes:** Davidson et al.

---

## Specialty Structures: Metamaterials, RIS, EBG, FSS

### metamaterial-negative-index (cross-domain alias: `nim`, `eps-and-mu-negative`, `veselago-medium`)
**Domain:** Electromagnetics / Antennas
**Definition:** Composite with simultaneously ε<0 and μ<0; n<0; backward-wave propagation.
**Atom or composite:** Composite — engineered effective ε, μ.
**Cost model:** Sub-λ unit cells; complex design.
**Real wall?** Yes — only within design band; loss limits.
**Cross-domain wiring:** photonics-optics (perfect lens), signal-processing-rf (super-resolution).
**Notes:** Veselago 1968; Smith et al. 2000.

### split-ring-resonator (cross-domain alias: `srr`, `magnetic-meta-atom`, `pendry-srr`)
**Domain:** Electromagnetics / Antennas
**Definition:** Sub-λ ring with gap; provides μ_eff(ω) with Lorentzian resonance.
**Atom or composite:** Composite — LC unit cell.
**Cost model:** Full-wave per unit cell.
**Real wall?** Yes — saturation due to ohmic loss.
**Cross-domain wiring:** photonics-optics (metamaterial slab), control-numerical-opt (homogenization).
**Notes:** Pendry 1999.

### frequency-selective-surface (cross-domain alias: `fss`, `bandpass-bandstop-screen`, `periodic-array-screen`)
**Domain:** Electromagnetics / Antennas
**Definition:** Periodic array of patches/apertures filters incident waves by frequency and polarization.
**Atom or composite:** Composite — 2D periodic array.
**Cost model:** Periodic-MoM per unit cell.
**Real wall?** Yes — Floquet harmonics set BW.
**Cross-domain wiring:** photonics-optics (dichroic), signal-processing-rf (radomes).
**Notes:** Munk "Frequency Selective Surfaces".

### reconfigurable-intelligent-surface (cross-domain alias: `ris`, `irs`, `programmable-metasurface`)
**Domain:** Electromagnetics / Antennas
**Definition:** Metasurface with electronically-controlled phase elements; relays/redirects EM waves.
**Atom or composite:** Composite — array of tunable meta-atoms.
**Cost model:** N varactors/PIN diodes + control.
**Real wall?** Yes — discrete phase quantization, loss.
**Cross-domain wiring:** networking (6G smart radio environment), signal-processing-rf (passive beamforming).
**Notes:** Di Renzo et al. 2020.

### artificial-magnetic-conductor (cross-domain alias: `amc`, `hi-z-surface`, `pmc-surrogate`)
**Domain:** Electromagnetics / Antennas
**Definition:** Periodic surface with reflection phase = 0 (PMC analog) over a band.
**Atom or composite:** Composite — patch+via mushroom or printed FSS.
**Cost model:** Unit-cell simulation.
**Real wall?** Yes — narrowband near AMC resonance.
**Cross-domain wiring:** signal-processing-rf (low-profile antennas).
**Notes:** Sievenpiper 1999.

### electromagnetic-bandgap (cross-domain alias: `ebg`, `photonic-bandgap-rf`, `mushroom-surface`)
**Domain:** Electromagnetics / Antennas
**Definition:** Periodic structure with stopband — no propagation in plane.
**Atom or composite:** Composite — periodic lattice + analysis via Floquet.
**Cost model:** Band diagram calculation.
**Real wall?** Yes — physical bandgap.
**Cross-domain wiring:** photonics-optics (photonic crystals), quantum-computing (PCB stopbands).
**Notes:** Sievenpiper 1999; Yablonovitch 1987.

### high-impedance-surface (cross-domain alias: `his`, `mushroom-his`, `surface-wave-suppression`)
**Domain:** Electromagnetics / Antennas
**Definition:** Surface with high Z_s suppresses surface waves; flat-AMC reflection at resonance.
**Atom or composite:** Composite — AMC + EBG combined.
**Cost model:** Unit-cell simulation.
**Real wall?** Yes — narrowband suppression.
**Cross-domain wiring:** signal-processing-rf (antenna platforms).
**Notes:** Sievenpiper 1999.

### transformation-optics (cross-domain alias: `to`, `coordinate-transform-em`, `cloaking-design`)
**Domain:** Electromagnetics / Antennas
**Definition:** Maxwell's form-invariance under coordinate transforms → design ε, μ tensors that mold field flow.
**Atom or composite:** Composite — push-forward of fields under diffeo.
**Cost model:** Closed-form transform; manufacturing challenging.
**Real wall?** Yes — needs anisotropic, inhomogeneous, possibly singular media.
**Cross-domain wiring:** photonics-optics (invisibility cloaks), type-theory-programming-languages (covariant tensors), formal-verification (gauge invariance).
**Notes:** Pendry, Schurig, Smith 2006.

### invisibility-cloak (cross-domain alias: `em-cloak`, `to-cloak`, `cylinder-shell-cloak`)
**Domain:** Electromagnetics / Antennas
**Definition:** TO-designed shell where energy flows around interior region.
**Atom or composite:** Composite — TO + metamaterial implementation.
**Cost model:** Narrowband; lossy.
**Real wall?** Yes — fundamental bandwidth limit (Hashimoto 2008).
**Cross-domain wiring:** photonics-optics (cloak demos), formal-verification (idealized symmetry).
**Notes:** Schurig et al. 2006.

### plasmonic-waveguide (cross-domain alias: `spp-waveguide`, `metal-dielectric-interface`, `nano-plasmonics`)
**Domain:** Electromagnetics / Antennas
**Definition:** Surface plasmon polaritons bound to metal-dielectric interface; sub-λ confinement.
**Atom or composite:** Composite — Drude metal + dielectric.
**Cost model:** Tight mesh near interface.
**Real wall?** Yes — ohmic loss strict.
**Cross-domain wiring:** photonics-optics (nanophotonics), quantum-computing (plasmonic qubits).
**Notes:** Maier "Plasmonics".

---

## Wireless Power, RFID & NFC

### inductive-coupling-wpt (cross-domain alias: `near-field-wpt`, `mutual-inductance-link`, `qi-charging`)
**Domain:** Electromagnetics / Antennas
**Definition:** Transformer-like power transfer at low f via magnetic flux linkage; range ≪ coil size.
**Atom or composite:** Composite — coupled inductors.
**Cost model:** η ∝ k² Q_TX Q_RX.
**Real wall?** Yes — coupling falls as r⁻³.
**Cross-domain wiring:** networking (Qi standard), signal-processing-rf (RFID HF).
**Notes:** Tesla 1891.

### magnetic-resonance-coupling (cross-domain alias: `mr-wpt`, `witricity`, `strongly-coupled-resonant`)
**Domain:** Electromagnetics / Antennas
**Definition:** Two high-Q resonant coils at same f₀; mid-range (~m) power transfer.
**Atom or composite:** Composite — coupled high-Q resonators.
**Cost model:** Coupling × Q product sets efficiency.
**Real wall?** Yes — radiation losses at ~MHz.
**Cross-domain wiring:** quantum-computing (resonator-resonator coupling), networking (mid-range charging).
**Notes:** Kurs et al. 2007 (MIT WiTricity).

### beamed-power-microwave (cross-domain alias: `mwpt`, `rectenna-array`, `sps-power`)
**Domain:** Electromagnetics / Antennas
**Definition:** Microwave beam to large rectenna; satellite-to-ground or drone power.
**Atom or composite:** Composite — phased-array TX + rectenna RX.
**Cost model:** Aperture-size limited; safety constraints.
**Real wall?** Yes — atmospheric absorption + diffraction.
**Cross-domain wiring:** networking (long-range WPT), signal-processing-rf (high-power beamforming).
**Notes:** Brown 1968; Glaser SPS concept.

### rectenna (cross-domain alias: `rf-to-dc-antenna`, `schottky-rectifier-ant`, `harvesting-antenna`)
**Domain:** Electromagnetics / Antennas
**Definition:** Antenna + matching + Schottky rectifier; converts RF → DC.
**Atom or composite:** Composite — antenna + rectifier circuit.
**Cost model:** Efficiency drops at low P_in (sub-threshold).
**Real wall?** Yes — Schottky knee voltage.
**Cross-domain wiring:** signal-processing-rf (energy harvesting), networking (passive IoT).
**Notes:** Brown 1965.

### rfid-hf-13-56-mhz (cross-domain alias: `hf-rfid`, `13mhz-tag`, `iso-14443`)
**Domain:** Electromagnetics / Antennas
**Definition:** Inductively-coupled 13.56 MHz; ~10 cm range; smart cards, NFC.
**Atom or composite:** Composite — loop antenna + LC tag chip.
**Cost model:** Low; mass-produced.
**Real wall?** Yes — near-field range limit.
**Cross-domain wiring:** networking (NFC payments), cryptography-advanced (RFID auth).
**Notes:** ISO/IEC 14443.

### rfid-uhf-900-mhz (cross-domain alias: `uhf-rfid`, `epc-gen2`, `backscatter-tag`)
**Domain:** Electromagnetics / Antennas
**Definition:** Backscatter modulation at 860–960 MHz; up to ~10 m range; warehouse, logistics.
**Atom or composite:** Composite — dipole tag + backscatter modulator.
**Cost model:** Reader EIRP limited by regulation.
**Real wall?** Yes — link-budget physics.
**Cross-domain wiring:** networking (supply chain), signal-processing-rf (backscatter modulation).
**Notes:** EPC Gen2.

### rfid-microwave-24-ghz (cross-domain alias: `microwave-rfid`, `etoll`, `chipless-rfid`)
**Domain:** Electromagnetics / Antennas
**Definition:** 2.4 or 5.8 GHz; high data-rate RFID for toll, asset tracking.
**Atom or composite:** Composite — patch antenna + active or backscatter tag.
**Cost model:** Higher than UHF.
**Real wall?** Yes — atmospheric absorption + multipath.
**Cross-domain wiring:** networking (active RFID).
**Notes:** ISO 18000-4.

### nfc-near-field-comms (cross-domain alias: `nfc`, `magnetic-near-field-link`, `iso-18092`)
**Domain:** Electromagnetics / Antennas
**Definition:** 13.56 MHz two-way magnetic-coupled comms; cm range; mobile payments.
**Atom or composite:** Composite — RFID HF + bidirectional protocol.
**Cost model:** Cheap; integrated into phones.
**Real wall?** Yes — physical near-field range.
**Cross-domain wiring:** networking (mobile payment), cryptography-advanced (secure element).
**Notes:** ISO/IEC 18092.

---

## Specialty Topics

### chu-harrington-limit (cross-domain alias: `q-of-small-antenna`, `min-Q-volume`, `electrically-small-bound`)
**Domain:** Electromagnetics / Antennas
**Definition:** Q ≥ 1/(ka)³ + 1/(ka). Lower bound on radiation Q of antenna in sphere ka.
**Atom or composite:** Atom — fundamental small-antenna limit.
**Cost model:** Free; informs design viability.
**Real wall?** Yes — strict derived from spherical-mode expansion.
**Cross-domain wiring:** networking (handset miniaturization), information-theory-coding (BW limits).
**Notes:** Chu 1948; Harrington 1960.

### foster-reactance-theorem (cross-domain alias: `foster`, `lossless-reactance-monotonic`, `dX-dw-positive`)
**Domain:** Electromagnetics / Antennas
**Definition:** For lossless network, dX/dω > 0; reactance monotonic increasing with frequency.
**Atom or composite:** Atom — passivity + losslessness consequence.
**Cost model:** Free constraint.
**Real wall?** Yes — strict for lossless.
**Cross-domain wiring:** networking (passive networks), control-numerical-opt (passivity).
**Notes:** Foster 1924.

### gain-bandwidth-product (cross-domain alias: `gbw-product`, `Q-times-BW`, `RC-limit`)
**Domain:** Electromagnetics / Antennas
**Definition:** For RC-loaded amplifier, G·BW = constant; passive matching similarly bounded.
**Atom or composite:** Atom — single-pole roll-off.
**Cost model:** Free.
**Real wall?** Yes — single-pole.
**Cross-domain wiring:** signal-processing-rf (amplifier design), information-theory-coding (Shannon-Hartley).
**Notes:** Bode.

### sommerfeld-integral (cross-domain alias: `half-space-greens`, `dipole-over-ground`, `sommerfeld-tail`)
**Domain:** Electromagnetics / Antennas
**Definition:** Spectral integral for fields of dipole over layered half-space; oscillatory.
**Atom or composite:** Composite — Hankel transform of layered Green's function.
**Cost model:** Slow convergence; tail acceleration needed.
**Real wall?** Yes — exact representation.
**Cross-domain wiring:** signal-processing-rf (subsurface sensing), retrieval-search (GPR imaging).
**Notes:** Sommerfeld 1909.

### image-theory (cross-domain alias: `pec-image`, `mirror-source`, `half-space-equivalence`)
**Domain:** Electromagnetics / Antennas
**Definition:** Source over PEC ground equivalent to source + opposite image source in free space.
**Atom or composite:** Composite — symmetry argument.
**Cost model:** Free.
**Real wall?** Yes — exact for PEC ground.
**Cross-domain wiring:** computational-geometry (reflections), graphics-rendering-lod (mirror surfaces).
**Notes:** Jackson §2.

### floquet-theorem-em (cross-domain alias: `bloch-em`, `periodic-em-modes`, `unit-cell-modes`)
**Domain:** Electromagnetics / Antennas
**Definition:** Fields in periodic structure: E(r+a) = E(r)exp(jk·a). Spectrum in k-space.
**Atom or composite:** Atom — periodic-symmetry consequence.
**Cost model:** Reduce 3D problem to unit-cell.
**Real wall?** Yes — exact for infinite periodic.
**Cross-domain wiring:** quantum-computing (Bloch states), signal-processing-rf (periodic structures).
**Notes:** Floquet 1883; Bloch 1928.

### kraus-formula-directivity (cross-domain alias: `D-equals-32400-over-hpbw-product`, `quick-D-estimate`, `radian-bw-shortcut`)
**Domain:** Electromagnetics / Antennas
**Definition:** D ≈ 32400/(θ_E · θ_H) [deg² approx]; quick directivity estimate.
**Atom or composite:** Composite — beam solid angle approximation.
**Cost model:** Trivial.
**Real wall?** No — ~10 % accuracy.
**Cross-domain wiring:** signal-processing-rf (link planning).
**Notes:** Kraus.

### tai-pereira-formula (cross-domain alias: `tp-directivity-approx`, `D-equals-101-over-hpbw-sum`, `accurate-quick-D`)
**Domain:** Electromagnetics / Antennas
**Definition:** D ≈ 101/(θ_E + θ_H) rad form; alternative Kraus.
**Atom or composite:** Composite — approximate.
**Cost model:** Trivial.
**Real wall?** No.
**Cross-domain wiring:** signal-processing-rf (link planning).
**Notes:** Tai & Pereira 1976.

### radome-design (cross-domain alias: `radome`, `dielectric-shell-antenna-cover`, `transmissive-shell`)
**Domain:** Electromagnetics / Antennas
**Definition:** Dielectric shell protects antenna while transmitting EM; thickness tuned to minimize reflection.
**Atom or composite:** Composite — multi-layer dielectric.
**Cost model:** TMM + GO; structural constraints.
**Real wall?** Yes — beam pointing errors, BW limits.
**Cross-domain wiring:** photonics-optics (AR coatings), signal-processing-rf (boresight-error budget).
**Notes:** Kozakoff "Analysis of Radome-Enclosed Antennas".

### polarization-mismatch-loss (cross-domain alias: `pml-pol`, `pol-loss-factor`, `plf`)
**Domain:** Electromagnetics / Antennas
**Definition:** PLF = |ê_TX · ê_RX|² ; loss when TX/RX polarizations don't align.
**Atom or composite:** Atom — inner product of polarization vectors.
**Cost model:** Trivial.
**Real wall?** Yes — physical alignment.
**Cross-domain wiring:** signal-processing-rf (link budget), networking (dual-pol systems).
**Notes:** Balanis §2.

### voltage-standing-wave-pattern (cross-domain alias: `vsw-pattern`, `position-dependent-V`, `mismatched-line-envelope`)
**Domain:** Electromagnetics / Antennas
**Definition:** |V(z)| = |V⁺|(1+|Γ|exp(j2βz)); periodic with λ/2.
**Atom or composite:** Composite — interference of incident + reflected.
**Cost model:** Trivial.
**Real wall?** Yes — physical envelope.
**Cross-domain wiring:** signal-processing-rf (slotted-line measurement).
**Notes:** Pozar §2.

### thermal-noise-antenna (cross-domain alias: `t-ant`, `brightness-temperature-rcv`, `nyquist-noise`)
**Domain:** Electromagnetics / Antennas
**Definition:** P_n = kT_A B; T_A averaged over antenna pattern.
**Atom or composite:** Atom — Johnson-Nyquist applied to RX.
**Cost model:** Pattern-integrate sky brightness temperature.
**Real wall?** Yes — sets RX noise floor at low f.
**Cross-domain wiring:** statistics-probability (thermal noise), networking (LNA design).
**Notes:** Friis 1944.

### figure-of-merit-G-over-T (cross-domain alias: `g-over-t`, `gain-temperature-ratio`, `link-figure-of-merit`)
**Domain:** Electromagnetics / Antennas
**Definition:** G/T (dB/K). Single metric capturing RX antenna+LNA performance for sat links.
**Atom or composite:** Composite — gain/noise ratio.
**Cost model:** Measured per system.
**Real wall?** Yes — sets minimum bit-rate.
**Cross-domain wiring:** networking (satellite link budgets), information-theory-coding (capacity).
**Notes:** Pratt "Satellite Communications".

### group-delay-antenna (cross-domain alias: `phase-delay-vs-freq`, `tau-g-antenna`, `time-delay-antenna`)
**Domain:** Electromagnetics / Antennas
**Definition:** τ_g = −dφ/dω. UWB antennas need flat group delay.
**Atom or composite:** Atom — derivative of phase response.
**Cost model:** Numerical differentiation.
**Real wall?** Yes — narrowband antennas have varying τ_g.
**Cross-domain wiring:** signal-processing-rf (UWB), control-numerical-opt (linear-phase design).
**Notes:** Schantz "UWB Antennas".

### array-thinning (cross-domain alias: `sparse-array`, `random-thinning`, `density-tapered-array`)
**Domain:** Electromagnetics / Antennas
**Definition:** Remove elements from filled array; reduces cost, raises SLL on average.
**Atom or composite:** Composite — element subset selection.
**Cost model:** Combinatorial; GA/SA optimization.
**Real wall?** Yes — SLL floor by random thinning ≈ −13 dB.
**Cross-domain wiring:** combinatorial-optimization (subset selection), signal-processing-rf (sparse arrays).
**Notes:** Skolnik et al. 1964.

### sparse-array-aperiodic (cross-domain alias: `aperiodic-array`, `non-uniform-array`, `compressive-array`)
**Domain:** Electromagnetics / Antennas
**Definition:** Element positions optimized to avoid grating lobes despite >λ/2 spacing.
**Atom or composite:** Composite — position optimization.
**Cost model:** Heavy optimization.
**Real wall?** Yes — beamwidth-SLL trade.
**Cross-domain wiring:** combinatorial-optimization (placement), signal-processing-rf (sparse sensing).
**Notes:** Lo 1964.

### compressive-array-imaging (cross-domain alias: `ca-imaging`, `cs-radar`, `sparse-recovery-imaging`)
**Domain:** Electromagnetics / Antennas
**Definition:** Random sparse aperture + CS reconstruction recovers full image from few measurements.
**Atom or composite:** Composite — sparse measurement + ℓ₁ reconstruction.
**Cost model:** Heavy reconstruction.
**Real wall?** Yes — sparsity in image.
**Cross-domain wiring:** retrieval-search (CS), statistics-probability (sparse recovery).
**Notes:** Herman & Strohmer 2009.

### mimo-channel-capacity (cross-domain alias: `mimo-cap`, `foschini-cap`, `log-det-cap`)
**Domain:** Electromagnetics / Antennas
**Definition:** C = log₂ det(I + (ρ/N_t)·HH*); scales linearly with min(N_t, N_r) at high SNR.
**Atom or composite:** Composite — Shannon capacity in MIMO.
**Cost model:** Determinant per channel realization.
**Real wall?** Yes — Shannon limit.
**Cross-domain wiring:** information-theory-coding (capacity), linear-algebra-matrix (SVD).
**Notes:** Foschini & Gans 1998.

### massive-mimo (cross-domain alias: `mu-mimo-large`, `tdd-reciprocity-bff`, `5g-massive-mimo`)
**Domain:** Electromagnetics / Antennas
**Definition:** Base station with hundreds of antennas serves many users with simple linear precoding.
**Atom or composite:** Composite — large array + precoding.
**Cost model:** O(N_BS · K_users) per slot.
**Real wall?** Yes — pilot contamination, mutual coupling.
**Cross-domain wiring:** networking (5G/6G), signal-processing-rf (precoding).
**Notes:** Marzetta 2010.

### intelligent-reflecting-surface-channel (cross-domain alias: `ris-channel`, `cascaded-channel`, `passive-relay-channel`)
**Domain:** Electromagnetics / Antennas
**Definition:** H_total = H_TX-RIS · diag(Φ) · H_RIS-RX; channel cascading through RIS phases.
**Atom or composite:** Composite — multiplicative channel.
**Cost model:** Estimation challenging (cascade).
**Real wall?** Yes — discrete phase, path loss.
**Cross-domain wiring:** networking (6G smart environment), control-numerical-opt (joint precoding).
**Notes:** Wu & Zhang 2020.

### terahertz-antenna (cross-domain alias: `thz-ant`, `mmwave-plus`, `0.1-10-thz-antenna`)
**Domain:** Electromagnetics / Antennas
**Definition:** Antennas for 0.1–10 THz; lens-coupled, photoconductive, plasmonic.
**Atom or composite:** Composite — bridging RF/optics design rules.
**Cost model:** Limited measurement equipment.
**Real wall?** Yes — atmospheric water vapor, surface roughness ≈ λ.
**Cross-domain wiring:** photonics-optics (THz devices), networking (6G THz).
**Notes:** Jornet & Akyildiz 2013.

### graphene-antenna (cross-domain alias: `plasmonic-graphene-ant`, `tunable-2d-antenna`, `chemical-potential-tune`)
**Domain:** Electromagnetics / Antennas
**Definition:** Graphene supports surface-plasmon-polaritons; chemical-potential-tunable; THz resonance.
**Atom or composite:** Composite — 2D-material resonator.
**Cost model:** Lossy.
**Real wall?** Yes — graphene conductivity Kubo formula.
**Cross-domain wiring:** photonics-optics (2D materials), control-numerical-opt (tunable design).
**Notes:** Llatser 2012.

### orbital-angular-momentum-em (cross-domain alias: `oam-em`, `vortex-beam`, `topological-charge`)
**Domain:** Electromagnetics / Antennas
**Definition:** Helical-phase beams with phase exp(jℓφ); orthogonal modes carry OAM ℓ.
**Atom or composite:** Composite — phase-twisted beam.
**Cost model:** Spiral-phase-plate or array generation.
**Real wall?** Yes — modes are orthogonal but not capacity-multiplying in far field.
**Cross-domain wiring:** quantum-computing (OAM qudits), photonics-optics (vortex beams), information-theory-coding (mode multiplexing).
**Notes:** Allen et al. 1992.

### dual-band-multiband-antenna (cross-domain alias: `multi-band-ant`, `coupled-resonator-ant`, `multimode-radiator`)
**Domain:** Electromagnetics / Antennas
**Definition:** Single antenna with multiple operating bands via multiple resonant paths.
**Atom or composite:** Composite — multi-resonator structure.
**Cost model:** Full-wave per band.
**Real wall?** Yes — Chu-Harrington per band; coupling tradeoffs.
**Cross-domain wiring:** networking (cellular multiband).
**Notes:** Wong "Compact and Broadband Microstrip Antennas".

### ultra-wideband-antenna (cross-domain alias: `uwb-antenna`, `decade-bw-ant`, `vivaldi-spiral-conical`)
**Domain:** Electromagnetics / Antennas
**Definition:** Antennas with > decade bandwidth; spirals, biconicals, log-periodic.
**Atom or composite:** Composite — self-complementary or self-similar.
**Cost model:** Full-wave verification.
**Real wall?** Yes — Bode-Fano matching limit.
**Cross-domain wiring:** signal-processing-rf (UWB radar/sensors).
**Notes:** Schantz.

### sar-antenna (cross-domain alias: `synthetic-aperture-radar-ant`, `airborne-sar-ant`, `slotted-waveguide-sar`)
**Domain:** Electromagnetics / Antennas
**Definition:** Side-looking airborne radar antenna; large aperture for cross-range resolution; slotted-WG arrays common.
**Atom or composite:** Composite — array tailored to SAR geometry.
**Cost model:** Heavy ground processing.
**Real wall?** Yes — antenna size sets azimuth-resolution lower bound.
**Cross-domain wiring:** signal-processing-rf (SAR processing), retrieval-search (image formation).
**Notes:** Curlander & McDonough.

### gnss-antenna (cross-domain alias: `gps-ant`, `cp-l1-l2-l5-ant`, `multipath-rejecting-ant`)
**Domain:** Electromagnetics / Antennas
**Definition:** CP patch or choke-ring at L1/L2/L5; rejects multipath via cross-pol discrimination.
**Atom or composite:** Composite — patch+ground+choke ring.
**Cost model:** Mass-produced.
**Real wall?** Yes — multipath sets sub-meter accuracy floor.
**Cross-domain wiring:** networking (GNSS receivers), signal-processing-rf (multipath mitigation).
**Notes:** Kaplan & Hegarty.

### high-power-microwave-antenna (cross-domain alias: `hpm-ant`, `breakdown-limited-ant`, `multi-mw-ant`)
**Domain:** Electromagnetics / Antennas
**Definition:** Antennas designed for multi-MW peak; breakdown, multipactor, corona constraints.
**Atom or composite:** Composite — antenna + breakdown management.
**Cost model:** Bulky; pressurized.
**Real wall?** Yes — Paschen curve sets max E field.
**Cross-domain wiring:** signal-processing-rf (high-power radar), control-numerical-opt (waveform shaping).
**Notes:** Benford et al.

### implantable-antenna (cross-domain alias: `medical-implant-ant`, `medradio-ant`, `tissue-coupled-ant`)
**Domain:** Electromagnetics / Antennas
**Definition:** Sub-cm antenna in lossy biological tissue; MICS 402–405 MHz, ISM 2.4 GHz; SAR-limited power.
**Atom or composite:** Composite — small antenna + lossy medium.
**Cost model:** Tissue simulators for design.
**Real wall?** Yes — tissue loss, Chu limit, SAR regulation.
**Cross-domain wiring:** biology-bioinformatics (tissue dielectric), networking (medical BAN).
**Notes:** Kiourti & Nikita 2014.

### wearable-textile-antenna (cross-domain alias: `textile-ant`, `body-conformal-ant`, `conductive-fabric-ant`)
**Domain:** Electromagnetics / Antennas
**Definition:** Antennas in clothing using conductive thread; flexible substrates.
**Atom or composite:** Composite — flexible antenna over body.
**Cost model:** Material-loss limited.
**Real wall?** Yes — body proximity detunes.
**Cross-domain wiring:** networking (body-area networks), biology-bioinformatics (health monitoring).
**Notes:** Salonen 2004.

### underwater-em-antenna (cross-domain alias: `vlf-underwater-ant`, `submarine-ant`, `salt-water-ant`)
**Domain:** Electromagnetics / Antennas
**Definition:** Antennas for sub-kHz – MHz under sea water; high conductivity → tiny skin depth, huge loss.
**Atom or composite:** Composite — antenna in lossy conductive medium.
**Cost model:** Very large; VLF transmitters km scale.
**Real wall?** Yes — sea water σ ≈ 5 S/m; skin depth ~m at kHz.
**Cross-domain wiring:** physics-diffusion (parabolic limit), networking (submarine comms).
**Notes:** Wheeler 1961.

### vlf-elf-antenna (cross-domain alias: `vlf-tx-ant`, `elf-tx-ant`, `cape-may-loop`)
**Domain:** Electromagnetics / Antennas
**Definition:** Antennas for 3–30 kHz (VLF) and 3 Hz – 3 kHz (ELF); enormous size or buried loops.
**Atom or composite:** Composite — huge electrically-small radiator.
**Cost model:** Site-sized; very low efficiency.
**Real wall?** Yes — Chu-Harrington fatal at these frequencies.
**Cross-domain wiring:** networking (submarine ELF), signal-processing-rf (atmospheric noise).
**Notes:** Watt "VLF Radio Engineering".

### satellite-ka-ku-antenna (cross-domain alias: `ka-band-ant`, `ku-band-ant`, `vsat`)
**Domain:** Electromagnetics / Antennas
**Definition:** High-gain offset reflectors / phased arrays for Ka (26–40 GHz) / Ku (12–18 GHz); rain-fade managed.
**Atom or composite:** Composite — large-aperture reflector or array.
**Cost model:** Capital intensive.
**Real wall?** Yes — rain attenuation budgets.
**Cross-domain wiring:** networking (GEO/LEO comms), signal-processing-rf (link-margin design).
**Notes:** Pratt et al.

### leo-satellite-array-antenna (cross-domain alias: `leo-esa`, `starlink-array`, `phased-array-satcom`)
**Domain:** Electromagnetics / Antennas
**Definition:** Electronically-scanned arrays on LEO satellites for fast handover and Earth coverage.
**Atom or composite:** Composite — AESA + Doppler/track management.
**Cost model:** Per-satellite cost dominated by array.
**Real wall?** Yes — Doppler shift, fast handover.
**Cross-domain wiring:** networking (LEO constellations), control-numerical-opt (beam scheduling).
**Notes:** Iridium, Starlink, OneWeb.

### deep-space-network-antenna (cross-domain alias: `dsn-70m`, `dsn-34m`, `goldstone-canberra-madrid`)
**Domain:** Electromagnetics / Antennas
**Definition:** 70 m / 34 m parabolic reflectors at S/X/Ka with cryogenically-cooled LNAs.
**Atom or composite:** Composite — giant reflector + ultra-low-noise RX.
**Cost model:** Government scale.
**Real wall?** Yes — sky-noise + LNA temperature.
**Cross-domain wiring:** networking (interplanetary comm), signal-processing-rf (deep-space telemetry).
**Notes:** NASA DSN.

### reflectarray-antenna (cross-domain alias: `printed-reflector`, `phased-reflector`, `reflectarray`)
**Domain:** Electromagnetics / Antennas
**Definition:** Flat array of printed elements with locally-varying phase mimics curved reflector.
**Atom or composite:** Composite — flat aperture with phase distribution.
**Cost model:** Printed-array assembly.
**Real wall?** Yes — element BW & scan capability.
**Cross-domain wiring:** signal-processing-rf (low-cost reflectors), photonics-optics (metasurface lens).
**Notes:** Huang & Encinar 2007.

### transmitarray-antenna (cross-domain alias: `printed-lens`, `phase-transmissive-array`, `flat-lens-antenna`)
**Domain:** Electromagnetics / Antennas
**Definition:** Two-sided printed surface with cell-by-cell transmission phase shapes wavefront.
**Atom or composite:** Composite — transmissive metasurface lens.
**Cost model:** PCB-fab; low loss vs reflectarray.
**Real wall?** Yes — cell loss & BW.
**Cross-domain wiring:** photonics-optics (metalens), signal-processing-rf (flat antennas).
**Notes:** McGrath 1986.

### bandwidth-aperture-product (cross-domain alias: `bw-area`, `space-bandwidth-product`, `aperture-bw-prod`)
**Domain:** Electromagnetics / Antennas
**Definition:** Number of orthogonal channels ≈ A·BW; analog of Shannon channels in space.
**Atom or composite:** Composite — generalized capacity bound.
**Cost model:** Free design law.
**Real wall?** Yes — diffraction + bandwidth bound.
**Cross-domain wiring:** information-theory-coding (capacity), photonics-optics (space-BW product).
**Notes:** Lohmann.

### radar-equation (cross-domain alias: `radar-range-eq`, `pr-equals-pt-g2-sigma-lambda2`, `monostatic-range`)
**Domain:** Electromagnetics / Antennas
**Definition:** P_r = P_t G² λ² σ / ((4π)³ R⁴ L). Sets range from EIRP, G/T, σ, integration.
**Atom or composite:** Composite — Friis applied twice + RCS.
**Cost model:** Trivial spreadsheet.
**Real wall?** Yes — R⁴ inverse falloff.
**Cross-domain wiring:** signal-processing-rf (detection), information-theory-coding (CFAR design).
**Notes:** Skolnik "Introduction to Radar Systems".

### maximum-effective-isotropic-radiated-power (cross-domain alias: `eirp`, `pt-times-gt`, `peak-tx-strength`)
**Domain:** Electromagnetics / Antennas
**Definition:** EIRP = P_t G_t; effective TX power for link/regulatory purposes.
**Atom or composite:** Atom — basic ledger entry.
**Cost model:** Trivial.
**Real wall?** Yes — regulatory caps.
**Cross-domain wiring:** networking (regulatory compliance).
**Notes:** ITU.

### total-radiated-power-trp (cross-domain alias: `trp`, `integrated-radiated-power`, `4-pi-integral-eirp`)
**Domain:** Electromagnetics / Antennas
**Definition:** TRP = ∫EIRP(θ,φ) dΩ/(4π); total power radiated by device.
**Atom or composite:** Composite — pattern integration.
**Cost model:** Sphere-integration OTA.
**Real wall?** Yes — power conservation.
**Cross-domain wiring:** networking (handset OTA tests).
**Notes:** CTIA OTA.

### total-isotropic-sensitivity-tis (cross-domain alias: `tis`, `eis-spherical-avg`, `ota-sensitivity`)
**Domain:** Electromagnetics / Antennas
**Definition:** Spherical-averaged RX sensitivity; counterpart to TRP.
**Atom or composite:** Composite — sphere-integrated sensitivity.
**Cost model:** OTA chamber, hours per device.
**Real wall?** Yes — RX noise floor + antenna eff.
**Cross-domain wiring:** networking (handset RX cert).
**Notes:** CTIA OTA.

### specific-absorption-rate (cross-domain alias: `sar-tissue`, `w-per-kg`, `tissue-power-density`)
**Domain:** Electromagnetics / Antennas
**Definition:** SAR = σ|E|²/(2ρ) W/kg in tissue; regulated for handset/medical EM exposure.
**Atom or composite:** Composite — local power dissipation in tissue.
**Cost model:** FDTD on heterogeneous body model.
**Real wall?** Yes — FCC/ICNIRP limits.
**Cross-domain wiring:** biology-bioinformatics (tissue heating), networking (handset compliance).
**Notes:** ICNIRP 2020.

### radiation-efficiency (cross-domain alias: `eta-rad`, `prad-over-pin`, `ohmic-loss-fraction`)
**Domain:** Electromagnetics / Antennas
**Definition:** η_rad = P_rad/(P_rad + P_loss). 1 − loss fraction.
**Atom or composite:** Atom — energy-conservation ratio.
**Cost model:** Wheeler-cap or directivity/gain comparison.
**Real wall?** Yes — material conductivity.
**Cross-domain wiring:** signal-processing-rf (antenna design), control-numerical-opt (loss minimization).
**Notes:** Wheeler 1959.

### wheeler-cap-method (cross-domain alias: `wheeler-cap`, `radiation-eff-meas`, `cap-shielded-q`)
**Domain:** Electromagnetics / Antennas
**Definition:** Measure Q with/without conducting cap; ratio gives η_rad.
**Atom or composite:** Composite — Q-based efficiency measurement.
**Cost model:** Simple measurement.
**Real wall?** Yes — assumes only loss differs.
**Cross-domain wiring:** signal-processing-rf (small antenna measurement).
**Notes:** Wheeler 1959.

### gain-temperature-figure-system (cross-domain alias: `g-t-system`, `rcv-system-fom`, `db-per-kelvin`)
**Domain:** Electromagnetics / Antennas
**Definition:** Includes T_ant, T_LNA, line losses; defines RX figure of merit.
**Atom or composite:** Composite — Friis cascade + antenna.
**Cost model:** Measured per system.
**Real wall?** Yes — thermal floor.
**Cross-domain wiring:** networking (sat link budgets).
**Notes:** Pratt.

### eirp-mask-regulatory (cross-domain alias: `eirp-mask`, `spectrum-mask`, `spurious-emission-limit`)
**Domain:** Electromagnetics / Antennas
**Definition:** Regulatory limit on EIRP vs frequency offset from band edge.
**Atom or composite:** Composite — frequency-dependent power cap.
**Cost model:** Filter design + back-off.
**Real wall?** Yes — legal limit.
**Cross-domain wiring:** networking (compliance), signal-processing-rf (filter design).
**Notes:** FCC, ETSI.

### log-periodic-toothed-planar (cross-domain alias: `log-periodic-tooth`, `frequency-independent-tooth`, `rumsey-tooth-antenna`)
**Domain:** Electromagnetics / Antennas
**Definition:** Planar self-similar toothed structure; frequency-independent over decade.
**Atom or composite:** Composite — Rumsey-class self-similar antenna.
**Cost model:** Printed; large planar.
**Real wall?** Yes — self-similarity over chosen range.
**Cross-domain wiring:** signal-processing-rf (broadband measurement).
**Notes:** DuHamel 1957.

### conical-spiral-antenna (cross-domain alias: `conical-spiral`, `decade-bw-cp-ant`, `unidirectional-spiral`)
**Domain:** Electromagnetics / Antennas
**Definition:** Spiral wrapped on cone; unidirectional CP over decade BW.
**Atom or composite:** Composite — cone+spiral geometry.
**Cost model:** Larger than planar spiral.
**Real wall?** Yes — frequency-independence within geometry limits.
**Cross-domain wiring:** signal-processing-rf (broadband CP).
**Notes:** Dyson 1959.

### sinuous-antenna (cross-domain alias: `sinuous-ant`, `dual-pol-broadband-ant`, `ew-receive-ant`)
**Domain:** Electromagnetics / Antennas
**Definition:** Self-similar sinuous arms give dual-pol, broadband operation.
**Atom or composite:** Composite — generalized self-similar geometry.
**Cost model:** Full-wave verification.
**Real wall?** Yes — broadband self-similarity.
**Cross-domain wiring:** signal-processing-rf (EW direction finding).
**Notes:** DuHamel 1987.

### bow-tie-antenna (cross-domain alias: `bowtie`, `biconical-print`, `triangle-dipole`)
**Domain:** Electromagnetics / Antennas
**Definition:** Printed planar biconical with triangle arms; broadband dipole-like.
**Atom or composite:** Composite — biconical analog.
**Cost model:** Trivial.
**Real wall?** Yes — broadband via tapered flare.
**Cross-domain wiring:** signal-processing-rf (UWB), networking (low-cost antennas).
**Notes:** Schelkunoff.

### log-periodic-zigzag-antenna (cross-domain alias: `zigzag-lpa`, `wire-lpa-zigzag`, `log-periodic-meander`)
**Domain:** Electromagnetics / Antennas
**Definition:** Zigzag wire LPA; compact alternative to LPDA.
**Atom or composite:** Composite — wire LPA variant.
**Cost model:** Wire-MoM.
**Real wall?** Yes — same band limits as LPDA.
**Cross-domain wiring:** signal-processing-rf (broadband).
**Notes:** DuHamel.

### archimedean-spiral-cavity-backed (cross-domain alias: `cb-spiral`, `absorber-cavity-spiral`, `unidirectional-arch-spiral`)
**Domain:** Electromagnetics / Antennas
**Definition:** Planar Archimedean spiral over absorber-loaded cavity; unidirectional radiation.
**Atom or composite:** Composite — spiral + cavity.
**Cost model:** Cavity adds depth and loss.
**Real wall?** Yes — cavity absorbs back lobe.
**Cross-domain wiring:** signal-processing-rf (avionics, EW).
**Notes:** DuHamel 1958.

### parasitic-array-element-coupling (cross-domain alias: `coupling-coeff-array`, `s-parameter-coupling`, `mutual-coupling-tightness`)
**Domain:** Electromagnetics / Antennas
**Definition:** Coupling strength between elements as S₂₁; affects pattern, BW.
**Atom or composite:** Composite — measured/simulated S-parameter.
**Cost model:** Per-pair simulation.
**Real wall?** Yes — element geometry & spacing.
**Cross-domain wiring:** signal-processing-rf (array calibration).
**Notes:** Hannan.

### scan-blindness (cross-domain alias: `null-in-active-impedance`, `surface-wave-onset`, `array-blind-angle`)
**Domain:** Electromagnetics / Antennas
**Definition:** Scan angle where active reflection coefficient → 1, no power radiates.
**Atom or composite:** Composite — surface-wave resonance in array.
**Cost model:** Floquet analysis.
**Real wall?** Yes — surface-wave mode.
**Cross-domain wiring:** signal-processing-rf (array design avoidance).
**Notes:** Pozar & Schaubert 1984.

### vivaldi-array-tcda (cross-domain alias: `tightly-coupled-dipole-array`, `tcda`, `current-sheet-array`)
**Domain:** Electromagnetics / Antennas
**Definition:** Tightly-coupled dipole array (Vivaldi-like) provides decade-BW wide-scan operation.
**Atom or composite:** Composite — strongly-coupled element array.
**Cost model:** Full-wave unit cell.
**Real wall?** Yes — substrate modes.
**Cross-domain wiring:** signal-processing-rf (wideband AESA).
**Notes:** Munk "Finite Antenna Arrays and FSS"; Wheeler current-sheet 1965.

### connected-array (cross-domain alias: `current-sheet-array`, `dipole-mesh-array`, `coupled-element-array`)
**Domain:** Electromagnetics / Antennas
**Definition:** Array elements electrically connected to neighbors; achieves Wheeler current-sheet behavior → broadband wide-scan.
**Atom or composite:** Composite — element connectivity scheme.
**Cost model:** Full-wave unit cell.
**Real wall?** Yes — element matching and substrate modes.
**Cross-domain wiring:** signal-processing-rf (wideband arrays).
**Notes:** Neto & Lee 2003.

### genetic-algorithm-antenna-design (cross-domain alias: `ga-ant`, `evolutionary-ant`, `parametric-ant-opt`)
**Domain:** Electromagnetics / Antennas
**Definition:** GA optimizes element/shape parameters using full-wave fitness; e.g., NASA ST5 antenna.
**Atom or composite:** Composite — heuristic optimization wrapping CEM solver.
**Cost model:** N_pop · N_gen full-wave runs.
**Real wall?** No — exploration heuristic.
**Cross-domain wiring:** combinatorial-optimization (GA), ml-training (search).
**Notes:** Hornby et al. 2006 (NASA ST5).

### topology-optimization-antenna (cross-domain alias: `topology-opt-ant`, `density-method-em`, `simp-em`)
**Domain:** Electromagnetics / Antennas
**Definition:** Continuous material-density field optimized for pattern/Q; SIMP penalization → 0/1 final geometry.
**Atom or composite:** Composite — PDE-constrained optimization.
**Cost model:** Adjoint per iteration; many iterations.
**Real wall?** No — design freedom.
**Cross-domain wiring:** control-numerical-opt (PDE-constrained opt), graphics-rendering-lod (level sets).
**Notes:** Bendsøe & Sigmund.

### adjoint-method-em (cross-domain alias: `em-adjoint`, `pde-adjoint-grad`, `gradient-from-two-sims`)
**Domain:** Electromagnetics / Antennas
**Definition:** Gradient w.r.t. design from forward+adjoint Maxwell solves; O(1) cost per gradient.
**Atom or composite:** Atom — adjoint of Maxwell operator.
**Cost model:** 2× forward-solve per gradient.
**Real wall?** No — formal duality.
**Cross-domain wiring:** ml-training (backprop analog), control-numerical-opt (adjoint methods), photonics-optics (inverse design).
**Notes:** Veronis 2004; Lalau-Keraly 2013.

### inverse-design-photonic (cross-domain alias: `topology-photonic`, `metasurface-inverse-design`, `meep-adjoint`)
**Domain:** Electromagnetics / Antennas
**Definition:** Topology / shape optimization for photonic devices via adjoint FDTD.
**Atom or composite:** Composite — adjoint + topology + photonics.
**Cost model:** Hundreds of FDTD runs.
**Real wall?** No — manufacturing constraints.
**Cross-domain wiring:** photonics-optics (silicon photonics), ml-training (generative design).
**Notes:** Molesky 2018 review.

### radar-absorber-tessellation (cross-domain alias: `pyramidal-absorber`, `wedge-absorber`, `convoluted-foam`)
**Domain:** Electromagnetics / Antennas
**Definition:** Geometric absorber profile gradually matches free-space → highly-loaded foam.
**Atom or composite:** Composite — geometric impedance taper.
**Cost model:** Lab-bench absorber sheets.
**Real wall?** Yes — absorber thickness vs. low-f reflection.
**Cross-domain wiring:** signal-processing-rf (chamber design).
**Notes:** Knott Ch. 6.

### radar-absorbing-structure-ras (cross-domain alias: `ras`, `structural-absorber`, `composite-ras`)
**Domain:** Electromagnetics / Antennas
**Definition:** Composite material with embedded absorber (e.g. ferrite-loaded honeycomb) doubles as structural.
**Atom or composite:** Composite — structural + RAM.
**Cost model:** Aerospace cost.
**Real wall?** Yes — Rozanov bound.
**Cross-domain wiring:** signal-processing-rf (stealth airframes).
**Notes:** Vinoy & Jha.

### rozanov-bandwidth-thickness-bound (cross-domain alias: `rozanov`, `thickness-bw-trade-absorber`, `integrated-reflection`)
**Domain:** Electromagnetics / Antennas
**Definition:** ∫₀^∞ ln|R(λ)| dλ ≥ −2π² μ_s d. Bound on absorber thickness-bandwidth.
**Atom or composite:** Atom — KK-type integral constraint.
**Cost model:** Free design law.
**Real wall?** Yes — strict causality bound.
**Cross-domain wiring:** information-theory-coding (channel bandwidth bounds).
**Notes:** Rozanov 2000.

### woodward-lawson-synthesis (cross-domain alias: `wls-pattern-synthesis`, `sampling-method-array`, `point-by-point-pattern`)
**Domain:** Electromagnetics / Antennas
**Definition:** Sample desired pattern at specific angles → weights via inverse DFT-like sum.
**Atom or composite:** Composite — sampling-based array synthesis.
**Cost model:** Closed-form; finite ripple between samples.
**Real wall?** No — sampling tradeoff.
**Cross-domain wiring:** signal-processing-rf (windowing), retrieval-search (sampling).
**Notes:** Woodward & Lawson 1948.

### fourier-pattern-synthesis (cross-domain alias: `fourier-array-design`, `dft-array-weights`, `continuous-aperture-design`)
**Domain:** Electromagnetics / Antennas
**Definition:** Aperture distribution = IFT of desired pattern; sampled at element positions.
**Atom or composite:** Composite — spatial-frequency design.
**Cost model:** O(N log N) FFT.
**Real wall?** Yes — finite aperture truncates ideal pattern.
**Cross-domain wiring:** signal-processing-rf (FIR analog), retrieval-search (Fourier features).
**Notes:** Balanis §7.6.

### orchard-elliott-synthesis (cross-domain alias: `orchard-method`, `controlled-null-synthesis`, `flexible-null-placement`)
**Domain:** Electromagnetics / Antennas
**Definition:** Iteratively place nulls/peaks at chosen angles using polynomial root manipulation.
**Atom or composite:** Composite — root manipulation in z-plane.
**Cost model:** Iterative; converges quickly.
**Real wall?** No — design freedom.
**Cross-domain wiring:** signal-processing-rf (null steering).
**Notes:** Orchard et al. 1985.

### iterative-projection-method (cross-domain alias: `alternating-projection-ant`, `array-mask-iteration`, `gerchberg-saxton-em`)
**Domain:** Electromagnetics / Antennas
**Definition:** Project alternately onto pattern-mask and feasible-weights sets to find compliant weights.
**Atom or composite:** Composite — alternating-projection optimization.
**Cost model:** Iterative; FFT per step.
**Real wall?** No — local convergence.
**Cross-domain wiring:** retrieval-search (Gerchberg-Saxton phase retrieval), control-numerical-opt (alternating projection).
**Notes:** Bucci et al.

### convex-optimization-array (cross-domain alias: `cvx-pattern-synth`, `lp-min-max-sll`, `socp-array-design`)
**Domain:** Electromagnetics / Antennas
**Definition:** Min-max SLL with linear amplitude constraints solved as LP / SOCP.
**Atom or composite:** Composite — convex-programming formulation.
**Cost model:** Polynomial; modern solvers handle 1000s of elements.
**Real wall?** No — global optimum within convex relaxation.
**Cross-domain wiring:** control-numerical-opt (convex), combinatorial-optimization (LP).
**Notes:** Lebret & Boyd 1997.

### deep-learning-antenna-design (cross-domain alias: `dl-ant-design`, `surrogate-em-nn`, `generative-antenna`)
**Domain:** Electromagnetics / Antennas
**Definition:** NN surrogates predict S-params/pattern from geometry; generative design uses GANs/diffusion.
**Atom or composite:** Composite — ML surrogate + optimizer.
**Cost model:** Training expensive; inference cheap.
**Real wall?** No — generalization beyond training distribution unreliable.
**Cross-domain wiring:** ml-training (surrogates), control-numerical-opt (Bayesian opt).
**Notes:** Khan 2020 review.

### bayesian-optimization-antenna (cross-domain alias: `bo-ant`, `gp-surrogate-em`, `gp-acquisition`)
**Domain:** Electromagnetics / Antennas
**Definition:** GP surrogate + acquisition function (EI, UCB) chooses next full-wave runs.
**Atom or composite:** Composite — GP + sequential-decision-making.
**Cost model:** Few dozen full-wave runs.
**Real wall?** No — sample-efficient under smoothness assumptions.
**Cross-domain wiring:** ml-training (BO), control-numerical-opt (sequential decision).
**Notes:** Chen et al. 2018.

### spherical-mode-expansion (cross-domain alias: `sme`, `vector-spherical-harmonics`, `tnm-modes`)
**Domain:** Electromagnetics / Antennas
**Definition:** Radiated field expanded in vector spherical harmonics; modes orthogonal.
**Atom or composite:** Composite — completeness of TM/TE_nm.
**Cost model:** Truncate at N = ka + few.
**Real wall?** Yes — small antenna implies finite modes.
**Cross-domain wiring:** quantum-computing (angular momentum), graphics-rendering-lod (SH lighting).
**Notes:** Hansen ed.

### radiation-q-factor (cross-domain alias: `q-rad-ant`, `bw-of-small-ant`, `chu-q-modal`)
**Domain:** Electromagnetics / Antennas
**Definition:** Q = ω·(stored energy)/(radiated power); BW ≈ 1/Q for matched antenna.
**Atom or composite:** Atom — energy-balance ratio.
**Cost model:** Surface-integration of fields outside Chu sphere.
**Real wall?** Yes — Chu lower bound.
**Cross-domain wiring:** information-theory-coding (BW-volume bounds).
**Notes:** Yaghjian & Best 2005.

### gustafsson-bound (cross-domain alias: `gustafsson-jonsson-bound`, `q-bw-bound-anisotropic`, `polarizability-bound`)
**Domain:** Electromagnetics / Antennas
**Definition:** Q·BW bound for arbitrary antenna in arbitrary shape via static polarizability tensor.
**Atom or composite:** Atom — KK + sum rule for finite scatterer.
**Cost model:** Polarizability via static solver.
**Real wall?** Yes — Sum-rule consequence of causality.
**Cross-domain wiring:** information-theory-coding (limit theorems).
**Notes:** Gustafsson, Sohl & Kristensson 2007.

### thal-2009-bound (cross-domain alias: `thal-bound`, `surface-antenna-q`, `internal-energy-bound`)
**Domain:** Electromagnetics / Antennas
**Definition:** Includes interior modes; tighter Q bound for hollow surface antennas.
**Atom or composite:** Atom — refined Chu bound.
**Cost model:** Free.
**Real wall?** Yes.
**Cross-domain wiring:** information-theory-coding (bounds).
**Notes:** Thal 2009.

### radiation-pattern-multiplication (cross-domain alias: `pattern-mult-theorem`, `element-times-af`, `decomposition-pattern`)
**Domain:** Electromagnetics / Antennas
**Definition:** Total pattern of identical-element array = element pattern × array factor.
**Atom or composite:** Atom — factorization theorem.
**Cost model:** Free.
**Real wall?** Yes — only for identical elements w/o mutual coupling.
**Cross-domain wiring:** signal-processing-rf (separable analysis).
**Notes:** Balanis §6.

### embedded-element-pattern (cross-domain alias: `eep`, `active-element-pattern`, `scan-element-pattern`)
**Domain:** Electromagnetics / Antennas
**Definition:** Pattern of one element with all others terminated; captures mutual coupling.
**Atom or composite:** Composite — N full-wave runs per array.
**Cost model:** Heavy but reusable.
**Real wall?** Yes — physical observable.
**Cross-domain wiring:** signal-processing-rf (calibrated arrays).
**Notes:** Hannan 1964.

### floquet-port-analysis (cross-domain alias: `unit-cell-port`, `infinite-array-analysis`, `floquet-modes-port`)
**Domain:** Electromagnetics / Antennas
**Definition:** Periodic BC + Floquet harmonic ports; gives infinite-array element response.
**Atom or composite:** Composite — periodic-solver port.
**Cost model:** Single unit-cell solve.
**Real wall?** Yes — assumes infinite periodicity.
**Cross-domain wiring:** signal-processing-rf (array design).
**Notes:** HFSS Floquet port.

### array-gain-realized-vs-ideal (cross-domain alias: `realized-array-gain`, `mismatch-array-loss`, `g-realized-array`)
**Domain:** Electromagnetics / Antennas
**Definition:** G_realized = G_ideal · (1 − |Γ_active|²); function of scan angle.
**Atom or composite:** Composite — accounts for active mismatch.
**Cost model:** Per scan angle.
**Real wall?** Yes — physical reflection.
**Cross-domain wiring:** signal-processing-rf (array realized gain).
**Notes:** Pozar.

### array-quantization-sidelobes (cross-domain alias: `quantization-lobes`, `phase-shifter-bit-error`, `digital-array-noise`)
**Domain:** Electromagnetics / Antennas
**Definition:** Discrete phase shifters → quantization error → raised sidelobes.
**Atom or composite:** Composite — quantization-noise-like leakage.
**Cost model:** SLL ≈ 1/(M·2^(2b)) for b-bit, M elements.
**Real wall?** Yes — number of bits sets floor.
**Cross-domain wiring:** signal-processing-rf (quantization noise), information-theory-coding (PCM analog).
**Notes:** Mailloux.

### subarray-tiled-beam-error (cross-domain alias: `tile-beam-pointing-error`, `subarray-grating`, `tile-quantization`)
**Domain:** Electromagnetics / Antennas
**Definition:** Tile-level phase shifters cause sub-array grating lobes + beam-pointing error.
**Atom or composite:** Composite — geometric quantization.
**Cost model:** Closed-form analysis.
**Real wall?** Yes — physical constraint.
**Cross-domain wiring:** signal-processing-rf (tiled AESA).
**Notes:** Mailloux Ch. 7.

### electronically-reconfigurable-array (cross-domain alias: `era`, `reconfig-aesa`, `pattern-reconfigurable`)
**Domain:** Electromagnetics / Antennas
**Definition:** Array whose pattern/polarization/freq reconfigured electronically.
**Atom or composite:** Composite — tunable elements + control.
**Cost model:** Element + control complexity.
**Real wall?** No — design space.
**Cross-domain wiring:** networking (cognitive radio), signal-processing-rf (multi-mission arrays).
**Notes:** Bernhard "Reconfigurable Antennas".

### pattern-reconfigurable-antenna (cross-domain alias: `pattern-recon-ant`, `pin-diode-pattern-switch`, `switched-element`)
**Domain:** Electromagnetics / Antennas
**Definition:** PIN/varactor switches select pattern direction without phased array.
**Atom or composite:** Composite — switched element/parasitic.
**Cost model:** Few discrete states.
**Real wall?** Yes — discrete state count.
**Cross-domain wiring:** networking (MIMO + reconfig).
**Notes:** Bernhard.

### frequency-reconfigurable-antenna (cross-domain alias: `freq-recon-ant`, `varactor-tuned-patch`, `mems-band-switch`)
**Domain:** Electromagnetics / Antennas
**Definition:** Resonant frequency electronically tuned (varactor, MEMS, PIN).
**Atom or composite:** Composite — tunable resonator.
**Cost model:** Tuning element loss.
**Real wall?** Yes — tuning element Q.
**Cross-domain wiring:** networking (cognitive radio), signal-processing-rf (multi-band).
**Notes:** Bernhard.

### polarization-reconfigurable-antenna (cross-domain alias: `pol-recon-ant`, `switched-pol`, `lhcp-rhcp-switch`)
**Domain:** Electromagnetics / Antennas
**Definition:** Selectable polarization via switches/phase shifters.
**Atom or composite:** Composite — switched feed.
**Cost model:** Switch losses.
**Real wall?** Yes — switch isolation.
**Cross-domain wiring:** networking (satcom), signal-processing-rf (dual-pol).
**Notes:** Bernhard.

### terahertz-photoconductive-antenna (cross-domain alias: `thz-pca`, `pulsed-thz-source`, `auston-switch`)
**Domain:** Electromagnetics / Antennas
**Definition:** Femtosecond laser pulse on biased semiconductor antenna → ps-duration THz radiation.
**Atom or composite:** Composite — optical pump + dipole antenna.
**Cost model:** Optics-grade setup.
**Real wall?** Yes — material carrier lifetime sets BW.
**Cross-domain wiring:** photonics-optics (ultrafast), signal-processing-rf (THz spectroscopy).
**Notes:** Auston 1975.

### photoconductive-mixer (cross-domain alias: `pc-mixer-thz`, `cw-thz-source`, `lt-gaas-mixer`)
**Domain:** Electromagnetics / Antennas
**Definition:** Two-laser optical beat on biased photoconductor produces CW THz.
**Atom or composite:** Composite — photomixing source.
**Cost model:** Optical infrastructure.
**Real wall?** Yes — material BW.
**Cross-domain wiring:** photonics-optics (homodyne), signal-processing-rf (THz CW).
**Notes:** Brown et al. 1995.

### quasi-optical-system (cross-domain alias: `qo-system`, `gaussian-beam-mw`, `lens-mirror-mw`)
**Domain:** Electromagnetics / Antennas
**Definition:** Microwave/THz propagation modeled with Gaussian beams + lenses/mirrors.
**Atom or composite:** Composite — Gaussian-beam optics applied to RF.
**Cost model:** ABCD-matrix beam propagation.
**Real wall?** Yes — paraxial limit.
**Cross-domain wiring:** photonics-optics (Gaussian beams), signal-processing-rf (mmWave imaging).
**Notes:** Goldsmith "Quasioptical Systems".

### gaussian-beam-em (cross-domain alias: `gb-em`, `paraxial-em-mode`, `w0-beam-waist`)
**Domain:** Electromagnetics / Antennas
**Definition:** Paraxial Helmholtz solution with Gaussian transverse profile, Rayleigh range z_R = πw₀²/λ.
**Atom or composite:** Atom — paraxial eigenmode.
**Cost model:** Closed-form.
**Real wall?** Yes — paraxial limit; diffraction at small w₀.
**Cross-domain wiring:** photonics-optics (lasers), signal-processing-rf (mmWave beams).
**Notes:** Kogelnik & Li 1966.

### hermite-gaussian-modes (cross-domain alias: `hg-modes`, `rect-symmetry-modes`, `tem-mn-laser-modes`)
**Domain:** Electromagnetics / Antennas
**Definition:** Higher-order paraxial modes with rectangular symmetry, indexed by m, n.
**Atom or composite:** Composite — generalized Gaussian basis.
**Cost model:** Truncate to N modes.
**Real wall?** Yes — orthogonal modal basis.
**Cross-domain wiring:** quantum-computing (HG mode qudits), photonics-optics (laser modes).
**Notes:** Siegman "Lasers".

### laguerre-gaussian-modes (cross-domain alias: `lg-modes`, `circular-symmetric-modes`, `oam-l-p-modes`)
**Domain:** Electromagnetics / Antennas
**Definition:** Higher-order paraxial modes with cylindrical symmetry; carry OAM.
**Atom or composite:** Composite — alternative paraxial basis.
**Cost model:** Truncate at L.
**Real wall?** Yes — orthogonal basis.
**Cross-domain wiring:** quantum-computing (OAM qudits), photonics-optics (vortex beams).
**Notes:** Allen et al. 1992.

### sommerfeld-arnold-radiation-condition (cross-domain alias: `radiation-condition`, `outgoing-wave-bc`, `silver-muller-rc`)
**Domain:** Electromagnetics / Antennas
**Definition:** Far-field condition ensuring uniqueness: only outgoing waves at infinity.
**Atom or composite:** Atom — uniqueness condition for Helmholtz/Maxwell.
**Cost model:** Free constraint.
**Real wall?** Yes — uniqueness theorem requirement.
**Cross-domain wiring:** formal-verification (PDE uniqueness), control-numerical-opt (open-region BCs).
**Notes:** Sommerfeld; Silver-Müller for vector case.

### perfectly-conducting-limit (cross-domain alias: `pec-limit`, `sigma-to-infinity`, `boundary-current-only`)
**Domain:** Electromagnetics / Antennas
**Definition:** σ → ∞ idealization; tangential E = 0 on surface.
**Atom or composite:** Atom — limit case BC.
**Cost model:** Free.
**Real wall?** No — idealization; surface impedance for real metals.
**Cross-domain wiring:** computational-geometry (surface meshing).
**Notes:** Standard textbook idealization.

### surface-impedance-boundary-condition (cross-domain alias: `sibc`, `leontovich-bc`, `eta-s-bc`)
**Domain:** Electromagnetics / Antennas
**Definition:** E_tan = Z_s · (n̂ × H_tan). Approximates lossy conductor without resolving skin depth.
**Atom or composite:** Composite — local impedance BC.
**Cost model:** Avoids meshing skin depth.
**Real wall?** Yes when skin depth ≪ structure.
**Cross-domain wiring:** physics-diffusion (transport BC), control-numerical-opt (reduced-order BCs).
**Notes:** Leontovich 1948.

### thin-wire-approximation (cross-domain alias: `pocklington`, `hallen-eq`, `wire-mom`)
**Domain:** Electromagnetics / Antennas
**Definition:** Reduce wire-antenna integral equation to 1D on axis assuming a ≪ λ.
**Atom or composite:** Composite — thin-wire model.
**Cost model:** N-segment MoM; N small.
**Real wall?** Yes — a/λ < 1/100.
**Cross-domain wiring:** linear-algebra-matrix (Toeplitz-like systems), networking (HF wire antennas).
**Notes:** Pocklington 1897; Hallén 1938.

### nec-2-numerical-electromagnetics (cross-domain alias: `nec2`, `wire-mom-code`, `eznec`)
**Domain:** Electromagnetics / Antennas
**Definition:** Method of moments for wire antennas with surface patches; HAM-radio standard.
**Atom or composite:** Composite — thin-wire MoM implementation.
**Cost model:** O(N²) memory; fast for HAM antennas.
**Real wall?** Yes — thin-wire assumption.
**Cross-domain wiring:** signal-processing-rf (HAM antenna design).
**Notes:** Burke 1981 (LLNL).

### scattering-matrix-microwave (cross-domain alias: `s-matrix-mw`, `n-port-s-params`, `power-wave-rep`)
**Domain:** Electromagnetics / Antennas
**Definition:** Relates incoming (a) and outgoing (b) wave amplitudes at each port: b = S·a.
**Atom or composite:** Atom — port-based representation.
**Cost model:** Measured by VNA.
**Real wall?** Yes — observable.
**Cross-domain wiring:** signal-processing-rf (microwave engineering), quantum-computing (scattering theory).
**Notes:** Kurokawa 1965.

### abcd-matrix-microwave (cross-domain alias: `transmission-matrix`, `2-port-cascade`, `chain-matrix`)
**Domain:** Electromagnetics / Antennas
**Definition:** [V₁; I₁] = [A B; C D] [V₂; −I₂]; cascade by matrix product.
**Atom or composite:** Atom — 2-port transmission rep.
**Cost model:** O(N) matmul for chain.
**Real wall?** No — algebraic transform.
**Cross-domain wiring:** linear-algebra-matrix (chain product), signal-processing-rf (filter cascade).
**Notes:** Pozar §4.

### s-to-abcd-conversion (cross-domain alias: `s-abcd-mapping`, `parameter-translation`, `2-port-convert`)
**Domain:** Electromagnetics / Antennas
**Definition:** Algebraic mapping between [S], [ABCD], [Z], [Y] for 2-port networks.
**Atom or composite:** Atom — equivalent representations.
**Cost model:** Trivial.
**Real wall?** No — formal change-of-basis.
**Cross-domain wiring:** linear-algebra-matrix (representations).
**Notes:** Pozar §4.

### power-waves-vs-pseudo-waves (cross-domain alias: `kurokawa-waves`, `incident-reflected-power`, `general-z0`)
**Domain:** Electromagnetics / Antennas
**Definition:** Power-wave definitions of a, b consistent with arbitrary complex Z₀; pseudo-waves for measurement.
**Atom or composite:** Atom — convention choice.
**Cost model:** Free.
**Real wall?** No — convention.
**Cross-domain wiring:** signal-processing-rf (negative-resistance amps), control-numerical-opt (passivity).
**Notes:** Kurokawa 1965; Marks & Williams 1992.

### unilateral-figure-of-merit (cross-domain alias: `u-fom`, `device-unilateral-gain`, `s12-zero-bound`)
**Domain:** Electromagnetics / Antennas
**Definition:** U = |S₁₂S₂₁|/(1−|S₁₁|²)(1−|S₂₂|²); 0 = unilateral, 1 = fully bilateral.
**Atom or composite:** Atom — device classification.
**Cost model:** Trivial.
**Real wall?** No — design metric.
**Cross-domain wiring:** signal-processing-rf (amp stability analysis).
**Notes:** Mason 1954.

### rollett-stability-factor (cross-domain alias: `k-rollett`, `stability-k`, `mu-stability`)
**Domain:** Electromagnetics / Antennas
**Definition:** K = (1 − |S₁₁|² − |S₂₂|² + |Δ|²)/(2|S₁₂S₂₁|); K>1, |Δ|<1 → unconditionally stable.
**Atom or composite:** Atom — stability criterion.
**Cost model:** Trivial.
**Real wall?** Yes — oscillation onset.
**Cross-domain wiring:** signal-processing-rf (amp stability), control-numerical-opt (Nyquist criterion).
**Notes:** Rollett 1962.

### noise-figure-friis-cascade (cross-domain alias: `friis-noise`, `nf-cascade`, `f1-plus-f2-minus-1-over-g1`)
**Domain:** Electromagnetics / Antennas
**Definition:** F = F₁ + (F₂−1)/G₁ + (F₃−1)/(G₁G₂) + …
**Atom or composite:** Atom — cascade noise budget.
**Cost model:** Trivial.
**Real wall?** Yes — first-stage noise dominates RX.
**Cross-domain wiring:** networking (RX design), information-theory-coding (SNR limit).
**Notes:** Friis 1944.

### intermodulation-distortion-im3 (cross-domain alias: `im3`, `ip3-third-order-intercept`, `psat-im3`)
**Domain:** Electromagnetics / Antennas
**Definition:** Third-order nonlinear products at 2f₁−f₂, 2f₂−f₁; extrapolated IP3.
**Atom or composite:** Atom — nonlinear-distortion product.
**Cost model:** 2-tone measurement.
**Real wall?** Yes — fundamental device nonlinearity.
**Cross-domain wiring:** signal-processing-rf (RX dynamic range), information-theory-coding (linearity-rate tradeoff).
**Notes:** Pedro & Carvalho "Intermodulation Distortion in Microwave and Wireless Circuits".

### phase-noise-leeson-model (cross-domain alias: `leeson-pn`, `oscillator-pn`, `flicker-corner`)
**Domain:** Electromagnetics / Antennas
**Definition:** L(f_m) decomposition: 1/f³ flicker + 1/f² thermal + thermal floor + flicker corner.
**Atom or composite:** Composite — empirical phase-noise model.
**Cost model:** Fit to measured spectrum.
**Real wall?** Yes — physical noise sources.
**Cross-domain wiring:** signal-processing-rf (LO PN), control-numerical-opt (allan variance).
**Notes:** Leeson 1966.

### allan-variance (cross-domain alias: `sigma-y-tau`, `oscillator-stability`, `time-domain-stability`)
**Domain:** Electromagnetics / Antennas
**Definition:** σ_y²(τ) = ½⟨(y_{n+1} − y_n)²⟩; tells you which noise regime dominates at τ.
**Atom or composite:** Atom — time-domain stability metric.
**Cost model:** O(N log N) per τ.
**Real wall?** Yes — oscillator noise.
**Cross-domain wiring:** signal-processing-rf (clock-stability), statistics-probability (noise statistics).
**Notes:** Allan 1966.

### dispersion-engineering-photonic (cross-domain alias: `dispersion-eng`, `nl-dispersion-design`, `soliton-dispersion`)
**Domain:** Electromagnetics / Antennas
**Definition:** Engineer β(ω) of waveguide for desired group-velocity dispersion.
**Atom or composite:** Composite — waveguide-mode dispersion design.
**Cost model:** Mode-solver per design iteration.
**Real wall?** Yes — material + geometric dispersion.
**Cross-domain wiring:** photonics-optics (solitons), signal-processing-rf (true-time-delay analog).
**Notes:** Foster et al. 2008.

### causal-dispersive-fdtd-z-transform (cross-domain alias: `z-transform-fdtd`, `discrete-time-dispersive`, `digital-filter-fdtd`)
**Domain:** Electromagnetics / Antennas
**Definition:** Implement dispersive ε(ω) via discrete-time IIR filter in FDTD update.
**Atom or composite:** Composite — z-transform of ADE.
**Cost model:** Few state vars per cell.
**Real wall?** Yes — discrete causality.
**Cross-domain wiring:** signal-processing-rf (IIR filter), control-numerical-opt (digital filtering).
**Notes:** Sullivan 1992.

### convolution-pml-cpml (cross-domain alias: `cpml`, `convolutional-pml`, `roden-gedney`)
**Domain:** Electromagnetics / Antennas
**Definition:** PML implemented via convolution updates with stretched-coordinate factors.
**Atom or composite:** Composite — PML variant; better at grazing.
**Cost model:** Few aux vars per PML cell.
**Real wall?** No — refinement of PML.
**Cross-domain wiring:** physics-diffusion (open BCs), control-numerical-opt (absorbing layers).
**Notes:** Roden & Gedney 2000.

### total-field-scattered-field-formulation (cross-domain alias: `tfsf`, `huygens-source-fdtd`, `plane-wave-source-fdtd`)
**Domain:** Electromagnetics / Antennas
**Definition:** FDTD region split into total-field (includes incident) and scattered-field (no incident); equivalent-surface source on boundary.
**Atom or composite:** Composite — TF/SF interface in FDTD.
**Cost model:** Adds source surface; clean incident-field injection.
**Real wall?** No — formulation.
**Cross-domain wiring:** signal-processing-rf (scattering computation).
**Notes:** Mur 1981; Taflove §5.

### huygens-surface-equivalence (cross-domain alias: `huygens-source`, `equivalent-currents-on-surface`, `near-to-far-field-transform`)
**Domain:** Electromagnetics / Antennas
**Definition:** Tangential E, H on closed surface fully determine exterior field via equivalent currents.
**Atom or composite:** Composite — Love's equivalence + near-to-far transform.
**Cost model:** Surface-only integration.
**Real wall?** Yes — uniqueness theorem.
**Cross-domain wiring:** signal-processing-rf (near-to-far transform).
**Notes:** Love 1901; Schelkunoff 1936.

### gain-versus-Q-tradeoff (cross-domain alias: `gain-q-bound`, `harrington-superdirectivity`, `superdirective-array`)
**Domain:** Electromagnetics / Antennas
**Definition:** Superdirective arrays have D > N but high Q → narrow BW + ohmic loss sensitivity.
**Atom or composite:** Atom — tradeoff between D and BW.
**Cost model:** Free design law.
**Real wall?** Yes — Harrington's gain-Q product.
**Cross-domain wiring:** signal-processing-rf (superdirectivity), information-theory-coding (gain-BW).
**Notes:** Harrington 1960.

### hansen-woodyard-end-fire (cross-domain alias: `hw-condition`, `increased-end-fire-directivity`, `progressive-phase-superdirective`)
**Domain:** Electromagnetics / Antennas
**Definition:** End-fire with extra phase |α| = kd + π/N gives D_HW > D_uniform_end_fire.
**Atom or composite:** Composite — modified end-fire phasing.
**Cost model:** Same as end-fire.
**Real wall?** Yes — superdirective penalty (SLL, BW).
**Cross-domain wiring:** signal-processing-rf (end-fire optimization).
**Notes:** Hansen & Woodyard 1938.

### scan-loss-cos-theta (cross-domain alias: `scan-loss`, `cos-theta-array-gain-loss`, `projected-aperture`)
**Domain:** Electromagnetics / Antennas
**Definition:** Array gain decreases by cos θ_scan due to projected aperture.
**Atom or composite:** Atom — geometric projection.
**Cost model:** Free.
**Real wall?** Yes — geometric.
**Cross-domain wiring:** signal-processing-rf (radar coverage), networking (sat tracking).
**Notes:** Balanis §6.

### radiation-impedance-mutual (cross-domain alias: `induced-emf-method`, `mutual-z-dipoles`, `carter-curves`)
**Domain:** Electromagnetics / Antennas
**Definition:** Mutual impedance between dipoles from induced-EMF method; Carter curves tabulated.
**Atom or composite:** Composite — integral over coupled currents.
**Cost model:** Closed-form / tabulated.
**Real wall?** Yes — physical mutual coupling.
**Cross-domain wiring:** signal-processing-rf (array design).
**Notes:** Carter 1932.

### array-pattern-with-coupling (cross-domain alias: `coupling-corrected-pattern`, `embedded-pattern-sum`, `active-pattern-sum`)
**Domain:** Electromagnetics / Antennas
**Definition:** Total pattern = sum of weighted embedded-element patterns, not weighted isolated patterns.
**Atom or composite:** Composite — corrected superposition.
**Cost model:** N-fold pre-computed patterns.
**Real wall?** Yes — coupling effects.
**Cross-domain wiring:** signal-processing-rf (calibration).
**Notes:** Hannan 1964.

### tapered-amplitude-array (cross-domain alias: `amp-taper-array`, `cosine-taper`, `hamming-array`)
**Domain:** Electromagnetics / Antennas
**Definition:** Amplitudes weighted by smooth window (cosine, Hamming, Hann); lowers SLL at HPBW cost.
**Atom or composite:** Composite — windowed array.
**Cost model:** Trivial weight set.
**Real wall?** Yes — fundamental SLL-HPBW tradeoff.
**Cross-domain wiring:** signal-processing-rf (DSP windows).
**Notes:** Harris 1978.

### grating-array-radar (cross-domain alias: `slotted-waveguide-array`, `traveling-wave-array`, `serial-fed-array`)
**Domain:** Electromagnetics / Antennas
**Definition:** Slots cut in waveguide radiate; spacing sets beam direction.
**Atom or composite:** Composite — traveling-wave-fed array.
**Cost model:** Mature design tables.
**Real wall?** Yes — frequency squint.
**Cross-domain wiring:** signal-processing-rf (airborne radar).
**Notes:** Elliott "Antenna Theory and Design".

### dispersive-true-time-delay-photonic (cross-domain alias: `photonic-ttd`, `fiber-delay-line`, `chirped-grating-ttd`)
**Domain:** Electromagnetics / Antennas
**Definition:** Use chirped fiber Bragg gratings or fiber delay to provide GHz-of-BW TTD.
**Atom or composite:** Composite — photonic component for RF TTD.
**Cost model:** Photonic packaging cost.
**Real wall?** Yes — RF-photonic conversion loss.
**Cross-domain wiring:** photonics-optics (delay lines), signal-processing-rf (wideband phased arrays).
**Notes:** Capmany & Novak 2007.

### electromagnetic-cosimulation (cross-domain alias: `em-circuit-cosim`, `eda-em-loop`, `multiphysics-em`)
**Domain:** Electromagnetics / Antennas
**Definition:** Couple EM solver to circuit simulator iteratively; e.g., antenna + nonlinear front-end.
**Atom or composite:** Composite — coupled-domain simulation.
**Cost model:** Iterative; per-iteration EM run.
**Real wall?** No — solver-coupling tradeoffs.
**Cross-domain wiring:** signal-processing-rf (system design), control-numerical-opt (multiphysics).
**Notes:** Keysight ADS, ANSYS HFSS-Circuit.

### tpu-fdtd (cross-domain alias: `fdtd-on-tpu`, `dense-matrix-em`, `xla-fdtd`)
**Domain:** Electromagnetics / Antennas
**Definition:** Map Yee updates onto TPU systolic arrays as dense linear ops.
**Atom or composite:** Composite — Yee + XLA.
**Cost model:** Excellent TF/s; memory-bandwidth bound.
**Real wall?** Yes — TPU memory size.
**Cross-domain wiring:** ml-training (TPU), graphics-rendering-lod (large grids).
**Notes:** Hughes 2019.

### spectral-element-method-em (cross-domain alias: `sem-em`, `high-order-em`, `gll-em-basis`)
**Domain:** Electromagnetics / Antennas
**Definition:** High-order polynomial basis on hex elements with GLL nodes; exponential convergence in smooth regions.
**Atom or composite:** Composite — high-order FEM.
**Cost model:** Few elements with high p; sparse but dense per-element block.
**Real wall?** Yes — element-level CFL.
**Cross-domain wiring:** control-numerical-opt (spectral methods), graphics-rendering-lod (high-order).
**Notes:** Liu 1998.

### pseudospectral-fdtd (cross-domain alias: `ps-fdtd`, `fourier-fdtd`, `chebyshev-fdtd`)
**Domain:** Electromagnetics / Antennas
**Definition:** Replace finite-difference spatial derivatives with FFT-based pseudospectral derivatives.
**Atom or composite:** Composite — Yee in time, spectral in space.
**Cost model:** O(N log N) per step; high accuracy per cell.
**Real wall?** Yes — global periodicity assumption.
**Cross-domain wiring:** signal-processing-rf (spectral derivatives), control-numerical-opt (spectral PDE).
**Notes:** Liu 1997.

### macromodel-extraction (cross-domain alias: `state-space-em`, `vector-fitting`, `rational-fit-s-params`)
**Domain:** Electromagnetics / Antennas
**Definition:** Fit measured/simulated S-parameters with pole-residue rational model for time-domain simulation.
**Atom or composite:** Composite — model order reduction.
**Cost model:** SVD + least-squares.
**Real wall?** Yes — passivity & causality enforcement.
**Cross-domain wiring:** control-numerical-opt (model reduction), signal-processing-rf (channel modeling).
**Notes:** Gustavsen & Semlyen 1999 vector fitting.

### waveform-shaping-radar (cross-domain alias: `radar-waveform-design`, `chirp-design`, `costas-codes`)
**Domain:** Electromagnetics / Antennas
**Definition:** Design TX waveform (chirp, Costas, Barker) for desired ambiguity function.
**Atom or composite:** Composite — waveform-design problem.
**Cost model:** Optimization over waveform space.
**Real wall?** Yes — Heisenberg-like time-BW limit.
**Cross-domain wiring:** signal-processing-rf (matched filter), information-theory-coding (sequences).
**Notes:** Levanon & Mozeson "Radar Signals".

### ambiguity-function-radar (cross-domain alias: `chi-tau-fd`, `radar-ambiguity`, `range-doppler-ambiguity`)
**Domain:** Electromagnetics / Antennas
**Definition:** χ(τ, f_d) = ∫ s(t)·s*(t−τ)·exp(j2π f_d t) dt. Range-Doppler resolution shape.
**Atom or composite:** Atom — Woodward ambiguity.
**Cost model:** 2D FFT.
**Real wall?** Yes — time-BW bound.
**Cross-domain wiring:** signal-processing-rf (radar resolution), retrieval-search (template matching).
**Notes:** Woodward 1953.

### micro-doppler-signature (cross-domain alias: `micro-doppler`, `time-frequency-radar`, `helicopter-blade-signature`)
**Domain:** Electromagnetics / Antennas
**Definition:** Time-frequency spectrogram of returns from moving sub-parts (blades, limbs); used for ATR.
**Atom or composite:** Composite — time-frequency analysis of radar returns.
**Cost model:** STFT.
**Real wall?** Yes — Heisenberg time-frequency.
**Cross-domain wiring:** signal-processing-rf (TF analysis), ml-training (classification features).
**Notes:** Chen "Micro-Doppler Effect in Radar".

### isar-imaging (cross-domain alias: `inverse-sar`, `range-doppler-imaging`, `target-self-rotation`)
**Domain:** Electromagnetics / Antennas
**Definition:** Use target rotation to synthesize azimuth aperture in single-antenna radar.
**Atom or composite:** Composite — 2D Fourier inversion.
**Cost model:** 2D FFT after motion compensation.
**Real wall?** Yes — rotation rate, motion compensation accuracy.
**Cross-domain wiring:** signal-processing-rf (SAR family), retrieval-search (image formation).
**Notes:** Chen & Martorella.

### through-wall-imaging (cross-domain alias: `twi`, `gpr-wall`, `uwb-through-wall`)
**Domain:** Electromagnetics / Antennas
**Definition:** UWB/MIMO radar penetrates walls; image moving/static targets behind.
**Atom or composite:** Composite — UWB beamforming + clutter cancellation.
**Cost model:** Imaging algorithms; CS reconstruction.
**Real wall?** Yes — wall attenuation + multipath.
**Cross-domain wiring:** signal-processing-rf (imaging), retrieval-search (sparse recovery).
**Notes:** Amin "Through-the-Wall Radar Imaging".

### gpr-ground-penetrating-radar (cross-domain alias: `gpr`, `subsurface-radar`, `geophysical-em`)
**Domain:** Electromagnetics / Antennas
**Definition:** UWB pulses transmitted into ground; reflections from subsurface dielectric contrasts.
**Atom or composite:** Composite — pulse radar in lossy medium.
**Cost model:** Field-portable system; image processing afterward.
**Real wall?** Yes — soil attenuation increases with f.
**Cross-domain wiring:** physics-diffusion (lossy media), retrieval-search (inverse imaging).
**Notes:** Daniels "Ground Penetrating Radar".

### squint-angle-array (cross-domain alias: `beam-squint`, `freq-dependent-pointing`, `wideband-array-pointing-error`)
**Domain:** Electromagnetics / Antennas
**Definition:** Phase-shifter arrays exhibit frequency-dependent beam direction → beam squint.
**Atom or composite:** Composite — limitation of phase vs delay.
**Cost model:** Free analysis.
**Real wall?** Yes — fixes by TTD.
**Cross-domain wiring:** signal-processing-rf (wideband arrays).
**Notes:** Mailloux.

### bandwidth-vs-scan-array (cross-domain alias: `bw-scan-tradeoff`, `wideband-wide-scan`, `array-design-tradeoff`)
**Domain:** Electromagnetics / Antennas
**Definition:** Wide scan + wide BW competes with element pattern + mutual coupling.
**Atom or composite:** Composite — design tradeoff.
**Cost model:** Free design analysis.
**Real wall?** Yes — current-sheet design needed for both.
**Cross-domain wiring:** signal-processing-rf (wideband AESA).
**Notes:** Wheeler 1965.

### dispersion-managed-array (cross-domain alias: `wideband-array-dispersion`, `flat-group-delay-array`, `phase-flat-array`)
**Domain:** Electromagnetics / Antennas
**Definition:** Element / array designs maintaining flat phase across BW for UWB.
**Atom or composite:** Composite — UWB array design.
**Cost model:** Full-wave + group-delay analysis.
**Real wall?** Yes — fundamental dispersion constraints.
**Cross-domain wiring:** signal-processing-rf (UWB), control-numerical-opt (linear-phase design).
**Notes:** Schantz "UWB Antennas".

### radiation-boundary-condition-fem (cross-domain alias: `rbc-fem`, `2nd-order-abc`, `bayliss-turkel`)
**Domain:** Electromagnetics / Antennas
**Definition:** Local PDE boundary conditions approximating Sommerfeld radiation at finite distance in FEM.
**Atom or composite:** Composite — local absorbing BC.
**Cost model:** Cheap.
**Real wall?** No — accuracy bounded; PML usually better.
**Cross-domain wiring:** control-numerical-opt (open BCs).
**Notes:** Bayliss & Turkel 1980.

### eigenmode-expansion-eme (cross-domain alias: `eme-method`, `mode-matching-eme`, `cascading-modes`)
**Domain:** Electromagnetics / Antennas
**Definition:** Express field as superposition of eigenmodes of each section; match at interfaces.
**Atom or composite:** Composite — mode-matching technique.
**Cost model:** Efficient for layered structures.
**Real wall?** Yes — mode truncation.
**Cross-domain wiring:** photonics-optics (waveguide design), linear-algebra-matrix (block solver).
**Notes:** Sudbo 1993.

### tunable-impedance-surface (cross-domain alias: `tis`, `varactor-his`, `electronic-amc`)
**Domain:** Electromagnetics / Antennas
**Definition:** Periodic surface with varactor-loaded cells; surface impedance Z_s tunable.
**Atom or composite:** Composite — tunable AMC.
**Cost model:** Cell + bias network.
**Real wall?** Yes — varactor Q limits BW.
**Cross-domain wiring:** signal-processing-rf (beam-steering surfaces).
**Notes:** Sievenpiper 2003.

### holographic-antenna (cross-domain alias: `holographic-metasurface-ant`, `reference-wave-leakage`, `kymeta-style`)
**Domain:** Electromagnetics / Antennas
**Definition:** Surface impedance pattern interferes with reference wave to radiate desired beam — "EM hologram".
**Atom or composite:** Composite — modulated reactance surface.
**Cost model:** Static or tunable cells.
**Real wall?** Yes — modulation depth + losses.
**Cross-domain wiring:** signal-processing-rf (flat-panel sat), photonics-optics (computer-generated holography).
**Notes:** Fong et al. 2010.

### modulated-metasurface-antenna (cross-domain alias: `mts-antenna`, `imp-modulation-radiator`, `printed-holographic`)
**Domain:** Electromagnetics / Antennas
**Definition:** Surface impedance Z_s(r) modulated to produce desired far-field; basis for low-profile sat antennas.
**Atom or composite:** Composite — holographic antenna implementation.
**Cost model:** Printed PCB / electronically tunable.
**Real wall?** Yes — reference wave coupling.
**Cross-domain wiring:** signal-processing-rf (flat-panel sat).
**Notes:** Sievenpiper, Maci.

### scanning-electron-microscope-em (cross-domain alias: `sem-imaging`, `electron-beam-em`, `nm-em-imaging`)
**Domain:** Electromagnetics / Antennas
**Definition:** Electron beam interactions for nm-scale imaging; uses EM lenses.
**Atom or composite:** Composite — charged-particle optics.
**Cost model:** SEM equipment.
**Real wall?** Yes — electron wavelength + aberration.
**Cross-domain wiring:** photonics-optics (electron optics), biology-bioinformatics (cellular imaging).
**Notes:** Reimer.

