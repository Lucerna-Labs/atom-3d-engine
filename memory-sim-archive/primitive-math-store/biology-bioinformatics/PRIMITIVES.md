# Biology / Bioinformatics — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions.

---

## Sequencing & Assembly

### seq-align-global (cross-domain alias: `Needleman-Wunsch`, `global-alignment`)
**Domain:** Biology / Bioinformatics
**Definition:** Align two sequences over their full length. Scoring: match = +m, mismatch = −s, gap = −g. Dynamic programming: DP[i,j] = max(DP[i-1,j]−g, DP[i,j-1]−g, DP[i-1,j-1]+score(a_i,b_j)).
**Atom or composite:** Composite: build DP matrix → traceback → emit alignment.
**Real wall?** Yes — O(m·n) space and time for sequences of length m, n. For genomes (billions of bp), exact global alignment is infeasible.
**Cross-domain wiring:** DP matrix = fold over sequence pairs. In signal: cross-correlation via dynamic programming. In text: edit distance.

### seq-align-local (cross-domain alias: `Smith-Waterman`, `local-alignment`)
**Domain:** Biology / Bioinformatics
**Definition:** Find the highest-scoring local alignment between subsequences. DP[i,j] = max(0, DP[i-1,j]−g, DP[i,j-1]−g, DP[i-1,j-1]+score). Only positive scores contribute.
**Atom or composite:** Composite: same as Needleman-Wunsch but with zero-floor on DP.
**Real wall?** Yes — O(m·n) per alignment. For large-scale comparison, heuristic indexing (BLAST) is necessary.
**Cross-domain wiring:** Smith-Waterman = cross-correlation with gap penalties. In signal: local similarity search. In retrieval: BM25 = probabilistic local alignment.

### seq-blast (cross-domain alias: `BLAST`, `heuristic-alignment`)
**Domain:** Biology / Bioinformatics
**Definition:** Heuristic local alignment: seed-and-extend. (1) Break query into words of length W. (2) Build neighborhood index for target. (3) Extend HSPs (high-scoring segment pairs) that meet threshold. (4) Combine HSPs.
**Atom or composite:** Composite: word scan → neighborhood lookup → HSP extension → gapped extension → combine.
**Real wall?** Yes — BLAST misses alignments below the word-score threshold. Sensitivity vs speed trade-off is the real design constraint.
**Cross-domain wiring:** BLAST = seeded local alignment with extension. In signal: matched filter with seeded trigger. In retrieval: TF-IDF + extension.
**Notes:** BLAT, LAST, DIAMOND are faster BLAST variants. HMMER (profile HMMs) is slower but more sensitive.

### seq-dna-seq (cross-domain alias: `DNA-sequencing`, `NGS`, `Illumina-sequencing`)
**Domain:** Biology / Bioinformatics
**Definition:** Convert physical DNA to digital sequence. Illumina: bridge PCR → clonal clusters → sequencing by synthesis (fluorescent nucleotides). Oxford Nanopore: single molecule, electrical signal.
**Atom or composite:** Composite: fragment DNA → attach adapters → amplify → sequence → basecall.
**Cost model:** ~$0.01/Mb (Illumina), ~$0.001/Mb (Nanopore, long reads).
**Real wall?** Yes — read length vs accuracy trade-off: Nanopore gives 10kb+ reads but ~95% accuracy; Illumina gives 150bp at 99.9%+ accuracy.
**Cross-domain wiring:** DNA sequencing = analog-to-digital conversion of biological information. In signal: ADC. In storage: biological storage.

### seq-assembly (cross-domain alias: `genome-assembly`, `de-Bruijn-graph`)
**Domain:** Biology / Bioinformatics
**Definition:** Assemble fragments (reads) into a genome. De Bruijn graph: nodes = k-mers, edges = (k+1)-mer overlaps. Eulerian path = assembled genome.
**Atom or composite:** Composite: build k-mer index → build de Bruijn graph → find Eulerian path → emit contigs.
**Real wall?** Yes — repeat regions in the genome create ambiguities in the de Bruijn graph. Repeats shorter than read length can't be resolved without extra information.
**Cross-domain wiring:** De Bruijn graph assembly = finding Eulerian path = same as finding Eulerian circuit in a directed graph. In signal: building a convolutional code from observed fragments.
**Notes:** Long reads (Nanopore, PacBio) resolve repeats that short reads can't. Hybrid assembly (long + short reads) is the current best approach.

---

## Molecular Dynamics

### md-potential (cross-domain alias: `force-field`, `molecular-potential`, `Lennard-Jones`)
**Domain:** Biology / Bioinformatics
**Definition:** V(r) = 4ε[(σ/r)^{12} − (σ/r)^6]. The Lennard-Jones potential: repulsive at short range, attractive at longer range.
**Atom or composite:** Atom (potential energy function)
**Real wall?** No. But Lennard-Jones is a simplified model — real molecular potentials include many-body terms, polarization, etc.
**Cross-domain wiring:** LJ potential = pairwise interaction = graph edge weight in physics. In ML: kernel function.

### md-verlet (cross-domain alias: `Verlet-integration`, `MD-integrator`)
**Domain:** Biology / Bioinformatics
**Definition:** x(t+Δt) = 2x(t) − x(t−Δt) + a(t)·Δt². Position-based integration, no explicit velocity. Energy-conserving.
**Atom or composite:** Composite: compute forces → update positions via Verlet → optionally compute velocities (v = (x(t+Δt)−x(t−Δt))/2Δt).
**Real wall?** No.
**Cross-domain wiring:** Verlet integration = discrete approximation of Newton's equations of motion. In physics: same integrator used in classical mechanics. In signal: trapezoidal integration.
**Notes:** Velocity Verlet: explicitly stores velocities, same accuracy as Verlet.

### md-leapfrog (cross-domain alias: `leapfrog-integration`, `half-step-integrator`)
**Domain:** Biology / Bioinformatics
**Definition:** v(t+Δt/2) = v(t−Δt/2) + a(t)·Δt; x(t+Δt) = x(t) + v(t+Δt/2)·Δt. Leapfrog over time.
**Atom or composite:** Composite: update half-step velocity → update position → update half-step velocity.
**Real wall?** No.
**Cross-domain wiring:** Leapfrog = Verlet in staggered form. In signal: finite difference scheme for second-order ODEs.

### md-thermostat (cross-domain alias: `NVT-ensemble`, `Langevin-thermostat`)
**Domain:** Biology / Bioinformatics
**Definition:** Langevin dynamics: add friction and random force: m·a = F − γ·v + √(2γkT)·R(t). Maintains temperature T.
**Atom or composite:** Composite: compute acceleration → add friction damping → add random force → integrate.
**Real wall?** No.
**Cross-domain wiring:** Langevin = Newtonian dynamics + damping + noise. In physics: Brownian motion = friction + random force. In signal: damped harmonic oscillator with noise input.

### md-barostat (cross-domain alias: `NPT-ensemble`, `pressure-coupling`)
**Domain:** Biology / Bioinformatics
**Definition:** Control system that maintains pressure by adjusting the simulation box volume. Berendsen barostat: box scaling at rate proportional to pressure deviation.
**Atom or composite:** Composite: compute pressure → compare to target → scale box → recompute coordinates.
**Real wall:** No.

### md-pme (cross-domain alias: `Particle-Mesh-Ewald`, `PME`, `electrostatics`)
**Domain:** Biology / Bioinformatics
**Definition:** Compute long-range electrostatic interactions efficiently. Split into: short-range real-space (direct sum) + long-range reciprocal space (FFT on mesh).
**Atom or composite:** Composite: assign charges to mesh → FFT → multiply by Green's function → inverse FFT → add to real-space.
**Cost model:** O(N log N) for N particles. vs O(N²) for direct sum.
**Real wall?** No.
**Cross-domain wiring:** PME = FFT-based convolution for long-range interactions. In physics: Ewald summation = same Fourier approach. In signal: convolution via FFT.
**Notes:** Particle Mesh Ewald is the standard for molecular dynamics electrostatics. Cutoff + shift is the cheaper approximation.

### md-free-energy (cross-domain alias: `FEP`, `TI`, `free-energy-calculation`)
**Domain:** Biology / Bioinformatics
**Definition:** Free energy perturbation (FEP): ΔG = −kT·log⟨exp(−(U_B−U_A)/kT)⟩_A. Thermodynamic integration: ΔG = ∫⟨∂U/∂λ⟩dλ.
**Atom or composite:** Composite: alchemical intermediate states → sample at each λ → compute ΔG via FEP or TI.
**Real wall?** Yes — sampling convergence is slow. The overlap between end states is the main bottleneck.
**Cross-domain wiring:** FEP = importance sampling in the alchemical space. In statistics: ratio of normalizing constants. In information theory: log-ratio of partition functions.

---

## Protein Structure

### protein-homology (cross-domain alias: `homology-modeling`, `threading`)
**Domain:** Biology / Bioinformatics
**Definition:** Build a protein model using a template of known structure. Align target sequence to template → copy template coordinates for conserved residues → model insertions/deletions.
**Atom or composite:** Composite: find template (BLAST/HMM) → align → copy coordinates → refine loops.
**Real wall?** No. But accuracy depends on sequence identity (>30% identity → reliable model).
**Cross-domain wiring:** Homology modeling = template-based structure prediction. In retrieval: nearest-neighbor lookup with alignment.

### protein-ab-initio (cross-domain alias: `AlphaFold2`, `RoseTTAFold`, `de-novo-folding`)
**Domain:** Biology / Bioinformatics
**Definition:** Predict 3D structure from sequence alone. AlphaFold2: (1) MSA generation + template search; (2) Evoformer (pairwise representation + MSA representation); (3) Structure module (3D coordinates from pairwise representations).
**Atom or composite:** Composite: MSA + templates → Evoformer → structure module → iterative refinement.
**Real wall?** Yes — AlphaFold2's accuracy degrades for proteins without good MSA (intrinsically disordered regions, novel folds, low-complexity).
**Cross-domain wiring:** AlphaFold2's attention mechanism = relational reasoning on the MSA. In ML: graph attention on residue contact graph.
**Notes:** AlphaFold2 (2020) was a breakthrough — CASP14 accuracy near experimental. RoseTTAFold (2021) is similar architecture.

### protein-docking (cross-domain alias: `molecular-docking`, `protein-ligand-docking`)
**Domain:** Biology / Bioinformatics
**Definition:** Predict binding mode of two molecules (e.g., protein + ligand). Search: sample orientations → score each → rank.
**Atom or composite:** Composite: sample orientations (search) → score (force field or ML) → rank.
**Real wall?** Yes — scoring functions are imperfect. The energy landscape has many local minima. Docking can miss the true binding mode.
**Cross-domain wiring:** Docking = fitting two objects together in 3D space = packing problem = box packing in logistics.
**Notes:** HADDOCK, HDOCK,ZDOCK are main tools. HDock = template-based + ab initio hybrid.

### protein-scoring (cross-domain alias: `force-field-score`, `ML-score`, `scoring-function`)
**Domain:** Biology / Bioinformatics
**Definition:** Evaluate the energy or binding affinity of a molecular pose. Physics-based: van der Waals + electrostatics + hydrogen bonds + desolvation. ML-based: trained on structural data.
**Atom or composite:** Composite: compute physical terms (Vdw, electrostatics, etc.) → combine → energy score.
**Real wall:** No.
**Cross-domain wiring:** Scoring = energy function evaluation. In optimization: objective function. In ML: loss function.

### protein-RMSD (cross-domain alias: `root-mean-square-deviation`, `structural-alignment`)
**Domain:** Biology / Bioinformatics
**Definition:** RMSD(A,B) = sqrt(1/N Σ ||a_i − b_i||²). After optimal rigid superposition (Kabsch algorithm).
**Atom or composite:** Composite: find optimal rotation (Kabsch) → compute RMSD of aligned structures.
**Real wall?** No.
**Cross-domain wiring:** RMSD = L2 distance between structures after optimal alignment. In graphics: ICP (iterative closest point) = same algorithm. In ML: Euclidean distance after Procrustes alignment.
**Notes:** Kabsch algorithm: find optimal rotation R that minimizes RMSD. Solved via SVD of the covariance matrix.

---

## Sequence Analysis

### seq-profile-hmm (cross-domain alias: `HMMER`, `profile-HMM`, `pfam`)
**Domain:** Biology / Bioinformatics
**Definition:** Profile HMM: a statistical model of a protein family. States: Match (M), Insert (I), Delete (D). Trained on aligned family members.
**Atom or composite:** Composite: build HMM from MSA → Viterbi algorithm to score new sequences → determine family membership.
**Real wall:** No.
**Cross-domain wiring:** Profile HMM = hidden Markov model with insert/delete states. In signal: HMM for sequence labeling. In NLP: CRF (conditional random field).
**Notes:** HMMER is the gold standard for protein family search. SAM-T and HHpred are alternative profile methods.

### seq-kmer-count (cross-domain alias: `k-mer`, `spectral-bin`, `k-tuple`)
**Domain:** Biology / Bioinformatics
**Definition:** Count all k-length substrings in a sequence. k=3 for nucleotides (codons), k=5-7 for proteins.
**Atom or composite:** Composite: scan(sequence) → fold(k-mer extraction) → hash → count.
**Real wall:** No. But k-mer space grows as 4^k (DNA) or 20^k (protein) — k must be chosen carefully.
**Cross-domain wiring:** K-mer counting = n-gram analysis in NLP. In signal: sliding window feature extraction.
**Notes:** Jellyfish and KMC2 are the fastest k-mer counters (streaming, parallel).

### seq-phylogeny (cross-domain alias: `phylogenetic-tree`, `UPGMA`, `NJ`)
**Domain:** Biology / Bioinformatics
**Definition:** Construct evolutionary tree from sequence alignments. Methods: UPGMA (distance-based), Neighbor-Joining (minimum evolution), Maximum Likelihood (statistical optimization).
**Atom or composite:** Composite: compute distance matrix → build tree → optionally optimize via ML.
**Real wall:** No. But phylogenetic trees have high variance — different methods can give different topologies.
**Cross-domain wiring:** Phylogeny = hierarchical clustering with evolutionary distance metric. In retrieval: hierarchical clustering.

### seq-motif-find (cross-domain alias: `regulatory-motif`, `PWM`, `position-weight-matrix`)
**Domain:** Biology / Bioinformatics
**Definition:** PWM: for each position j in motif of length L, count occurrences of each nucleotide/amino acid. Score sequence: product of position-specific scores.
**Atom or composite:** Composite: scan(sequence) → for each window: fold(product of PWM scores) → emit hits.
**Real wall:** No.
**Cross-domain wiring:** PWM = probabilistic pattern matching = a generative model of the motif. In signal: matched filter for known signal pattern.
**Notes:** log-odds PWM = log(PWM/P_background) improves discrimination.

---

## RNA & Genomics

### rna-fold (cross-domain alias: `RNA-secondary-structure`, `Nussinov-algorithm`)
**Domain:** Biology / Bioinformatics
**Definition:** Predict RNA secondary structure (base pairs) using dynamic programming. Nussinov: DP[i,j] = max(DP[i+1,j], DP[i,j-1], DP[i+1,j-1] + δ(i,j), max_k DP[i,k]+DP[k+1,j]).
**Atom or composite:** Composite: build DP matrix → traceback → emit base pairs.
**Real wall?** Yes — Nussinov ignores pseudoknots (crossing base pairs). Real RNA structures include pseudoknots.
**Cross-domain wiring:** RNA folding = optimal bracketing with pairing constraints. In parsing: CYK algorithm for RNA grammar.
**Notes:** ViennaRNA uses more sophisticated energy models. Context-free grammars (SCFGs) can handle pseudoknots.

### rna-ribozyme (cross-domain alias: `riboswitch`, `regulatory-RNA`)
**Domain:** Biology / Bioinformatics
**Definition:** RNA element that changes conformation upon binding a ligand, regulating gene expression.
**Atom or composite:** Composite: ligand binding → conformational change → expose/hide regulatory sequence.
**Real wall:** No.

### genome-annotate (cross-domain alias: `gene-prediction`, `ORF-finding`, `GFF-format`)
**Domain:** Biology / Bioinformatics
**Definition:** Identify genes, exons, introns, regulatory elements in a genome. Methods: ab initio (HMM, neural nets), evidence-based (RNA-seq, protein homology).
**Atom or composite:** Composite: scan(genome) → apply gene predictors → combine evidence → emit annotations.
**Real wall:** No. But gene annotation is uncertain — different predictors give different results.
**Cross-domain wiring:** Gene prediction = sequence labeling = HMM/CRF on genomic sequence. In NLP: named entity recognition.

### genome-variation (cross-domain alias: `SNP-calling`, `variant-calling`, `VCF-format`)
**Domain:** Biology / Bioinformatics
**Definition:** Identify differences between a sequenced genome and a reference. SNPs, indels, structural variants.
**Atom or composite:** Composite: align reads to reference → call variants (Bayesian: posterior probability) → filter → emit VCF.
**Real wall?** Yes — low-complexity regions, repeats, and repetitive sequences make variant calling unreliable.
**Cross-domain wiring:** Variant calling = difference detection between two sequences. In version control: diff algorithm. In signal: change detection.

---

## ML for Biology

### bio-cnn-protein (cross-domain alias: `ResNet-for-proteins`, `1D-CNN`)
**Domain:** Biology / Bioinformatics
**Definition:** 1D CNN over protein sequence (AAs encoded as one-hot or embeddings). Captures local sequence motifs.
**Atom or composite:** Composite: embed sequence → conv1D layers → pool → classify.
**Real wall:** No.
**Cross-domain wiring:** 1D CNN on sequence = local pattern detection. In signal: 1D convolution for time series.
**Notes:** DeepSequence, UniRep use LSTMs or transformers on sequences. ESM-2 (transformer) is the current best for protein representation learning.

### bio-attention-protein (cross-domain alias: `protein-Transformer`, `ESM-2`)
**Domain:** Biology / Bioinformatics
**Definition:** Transformer architecture trained on protein sequences (masked language modeling). Produces per-residue embeddings useful for structure, function, and interaction prediction.
**Atom or composite:** Composite: tokenize sequence → transformer layers → per-residue embeddings → task head.
**Real wall?** Yes — ESM-2 requires significant compute (ESM-2 650M parameters). Embedding a single protein is not trivial.
**Cross-domain wiring:** Protein language model = masked language model on biological sequence. In NLP: BERT on text.
**Notes:** ESM-2 embeddings are among the best features for protein function prediction, contact prediction, and structure prediction.

### bio-graph-protein (cross-domain alias: `Graph-NN-for-proteins`, `protein-graph-NN`)
**Domain:** Biology / Bioinformatics
**Definition:** Graph neural network where nodes = residues (or atoms), edges = spatial contacts (within 8Å) or sequence neighbors. Message passing over the graph.
**Atom or composite:** Composite: build graph from PDB or predicted structure → message-passing layers → graph-level readout → classify.
**Real wall:** No.
**Cross-domain wiring:** GNN on protein = message passing on residue graph. In chemistry: GNN for molecules. In social networks: GNN on social graph.
**Notes:** AlphaFold-Multimer (2021) and RosettaFold-DB use GNN for protein complex prediction.

### bio-gene-regulatory-network (cross-domain alias: `GRN-inference`, `Boolean-network`)
**Domain:** Biology / Bioinformatics
**Definition:** Model gene regulation as a Boolean network: X_i(t+1) = f(X_neighbor1, X_neighbor2, ...). Or continuous ODEs: dX_i/dt = f(Σ w_ij X_j).
**Atom or composite:** Composite: define network structure (from ChIP-seq, co-expression) → set dynamics rules → simulate.
**Real wall:** Yes — network inference from data is underdetermined (many possible networks explain the same data).
**Cross-domain wiring:** Boolean gene network = discrete dynamical system. In CS: finite state machine. In physics: Ising model dynamics.

---

## Systems Biology

### bio-sba-sir (cross-domain alias: `SIR-model`, `compartmental-epidemiology`)
**Domain:** Biology / Bioinformatics
**Definition:** dS/dt = −βSI/N; dI/dt = βSI/N − γI; dR/dt = γI. Susceptible → Infected → Recovered.
**Atom or composite:** Composite: ODE integration → update compartment sizes.
**Real wall:** No. But SIR is highly simplified — no spatial structure, homogeneous mixing.
**Cross-domain wiring:** SIR = compartmental model with mass action kinetics. In chemistry: reaction-diffusion. In distributed systems: epidemic gossip.
**Notes:** SEIR adds Exposed (infected but not yet infectious). SIS = recovering doesn't grant immunity.

### bio-sba-contact-network (cross-domain alias: `network-epidemiology`, `percolation-epidemic`)
**Domain:** Biology / Bioinformatics
**Definition:** Disease spreads over a contact network. Each node infected with probability β per infected neighbor per time step.
**Atom or composite:** Composite: simulate spread over network → compute attack rate, epidemic threshold.
**Real wall:** No.
**Cross-domain wiring:** Contact network epidemic = percolation on the network. In distributed systems: gossip protocol. In physics: bond percolation.
**Notes:** The epidemic threshold λ_c = 1/ρ_max where ρ_max = largest eigenvalue of adjacency matrix.

### bio-sba-lotka-volterra (cross-domain alias: `predator-prey`, `LV-equation`)
**Domain:** Biology / Bioinformatics
**Definition:** dX/dt = αX − βXY; dY/dt = δXY − γY. Prey (X) grows, gets eaten; predator (Y) reproduces proportional to consumption, dies naturally.
**Atom or composite:** Composite: ODE integration.
**Real wall:** No. But the pure LV model has neutral stability (cycles don't damp), real populations show damping or chaos.

### bio-sba-fba (cross-domain alias: `flux-balance-analysis`, `metabolic-network`)
**Domain:** Biology / Bioinformatics
**Definition:** Given a metabolic network (stoichiometry matrix S), find feasible flux vector v: maximize an objective (e.g., biomass) subject to S·v = 0 and v_min ≤ v ≤ v_max.
**Atom or composite:** Composite: define objective → solve LP: max cᵀv s.t. S·v = 0, bounds.
**Real wall:** No. FBA gives steady-state fluxes, not transient dynamics.
**Cross-domain wiring:** FBA = linear programming on a metabolic network. In engineering: linear constraints on flow networks.
**Notes:** FBA is genome-scale: E. coli model has ~2500 reactions, 1000 metabolites. COBRApy is the standard tool.

### bio-sba-petri-net (cross-domain alias: `Petri-net`, `concurrency-model`)
**Domain:** Biology / Bioinformatics
**Definition:** Places (tokens = chemical species) + Transitions (reactions) + Arcs (stoichiometry). Firing: tokens flow from input places to output places.
**Atom or composite:** Composite: determine enabled transitions → fire (consume inputs, produce outputs) → repeat.
**Real wall:** No.
**Cross-domain wiring:** Petri nets = concurrent system model = state machine with parallel transitions. In CS: process algebra. In engineering: workflow nets.
**Notes:** Colored Petri nets extend with token types (different chemical species).

### bio-sba-boolean-network (cross-domain alias: `Boolean-network`, `discrete-dynamics`)
**Domain:** Biology / Bioinformatics
**Definition:** Each node is 0 or 1. Each node's update rule is a Boolean function of its inputs. Iterate synchronously or asynchronously.
**Atom or composite:** Composite: evaluate all Boolean functions → update state → repeat.
**Real wall:** No. But Boolean networks have exponentially many attractors (cyclic attractors = cell types).
**Cross-domain wiring:** Boolean network = discrete dynamical system. In CS: digital logic circuit. In physics: Ising model with asynchronous updates.

---

## Summary: Biology Atom → Cross-Domain Wiring

| Biology Primitive | CS Alias | Physics Alias | ML Alias |
|---|---|---|---|
| seq-align-global | edit distance | energy minimization | sequence model |
| seq-blast | indexed search | signal matching | nearest-neighbor |
| md-verlet | numerical ODE | particle dynamics | physics simulation |
| md-thermostat | noise injection | Langevin dynamics | stochastic gradient |
| md-pme | FFT convolution | Ewald summation | conv layer |
| protein-docking | 3D packing | fitting problem | pose prediction |
| seq-profile-hmm | HMM | profile matching | CRFs |
| seq-kmer-count | n-gram count | feature extraction | bag-of-words |
| bio-sba-sir | compartmental model | reaction network | mixture model |
| bio-sba-contact-network | graph epidemic | percolation | GNN |
| bio-sba-fba | LP on network | steady-state flow | linear model |
| bio-sba-petri-net | workflow model | reaction network | process mining |
| bio-gene-reg-network | Boolean circuit | dynamical system | recurrent network |

---

*Last updated: 2026-06-21*
*Source doctrine: The Painted Fence — Jesse*


---

## Computational Neuroscience / Neural Coding Atoms

### stdp-learn (cross-domain alias: `spike-timing-dependent-plasticity`, `Hebbian-update`, `temporal Hebb`)
**Domain:** Biology / Bioinformatics
**Definition:** Synaptic strength changes based on the relative timing of pre- and post-synaptic spikes: Δw ∝ exp(−|Δt|/τ) where Δt = t_post − t_pre. Pre before post = LTP (potentiation). Post before pre = LTD (depression).
**Atom or composite:** Composite: detect spike pair → compute Δt → apply exponential weight update → accumulate.
**Cost model:** O(1) per spike pair. Implementable in hardware as analog circuits.
**Real wall?** Yes — STDP curves vary across brain regions and cell types. A single STDP rule cannot explain all learning.
**Cross-domain wiring:** STDP = temporal Hebbian learning = spike-time version of Oja's rule. In ML: contrastive Hebbian learning. In retrieval: attention = Hebbian plasticity between query and document tokens.
**Notes:** The classic STDP rule (Bi & Poo, 1998) is an exponential pair-based rule. Triplet STDP (Pfister & Gerstner, 2006) uses triplets of spikes for better modeling.

### neural-code-rate (cross-domain alias: `rate-coding`, `firing-rate`, `population-vector`)
**Domain:** Biology / Bioinformatics
**Definition:** Information is encoded in the average firing rate over a time window. Rate = spikes per second. Population vector: direction estimate = Σ w_i · r_i where w_i is preferred direction of neuron i, r_i is its firing rate.
**Atom or composite:** Composite: count spikes in window → divide by window width → encode.
**Cost model:** Requires a time window (typically 50-200ms). Too short = noisy rate estimate; too long = poor temporal resolution.
**Real wall?** Yes — rate coding ignores timing information. Many neurons use precise spike timing (temporal coding) that rate coding discards.
**Cross-domain wiring:** Rate coding = bag-of-words representation in NLP. In retrieval: frequency-based retrieval = rate coding.
**Notes:** Population vector decoding is robust to neuron dropout and noise — the brain uses population codes for this reason.

### neural-code-temporal (cross-domain alias: `temporal-coding`, `precise-spike-timing`, `latency-code`)
**Domain:** Biology / Bioinformatics
**Definition:** Information is encoded in the precise timing of spikes, not just rate. Latency code: time to first spike encodes stimulus intensity. Phase-of-firing: spike timing relative to LFP oscillation encodes information.
**Atom or composite:** Composite: detect spike → record precise time → encode relative to reference (LFP phase, stimulus onset).
**Cost model:** Higher temporal resolution requires faster spike detection — hardware constraints.
**Real wall?** Yes — temporal coding is fragile: small timing jitter destroys information. Rate coding is more robust.
**Cross-domain wiring:** Temporal coding = TF-IDF in the time domain (precise positions matter). In retrieval: temporal relevance weighting.
**Notes:** Phase-of-firing code (Pinkser & Meister, 2014) is particularly robust: LFP oscillations provide a reference clock that is shared across neurons.

### neural-decoder (cross-domain alias: `population-decoder`, `Bayesian-decoder`, `optimal-decoder`)
**Domain:** Biology / Bioinformatics
**Definition:** Given population activity, decode the represented stimulus. Bayesian decoder: P(stimulus | spike_trains) ∝ P(spike_trains | stimulus) · P(stimulus). Maximum likelihood: most likely stimulus given the spikes.
**Atom or composite:** Composite: compute likelihood of observed spikes given each stimulus → multiply by prior → maximize posterior.
**Cost model:** O(N_stimuli × N_neurons) for ML decoder. Gaussian process decoder: O(n³) for n observations.
**Real wall?** Yes — the decoder quality depends on having a good model of P(spikes | stimulus). For novel stimuli, the decoder fails.
**Cross-domain wiring:** Neural decoding = inverse inference on a latent stimulus variable. In retrieval: query decoding from retrieval scores.
**Notes:** The optimal (Bayesian) decoder achieves the information-theoretic limit set by the channel capacity of the neural code.

### neural-attention-brain (cross-domain alias: `visual-attention`, `feature-based-attention`, `spatial-attention`)
**Domain:** Biology / Bioinformatics
**Definition:** The brain selectively processes attended stimuli: attended features/locations show enhanced neural responses. Bottom-up: saliency-driven (fast). Top-down: task-driven (slower, voluntary).
**Atom or composite:** Composite: compute saliency map → apply gain modulation to sensory responses → enhanced processing of attended region.
**Cost model:** Attentional modulation is applied as a multiplicative gain — near-zero computational cost.
**Real wall?** Yes — attention is limited: only one or a few objects can be attended at once (the "attentional bottleneck").
**Cross-domain wiring:** Neural attention = feature-based gating = the same as attention mechanisms in transformers. In retrieval: query-dependent gating of document representations.
**Notes:** The zoom lens model: attention can be coarse (wide field, low resolution) or fine (narrow field, high resolution).

### neural-oscillation (cross-domain alias: `LFP-oscillation`, `gamma-alpha-theta`, `neural-rhythm`)
**Domain:** Biology / Bioinformatics
**Definition:** Local field potentials show oscillations: theta (4-8 Hz, hippocampus), alpha (8-12 Hz, visual cortex), beta (12-30 Hz, motor cortex), gamma (30-100 Hz, sensory cortex). Oscillations coordinate neural synchronization.
**Atom or composite:** Composite: record LFP → bandpass filter → compute power (gamma power increases with attention) or phase.
**Cost model:** Bandpass filtering is O(N) for N samples. Computing power spectrum adds O(N log N) (FFT).
**Real wall?** Yes — oscillations are not independent rhythms; they interact (cross-frequency coupling: theta phase modulates gamma amplitude).
**Cross-domain wiring:** Neural oscillations = spectral analysis of neural population activity. In signal: PSD analysis. In retrieval: spectral retrieval on the similarity matrix.
**Notes:** Gamma oscillations in visual cortex are phase-locked to attended stimuli — attention increases gamma power and interneuron synchronization.

### neural-place-cell (cross-domain alias: `place-cell`, `spatial-map`, `cognitive-map`)
**Domain:** Biology / Bioinformatics
**Definition:** Hippocampal neurons that fire when the animal is in a specific location ("place field"). Place fields are overlapping Gaussian-like receptive fields covering the environment.
**Atom or composite:** Composite: animal location → activate neurons whose place fields contain that location → population vector encodes location.
**Cost model:** Place fields are learned (LTP at location-triggered synapses). Formation takes minutes to hours.
**Real wall?** Yes — place fields are plastic: they can remap (change firing fields) when the environment changes.
**Cross-domain wiring:** Place cells = location basis functions = Gaussian RBF encoding of space. In retrieval: place cells = location-based indexing of the semantic space.
**Notes:** The population vector of place cells provides a continuous, robust estimate of location. Grid cells (medial entorhinal cortex) tile space with hexagonal lattices.

### neural-grid-cell (cross-domain alias: `grid-cell`, `hexagonal-tiling`, `path-integration`)
**Domain:** Biology / Bioinformatics
**Definition:** Medial entorhinal neurons with multiple place fields arranged on a hexagonal lattice. The spatial period (wavelength) ranges from ~30cm (small scale) to many meters (large scale).
**Atom or composite:** Composite: velocity integration → update path integrator state → project to grid cell firing pattern.
**Cost model:** Path integration requires continuous velocity input — the brain's vestibular system provides this.
**Real wall?** Yes — grid cells require continuous velocity input. Deafferentation (removing sensory input) disrupts grid cell firing.
**Cross-domain wiring:** Grid cells = Fourier basis functions for space = the same as Fourier features in ML. In retrieval: grid cells = multi-scale periodic indexing.
**Notes:** The hexagonal lattice is a near-optimal tessellation for maximizing spatial coverage with minimal redundancy (circle packing on a hexagonal lattice).

### neural-receptive-field (cross-domain alias: `RF-mapping`, `spike-triggered-average`, `reverse-correlation`)
**Domain:** Biology / Bioinformatics
**Definition:** Receptive field: the region of stimulus space where a neuron fires. Reverse correlation / spike-triggered average (STA): average the stimuli that preceded each spike → estimate the RF.
**Atom or composite:** Composite: present white noise stimulus → record spikes → average stimulus segments preceding each spike → STA.
**Cost model:** O(N_spikes × N_stimulus_dims) for STA computation.
**Real wall?** Yes — STA assumes the stimulus is uncorrelated over time (white noise). With correlated stimuli, STA is biased.
**Cross-domain wiring:** Receptive field = kernel of a filter = impulse response. In signal: the RF is the filter's impulse response. In ML: the RF of a convolutional filter.
**Notes:** Spike-triggered covariance (STC) finds multiple filters (not just one like STA) — essential for complex cells.

### neural-synchrony (cross-domain alias: `phase-synchrony`, `spike-phase-coupling`, `coordination`)
**Domain:** Biology / Bioinformatics
**Definition:** Neurons synchronize their spike timing relative to an oscillation phase (phase locking). Phase-locking value (PLV): circular mean of phase differences across trials.
**Atom or composite:** Composite: extract LFP phase → for each spike: record phase → compute circular mean → PLV.
**Cost model:** Hilbert transform for LFP phase extraction: O(N). PLV computation: O(N_trials).
**Real wall?** Yes — synchrony is dynamic (changes with cognitive state). Static synchrony measures miss important transient coordination.
**Cross-domain wiring:** Phase synchrony = phase alignment in signal processing. In retrieval: coordinating retrieval signals across passes.
**Notes:** Gamma oscillations (30-100 Hz) carry spike timing information for inter-areal communication ("communication through coherence" hypothesis).

### neural-memory-consolidation (cross-domain alias: `systems-consolidation`, `replay`, `hippocampal-cortex`)
**Domain:** Biology / Bioinformatics
**Definition:** Short-term memories in hippocampus are transferred to neocortex for long-term storage during sleep (slow-wave sleep, REM). Replay: hippocampal spike sequences are re-activated during sleep.
**Atom or composite:** Composite: encode experience in hippocampus → during sleep: replay sequences → consolidate to cortex.
**Cost model:** Consolidation is offline (during sleep) — not a real-time computational constraint.
**Real wall?** Yes — consolidation requires sleep. Sleep deprivation impairs memory consolidation.
**Cross-domain wiring:** Memory consolidation = offline processing to improve future retrieval. In retrieval: background index updates = offline consolidation.
**Notes:** Sharp-wave ripples (SWRs, 150-200 Hz) in hippocampus during sleep/awake rest are the substrate of replay — disrupting SWRs impairs consolidation.

### neural-dyn-system (cross-domain alias: `neural-dynamics`, `continuous-attractor`, `line-attractor`)
**Domain:** Biology / Bioinformatics
**Definition:** Neural circuits as dynamical systems: continuous attractor networks implement continuous variables (head direction, spatial location). Stable states = attractor positions.
**Atom or composite:** Composite: define network connectivity (recurrent weights) → dynamics: dθ/dt = −∂E/∂θ + noise → settle to attractor.
**Cost model:** Simulation of large neural networks is expensive (O(N²) for N neurons). Use mean-field approximations for tractability.
**Real wall?** Yes — continuous attractors require fine-tuned recurrent connectivity. Biological circuits approximate this with distributed, noisy components.
**Cross-domain wiring:** Neural dynamics = continuous attractor = gradient descent on an energy landscape. In ML: energy-based models.
**Notes:** Head direction cells (in entorhinal cortex) are a continuous attractor for head orientation. They persist without visual input (dead reckoning).

### neural-population-dynamics (cross-domain alias: `manifold-decoding`, `neural-geometry`, `tangential-intrinsic`)
**Domain:** Biology / Bioinformatics
**Definition:** Population activity lies on a low-dimensional manifold embedded in the high-dimensional firing rate space. Geometry of this manifold encodes the cognitive variable.
**Atom or composite:** Composite: record from N neurons → PCA/dimensionality reduction → analyze manifold geometry.
**Cost model:** PCA: O(N·T²) for T time points, N neurons. jPCA (Johnson et al.) is specific for dynamical systems.
**Real wall?** Yes — neural manifolds are not static; they change with brain state (attention, learning, task demands).
**Cross-domain wiring:** Neural manifold = latent space of neural population = the same as latent space in VAEs. In retrieval: semantic manifold.
**Notes:** jPCA (Johnston et al., 2016) finds rotational dynamics in motor cortex population activity — the brain uses rotation for motor planning.

*Last updated: 2026-06-25 (expanded with computational neuroscience)*
*Source doctrine: The Painted Fence — Jesse*
