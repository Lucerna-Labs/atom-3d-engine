# Hybrid LoRA Geometry Simulation

Date: 2026-06-05

Run folder:

- `runs/hybrid-lora-geometry-sim`

Script:

- `scripts/simulate_hybrid_lora_geometry.py`

## Purpose

The first structural primitive LoRA worked, moving the quick probe from `2/6` to `6/6`, but several outputs continued into extra training-example-like text after the correct answer.

That suggests standard additive LoRA can make the structural packet fire, but it does not know when to shut the packet off.

This simulation compares four adapter geometry ideas:

- additive LoRA: memory payload enters by addition
- gated LoRA: multiplicative gate controls strength
- Kronecker/web adapter: repeated association geometry
- Fourier/carrier adapter: global rhythm or signal carrier

It also tests pairwise hybrids and all-three hybrids.

## Metrics

The simulation tracks:

- `pass_rate`: task success even if the output echoes
- `clean_pass_rate`: task success with no echo and no overdrive
- `echo_rate`: continuation/scaffold echo after the answer
- `overdrive_rate`: too much structure becoming noise
- `cascade_rate`: missing steps causing failure cascades
- `repair_rate`: missing operation repair

`clean_pass_rate` is the most important metric because the current real LoRA already showed the echo problem.

## Refined Hybrid Results

Refined top hybrids used `6000` iterations per task/mode.

| Geometry | Clean Pass | Pass | Mean Score | Echo | Overdrive | Cascade | Repair |
|---|---:|---:|---:|---:|---:|---:|---:|
| `hybrid_all_three_overbuilt` | 85.99% | 86.68% | 0.925 | 0.97% | 0.00% | 2.56% | 0.420 |
| `hybrid_all_three_balanced` | 78.97% | 78.97% | 0.886 | 0.00% | 0.00% | 5.96% | 0.542 |
| `hybrid_no_add_payload` | 73.72% | 73.72% | 0.858 | 0.00% | 0.00% | 7.88% | 0.763 |
| `hybrid_all_three_light` | 64.82% | 65.58% | 0.820 | 1.87% | 0.00% | 11.88% | 0.624 |

## Full Sweep Highlights

Standard additive LoRA in the sim:

- clean pass: `6.58%`
- pass: `7.70%`
- echo: `31.77%`
- cascade: `51.28%`

Balanced all-three hybrid:

- clean pass: `79.31%`
- pass: `79.31%`
- echo: `0.00%`
- cascade: `5.92%`

Overbuilt all-three hybrid:

- clean pass: `85.65%`
- pass: `86.30%`
- echo: `0.95%`
- cascade: `2.78%`

## Interpretation

The hybrid idea is strongly supported in simulation.

The useful division of labor appears to be:

- additive path: carries structural primitive content
- multiplicative gate: controls activation and shutoff
- Kronecker/web path: preserves associations and supports repair
- Fourier/carrier path: stabilizes global sequence and packet receipt
- anti-echo objective: prevents scaffold continuation after the answer

The overbuilt hybrid wins the toy score, but that does not mean it should be the first real architecture. It has the highest training complexity and a small echo rate under echo-trap pressure.

The balanced hybrid is the best first engineering target:

- still large lift in simulation
- zero simulated echo
- simpler than overbuilt
- easier to ablate against the current standard LoRA

## Proposed Next Real Adapter

Build `Gated-Web-Carrier LoRA v1`:

```text
base = W0x
payload = AdditiveLoRA(x)
gate = sigmoid(GateLoRA(x))
web = WebAdapter(x)
carrier = CarrierAdapter(x)
output = base + gate * (payload + web + carrier)
```

Training objective:

- keep the current structural primitive SFT examples
- add explicit clean-stop examples
- add anti-echo negatives where repeated `### User`, `### Assistant`, and second-task continuations are penalized
- compare against standard LoRA on the same model, same data, same steps

First real comparison:

| Adapter | Purpose |
|---|---|
| standard LoRA | current baseline |
| gated LoRA | tests shutoff/gain control |
| gated + web | tests association repair |
| gated + web + carrier | tests full hybrid |

Do not train the overbuilt version first. Use it as a second-stage target after the balanced hybrid proves the operator change matters.
