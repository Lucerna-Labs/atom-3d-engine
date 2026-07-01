# Rarest Domain-Pair Bridges (structural glue gaps)

**Date:** 2026-06-28  
Pairs of domains joined by the FEWEST primitives. A bridge nobody else makes is the highest-leverage glue to extract or verify.

| Domain A | Domain B | # bridging prims | Bridging primitives |
|---|---|---:|---|
| logic-reasoning | retrieval-search | 1 | PRIM-001(agentic-reasoning) |
| agentic-reasoning | formal-verification | 1 | PRIM-034(agentic-reasoning) |
| astrophysics-cosmology | causal-inference | 1 | excision(astrophysics-cosmology) |
| astrophysics-cosmology | database-streaming-sketching | 1 | pseudo-cl(astrophysics-cosmology) |
| cryptography-advanced | electromagnetics-antennas | 1 | last-scattering-surface(astrophysics-cosmology) |
| biology-bioinformatics | linear-algebra-matrix | 1 | protein-RMSD(biology-bioinformatics) |
| biology-bioinformatics | causal-inference | 1 | PRIM-147(causal-inference) |
| causal-inference | queueing-theory-stochastic-processes | 1 | PRIM-188(causal-inference) |
| causal-inference | cryptography-advanced | 1 | PRIM-317(causal-inference) |
| combinatorial-optimization | queueing-theory-stochastic-processes | 1 | PRIM-126(combinatorial-optimization) |
| biology-bioinformatics | combinatorial-optimization | 1 | PRIM-149(combinatorial-optimization) |
| combinatorial-optimization | ml-training | 1 | PRIM-287(combinatorial-optimization) |
| combinatorial-optimization | distributed-systems | 1 | PRIM-325(combinatorial-optimization) |
| combinatorial-optimization | physics-diffusion | 1 | PRIM-331(combinatorial-optimization) |
| agentic-reasoning | combinatorial-optimization | 1 | PRIM-350(combinatorial-optimization) |
| biology-bioinformatics | computational-geometry | 1 | CG.014:-Voronoi-Diagram(computational-geometry) |
| condensed-matter | linear-algebra-matrix | 1 | exact-diagonalization(condensed-matter) |
| agentic-reasoning | condensed-matter | 1 | bosonization(condensed-matter) |
| causal-inference | condensed-matter | 1 | kramers-kronig(condensed-matter) |
| control-numerical-opt | distributed-systems | 1 | opt-admm(control-numerical-opt) |
| computational-geometry | linear-algebra-matrix | 1 | chomp-smooth(control-numerical-opt) |
| cryptography-advanced | networking | 1 | PRIM-041(cryptography-advanced) |
| cryptography-advanced | distributed-systems | 1 | digital-signature(cryptography-hashing) |
| cryptography-hashing | networking | 1 | authenticated-encrypt(cryptography-hashing) |
| networking | retrieval-search | 1 | authenticated-encrypt(cryptography-hashing) |
| cryptography-hashing | photonics-optics | 1 | fault-injection(cryptography-hashing) |
| cryptography-hashing | electromagnetics-antennas | 1 | em-sidechannel(cryptography-hashing) |
| control-numerical-opt | database-streaming-sketching | 1 | sample-approx(database-streaming-sketching) |
| causal-inference | database-streaming-sketching | 1 | biased-sampler(database-streaming-sketching) |
| cognitive-primitives | operating-systems | 1 | atomic-rule(decision-logic) |
| decision-logic | graphics-rendering-lod | 1 | heuristic-search-a-star(decision-logic) |
| agentic-reasoning | queueing-theory-stochastic-processes | 1 | pomdp-runtime(decision-logic) |
| distributed-systems | electromagnetics-antennas | 1 | cons-eventual(distributed-systems) |
| distributed-systems | linear-algebra-matrix | 1 | dist-map-reduce(distributed-systems) |
| formal-verification | queueing-theory-stochastic-processes | 1 | PRIM-015(formal-verification) |
| distributed-systems | formal-verification | 1 | PRIM-023(formal-verification) |
| distributed-systems | logic-reasoning | 1 | PRIM-023(formal-verification) |
| linear-algebra-matrix | photonics-optics | 1 | reflect(graphics-rendering-lod) |
| computational-geometry | statistics-probability | 1 | kd-tree-build(graphics-rendering-lod) |
| computational-geometry | database-streaming-sketching | 1 | spatial-accel(graphics-rendering-lod) |
| database-streaming-sketching | graphics-rendering-lod | 1 | spatial-accel(graphics-rendering-lod) |
| computational-geometry | information-theory-coding | 1 | MAC-capacity(information-theory-coding) |
| information-theory-coding | queueing-theory-stochastic-processes | 1 | cascade-entropy(information-theory-coding) |
| causal-inference | networking | 1 | capacity-with-feedback(information-theory-coding) |
| logic-reasoning | statistics-probability | 1 | LR.155:-Probabilistic-Logic-Programming(logic-reasoning) |
| formal-verification | information-theory-coding | 1 | LR.189:-Algebraic-Decision-Diagram(logic-reasoning) |
| information-theory-coding | logic-reasoning | 1 | LR.189:-Algebraic-Decision-Diagram(logic-reasoning) |
| control-numerical-opt | decision-logic | 1 | ftrl(ml-training) |
| cognitive-primitives | networking | 1 | PRIM-001(networking) |
| cognitive-primitives | photonics-optics | 1 | PRIM-001(networking) |
| astrophysics-cosmology | networking | 1 | PRIM-075(networking) |
| networking | type-theory-programming-languages | 1 | PRIM-094(networking) |
| control-numerical-opt | operating-systems | 1 | energy-aware-scheduling-eas(operating-systems) |
| causal-inference | operating-systems | 1 | hydra-consistency-policy-overlay(operating-systems) |
| photonics-optics | type-theory-programming-languages | 1 | lspr(photonics-optics) |
| cryptography-advanced | physics-diffusion | 1 | friedmann-equation(physics-diffusion) |
| control-numerical-opt | queueing-theory-stochastic-processes | 1 | QT.171:-Semi-Markov-Decision-Process(queueing-theory-stochastic-processes) |
| linear-algebra-matrix | queueing-theory-stochastic-processes | 1 | QT.351:-Multitype-Branching(queueing-theory-stochastic-processes) |
| cryptography-hashing | information-theory-coding | 1 | fold(retrieval-search) |
| cryptography-hashing | decision-logic | 1 | simhash(retrieval-search) |

---
*Generated by `find_bindings.py`.*