# Primitive Self-Reinforcement Simulation

Date: 2026-06-04

Run folder:

- `runs/q2-primitive-self-reinforcement`

Script:

- `scripts/simulate_q2_primitive_self_reinforcement.py`

## Purpose

This simulation tests Jesse's "reinforce the primitives themselves" idea before LoRA training.

The prior primitive simulations mostly asked whether functional primitives help a damaged Q2-like model. This run asks a lower-level question:

Can a structural primitive layer make the functional primitives survive noise, drift, cascade, and overactivation?

This is a toy simulation, not real model evidence. It is useful for choosing what to train into a LoRA next.

## Setup

Quantization damage is fixed to the existing `damaged_q2` profile.

Functional payload is fixed:

- `EXTRACT_GIVENS`
- `SOURCE_BOUNDARY`
- `MAP_RELATION`
- `UNIT_TRACK`
- `STEP_COMPUTE`
- `VERIFY_RESULT`
- `COMPARE_AXES`
- `UNCERTAINTY_BOUND`
- `ORIGINAL_TASK_RETURN`
- `DANGER_CHECK`

The sweep varies structural primitives:

- `FRAME`
- `CLOCK`
- `CARRIER`
- `ROUTER`
- `PARITY`
- `CHECKSUM`
- `ECC`
- `REDUNDANCY`
- `SUPPRESSION`
- `COMPRESSION`
- `STATE_BUFFER`
- `GAIN_CLAMP`

The simulation gives these structural primitives their own failure and repair behavior. That means `ECC`, `CHECKSUM`, `FRAME`, and similar mechanisms have to survive before they can help the functional primitives.

## Refined Results

The top candidates were refined with 420 iterations per task.

| Condition | Pass | Worst Task | Mean Score | Structural Survival | Packet Rx | Cascade |
|---|---:|---:|---:|---:|---:|---:|
| No structural layer, `rag_like` | 28.4% | 23.1% | 0.528 | n/a | 42.9% | 28.7% |
| No structural layer, `kv_packet` | 49.7% | 45.7% | 0.688 | n/a | 45.5% | 21.8% |
| No structural layer, `lora_candidate` | 52.0% | 45.5% | 0.715 | n/a | 48.1% | 22.2% |
| Best binary reinforced layer, `lora_candidate` | 87.1% | 85.5% | 0.887 | 86.2% | 75.5% | 0.1% |
| Best repeat-reinforced layer, `lora_candidate` | 89.3% | 87.8% | 0.895 | 65.5% | 78.7% | 0.02% |

Best refined combo:

```text
FRAME
CLOCK
CARRIER
PARITY
CHECKSUM
ECC
REDUNDANCY
SUPPRESSION
COMPRESSION
STATE_BUFFER
GAIN_CLAMP
```

Best repeat-reinforced combo:

```text
FRAME
CLOCK
CARRIER
PARITY
CHECKSUM
ECC
REDUNDANCY
SUPPRESSION
COMPRESSION
STATE_BUFFER
GAIN_CLAMP
FRAME
CLOCK
ECC
GAIN_CLAMP
```

Compared with the refined no-structure `lora_candidate` baseline, the best structural layer gives:

- pass rate: `52.0% -> 89.3%`
- lift: `+37.3 points`
- worst task: `45.5% -> 87.8%`
- cascade: `22.2% -> 0.02%`
- packet receipt: `48.1% -> 78.7%`

## Mechanism Finding

Average mechanism lift from the broad exploratory sweep:

| Primitive | Pattern |
|---|---|
| `FRAME` | strongest average lift across signal levels |
| `CLOCK` | second strongest; especially useful against cascade |
| `CARRIER` | improves activation strength, but should be paired with gain control |
| `STATE_BUFFER` | strong lift; helps keep givens and task state alive |
| `REDUNDANCY` | useful, especially at low signal |
| `GAIN_CLAMP` | useful for avoiding overdrive damage |
| `CHECKSUM` | modest average lift but important in top combos |
| `ECC` | modest average lift alone, stronger when paired with parity/checksum |
| `PARITY` | weak alone, useful in the best full combo |
| `COMPRESSION` | small lift; mainly controls overhead |
| `SUPPRESSION` | important in top combos, even when average lift is not huge |
| `ROUTER` | negative average lift in this sim |

`ROUTER` is the surprising one. The likely reason is that routing creates a second failure surface. If routing fails, the packet can become coherently wrong. For LoRA, train router memories separately or leave routing to retrieval until it is reliable.

Repeat reinforcement helped, but only when targeted. Repeating `FRAME`, `CLOCK`, `ECC`, and `GAIN_CLAMP` gave the best stable result. Repeating too many primitives increases overhead.

## LoRA Implication

The next LoRA should not only train functional memories like "extract givens" and "verify result."

It should also train structural memories that make those operations stable:

- framing memories: I recognized the boundary between task, source, and distractor before acting.
- clock memories: I kept a steady sequence and did not skip ahead.
- carrier memories: I kept one task-intent active through the whole answer.
- parity/checksum memories: I checked whether the active operations matched the original task.
- ECC memories: I noticed a missing operation and repaired it before finalizing.
- redundancy memories: I carried the same primitive through multiple equivalent cues.
- suppression memories: I felt the pull of irrelevant text and kept it outside the working path.
- compression memories: I kept the packet short enough that it did not become noise.
- state-buffer memories: I held givens and intermediate values until the final check.
- gain-clamp memories: I avoided overdriving the model with too much structure.

The LoRA target should be a compact structural packet, not a giant context dump.

## Caution

This run supports the architecture direction, not a performance claim.

The real test is whether a LoRA trained on these structural primitives improves a compressed model against an instruct baseline and against the same compressed model without the structural LoRA.
