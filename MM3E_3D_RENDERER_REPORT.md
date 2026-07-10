# MM3E 3D Renderer Experiment Report

Date: 2026-07-02

This document is the renderer-side companion to the primitive simulation report. It focuses on the 3D renderer itself: what has been built, how it works, which simulation primitives already helped it, which ones did not survive renderer validation, and which rejected results are still worth keeping for the spiderweb bus, Redos kernel, inference engine, LLM model ecoskeletons, RF transport, networking, security, crypto, and false-memory research.

## Executive Summary

MM3E is a real-time 3D engine built from analytic math primitives instead of triangle meshes. The core is intentionally small and mechanical: `mm3e-kit` holds the dependency-free math, field, marching, shading, color, and framebuffer machinery; `mm3e-orchestrator` holds policy such as scene graph traversal, lighting, reflections, fog, global illumination, post-processing, render modes, animation, and scene I/O. The optional `mm3e-gpu` crate quarantines the GPU dependency stack and compiles fixed SDF scenes into WGSL compute shaders.

The renderer is not a clone of Unreal, Unity, or Godot. It is the 3D elevation of the older MMPE idea: a field-first renderer where geometry is expressed as signed distance fields, rays step through the field, surfaces are shaded analytically, and the whole frame emerges from a small set of repeated mechanical operations.

The most useful renderer discoveries so far are:

- Clear-margin / subitize stepping: shipped as a guarded marcher optimization. It reduces field evaluations when the ray is clearly away from the surface, with bounded quality loss only inside a controlled range.
- Dual-number analytic normals: shipped where the scene is marked dual-safe. It avoids expensive finite-difference normal sampling and gives exact analytic gradients for supported primitives and operators.
- Guarded over-relaxation and secant correction: useful, but only within conservative limits. Higher TPU-discovered values looked strong in abstract simulations and then degraded real renderer behavior.
- GPU codegen: not a Kaggle primitive by itself, but it is the most important renderer execution primitive. The scene is compiled into a WGSL `map(point)` field function and dispatched one invocation per pixel.

The main lesson is that renderer validation compresses the hype out of raw primitive search. A primitive that looks powerful in a toy or TPU simulation may still fail under real camera rays, depth error, silhouettes, normal stability, material lookup, and post-processing. That does not make the result worthless. It means each primitive needs to be classified by domain instead of simply accepted or discarded.

## Current Renderer Progress

The renderer currently has these major pieces in place:

- Analytic SDF geometry for spheres, boxes, rounded boxes, torus, cylinder, capsule, cone, ellipsoid, octahedron, hex prism, and plane.
- Constructive solid geometry: union, intersection, subtraction, plus smooth variants.
- Domain operators: round, onion, elongate, infinite repeat, twist, bend, and mirror.
- A CPU renderer built around SDF ray marching / sphere tracing.
- A GPU renderer path in `mm3e-gpu` using WGSL compute via `wgpu`.
- Cook-Torrance GGX-style physically based shading.
- Directional, point, area, and sphere light support.
- Soft shadows, ambient occlusion, distance fog, recursive mirror reflections, and gradient normals.
- HDR color flow with bloom, exposure, ACES-style tone mapping, gamma conversion, and BMP output.
- AOV output modes such as normal, depth, AO, albedo, and step count.
- Keyframe animation and a `.mm3e` scene format.
- A raw Win32/GDI interactive viewer path.
- Multithreaded CPU rendering and a fixed-scene GPU compute path.

The `mm3e-gpu` README records a GPU backend architecture where the engine walks the scene, emits WGSL for a `map(p) -> (distance, material_id)` function, uploads a camera uniform, and dispatches a compute shader across the output image. In prior local probing, the GPU path selected the NVIDIA RTX 5070 Ti through Vulkan. If the goal is explicitly to validate the renderer on the Intel Arc GPU, the next concrete implementation step is adapter selection in the GPU backend or example harness so Arc can be chosen deliberately instead of accepting the first high-priority discrete adapter.

## How The Renderer Works

The renderer turns a scene into a world-field query:

1. The camera generates one ray per sample.
2. The ray marcher asks the scene field for the signed distance at the current point.
3. The distance tells the ray how far it can safely advance.
4. When the distance falls below the hit epsilon, the marcher records a surface hit.
5. The renderer estimates or analytically computes the surface normal.
6. The shader combines material, lights, shadows, ambient terms, reflections, fog, and GI.
7. The frame is accumulated in linear HDR color.
8. Post-processing applies bloom, exposure, tone mapping, gamma, and output conversion.

For the GPU path, the same idea is compiled into shader form. A fixed scene becomes a WGSL function. The compute shader runs each pixel independently, making the renderer a good match for GPU hardware because most work is embarrassingly parallel. The CPU path is still important because it is easier to validate, easier to debug, and preserves the standard-library-only core doctrine.

## The Eight Root Atoms In Renderer Terms

MM3E keeps mapping complex behavior back onto eight mechanical atoms:

- `scan`: walk pixels, rays, samples, scene objects, and march steps.
- `hash`: generate deterministic jitter, sampling choices, and repeatable stochastic variation.
- `fold`: accumulate ray steps, lighting, AO, GI, bloom, and frame reductions.
- `project`: turn camera coordinates into rays, surface positions into normals, and 3D fields into 2D images.
- `scale`: apply transforms, step factors, material parameters, exposure, and tone mapping.
- `compare`: choose nearest SDF distance, detect hits, sort depth, clamp errors, and select CSG branches.
- `combine`: compose primitives, materials, lighting terms, reflections, fog, and post effects.
- `order`: enforce pipeline order, pass order, scene traversal order, and deterministic render sequencing.

This matters because it keeps the renderer aligned with the broader engine doctrine. The 3D renderer is not just a pile of graphics features. It is a mechanical proof that the same small primitive vocabulary can produce a complete visual system.

## Primitive Inventory

### Geometry Primitives

Currently useful and applied directly in the renderer:

- Sphere
- Box
- Rounded box
- Torus
- Cylinder
- Capsule
- Cone
- Ellipsoid
- Octahedron
- Hexagonal prism
- Plane

These are not mesh assets. Each one is a distance function. That means collision, marching, normals, AO, and shading can all query the same underlying field.

### Composition Primitives

Applied:

- Union
- Intersection
- Subtraction
- Smooth union
- Smooth intersection
- Smooth subtraction

These are renderer-native examples of `compare` plus `combine`: compare distances, choose or blend surfaces, and preserve material identity.

### Domain Operators

Applied:

- Round
- Onion / shell
- Elongate
- Infinite repeat
- Twist
- Bend
- Mirror

These are especially important because they multiply the expressive power of a small primitive set. A small number of base fields can produce much richer visual structure once domain deformation is included.

### Shading And Output Primitives

Applied:

- Gradient / analytic normals
- Soft shadow rays
- Ambient occlusion sampling
- GGX-style specular response
- Reflection recursion
- Fog integration
- HDR accumulation
- Bloom extraction and blur
- Tone mapping
- AOV channel output

These are the visual equivalents of the engine atoms. They turn pure field hits into understandable rendered images.

## Simulation Discoveries Applied To The Renderer

### Clear-Margin / Subitize Stepping

Status: applied.

The useful version is the guarded version, not the raw version. The marcher can sometimes take a slightly larger step when it is clearly far from the surface and the margin is safe. This reduces field evaluations in open space. The reported validated range showed useful savings around conservative subitize values, while aggressive values quickly created depth error.

Renderer meaning:

- Good for open-space acceleration.
- Good for scenes where the field is smooth and distances are trustworthy.
- Risky near silhouettes, thin objects, high curvature, or domain-deformed geometry.
- Needs a real renderer error metric, not just "fewer steps."

Current classification: keep as renderer optimization, but gated.

### Dual-Number Analytic Normals

Status: applied where dual-safe.

Finite-difference normals require multiple neighboring field samples. Dual-number evaluation carries value and derivative through the field calculation so supported scenes can produce analytic normals more directly. This is one of the cleanest wins because it improves both speed and correctness for compatible fields.

Renderer meaning:

- Excellent match for analytic SDFs.
- Reduces normal-sampling cost.
- Improves stability where the dual implementation covers the primitive/operator chain.
- Must fall back when the scene includes non-dual-safe features.

Current classification: keep and expand coverage.

### Guarded Over-Relaxation

Status: partly applied / conservative.

Over-relaxation can reduce march steps by stepping farther than the raw SDF distance. The renderer already benefits from careful step policy, but the TPU-favored aggressive omega values did not transfer cleanly. A higher value can lower step count while increasing depth error, missed details, or unstable silhouettes.

Renderer meaning:

- Conservative omega values are useful.
- Higher search-discovered values are not renderer defaults.
- Any tuning must be checked against depth error and image quality.

Current classification: keep conservative baseline; reject aggressive default.

### Secant Correction

Status: useful as a local refinement concept.

Secant-style correction can improve hit location after a bracket or near-surface transition exists. It is more useful as a refinement stage than as a broad march policy.

Renderer meaning:

- Good near the hit point.
- Useful for reducing surface placement error.
- Not a replacement for safe field traversal.

Current classification: keep as refinement primitive.

## TPU / Kaggle Primitive Results That Did Not Transfer Directly

The following are still worth recording because "not renderer-safe" is not the same as "worthless."

### TPU Omega 1.70

Renderer result: rejected as a default.

The TPU search favored a stronger over-relaxation value, but real renderer validation showed worse behavior than the existing shipped baseline. It increased evaluation cost in the real harness instead of improving it.

Useful elsewhere:

- May still help non-visual traversal problems.
- Could be useful in transport scans where the cost of overshoot is recoverable.
- Could be tested in inference search spaces where approximate early exploration is acceptable.

### Raw Radius Formula: `(radius + i * 0.02) / 0.588`

Renderer result: rejected as-is.

This kind of formula showed impressive evaluation reduction in primitive simulation, but depth error was too high for renderer output. A renderer cannot simply accept a faster march if surface placement becomes visibly wrong.

Useful elsewhere:

- Candidate for coarse prepass search.
- Candidate for RF packet recovery windows where approximate region discovery precedes exact repair.
- Candidate for spiderweb bus path probing where the result can be verified downstream.

### Raw Overlap Formula: `overlap + t * 0.1`

Renderer result: rejected as-is.

This was one of the more aggressive reduction signals, but it created too much error for direct image formation.

Useful elsewhere:

- Interesting for self-repair windows.
- Interesting for network retransmission prediction.
- Interesting for security anomaly clustering, where loose overlap may find candidate regions before strict verification.

### Stochastic Omega Variants

Renderer result: low priority.

Stochastic tuning did not produce a strong enough renderer win compared with the simpler guarded policies. It may still have use as a sampling diversity mechanism, but not as a primary march optimizer.

Useful elsewhere:

- Search diversification.
- False-memory detection experiments, where controlled stochastic disagreement may reveal unstable recall paths.
- LLM ecoskeleton routing, where stochastic probes can expose brittle routing choices.

### Binary Refine / Glue Results

Renderer result: no major contribution so far.

The binary refine and glue-style signals did not appear to meaningfully improve the renderer's main loop. They may still be useful in boundary repair or packet assembly domains.

Useful elsewhere:

- RF transport repair.
- Bus packet reassembly.
- Security event stitching.
- Inference cache reconciliation.

## Fascinating Findings

The strongest lesson is that real renderer validation is harsher than primitive simulation. A field marcher is sensitive to small geometric errors because image output makes those errors visible. A primitive can reduce math work and still be wrong if it bends surface depth, normals, silhouettes, or material selection.

The second important finding is that failure modes have structure. Aggressive step formulas fail in ways that look similar to transport overshoot: they move too fast through a domain and then need a later repair or verification stage. That makes them poor final-render policies but promising search or recovery policies for networking, RF, crypto-adjacent authentication flows, and spiderweb bus routing.

The third finding is that dual normals are more than a renderer trick. They suggest a broader pattern: carry value plus derivative or certainty metadata through a computation instead of recomputing it later from nearby samples. That pattern may apply to inference traces, false-memory detection, bus route confidence, and kernel scheduling.

The fourth finding is that the GPU renderer validates the architecture. The renderer's core field query can be compiled into a compact shader. That means the MM3E primitive language is not just conceptual. It can cross execution targets: CPU Rust, GPU WGSL, and potentially future TPU-style experimentation.

## Relevance Outside The Renderer

### Spiderweb Bus

Useful transfers:

- Clear-margin stepping maps to route confidence: move quickly through known-safe spans, slow down near intersections.
- Overlap formulas map to candidate path stitching, but only with verification.
- Rejected renderer formulas may still work for exploratory routing because bus paths can be checked after traversal.

### Redos Kernel

Useful transfers:

- Dual metadata can help carry scheduling confidence, locality, or dependency gradients.
- Conservative over-relaxation maps to speculative execution only when rollback or validation exists.
- The renderer's strict pass ordering is a useful model for deterministic kernel stages.

### Inference Engine

Useful transfers:

- Subitize-like early exits can skip low-value inference work when confidence margins are wide.
- Dual-value thinking can carry score plus sensitivity through a model pipeline.
- AOV-style debug outputs map to inference introspection channels.

### LLM Model Ecoskeletons

Useful transfers:

- Scene graph to field compilation resembles model-route to execution-plan compilation.
- Stochastic disagreement may help find brittle memory or routing states.
- Failed fast paths can still become candidate generators before verifier passes.

### RF Transport And Self-Repair

Useful transfers:

- Overlap signals are more promising here than in final rendering.
- Packet repair can tolerate approximate candidate windows when followed by CRC, HMAC, parity, or replay checks.
- The "fast coarse pass, exact repair pass" pattern fits RF much better than visual surface marching.

### Networking And Security

Useful transfers:

- Clear-margin classification can reduce scan cost over trusted spans.
- Aggressive overlap can cluster suspicious event windows for later strict verification.
- The renderer's depth-error lesson maps directly to security: speed is not useful if it increases false trust.

### False Memories

Useful transfers:

- Stochastic replay and disagreement can expose unstable recall paths.
- Dual metadata can carry confidence and sensitivity through memory reconstruction.
- AOV-style outputs can show why a memory path was accepted, rejected, or repaired.

## Immediate Next Steps For The 3D Renderer

1. Add explicit GPU adapter selection to `mm3e-gpu` examples or backend configuration so Intel Arc can be selected intentionally.
2. Re-run GPU probe and render examples on Intel Arc, not only the default selected discrete adapter.
3. Build a renderer validation harness that compares baseline, subitize, omega, secant, and dual-normal configurations on the same scenes.
4. Track image quality metrics alongside field evaluations: depth error, normal error, missed-hit count, silhouette difference, and material mismatch.
5. Keep rejected primitive signals in a cross-domain registry instead of deleting them.
6. Add a transfer table mapping every primitive to renderer, bus, kernel, inference, RF, networking, security, crypto, and false-memory usefulness.
7. Extend WGSL codegen coverage only after CPU validation shows the primitive is renderer-safe.

## Practical Commands

Useful local checks:

```powershell
cargo test
cargo run -p mm3e-orchestrator --example spheres --release
cargo run -p mm3e-orchestrator --example stoch_subitize_real --release
cargo run -p mm3e-orchestrator --example dual_normal_real --release
cargo run -p mm3e-orchestrator --example tpu_signals_real --release
cargo run -p mm3e-gpu --example gpu_probe --release
cargo run -p mm3e-gpu --example gpu_render --release
```

For the Intel Arc goal, the `gpu_probe` result should explicitly identify the Arc adapter before renderer numbers are treated as Arc results.

## Current Classification Table

| Primitive / Signal | Renderer Status | Why | Transfer Status |
| --- | --- | --- | --- |
| SDF geometry primitives | Applied | Core scene representation | Useful everywhere as field vocabulary |
| CSG composition | Applied | Native distance compare/combine | Useful for bus and inference composition |
| Domain operators | Applied | Expands expressive power | Useful for route deformation and search spaces |
| Clear-margin / subitize | Applied, guarded | Saves evaluations in safe open space | Strong bus/network scan candidate |
| Dual analytic normals | Applied when dual-safe | Faster and more exact normals | Strong inference/kernel metadata pattern |
| Conservative over-relaxation | Applied cautiously | Can reduce steps but risks misses | Useful with validation/rollback |
| TPU omega 1.70 | Rejected as renderer default | Worse in real renderer validation | Possible exploratory search primitive |
| Raw radius formula | Rejected as-is | Too much depth error | Coarse prepass / RF window candidate |
| Raw overlap formula | Rejected as-is | Too much depth error | RF repair / event clustering candidate |
| Stochastic omega | Low priority | Weak renderer benefit | Useful for disagreement testing |
| Binary refine / glue | Not useful yet | No major renderer gain | Useful for packet and event stitching |

## Bottom Line

The 3D renderer is one of the best proving grounds for the primitive doctrine because it punishes sloppy math immediately. It has already absorbed the strongest safe results: guarded subitize stepping, analytic dual normals, conservative step policy, and GPU field compilation. The aggressive TPU signals should not be forced into the renderer as defaults, but they should absolutely be preserved. Their failure modes look valuable for RF repair, network/security clustering, bus route probing, crypto-adjacent verification workflows, inference fast paths, and false-memory research.

