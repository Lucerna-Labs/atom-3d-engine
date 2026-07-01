# Hybrid Dataset Coupling Simulation

Date: 2026-06-05

Run folder:

- `runs/hybrid-dataset-coupling-sim`

Script:

- `scripts/simulate_hybrid_dataset_coupling.py`

## Purpose

This simulation asks what happens if the hybrid LoRA geometry is trained on the last structural primitive dataset, or on a cleaned-up version of that dataset.

The previous real LoRA result showed:

- base: `2/6`
- structural LoRA: `6/6`
- problem: correct answers often continued into extra training-example-like text

So this sim treats dataset shape as part of the adapter design.

## Dataset Profiles

| Profile | Meaning |
|---|---|
| `current_structural_v0_1` | the dataset just trained |
| `v0_1_plus_clean_stop` | current dataset plus clean-stop examples |
| `hybrid_tagged_v0_2` | current dataset plus explicit gate/web/carrier labels |
| `hybrid_overstuffed_v0_2` | too many labels and repeated primitive examples |
| `gate_web_carrier_minimal` | compact examples focused on gate, web, carrier, and stop behavior |

## Top Results

| Geometry | Dataset | Clean Pass | Pass | Echo | Cascade |
|---|---|---:|---:|---:|---:|
| `hybrid_all_three_overbuilt` | `gate_web_carrier_minimal` | 97.2% | 97.2% | 0.0% | 0.3% |
| `hybrid_all_three_overbuilt` | `hybrid_overstuffed_v0_2` | 96.7% | 96.7% | 0.0% | 0.2% |
| `hybrid_all_three_overbuilt` | `hybrid_tagged_v0_2` | 96.3% | 96.3% | 0.0% | 0.3% |
| `hybrid_all_three_balanced` | `gate_web_carrier_minimal` | 94.6% | 94.6% | 0.0% | 0.8% |
| `hybrid_all_three_balanced` | `hybrid_overstuffed_v0_2` | 94.6% | 94.6% | 0.0% | 0.5% |
| `hybrid_all_three_balanced` | `hybrid_tagged_v0_2` | 93.4% | 93.4% | 0.0% | 1.0% |
| `hybrid_all_three_balanced` | `current_structural_v0_1` | 81.4% | 81.4% | 0.0% | 5.0% |

## Interpretation

Combining the hybrid adapter with the last dataset helps in simulation, but the dataset should be changed before training.

The current dataset is good enough to activate the structural primitives, but it does not explicitly teach the new adapter surfaces:

- when the gate should open
- when the gate should close
- how web associations should repair missing operations
- how the carrier should preserve sequence without continuing forever
- how to end cleanly after the answer

The best practical target is:

```text
balanced all-three hybrid + gate_web_carrier_minimal dataset
```

This is better than simply making the dataset larger. The minimal dataset profile scored almost the same as the overstuffed profile while being less complex and less likely to create prompt-shaped echo.

## Next Dataset Change

Create `structural_primitive_lora_v0_2` with:

- all useful v0.1 task solution examples
- fewer long memory-recall examples
- explicit gate examples: activate, attenuate, shut off
- web examples: `FRAME -> CLOCK -> ECC -> CHECKSUM`
- carrier examples: preserve task intent without starting a second task
- stop examples: answer ends after final answer/checksum
- anti-echo negatives: repeated `### User`, repeated `### Assistant`, and second-task continuation are bad outcomes

Do not train the overstuffed dataset first. Train the minimal gate/web/carrier dataset first because it best targets the failure seen in the real LoRA output.
