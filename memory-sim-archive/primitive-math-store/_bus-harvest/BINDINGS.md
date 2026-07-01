# Binding ("glue") Primitive Ranking

**Date:** 2026-06-28  
**Primitives scanned:** 5973  
**Primitives that bridge ≥2 domains:** 3170 (53.1%)  
**Distinct domain pairs observed:** 309  
**Domains:** 31

## Score
`binding_power = ( Σ over each bridged pair {A,B} of 1/pair_count[A,B] ) × reach_breadth × real_wall_bonus`  
Rare pairs (few primitives connect A↔B) dominate. `real_wall_bonus = 1.5` if the primitive names a conserved currency, else 1.0.

## Top 30 binding primitives

| Rank | Score | Prim | Home domain | Reach | Rarest bridge (A↔B : #shared) | Real wall? |
|---:|---:|---|---|---|---|:--:|
| 1 | 23.4914 | `PRIM-001` [PRIM-001] photon-arrival | networking | cognitive-primitives, networking, photonics-optics, quantum-computing, signal-processing-rf | cognitive-primitives ↔ networking : 1 | yes |
| 2 | 19.1373 | `chomp-smooth` chomp-smooth | control-numerical-opt | computational-geometry, control-numerical-opt, linear-algebra-matrix, ml-training, retrieval-search | computational-geometry ↔ linear-algebra-matrix : 1 | yes |
| 3 | 10.7765 | `MAC-capacity` MAC-capacity | information-theory-coding | computational-geometry, information-theory-coding, quantum-computing, retrieval-search | computational-geometry ↔ information-theory-coding : 1 | yes |
| 4 | 10.3764 | `gate-scheduling` gate-scheduling | quantum-computing | causal-inference, quantum-computing, signal-processing-rf, type-theory-programming-languages | causal-inference ↔ type-theory-programming-languages : 2 | yes |
| 5 | 10.1773 | `digital-signature` digital-signature | cryptography-hashing | cryptography-advanced, cryptography-hashing, distributed-systems, retrieval-search | cryptography-advanced ↔ distributed-systems : 1 | yes |
| 6 | 9.2647 | `spatial-accel` spatial-accel | graphics-rendering-lod | computational-geometry, database-streaming-sketching, graphics-rendering-lod | computational-geometry ↔ database-streaming-sketching : 1 | yes |
| 7 | 9.1987 | `atomic-rule` atomic-rule | decision-logic | agentic-reasoning, cognitive-primitives, decision-logic, operating-systems | cognitive-primitives ↔ operating-systems : 1 | yes |
| 8 | 9.1364 | `authenticated-encrypt` authenticated-encrypt | cryptography-hashing | cryptography-hashing, networking, retrieval-search | cryptography-hashing ↔ networking : 1 | yes |
| 9 | 9.1216 | `PRIM-023` [PRIM-023] tla-specification | formal-verification | distributed-systems, formal-verification, logic-reasoning | distributed-systems ↔ formal-verification : 1 | yes |
| 10 | 9.1216 | `LR.189:-Algebraic-Decision-Diagram` LR.189: Algebraic Decision Diagram | logic-reasoning | formal-verification, information-theory-coding, logic-reasoning | formal-verification ↔ information-theory-coding : 1 | yes |
| 11 | 9.0804 | `last-scattering-surface` last-scattering-surface | astrophysics-cosmology | astrophysics-cosmology, cryptography-advanced, electromagnetics-antennas, photonics-optics | cryptography-advanced ↔ electromagnetics-antennas : 1 | yes |
| 12 | 8.6533 | `sample-approx` sample-approx | database-streaming-sketching | control-numerical-opt, database-streaming-sketching, ml-training, statistics-probability | control-numerical-opt ↔ database-streaming-sketching : 1 | yes |
| 13 | 8.2494 | `reflect` reflect | graphics-rendering-lod | graphics-rendering-lod, linear-algebra-matrix, photonics-optics, retrieval-search | linear-algebra-matrix ↔ photonics-optics : 1 | yes |
| 14 | 8.2027 | `anomaly-detect-stream` anomaly-detect-stream | database-streaming-sketching | database-streaming-sketching, distributed-systems, retrieval-search, signal-processing-rf, statistics-probability | distributed-systems ↔ signal-processing-rf : 3 | yes |
| 15 | 8.1547 | `biased-sampler` biased-sampler | database-streaming-sketching | causal-inference, database-streaming-sketching, retrieval-search, statistics-probability | causal-inference ↔ database-streaming-sketching : 1 | yes |
| 16 | 8.1122 | `fold` fold | retrieval-search | cryptography-hashing, information-theory-coding, retrieval-search, signal-processing-rf | cryptography-hashing ↔ information-theory-coding : 1 | yes |
| 17 | 8.0916 | `PRIM-069` [PRIM-069] concept-induction | agentic-reasoning | agentic-reasoning, causal-inference, retrieval-search, type-theory-programming-languages | causal-inference ↔ type-theory-programming-languages : 2 | yes |
| 18 | 7.875 | `capacity-with-feedback` capacity-with-feedback | information-theory-coding | causal-inference, information-theory-coding, networking | causal-inference ↔ networking : 1 | yes |
| 19 | 7.7999 | `compare` compare | retrieval-search | cryptography-hashing, linear-algebra-matrix, retrieval-search, signal-processing-rf, statistics-probability | cryptography-hashing ↔ linear-algebra-matrix : 3 | yes |
| 20 | 7.2548 | `pomdp-runtime` pomdp-runtime | decision-logic | agentic-reasoning, decision-logic, queueing-theory-stochastic-processes, statistics-probability | agentic-reasoning ↔ queueing-theory-stochastic-processes : 1 | yes |
| 21 | 7.2467 | `compound-channel` compound-channel | information-theory-coding | cognitive-primitives, information-theory-coding, quantum-computing, statistics-probability | cognitive-primitives ↔ information-theory-coding : 2 | yes |
| 22 | 7.125 | `PRIM-034` [PRIM-034] planning-with-constraints | agentic-reasoning | agentic-reasoning, electromagnetics-antennas, formal-verification | agentic-reasoning ↔ formal-verification : 1 | yes |
| 23 | 7.0147 | `PRIM-094` [PRIM-094] sdn-data-plane-programming | networking | networking, signal-processing-rf, type-theory-programming-languages | networking ↔ type-theory-programming-languages : 1 | yes |
| 24 | 6.9868 | `protein-RMSD` protein-RMSD | biology-bioinformatics | biology-bioinformatics, linear-algebra-matrix, quantum-computing | biology-bioinformatics ↔ linear-algebra-matrix : 1 | yes |
| 25 | 6.8457 | `kramers-kronig` kramers-kronig | condensed-matter | causal-inference, condensed-matter, photonics-optics | causal-inference ↔ condensed-matter : 1 | yes |
| 26 | 6.7794 | `lspr` lspr | photonics-optics | electromagnetics-antennas, photonics-optics, type-theory-programming-languages | photonics-optics ↔ type-theory-programming-languages : 1 | yes |
| 27 | 6.7345 | `normal-map` normal-map | graphics-rendering-lod | control-numerical-opt, graphics-rendering-lod, ml-training, retrieval-search, signal-processing-rf | control-numerical-opt ↔ graphics-rendering-lod : 2 | yes |
| 28 | 6.1364 | `simhash` simhash | retrieval-search | cryptography-hashing, decision-logic, retrieval-search | cryptography-hashing ↔ decision-logic : 1 | yes |
| 29 | 6.1125 | `PRIM-001` [PRIM-001] chain-of-thought | agentic-reasoning | agentic-reasoning, logic-reasoning, retrieval-search | logic-reasoning ↔ retrieval-search : 1 | yes |
| 30 | 5.85 | `energy-aware-scheduling-eas` energy-aware-scheduling-eas | operating-systems | control-numerical-opt, operating-systems, signal-processing-rf | control-numerical-opt ↔ operating-systems : 1 | yes |

## What "binding" means here
A binding primitive is GLUE: it joins primitives or whole domains that normally would not compose. 
A primitive reaching only its home domain scores 0 (it bridges nothing). The top entries are the rare 
cross-domain bridges — the adapters that make distant fields compose. Validate the engine-relevant ones 
empirically (xdsim `--mode optimize`); a high binding score is a nomination, not a proof of utility.

---
*Generated by `find_bindings.py` (glue/binding finder). Not the ∀∃λ logical-binder analysis.*