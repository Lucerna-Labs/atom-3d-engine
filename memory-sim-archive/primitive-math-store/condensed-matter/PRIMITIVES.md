# Condensed Matter Physics — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## Crystal Structure & Diffraction

### bravais-lattice (cross-domain alias: `periodic-point-set`, `translation-group`, `lattice-Z3`)
**Domain:** Condensed Matter
**Definition:** Infinite array of discrete points R = n₁a₁ + n₂a₂ + n₃a₃ with nᵢ ∈ ℤ that looks identical from every site; 14 distinct types in 3D, 5 in 2D.
**Atom or composite:** Atom — the translational backbone of crystalline matter.
**Cost model:** Symmetry classification O(1); enumerating Wyckoff positions O(|G|).
**Real wall?** Yes — only 14 inequivalent translation+point groups exist in 3D (Bieberbach theorem).
**Cross-domain wiring:** Defines `fold` periodicity used by `bloch-theorem-cm`; the dual of `reciprocal-lattice` (Fourier pair with signal-processing-rf); group-theoretic structure shared with linear-algebra-matrix `crystallographic-orbits`.
**Notes:** Bravais (1850); Ashcroft & Mermin Ch. 4; International Tables for Crystallography Vol. A.

### unit-cell (cross-domain alias: `tile`, `fundamental-domain`, `repeat-motif`)
**Domain:** Condensed Matter
**Definition:** Parallelepiped spanned by primitive vectors {a₁,a₂,a₃} whose translation tiles all of space; volume V = a₁·(a₂×a₃).
**Atom or composite:** Composite (Bravais lattice + basis atoms).
**Cost model:** Storage O(N_basis); reciprocal volume (2π)³/V.
**Real wall?** No — choice non-unique, but volume invariant.
**Cross-domain wiring:** Project of `bravais-lattice`; conjugate cell in reciprocal space is the `brillouin-zone`; analogous to a fundamental domain in physics-diffusion homogenization.
**Notes:** Buerger conventions; Hahn (Int. Tables) for asymmetric units.

### primitive-cell (cross-domain alias: `minimal-tile`, `one-lattice-point`)
**Domain:** Condensed Matter
**Definition:** Unit cell containing exactly one lattice point; smallest repeat unit, volume V_prim = V_conv/N.
**Atom or composite:** Atom (irreducible periodicity).
**Cost model:** Defines minimum DFT supercell — O(N_prim³) for diagonalization vs N_conv³.
**Real wall?** Yes — cannot be subdivided without breaking lattice symmetry.
**Cross-domain wiring:** Sets sampling minimum for `bloch-theorem-cm`; analogous to `nyquist-sample` in signal-processing-rf along reciprocal directions.
**Notes:** Ashcroft & Mermin Ch. 4; used throughout VASP/Quantum ESPRESSO setups.

### wigner-seitz-cell (cross-domain alias: `voronoi-cell`, `proximity-region`)
**Domain:** Condensed Matter
**Definition:** Set of points closer to a given lattice site than any other; primitive cell symmetric under full point group of the lattice.
**Atom or composite:** Composite (Voronoi tessellation applied to Bravais lattice).
**Cost model:** Geometric construction O(z) where z is coordination number.
**Real wall?** No — equivalent to but more symmetric than conventional primitive cell.
**Cross-domain wiring:** Real-space dual of `brillouin-zone`; Voronoi construction shared with computational-geometry, particle-in-cell physics-diffusion.
**Notes:** Wigner & Seitz (1933); foundational for KKR multiple-scattering DFT.

### reciprocal-lattice (cross-domain alias: `dual-lattice`, `momentum-lattice`, `fourier-grid`)
**Domain:** Condensed Matter
**Definition:** Set of vectors G with e^(iG·R) = 1 for all Bravais R; spanned by bᵢ with aᵢ·bⱼ = 2πδᵢⱼ.
**Atom or composite:** Atom — momentum-space companion to `bravais-lattice`.
**Cost model:** Construction O(1); Ewald summation cutoff scales with |G_max|.
**Real wall?** Yes — Fourier dual is rigid by Pontryagin duality.
**Cross-domain wiring:** Fourier `fold` partner of real-space lattice; identical to `dft-grid` in signal-processing-rf; underlies `bragg-condition` and plane-wave basis for DFT.
**Notes:** Ewald (1921); Kittel Ch. 2.

### brillouin-zone (cross-domain alias: `first-bz`, `momentum-fundamental-domain`)
**Domain:** Condensed Matter
**Definition:** Wigner-Seitz cell of the reciprocal lattice; primitive cell in k-space containing one quantum of crystal momentum.
**Atom or composite:** Composite (Wigner-Seitz construction on reciprocal lattice).
**Cost model:** k-point sampling O(N_k); convergence ~1/N_k for metals.
**Real wall?** Yes — physics periodic mod G; integration over BZ unavoidable for densities.
**Cross-domain wiring:** Defines support of `bloch-state`; high-symmetry path (Γ-X-M…) drives `band-structure`; analog of `principal-domain` in signal-processing-rf.
**Notes:** Brillouin (1930); Monkhorst-Pack (1976) mesh.

### miller-indices (cross-domain alias: `lattice-plane-label`, `hkl-triple`)
**Domain:** Condensed Matter
**Definition:** Triple (hkl) of coprime integers labeling a family of parallel lattice planes with intercepts a/h, b/k, c/l.
**Atom or composite:** Atom (orientation `hash` of a plane family).
**Cost model:** O(1) lookup; plane spacing d_hkl = 2π/|G_hkl|.
**Real wall?** No — purely a labeling convention, but coupled to physical X-ray reflections.
**Cross-domain wiring:** Indexes `bragg-condition` peaks; orientation hashing analogous to direction cosines in electromagnetics-antennas array steering.
**Notes:** Miller (1839); cf. Bravais-Miller (hkil) for hexagonal.

### point-group (cross-domain alias: `crystal-symmetry-group`, `rotation-reflection-set`)
**Domain:** Condensed Matter
**Definition:** Finite group of symmetry operations (rotations, reflections, inversion) leaving a lattice point fixed; 32 crystallographic point groups in 3D.
**Atom or composite:** Atom (algebraic `compare` symmetry).
**Cost model:** |G| ≤ 48; character tables tabulated.
**Real wall?** Yes — crystallographic restriction limits rotation orders to 1,2,3,4,6.
**Cross-domain wiring:** Selects allowed `bloch-state` degeneracies; representation theory shared with quantum-computing irreps; tensor selection rules govern `piezoelectric-tensor`.
**Notes:** Schönflies, Hermann-Mauguin notations; Bradley & Cracknell tables.

### space-group (cross-domain alias: `crystallographic-group`, `230-groups`)
**Domain:** Condensed Matter
**Definition:** Combination of Bravais lattice translations with point-group operations (and screws/glides); 230 distinct space groups in 3D.
**Atom or composite:** Composite (translation `combine` point group, possibly nonsymmorphic).
**Cost model:** Enumeration tabulated; symmetrization speedups O(|G|) for DFT.
**Real wall?** Yes — only 230 by Federov-Schönflies (1891) theorem.
**Cross-domain wiring:** Wallpaper-group analog in 2D; nonsymmorphic groups protect band crossings exploited by `topological-crystalline-insulator`; Hilbert series feeds ml-training equivariant networks.
**Notes:** Int. Tables Vol. A; Bilbao Crystallographic Server.

### bragg-condition (cross-domain alias: `2d-sin-theta`, `diffraction-resonance`, `nλ-rule`)
**Domain:** Condensed Matter
**Definition:** Constructive interference when 2d sin θ = nλ; equivalently Δk = G with G a reciprocal lattice vector.
**Atom or composite:** Atom (`compare` momentum transfer to lattice grid).
**Cost model:** Detection O(1) per (hkl); Rietveld fit O(N_peaks × N_params).
**Real wall?** Yes — only G-vectors satisfying Δk=G scatter elastically.
**Cross-domain wiring:** Identical math to `phased-array-grating` in electromagnetics-antennas; underpins `xray-diffraction`, `neutron-diffraction`, `electron-diffraction`.
**Notes:** W.H. & W.L. Bragg (1913 Nobel).

### laue-condition (cross-domain alias: `delta-k-equals-g`, `momentum-conservation-cm`)
**Domain:** Condensed Matter
**Definition:** Scattering vector q = k′−k must equal a reciprocal lattice vector G for elastic diffraction.
**Atom or composite:** Atom (momentum-space form of Bragg).
**Cost model:** Ewald-sphere construction O(N_G).
**Real wall?** Yes — equivalent to Bragg via 2k·G = G².
**Cross-domain wiring:** Same as phase-matching in photonics-optics (Δk=0 for nonlinear processes); Ewald-sphere geometry reused in `arpes-spectroscopy`.
**Notes:** von Laue (1914 Nobel).

### structure-factor (cross-domain alias: `unit-cell-amplitude`, `F-hkl`)
**Domain:** Condensed Matter
**Definition:** F_hkl = Σⱼ fⱼ exp(2πi(hxⱼ+kyⱼ+lzⱼ)); diffraction intensity I ∝ |F|².
**Atom or composite:** Composite (atomic form factors `combine` with phase factors).
**Cost model:** O(N_atoms × N_reflections).
**Real wall?** Phase information lost in |F|² — `phase-problem` of crystallography.
**Cross-domain wiring:** Discrete Fourier transform of unit-cell density — same `project` operation as signal-processing-rf DFT; drives Rietveld refinement and direct methods.
**Notes:** Patterson (1934); Hauptman-Karle (1985 Nobel) direct methods.

### atomic-form-factor (cross-domain alias: `scattering-amplitude-atom`, `f-of-q`)
**Domain:** Condensed Matter
**Definition:** Fourier transform of electron density of an atom: f(q) = ∫ρ(r)e^(-iq·r)d³r.
**Atom or composite:** Atom (single-atom `project` to momentum space).
**Cost model:** Tabulated (Cromer-Mann); analytic O(1) per q.
**Real wall?** Yes — atomic shell physics fixes the q-dependence.
**Cross-domain wiring:** Analog of `antenna-element-pattern` in electromagnetics-antennas — the unit cell is the array, atoms are elements.
**Notes:** Int. Tables Vol. C; Cromer-Mann coefficients.

### xray-diffraction (cross-domain alias: `xrd`, `bragg-spectroscopy`)
**Domain:** Condensed Matter
**Definition:** Elastic scattering of ~Å X-rays from electron density, producing peaks at reciprocal lattice vectors.
**Atom or composite:** Composite (Bragg + structure factor + Debye-Waller).
**Cost model:** Powder pattern fit O(N_peaks); single-crystal full data O(10⁵) reflections.
**Real wall?** Wavelength must satisfy λ ≲ 2d_max for accessible peaks.
**Cross-domain wiring:** Lab-scale realization of `bragg-condition`; competes with `neutron-diffraction` (nuclear), `electron-diffraction` (smaller samples).
**Notes:** Cullity & Stock; SHELXL refinement.

### neutron-diffraction (cross-domain alias: `nuclear-scattering`, `magnetic-bragg`)
**Domain:** Condensed Matter
**Definition:** Diffraction of thermal/cold neutrons (λ ~ Å) from nuclei and magnetic moments; sensitive to light atoms and spin order.
**Atom or composite:** Composite (nuclear + magnetic form factors).
**Cost model:** Requires reactor/spallation source; counting times hours.
**Real wall?** Flux limited (~10⁸ n/cm²/s) — sets sample size and statistics.
**Cross-domain wiring:** Reveals `magnetic-order` invisible to X-ray; inelastic mode is INS — direct probe of `phonon-dispersion` and `spin-wave`.
**Notes:** Shull (1994 Nobel); Squires textbook.

### electron-diffraction (cross-domain alias: `selected-area-diffraction`, `SAED`)
**Domain:** Condensed Matter
**Definition:** Diffraction of ~100 keV electrons (λ~pm) by lattice; performed in TEM, gives reciprocal-space patterns from nanoscale regions.
**Atom or composite:** Composite (kinematic + dynamical scattering theories).
**Cost model:** Per-pattern ms; dynamical simulation (Bloch wave / multislice) O(N_g²) per slice.
**Real wall?** Dynamical effects ruin kinematic |F|² interpretation in thick samples.
**Cross-domain wiring:** Multislice algorithm is split-step `propagator` shared with photonics-optics BPM; CBED reveals point-group symmetries.
**Notes:** Davisson-Germer (1937 Nobel); Cowley-Moodie multislice.

### rietveld-refinement (cross-domain alias: `powder-fit`, `whole-pattern-fitting`)
**Domain:** Condensed Matter
**Definition:** Least-squares fit of full powder diffraction profile to a structural model + peak shape + background.
**Atom or composite:** Composite (forward model + nonlinear LS).
**Cost model:** O(N_data × N_params) per Marquardt iteration.
**Real wall?** Local minima; peak overlap; preferred orientation.
**Cross-domain wiring:** Same nonlinear-LS engine as `levenberg-marquardt` in ml-training; profile functions (pseudo-Voigt) borrowed from signal-processing-rf.
**Notes:** Rietveld (1969); GSAS-II, FullProf.

### debye-waller-factor (cross-domain alias: `thermal-disorder`, `B-factor`)
**Domain:** Condensed Matter
**Definition:** Reduction exp(−2W) = exp(−⟨(q·u)²⟩) of diffraction intensity due to thermal atomic displacements u.
**Atom or composite:** Atom (`scale` damping from harmonic vibration variance).
**Cost model:** O(N_atoms × N_phonon_modes) for ab initio evaluation.
**Real wall?** Yes — finite T always produces nonzero ⟨u²⟩ by zero-point + thermal motion.
**Cross-domain wiring:** Same Gaussian-process variance damping as `pulse-broadening` in signal-processing-rf; `b-factor` in protein crystallography.
**Notes:** Debye (1913), Waller (1923); Willis & Pryor textbook.

### structure-determination (cross-domain alias: `crystal-solving`, `direct-methods`)
**Domain:** Condensed Matter
**Definition:** Inverse problem of recovering atomic positions from |F_hkl|² intensities; phase problem solved by direct methods, Patterson, or anomalous scattering.
**Atom or composite:** Composite (FFT + statistical phasing + refinement).
**Cost model:** Tangent formula O(N²); SHELXD dual-space iterations.
**Real wall?** Phase problem — single-set intensities cannot fix phases uniquely without prior.
**Cross-domain wiring:** Phase retrieval same as `ptychography` in photonics-optics; ML phasing now uses ml-training generative priors.
**Notes:** Hauptman, Karle (Nobel 1985); Sheldrick SHELX.

### ewald-sphere (cross-domain alias: `reflection-sphere`, `k-shell`)
**Domain:** Condensed Matter
**Definition:** Sphere of radius |k|=2π/λ in reciprocal space; reflections lie at reciprocal lattice points intersected by this sphere.
**Atom or composite:** Atom (geometric `scan` device for diffraction conditions).
**Cost model:** O(1) per orientation; rotation scans access full hkl set.
**Real wall?** Yes — only G inside the limiting sphere |G|<2|k| are accessible.
**Cross-domain wiring:** Same geometric construction as `visible-region` in electromagnetics-antennas k-space arrays.
**Notes:** Ewald (1913); Authier IUCr monograph.

---

## Band Theory & Bloch States

### bloch-theorem-cm (cross-domain alias: `bloch-floquet`, `crystal-momentum`)
**Domain:** Condensed Matter
**Definition:** Eigenstates in a periodic potential have form ψ_nk(r) = e^(ik·r) u_nk(r) with u_nk periodic; k labels irreps of translation group.
**Atom or composite:** Atom (`fold` Hilbert space by lattice symmetry).
**Cost model:** Diagonalize H(k) at each k-point — O(N_basis³ × N_k).
**Real wall?** Yes — exact for noninteracting electrons in any periodic V.
**Cross-domain wiring:** Spatial analog of `floquet-theorem` in physics-diffusion / quantum-computing; foundation of all band theory; basis of `wannier-function`.
**Notes:** Bloch (1928); Ashcroft & Mermin Ch. 8.

### free-electron-gas (cross-domain alias: `jellium`, `fermi-sea`)
**Domain:** Condensed Matter
**Definition:** Noninteracting electrons in a 3D box with uniform positive background; ε_k = ℏ²k²/2m, filled to k_F.
**Atom or composite:** Atom (simplest interacting-free electron baseline).
**Cost model:** Analytic; n = k_F³/3π², ε_F ∝ n^(2/3).
**Real wall?** Ignores periodicity, interactions; surprisingly accurate for alkali metals.
**Cross-domain wiring:** Reference state for `sommerfeld-expansion`; jellium DFT benchmark; underlies `drude-model` and `plasma-frequency-solid`.
**Notes:** Sommerfeld (1928); textbook starting point.

### sommerfeld-expansion (cross-domain alias: `low-T-fermi-expansion`, `pi-squared-over-6`)
**Domain:** Condensed Matter
**Definition:** Asymptotic series for ∫H(ε)f(ε)dε at low T: I = I₀ + (π²/6)(k_BT)²H′(ε_F)+…
**Atom or composite:** Composite (Taylor `fold` of Fermi-Dirac integral).
**Cost model:** O(N) terms for arbitrary T/T_F polynomial accuracy.
**Real wall?** Asymptotic — diverges for T~T_F.
**Cross-domain wiring:** Yields linear electronic specific heat γT; analogous to `bose-einstein-integral` expansions; used in `mott-formula` for thermopower.
**Notes:** Sommerfeld (1928); Ashcroft & Mermin App. C.

### fermi-dirac-distribution-cm (cross-domain alias: `fd-occupation`, `f-of-epsilon`)
**Domain:** Condensed Matter
**Definition:** f(ε) = 1/(exp((ε−μ)/k_BT)+1); occupation probability of single-particle state at energy ε.
**Atom or composite:** Atom (Pauli `order` constraint applied statistically).
**Cost model:** Pointwise O(1); chemical potential solve via bisection.
**Real wall?** Yes — antisymmetry + thermal equilibrium fix the form.
**Cross-domain wiring:** Drives `electron-transport` Boltzmann; cousin of bosonic `bose-einstein` and FD vs MB in physics-diffusion kinetic theory.
**Notes:** Fermi (1926), Dirac (1926).

### nearly-free-electron (cross-domain alias: `weak-pseudopotential-cm`, `nfe`)
**Domain:** Condensed Matter
**Definition:** Perturbative treatment of free electrons with weak periodic potential; gaps 2|V_G| open at BZ boundaries.
**Atom or composite:** Composite (free electrons + perturbation `combine`).
**Cost model:** Two-plane-wave reduction O(1) per crossing; full plane-wave O(N_PW³).
**Real wall?** Yes — Bragg reflection mixes ψ_k and ψ_{k−G} whenever they are degenerate.
**Cross-domain wiring:** Mirror of `tight-binding`; together they bracket band physics; basis for plane-wave DFT pseudopotentials.
**Notes:** Harrison Electronic Structure; Ashcroft & Mermin Ch. 9.

### tight-binding-cm (cross-domain alias: `lcao`, `hopping-hamiltonian`)
**Domain:** Condensed Matter
**Definition:** H = −t Σ_⟨ij⟩ c†ᵢcⱼ + h.c.; bands constructed from atomic orbitals with nearest-neighbor hopping.
**Atom or composite:** Composite (atomic orbitals `combine` via overlap and hopping integrals).
**Cost model:** Bloch-summed N×N Hamiltonian per k; O(N³) diagonalization.
**Real wall?** Yes — sparse and exact for well-localized orbitals (e.g., d/f electrons).
**Cross-domain wiring:** Lattice analog of `hopping-matrix` in quantum-computing; basis of Wannier-interpolation, Slater-Koster tables, and toy models like Haldane and Kitaev.
**Notes:** Slater & Koster (1954); Harrison (1980).

### lcao-method (cross-domain alias: `linear-combination-atomic-orbitals`, `gaussian-basis`)
**Domain:** Condensed Matter
**Definition:** Expand crystal orbitals ψ_nk = Σ_α c_nkα φ_α(r) using localized atomic basis functions.
**Atom or composite:** Atom (basis-set choice).
**Cost model:** Overlap, kinetic, V matrices O(N²); diagonalization O(N³).
**Real wall?** Linear dependence and BSSE limit basis-set extension.
**Cross-domain wiring:** Same idea as `galerkin-basis` in physics-diffusion FEM; used in CRYSTAL, SIESTA, FHI-aims.
**Notes:** Roothaan-Hall (1951); Slater-type/Gaussian-type orbitals.

### wannier-function (cross-domain alias: `localized-orbital`, `inverse-bloch`)
**Domain:** Condensed Matter
**Definition:** w_n(r−R) = (V/(2π)³)∫_BZ ψ_nk(r) e^(−ik·R) d³k; spatially localized counterparts to Bloch states.
**Atom or composite:** Composite (Fourier `fold` of Bloch over BZ).
**Cost model:** Maximally-localized via Marzari-Vanderbilt optimization O(N_k N_b² N_iter).
**Real wall?** Gauge ambiguity; for topologically nontrivial bands no exponentially localized Wannier set exists.
**Cross-domain wiring:** Inverse Fourier of `bloch-state`; same as `lattice-fourier` in signal-processing-rf; basis for DMFT downfolding and topological invariants.
**Notes:** Wannier (1937); Marzari-Vanderbilt (1997); Wannier90 code.

### band-structure (cross-domain alias: `dispersion-cm`, `eigenvalue-spectrum`)
**Domain:** Condensed Matter
**Definition:** Set of energy bands ε_n(k) over the Brillouin zone; central object of single-particle solid-state.
**Atom or composite:** Composite (`scan` k along high-symmetry path through diagonalized H(k)).
**Cost model:** O(N_k × N_basis³).
**Real wall?** Sets all single-particle observables (gap, m*, DOS); accuracy limited by exchange-correlation functional.
**Cross-domain wiring:** Real-space analog of `mode-spectrum` in photonics-optics waveguides; ML surrogates (kernel ridge, GNN) increasingly common in ml-training.
**Notes:** Sham-Kohn; Materials Project, AFLOW, OQMD.

### fermi-surface (cross-domain alias: `fs-cm`, `iso-energy-k-shell`)
**Domain:** Condensed Matter
**Definition:** Locus in k-space where ε_n(k) = ε_F at T=0; geometry controls transport and quantum oscillations.
**Atom or composite:** Composite (level-set of band structure).
**Cost model:** Marching cubes / tetrahedron method O(N_k log N_k).
**Real wall?** Yes — Luttinger theorem fixes its enclosed volume in interacting Fermi liquids.
**Cross-domain wiring:** Iso-surface extraction same as `marching-cubes` in computational geometry; probed by `dHvA`, `arpes-spectroscopy`.
**Notes:** Onsager, Lifshitz-Kosevich; Pippard textbook.

### density-of-states-cm (cross-domain alias: `dos`, `g-of-epsilon`)
**Domain:** Condensed Matter
**Definition:** g(ε) = Σ_n ∫_BZ δ(ε−ε_n(k)) d³k/(2π)³; number of states per unit energy per unit volume.
**Atom or composite:** Composite (level-set `count` over band).
**Cross-domain wiring:** Used in `sommerfeld-expansion`, optical absorption, Stoner criterion; ML predicted via ml-training neural-DOS.
**Cost model:** Tetrahedron method (Blöchl) O(N_k log N_k).
**Real wall?** Van Hove singularities are unavoidable topological features.
**Notes:** Van Hove (1953); Lehmann-Taut tetrahedron.

### effective-mass (cross-domain alias: `m-star`, `inverse-curvature-tensor`)
**Domain:** Condensed Matter
**Definition:** (1/m*)ᵢⱼ = (1/ℏ²) ∂²ε/∂kᵢ∂kⱼ at band extremum; controls semiclassical inertia of electrons.
**Atom or composite:** Atom (`project` of band curvature).
**Cost model:** Finite-difference O(N_k near extremum).
**Real wall?** Yes — sign + tensor structure follow rigorously from band geometry near a point.
**Cross-domain wiring:** Enters `landau-level-cm` cyclotron mass; renormalized by interactions (heavy fermions m*~10³m); ML feature for materials screening.
**Notes:** Luttinger-Kohn (1955) k·p; Yu & Cardona.

### hole-quasiparticle (cross-domain alias: `electron-vacancy`, `positive-charge-carrier`)
**Domain:** Condensed Matter
**Definition:** Absence of an electron near top of an otherwise filled band; behaves as +e particle with opposite momentum and curvature.
**Atom or composite:** Composite (Bogoliubov-like `combine` of filled-band states).
**Cost model:** Same as electron picture; convenient for valence bands.
**Real wall?** Yes — mirrors particle-hole symmetry of band insulators / semiconductors.
**Cross-domain wiring:** Analog of `positron` in QED; pivotal for p-type semiconductors, `cooper-pair` (hole-hole binding), and `arpes-spectroscopy` spectral function.
**Notes:** Wilson (1931); Kittel Ch. 8.

### kp-method (cross-domain alias: `k-dot-p`, `envelope-perturbation`)
**Domain:** Condensed Matter
**Definition:** Expansion of H around a k₀ via H(k) = H(k₀) + (ℏ/m)k·p + ℏ²k²/2m; produces effective-mass Hamiltonians.
**Atom or composite:** Composite (perturbation in k away from a reference point).
**Cost model:** Small N×N matrix (4-, 6-, 8-band); analytic dispersion.
**Real wall?** Accurate only near k₀; ignores remote bands except perturbatively.
**Cross-domain wiring:** Basis for `envelope-function-approximation` in semiconductor heterostructures; identical math to `tight-binding` perturbation around band edges.
**Notes:** Luttinger-Kohn (1955); Kane (1957); Winkler monograph.

### envelope-function-approximation (cross-domain alias: `efa`, `effective-mass-eq`)
**Domain:** Condensed Matter
**Definition:** Slowly-varying envelope F(r) multiplies rapidly oscillating Bloch factor u_nk₀; F satisfies Schrödinger eq. with V_ext and m*.
**Atom or composite:** Composite (factorize ψ into envelope × periodic part).
**Cost model:** Schrödinger solve in continuum O(N_grid³).
**Real wall?** Valid when V_ext varies slowly on lattice scale.
**Cross-domain wiring:** Same paraxial separation as `slowly-varying-envelope` in photonics-optics; backbone of `quantum-well` and 2DEG modeling.
**Notes:** BenDaniel-Duke (1966); Bastard heterostructure book.

### semiclassical-eom (cross-domain alias: `wave-packet-eom`, `karplus-luttinger`)
**Domain:** Condensed Matter
**Definition:** ṙ = (1/ℏ)∂ε/∂k − k̇×Ω_n; ℏk̇ = −eE − e(ṙ×B); Ω_n = Berry curvature.
**Atom or composite:** Composite (band derivative `combine` with Berry curvature `cross` product).
**Cost model:** ODE per packet — O(N_t) per trajectory.
**Real wall?** Valid for slowly varying fields and single-band wave packets.
**Cross-domain wiring:** Anomalous velocity term underlies `anomalous-hall`, `chern-insulator`, valley Hall; analog of `geometric-phase-eom` in quantum-computing.
**Notes:** Karplus-Luttinger (1954); Xiao-Chang-Niu RMP (2010).

### berry-phase-cm (cross-domain alias: `geometric-phase-bands`, `pancharatnam-cm`)
**Domain:** Condensed Matter
**Definition:** γ = ∮_C i⟨u_nk|∇_k u_nk⟩·dk acquired by Bloch state on a closed loop in BZ.
**Atom or composite:** Atom (gauge-invariant `scan` of parameter space).
**Cost model:** Discrete Wilson loop O(N_k_on_loop · N_b²).
**Real wall?** Yes — topological obstruction to global smooth gauge.
**Cross-domain wiring:** Identical concept as `geometric-phase` in quantum-computing; integrates to `chern-number`; controls electric polarization (King-Smith-Vanderbilt).
**Notes:** Berry (1984); Resta polarization theory.

### berry-curvature-cm (cross-domain alias: `omega-of-k`, `k-space-magnetic-field`)
**Domain:** Condensed Matter
**Definition:** Ω_n(k) = i⟨∇_k u|×|∇_k u⟩; acts as a momentum-space "magnetic field".
**Atom or composite:** Atom (`fold` derivative of Bloch periodic part).
**Cost model:** Per-k Kubo-like formula over remote bands O(N_b²).
**Real wall?** Time-reversal forces Ω(−k) = −Ω(k); inversion forces Ω(−k) = Ω(k).
**Cross-domain wiring:** Source of anomalous Hall, valley Hall, intrinsic spin Hall; analog of vorticity in fluid mechanics; hot spots near band crossings.
**Notes:** Thouless-Kohmoto-Nightingale-den Nijs (1982); Xiao RMP (2010).

### chern-number (cross-domain alias: `tknn-integer`, `c1-invariant`)
**Domain:** Condensed Matter
**Definition:** C = (1/2π)∫_BZ Ω_n d²k ∈ ℤ; topological invariant of 2D gapped band.
**Atom or composite:** Atom (topological `count`).
**Cost model:** Discrete Fukui-Hatsugai algorithm O(N_k log N_k).
**Real wall?** Yes — quantized integer, robust to smooth deformations.
**Cross-domain wiring:** Equals σ_xy/(e²/h) for filled band (TKNN); precursor of `z2-invariant`, `chern-insulator`, Haldane model.
**Notes:** TKNN (1982); Kohmoto (1985).

### tknn-formula (cross-domain alias: `kubo-hall`, `quantized-hall`)
**Domain:** Condensed Matter
**Definition:** σ_xy = (e²/h)Σ_n C_n, where C_n is Chern number of filled band n.
**Atom or composite:** Composite (Kubo + topology).
**Cost model:** Equivalent to BZ integral of Berry curvature.
**Real wall?** Yes — quantization protected by gap + periodicity.
**Cross-domain wiring:** Integer quantum Hall expressed as a topological invariant; sets up `chern-insulator` without external B.
**Notes:** Thouless-Kohmoto-Nightingale-den Nijs (1982 PRL).

---

## Bonding & Chemistry

### ionic-bond (cross-domain alias: `electrostatic-bond`, `madelung-lattice`)
**Domain:** Condensed Matter
**Definition:** Bond formed by electron transfer producing cation+anion held together by Coulomb interaction; energy U_M = αM e²/4πε₀r₀ with Madelung constant α_M.
**Atom or composite:** Atom (limit of full charge transfer).
**Cost model:** Ewald summation O(N log N).
**Real wall?** Yes — long-range Coulomb requires conditionally convergent sum.
**Cross-domain wiring:** Madelung sum same as `ewald-sum` in electromagnetics-antennas / molecular dynamics; basis of rock-salt, fluorite, perovskite structures.
**Notes:** Born-Madelung; Pauling (1939).

### covalent-bond (cross-domain alias: `shared-pair`, `mo-bond`)
**Domain:** Condensed Matter
**Definition:** Bond from quantum-mechanical sharing of electron pair between atoms with similar electronegativity; described by molecular-orbital or valence-bond theory.
**Atom or composite:** Composite (orbital overlap + exchange + correlation).
**Cost model:** Hartree-Fock O(N⁴); CCSD(T) O(N⁷); DFT O(N³).
**Real wall?** Pauli + exchange-correlation set bond strength and angle.
**Cross-domain wiring:** Directional bonding underlies diamond, Si, GaAs; basis of `tight-binding-cm` Slater-Koster integrals; `bond-order` is fundamental ML feature.
**Notes:** Heitler-London (1927); Mulliken; Pauling Nature of Chemical Bond.

### metallic-bond (cross-domain alias: `electron-glue`, `delocalized-bond`)
**Domain:** Condensed Matter
**Definition:** Cohesion from delocalized valence electrons forming a "sea" between positive ion cores; nondirectional.
**Atom or composite:** Atom (limit of full electron delocalization).
**Cost model:** Pseudopotential DFT O(N³); EAM potentials O(N).
**Real wall?** Yes — large compressibility, low resistivity, ductility follow.
**Cross-domain wiring:** Underlies `free-electron-gas`, `drude-model`; embedded-atom-method bridges with classical MD.
**Notes:** Drude (1900); Sommerfeld (1928).

### hydrogen-bond (cross-domain alias: `H-bond`, `proton-bridge`)
**Domain:** Condensed Matter
**Definition:** Attractive interaction D−H···A between a hydrogen donor and acceptor; 5–40 kJ/mol; directional and partly covalent.
**Atom or composite:** Composite (electrostatic + dispersion + slight covalency).
**Cost model:** Captured by GGA/B3LYP DFT; long-range correction via dispersion D3.
**Real wall?** Cooperative effects in water, ice, DNA; not reducible to point dipoles.
**Cross-domain wiring:** Drives water anomalies, ice phases, protein folding; ML force-field benchmarks in ml-training (e.g., MD17).
**Notes:** Pauling (1939); Steiner Angew. Chem. (2002).

### van-der-waals-bond (cross-domain alias: `london-dispersion`, `induced-dipole-coupling`)
**Domain:** Condensed Matter
**Definition:** Weak attraction U(r) ~ −C₆/r⁶ from correlated quantum fluctuations of charge densities.
**Atom or composite:** Composite (RPA-level correlation).
**Cost model:** DFT-D3, MBD O(N²); RPA O(N⁴).
**Real wall?** Required for layered materials (graphite, h-BN, MoS₂) and noble-gas solids.
**Cross-domain wiring:** Same multipole correlation as `casimir-force` in photonics-optics; ml-training force fields must respect ~r⁻⁶ tail.
**Notes:** London (1930); Tkatchenko-Scheffler (2009).

### pauling-electronegativity (cross-domain alias: `chi-pauling`, `bond-polarity-scalar`)
**Domain:** Condensed Matter
**Definition:** Empirical scale χ of an atom's tendency to attract electrons in a bond; calibrated via bond energies.
**Atom or composite:** Atom (per-element scalar).
**Cost model:** O(1) lookup; |Δχ|² ∝ ionicity.
**Real wall?** Heuristic — different scales (Mulliken, Allred-Rochow) disagree quantitatively.
**Cross-domain wiring:** Feature input in ml-training materials descriptors (Magpie, Matminer); guides ionicity, lattice-energy prediction.
**Notes:** Pauling (1932).

### valence-bond-theory (cross-domain alias: `vb-theory`, `heitler-london`)
**Domain:** Condensed Matter
**Definition:** Wave function built from localized bond pairs with explicit resonance among Lewis structures.
**Atom or composite:** Composite (antisymmetrized product of localized pairs).
**Cost model:** Exponential in number of resonance structures.
**Real wall?** Complementary to MO; needed to recover correct dissociation limits.
**Cross-domain wiring:** Resonance structures evolve into `rvb-state` in correlated CM; provides chemical intuition for tight-binding parameters.
**Notes:** Heitler-London (1927); Pauling; Shaik-Hiberty modern VB.

### molecular-orbital-theory (cross-domain alias: `mo-theory`, `huckel-extended`)
**Domain:** Condensed Matter
**Definition:** Build delocalized orbitals as LCAO over the whole molecule/cluster; bonding-antibonding splitting from off-diagonal H_ij.
**Atom or composite:** Composite (diagonalize H in atomic basis).
**Cost model:** O(N³) for Hartree-Fock or DFT in MO basis.
**Real wall?** Misses static correlation without CASSCF/MRCI.
**Cross-domain wiring:** Crystalline limit is `band-structure`; in CM, MO levels become Bloch bands as cluster grows.
**Notes:** Hund-Mulliken (1928); Hoffmann extended Hückel (1963).

### crystal-field-theory (cross-domain alias: `cf-splitting`, `t2g-eg`)
**Domain:** Condensed Matter
**Definition:** Electrostatic field of surrounding ligands splits degenerate d (or f) orbitals; Δ_oct, Δ_tet, etc.
**Atom or composite:** Composite (point-charge multipole expansion).
**Cost model:** Lookup of irreducible reps; O(1) per coordination.
**Real wall?** Ignores covalency — replaced by ligand-field theory when needed.
**Cross-domain wiring:** Sets `magnetic-anisotropy`, optical d-d transitions; basis of Jahn-Teller analysis.
**Notes:** Bethe (1929); Van Vleck.

### ligand-field-theory (cross-domain alias: `lft`, `mo-with-ligands`)
**Domain:** Condensed Matter
**Definition:** Extension of crystal field including covalent mixing with ligand orbitals; uses MO theory in the cluster.
**Atom or composite:** Composite (CF + ligand-orbital mixing).
**Cost model:** CASSCF/NEVPT2 O(N⁵–N⁷); semiempirical Racah parameters tabulated.
**Real wall?** Required for nephelauxetic, charge-transfer transitions, Mott insulators.
**Cross-domain wiring:** Underlies `superexchange` in magnets; key for transition-metal oxide DFT+U; informs `kondo-effect` parameters.
**Notes:** Griffith (1961); Ballhausen.

---

## Phonons & Lattice Dynamics

### lattice-dynamics (cross-domain alias: `harmonic-vibration`, `phonon-hamiltonian`)
**Domain:** Condensed Matter
**Definition:** Quantization of small displacements around equilibrium: H = Σ_qν ℏω_qν(b†+½); modes labeled by wavevector q and branch ν.
**Atom or composite:** Composite (harmonic expansion of total energy + canonical quantization).
**Cost model:** Hessian eigendecomposition O((3N)³); finite-difference frozen-phonon O(N_supercell × 3N).
**Real wall?** Harmonic approximation breaks down for soft modes and ferroelectric instabilities.
**Cross-domain wiring:** Phonon bands fold like electronic bands; same Bloch theorem; foundational for thermal transport and Debye-Waller.
**Notes:** Born-Huang Dynamical Theory; Maradudin.

### monatomic-chain-cm (cross-domain alias: `1d-spring-chain`, `linear-chain`)
**Domain:** Condensed Matter
**Definition:** Toy model: identical masses m on a 1D lattice with spring K; dispersion ω(q) = 2√(K/m)|sin(qa/2)|.
**Atom or composite:** Atom (simplest periodic vibrational system).
**Cost model:** Analytic O(1).
**Real wall?** Yes — closed-form benchmark for any numerical phonon code.
**Cross-domain wiring:** Same as `discrete-laplacian-1d` in signal-processing-rf; archetype for nearest-neighbor tight-binding.
**Notes:** Born; Kittel Ch. 4.

### diatomic-chain (cross-domain alias: `optic-acoustic-chain`, `two-mass-chain`)
**Domain:** Condensed Matter
**Definition:** Alternating masses M, m on a 1D chain; produces acoustic and optical branches separated by a gap.
**Atom or composite:** Composite (basis of 2 atoms in 1D Bravais lattice).
**Cost model:** Analytic 2×2 dynamical matrix.
**Real wall?** Optical branch starts at finite ω_0 ≈ √(2K/μ).
**Cross-domain wiring:** Prototype for IR-active modes, polar phonon `lo-to-splitting`, and topological 1D `ssh-model` analogs.
**Notes:** Born-Huang; Kittel Ch. 4.

### acoustic-branch (cross-domain alias: `sound-branch`, `goldstone-phonon`)
**Domain:** Condensed Matter
**Definition:** Phonon mode with ω → 0 as q → 0 linearly; sound waves.
**Atom or composite:** Atom (Goldstone mode of broken translation symmetry).
**Cost model:** Slope = sound speed v_s.
**Real wall?** Yes — Goldstone theorem requires gapless mode from broken continuous symmetry.
**Cross-domain wiring:** Long-wavelength elasticity = continuum limit of acoustic branch; underlies Debye model, sound velocity in ultrasonics.
**Notes:** Born-Huang.

### optical-branch (cross-domain alias: `optical-phonon`, `lo-to-mode`)
**Domain:** Condensed Matter
**Definition:** Phonon branch with ω(q→0) > 0; usually IR-active; LO-TO splitting in polar crystals.
**Atom or composite:** Composite (basis-driven optical-acoustic splitting).
**Cost model:** Non-analytic correction from Born effective charges.
**Real wall?** LO-TO gap fixed by Lyddane-Sachs-Teller relation.
**Cross-domain wiring:** Drives `raman-spectroscopy` and IR absorption; couples to electrons in `frohlich-coupling` and polaron formation.
**Notes:** Lyddane-Sachs-Teller (1941); Cochran soft mode theory.

### phonon-dispersion (cross-domain alias: `omega-q-curve`, `bands-of-vibration`)
**Domain:** Condensed Matter
**Definition:** ω_ν(q) for all branches; obtained by diagonalizing dynamical matrix D(q) at each q.
**Atom or composite:** Composite (`scan` of dynamical eigenproblem along BZ path).
**Cost model:** O((3N)³ × N_q).
**Real wall?** Imaginary frequencies signal structural instability.
**Cross-domain wiring:** Probed directly by inelastic neutron scattering (INS) and Raman/IR; basis of `thermal-conductivity-callaway`.
**Notes:** Phonopy; DFPT; Quantum ESPRESSO.

### debye-model (cross-domain alias: `linear-dispersion-model`, `omega-D-cutoff`)
**Domain:** Condensed Matter
**Definition:** Approximate phonon spectrum as ω = v_s|q| up to Debye cutoff ω_D = v_s(6π²n)^(1/3); yields C_v ∝ T³ at low T.
**Atom or composite:** Composite (linear dispersion + cutoff).
**Cost model:** Analytic Debye function D(x); O(1).
**Real wall?** Captures low-T limit exactly; misses optical modes.
**Cross-domain wiring:** Same UV cutoff trick as `frequency-cutoff` in signal-processing-rf; integrable approximation used in thermal-property fits.
**Notes:** Debye (1912); Ashcroft & Mermin Ch. 23.

### einstein-model-cm (cross-domain alias: `localized-oscillator-model`, `single-frequency-phonon`)
**Domain:** Condensed Matter
**Definition:** All atoms vibrate independently at a single frequency ω_E; predicts C_v → 0 exponentially at low T.
**Atom or composite:** Atom (high-T baseline, dispersionless).
**Cost model:** Analytic O(1).
**Real wall?** Misses low-T T³ tail; good for optical modes.
**Cross-domain wiring:** Per-mode contribution underlies modern `phonon-thermodynamics`; conceptually links to dispersionless flat bands.
**Notes:** Einstein (1907); first quantum solid-state model.

### debye-waller-deeper (cross-domain alias: `mean-sq-displacement`, `u-squared`)
**Domain:** Condensed Matter
**Definition:** ⟨u²⟩ = (ℏ/2NM)Σ_qν (1/ω_qν)(2n_qν+1); thermal+zero-point mean-square displacement.
**Atom or composite:** Composite (phonon-population integral).
**Cost model:** O(N_q × N_branches).
**Real wall?** Diverges in 1D/2D long-range orders (Mermin-Wagner side).
**Cross-domain wiring:** Damps `bragg-condition` peaks; cousin of `lamb-mossbauer-factor`; ML potentials must reproduce ⟨u²⟩(T).
**Notes:** Willis & Pryor.

### gruneisen-parameter (cross-domain alias: `mode-gruneisen`, `gamma-q-nu`)
**Domain:** Condensed Matter
**Definition:** γ_qν = −∂ln ω_qν/∂ln V; measures volume sensitivity of phonon frequencies.
**Atom or composite:** Atom (`scale` derivative of phonon).
**Cost model:** Finite-difference DFPT O(2 × phonon calc).
**Real wall?** Yes — fundamentally anharmonic; γ=0 implies no thermal expansion.
**Cross-domain wiring:** Drives `thermal-expansion`; appears in equation of state (Mie-Grüneisen).
**Notes:** Grüneisen (1912); Wallace Stat Phys of Crystals.

### thermal-expansion-cm (cross-domain alias: `alpha-T`, `lattice-dilation`)
**Domain:** Condensed Matter
**Definition:** α = (1/V)(∂V/∂T)_P = γC_v/(VB); positive in most materials, anomalously negative in some (Invar, ZrW₂O₈).
**Atom or composite:** Composite (Grüneisen × specific heat / bulk modulus).
**Cost model:** Quasi-harmonic approximation O(N_V × N_phonon).
**Real wall?** Requires anharmonicity; harmonic crystal has α = 0.
**Cross-domain wiring:** Coupling to elastic moduli ties to acoustics; key for thermal-stress engineering.
**Notes:** Grüneisen; Barron-White Heat Capacity.

### anharmonic-phonon (cross-domain alias: `cubic-quartic-coupling`, `phi-3-phi-4`)
**Domain:** Condensed Matter
**Definition:** Cubic + quartic terms in lattice potential beyond harmonic expansion; mediate phonon-phonon scattering and thermal conductivity.
**Atom or composite:** Composite (Taylor expansion of V beyond quadratic).
**Cost model:** SCPH, SSCHA O(N_supercell³ × N_iter); ALAMODE.
**Real wall?** Required for finite κ_lattice; ignored gives infinite thermal conductivity.
**Cross-domain wiring:** Same φ³/φ⁴ couplings as nonlinear `wave-equation` in signal-processing-rf; underlies frequency renormalization at high T.
**Notes:** Cowley (1968); Errea SSCHA.

### umklapp-scattering (cross-domain alias: `u-process`, `g-flip`)
**Domain:** Condensed Matter
**Definition:** Phonon-phonon scattering q₁+q₂ = q₃+G with G ≠ 0; degrades crystal momentum and limits κ_lattice at high T.
**Atom or composite:** Atom (momentum-non-conserving lattice scattering).
**Cost model:** Triple-q integral, weighted by Bose factors.
**Real wall?** Yes — without umklapp, κ_lattice would diverge.
**Cross-domain wiring:** Sets T-dependence κ ∝ 1/T at high T; analog of large-angle scattering in plasma transport.
**Notes:** Peierls (1929); Klemens; Ziman Electrons and Phonons.

### normal-scattering (cross-domain alias: `n-process`, `momentum-conserving-pp`)
**Domain:** Condensed Matter
**Definition:** Phonon-phonon scattering with G = 0; conserves crystal momentum; redistributes mode populations without degrading current.
**Atom or composite:** Atom (momentum-conserving scattering channel).
**Cost model:** Same as U but with G=0.
**Real wall?** Alone, doesn't relax current — needs U or boundary.
**Cross-domain wiring:** Enables `second-sound` and Poiseuille phonon flow analog to fluid dynamics.
**Notes:** Callaway (1959); Beck-Maris.

### callaway-model (cross-domain alias: `single-relaxation-time-kappa`, `kappa-callaway`)
**Domain:** Condensed Matter
**Definition:** κ_L = (ℏ²/k_BT²V)Σ_qν τ_qν v² ω² n(n+1); single-mode-RTA with separate N and U times.
**Atom or composite:** Composite (Boltzmann transport `combine` with RTA).
**Cost model:** O(N_q × N_branches).
**Real wall?** Misses full BTE coupling between N-channels and drift.
**Cross-domain wiring:** Same RTA logic as Drude conductivity; predecessor of `bte-phonon` full solutions in ShengBTE, AlmaBTE.
**Notes:** Callaway (1959).

### phonon-thermal-conductivity (cross-domain alias: `kappa-lattice`, `kl`)
**Domain:** Condensed Matter
**Definition:** κ_L from phonons; dominant in insulators and semiconductors; computed via BTE or Green-Kubo.
**Atom or composite:** Composite (anharmonic 3-phonon + boundary + isotope).
**Cost model:** ShengBTE O(N_q × N_ph_pairs); ab initio MD Green-Kubo O(N_MD).
**Real wall?** Set by anharmonicity strength and lifetime.
**Cross-domain wiring:** Engineered low κ_L drives thermoelectrics (PbTe, SnSe); high κ_L (diamond, BAs) for thermal management.
**Notes:** Broido (2007); Lindsay & Broido.

### specific-heat-lattice (cross-domain alias: `c-v-phonon`, `dulong-petit-debye`)
**Domain:** Condensed Matter
**Definition:** C_v^lat = Σ_qν ℏω_qν ∂n_qν/∂T; reduces to 3Nk_B at high T (Dulong-Petit) and ~T³ at low T (Debye).
**Atom or composite:** Composite (Bose distribution × phonon DOS).
**Cost model:** Integral over phonon DOS; O(N_modes).
**Real wall?** Quantum suppression at T<<Θ_D.
**Cross-domain wiring:** Plus electronic γT in metals; same statistical mechanics applied to bosons in cold-atoms.
**Notes:** Debye (1912); Ashcroft & Mermin Ch. 23.

### second-sound-cm (cross-domain alias: `phonon-hydrodynamics`, `temperature-wave`)
**Domain:** Condensed Matter
**Definition:** Wavelike heat transport when normal phonon scattering dominates resistive scattering; observed in solid He, Bi, graphite.
**Atom or composite:** Composite (Boltzmann hydrodynamic regime).
**Cost model:** Pulse experiments; ab-initio BTE phonon hydrodynamics.
**Real wall?** Window between strong-N and weak-U scattering — narrow temperature range.
**Cross-domain wiring:** Same hydrodynamic regime as classical fluid sound but in phonon gas; recently observed in graphite via thermal-pulse experiments.
**Notes:** Cepellotti-Marzari (2015); Huberman Science (2019).

---

## Electronic Transport

### drude-model (cross-domain alias: `electron-gas-classical`, `tau-relaxation`)
**Domain:** Condensed Matter
**Definition:** σ = ne²τ/m; classical free-electron transport with mean free time τ.
**Atom or composite:** Atom (simplest transport model).
**Cost model:** Analytic O(1).
**Real wall?** Ignores quantum statistics, band structure; surprisingly successful for ε_F >> k_BT.
**Cross-domain wiring:** Same form as `lorentz-oscillator` in photonics-optics dielectric model; enters `plasma-frequency-solid` and Hall conductivity.
**Notes:** Drude (1900); Ashcroft & Mermin Ch. 1.

### bloch-grueneisen (cross-domain alias: `electron-phonon-resistivity`, `T5-law`)
**Domain:** Condensed Matter
**Definition:** ρ(T) from electron-phonon scattering: ρ ∝ T⁵ at low T (T << Θ_D), ρ ∝ T at high T.
**Atom or composite:** Composite (phonon-electron scattering integral).
**Cost model:** Integral over phonon DOS × form factor.
**Real wall?** Yes — fundamentally tied to Debye spectrum.
**Cross-domain wiring:** Provides T-dependence verified in clean metals; basis for `eliashberg-spectral-function` α²F(ω).
**Notes:** Bloch (1930), Grüneisen (1933); Ziman Ch. 9.

### wiedemann-franz (cross-domain alias: `wf-law`, `lorenz-number`)
**Domain:** Condensed Matter
**Definition:** κ_e/σT = L = π²k_B²/3e² ≈ 2.44×10⁻⁸ WΩK⁻²; universal at low T for elastic scattering.
**Atom or composite:** Atom (`compare` thermal vs electrical transport).
**Cost model:** Trivial once σ measured.
**Real wall?** Yes — direct consequence of Sommerfeld expansion and elastic scattering.
**Cross-domain wiring:** Violated in non-Fermi liquids (strange metals) — major diagnostic in heavy-fermion and cuprate physics.
**Notes:** Wiedemann-Franz (1853); Lorenz (1872).

### mott-formula (cross-domain alias: `thermopower-cm`, `seebeck-mott`)
**Domain:** Condensed Matter
**Definition:** S = (π²k_B²T/3e)[∂ ln σ(ε)/∂ε]_εF; thermopower from energy-derivative of conductivity at Fermi level.
**Atom or composite:** Composite (Sommerfeld expansion of Boltzmann transport).
**Cost model:** Requires σ(ε) — derivative of DOS-weighted velocity².
**Real wall?** Misses phonon drag and strong-correlation contributions.
**Cross-domain wiring:** Central to thermoelectrics; ZT optimization in ml-training materials screening.
**Notes:** Mott-Jones (1936); Snyder & Toberer Nat. Mater. (2008).

### hall-effect-cm (cross-domain alias: `transverse-voltage-cm`, `r-h`)
**Domain:** Condensed Matter
**Definition:** Transverse voltage V_H = R_H I B/t developing perpendicular to current under B; R_H = 1/nq sign-gives carrier type.
**Atom or composite:** Atom (Lorentz-force `combine` with steady-state current).
**Cost model:** Two-band model O(1); ab initio Boltzmann O(N_k).
**Real wall?** Yes — direct measurement of carrier density and sign.
**Cross-domain wiring:** Generalizes to `anomalous-hall`, `spin-hall`, `valley-hall`, `quantum-hall-integer`.
**Notes:** Hall (1879); Hurd Hall Effect monograph.

### magnetoresistance (cross-domain alias: `mr`, `delta-rho-by-rho`)
**Domain:** Condensed Matter
**Definition:** Δρ(B)/ρ(0); reveals Fermi surface, multi-band physics, weak (anti)localization.
**Atom or composite:** Composite (transport tensor under B).
**Cost model:** Kohler scaling tests; full BTE O(N_k × N_τ).
**Real wall?** Linear MR (Abrikosov) in compensated semimetals; quadratic generic.
**Cross-domain wiring:** GMR/TMR underpin spintronics read heads; quantum oscillations probe `fermi-surface`.
**Notes:** Pippard textbook; Fert-Grünberg (Nobel 2007).

### boltzmann-transport-cm (cross-domain alias: `bte-solid`, `linear-response-transport`)
**Domain:** Condensed Matter
**Definition:** ∂f/∂t + v·∇f + F·∂f/∂ℏk = (∂f/∂t)_coll; semiclassical kinetic equation for distribution f(r,k,t).
**Atom or composite:** Composite (advection in phase space + collision integral).
**Cost model:** Iterative LBTE (BoltzTraP, EPW); O(N_k × N_iter).
**Real wall?** Valid when |k|·mean free path >> 1.
**Cross-domain wiring:** Generic kinetic equation shared with plasma physics and phonon transport; mean-free-path concept used in nanoscale device modeling.
**Notes:** Boltzmann (1872); Ziman Electrons and Phonons.

### landau-level-cm (cross-domain alias: `ll`, `cyclotron-quantum`)
**Domain:** Condensed Matter
**Definition:** Discrete energy levels E_n = ℏω_c(n+½) of 2D electron gas in perpendicular B; ω_c = eB/m*.
**Atom or composite:** Atom (quantization of cyclotron motion).
**Cost model:** Analytic; degeneracy per LL = eB/h per area.
**Real wall?** Quantized — basis for `iqhe` and `fqhe`.
**Cross-domain wiring:** Same harmonic-oscillator spectrum as `qho` in quantum-computing; magnetic length ℓ_B = √(ℏ/eB) sets quantum-Hall scale.
**Notes:** Landau (1930); Yoshioka book.

### iqhe (cross-domain alias: `integer-quantum-hall`, `klitzing-constant`)
**Domain:** Condensed Matter
**Definition:** σ_xy = νe²/h plateaus, ν ∈ ℤ; observed in 2DEG at high B, low T.
**Atom or composite:** Composite (Landau levels + disorder broadening + topology).
**Cost model:** Resistance metrology now defines the ohm.
**Real wall?** Yes — quantization protected by topology of filled Landau levels (Chern numbers).
**Cross-domain wiring:** First topological state of matter; precursor of `chern-insulator`, `quantum-anomalous-hall`.
**Notes:** von Klitzing (1980 Nobel); Laughlin gauge argument (1981).

### fqhe (cross-domain alias: `fractional-quantum-hall`, `laughlin-state-cm`)
**Domain:** Condensed Matter
**Definition:** σ_xy = (p/q)e²/h with p/q rational; ν = 1/3, 2/5, 5/2…
**Atom or composite:** Composite (strong correlations within partially-filled LL).
**Cost model:** ED on torus/sphere; DMRG on cylinder.
**Real wall?** Yes — fractional charge e/q quasiparticles, anyonic statistics.
**Cross-domain wiring:** Composite-fermion mapping; Moore-Read ν=5/2 hosts non-abelian anyons sought for topological quantum-computing.
**Notes:** Tsui, Stormer, Laughlin (1998 Nobel).

### quantum-hall-plateau (cross-domain alias: `hall-plateau`, `rxx-zero`)
**Domain:** Condensed Matter
**Definition:** Flat region in σ_xy(B) where ρ_xx → 0 simultaneously; topological gap protected by disorder-localized bulk states.
**Atom or composite:** Composite (localization + topology).
**Cost model:** Plateau width sets disorder strength.
**Real wall?** Yes — bulk gap pinning + chiral edge.
**Cross-domain wiring:** Bulk-boundary correspondence motif reused across topological matter.
**Notes:** Halperin (1982); Aoki-Ando.

### edge-states-cm (cross-domain alias: `chiral-edge`, `topological-boundary-modes`)
**Domain:** Condensed Matter
**Definition:** 1D conducting channels at the boundary of a 2D topological state; chirality (number, helicity) set by bulk Chern/Z₂ invariant.
**Atom or composite:** Atom (bulk-boundary correspondence).
**Cost model:** Tight-binding strip calculation O(L × N_b²).
**Real wall?** Yes — gauge anomaly inflow forbids removal without closing bulk gap.
**Cross-domain wiring:** Topological protection underlies dissipationless transport; cousin of `helical-edge` in `qsh-insulator`.
**Notes:** Halperin (1982); Hatsugai (1993).

### anomalous-hall-cm (cross-domain alias: `ahe`, `karplus-luttinger-hall`)
**Domain:** Condensed Matter
**Definition:** σ_xy ≠ 0 in ferromagnets without external B, from Berry curvature + side-jump + skew scattering.
**Atom or composite:** Composite (intrinsic Berry + extrinsic scattering).
**Cost model:** Wannier-based BZ integral O(N_k × N_b²).
**Real wall?** Symmetry: requires broken T-reversal.
**Cross-domain wiring:** Berry-curvature engineering target in spintronics, magnetic Weyl semimetals.
**Notes:** Karplus-Luttinger (1954); Nagaosa RMP (2010).

### spin-hall-effect-cm (cross-domain alias: `she`, `dyakonov-perel-hall`)
**Domain:** Condensed Matter
**Definition:** Transverse spin current under longitudinal charge current, no B; intrinsic (Berry) and extrinsic (skew, side-jump) contributions.
**Atom or composite:** Composite (spin-orbit coupling-driven Berry curvature).
**Cost model:** Linear-response Kubo over occupied states.
**Real wall?** Bounded by SOC strength.
**Cross-domain wiring:** Foundation of spin-orbit torque MRAM, spintronics signaling.
**Notes:** Murakami-Nagaosa-Zhang (2003); Sinova RMP (2015).

### valley-hall-effect (cross-domain alias: `vhe`, `graphene-bilayer-valley`)
**Domain:** Condensed Matter
**Definition:** Berry-curvature with opposite sign at K and K′ valleys drives transverse valley currents in TMDs and gapped graphene.
**Atom or composite:** Composite (valley-resolved Berry curvature).
**Cost model:** k·p Hamiltonian, analytic for massive Dirac.
**Real wall?** Inversion symmetry breaking required.
**Cross-domain wiring:** Basis of valleytronics; closely tied to `2d-material-tmd` MoS₂, WSe₂ optoelectronics.
**Notes:** Xiao-Yao-Niu (2007); Mak et al. Science (2014).

---

## Magnetism

### paramagnetism (cross-domain alias: `curie-law`, `chi-1-over-T`)
**Domain:** Condensed Matter
**Definition:** Magnetic moments noninteracting, aligned by external field; χ = C/T (Curie law) for localized spins.
**Atom or composite:** Atom (independent-moment limit).
**Cost model:** Brillouin function B_J(x); O(1).
**Real wall?** Saturates at high B/T.
**Cross-domain wiring:** Underlies EPR, NMR; baseline for stronger magnetic phases.
**Notes:** Curie (1895); Van Vleck Nobel 1977.

### diamagnetism-cm (cross-domain alias: `langevin-diamagnetism`, `chi-negative`)
**Domain:** Condensed Matter
**Definition:** Negative χ from Lenz-law response of closed shells; Landau diamagnetism gives χ_L = −(1/3)χ_P for free electrons.
**Atom or composite:** Atom (universal QM response of orbital motion).
**Cost model:** Lookup from chemical formula; O(1).
**Real wall?** Yes — present in all matter, exceeded only by stronger paramagnetism.
**Cross-domain wiring:** Perfect diamagnet = `meissner-effect`; basis of magnetic levitation, MRI shielding.
**Notes:** Langevin (1905); Landau (1930).

### ferromagnetism (cross-domain alias: `fm-order`, `weiss-mean-field`)
**Domain:** Condensed Matter
**Definition:** Spontaneous parallel alignment of spins below T_C; described by Heisenberg or Ising on positive-J lattices.
**Atom or composite:** Composite (exchange `combine` thermodynamic ordering).
**Cost model:** Mean-field T_C = JzS(S+1)/3k_B; MC corrections O(N × N_MC).
**Real wall?** Mermin-Wagner forbids true LRO in 2D Heisenberg at T>0.
**Cross-domain wiring:** Underlies hard-disk media, permanent magnets; magnonics; Ising = template for ml-training Boltzmann machines.
**Notes:** Weiss (1907); Heisenberg (1928).

### antiferromagnetism (cross-domain alias: `afm-order`, `neel-state`)
**Domain:** Condensed Matter
**Definition:** Antiparallel alignment on sublattices below T_N; staggered magnetization is order parameter; net moment zero.
**Atom or composite:** Composite (broken sublattice symmetry).
**Cost model:** Sublattice mean-field; QMC for unfrustrated bipartite lattices.
**Real wall?** Sign problem in QMC for frustrated cases.
**Cross-domain wiring:** Hosts magnons; substrate for cuprate parent compounds; AFM spintronics rising.
**Notes:** Néel (1970 Nobel).

### ferrimagnetism (cross-domain alias: `fim-order`, `unequal-sublattice`)
**Domain:** Condensed Matter
**Definition:** Antiparallel sublattices with unequal moments → net magnetization (e.g., Fe₃O₄ magnetite).
**Atom or composite:** Composite (AFM-like coupling, unequal moments).
**Cost model:** Multi-sublattice Weiss MF.
**Real wall?** Compensation point T_comp where net M = 0.
**Cross-domain wiring:** Garnets used in microwave isolators (Faraday rotation), bridging electromagnetics-antennas.
**Notes:** Néel (1948).

### heisenberg-model-cm (cross-domain alias: `quantum-spin-model`, `J-S-S`)
**Domain:** Condensed Matter
**Definition:** H = −J Σ_⟨ij⟩ Sᵢ·Sⱼ; isotropic exchange between quantum spins.
**Atom or composite:** Composite (lattice + quantum spins + exchange).
**Cost model:** ED ≤ 36 spins; DMRG large 1D/quasi-1D; QMC for J>0 bipartite (no sign problem).
**Real wall?** Spin-rotational symmetry forbids Ising-like ordering in 2D at T>0.
**Cross-domain wiring:** Cousin of `ising-model` (Z_2 anisotropy limit); template for quantum-computing variational ansätze (VQE on spin Hamiltonians).
**Notes:** Heisenberg (1928); Bethe ansatz (1931).

### ising-model-cm (cross-domain alias: `z2-spin`, `s-z-only`)
**Domain:** Condensed Matter
**Definition:** H = −J Σ_⟨ij⟩ sᵢsⱼ − h Σᵢ sᵢ, sᵢ = ±1; canonical statistical mechanics model.
**Atom or composite:** Atom (simplest interacting spin model).
**Cost model:** Onsager (2D) analytic; MC for higher D.
**Real wall?** Yes — universality-class anchor; 2D exact solution.
**Cross-domain wiring:** Same as `binary-grid` in ml-training (Boltzmann machines); cf. `lattice-gas` in physics-diffusion.
**Notes:** Ising (1925); Onsager (1944).

### xy-model (cross-domain alias: `o2-spin-model`, `planar-rotor-model`)
**Domain:** Condensed Matter
**Definition:** H = −J Σ cos(θᵢ−θⱼ); spins constrained to a plane; exhibits BKT transition in 2D.
**Atom or composite:** Composite (continuous-symmetry breaking template).
**Cost model:** MC standard; tensor-network for ground-state.
**Real wall?** No long-range order in 2D (Mermin-Wagner), but quasi-LRO + BKT.
**Cross-domain wiring:** Same group as `superfluid-helium`, `phase-field` in physics-diffusion; underlies `bkt-transition`.
**Notes:** Berezinskii (1971), Kosterlitz-Thouless (1973, 2016 Nobel).

### spin-wave (cross-domain alias: `magnon`, `bloch-wave-of-spins`)
**Domain:** Condensed Matter
**Definition:** Collective small-amplitude precession of spins in an ordered magnet; dispersion ω(q) ∝ q² (FM) or ω(q) ∝ q (AFM).
**Atom or composite:** Composite (Holstein-Primakoff bosonization of spin operators).
**Cost model:** SWT analytic; nonlinear SWT for corrections.
**Real wall?** Mermin-Wagner: 2D Heisenberg has no LRO at T>0 but magnons still defined locally.
**Cross-domain wiring:** Magnonics for low-dissipation logic; observed by inelastic neutron scattering, BLS spectroscopy.
**Notes:** Bloch (1930); Holstein-Primakoff (1940).

### bloch-t-3-2-law (cross-domain alias: `magnetization-tail`, `m-T3/2`)
**Domain:** Condensed Matter
**Definition:** ΔM(T)/M(0) ∝ −T^(3/2) at low T from thermally excited magnons in 3D FM.
**Atom or composite:** Atom (Bose-statistics of magnons + ω∝q²).
**Cost model:** Integral O(1).
**Real wall?** Universal exponent — direct test of magnon dispersion.
**Cross-domain wiring:** Analogous to phonon T³ law; combined fits constrain magnon vs phonon contributions.
**Notes:** Bloch (1930); Akhiezer textbook.

### magnetic-anisotropy (cross-domain alias: `mae`, `easy-axis-energy`)
**Domain:** Condensed Matter
**Definition:** Energy difference between magnetization directions; arises from SOC, dipolar, shape anisotropy.
**Atom or composite:** Composite (SOC + crystal field + shape).
**Cost model:** DFT+SOC noncollinear; meV scale per atom requires high accuracy.
**Real wall?** Sets thermal stability of bits in magnetic memory (KuV/k_BT ≥ 50).
**Cross-domain wiring:** Engineering target for permanent magnets, MRAM; ML potentials must reproduce K_u(c/a) trends.
**Notes:** Daalderop et al. (1990); Brooks (2014).

### magnetic-domain (cross-domain alias: `weiss-domain`, `m-region`)
**Domain:** Condensed Matter
**Definition:** Region of uniform magnetization separated from neighbors by domain walls; minimizes magnetostatic energy.
**Atom or composite:** Composite (energy minimization across length scales).
**Cost model:** Micromagnetics (LLG) O(N_grid × N_t).
**Real wall?** Domain size ∝ √(A/K_u); below this, single-domain particle.
**Cross-domain wiring:** Domain imaging via Kerr, Lorentz TEM, NV centers; OOMMF / mumax simulations.
**Notes:** Weiss (1907); Brown Micromagnetics.

### bloch-wall (cross-domain alias: `180-deg-bloch`, `out-of-plane-rotation`)
**Domain:** Condensed Matter
**Definition:** Domain wall where M rotates out of the wall plane; width δ_B = π√(A/K_u).
**Atom or composite:** Atom (canonical 3D wall solution).
**Cost model:** Analytic in continuum micromagnetic limit.
**Real wall?** Wall energy γ_B = 4√(AK_u).
**Cross-domain wiring:** Soliton solution of the sine-Gordon family — same math in physics-diffusion solitons.
**Notes:** Bloch (1932); Hubert & Schäfer book.

### neel-wall (cross-domain alias: `in-plane-domain-wall`, `thin-film-wall`)
**Domain:** Condensed Matter
**Definition:** Domain wall where M rotates in the film plane; favored in thin films.
**Atom or composite:** Atom (planar wall solution).
**Cost model:** Continuum LLG solution.
**Real wall?** Crossover from Bloch to Néel near film thickness ≈ Bloch length.
**Cross-domain wiring:** Asymmetric Dzyaloshinskii-Néel walls underlie chiral magnetism, skyrmions.
**Notes:** Néel (1955).

### hysteresis-cm (cross-domain alias: `mh-loop`, `magnetic-memory-loop`)
**Domain:** Condensed Matter
**Definition:** Magnetization lag behind H sweeps; characterized by coercivity H_c, remanence M_r, saturation M_s.
**Atom or composite:** Composite (energy-barrier hopping among metastable states).
**Cost model:** Stoner-Wohlfarth O(1); Preisach hysterons; LLG simulation O(N_t).
**Real wall?** Yes — sets practical magnetic-storage retention.
**Cross-domain wiring:** Same loop structure in ferroelectrics, structural phase transitions; ML emulators predict loops.
**Notes:** Ewing (1881); Stoner-Wohlfarth (1948).

### exchange-bias (cross-domain alias: `pinned-layer`, `fm-afm-interface`)
**Domain:** Condensed Matter
**Definition:** Shifted hysteresis loop in a FM/AFM heterostructure due to interfacial exchange coupling pinning FM moments.
**Atom or composite:** Composite (interfacial Heisenberg exchange between FM and AFM).
**Cost model:** DFT slab + Monte Carlo Heisenberg.
**Real wall?** Yes — only below T_blocking < T_Néel.
**Cross-domain wiring:** Backbone of spin-valves, hard-disk read heads, TMR sensors.
**Notes:** Meiklejohn-Bean (1956); Nogués review (2005).

### rkky-interaction (cross-domain alias: `ruderman-kittel-kasuya-yosida`, `oscillating-exchange`)
**Domain:** Condensed Matter
**Definition:** Indirect exchange J_RKKY(r) ∝ cos(2k_F r)/r³ between localized moments mediated by conduction electrons.
**Atom or composite:** Composite (second-order perturbation through Fermi sea).
**Cost model:** Analytic for free electrons; ab initio for realistic Fermi surfaces.
**Real wall?** Long-range, oscillatory — drives spin glasses in dilute alloys.
**Cross-domain wiring:** Underlies giant magnetoresistance multilayers, magnetic semiconductors.
**Notes:** Ruderman-Kittel (1954), Kasuya, Yosida.

### dzyaloshinskii-moriya (cross-domain alias: `dmi`, `antisymmetric-exchange`)
**Domain:** Condensed Matter
**Definition:** H_DM = D·(Sᵢ×Sⱼ); antisymmetric exchange from SOC + broken inversion; favors canted/chiral spin order.
**Atom or composite:** Atom (SOC-induced chiral coupling).
**Cost model:** DFT noncollinear ΔE between chiralities.
**Real wall?** Required for `skyrmion-cm`, helical magnets, chiral domain walls.
**Cross-domain wiring:** Backbone of skyrmion racetrack memory; same antisymmetric coupling in photonics-optics Faraday-type effects.
**Notes:** Dzyaloshinskii (1958); Moriya (1960).

### spin-orbit-coupling-cm (cross-domain alias: `soc`, `lambda-L-dot-S`)
**Domain:** Condensed Matter
**Definition:** H_SOC = λ L·S; relativistic correction linking spin and orbital degrees of freedom; scales as Z⁴ for heavy elements.
**Atom or composite:** Atom (relativistic Dirac → Pauli reduction).
**Cost model:** Treated as perturbation or full noncollinear DFT.
**Real wall?** Yes — fundamental relativistic effect; cannot be screened.
**Cross-domain wiring:** Enables `topological-insulator`, `spin-hall`, magnetic anisotropy; sets the scale of fine structure in optical spectra.
**Notes:** Pauli; Dirac; Bertotti SOC review.

---

## Frustrated Magnetism & Spin Liquids

### spin-glass (cross-domain alias: `quenched-disorder-magnet`, `edwards-anderson`)
**Domain:** Condensed Matter
**Definition:** Random-bond magnet with no long-range order but frozen, glassy spin configurations below T_g; Edwards-Anderson order parameter q ≠ 0.
**Atom or composite:** Composite (random exchange + frustration).
**Cost model:** Replica-symmetry-breaking (Parisi); MC with long equilibration.
**Real wall?** Yes — ergodicity broken; aging, memory effects.
**Cross-domain wiring:** Parisi RSB underlies disordered systems across ml-training (deep network landscapes), neural-net energy landscapes.
**Notes:** Edwards-Anderson (1975); Parisi (Nobel 2021).

### geometric-frustration (cross-domain alias: `frustration-lattice`, `triangle-no-anti-align`)
**Domain:** Condensed Matter
**Definition:** Impossibility of simultaneously satisfying all pairwise interactions due to lattice geometry (triangular, kagome, pyrochlore).
**Atom or composite:** Atom (lattice topology vs interaction sign).
**Cost model:** Macroscopic ground-state degeneracy.
**Real wall?** Yes — extensive entropy at T=0 (Pauling ice rule); kills mean-field.
**Cross-domain wiring:** Drives `spin-liquid`, residual entropy; mirrors constraint-satisfaction problems in ml-training.
**Notes:** Wannier (1950); Ramirez review (1994).

### kagome-lattice (cross-domain alias: `corner-sharing-triangles`, `kagome-net`)
**Domain:** Condensed Matter
**Definition:** 2D lattice of corner-sharing triangles; hosts flat bands + Dirac cones in tight-binding; iconic for frustration.
**Atom or composite:** Composite (3-site basis on triangular Bravais lattice).
**Cost model:** 3-band tight-binding analytic.
**Real wall?** Geometric flat band — diverging DOS.
**Cross-domain wiring:** Hosts QSL candidates (herbertsmithite); Co₃Sn₂S₂ kagome Weyl semimetal; analog of `lieb-flat-band`.
**Notes:** Mielke; Norman RMP (2016).

### pyrochlore-lattice (cross-domain alias: `corner-sharing-tetrahedra`, `a2b2o7`)
**Domain:** Condensed Matter
**Definition:** 3D network of corner-sharing tetrahedra; host of spin ice and 3D quantum spin liquids.
**Atom or composite:** Composite (tetrahedral motif).
**Cost model:** Single-tetrahedron approximation + MC.
**Real wall?** Pauling 2-in-2-out ice rule yields Pauling entropy.
**Cross-domain wiring:** Spin-ice realizes magnetic monopole-like excitations — direct CM analog of EM monopoles.
**Notes:** Bramwell-Gingras Science (2001).

### triangular-lattice (cross-domain alias: `2d-frustrated-bravais`, `s-equilateral`)
**Domain:** Condensed Matter
**Definition:** 2D Bravais lattice with 60° basis vectors; smallest geometrically frustrated lattice for AFM Ising.
**Atom or composite:** Atom (planar densest packing).
**Cost model:** Wannier (1950) analytic ground-state entropy = 0.323 k_B.
**Real wall?** Macroscopic degeneracy of AFM Ising ground state.
**Cross-domain wiring:** Host of 120° Heisenberg order; substrate for moiré flat bands when stacked.
**Notes:** Wannier (1950); Anderson RVB (1973).

### spin-ice (cross-domain alias: `pauling-ice-magnetic`, `2in-2out`)
**Domain:** Condensed Matter
**Definition:** Ising spins on pyrochlore obeying 2-in/2-out ice rule per tetrahedron; emergent magnetic monopoles as excitations.
**Atom or composite:** Composite (pyrochlore + Ising anisotropy + dipolar).
**Cost model:** Monte Carlo with dipolar Ewald.
**Real wall?** Yes — Coulomb gas of magnetic monopoles.
**Cross-domain wiring:** Direct lab analog of Dirac magnetic monopoles; fractionalized excitations.
**Notes:** Castelnovo-Moessner-Sondhi Nature (2008).

### kitaev-model (cross-domain alias: `honeycomb-bond-dependent`, `kitaev-honeycomb`)
**Domain:** Condensed Matter
**Definition:** S=½ on honeycomb with bond-direction-dependent Ising: H = −Σ K_α S_i^α S_j^α; exactly solvable via Majorana fermionization; spin-liquid phases.
**Atom or composite:** Composite (Majorana decomposition + Z₂ gauge field).
**Cost model:** Free-Majorana band structure O(N³).
**Real wall?** Exactly solvable Z_2 spin liquid; gapped vs gapless phases.
**Cross-domain wiring:** Predicted in α-RuCl₃, Na₂IrO₃; non-abelian phase relevant to topological quantum-computing.
**Notes:** Kitaev (2006); Trebst RoPP (2017).

### z2-spin-liquid (cross-domain alias: `toric-code-spin`, `z2-qsl`)
**Domain:** Condensed Matter
**Definition:** Gapped quantum spin liquid with Z_2 topological order; vison + spinon excitations; ground-state degeneracy 4 on torus.
**Atom or composite:** Composite (emergent Z_2 gauge field).
**Cost model:** Variational PEPS, DMRG.
**Real wall?** Yes — topological order classified by anyon content.
**Cross-domain wiring:** Same gauge structure as `toric-code` in quantum-computing; potential resource for fault-tolerant qubits.
**Notes:** Anderson (1987); Read-Sachdev (1991); Kitaev (2003).

### u1-spin-liquid (cross-domain alias: `algebraic-spin-liquid`, `gapless-qsl`)
**Domain:** Condensed Matter
**Definition:** Gapless quantum spin liquid with emergent U(1) gauge field and Dirac/spinon Fermi surface; algebraic spin correlations.
**Atom or composite:** Composite (slave-fermion mean field + gauge fluctuations).
**Cost model:** DMFT-like and large-N expansions.
**Real wall?** Stability against confinement requires sufficient matter fields.
**Cross-domain wiring:** Conjectured in herbertsmithite kagome; same emergent QED structure as some lattice gauge theories.
**Notes:** Hermele-Senthil-Fisher (2008).

### emergent-monopole (cross-domain alias: `magnetic-monopole-cm`, `spinon-deconfinement`)
**Domain:** Condensed Matter
**Definition:** Fractional excitations carrying magnetic charge in spin-ice or U(1) QSL; couple via emergent gauge field.
**Atom or composite:** Atom (fractionalized excitation of emergent gauge theory).
**Cost model:** Observed via specific-heat anomalies, magnetic-Wien effect.
**Real wall?** Yes — confinement-deconfinement transitions exist.
**Cross-domain wiring:** Lab realization of Dirac-style monopoles; conceptual import for gauge theories beyond CM.
**Notes:** Castelnovo-Moessner-Sondhi (2008).

---

## Superconductivity

### meissner-effect (cross-domain alias: `flux-expulsion`, `perfect-diamagnetism`)
**Domain:** Condensed Matter
**Definition:** Active expulsion of magnetic flux B from a superconductor below T_c, B = 0 in bulk (type I) or restricted to vortices (type II).
**Atom or composite:** Atom (defining property of superconductivity).
**Cost model:** London equations O(1) analytic.
**Real wall?** Yes — distinct from perfect conductor; reversibility under field cooling.
**Cross-domain wiring:** Manifestation of broken U(1) gauge symmetry; analogous to Higgs mechanism gauge-boson mass acquisition.
**Notes:** Meissner-Ochsenfeld (1933).

### london-equations (cross-domain alias: `local-electrodynamics-sc`, `lambda-L`)
**Domain:** Condensed Matter
**Definition:** ∂J/∂t = (n_s e²/m)E; ∇×J = −(n_s e²/m)B; yield B(x) = B₀e^(−x/λ_L) with λ_L = √(m/μ₀n_s e²).
**Atom or composite:** Composite (constitutive relation for superconducting fluid).
**Cost model:** Analytic O(1).
**Real wall?** Local form fails when ξ_0 < λ_L (Pippard nonlocal).
**Cross-domain wiring:** Same exponential screening as `skin-depth` in electromagnetics-antennas; defines penetration depth λ_L.
**Notes:** F. & H. London (1935).

### pippard-nonlocal (cross-domain alias: `nonlocal-electrodynamics-sc`, `xi-zero-vs-lambda`)
**Domain:** Condensed Matter
**Definition:** Current at point r averages over volume of radius ξ_0 — coherence length introduces nonlocal kernel like an anomalous skin effect.
**Atom or composite:** Composite (London + spatial coherence).
**Cost model:** Convolution kernel; analytic in q-space.
**Real wall?** Required for clean SCs where ξ_0 >> λ.
**Cross-domain wiring:** Analogous to anomalous skin effect in normal metals; bridges to clean-limit BCS.
**Notes:** Pippard (1953).

### ginzburg-landau-cm (cross-domain alias: `gl-theory-cm`, `psi-order-parameter`)
**Domain:** Condensed Matter
**Definition:** Free energy F = α|ψ|² + (β/2)|ψ|⁴ + (1/2m*)|(−iℏ∇−2eA)ψ|² + B²/2μ₀; ψ = SC complex order parameter.
**Atom or composite:** Composite (field-theoretic Landau description).
**Cost model:** Variational / TDGL O(N_grid × N_t).
**Real wall?** Valid near T_c.
**Cross-domain wiring:** Same Mexican-hat as Higgs; vortices analogous to topological defects across photonics-optics, BEC.
**Notes:** Ginzburg-Landau (1950); Abrikosov derivation.

### coherence-length-sc (cross-domain alias: `xi-zero`, `pair-size`)
**Domain:** Condensed Matter
**Definition:** Spatial extent of Cooper pair / amplitude variation: ξ_0 = ℏv_F/πΔ (BCS).
**Atom or composite:** Atom (intrinsic SC length scale).
**Cost model:** O(1) from gap and Fermi velocity.
**Real wall?** Yes — sets vortex core size and quasiparticle confinement.
**Cross-domain wiring:** Compares with `penetration-depth-sc` via κ = λ/ξ; determines type-I vs type-II.
**Notes:** Pippard (1953); Tinkham Ch. 1.

### penetration-depth-sc (cross-domain alias: `lambda-l-sc`, `magnetic-field-decay-sc`)
**Domain:** Condensed Matter
**Definition:** λ = √(m*/μ₀n_s e²); length over which magnetic field decays inside SC.
**Atom or composite:** Atom (London magnetic length scale).
**Cost model:** O(1) from n_s.
**Real wall?** λ(T) → ∞ as T→T_c.
**Cross-domain wiring:** Mirrors plasma skin depth in conducting plasmas; measured via μSR, microwave surface impedance.
**Notes:** London (1935); Tinkham Ch. 3.

### type-i-vs-type-ii (cross-domain alias: `gl-parameter-kappa`, `1-over-sqrt-2-threshold`)
**Domain:** Condensed Matter
**Definition:** κ = λ/ξ; κ<1/√2 (type I) expels flux completely up to H_c; κ>1/√2 (type II) admits vortex lattice between H_c1 and H_c2.
**Atom or composite:** Atom (dimensionless ratio classifies SCs).
**Cost model:** O(1) ratio.
**Real wall?** Yes — sets mixed-state physics in all engineering SCs.
**Cross-domain wiring:** Determines whether NbTi/Nb₃Sn (type II) can be used in high-field magnets.
**Notes:** Ginzburg-Landau-Abrikosov-Gor'kov (GLAG).

### abrikosov-vortex (cross-domain alias: `flux-tube`, `phi-0-quantum`)
**Domain:** Condensed Matter
**Definition:** Topological excitation in type-II SC carrying flux Φ₀ = h/2e; arranged in triangular lattice for ideal samples.
**Atom or composite:** Composite (vortex solution of GL equations).
**Cost model:** TDGL or vortex-dynamics simulations.
**Real wall?** Quantized flux Φ₀ enforces lattice density n_v = B/Φ₀.
**Cross-domain wiring:** Same topological soliton as superfluid vortices, cosmic strings, BEC quantized vortices.
**Notes:** Abrikosov (2003 Nobel).

### flux-quantization (cross-domain alias: `phi-naught`, `h-over-2e`)
**Domain:** Condensed Matter
**Definition:** Magnetic flux through a SC ring is quantized in units of Φ₀ = h/2e, evidencing Cooper pairing.
**Atom or composite:** Atom (single-valuedness of macroscopic wave function).
**Cost model:** Direct experimental measurement.
**Real wall?** Yes — exact quantization from gauge invariance.
**Cross-domain wiring:** Basis of `squid` magnetometers; same flux quantization in `iqhe` per Landau level.
**Notes:** Deaver-Fairbank, Doll-Näbauer (1961).

### bcs-theory (cross-domain alias: `bcs`, `phonon-mediated-pairing`)
**Domain:** Condensed Matter
**Definition:** Microscopic theory of SC via phonon-mediated attractive interaction producing Cooper pairs; gap equation Δ = V Σ_k Δ/2E_k tanh(E_k/2T).
**Atom or composite:** Composite (mean-field pairing + Bogoliubov diagonalization).
**Cost model:** Self-consistent gap equation O(N_k × N_iter).
**Real wall?** Weak-coupling BCS: Δ(0)/k_BT_c = 1.764; isotope effect α=1/2.
**Cross-domain wiring:** Same condensate physics as BEC, fermionic atomic gases (BEC-BCS crossover); ml-training-style mean-field self-consistency.
**Notes:** Bardeen-Cooper-Schrieffer (1957 Nobel 1972).

### cooper-pair (cross-domain alias: `bound-electron-pair`, `s-wave-pair`)
**Domain:** Condensed Matter
**Definition:** Bound state of two electrons near the Fermi surface with opposite momenta and spins, formed by arbitrarily weak attractive interaction.
**Atom or composite:** Atom (basic pairing unit).
**Cost model:** Cooper's two-body problem analytic.
**Real wall?** Yes — instability of normal Fermi sea to pairing for V<0.
**Cross-domain wiring:** Macroscopic occupation of pair state → BCS condensate; bosonic pair is precursor to BEC of molecules.
**Notes:** Cooper (1956).

### bogoliubov-de-gennes (cross-domain alias: `bdg-equations`, `particle-hole-hamiltonian`)
**Domain:** Condensed Matter
**Definition:** 2N×2N Hamiltonian in particle-hole space: H_BdG = [[H₀−μ, Δ],[Δ†, −(H₀−μ)*]]; eigenvalues come in ±E pairs.
**Atom or composite:** Composite (Nambu spinor + pairing).
**Cost model:** O(N³) diagonalization.
**Real wall?** Particle-hole symmetry inherent; zero modes = Majoranas.
**Cross-domain wiring:** Backbone for inhomogeneous SC, vortex cores, SC/F junctions, Majorana physics in topological wires.
**Notes:** de Gennes (1966).

### s-wave-pairing (cross-domain alias: `singlet-spherical-pair`, `bcs-symmetry`)
**Domain:** Condensed Matter
**Definition:** Isotropic gap Δ(k) = Δ; spin singlet; canonical BCS.
**Atom or composite:** Atom (simplest gap symmetry).
**Cost model:** Spherical k-shell sum.
**Real wall?** Robust against nonmagnetic disorder (Anderson theorem).
**Cross-domain wiring:** Default in conventional SCs; baseline for comparison with unconventional symmetries.
**Notes:** BCS (1957).

### p-wave-pairing (cross-domain alias: `triplet-pair`, `topological-sc-1d`)
**Domain:** Condensed Matter
**Definition:** Pair amplitude odd in k, even-spin (triplet); chiral p_x±ip_y gap hosts Majorana edge modes.
**Atom or composite:** Composite (l=1 angular pairing).
**Cost model:** Self-consistent BdG with anisotropic Δ.
**Real wall?** Disorder-fragile (Anderson theorem violated).
**Cross-domain wiring:** Proposed in Sr₂RuO₄ (now contested); engineered in semiconductor-SC-magnet hybrids (Kitaev wire) for topological qubits.
**Notes:** Read-Green (2000); Kitaev (2001).

### d-wave-pairing (cross-domain alias: `d-x2-y2`, `cuprate-pairing`)
**Domain:** Condensed Matter
**Definition:** Gap Δ(k) = Δ_0 (cos k_x − cos k_y); nodes along zone diagonals; singlet, l=2.
**Atom or composite:** Composite (anisotropic singlet gap).
**Cost model:** Self-consistent gap equation with momentum dependence.
**Real wall?** Sensitive to nonmagnetic disorder; T^? power laws at low T from nodes.
**Cross-domain wiring:** Canonical for cuprates; phase-sensitive measurements (Tsuei-Kirtley) confirmed sign change.
**Notes:** Scalapino review (1995); Tsuei-Kirtley RMP (2000).

### eliashberg-theory (cross-domain alias: `strong-coupling-bcs`, `alpha2f`)
**Domain:** Condensed Matter
**Definition:** Strong-coupling extension of BCS using retarded electron-phonon interaction α²F(ω); self-consistent Migdal-Eliashberg equations.
**Atom or composite:** Composite (Migdal approximation + BCS pairing).
**Cost model:** Matsubara-frequency self-consistent eqs; ab initio via EPW, SCDFT.
**Real wall?** Captures λ>0.3 strong-coupling effects missed by BCS.
**Cross-domain wiring:** Underpins ab initio T_c prediction; α²F same kernel used in transport (Bloch-Grüneisen).
**Notes:** Eliashberg (1960); Allen-Mitrovic.

### mcmillan-formula (cross-domain alias: `tc-estimate-eliashberg`, `allen-dynes`)
**Domain:** Condensed Matter
**Definition:** T_c = (Θ_D/1.45) exp(−1.04(1+λ)/(λ−μ*(1+0.62λ))); approximate Eliashberg-based T_c.
**Atom or composite:** Composite (closed-form fit to Eliashberg solutions).
**Cost model:** O(1) given λ, μ*, ⟨ω_log⟩.
**Real wall?** Empirically calibrated; breaks down at very strong coupling.
**Cross-domain wiring:** Used in high-pressure hydride SC predictions (H₃S, LaH₁₀); ML T_c models in ml-training.
**Notes:** McMillan (1968); Allen-Dynes (1975).

### isotope-effect-cm (cross-domain alias: `m-alpha-tc`, `phonon-fingerprint`)
**Domain:** Condensed Matter
**Definition:** T_c ∝ M^(−α), α≈0.5 for BCS phonon-mediated; α<<0.5 in cuprates suggests non-phonon mechanism.
**Atom or composite:** Atom (mass scaling test).
**Cost model:** Compare T_c across isotopes.
**Real wall?** Yes — sensitive probe of pairing glue.
**Cross-domain wiring:** Smoking-gun for phonon role; absent in many unconventional SCs.
**Notes:** Maxwell, Reynolds (1950).

### josephson-effect (cross-domain alias: `tunneling-supercurrent`, `dc-ac-josephson`)
**Domain:** Condensed Matter
**Definition:** DC: I = I_c sin(Δφ); AC: V = (ℏ/2e)dφ/dt; supercurrent through weak link controlled by phase difference.
**Atom or composite:** Composite (macroscopic phase coherence + tunneling).
**Cost model:** Resistively-shunted-junction model O(N_t).
**Real wall?** Yes — exact for SIS junctions in weak-coupling limit.
**Cross-domain wiring:** Voltage standard (Josephson volt); basis of SQUID and superconducting qubits in quantum-computing.
**Notes:** Josephson (1962, 1973 Nobel).

### squid-cm (cross-domain alias: `superconducting-interference-device`, `magnetometer-quantum`)
**Domain:** Condensed Matter
**Definition:** Loop with two Josephson junctions; critical current modulates with flux period Φ₀; sensitivity ~fT/√Hz.
**Atom or composite:** Composite (Josephson + flux quantization in ring).
**Cost model:** Real device; readout electronics.
**Real wall?** Ultimate magnetometer; limited by Φ_0/A and noise temperature.
**Cross-domain wiring:** Magnetoencephalography (MEG) imaging; geophysics; superconducting qubit readout.
**Notes:** Clarke & Braginski Vol. 1.

### andreev-reflection (cross-domain alias: `electron-hole-conversion`, `normal-sc-boundary`)
**Domain:** Condensed Matter
**Definition:** Incoming electron of energy E<Δ from N reflects as a hole, transferring a Cooper pair into the SC.
**Atom or composite:** Composite (BdG boundary-condition process).
**Cost model:** BTK formula analytic.
**Real wall?** Doubles subgap conductance — diagnostic of SC gap.
**Cross-domain wiring:** Probes pairing symmetry, zero-bias peaks ↔ Majorana physics, proximity gap.
**Notes:** Andreev (1964); Blonder-Tinkham-Klapwijk (1982).

### proximity-effect (cross-domain alias: `induced-pairing`, `s-n-coupling`)
**Domain:** Condensed Matter
**Definition:** Cooper pair amplitude leaks into a normal metal in contact with SC, with decay length ~ℏv_F/k_BT (Usadel for dirty).
**Atom or composite:** Composite (Andreev + diffusion).
**Cost model:** Usadel equations O(N_grid × self-consist).
**Real wall?** Limited by exchange-field/SOC in F or N.
**Cross-domain wiring:** Builds topological SCs by combining SC + semiconductor wire + B; basis of Lutchyn-Sau-Das Sarma proposal.
**Notes:** de Gennes Boundary effects (1964).

### cuprate-sc (cross-domain alias: `high-tc-cuprate`, `ybco-bscco`)
**Domain:** Condensed Matter
**Definition:** Family of layered copper-oxide SCs with T_c up to 138 K (HgBaCaCuO); d-wave pairing; pseudogap phenomenology.
**Atom or composite:** Composite (Mott-insulating parent + doping + 2D CuO₂ planes).
**Cost model:** Hubbard / t-J model + DMFT, DCA, DQMC.
**Real wall?** Mechanism still debated 40 years on.
**Cross-domain wiring:** Engine for cluster-DMFT, ML phase-classification, neutron and ARPES advances.
**Notes:** Bednorz-Müller (1986 Nobel).

### iron-pnictide-sc (cross-domain alias: `fe-based-sc`, `s-plus-minus-pairing`)
**Domain:** Condensed Matter
**Definition:** Layered iron arsenide/selenide SCs with T_c up to 56 K; multiband; proposed s±-wave pairing with sign change between electron and hole pockets.
**Atom or composite:** Composite (multiorbital Hubbard + nesting-driven SC).
**Cost model:** RPA, FLEX, FRG on multiorbital models.
**Real wall?** Different parent state (bad metal) than cuprates.
**Cross-domain wiring:** Provided second high-T_c family; emphasizes role of multiband nesting and spin fluctuations.
**Notes:** Kamihara (2008); Mazin-Singh-Johannes-Du (2008).

### heavy-fermion-sc (cross-domain alias: `hf-sc`, `unconventional-low-tc`)
**Domain:** Condensed Matter
**Definition:** SC emerging from Kondo-lattice heavy electrons (CeCu₂Si₂ etc.); often near AFM quantum critical points; T_c ~ 1 K but m* ~10²–10³ m_e.
**Atom or composite:** Composite (Kondo coherence + magnetic-fluctuation pairing).
**Cost model:** DMFT + cluster expansions.
**Real wall?** Limited by competition with AFM; ξ_0 huge in units of lattice.
**Cross-domain wiring:** Sets paradigm for non-phonon pairing; QCP physics shared with cuprates, pnictides.
**Notes:** Steglich CeCu₂Si₂ (1979).

### twisted-bilayer-graphene-sc (cross-domain alias: `tbg-sc`, `magic-angle-sc`)
**Domain:** Condensed Matter
**Definition:** SC observed in twisted bilayer graphene at magic angle (~1.1°) flat bands; T_c~3 K but enormous T_c/E_F ratio.
**Atom or composite:** Composite (moiré flat band + Coulomb interaction).
**Cost model:** Continuum BM model + Hartree-Fock + RPA.
**Real wall?** Strongly correlated; mechanism unsettled.
**Cross-domain wiring:** Marries 2D-material engineering with flat-band physics; tunable platform for cuprate-like phenomena.
**Notes:** Cao-Jarillo-Herrero Nature (2018).

---

## Topological Matter

### topological-insulator-2d (cross-domain alias: `qsh-insulator`, `kane-mele-state`)
**Domain:** Condensed Matter
**Definition:** Bulk-gapped 2D state with Z_2 invariant ν=1; helical edge states protected by time-reversal symmetry.
**Atom or composite:** Composite (SOC-induced band inversion + T-symmetry).
**Cost model:** Z_2 invariant via Wilson loop O(N_k × N_b²).
**Real wall?** Yes — bulk-boundary correspondence guarantees gapless edge.
**Cross-domain wiring:** Realized in HgTe/CdTe QWs (BHZ model); spin-momentum locking analog in photonic-optics photonic TIs.
**Notes:** Kane-Mele (2005); König et al. Science (2007).

### topological-insulator-3d (cross-domain alias: `3d-ti`, `bi2se3-class`)
**Domain:** Condensed Matter
**Definition:** 3D state with four Z_2 invariants (ν₀;ν₁ν₂ν₃); strong TI hosts odd number of Dirac cones on each surface.
**Atom or composite:** Composite (3D band inversion at TRIM points).
**Cost model:** Fu-Kane parity criterion O(N_TRIM × N_b).
**Real wall?** Robust to disorder preserving T-symmetry.
**Cross-domain wiring:** Bi₂Se₃, Bi₂Te₃ surface states probed by ARPES; foundation for proposed topological-SC Majorana physics.
**Notes:** Fu-Kane-Mele (2007); Hsieh Nature (2008).

### z2-invariant (cross-domain alias: `kane-mele-invariant`, `time-reversal-topology`)
**Domain:** Condensed Matter
**Definition:** ν ∈ {0,1} characterizing T-invariant insulators; ν=1 means topologically nontrivial.
**Atom or composite:** Atom (mod-2 reduction of Chern-like quantity).
**Cost model:** Wilson loop, Pfaffian formula, or parity (when inversion present).
**Real wall?** Yes — gauge-invariant integer mod 2.
**Cross-domain wiring:** Discrete analog of Chern; generalizes to Z₂² in 3D; cousin of Stiefel-Whitney classes.
**Notes:** Kane-Mele (2005); Fu-Kane (2007).

### kane-mele-model (cross-domain alias: `km-model`, `honeycomb-with-soc`)
**Domain:** Condensed Matter
**Definition:** Graphene tight-binding + intrinsic SOC (next-nearest-neighbor imaginary hopping); first theoretical Z_2 TI.
**Atom or composite:** Composite (graphene + Haldane-like SOC).
**Cost model:** 4-band tight-binding analytic.
**Real wall?** SOC in graphene tiny (~μeV) — practical realization needs engineered systems.
**Cross-domain wiring:** Prototype for 2D Z_2 topology; experimentally realized in Bi/Cu(111) and stanene.
**Notes:** Kane-Mele (2005).

### bhz-model (cross-domain alias: `bernevig-hughes-zhang`, `hgte-quantum-well`)
**Domain:** Condensed Matter
**Definition:** 4-band k·p model of HgTe/CdTe QW; transitions to QSH state when well thickness exceeds critical d_c≈6.3 nm.
**Atom or composite:** Composite (heterostructure-driven band inversion).
**Cost model:** Analytic 4×4 Hamiltonian.
**Real wall?** Mass term M(k) crosses zero with thickness.
**Cross-domain wiring:** Foundational example of band-inversion topological transition; experimentally confirmed.
**Notes:** Bernevig-Hughes-Zhang (2006).

### topological-surface-state (cross-domain alias: `dirac-cone-surface`, `spin-locked-surface`)
**Domain:** Condensed Matter
**Definition:** Gapless 2D surface states of 3D TI with linear Dirac dispersion and spin-momentum locking.
**Atom or composite:** Composite (bulk-boundary correspondence projection).
**Cost model:** ARPES, STM, transport; slab tight-binding.
**Real wall?** Yes — protected by T-symmetry + bulk gap; gapped only by magnetic perturbations.
**Cross-domain wiring:** Hosts predicted Majorana modes when proximity-coupled to s-wave SC.
**Notes:** Hsieh et al. Nature (2008).

### helical-edge-state (cross-domain alias: `spin-momentum-locked-edge`, `qsh-edge`)
**Domain:** Condensed Matter
**Definition:** 1D edge channels of 2D Z_2 TI: right-movers spin up, left-movers spin down (Kramers pair).
**Atom or composite:** Atom (1D helical Luttinger liquid).
**Cost model:** Quantized G = 2e²/h conductance.
**Real wall?** Yes — backscattering suppressed by T-symmetry.
**Cross-domain wiring:** Underlies dissipationless edge transport; ingredients for fractional helical liquid proposals.
**Notes:** König et al. (2007).

### dirac-semimetal-cm (cross-domain alias: `dsm`, `cd3as2-na3bi`)
**Domain:** Condensed Matter
**Definition:** 3D bulk band-crossing with linear dispersion in all directions; symmetry-protected 4-fold Dirac points.
**Atom or composite:** Composite (band inversion + rotational symmetry).
**Cost model:** k·p with C_n symmetry constraints.
**Real wall?** Dirac points protected by crystal symmetry; breaking gives TI or Weyl.
**Cross-domain wiring:** 3D analog of graphene; ARPES, magnetotransport probes; ML descriptors target band touchings.
**Notes:** Wang Na₃Bi (2012); Liu Cd₃As₂ (2014).

### weyl-semimetal (cross-domain alias: `wsm`, `chiral-anomaly-material`)
**Domain:** Condensed Matter
**Definition:** Pair (or more) of nondegenerate band crossings with chirality ±1; sources/sinks of Berry curvature; Fermi-arc surface states.
**Atom or composite:** Composite (broken inversion or T-symmetry + band crossing).
**Cost model:** k·p Weyl Hamiltonian; Wannier-based ab initio.
**Real wall?** Weyl points robust until they pair-annihilate.
**Cross-domain wiring:** Realizes chiral anomaly in solid state; same math as Weyl fermions in particle physics.
**Notes:** Wan et al. (2011); TaAs experiments Xu/Lv (2015).

### nodal-line-semimetal (cross-domain alias: `nls`, `1d-band-touching`)
**Domain:** Condensed Matter
**Definition:** 3D bulk hosts 1D loops of band crossings; protected by mirror symmetry (in absence of SOC); drumhead surface states.
**Atom or composite:** Composite (line of accidental degeneracies).
**Cost model:** k·p along the loop; ab initio search.
**Real wall?** SOC typically gaps the line; needs symmetry protection.
**Cross-domain wiring:** Drumhead surface states predicted in Cu₃PdN, CaP₃; ML predicting NLS candidates.
**Notes:** Burkov-Hook-Balents (2011).

### chern-insulator (cross-domain alias: `qah-state`, `haldane-class`)
**Domain:** Condensed Matter
**Definition:** 2D insulator with broken T-symmetry and nonzero Chern number, exhibiting QH-like edge transport at B=0.
**Atom or composite:** Composite (Haldane-style flux pattern + gap).
**Cost model:** Same as `tknn-formula`.
**Real wall?** Yes — protected by gap and topology.
**Cross-domain wiring:** Realized in Cr-doped Bi₂Se₃ thin films; magic-angle graphene; bridges to integer QH at B=0.
**Notes:** Haldane (1988, 2016 Nobel); Chang et al. Science (2013).

### axion-insulator (cross-domain alias: `theta-pi-state`, `magnetic-topological`)
**Domain:** Condensed Matter
**Definition:** 3D TI with effective axion angle θ=π; gapped surfaces with half-integer Hall σ_xy=e²/2h via TR or inversion breaking.
**Atom or composite:** Composite (3D TI + symmetry-breaking surface mass).
**Cost model:** Chern-Simons effective action; ab initio θ via Wannier.
**Real wall?** Surface Hall + magnetoelectric coupling α=θ e²/2πh.
**Cross-domain wiring:** Direct CM analog of axion electrodynamics from particle physics; proposed in MnBi₂Te₄.
**Notes:** Qi-Hughes-Zhang (2008); Vanderbilt review.

### higher-order-ti (cross-domain alias: `hoti`, `corner-hinge-states`)
**Domain:** Condensed Matter
**Definition:** d-dimensional insulator with (d−2) or (d−3) dimensional boundary modes (e.g., corners in 2D, hinges in 3D).
**Atom or composite:** Composite (nested Wilson loops; quadrupole topology).
**Cost model:** Multipole indices; symmetry indicators.
**Real wall?** Protected by crystalline symmetries (C_n, mirror).
**Cross-domain wiring:** Realized in bismuth crystal hinges; photonic and acoustic HOTI analogs across photonics-optics.
**Notes:** Benalcazar-Bernevig-Hughes (2017).

### topological-crystalline-insulator (cross-domain alias: `tci`, `mirror-chern`)
**Domain:** Condensed Matter
**Definition:** Insulator with topology protected by point-group (not just T) symmetry; mirror Chern number a typical invariant.
**Atom or composite:** Composite (band topology + crystal symmetry).
**Cost model:** Symmetry indicator analysis (Po-Vishwanath, Watanabe).
**Real wall?** Robust as long as protecting crystalline symmetry is intact.
**Cross-domain wiring:** SnTe family; expands topological materials database; backbone of symmetry-indicator high-throughput screens (Topological Materials Database).
**Notes:** Fu (2011); Hsieh Nature (2012).

### hofstadter-butterfly (cross-domain alias: `magnetic-bz-fractal`, `harper-spectrum`)
**Domain:** Condensed Matter
**Definition:** Self-similar energy spectrum of 2D tight-binding electrons in a perpendicular B as a function of flux Φ/Φ₀.
**Atom or composite:** Composite (lattice + commensurate B).
**Cost model:** Diagonalize Harper Hamiltonian per rational flux.
**Real wall?** Quasi-periodic Cantor set spectrum at irrational fluxes.
**Cross-domain wiring:** Observed in moiré superlattices (graphene/h-BN); ties to Aubry-André quasiperiodic models.
**Notes:** Hofstadter (1976); Dean et al. Nature (2013).

### haldane-model (cross-domain alias: `honeycomb-flux-model`, `c-1-without-b`)
**Domain:** Condensed Matter
**Definition:** Honeycomb tight-binding with NN hopping + complex NNN hoppings creating staggered flux; first Chern insulator without net B.
**Atom or composite:** Composite (graphene + Peierls-like NNN flux).
**Cost model:** 2-band analytic; Chern number ±1 in topological phase.
**Real wall?** Realized in cold atoms (Jotzu et al. 2014) and magnetic moiré systems.
**Cross-domain wiring:** Birthplace of Chern-insulator concept; toy template for ab initio and ML topology studies.
**Notes:** Haldane (1988, 2016 Nobel).

---

## Quantum Hall Family

### laughlin-wavefunction (cross-domain alias: `nu-1-over-m-state`, `lwf`)
**Domain:** Condensed Matter
**Definition:** Ψ_L = Π_{i<j}(z_i−z_j)^m exp(−Σ|z_i|²/4ℓ_B²); ground state of FQH at ν=1/m for odd m.
**Atom or composite:** Atom (variational FQH wave function).
**Cost model:** O(N²) for matrix elements; QMC for energies.
**Real wall?** Yes — incompressible state with fractional quasiparticle charge e/m.
**Cross-domain wiring:** First quantitative theory of FQH; quasi-hole braiding gives anyonic phases.
**Notes:** Laughlin (1983, 1998 Nobel).

### composite-fermion (cross-domain alias: `cf-theory`, `jain-sequence`)
**Domain:** Condensed Matter
**Definition:** Electron + even number of attached flux quanta; CFs see reduced effective B and fill integer "Λ-levels", giving Jain sequence ν=p/(2pn±1).
**Atom or composite:** Composite (flux attachment Chern-Simons transformation).
**Cost model:** Numerical via projected wave functions.
**Real wall?** Excellent agreement with FQH plateaus up to ν~6/13.
**Cross-domain wiring:** Bridges IQH and FQH; Chern-Simons flux attachment shared with anyon physics.
**Notes:** Jain (1989).

### pfaffian-state (cross-domain alias: `moore-read`, `nu-5-over-2`)
**Domain:** Condensed Matter
**Definition:** Ψ_MR = Pf(1/(z_i−z_j)) Π(z_i−z_j)² exp(−Σ|z|²/4); proposed for ν=5/2 FQH plateau; non-abelian anyons.
**Atom or composite:** Composite (p-wave pair + Laughlin Jastrow).
**Cost model:** Numerical: ED on torus, DMRG.
**Real wall?** Hosts Ising anyons (Majorana zero modes in vortices).
**Cross-domain wiring:** Non-abelian statistics enable fault-tolerant topological quantum-computing (Nayak RMP).
**Notes:** Moore-Read (1991); Greiter-Wen-Wilczek.

### read-rezayi-state (cross-domain alias: `parafermion-state`, `nu-12-over-5`)
**Domain:** Condensed Matter
**Definition:** Z_k clustered FQH states generalizing Pfaffian; ν=12/5 proposed RR state with Fibonacci anyons.
**Atom or composite:** Composite (k-cluster correlations).
**Cost model:** ED on sphere/torus; DMRG.
**Real wall?** Fibonacci anyons universal for quantum-computing.
**Cross-domain wiring:** Holy grail for topological quantum-computing because Fibonacci anyons enable universal braid-based computation.
**Notes:** Read-Rezayi (1999).

### non-abelian-anyon (cross-domain alias: `na-anyon`, `braid-matrix-noncommuting`)
**Domain:** Condensed Matter
**Definition:** Quasiparticle whose exchange yields non-commuting matrix on degenerate Hilbert space; Ising and Fibonacci are canonical.
**Atom or composite:** Atom (representation of braid group).
**Cost model:** F- and R-symbols for fusion category.
**Real wall?** Robust against local perturbations within topological gap.
**Cross-domain wiring:** Resource for topological quantum-computing; explicit braiding underlies Kitaev's toric/honeycomb codes.
**Notes:** Nayak-Simon-Stern-Freedman-Das Sarma RMP (2008).

### charge-fractionalization (cross-domain alias: `e-over-q`, `quasiparticle-charge`)
**Domain:** Condensed Matter
**Definition:** Emergent excitations carrying fractional electron charge (e/3, e/5…) in FQH and related systems.
**Atom or composite:** Atom (emergent from collective state).
**Cost model:** Shot-noise Fano factor measurements.
**Real wall?** Yes — confirmed by quantum-shot-noise (Saminadayar 1997).
**Cross-domain wiring:** Conceptually adjacent to fractional spin, magnetic monopoles in spin-ice, and `polyacetylene-soliton`.
**Notes:** Laughlin (1983); de-Picciotto Nature (1997).

---

## Many-Body Methods

### second-quantization (cross-domain alias: `creation-annihilation-ops`, `fock-space`)
**Domain:** Condensed Matter
**Definition:** Formulation in terms of c†, c (b†, b) acting on Fock space; particle exchange built into (anti)commutation.
**Atom or composite:** Atom (operator algebra).
**Cost model:** Trade wave functions for sparse operators.
**Real wall?** Yes — canonical (anti)commutation enforce statistics.
**Cross-domain wiring:** Universal language across CM, QFT, quantum-computing fermionic encodings (Jordan-Wigner, Bravyi-Kitaev).
**Notes:** Dirac, Jordan-Wigner; Negele-Orland.

### hartree-fock-cm (cross-domain alias: `hf-mean-field`, `slater-determinant-state`)
**Domain:** Condensed Matter
**Definition:** Variational single Slater determinant minimizing ⟨H⟩; self-consistent solution of HF equations.
**Atom or composite:** Composite (one-body mean field + exchange).
**Cost model:** O(N⁴) (4-index integrals); often O(N³) with cutoff.
**Real wall?** Misses correlation energy; bandgap typically overestimated.
**Cross-domain wiring:** Baseline for post-HF, DFT comparison; entry point for HFB superconducting mean-field.
**Notes:** Hartree (1928), Fock (1930).

### hartree-fock-bogoliubov (cross-domain alias: `hfb`, `mean-field-superconductor`)
**Domain:** Condensed Matter
**Definition:** HF for pair amplitudes; self-consistent gap Δ and density n via Bogoliubov rotation.
**Atom or composite:** Composite (HF + BdG).
**Cost model:** O(N³) per iteration.
**Real wall?** Captures BCS pairing self-consistently in inhomogeneous systems.
**Cross-domain wiring:** Standard for nuclear-physics pairing; basis of inhomogeneous SC simulations (vortex cores, junctions).
**Notes:** Valatin, Bogoliubov (1958).

### mean-field-theory-cm (cross-domain alias: `mft-cm`, `weiss-or-bcs`)
**Domain:** Condensed Matter
**Definition:** Replace fluctuating operators by averages plus small fluctuations; closes equations at lowest order.
**Atom or composite:** Atom (factorization ansatz).
**Cost model:** O(N) per self-consistent iteration.
**Real wall?** Misses fluctuations critical near phase transitions; valid in high d / large coordination.
**Cross-domain wiring:** Same idea spans BCS, Stoner FM, Curie-Weiss; mean-field bound from variational principle.
**Notes:** Weiss; Bragg-Williams.

### dmrg (cross-domain alias: `density-matrix-rg`, `mps-variational`)
**Domain:** Condensed Matter
**Definition:** Variational algorithm over Matrix Product States; targets ground state of 1D-like systems with controlled bond dimension χ.
**Atom or composite:** Composite (truncated Schmidt + sweeps).
**Cost model:** O(d³χ³L) per sweep.
**Real wall?** Bond dimension required scales exponentially in 2D area (entanglement area law).
**Cross-domain wiring:** Underlies tensor networks; cousin of `mps`; bridges to ml-training tensor-network classifiers.
**Notes:** White (1992); Schollwöck RMP (2011).

### mps (cross-domain alias: `matrix-product-state`, `1d-tensor-network`)
**Domain:** Condensed Matter
**Definition:** Wave function |ψ⟩ = Σ Tr(A^{s_1}…A^{s_N})|s_1…s_N⟩; gauge-invariant tensor decomposition optimal for gapped 1D states.
**Atom or composite:** Atom (1D tensor-network ansatz).
**Cost model:** Storage O(Lχ²); expectation O(Lχ³).
**Real wall?** Area law in 1D bounds required χ for gapped phases.
**Cross-domain wiring:** Same algebraic object as HMMs in ml-training; underlies DMRG, TEBD.
**Notes:** Östlund-Rommer (1995); Vidal.

### peps (cross-domain alias: `2d-tensor-network`, `projected-entangled-pair-states`)
**Domain:** Condensed Matter
**Definition:** 2D generalization of MPS with tensors on each lattice site; bond indices to neighbors.
**Atom or composite:** Composite (planar tensor network).
**Cost model:** Contraction NP-hard exact; approximate boundary-MPS O(D^10).
**Real wall?** 2D area-law representable but expensive.
**Cross-domain wiring:** Foundational for 2D frustrated magnets, fractional quantum Hall variational states; recently used in ml-training quantum machine learning.
**Notes:** Verstraete-Cirac (2004).

### mera (cross-domain alias: `multi-scale-entanglement-renorm`, `causal-tensor-network`)
**Domain:** Condensed Matter
**Definition:** Hierarchical tensor network with isometries+disentanglers across scales; efficient for critical systems with logarithmic entanglement.
**Atom or composite:** Composite (renormalization built into network).
**Cost model:** O(χ^9) per scale.
**Real wall?** Captures critical scaling missed by MPS.
**Cross-domain wiring:** Connections to AdS/CFT holography (Swingle 2012); inspires ml-training hierarchical models.
**Notes:** Vidal (2007).

### dmft (cross-domain alias: `dynamical-mean-field-theory`, `single-impurity-map`)
**Domain:** Condensed Matter
**Definition:** Maps lattice many-body problem to self-consistent quantum impurity in effective bath; exact in d=∞.
**Atom or composite:** Composite (impurity solver + self-consistency).
**Cost model:** Per-iteration impurity solve dominates; CT-QMC O(N_τ²−N_τ³).
**Real wall?** Single-site DMFT misses nonlocal correlations; cluster DMFT recovers some.
**Cross-domain wiring:** Backbone for Mott physics in transition-metal oxides; combined with DFT in DFT+DMFT.
**Notes:** Metzner-Vollhardt (1989); Georges RMP (1996).

### ct-qmc (cross-domain alias: `continuous-time-qmc`, `expansion-monte-carlo`)
**Domain:** Condensed Matter
**Definition:** Stochastic sampling of Dyson series in interaction (CT-INT) or hybridization (CT-HYB); standard impurity solver.
**Atom or composite:** Composite (Monte Carlo + diagrammatic expansion).
**Cost model:** O(β³) typically; sign problem for general interactions.
**Real wall?** Sign problem in frustrated/multiorbital regimes.
**Cross-domain wiring:** Workhorse of DMFT for cuprates, heavy fermions; cousins in lattice QCD.
**Notes:** Rubtsov (2005); Werner-Millis (2006).

### qmc-cm (cross-domain alias: `quantum-monte-carlo-cm`, `vqmc-dmc-pimc`)
**Domain:** Condensed Matter
**Definition:** Family of stochastic methods (variational, diffusion, path-integral) for ground-state and finite-T many-body problems.
**Atom or composite:** Composite (importance sampling + walker dynamics).
**Cost model:** O(N²−N³) per step; scales gracefully with N.
**Real wall?** Fermion sign problem for nonpositive weights.
**Cross-domain wiring:** Same Monte Carlo machinery as ml-training MCMC; PIMC for superfluid He from Ceperley.
**Notes:** Ceperley-Alder (1980); Foulkes RMP (2001).

### exact-diagonalization (cross-domain alias: `ed-cm`, `lanczos-sparse`)
**Domain:** Condensed Matter
**Definition:** Construct full Hamiltonian matrix in a finite-dimensional Hilbert space and diagonalize (full or Lanczos).
**Atom or composite:** Composite (Hilbert-space construction + linear algebra).
**Cost model:** Exponential in N; ~40-spin cap with symmetries.
**Real wall?** Hilbert-space curse of dimensionality.
**Cross-domain wiring:** Gold standard for small clusters; benchmark for DMRG, DMFT, ML; Lanczos used in eigensolvers across HPC.
**Notes:** Lanczos (1950); QuSpin, ALPS.

### lanczos-algorithm (cross-domain alias: `krylov-eigensolver`, `tridiag-iteration`)
**Domain:** Condensed Matter
**Definition:** Iterative construction of Krylov subspace giving tridiagonal projection; extracts extremal eigenvalues efficiently.
**Atom or composite:** Atom (sparse iterative eigensolver).
**Cost model:** O(N_iter × N_nonzeros).
**Real wall?** Loss of orthogonality from round-off; needs reorthogonalization.
**Cross-domain wiring:** Same algorithm used in PageRank, ml-training spectral methods; central to ED in CM.
**Notes:** Lanczos (1950); Saad textbook.

### davidson-algorithm (cross-domain alias: `davidson-eigensolver`, `preconditioned-eigen`)
**Domain:** Condensed Matter
**Definition:** Iterative eigensolver with preconditioner; faster than Lanczos for moderate clustering of eigenvalues.
**Atom or composite:** Atom (preconditioned Krylov).
**Cost model:** O(N_iter × N_nonzeros) with smaller N_iter than Lanczos.
**Real wall?** Quality of preconditioner is decisive.
**Cross-domain wiring:** Backbone of CI/QC software (MOLPRO, ORCA); same preconditioned iteration used in plane-wave DFT.
**Notes:** Davidson (1975).

### nrg (cross-domain alias: `numerical-rg`, `wilson-impurity-rg`)
**Domain:** Condensed Matter
**Definition:** Logarithmic discretization of continuous bath + iterative diagonalization; solves Kondo and Anderson impurity exactly to arbitrary precision.
**Atom or composite:** Composite (Wilsonian RG + ED).
**Cost model:** O(N_iter × χ³); excellent low-T accuracy.
**Real wall?** Difficult for multi-band / multi-channel problems.
**Cross-domain wiring:** Established Kondo screening crossover scale T_K; impurity solver for DMFT in some regimes.
**Notes:** Wilson (1975 Nobel); Bulla RMP (2008).

### ct-hyb (cross-domain alias: `hybridization-expansion-qmc`, `werner-millis`)
**Domain:** Condensed Matter
**Definition:** Expand around atomic limit in hybridization Δ(τ); each MC update inserts/removes pairs of operators.
**Atom or composite:** Composite (CT-QMC variant).
**Cost model:** Efficient for strong-coupling regime; O(N_τ²−N_τ³).
**Real wall?** Sign problem in off-diagonal hybridization (multiorbital with SOC).
**Cross-domain wiring:** Standard DMFT solver for multiorbital Hubbard; used in correlated DFT+DMFT for actinides, transition-metal oxides.
**Notes:** Werner-Millis (2006).

### ct-aux (cross-domain alias: `auxiliary-field-ctqmc`, `decoupled-interaction-qmc`)
**Domain:** Condensed Matter
**Definition:** CT-QMC with continuous Hubbard-Stratonovich auxiliary field; efficient at weak-to-intermediate U.
**Atom or composite:** Composite (HS field + CT-INT).
**Cost model:** Comparable to CT-INT; better sign in some regimes.
**Real wall?** Similar sign problem.
**Cross-domain wiring:** Workhorse alongside CT-HYB for DMFT studies of correlated electrons.
**Notes:** Gull-Werner-Millis-Troyer (2008).

---

## Density Functional Theory

### hohenberg-kohn (cross-domain alias: `hk-theorems`, `density-determines-state`)
**Domain:** Condensed Matter
**Definition:** Two theorems: (1) ground-state density uniquely determines V_ext up to constant; (2) E[ρ] is variational with minimum at ground-state ρ.
**Atom or composite:** Atom (formal foundation of DFT).
**Cost model:** Existence proof — does not give E[ρ] form.
**Real wall?** Yes — universal functional F[ρ] exists but unknown exactly.
**Cross-domain wiring:** Theoretical underpinning of all DFT; analogous to Levy constrained search formulation.
**Notes:** Hohenberg-Kohn (1964); Kohn Nobel (1998).

### kohn-sham (cross-domain alias: `ks-equations`, `noninteracting-reference`)
**Domain:** Condensed Matter
**Definition:** Replace interacting electron problem by noninteracting one with same density: [−∇²/2 + v_eff(r)]φ_i = ε_i φ_i.
**Atom or composite:** Composite (KS mapping + SCF).
**Cost model:** O(N³) per SCF iteration.
**Real wall?** Exact in principle if v_xc[ρ] were known.
**Cross-domain wiring:** Backbone of computational materials science; powering ml-training "DFT in the loop".
**Notes:** Kohn-Sham (1965).

### lda-dft (cross-domain alias: `local-density-approx`, `homogeneous-electron-gas-xc`)
**Domain:** Condensed Matter
**Definition:** Approximate ε_xc[ρ(r)] = ε_xc^HEG(ρ(r)); uses uniform-gas correlation (Ceperley-Alder QMC).
**Atom or composite:** Atom (rung 1 of Jacob's ladder).
**Cost model:** Negligible vs core SCF; pointwise.
**Real wall?** Underestimates band gaps; struggles with strong correlation.
**Cross-domain wiring:** First-rung benchmark; baseline against which GGA, hybrids are compared.
**Notes:** Kohn-Sham (1965); Perdew-Zunger (1981).

### gga-pbe (cross-domain alias: `pbe-functional`, `gga`)
**Domain:** Condensed Matter
**Definition:** Generalized-gradient approximation: ε_xc[ρ,∇ρ]; PBE form widely used in solids.
**Atom or composite:** Atom (rung 2 of Jacob's ladder).
**Cost model:** Same O(N³) SCF.
**Real wall?** Improves cohesive energies, geometries but still underestimates gaps.
**Cross-domain wiring:** Default in VASP/QE for most materials studies; baseline for ML potentials trained on DFT data.
**Notes:** Perdew-Burke-Ernzerhof (1996).

### meta-gga-scan (cross-domain alias: `scan-functional`, `tau-dependent-gga`)
**Domain:** Condensed Matter
**Definition:** Meta-GGA using kinetic-energy density τ; SCAN satisfies 17 known exact constraints.
**Atom or composite:** Composite (GGA + τ).
**Cost model:** Slightly more expensive than GGA.
**Real wall?** Improves layered, vdW-like systems modestly.
**Cross-domain wiring:** Rung 3 of Jacob's ladder; benchmarked on materials databases.
**Notes:** Sun-Ruzsinszky-Perdew (2015).

### hybrid-functional (cross-domain alias: `b3lyp-hse`, `exact-exchange-mix`)
**Domain:** Condensed Matter
**Definition:** Mix fraction of exact (HF) exchange with semilocal: e.g. HSE06 = 25% HF (screened) + 75% PBE.
**Atom or composite:** Composite (HF + DFT exchange).
**Cost model:** ~10× more expensive than GGA for plane-wave codes.
**Real wall?** Better gaps; range-separation needed for solids.
**Cross-domain wiring:** Rung 4 of Jacob's ladder; routine in chemistry; in solids via HSE, range-separated screened exchange.
**Notes:** Becke (1993); Heyd-Scuseria-Ernzerhof (2003).

### dft-plus-u (cross-domain alias: `lda-u`, `hubbard-correction`)
**Domain:** Condensed Matter
**Definition:** Add on-site Hubbard U penalty to localized orbitals: E_U = (U/2)Σ_σ Tr(n^σ−n^σ n^σ); cheap fix for self-interaction.
**Atom or composite:** Composite (DFT + Hubbard mean field).
**Cost model:** Same as DFT.
**Real wall?** U is empirical (or from linear-response cRPA); double-counting subtle.
**Cross-domain wiring:** Cheaper alternative to DMFT for transition-metal oxides; widely used in materials screening.
**Notes:** Anisimov-Zaanen-Andersen (1991); Cococcioni-de Gironcoli (2005).

### plane-wave-dft (cross-domain alias: `pwd`, `pseudopotential-dft`)
**Domain:** Condensed Matter
**Definition:** Expand KS orbitals in plane waves with energy cutoff E_cut; core electrons replaced by pseudopotentials (norm-conserving, ultrasoft) or PAW projectors.
**Atom or composite:** Composite (basis + pseudopotential + FFT).
**Cost model:** O(N N_PW log N_PW) per SCF iteration via FFT.
**Real wall?** Memory scales with N_PW³ for some operations.
**Cross-domain wiring:** VASP, QE, Abinit; underpins materials databases.
**Notes:** Kohn-Sham; Vanderbilt USPP (1990); Blöchl PAW (1994).

### paw-method (cross-domain alias: `projector-augmented-wave`, `blochl-paw`)
**Domain:** Condensed Matter
**Definition:** All-electron reconstruction inside augmentation spheres + smooth pseudo wave function outside; combines accuracy of all-electron with speed of pseudopotential.
**Atom or composite:** Composite (linear transform).
**Cost model:** Similar to USPP; better accuracy.
**Real wall?** Quality of PAW dataset critical.
**Cross-domain wiring:** Standard in VASP, GPAW; tuned PAW sets in pseudopotential libraries.
**Notes:** Blöchl (1994).

### flapw (cross-domain alias: `full-potential-lapw`, `all-electron-dft`)
**Domain:** Condensed Matter
**Definition:** All-electron, full-potential DFT using linearized augmented plane waves; high precision for tight binding around nuclei and bulk regions.
**Atom or composite:** Composite (basis partitioned spheres + interstitial).
**Cost model:** O(N³); slower than pseudopotential DFT but more accurate.
**Real wall?** No pseudo error.
**Cross-domain wiring:** Reference benchmarks (Wien2k, FLEUR); used to validate PAW datasets.
**Notes:** Andersen (1975); Wimmer-Krakauer-Weinert-Freeman (1981).

### tddft (cross-domain alias: `time-dependent-dft`, `runge-gross`)
**Domain:** Condensed Matter
**Definition:** Time-dependent extension of DFT: time-dependent KS equations; linear-response variant gives excitation spectra.
**Atom or composite:** Composite (TDKS + xc kernel).
**Cost model:** Linear-response O(N⁴); real-time O(N³ N_t).
**Real wall?** Adiabatic approximation misses double excitations; charge-transfer errors.
**Cross-domain wiring:** Optical spectra of molecules, solids; ML for excited states emerging.
**Notes:** Runge-Gross (1984); Casida (1995).

### gw-approximation (cross-domain alias: `gw-self-energy`, `quasiparticle-correction`)
**Domain:** Condensed Matter
**Definition:** Self-energy Σ = iGW; G = Green's function, W = screened Coulomb; first-principles quasiparticle band structure.
**Atom or composite:** Composite (Hedin's equations to first order in W).
**Cost model:** O(N⁴−N⁶); G₀W₀ commonly used.
**Real wall?** Corrects DFT gaps to within ~0.1 eV for sp semiconductors.
**Cross-domain wiring:** Standard for ab initio gaps; benchmark for BSE, ML-band-gap models.
**Notes:** Hedin (1965); Hybertsen-Louie (1986).

### bse-cm (cross-domain alias: `bethe-salpeter`, `electron-hole-exciton-eq`)
**Domain:** Condensed Matter
**Definition:** Two-particle equation for electron-hole pair; produces optical absorption with excitonic effects on top of GW.
**Atom or composite:** Composite (GW + e-h kernel).
**Cost model:** O(N⁶) effective Hamiltonian diagonalization.
**Real wall?** Standard for excitons in semiconductors and insulators.
**Cross-domain wiring:** Same Bethe-Salpeter framework used in particle physics; ML surrogates emerging.
**Notes:** Salpeter-Bethe (1951); Rohlfing-Louie (2000).

### dft-dmft (cross-domain alias: `combined-dft-dmft`, `ldap-dmft`)
**Domain:** Condensed Matter
**Definition:** Use DFT to set up multiorbital tight-binding; treat correlated subspace with DMFT impurity solver; self-consistent over both.
**Atom or composite:** Composite (multiscale DFT + DMFT).
**Cost model:** Cluster impurity solves dominate; weeks of HPC.
**Real wall?** Double counting; choice of correlated subspace.
**Cross-domain wiring:** State-of-the-art for actinides, heavy fermions, transition-metal oxides; ML for impurity-solver speedup.
**Notes:** Kotliar RMP (2006); TRIQS, EDMFTF.

---

## Strongly Correlated Models

### hubbard-model (cross-domain alias: `single-band-hubbard`, `t-U-model`)
**Domain:** Condensed Matter
**Definition:** H = −t Σ c†_iσ c_jσ + U Σ n_i↑ n_i↓; minimal lattice model for correlations.
**Atom or composite:** Atom (canonical strongly-correlated model).
**Cost model:** ED ≤16 sites; DMRG (1D); DMFT (∞D); DCA, DQMC.
**Real wall?** No exact solution in 2D; sign problem in QMC away from half-filling.
**Cross-domain wiring:** Workhorse for cuprate phenomenology, cold-atom emulators, ML phase classification.
**Notes:** Hubbard (1963); Anderson (1987).

### t-j-model (cross-domain alias: `effective-low-U-limit`, `large-U-projection`)
**Domain:** Condensed Matter
**Definition:** H = −t Σ P c†_iσ c_jσ P + J Σ(Sᵢ·Sⱼ − n_in_j/4); large-U projection of Hubbard onto no-double-occupancy subspace.
**Atom or composite:** Composite (Schrieffer-Wolff transform of Hubbard).
**Cost model:** Constrained Hilbert space; DMRG, Gutzwiller VMC.
**Real wall?** J = 4t²/U; valid for U>>t.
**Cross-domain wiring:** Used in cuprate theories; underlies RVB superconductivity proposal.
**Notes:** Anderson (1987); Zhang-Rice singlet (1988).

### anderson-impurity-model (cross-domain alias: `aim`, `localized-level-plus-bath`)
**Domain:** Condensed Matter
**Definition:** H = ε_d n_d + Un_d↑n_d↓ + Σ ε_k c†_k c_k + Σ V_k(c†_k d + h.c.); single correlated level coupled to bath.
**Atom or composite:** Composite (impurity + free bath + hybridization).
**Cost model:** NRG exact; CT-QMC; ED of star geometry.
**Real wall?** Captures Kondo physics, mixed valence, heavy-fermion local part.
**Cross-domain wiring:** Building block of DMFT; relevant to quantum dots and SET physics.
**Notes:** Anderson (1961, Nobel 1977).

### kondo-effect (cross-domain alias: `magnetic-impurity-screening`, `tk-scale`)
**Domain:** Condensed Matter
**Definition:** Resistivity minimum + log(T) divergence from antiferromagnetic exchange between local moment and conduction electrons; below T_K, moment screened into singlet.
**Atom or composite:** Composite (s-d exchange + many-body screening).
**Cost model:** NRG, Bethe ansatz exact; perturbation theory diverges.
**Real wall?** Yes — T_K is dynamically generated low-energy scale.
**Cross-domain wiring:** Direct analog of asymptotic freedom in QCD; central to heavy-fermion physics.
**Notes:** Kondo (1964); Wilson (1975 Nobel).

### kondo-lattice (cross-domain alias: `dense-kondo`, `heavy-fermion-model`)
**Domain:** Condensed Matter
**Definition:** Lattice of magnetic moments coupled to conduction sea; coherent state below T* with heavy-fermion bands.
**Atom or composite:** Composite (Kondo at every site + RKKY).
**Cost model:** DMFT for periodic Anderson; cluster methods for nonlocal effects.
**Real wall?** Doniach: competition between RKKY (∝J²) and Kondo (∝exp(−1/Jρ)) determines magnetic vs heavy-FL.
**Cross-domain wiring:** Underlies CeCu₂Si₂, YbRh₂Si₂; quantum critical point physics.
**Notes:** Doniach (1977); Coleman review (2007).

### mott-insulator (cross-domain alias: `correlation-insulator`, `u-gt-w`)
**Domain:** Condensed Matter
**Definition:** Insulator from strong Coulomb U opening a gap in a partially-filled band; not a band insulator.
**Atom or composite:** Composite (Coulomb interaction-driven).
**Cost model:** DMFT for single-site Mott transition; QMC for unfrustrated bipartite half-filling.
**Real wall?** U_c ~ bandwidth; bandwidth-driven vs filling-driven transitions.
**Cross-domain wiring:** Parent state of cuprates; cold-atom realization in optical lattices; bridges to charge-transfer insulators.
**Notes:** Mott (1949); Imada-Fujimori-Tokura RMP (1998).

### charge-transfer-insulator (cross-domain alias: `cti`, `zaanen-sawatzky-allen`)
**Domain:** Condensed Matter
**Definition:** Insulator with gap set by charge-transfer energy Δ < U; lowest excitation is ligand → metal rather than Mott U.
**Atom or composite:** Composite (CT energy + Hubbard U).
**Cost model:** Cluster ED with explicit ligands.
**Real wall?** Distinct from Mott; common in late transition-metal oxides.
**Cross-domain wiring:** ZSA classification underlies cuprate phenomenology (O 2p ↔ Cu 3d).
**Notes:** Zaanen-Sawatzky-Allen (1985).

### doped-mott-insulator (cross-domain alias: `hole-doped-mott`, `cuprate-parent`)
**Domain:** Condensed Matter
**Definition:** Mott insulator with electrons or holes added by chemical doping; complex phase diagram (AFM → pseudogap → SC → strange metal → FL).
**Atom or composite:** Composite (Hubbard/t-J + dopants).
**Cost model:** Cluster DMFT (DCA, CDMFT); functional RG; ML phases.
**Real wall?** Sign problem in QMC; mechanism of SC still open.
**Cross-domain wiring:** Hosts unconventional SC, pseudogap, stripe order; cold-atom Fermi-Hubbard emulators.
**Notes:** Anderson Science (1987); Lee-Nagaosa-Wen RMP (2006).

### rvb-state (cross-domain alias: `resonating-valence-bond`, `anderson-rvb`)
**Domain:** Condensed Matter
**Definition:** Superposition of singlet-pair coverings of lattice; proposed parent state for high-T_c SC by Anderson.
**Atom or composite:** Composite (singlet basis + projection).
**Cost model:** Gutzwiller projected wave function VMC.
**Real wall?** Conceptual framework — quantitative numerical status mixed.
**Cross-domain wiring:** Connects spin liquid, doped Mott, and SC; predicted Z_2 topological order before that concept matured.
**Notes:** Anderson (1973, 1987).

### luttinger-liquid (cross-domain alias: `tl-liquid`, `1d-non-fermi-liquid`)
**Domain:** Condensed Matter
**Definition:** Universal low-energy fixed point of 1D interacting fermions; spin-charge separation, power-law correlations, no quasiparticles.
**Atom or composite:** Composite (bosonization + RG flow).
**Cost model:** Analytic via bosonization; DMRG for microscopics.
**Real wall?** Yes — Fermi liquid theory fails in 1D.
**Cross-domain wiring:** Realized in carbon nanotubes, quantum wires, edge states of FQH; same CFT structure as 1D CFTs.
**Notes:** Haldane (1981); Voit review (1995).

### bosonization (cross-domain alias: `boson-rep-of-fermions`, `mattis-luther`)
**Domain:** Condensed Matter
**Definition:** Express 1D fermion operators as exponentials of boson fields; ψ_R(x) ~ exp(iφ(x)).
**Atom or composite:** Composite (functional mapping).
**Cost model:** Analytic for low-energy properties.
**Real wall?** Specific to 1D (or quasi-1D).
**Cross-domain wiring:** Same tool used in string theory worldsheets and 2D CFTs; underlies analytic treatment of Luttinger liquid.
**Notes:** Mattis-Lieb (1965); Haldane (1981); Giamarchi book.

---

## Disorder & Localization

### anderson-localization (cross-domain alias: `disorder-localization`, `exponential-confinement`)
**Domain:** Condensed Matter
**Definition:** Absence of diffusion in random potential beyond critical disorder; eigenstates exponentially localized ψ ∝ e^(−|r−r_0|/ξ).
**Atom or composite:** Atom (universal quantum-interference phenomenon).
**Cost model:** Transfer matrix; kernel polynomial method O(N²).
**Real wall?** Yes — in 1D/2D, all states localized for any disorder (scaling theory).
**Cross-domain wiring:** Same mechanism for photonic localization, ultrasound localization; bridges to ml-training as random-feature noise.
**Notes:** Anderson (1958, Nobel 1977); Abrahams et al. (1979).

### weak-localization (cross-domain alias: `wl`, `coherent-backscattering`)
**Domain:** Condensed Matter
**Definition:** Logarithmic increase of resistivity at low T from constructive interference of time-reversed paths.
**Atom or composite:** Composite (Cooperon diffuson contribution).
**Cost model:** Diagrammatic perturbation; B-field dependence of magnetoresistance.
**Real wall?** Diverges in 2D — precursor of Anderson localization.
**Cross-domain wiring:** Magnetoresistance fingerprint; analog phenomena in light scattering (coherent backscattering peak).
**Notes:** Gor'kov-Larkin-Khmel'nitskii (1979); Bergmann review.

### scaling-theory-localization (cross-domain alias: `gang-of-four`, `beta-function-disorder`)
**Domain:** Condensed Matter
**Definition:** β(g) = dlng/dlnL describes RG flow of dimensionless conductance; β<0 in 2D for all g.
**Atom or composite:** Composite (RG framework).
**Cost model:** Conceptual; verified numerically with transfer matrices.
**Real wall?** Yes — symmetry classification (Wigner-Dyson) determines β(g) form.
**Cross-domain wiring:** Inspired by Wilson RG; foundational for `random-matrix-theory` classification.
**Notes:** Abrahams-Anderson-Licciardello-Ramakrishnan (1979).

### mobility-edge (cross-domain alias: `ec`, `localized-extended-boundary`)
**Domain:** Condensed Matter
**Definition:** Energy separating extended from localized states in 3D disordered system; Anderson transition.
**Atom or composite:** Atom (critical energy).
**Cost model:** Multifractal scaling at E_c.
**Real wall?** Yes — sharp transition in 3D (universality class 3D Anderson).
**Cross-domain wiring:** Anderson transition is a quantum phase transition; related to many-body localization (MBL) in interacting systems.
**Notes:** Mott (1968); Slevin-Ohtsuki (1999) ν≈1.57.

### multifractality-cm (cross-domain alias: `mf-spectrum`, `f-of-alpha`)
**Domain:** Condensed Matter
**Definition:** Wave function at Anderson criticality scales as |ψ|² ~ L^(-α(x)) with continuous spectrum f(α).
**Atom or composite:** Composite (statistical scaling of wave function).
**Cost model:** Box-counting analysis at criticality.
**Real wall?** Universal exponents in each symmetry class.
**Cross-domain wiring:** Same multifractal apparatus as in turbulence (Kolmogorov), DLA, and ml-training neural-tangent-kernel critical points.
**Notes:** Castellani-Peliti (1986); Evers-Mirlin RMP (2008).

### dirty-superconductor (cross-domain alias: `disordered-bcs`, `anderson-theorem`)
**Domain:** Condensed Matter
**Definition:** SC with mean free path ℓ < ξ_0; gap and T_c essentially unchanged by nonmagnetic scattering (Anderson theorem).
**Atom or composite:** Composite (BCS + disorder average).
**Cost model:** Usadel equations for diffusive limit.
**Real wall?** Magnetic impurities suppress T_c (Abrikosov-Gor'kov).
**Cross-domain wiring:** Robustness underlies practical thin-film SC devices; failure of Anderson theorem signals unconventional pairing.
**Notes:** Anderson (1959); Abrikosov-Gor'kov (1961).

### random-matrix-theory-cm (cross-domain alias: `rmt`, `gaussian-ensembles`)
**Domain:** Condensed Matter
**Definition:** Statistical theory of eigenvalues of random Hamiltonians; level statistics universal within symmetry class.
**Atom or composite:** Composite (ensemble + invariance).
**Cost model:** Analytic for Gaussian ensembles; numerics for arbitrary.
**Real wall?** Universality applies in quantum chaotic regimes.
**Cross-domain wiring:** Same statistics in nuclear levels, riemann zeros, ml-training Hessian spectra.
**Notes:** Wigner-Dyson; Mehta book.

### wigner-dyson-statistics (cross-domain alias: `goe-gue-gse`, `level-repulsion`)
**Domain:** Condensed Matter
**Definition:** Eigenvalue spacing distribution p(s) ~ s^β exp(−As²); β=1 GOE (T), β=2 GUE (broken T), β=4 GSE (T+SU(2) breaking).
**Atom or composite:** Atom (universal spacing law).
**Cost model:** O(1) after diagonalization.
**Real wall?** Universal — diagnostic of chaotic vs integrable spectra.
**Cross-domain wiring:** Used as diagnostic in many-body localization, quantum chaos, MBL transitions.
**Notes:** Dyson (1962); Mehta.

### gue (cross-domain alias: `gaussian-unitary-ensemble`, `t-broken-rmt`)
**Domain:** Condensed Matter
**Definition:** Ensemble of Hermitian random matrices invariant under unitary transformations; β=2.
**Atom or composite:** Atom (basic RMT class).
**Cost model:** Analytic via orthogonal polynomial method.
**Real wall?** Realized when T-symmetry broken (e.g., magnetic field).
**Cross-domain wiring:** Models magneto-conductance fluctuations in mesoscopic samples.
**Notes:** Dyson (1962).

### goe (cross-domain alias: `gaussian-orthogonal-ensemble`, `t-symmetric-rmt`)
**Domain:** Condensed Matter
**Definition:** Real symmetric random matrices invariant under orthogonal transformations; β=1.
**Atom or composite:** Atom (T-symmetric RMT class).
**Cost model:** Analytic.
**Real wall?** Models systems with T-symmetry and integer spin.
**Cross-domain wiring:** Heavy-nucleus level statistics; weak-localization conductance distributions.
**Notes:** Wigner (1955); Dyson (1962).

### gse (cross-domain alias: `gaussian-symplectic-ensemble`, `soc-rmt`)
**Domain:** Condensed Matter
**Definition:** Quaternionic self-dual random matrices; β=4; T-invariant with strong SOC.
**Atom or composite:** Atom (third Wigner-Dyson class).
**Cost model:** Analytic.
**Real wall?** Realized in T-invariant systems with half-integer total spin.
**Cross-domain wiring:** Statistical model for spin-orbit-coupled disordered samples.
**Notes:** Dyson (1962).

### dyson-brownian-motion (cross-domain alias: `level-dynamics`, `rmt-evolution`)
**Domain:** Condensed Matter
**Definition:** Stochastic evolution of random-matrix eigenvalues; eigenvalue repulsion enters as logarithmic interaction.
**Atom or composite:** Composite (Brownian motion + Vandermonde repulsion).
**Cost model:** Fokker-Planck equation analytic.
**Real wall?** Universal at equilibrium.
**Cross-domain wiring:** Models parametric level dynamics in driven mesoscopic systems; same math as Coulomb gas in 1D.
**Notes:** Dyson (1962).

---

## Mesoscopic & Nanoscale

### quantum-dot (cross-domain alias: `0d-system`, `artificial-atom`)
**Domain:** Condensed Matter
**Definition:** Nanoscale electron confinement in 3D giving discrete-level spectrum; ~10–100 nm.
**Atom or composite:** Composite (confinement + Coulomb).
**Cost model:** Constant-interaction model O(1) per addition energy.
**Real wall?** Charging energy E_C >> k_BT for Coulomb blockade.
**Cross-domain wiring:** Spin-qubit hosts in semiconductor quantum-computing; single-photon sources in photonics-optics.
**Notes:** Kastner Physics Today (1993).

### quantum-wire (cross-domain alias: `1d-channel`, `qwire`)
**Domain:** Condensed Matter
**Definition:** Quasi-1D conductor with quantized transverse modes; conductance steps 2e²/h per mode.
**Atom or composite:** Composite (1D Schrödinger eq.).
**Cost model:** Landauer formula O(N_modes).
**Real wall?** Mode opening as gate voltage tunes Fermi level past subband edges.
**Cross-domain wiring:** Underlies QPC physics; Luttinger-liquid host; Majorana wires (Lutchyn-Sau-Das Sarma).
**Notes:** van Wees et al. (1988); Wharam et al. (1988).

### quantum-well-cm (cross-domain alias: `qw`, `2d-electron-gas-confinement`)
**Domain:** Condensed Matter
**Definition:** Sandwich of narrower-gap material between barriers; confined motion in z, free in (x,y); subbands E_n + ℏ²k_⊥²/2m*.
**Atom or composite:** Composite (heterojunction + envelope-function).
**Cost model:** Solve 1D Schrödinger for subbands; O(N_z).
**Real wall?** Strain, quantum confinement set photon energies in lasers.
**Cross-domain wiring:** Engine of QW lasers, HEMTs; underlies 2DEG QH experiments.
**Notes:** Esaki-Tsu (1970); Bastard book.

### coulomb-blockade (cross-domain alias: `cb`, `single-electron-charging`)
**Domain:** Condensed Matter
**Definition:** Suppression of current through a small island when E_C = e²/2C >> k_BT, eV; addition of one electron costs E_C.
**Atom or composite:** Composite (capacitance + Pauli + transport).
**Cost model:** Master equation O(N_states).
**Real wall?** Yes — discreteness of electron charge.
**Cross-domain wiring:** Basis of SET; analogous to artificial-atom level spectroscopy; relevant for thermometry standards.
**Notes:** Averin-Likharev (1986); Grabert-Devoret SET book.

### single-electron-transistor (cross-domain alias: `set`, `cb-transistor`)
**Domain:** Condensed Matter
**Definition:** Three-terminal device with central island where current modulated by gate-controlled single-electron charging.
**Atom or composite:** Composite (Coulomb blockade + tunneling).
**Cost model:** Orthodox theory O(N_states).
**Real wall?** Operates only when E_C >> k_BT.
**Cross-domain wiring:** Electrometer with charge sensitivity ~10⁻⁶ e/√Hz; readout for spin qubits.
**Notes:** Fulton-Dolan (1987); Kastner (1992).

### landauer-buttiker (cross-domain alias: `multi-terminal-conductance`, `scattering-formula`)
**Domain:** Condensed Matter
**Definition:** G_αβ = (e²/h) Tr[t†_αβ t_αβ]; conductance from many-terminal scattering matrix.
**Atom or composite:** Composite (S-matrix + occupation).
**Cost model:** Recursive Green's function O(L W³).
**Real wall?** Yes — quantum-coherent transport at zero T.
**Cross-domain wiring:** Workhorse of mesoscopic transport; identical formalism to ML-based quantum-transport surrogates.
**Notes:** Landauer (1957); Büttiker (1986).

### quantum-point-contact (cross-domain alias: `qpc`, `2e2-over-h-steps`)
**Domain:** Condensed Matter
**Definition:** Narrow constriction in 2DEG; conductance quantized in steps of 2e²/h as electrostatic gates change width.
**Atom or composite:** Composite (1D adiabatic channel).
**Cost model:** Saddle-point model analytic.
**Real wall?** Yes — clear quantization at low T, T < 1 K.
**Cross-domain wiring:** Building block for mesoscopic devices, interferometers, and edge-mode partitioning in QH.
**Notes:** van Wees, Wharam (1988).

### shot-noise (cross-domain alias: `current-fluctuations`, `poisson-noise`)
**Domain:** Condensed Matter
**Definition:** Current noise S_I = 2eI(1−T) at zero T for full Poisson is S = 2eI; Fano factor F = S/2eI.
**Atom or composite:** Atom (granularity of current).
**Cost model:** Lesovik-Levitov formula for non-interacting.
**Real wall?** Yes — direct probe of charge of transferred quasiparticles.
**Cross-domain wiring:** Revealed e/3 in FQH (Saminadayar 1997); used for charge spectroscopy across mesoscopic devices.
**Notes:** Schottky (1918); Blanter-Büttiker review (2000).

### conductance-quantization (cross-domain alias: `2e2-over-h`, `landauer-step`)
**Domain:** Condensed Matter
**Definition:** Conductance of clean ballistic channel = Ne²/h × spin degeneracy; integer plateaus.
**Atom or composite:** Atom (universal conductance step).
**Cost model:** Sum over open modes.
**Real wall?** Yes — quantization is universal; values like 12.9 kΩ tied to fundamental constants.
**Cross-domain wiring:** Same e²/h motif in IQHE, QPC, helical edges; metrological role.
**Notes:** Landauer (1957); van Wees-Wharam (1988).

### fano-factor (cross-domain alias: `noise-to-poisson-ratio`, `f`)
**Domain:** Condensed Matter
**Definition:** F = S/(2eI); F=1 Poisson, F<1 sub-Poissonian (Pauli suppression), F>1 super-Poissonian.
**Atom or composite:** Atom (dimensionless ratio).
**Cost model:** Direct from shot-noise measurement.
**Real wall?** Bounded by 0 (noiseless) to 1 (full Poisson) for fermionic channels.
**Cross-domain wiring:** F=1/3 for diffusive metals; F=1/4 for chaotic dot; F=1/3 also in FQH at ν=1/3 reflects e/3 charge.
**Notes:** Beenakker-Schönenberger Physics Today (2003).

### magic-angle (cross-domain alias: `1.1-deg-tbg`, `flat-band-twist`)
**Domain:** Condensed Matter
**Definition:** Twist angle ~1.1° between graphene layers where flat band emerges in continuum BM model; Coulomb dominates.
**Atom or composite:** Composite (Bistritzer-MacDonald continuum + interactions).
**Cost model:** Continuum + HF + RPA.
**Real wall?** Yes — discrete angle window; flat-band condition.
**Cross-domain wiring:** Drives correlated insulating and SC phases; tunable strongly-correlated platform.
**Notes:** Bistritzer-MacDonald (2011); Cao et al. Nature (2018).

### moire-superlattice (cross-domain alias: `moire-cm`, `long-period-superlattice`)
**Domain:** Condensed Matter
**Definition:** Long-wavelength pattern from small lattice or rotation mismatch of stacked 2D materials; period ~ a/sin θ.
**Atom or composite:** Composite (superposition of lattices).
**Cost model:** Continuum theory; brute-force tight-binding O(N_moire³).
**Real wall?** Periodic only at commensurate ratios.
**Cross-domain wiring:** Hosts flat bands, correlated and topological states across TBG, t-WSe₂, t-MoTe₂; same beat structure as `moire-pattern` in signal-processing-rf.
**Notes:** Bistritzer-MacDonald (2011); Wu et al. (2018).

### flat-band-cm (cross-domain alias: `dispersionless-band`, `kinetic-quench`)
**Domain:** Condensed Matter
**Definition:** Bloch band with vanishing dispersion ε_n(k) ≈ const; bandwidth W → 0 makes interactions dominate.
**Atom or composite:** Atom (zero kinetic energy scale).
**Cost model:** Strong-coupling expansion since W << U.
**Real wall?** Yes — geometric / topological obstructions for projecting interactions.
**Cross-domain wiring:** Realized in kagome (Lieb), moiré, twisted bilayer; cousin of `landau-level-cm` flat dispersion.
**Notes:** Mielke; Aoki review (2017).

---

## Phase Transitions & Critical Phenomena (CM-specific)

### landau-theory-cm (cross-domain alias: `lt-cm`, `psi-power-expansion`)
**Domain:** Condensed Matter
**Definition:** Free energy expansion F = αψ² + βψ⁴+… in order parameter ψ; α changes sign at T_c.
**Atom or composite:** Composite (symmetry-based phenomenology).
**Cost model:** Analytic O(1).
**Real wall?** Mean-field critical exponents (α=0, β=1/2, γ=1, ν=1/2) systematically wrong below upper critical dimension.
**Cross-domain wiring:** Universal template across magnets, SC, BEC, liquid-gas; basis of GL theory.
**Notes:** Landau (1937).

### ginzburg-criterion (cross-domain alias: `mft-failure`, `gi-number`)
**Domain:** Condensed Matter
**Definition:** Estimate of width of critical region |T−T_c|/T_c where MFT fails; Gi = (k_BT_c)²/(αξ_0³ Δc).
**Atom or composite:** Atom (`compare` fluctuation to mean-field gap).
**Cost model:** O(1) estimate.
**Real wall?** Determines when fluctuations matter.
**Cross-domain wiring:** Sets criterion for when MFT breaks down; relevant for cuprates (small Gi) vs conventional SCs (tiny Gi).
**Notes:** Ginzburg (1960).

### critical-exponents-cm (cross-domain alias: `alpha-beta-gamma-delta`, `c-v-singularities`)
**Domain:** Condensed Matter
**Definition:** Power-law singularities ξ ~ |t|^{−ν}, M ~ |t|^β, χ ~ |t|^{−γ}, C ~ |t|^{−α} at T_c.
**Atom or composite:** Atom (universal scaling).
**Cost model:** MC + finite-size scaling; ε-expansion.
**Real wall?** Yes — universal within universality class.
**Cross-domain wiring:** Same exponents across magnets, fluids, polymer systems; foundation of universality classes.
**Notes:** Wilson (1971-72 Nobel 1982); Fisher review.

### universality-class-cm (cross-domain alias: `uc-class`, `d-n-rg-fixed-point`)
**Domain:** Condensed Matter
**Definition:** Set of models flowing under RG to same fixed point; determined by dimension d, order-parameter symmetry, and range of interactions.
**Atom or composite:** Atom (RG equivalence relation).
**Cost model:** Mapping to known UC.
**Real wall?** Yes — disparate systems share exponents (Ising d=3 fluids, magnets, etc.).
**Cross-domain wiring:** RG framework underlies ml-training learning theory of critical neural networks.
**Notes:** Kadanoff-Wilson; Fisher RMP (1998).

### scaling-relations (cross-domain alias: `widom-rushbrooke`, `2-minus-alpha-2-beta`)
**Domain:** Condensed Matter
**Definition:** α+2β+γ=2 (Rushbrooke); 2β+γ=2ν (Widom); reduce exponent count to two independent.
**Atom or composite:** Atom (hyperscaling relations).
**Cost model:** Algebraic.
**Real wall?** Yes — consequences of scale invariance + thermodynamic stability.
**Cross-domain wiring:** Same scaling logic underlies finite-size scaling collapses in MC and DMRG.
**Notes:** Widom-Rushbrooke-Fisher 1960s.

### bkt-transition (cross-domain alias: `berezinskii-kosterlitz-thouless`, `2d-xy-transition`)
**Domain:** Condensed Matter
**Definition:** Topological transition in 2D XY-like systems driven by unbinding of vortex-antivortex pairs; correlation function changes from power law to exponential.
**Atom or composite:** Composite (vortices + RG).
**Cost model:** Coulomb-gas mapping; MC with vortex counting.
**Real wall?** Yes — infinite-order transition; superfluid density jumps by universal value 2T/π.
**Cross-domain wiring:** Universal jump observed in He films, 2D SCs, BEC; same RG flow as Kosterlitz equations.
**Notes:** Berezinskii (1971), Kosterlitz-Thouless (1973, 2016 Nobel).

### vortex-antivortex-unbinding (cross-domain alias: `topological-defect-unbinding`, `kt-mechanism`)
**Domain:** Condensed Matter
**Definition:** At T_BKT, bound vortex-antivortex pairs proliferate and ionize, destroying quasi-LRO.
**Atom or composite:** Composite (defect statistics + Coulomb interaction).
**Cost model:** RG flow of fugacity-stiffness plane.
**Real wall?** Yes — entropy of free vortex matches interaction energy at T_BKT.
**Cross-domain wiring:** Mechanism reused in 2D melting (KTHNY), 2D Coulomb gas, gauge theories.
**Notes:** Kosterlitz-Thouless (1973).

### quantum-phase-transition (cross-domain alias: `qpt`, `t-zero-transition`)
**Domain:** Condensed Matter
**Definition:** Continuous transition at T=0 driven by a nonthermal parameter (pressure, doping, field); critical fluctuations are quantum.
**Atom or composite:** Composite (quantum critical fixed point + dimensional reduction).
**Cost model:** Mapped to d+z classical model.
**Real wall?** Yes — quantum critical fan extends to finite T.
**Cross-domain wiring:** Underlies heavy fermions, cuprates, magnetic systems; tests of holographic dualities.
**Notes:** Hertz (1976); Sachdev textbook.

### quantum-critical-point (cross-domain alias: `qcp`, `strange-metal-source`)
**Domain:** Condensed Matter
**Definition:** Point at T=0 in (g, T) phase diagram where ξ diverges; ρ(T)~T, χ singular above QCP.
**Atom or composite:** Atom (critical end point of QPT).
**Cost model:** Locating via thermodynamic, transport scaling.
**Real wall?** Often masked by SC dome (cuprates, pnictides, heavy fermions).
**Cross-domain wiring:** Source of non-Fermi-liquid (strange-metal) behavior; ML phase classifiers near QCPs.
**Notes:** Sachdev (1999); Lohneysen RMP (2007).

### deconfined-criticality (cross-domain alias: `dqcp`, `landau-violating-transition`)
**Domain:** Condensed Matter
**Definition:** Direct continuous transition between two ordered phases with distinct symmetry breakings, mediated by deconfined gauge field; violates Landau paradigm.
**Atom or composite:** Composite (emergent gauge field + matter fields).
**Cost model:** Lattice MC, J-Q model.
**Real wall?** Continuous in conjectures; recent evidence for weakly first-order in some models.
**Cross-domain wiring:** Same structure as some lattice gauge theories; conceptual import for emergent gauge physics.
**Notes:** Senthil-Vishwanath-Balents-Sachdev-Fisher Science (2004).

### conformal-field-theory-cm (cross-domain alias: `cft-cm`, `2d-critical-cft`)
**Domain:** Condensed Matter
**Definition:** Effective field theory at critical point with conformal symmetry; 2D CFTs (Ising, Potts, Luttinger) classified by central charge c and primary fields.
**Atom or composite:** Composite (RG fixed point + conformal symmetry).
**Cost model:** Algebraic in primary content.
**Real wall?** Bootstrap constraints rigid.
**Cross-domain wiring:** Underpins Luttinger liquids, FQH edge theory, AdS/CFT in holographic CM.
**Notes:** Belavin-Polyakov-Zamolodchikov (1984); Cardy book.

---

## Optical Properties

### drude-lorentz-cm (cross-domain alias: `dl-dielectric`, `metal-plus-oscillator`)
**Domain:** Condensed Matter
**Definition:** ε(ω) = ε_∞ − ω_p²/(ω²+iωγ) + Σ_j f_j ω_pj²/(ω_j²−ω²−iωγ_j); free + bound electrons.
**Atom or composite:** Composite (Drude + Lorentz oscillators).
**Cost model:** Analytic; fit to optical data.
**Real wall?** Kramers-Kronig consistent.
**Cross-domain wiring:** Same dielectric function used in photonics-optics; fits ARPES, ellipsometry data; underlies plasmonics.
**Notes:** Drude (1900); Wooten Optical Properties of Solids.

### plasma-frequency-solid (cross-domain alias: `omega-p`, `bulk-plasmon`)
**Domain:** Condensed Matter
**Definition:** ω_p = √(ne²/ε₀m*); below ω_p, metals reflect; above, become transparent.
**Atom or composite:** Atom (bulk plasmon frequency).
**Cost model:** O(1) from n, m*.
**Real wall?** Yes — controls UV plasma edge.
**Cross-domain wiring:** Same as plasma physics ω_p; basis of `plasmon-polariton` in photonics-optics.
**Notes:** Bohm-Pines (1953); Pines-Nozières.

### dielectric-function (cross-domain alias: `epsilon-omega`, `linear-response-eps`)
**Domain:** Condensed Matter
**Definition:** Complex ε(q,ω) = ε₁ + iε₂; relates D to E in linear response; ε₂(ω) → optical absorption.
**Atom or composite:** Composite (Lindhard or RPA from band structure).
**Cost model:** Sum over occupied-unoccupied O(N_b² N_k).
**Real wall?** Causality + Kramers-Kronig fix ε₁ given ε₂.
**Cross-domain wiring:** Backbone of optical-spectra simulation; coupled to GW (W=ε⁻¹V) and BSE.
**Notes:** Lindhard (1954); Ehrenreich-Cohen.

### kramers-kronig (cross-domain alias: `kk-relations`, `causality-link`)
**Domain:** Condensed Matter
**Definition:** Real and imaginary parts of causal response are Hilbert transforms: χ₁(ω) = (1/π)P∫χ₂(ω′)/(ω′−ω)dω′.
**Atom or composite:** Atom (causality `compare` of real-imag).
**Cost model:** Hilbert transform O(N log N).
**Real wall?** Yes — direct consequence of causality + integrability.
**Cross-domain wiring:** Same KK relations across all linear response; used in optical, magnetic, transport spectroscopies.
**Notes:** Kramers (1927), Kronig (1926).

### exciton-cm (cross-domain alias: `bound-electron-hole-pair`, `wannier-or-frenkel-exciton`)
**Domain:** Condensed Matter
**Definition:** Coulomb-bound electron-hole pair below band gap; Wannier (large, semiconductor) vs Frenkel (small, molecular).
**Atom or composite:** Composite (e-h relative-motion problem).
**Cost model:** Hydrogenic for Wannier; ED in molecular crystals.
**Real wall?** Binding energy E_b = μe⁴/(2ε²ℏ²); subtracts from optical gap.
**Cross-domain wiring:** Drives optoelectronics in semiconductors; 2D TMD excitons have large E_b ~ 0.5 eV.
**Notes:** Wannier (1937); Frenkel (1931); Klingshirn semiconductor optics.

### polariton-cm (cross-domain alias: `light-matter-mode`, `dispersion-anticrossing`)
**Domain:** Condensed Matter
**Definition:** Mixed state of photon + matter excitation (exciton, phonon, plasmon, magnon); dispersion shows anticrossing.
**Atom or composite:** Composite (cavity QED-like dressing).
**Cost model:** 2-level coupled-mode O(1).
**Real wall?** Rabi splitting Ω_R measures coupling strength.
**Cross-domain wiring:** Underlies microcavity exciton-polariton BEC (Kasprzak 2006); strong-coupling chemistry, magnon-polaritons in YIG-cavity.
**Notes:** Pekar; Hopfield (1958).

### exciton-polariton-bec (cross-domain alias: `polariton-condensate`, `non-equilibrium-bec`)
**Domain:** Condensed Matter
**Definition:** Non-equilibrium BEC of cavity exciton-polaritons at K; coherent emission ("polariton laser").
**Atom or composite:** Composite (driven-dissipative BEC).
**Cost model:** Gross-Pitaevskii with gain/loss.
**Real wall?** Light effective mass ~10⁻⁴ m_e enables high T_BEC; finite polariton lifetime sets nonequilibrium nature.
**Cross-domain wiring:** Bridges photonics-optics and BEC physics; analog quantum simulator.
**Notes:** Kasprzak et al. Nature (2006); Deng-Haug-Yamamoto RMP.

### polaron-cm (cross-domain alias: `electron-phonon-quasiparticle`, `frohlich-polaron`)
**Domain:** Condensed Matter
**Definition:** Electron dressed by lattice deformation cloud; small (Holstein) when localized in unit cell, large (Fröhlich) when extended.
**Atom or composite:** Composite (electron + phonon dressing).
**Cost model:** Diagrammatic QMC; variational Feynman.
**Real wall?** Polaronic m* > m_band; mobility scales accordingly.
**Cross-domain wiring:** Key for organic semiconductors, polar perovskites; first-principles polaron calculations via Frohlich coupling extracted from DFT.
**Notes:** Landau-Pekar (1948); Fröhlich (1954).

### bipolaron (cross-domain alias: `two-electron-bound-polaron`, `pair-polaron`)
**Domain:** Condensed Matter
**Definition:** Bound state of two polarons by overscreened phonon-mediated attraction; candidate pairing mechanism in some SCs.
**Atom or composite:** Composite (two polarons + Coulomb + phonon).
**Cost model:** DQMC, exact diagonalization on small clusters.
**Real wall?** Stability vs Coulomb repulsion requires strong e-ph coupling.
**Cross-domain wiring:** Bipolaron BEC proposed for high-T_c by Alexandrov-Mott; relevant for some organic and oxide SCs.
**Notes:** Alexandrov-Ranninger (1981).

### magnon-polariton (cross-domain alias: `cavity-magnonics`, `magnon-photon-coupling`)
**Domain:** Condensed Matter
**Definition:** Hybridized magnon + microwave-photon mode in cavity + magnet (YIG-on-superconducting-resonator); Rabi splitting Ω_R.
**Atom or composite:** Composite (cavity + magnon coupling).
**Cost model:** Two-mode coupled equations O(1).
**Real wall?** Strong coupling Ω > κ, γ required for coherence.
**Cross-domain wiring:** Platform for quantum-magnonics, microwave-to-optical transduction in quantum-computing.
**Notes:** Tabuchi et al. PRL (2014).

---

## Semiconductors

### doping-cm (cross-domain alias: `n-p-doping`, `donor-acceptor`)
**Domain:** Condensed Matter
**Definition:** Substitutional impurities donating (n-type) or accepting (p-type) carriers; controls n, p in semiconductors over many decades.
**Atom or composite:** Atom (chemical control of carrier density).
**Cost model:** O(1) per dopant; clustering by Monte Carlo at high doping.
**Real wall?** Solubility, compensation, Mott criterion n^(1/3) a_B ≈ 0.25.
**Cross-domain wiring:** Foundation of all semiconductor devices; ML for dopant-defect prediction.
**Notes:** Kittel Ch. 8; Yu-Cardona.

### band-gap-engineering (cross-domain alias: `bge`, `alloy-and-strain-tuning`)
**Domain:** Condensed Matter
**Definition:** Tuning E_g via alloying (Al_xGa_{1-x}As), strain, quantum confinement, or external field.
**Atom or composite:** Composite (compositional + structural tuning).
**Cost model:** DFT screening; ML surrogates trained on Materials Project.
**Real wall?** Vegard's law deviations; bowing parameters.
**Cross-domain wiring:** Backbone for LEDs, lasers, detectors across the IR/visible/UV; ML high-throughput screens.
**Notes:** Yu-Cardona; Adachi Properties of III-V book.

### heterostructure (cross-domain alias: `band-offset`, `multilayer-semiconductor`)
**Domain:** Condensed Matter
**Definition:** Stack of dissimilar semiconductors creating band-offsets; type-I, type-II, type-III line-ups.
**Atom or composite:** Composite (interface + bulk band structures).
**Cost model:** Anderson rule (electron affinities) for first approximation.
**Real wall?** Polarization, strain, interfacial states modify offsets.
**Cross-domain wiring:** Enabling technology for HEMTs, QW lasers, solar cells.
**Notes:** Anderson (1962); Capasso review.

### multi-quantum-well (cross-domain alias: `mqw`, `stack-of-wells`)
**Domain:** Condensed Matter
**Definition:** Periodic stack of QWs with barriers thick enough to suppress coupling; multiplies optical gain.
**Atom or composite:** Composite (array of QWs).
**Cost model:** Per-QW + ensemble averaging.
**Real wall?** Strain budget; thermal stability.
**Cross-domain wiring:** Workhorse active region of telecom and visible diode lasers.
**Notes:** Capasso (1985); Yu-Cardona.

### superlattice-cm (cross-domain alias: `mini-bands`, `coupled-qw-stack`)
**Domain:** Condensed Matter
**Definition:** Periodic stack of thin layers; long-period lattice generates "mini-bands" in growth direction; basis of QCL.
**Atom or composite:** Composite (artificial periodicity in z).
**Cost model:** Solve 1D Kronig-Penney or DFT.
**Real wall?** Bloch oscillations only when ωτ > 1.
**Cross-domain wiring:** Esaki-Tsu superlattices launched 2D electron physics; QCL family of mid-IR lasers; cousin of photonic crystals.
**Notes:** Esaki-Tsu (1970); Faist QCL (1994).

### 2deg (cross-domain alias: `two-dimensional-electron-gas`, `quasi-2d-carriers`)
**Domain:** Condensed Matter
**Definition:** Electrons confined to nm-scale layer with free in-plane motion; n ~ 10¹¹–10¹² cm⁻²; high mobility μ ~ 10⁷ cm²/Vs.
**Atom or composite:** Composite (heterojunction + modulation doping).
**Cost model:** Self-consistent Schrödinger-Poisson O(N_z) per iter.
**Real wall?** Mobility set by ionized impurities (modulation doping reduces).
**Cross-domain wiring:** Substrate for IQHE/FQHE; topological surface state in TIs; high-electron-mobility transistors (HEMTs).
**Notes:** Ando-Fowler-Stern RMP (1982).

### mosfet-physics (cross-domain alias: `mos-transistor`, `field-effect-transistor`)
**Domain:** Condensed Matter
**Definition:** Three-terminal device where gate field controls channel carrier density; long-channel I-V: I_D = (W/L)μC_ox[(V_GS−V_T)V_DS − V_DS²/2].
**Atom or composite:** Composite (MOS capacitor + inversion channel + ohmic contacts).
**Cost model:** Long-channel analytic; TCAD numerical for short channel.
**Real wall?** Scaling pinned by short-channel effects, gate-leakage tunneling.
**Cross-domain wiring:** Foundation of digital electronics; ML transistor compact-models (BSIM, NN-FET).
**Notes:** Sze Physics of Semiconductor Devices.

### mos-capacitor (cross-domain alias: `gate-dielectric-stack`, `metal-oxide-semiconductor`)
**Domain:** Condensed Matter
**Definition:** Metal-insulator-semiconductor structure; controls inversion/accumulation/depletion regimes via V_G.
**Atom or composite:** Composite (capacitor + space charge).
**Cost model:** Depletion approximation + iterative Poisson.
**Real wall?** Inversion charge ~exp((V_G−V_T)/V_T).
**Cross-domain wiring:** Building block of MOSFET; underlies QV characterization, dielectric scaling debates.
**Notes:** Sze; Nicollian-Brews MOS book.

### threshold-voltage (cross-domain alias: `v-t-fet`, `strong-inversion-onset`)
**Domain:** Condensed Matter
**Definition:** Gate voltage V_T at which channel inverts and strong inversion begins; depends on doping, oxide thickness, work-function.
**Atom or composite:** Atom (device parameter).
**Cost model:** O(1) from analytics.
**Real wall?** Yes — V_T spread is a key process-variation metric.
**Cross-domain wiring:** ML compact-models predict V_T across process corners.
**Notes:** Sze; BSIM models.

### channel-length-modulation (cross-domain alias: `clm`, `early-effect-mosfet`)
**Domain:** Condensed Matter
**Definition:** Effective channel shortening with V_DS in saturation; finite output conductance g_o = λI_D.
**Atom or composite:** Composite (depletion encroachment).
**Cost model:** First-order analytic.
**Real wall?** Yes — sets transistor intrinsic gain g_m/g_o.
**Cross-domain wiring:** Analog circuit design lifeblood; same Early effect in BJTs.
**Notes:** Sze.

### short-channel-effects (cross-domain alias: `sce`, `dibl-roll-off`)
**Domain:** Condensed Matter
**Definition:** V_T roll-off, DIBL, punch-through when L → drain depletion width; subthreshold slope degradation.
**Atom or composite:** Composite (drain-induced barrier lowering + lateral fields).
**Cost model:** TCAD; analytical with empirical fit.
**Real wall?** Yes — limit of planar MOSFET scaling; motivated FinFET, GAA-FET.
**Cross-domain wiring:** Drove evolution to finFET/nanowire/sheet transistors; ML/process co-design.
**Notes:** Taur-Ning; IRDS roadmap.

### bsim-model (cross-domain alias: `bsim-compact`, `mosfet-spice-model`)
**Domain:** Condensed Matter
**Definition:** Hierarchy of compact MOSFET models (BSIM3/4/CMG) for circuit simulation; physics-based parameter set ~100s.
**Atom or composite:** Composite (analytic device model).
**Cost model:** O(1) per node evaluation in SPICE.
**Real wall?** Calibration accuracy bounded by extraction noise + process variability.
**Cross-domain wiring:** Industry standard SPICE; ML compact-modeling explored as next-gen.
**Notes:** Hu-Chenming et al. BSIM (UC Berkeley); Si2 standard.

---

## 2D Materials

### graphene (cross-domain alias: `2d-c-honeycomb`, `dirac-2d-material`)
**Domain:** Condensed Matter
**Definition:** Single layer of carbon in honeycomb lattice; linear band structure near K, K′ with v_F ≈ 10⁶ m/s.
**Atom or composite:** Atom (massless 2D Dirac fermions).
**Cost model:** 2-band tight-binding analytic.
**Real wall?** Stable as suspended membrane; quantum Hall accessible at room T due to large ω_c.
**Cross-domain wiring:** Same Dirac equation as TI surface, 2D Weyl; basis for moiré flat bands.
**Notes:** Novoselov-Geim (2004, Nobel 2010).

### klein-tunneling (cross-domain alias: `chiral-tunneling`, `perfect-transmission`)
**Domain:** Condensed Matter
**Definition:** Normal-incidence transmission probability T=1 through a potential barrier for Dirac electrons due to pseudospin conservation.
**Atom or composite:** Atom (relativistic tunneling).
**Cost model:** Analytic from Dirac equation.
**Real wall?** Yes — no exponential suppression for normal incidence.
**Cross-domain wiring:** Particle-physics Klein paradox realized in graphene transport; underlies pn-junction transport in graphene.
**Notes:** Klein (1929); Katsnelson-Novoselov-Geim (2006).

### graphene-transistor (cross-domain alias: `gfet`, `2d-fet`)
**Domain:** Condensed Matter
**Definition:** FET with graphene channel; ambipolar; absence of gap limits I_on/I_off but enables HF/RF performance.
**Atom or composite:** Composite (Dirac channel + gate + contacts).
**Cost model:** Drift-diffusion plus quantum capacitance.
**Real wall?** No gap → no off-state for digital logic; analog/RF roles.
**Cross-domain wiring:** Drove search for gapped 2D materials (TMDs, phosphorene) for digital roles.
**Notes:** Schwierz Nat. Nanotech. (2010).

### half-integer-qhe (cross-domain alias: `n-plus-half-graphene-qhe`, `4-fold-degenerate-ll`)
**Domain:** Condensed Matter
**Definition:** σ_xy = 4(N+1/2)e²/h in monolayer graphene; LL spectrum E_N = sgn(N)v_F√(2eℏB|N|) with N=0 LL at zero energy.
**Atom or composite:** Composite (Dirac LLs + valley-spin degeneracy).
**Cost model:** Analytic Dirac LL spectrum.
**Real wall?** Yes — direct fingerprint of Dirac fermions; observed at room T.
**Cross-domain wiring:** Distinct from conventional 2DEG IQHE; same N=0 anomaly in other Dirac materials.
**Notes:** Novoselov, Zhang Nature (2005).

### tmd-2d (cross-domain alias: `transition-metal-dichalcogenide`, `mos2-class`)
**Domain:** Condensed Matter
**Definition:** Family MX_2 (M = Mo, W; X = S, Se, Te) with monolayer direct gap 1–2 eV; valleys at K, K′; strong SOC at top of valence band.
**Atom or composite:** Composite (transition-metal d bands + chalcogen p).
**Cost model:** 3-band tight-binding; ab initio with SOC.
**Real wall?** Monolayer direct, bilayer indirect — interlayer coupling matters.
**Cross-domain wiring:** Hosts valley physics, valley Hall, optoelectronics, twistable platforms.
**Notes:** Mak-Heinz (2010); Xiao-Yao-Niu (2012).

### valleytronics (cross-domain alias: `valley-pseudospin`, `k-and-k-prime-logic`)
**Domain:** Condensed Matter
**Definition:** Use valley (K vs K′) as information carrier; selected by circularly polarized light in TMDs.
**Atom or composite:** Atom (quantum number beyond spin).
**Cost model:** Optical selection rules from k·p.
**Real wall?** Valley relaxation times τ_v limit logic.
**Cross-domain wiring:** Parallel to spintronics; same chirality-helicity matching as in chiral photonics-optics.
**Notes:** Xiao-Liu-Feng-Heinz-Mak (Nat. Phys. 2012).

### hbn-substrate (cross-domain alias: `hexagonal-boron-nitride`, `atomically-flat-substrate`)
**Domain:** Condensed Matter
**Definition:** Wide-gap (≈ 6 eV) layered insulator; atomically flat, low-disorder substrate for 2D materials.
**Atom or composite:** Composite (2D wide-gap insulator).
**Cost model:** Layer-by-layer DFT.
**Real wall?** Lattice mismatch with graphene ~1.8% creates moiré superlattice.
**Cross-domain wiring:** Substrate of choice for high-mobility graphene + TMD heterostructures; UV photonics gap material.
**Notes:** Dean et al. Nat. Nanotech. (2010).

### phosphorene (cross-domain alias: `2d-black-phosphorus`, `anisotropic-2d`)
**Domain:** Condensed Matter
**Definition:** Monolayer black phosphorus with puckered structure; direct gap ~2 eV; strongly anisotropic effective masses.
**Atom or composite:** Composite (puckered 2D group-V).
**Cost model:** DFT + GW; tight-binding analytic.
**Real wall?** Air-sensitive; encapsulation required.
**Cross-domain wiring:** Anisotropic in-plane transport, polarization-sensitive optics; flexible electronics target.
**Notes:** Li et al. Nat. Nanotech. (2014).

### mxene (cross-domain alias: `2d-carbide-nitride`, `ti3c2-class`)
**Domain:** Condensed Matter
**Definition:** Family of 2D transition-metal carbides/nitrides M_{n+1}X_n; high conductivity, tunable surface chemistry.
**Atom or composite:** Composite (layered + functional groups).
**Cost model:** DFT on slabs.
**Real wall?** Surface termination strongly affects electronic structure.
**Cross-domain wiring:** Targets for EMI shielding, supercapacitors, electrocatalysis; ML high-throughput.
**Notes:** Naguib-Gogotsi (2011).

### bilayer-graphene (cross-domain alias: `blg`, `tunable-gap-graphene`)
**Domain:** Condensed Matter
**Definition:** Two stacked graphene layers (Bernal AB); parabolic touching at K; perpendicular E-field opens tunable gap.
**Atom or composite:** Composite (intra + inter-layer hoppings).
**Cost model:** 4-band tight-binding analytic.
**Real wall?** Field-induced gap ≤ 0.25 eV.
**Cross-domain wiring:** Hosts displacement-field-tuned topology, gate-induced valley Hall; substrate for twisted bilayers.
**Notes:** McCann-Falko (2006); Castro Neto RMP (2009).

### twisted-bilayer-graphene (cross-domain alias: `tbg`, `magic-angle-graphene`)
**Domain:** Condensed Matter
**Definition:** Two graphene layers stacked with relative twist θ; near θ_m ≈ 1.1° flat bands emerge with bandwidth ~10 meV.
**Atom or composite:** Composite (moiré + interlayer coupling).
**Cost model:** Bistritzer-MacDonald continuum + HF.
**Real wall?** Twist-angle disorder ~0.1°; sample dependence.
**Cross-domain wiring:** Hosts correlated insulators, SC, magnetism in same device; tunable strongly-correlated playground.
**Notes:** Bistritzer-MacDonald (2011); Cao et al. (2018).

### moire-flat-band (cross-domain alias: `mfb`, `flat-band-from-twist`)
**Domain:** Condensed Matter
**Definition:** Narrow Bloch band emerging from moiré periodicity in twisted/heterostructured 2D materials; ratio U/W >> 1 makes correlations dominate.
**Atom or composite:** Composite (moiré superlattice + band engineering).
**Cost model:** Continuum + interactions.
**Real wall?** Sensitive to lattice relaxation, displacement field, dielectric environment.
**Cross-domain wiring:** Discovery platform for correlated/topological physics across graphene, TMDs, rhombohedral graphene.
**Notes:** Bistritzer-MacDonald (2011); Cao et al. (2018); Park et al. (2021).

---

## Computational Methods & Spectroscopies

### car-parrinello-md (cross-domain alias: `cpmd`, `unified-electronic-ionic-md`)
**Domain:** Condensed Matter
**Definition:** Lagrangian dynamics simulating electrons and ions simultaneously; fictitious electron mass keeps electrons near Born-Oppenheimer surface.
**Atom or composite:** Composite (extended Lagrangian + MD).
**Cost model:** O(N³) per MD step (or O(N) linear-scaling).
**Real wall?** Time step bounded by fictitious electron oscillation; needs μ << M_ion.
**Cross-domain wiring:** First-principles MD on ps timescales; replaced now by efficient Born-Oppenheimer MD + ML force fields.
**Notes:** Car-Parrinello PRL (1985).

### born-oppenheimer-md (cross-domain alias: `bomd`, `dft-on-the-fly`)
**Domain:** Condensed Matter
**Definition:** Solve KS equations self-consistently at each ionic step, then propagate ions classically.
**Atom or composite:** Composite (DFT + ion MD).
**Cost model:** O(N_scf × N³) per ionic step; faster with extrapolated wavefunctions.
**Real wall?** Femtosecond timestep needed for H-containing systems.
**Cross-domain wiring:** Standard ab initio MD; major training data source for ML force fields.
**Notes:** Iftimie-Tuckerman-Minary review.

### classical-md (cross-domain alias: `md`, `lennard-jones-md`)
**Domain:** Condensed Matter
**Definition:** Integrate Newton's equations for classical particles with empirical interaction potential.
**Atom or composite:** Composite (force-field + ODE integrator).
**Cost model:** O(N) with neighbor lists; long-range via Ewald/PME.
**Real wall?** Empirical potentials limit accuracy and transferability.
**Cross-domain wiring:** LAMMPS, GROMACS; foundational across CM, chemistry, biology.
**Notes:** Alder-Wainwright (1957); Frenkel-Smit book.

### embedded-atom-method (cross-domain alias: `eam`, `metallic-many-body-potential`)
**Domain:** Condensed Matter
**Definition:** U = (1/2)Σ_ij φ(r_ij) + Σ_i F(ρ_i); embedding-energy functional of local density mimicking metallic bonding.
**Atom or composite:** Composite (pair + embedding).
**Cost model:** O(N) with neighbor lists.
**Real wall?** Misses angular dependence; insufficient for covalent.
**Cross-domain wiring:** Workhorse for metals MD; basis of ML EAM extensions (HDNNP).
**Notes:** Daw-Baskes (1984).

### tersoff-potential (cross-domain alias: `bond-order-potential`, `bop`)
**Domain:** Condensed Matter
**Definition:** Bond-order-dependent potential including angular terms; suitable for covalent Si, C, Ge.
**Atom or composite:** Composite (pair + 3-body + bond order).
**Cost model:** O(N) with neighbor lists.
**Real wall?** Element-specific; limited transferability.
**Cross-domain wiring:** Predecessor of REBO, AIREBO for hydrocarbons; ML potentials supersede for new chemistry.
**Notes:** Tersoff (1988).

### rebo-potential (cross-domain alias: `reactive-empirical-bond-order`, `airebo`)
**Domain:** Condensed Matter
**Definition:** Reactive bond-order for hydrocarbons; allows bond making/breaking.
**Atom or composite:** Composite (Tersoff + chemistry-aware bond switching).
**Cost model:** O(N).
**Real wall?** No charge equilibration; less accurate for π systems.
**Cross-domain wiring:** Standard for nanotubes, graphene MD; replaced by ML for high accuracy.
**Notes:** Brenner (1990); AIREBO Stuart (2000).

### reaxff (cross-domain alias: `reactive-force-field`, `charge-equilibration-bond-order`)
**Domain:** Condensed Matter
**Definition:** Bond-order force field with charge equilibration (EEM); simulates reactions across diverse chemistries.
**Atom or composite:** Composite (bond-order + electrostatics + many-body).
**Cost model:** O(N) per MD step; charge update O(N²) or iterative O(N).
**Real wall?** Parameterization complex; transferability bounded.
**Cross-domain wiring:** Bridge between empirical FF and full DFT; calibration target for ML potentials.
**Notes:** van Duin-Goddard (2001).

### kinetic-monte-carlo (cross-domain alias: `kmc`, `event-driven-mc`)
**Domain:** Condensed Matter
**Definition:** Stochastic simulation evolving system via discrete events with rates k_i; advances time by Δt = -ln(R)/Σk_i.
**Atom or composite:** Composite (rate catalog + Gillespie-like algorithm).
**Cost model:** O(log N_events) per step (BKL).
**Real wall?** Requires precomputed rate catalog and barrier search.
**Cross-domain wiring:** Surface growth (MBE), diffusion, defect dynamics; bridges DFT (barriers) and ms-scale evolution.
**Notes:** Bortz-Kalos-Lebowitz (1975); Voter review.

### multiscale-modeling (cross-domain alias: `multi-scale`, `qm-mm-coupling`)
**Domain:** Condensed Matter
**Definition:** Coupling models across length/time scales: QM/MM, AtC, MD-FEM; consistent passing of forces/energies.
**Atom or composite:** Composite (multi-method orchestration).
**Cost model:** Method-dependent; bottleneck usually in highest-fidelity region.
**Real wall?** Handshake regions, boundary-condition consistency.
**Cross-domain wiring:** Workflow analog of multi-physics elsewhere; ML potentials now serve as bridging models.
**Notes:** Warshel-Levitt (Nobel 2013) QM/MM.

### arpes-spectroscopy (cross-domain alias: `arpes`, `angle-resolved-photoemission`)
**Domain:** Condensed Matter
**Definition:** Photoemission with angle resolution; measures occupied band structure E(k) directly.
**Atom or composite:** Composite (photoemission + analyzer).
**Cost model:** Each spectrum O(N_E × N_k_∥).
**Real wall?** Surface-sensitive (~Å escape depth); k_⊥ uncertain.
**Cross-domain wiring:** Gold-standard probe of bands, Fermi surface, kinks (e-ph), gap symmetry in SCs.
**Notes:** Damascelli RMP (2003).

### stm-sts (cross-domain alias: `scanning-tunneling-microscopy`, `tunneling-spectroscopy`)
**Domain:** Condensed Matter
**Definition:** Atomic-scale imaging via tunneling current; dI/dV ∝ local DOS.
**Atom or composite:** Composite (tip + sample + tunneling).
**Cost model:** Pixel-by-pixel scanning; lock-in for dI/dV.
**Real wall?** Resolution limited by tip; vacuum tunneling barrier.
**Cross-domain wiring:** Sees atomic orbitals, vortex cores, Friedel oscillations; SP-STM for spin texture.
**Notes:** Binnig-Rohrer (Nobel 1986).

### afm-mfm (cross-domain alias: `atomic-force-microscopy`, `magnetic-force-microscopy`)
**Domain:** Condensed Matter
**Definition:** Non-contact imaging by cantilever response to force; MFM uses magnetic tip for stray-field imaging.
**Atom or composite:** Composite (cantilever + interaction force).
**Cost model:** Per-pixel mechanical response.
**Real wall?** Tip-sample convolution; long-range vs short-range force distinction.
**Cross-domain wiring:** Images insulators (vs STM); domain imaging in MFM; basis of qPlus and AFM-IR variants.
**Notes:** Binnig-Quate-Gerber (1986).

### inelastic-neutron-scattering (cross-domain alias: `ins`, `phonon-magnon-spectroscopy`)
**Domain:** Condensed Matter
**Definition:** Neutron scattering with energy + momentum transfer; directly maps dispersion of phonons and magnons.
**Atom or composite:** Composite (triple-axis or TOF spectrometer).
**Cost model:** Reactor/spallation; counting times of order day.
**Real wall?** Flux + sample size; resolution function trade-off.
**Cross-domain wiring:** Reveals phonon dispersion (benchmark for DFT), spin-wave spectra, fractionalized excitations.
**Notes:** Brockhouse (Nobel 1994); Squires.

### raman-spectroscopy-cm (cross-domain alias: `raman-solid`, `phonon-spectroscopy-optical`)
**Domain:** Condensed Matter
**Definition:** Inelastic scattering of light by phonons (and magnons, plasmons); selection rules from crystal symmetry.
**Atom or composite:** Composite (light + phonon scattering).
**Cost model:** Sub-second per spectrum; mapping seconds-hours.
**Real wall?** Only zone-center modes (k≈0) accessible.
**Cross-domain wiring:** Fingerprint of structure (G/D bands in graphene, 2D layer count); ML for spectrum classification.
**Notes:** Raman (Nobel 1930); Cardona Light Scattering in Solids.

### infrared-spectroscopy-cm (cross-domain alias: `ir-solid`, `phonon-absorption`)
**Domain:** Condensed Matter
**Definition:** Absorption of IR light by IR-active (polar) phonons and electronic transitions; reflectivity → ε(ω) via KK.
**Atom or composite:** Composite (light + phonon/electron coupling).
**Cost model:** FTIR fast O(1) per spectrum.
**Real wall?** Only IR-active modes via dipole selection rule.
**Cross-domain wiring:** Probes LO-TO splitting, vibrational fingerprint; superconducting gap optics.
**Notes:** Born-Huang; Wooten textbook.

### x-ray-absorption-spec (cross-domain alias: `xas`, `xanes-exafs`)
**Domain:** Condensed Matter
**Definition:** Absorption cross-section near and above core edges; XANES = bonding/oxidation state; EXAFS = radial structure.
**Atom or composite:** Composite (core-level excitation + multiple scattering).
**Cost model:** FEFF, FDMNES; ML XANES interpretation.
**Real wall?** Element-selective; synchrotron-class sources needed.
**Cross-domain wiring:** Local structure probe complementing diffraction; operando catalysis, batteries.
**Notes:** Stern-Sayers (1971); Rehr-Albers (2000).

### nmr-solids (cross-domain alias: `solid-state-nmr`, `mas-nmr`)
**Domain:** Condensed Matter
**Definition:** Nuclear magnetic resonance in solids; chemical shift, dipolar, quadrupolar, Knight shift; MAS narrows lines.
**Atom or composite:** Composite (Zeeman + chemical shift + dipolar/quadrupolar).
**Cost model:** Pulse sequences; multidimensional acquisition.
**Real wall?** Sensitivity scales with γ³ B² N.
**Cross-domain wiring:** Probes local electronic structure (Knight shift), spin dynamics, glassy disorder.
**Notes:** Slichter NMR book; MAS Andrew (1959).

### esr-epr (cross-domain alias: `electron-spin-resonance`, `epr`)
**Domain:** Condensed Matter
**Definition:** Microwave absorption by electron spins in B; g-factor and hyperfine fingerprints.
**Atom or composite:** Composite (Zeeman + crystal field + hyperfine).
**Cost model:** CW or pulsed EPR.
**Real wall?** Requires unpaired electrons; sensitivity ~10⁹ spins.
**Cross-domain wiring:** Identifies defects, transition-metal ions; ENDOR adds nuclear precision; basis of NV-center magnetometry in quantum-computing.
**Notes:** Zavoisky (1944); Atherton textbook.

### mossbauer-spec (cross-domain alias: `recoilless-gamma`, `isomer-shift-spec`)
**Domain:** Condensed Matter
**Definition:** Recoilless emission/absorption of γ-rays (e.g., ⁵⁷Fe 14.4 keV); ultra-narrow lines probe isomer shift, quadrupole splitting, magnetic hyperfine.
**Atom or composite:** Composite (Mössbauer effect + hyperfine probe).
**Cost model:** Velocity-scanned absorption; min ÷hr per spectrum.
**Real wall?** Requires Mössbauer-active isotope; recoil-free fraction ∝ exp(−⟨u²⟩q²).
**Cross-domain wiring:** Iron oxidation states, magnetism, charge ordering; planetary mineralogy.
**Notes:** Mössbauer (Nobel 1961).

---

## Soft Matter & Other CM Phenomena

### liquid-crystal-nematic (cross-domain alias: `nematic`, `n-director`)
**Domain:** Condensed Matter
**Definition:** Anisotropic fluid with orientational order along director n but no positional order; order parameter Q_ij = S(3n_in_j−δ_ij)/2.
**Atom or composite:** Composite (rod-like molecules + alignment).
**Cost model:** Landau-de Gennes; coarse-grained MD.
**Real wall?** Yes — first observed Reinitzer 1888; basis of LCD displays.
**Cross-domain wiring:** Same Q-tensor symmetry as biaxial molecular materials; foundation of LC physics.
**Notes:** Reinitzer (1888); de Gennes (Nobel 1991).

### liquid-crystal-smectic (cross-domain alias: `smectic-a`, `layered-lc`)
**Domain:** Condensed Matter
**Definition:** Layered LC phase with positional order in one direction + fluid layers; SmA director ⊥ layers, SmC tilted.
**Atom or composite:** Composite (layering + nematic order).
**Cost model:** Landau theory with layering order parameter.
**Real wall?** 1D positional + 2D fluidic — Mermin-Wagner-like fluctuation suppression of true 3D LRO.
**Cross-domain wiring:** Ferroelectric SmC* used in fast LC displays; LC analog of 2D melting theories.
**Notes:** de Gennes-Prost LC textbook.

### liquid-crystal-cholesteric (cross-domain alias: `chiral-nematic`, `helical-pitch`)
**Domain:** Condensed Matter
**Definition:** Nematic with chiral molecules forms helix of pitch p; Bragg reflection at λ = np gives structural color.
**Atom or composite:** Composite (nematic + intrinsic twist).
**Cost model:** Frank elasticity; Bragg analysis.
**Real wall?** Pitch sensitive to T, fields (thermochromism).
**Cross-domain wiring:** Iridescent biological structures (beetles); selective mirrors in photonics.
**Notes:** Reinitzer; de Vries (1951).

### elastomer-cm (cross-domain alias: `polymer-network`, `rubber`)
**Domain:** Condensed Matter
**Definition:** Cross-linked polymer network deformable up to several × original size; entropic elasticity F ~ k_BT·n.
**Atom or composite:** Composite (cross-linked polymer + entropy).
**Cost model:** Gaussian chain theory; FEM for large deformation.
**Real wall?** Glass transition T_g sets working window.
**Cross-domain wiring:** Liquid-crystal elastomers couple LC order and elasticity; programmable shape change.
**Notes:** Flory; Treloar; Warner-Terentjev LCE book.

### gel-cm (cross-domain alias: `polymer-gel`, `swollen-network`)
**Domain:** Condensed Matter
**Definition:** Cross-linked polymer + solvent; volume sensitive to T, pH, ions (Flory-Huggins).
**Atom or composite:** Composite (polymer + solvent + cross-links).
**Cost model:** Flory-Rehner theory; MD for dynamics.
**Real wall?** Phase separation, syneresis.
**Cross-domain wiring:** Drug delivery, sensors, soft robotics; ml-training-enabled responsive gels.
**Notes:** Flory; Tanaka (1978) gel collapse.

### colloidal-crystal (cross-domain alias: `microsphere-lattice`, `opal-like-array`)
**Domain:** Condensed Matter
**Definition:** Self-assembled periodic arrays of μm-scale colloids; lattice constant in optical wavelength range.
**Atom or composite:** Composite (entropy or charge-driven self-assembly).
**Cost model:** Brownian dynamics; phase-field for ordering.
**Real wall?** Crystallization defects similar to atomic crystals.
**Cross-domain wiring:** Photonic-bandgap materials in photonics-optics; model system for atomic crystallography.
**Notes:** Pieranski (1983); Pusey-van Megen (1986).

### flory-polymer-theory (cross-domain alias: `polymer-statistics`, `r-n-half-scaling`)
**Domain:** Condensed Matter
**Definition:** Self-avoiding walk statistics: R ∝ N^ν with ν = 3/(d+2) (Flory); good-solvent expansion.
**Atom or composite:** Composite (random-walk + excluded volume).
**Cost model:** Analytic + MC verification.
**Real wall?** Universal in good solvent; theta-point distinct.
**Cross-domain wiring:** Same statistics underlie SAW in graph theory, polymer-melt rheology, protein conformations.
**Notes:** Flory (1953); de Gennes Scaling Concepts.

### reptation-cm (cross-domain alias: `polymer-tube-model`, `de-gennes-reptation`)
**Domain:** Condensed Matter
**Definition:** Long-polymer dynamics in melt: chain reptates along contour ("tube") imposed by entanglements; τ_rep ∝ N³.
**Atom or composite:** Composite (entanglement + diffusion).
**Cost model:** Doi-Edwards stochastic theory.
**Real wall?** Yes — viscosity η ∝ N^{3.4} explained quantitatively.
**Cross-domain wiring:** Foundation of polymer melt rheology; underlies extrusion, processing.
**Notes:** de Gennes (1971); Doi-Edwards book.

### rouse-model (cross-domain alias: `bead-spring-no-hydrodynamics`, `linear-mode-polymer`)
**Domain:** Condensed Matter
**Definition:** Polymer as bead-spring chain in viscous solvent without hydrodynamic interaction; relaxation modes τ_n ∝ N²/n².
**Atom or composite:** Composite (Gaussian chain + viscous drag).
**Cost model:** Analytic mode decomposition.
**Real wall?** Misses long-range hydrodynamics (Zimm correction).
**Cross-domain wiring:** Same normal-mode decomposition as `harmonic-oscillator-chain`; basis of polymer dynamics.
**Notes:** Rouse (1953).

### zimm-model (cross-domain alias: `hydrodynamic-polymer`, `bead-spring-with-oseen`)
**Domain:** Condensed Matter
**Definition:** Rouse + Oseen hydrodynamic interactions in dilute solvent; modifies relaxation spectrum (τ_n ∝ N^{3ν}).
**Atom or composite:** Composite (chain + solvent backflow).
**Cost model:** Pre-averaged Oseen for analytic; Brownian dynamics for full.
**Real wall?** Captures dilute solution viscosity, diffusion.
**Cross-domain wiring:** Underlies SEC chromatography, polymer-solution rheology.
**Notes:** Zimm (1956).

### self-assembly-cm (cross-domain alias: `bottom-up-assembly`, `entropy-driven-ordering`)
**Domain:** Condensed Matter
**Definition:** Spontaneous organization of building blocks into ordered structures, driven by entropy/interactions without external direction.
**Atom or composite:** Composite (interactions + free-energy landscape).
**Cost model:** Brownian dynamics, dissipative particle dynamics, MC.
**Real wall?** Kinetic trapping (glasses) frequent.
**Cross-domain wiring:** Liquid crystals, block copolymers, DNA origami, colloidal photonic crystals; ML for design.
**Notes:** Whitesides Science (2002).

---

## Other CM Phenomena

### charge-density-wave (cross-domain alias: `cdw`, `peierls-state`)
**Domain:** Condensed Matter
**Definition:** Periodic modulation of electron density ρ(r) = ρ₀ + δρ cos(Q·r+φ) accompanied by lattice distortion; opens partial gap at Fermi surface.
**Atom or composite:** Composite (Fermi surface nesting + electron-phonon coupling).
**Cost model:** Self-consistent linear-response; ab-initio identification of Kohn anomalies.
**Real wall?** Yes — preferred Q from nesting; commensurate vs incommensurate.
**Cross-domain wiring:** Sliding CDW transport (Fröhlich); competes with SC in NbSe₂, TaS₂; ML for transition detection.
**Notes:** Peierls (1955); Grüner RMP (1988).

### spin-density-wave (cross-domain alias: `sdw`, `magnetic-version-of-cdw`)
**Domain:** Condensed Matter
**Definition:** Itinerant antiferromagnetic order with periodic magnetization S(r) = S₀ cos(Q·r); incommensurate Q possible.
**Atom or composite:** Composite (nesting-driven instability in spin channel).
**Cost model:** RPA susceptibility peak at Q.
**Real wall?** Same nesting physics as CDW but spin channel.
**Cross-domain wiring:** Realized in Cr (Q≠0), pnictides (stripe AFM); competes with SC.
**Notes:** Overhauser (1962).

### peierls-instability (cross-domain alias: `1d-dimerization`, `kohn-anomaly-1d`)
**Domain:** Condensed Matter
**Definition:** 1D metal unstable to lattice dimerization opening gap at k_F; trans-polyacetylene paradigm.
**Atom or composite:** Composite (1D nesting + electron-phonon coupling).
**Cost model:** Mean-field BCS-like gap equation.
**Real wall?** Always unstable at T=0 for 1D metal with e-ph coupling.
**Cross-domain wiring:** Mechanism behind SSH model; hosts solitons and fractional charge in polyacetylene.
**Notes:** Peierls (1955).

### mott-transition (cross-domain alias: `metal-insulator-transition`, `mit-correlation`)
**Domain:** Condensed Matter
**Definition:** Transition from metal to insulator driven by U/W or filling change; bandwidth-driven (V₂O₃) or doping-driven (cuprates).
**Atom or composite:** Composite (Hubbard U + bandwidth competition).
**Cost model:** DMFT shows characteristic 3-peak spectral function near transition.
**Real wall?** First-order at finite T for bandwidth-driven (V₂O₃).
**Cross-domain wiring:** Central paradigm; cold-atom realizations in Fermi-Hubbard optical lattices.
**Notes:** Mott (1949); Imada-Fujimori-Tokura RMP (1998).

### metal-insulator-transition-cm (cross-domain alias: `mit`, `general-mit`)
**Domain:** Condensed Matter
**Definition:** Sharp change in resistivity vs control parameter (T, n, p, B); mechanisms: Mott, Anderson, Peierls, magnetic.
**Atom or composite:** Composite (multiple mechanisms in same phase space).
**Cost model:** Transport, ARPES, optical; theory case-by-case.
**Real wall?** Each mechanism has distinct signatures (compressibility, gap formation, localization length).
**Cross-domain wiring:** Direct industrial relevance (VO₂ thermal switches); ML phase classifiers.
**Notes:** Mott (1990) book.

### polaron-formation (cross-domain alias: `electron-lattice-self-trapping`, `holstein-trap`)
**Domain:** Condensed Matter
**Definition:** Crossover from large to small polaron as e-ph coupling λ grows; small polaron mobility activated.
**Atom or composite:** Composite (electron + lattice deformation self-energy).
**Cost model:** Diagrammatic QMC, variational.
**Real wall?** Yes — controls carrier transport in polar materials (TiO₂, halide perovskites).
**Cross-domain wiring:** Important for organic semiconductors, oxide thermoelectrics; ml-training carrier-mobility models.
**Notes:** Holstein (1959); Alexandrov-Devreese review.

### jahn-teller-distortion (cross-domain alias: `jt-effect`, `orbital-symmetry-breaking`)
**Domain:** Condensed Matter
**Definition:** Spontaneous lattice distortion lifting orbital degeneracy; Mn³⁺, Cu²⁺ ions in octahedra famously JT-active.
**Atom or composite:** Composite (orbital + lattice coupling).
**Cost model:** DFT optimization with broken symmetry; vibronic Hamiltonians.
**Real wall?** Theorem: any non-Kramers orbital degeneracy is unstable.
**Cross-domain wiring:** Drives orbital order in manganites; cooperative JT in colossal-magnetoresistance materials.
**Notes:** Jahn-Teller (1937); Bersuker book.

### ferroelectricity (cross-domain alias: `fe-polarization`, `displacive-soft-mode`)
**Domain:** Condensed Matter
**Definition:** Spontaneous switchable electric polarization below T_c; soft-mode (displacive) or order-disorder mechanism.
**Atom or composite:** Composite (broken inversion + structural order parameter).
**Cost model:** Berry-phase polarization (King-Smith-Vanderbilt); Landau theory.
**Real wall?** P quantized mod electron displacement per lattice; tunneling switching.
**Cross-domain wiring:** Underlies FeRAM, FE field-effect transistors; analog of FM with E ↔ B.
**Notes:** Lines & Glass; Cochran (1960) soft mode.

### multiferroic (cross-domain alias: `mf`, `magnetoelectric-coupled`)
**Domain:** Condensed Matter
**Definition:** Material with simultaneous ferroic orders (magnetic + electric ± elastic); coupling enables electric control of magnetism.
**Atom or composite:** Composite (M order + P order + coupling).
**Cost model:** DFT + Berry phase; spin spirals.
**Real wall?** Type-I (independent) vs type-II (magnetism-induced P) — latter rare but couples strongly.
**Cross-domain wiring:** Low-power memory candidates; BiFeO₃, TbMnO₃ flagship materials.
**Notes:** Schmid (1994); Spaldin-Fiebig review.

### magnetoelectric-effect (cross-domain alias: `me-coupling`, `cross-coupling-h-e`)
**Domain:** Condensed Matter
**Definition:** Magnetization induced by E and polarization induced by B: P_i = α_ij H_j, M_j = α_ij E_i.
**Atom or composite:** Composite (cross-tensor response).
**Cost model:** DFT linear-response of M and P.
**Real wall?** α bounded by √(ε χ_m).
**Cross-domain wiring:** Cr₂O₃ as classic; topological axion ME proposed in TI; underlies multiferroic devices.
**Notes:** Astrov (1960); Fiebig review.

### piezoelectricity (cross-domain alias: `mechanical-electrical-coupling`, `e-from-strain`)
**Domain:** Condensed Matter
**Definition:** Polarization induced by mechanical strain (and vice versa): P_i = d_ijk σ_jk; requires noncentrosymmetric crystal.
**Atom or composite:** Composite (strain-polarization coupling).
**Cost model:** DFT piezoelectric tensor d_ijk via Berry phase + finite strain.
**Real wall?** Symmetry: 20/32 point groups allow nonzero d.
**Cross-domain wiring:** Sensors, actuators, SAW devices in electromagnetics-antennas; quartz oscillators set time standards.
**Notes:** Curie brothers (1880); Nye Tensor textbook.

### pyroelectricity (cross-domain alias: `pe-thermal-polarization`, `dp-dt-effect`)
**Domain:** Condensed Matter
**Definition:** Spontaneous polarization changes with temperature: ΔP = p ΔT; only in 10/32 point groups.
**Atom or composite:** Atom (linear ME for T).
**Cost model:** Temperature-derivative of Berry-phase P.
**Real wall?** Symmetry-restricted to polar point groups.
**Cross-domain wiring:** IR detectors, thermal imaging; subset of ferroelectric materials.
**Notes:** Lang Pyroelectricity book; Tagantsev.

### kondo-insulator (cross-domain alias: `ki`, `narrow-gap-heavy-fermion`)
**Domain:** Condensed Matter
**Definition:** Heavy-fermion compound with hybridization gap at low T; small but finite charge gap; SmB₆, FeSi.
**Atom or composite:** Composite (Kondo lattice at integer filling + hybridization).
**Cost model:** PAM mean field; DMFT.
**Real wall?** Gap closes under pressure; possibly topological (TKI).
**Cross-domain wiring:** Proposed topological Kondo insulator SmB₆ links HF and TI physics.
**Notes:** Aeppli-Fisk review (1992); Dzero-Sun-Galitski-Coleman (2010).

### heavy-fermion-state (cross-domain alias: `hf-state`, `m-star-1000`)
**Domain:** Condensed Matter
**Definition:** Metal with renormalized m*/m ~ 10²–10³ from Kondo lattice coherence; γT linear specific heat huge.
**Atom or composite:** Composite (Kondo lattice + Fermi liquid).
**Cost model:** DMFT, slave-boson.
**Real wall?** Coherence below T* ~ T_K.
**Cross-domain wiring:** Hosts unconventional SC, QCP, hidden order; tests of Luttinger theorem.
**Notes:** Stewart RMP (1984); Coleman review.

### cdw-sliding (cross-domain alias: `frohlich-conduction`, `non-ohmic-cdw`)
**Domain:** Condensed Matter
**Definition:** Nonohmic conduction by collective sliding of CDW above threshold field E_T; narrow-band noise from washboard.
**Atom or composite:** Composite (CDW + pinning + dissipation).
**Cost model:** Fukuyama-Lee-Rice; Larkin pinning.
**Real wall?** Threshold field set by impurity pinning strength.
**Cross-domain wiring:** Same dynamics as flux-flow in vortex lattices; depinning analogous to friction transitions.
**Notes:** Grüner-Zawadowski-Chaikin (1981); Brazovskii-Nattermann review.

### luttinger-theorem (cross-domain alias: `lt`, `fs-volume-counts-electrons`)
**Domain:** Condensed Matter
**Definition:** Volume enclosed by Fermi surface counts particle number per unit cell, even with interactions.
**Atom or composite:** Atom (topological invariant of Fermi liquid).
**Cost model:** Conceptual; verified by ARPES, dHvA.
**Real wall?** Holds for any Fermi liquid; violated in fractionalized phases.
**Cross-domain wiring:** Anchors interpretation of ARPES on cuprates; violations signal non-Fermi liquid / topological order.
**Notes:** Luttinger (1960); Oshikawa (2000) generalization.

### stoner-criterion (cross-domain alias: `n-ef-times-i`, `ferromagnetism-condition`)
**Domain:** Condensed Matter
**Definition:** Itinerant ferromagnetism when I N(ε_F) > 1; I = exchange-energy parameter.
**Atom or composite:** Atom (instability criterion).
**Cost model:** N(ε_F) from DFT × Stoner I.
**Real wall?** Yes — Fe, Co, Ni satisfy criterion; many magnetic-impurity systems near boundary.
**Cross-domain wiring:** Used for predicting itinerant magnetism in alloys; ml-training high-throughput magnet screening.
**Notes:** Stoner (1938); Andersen.

### luttinger-kohn-method (cross-domain alias: `lk-multiband-kp`, `4-band-valence`)
**Domain:** Condensed Matter
**Definition:** Multiband k·p Hamiltonian including degenerate-band perturbation; standard for valence-band structure in zincblende.
**Atom or composite:** Composite (k·p + degenerate perturbation).
**Cost model:** Diagonalize small N×N matrix per k.
**Real wall?** Accurate near Γ-point; needs more bands for k farther out.
**Cross-domain wiring:** Workhorse for IR/THz semiconductor optics, QW band structures.
**Notes:** Luttinger-Kohn (1955); Pikus-Bir.

### kane-mele-z2 (cross-domain alias: `z2-from-fu-kane`, `parity-criterion`)
**Domain:** Condensed Matter
**Definition:** For centrosymmetric systems, (−1)^ν = Π_TRIM Π_n ξ_n with ξ_n parities of occupied bands.
**Atom or composite:** Atom (parity-based shortcut).
**Cost model:** O(N_TRIM × N_b).
**Real wall?** Requires inversion symmetry.
**Cross-domain wiring:** Underlies high-throughput topological screening; ICSD-based searches by Materials Project.
**Notes:** Fu-Kane (2007).

### symmetry-indicators (cross-domain alias: `si-topology`, `po-vishwanath`)
**Domain:** Condensed Matter
**Definition:** Set of integers built from irreducible-rep labels at high-symmetry points that detect band topology.
**Atom or composite:** Composite (representation theory + topology).
**Cost model:** Lookup tables per space group (Bilbao).
**Real wall?** Enumerates classes; doesn't always uniquely identify topology.
**Cross-domain wiring:** Enabled topological materials databases (TMD, Materiae); standard tool in high-throughput discovery.
**Notes:** Po-Vishwanath-Watanabe (2017); Bradlyn et al. Topological Quantum Chemistry.

### topological-materials-database (cross-domain alias: `tmd-database`, `materiae`)
**Domain:** Condensed Matter
**Definition:** Database of materials classified by topology using symmetry indicators on existing crystal structures (ICSD).
**Atom or composite:** Composite (high-throughput DFT + topology workflows).
**Cost model:** O(N_materials × N_DFT).
**Real wall?** Limited by quality of DFT and structure data.
**Cross-domain wiring:** Drives ML-augmented screening; Bilbao topological server.
**Notes:** Vergniory et al. Nature (2019); Tang-Po-Vishwanath-Wan (2019).

---


