# MM3E Roadmap — what it takes to be competitive with industry 3-D engines

> Honest framing: **MM3E today is a pure-math, zero-dependency, CPU SDF raymarcher that writes
> offline BMP frames** — elegant, and roughly **three orders of magnitude** below an interactive
> engine. Unreal/Unity/Godot/Bevy are GPU-first mesh rasterizers with decades of work behind
> them (hardware pipelines, asset ecosystems, GI, PBR, physics, editors, ECS, networking). MM3E
> has **none of the platform layer**: no GPU, no window, no event loop, no input, no audio, no
> meshes, no asset import, no retained/queryable scene, no ECS, no serialization, no editor.

## Implemented so far (from this roadmap)

These shipped already — all **doctrine-preserving** (zero-dep, CPU, SDF, offline), no constraint broken:

- ✅ **Tier 0 — Cook-Torrance GGX metallic-roughness BRDF** replacing Blinn-Phong (`mm3e-kit/src/shade.rs`
  `brdf()` + `Material.metallic`/`specular`-as-reflectance). The empirical→PBR jump.
- ✅ **Tier 0 — Conservative bounding-sphere pruning** of the world field (`Prim::local_bound` /
  `Object::world_bound`; bounded fold in `Scene::world`). O(1) lower-bound early-out; ~2× on the showcase
  scene and the substrate a BVH will sit on.
- ✅ **Tier 0/1 — Linear-HDR scene-color target + post pass** (`mm3e-orchestrator/src/post.rs`): bloom
  (bright-pass + separable Gaussian), exposure, ACES, gamma — tone-map deferred out of the per-pixel path.

Still open and highest-leverage next: **BVH + cascaded SDFGI**, then the **data layer** (scene format +
march-step heatmap + profiler). See the tiers below.

## The strategic fork (pick one)

**Fork A — best-in-class real-time SDF / raymarch renderer + SDF-native physics.** *(Recommended.)*
Lineage: Media Molecule's *Dreams*, *Claybook*, Inigo Quilez / Shadertoy, MagicaCSG. You concede
**CPU-only** and **offline** (add a GPU backend), but the rest of the doctrine flips from a *cap*
into a **moat**:
- **SDFGI is free** where Godot must voxelize/distance-field meshes to fake it — MM3E already has
  the global `Fn(Vec3)->Field`.
- **Soft shadows / AO are exact analytic cone traces**, not stochastic approximations.
- **The world field *is* the collision oracle** that mesh engines spend GJK/EPA/BVHs approximating
  — distance, contact normal, and penetration depth are closed-form everywhere via
  `Marcher::normal` + the safe-step march.
- **SDF skinning bends topology-free solids** no mesh skinner can match.
- **Triplanar mapping** is the native answer to the SDF-UV problem.

**Fork B — general game engine.** Requires relaxing the core constraints *in sequence*: GPU-first
(break CPU-only + zero-dep) → triangle meshes + glTF/OBJ (break SDF-only) → real-time loop (break
offline) → ECS + editor + asset pipeline → networking. Every step erases what makes MM3E
distinctive; the endpoint is "a worse Bevy that took years to reach parity Bevy already has."

**Recommendation:** Commit to **Fork A**. Keep the pure-CPU path as the doctrine reference /
ground-truth renderer; add a wgpu/WGSL backend as the real-time path. Pursue Fork-B features
(meshes, ECS) *only* where they serve the SDF niche (e.g. mesh→SDF bake), not where they abandon it.

## The five hard blockers

1. **Geometry-as-a-Rust-closure** (`field: &dyn Fn(Vec3)->Field`). MM3E's signature move is exactly
   what a GPU cannot consume — closures don't cross the CPU→GPU boundary. Going real-time isn't a
   *port*, it's a **re-expression** of the world field as generated shader **source** or an
   interpreted instruction **buffer** — a second implementation of the kit, kept in sync.
2. **O(N) field over *all* objects at *every* sample.** `world()` folds over `self.objects` per
   point, ~160×/march + 4×/normal + ≤64×/shadow + 5×/AO + AA² + bounces. Zero acceleration → caps
   scenes at a handful of objects. (A BVH fixes this *and* is the substrate mesh ray tracing needs.)
3. **Per-pixel tonemap destroys the HDR frame.** `shade_pixel` applies ACES+gamma and stores a
   *clamped* `Rgba`; no float scene-color target survives. Every full-frame post effect (bloom,
   exposure, DoF, grading) is impossible until this is refactored. The depth/normal a G-buffer needs
   are already in `Hit` — and discarded.
4. **Statelessness.** The whole codebase rebuilds the world each `render()` with no concept of time,
   persistent state, or a frame loop. Animation, physics, and anything interactive need that axis.
5. **Zero-dep vs the OS.** Opening any GPU device, window, audio stream, or web canvas needs OS code
   `std` doesn't provide. This is the constraint with the worst effort-to-benefit ratio to preserve;
   it (and CPU-only) are the *only two* of the four defining constraints that must be conceded to go
   real-time. SDF-only and offline are **assets, not liabilities**.

## Tiered roadmap

### Tier 0 — free wins on the existing offline CPU path (break nothing; do first)
- **Conservative per-object bounds + bounded-fold pruning** — `bound()` per `Prim` (kit), prune in
  `world()`. *S.* Prerequisite for the BVH and any non-trivial scene.
- **Cook-Torrance GGX metallic-roughness BRDF** replacing Blinn-Phong in `shade.rs` (+`Material.metallic`).
  *M.* The line between a 1990s shader and a modern engine; every GI/IBL feature integrates against it.
- **Linear HDR scene-color target + G-buffer** (depth=`Hit.t`, normal already in `Hit`); defer
  tonemap to a post pass. *M.* Unblocks the entire post stack.
- **March-step heatmap / AOV debug modes** from the discarded `Hit.steps`. *S, nearly free.*
- **In-engine CPU profiler** over the existing thread bands (`std::time::Instant`). *S.*
- **Hand-rolled scene file format** + serde (data is already `Copy` plain-data). *M.* De-Rusts authoring.
- **mtime hot-reload + undo/redo snapshot stack** (`Object: Copy` makes undo trivial). *S.*

### Tier 1 — modern offline look + scalability (still pure-doctrine: zero-dep, CPU, SDF, offline)
- **BVH / loose-octree tree-fold** over object bounds. *L.* O(N)→O(log N).
- **Triplanar texturing + bilinear `Texture2D` + std-only PNG decoder** (inflate). *M+L.*
- **IBL from the analytic sky** (split-sum prefilter + irradiance + BRDF LUT). *L.* Turns the existing
  `sky()` into real environment lighting + glossy reflections.
- **LTC analytic area/rect lights** — noise-free, deterministic. *M.*
- **Cascaded SDFGI** — cone-trace the world field into an SH probe grid; replaces fake constant ambient
  with real multi-bounce color bleed. *L.* **The structural superpower.**
- **Refraction (IOR + Beer-Lambert, interior is free from SDF sign), clearcoat/anisotropy, mip/LOD.** *M each.*
- **Post stack on the new linear target**: bloom, auto-exposure, DoF (CoC from `Hit.t`), 3D-LUT grading,
  vignette/CA/grain/dither (fixes BMP banding). *S–M each.*
- **Offline Monte Carlo path tracer** (GGX-VNDF + MIS + SVGF denoise on the free G-buffer) as a
  ground-truth reference mode. *L.* (Introduces noise — keep it as *reference*, not the primary look.)

### Tier 2 — animation, physics & content ingestion (offline frame-sequence; one real SDF-only break)
- **Time as a first-class parameter + keyframe tracks + `Quat`.** *M.* The engine is one parameter from animating.
- **SDF-native skeletal deformation** via domain warping + morph-as-field-combine + IK. *L.* Mesh-free
  skinning of topology-free solids. (Caveat: warped/blended SDFs need step under-relaxation — Lipschitz.)
- **SDF-native collision layer** + capsule character controller + XPBD particles/cloth + conservative-
  advancement CCD. *M–L.* **The collision moat.**
- **Stateful `PhysicsWorld`** (semi-implicit Euler + contact manifolds + sequential-impulse/XPBD +
  sweep-and-prune). *M–L.* Runs as bake-trajectory → render-per-step. (FEM soft bodies are the one place
  the composition doctrine genuinely fails — a sparse solve doesn't fold into the 8 atoms; use
  shape-matching/XPBD instead.)
- **Triangle mesh primitive (Möller-Trumbore + BVH) + glTF/OBJ import.** *L+L.* **Breaks SDF-only** —
  the deliberate bridge to art content. Pair with **mesh→SDF baking** (narrow-band distance volume) to
  *restore* the doctrine: triangles re-expressed as a sampled `Field` flow through the existing
  tracer/CSG/GI unchanged. Add **SDF→mesh** (Marching Cubes / Dual Contouring) for the round trip.
- **Non-uniform affine `Mat4` + Lipschitz/singular-value distance correction.** *M.* Real refactor — the
  uniform-scale shortcut `scale * sdf(local)` in `lib.rs:90` is load-bearing.
- **Node-based SDF/material graph** (domain warp/repeat/twist/bend). *XL but doctrine-defining* —
  composition made explicit; the destined authoring tool for an SDF engine.

### Tier 3 — real-time GPU (concede CPU-only + zero-dep; the niche-defining leap)
- **wgpu device + swapchain + WGSL compute/fragment dispatch + uniform buffers** (new `mm3e-gpu`/`mm3e-app`
  crate). *L.* The actual 60 Hz loop. *Breaks zero-dep + CPU-only + offline.*
- **WGSL codegen of the world field** — orchestrator emits `map(p)->Field` source; kit ships a
  zero-runtime-dep WGSL snippet library for the 8 primitives + CSG + the raymarch/shade kernel. *L.* The
  Shadertoy/Dreams model — largest perf jump per unit effort; re-targets the *same atoms* into shader text.
- **Scene-as-instruction-buffer SDF interpreter** — flatten scene → typed node storage buffer + one static
  compute stack machine. *L.* Scene edits become microsecond buffer uploads (no recompile stall) — the
  Dreams editable-at-60 Hz workflow; arguably *more* doctrine-faithful (scene becomes explicit data).
- **Bake a 3-D distance brick-atlas** from the analytic field for O(1) far-field shadows/AO/GI. *M.*
- **Real-time window + input** wired to the existing `orbit_camera`. *L+M.* (`Framebuffer::to_u32()`
  already feeds a software-present surface.) *Purist escape hatch:* hand-written Vulkan FFI + hand-emitted
  SPIR-V keeps literal zero-dep but is XL and platform-forked — listed last, not recommended.

### Tier 4 — engine/ecosystem shell (only on the general-engine fork; mostly undifferentiated)
Retained scene graph + change detection · ECS + parallel scheduler · `App::add_plugin` plugin trait
(the Bevy organizing principle) · interactive editor (viewport + ray-picked gizmos + inspector) ·
wasm32 web target · audio mixer (the `combine` atom over sample streams) · networking (`std::net`
transport stays zero-dep; the system is XL scope) · packaging/docs/community.

## What preserves the doctrine vs what breaks it

**Preserves it (pure std, decomposes into the 8 atoms):** bounded fold + BVH (a *tree*-fold), GGX BRDF,
triplanar + std PNG, IBL/LTC, **SDFGI**, linear-HDR target + post stack, **SDF-native collision/skinning**,
time/keyframes, scene format + heatmap + profiler. *Most of the high-value work is here.*

**Breaks a constraint (state which):** GPU backend & WGSL codegen & instruction-buffer (break zero-dep +
CPU-only + offline) · real-time window/input/audio/wasm (break offline + zero-dep) · triangle meshes +
glTF (break SDF-only) · non-uniform affine (weakens the exact-distance assumption) · FEM soft bodies (the
genuine doctrine failure — needs a sparse solver).

## Top 5 next steps (highest leverage, valuable on *either* fork)

1. **Bounded fold.** Add `bound()` per `Prim`; turn `world()`'s linear `fold` into a pruned fold. *S, breaks
   nothing, helps CPU and GPU alike, prerequisite for the BVH.*
2. **GGX BRDF.** Replace Blinn-Phong in `shade.rs`; add `Material.metallic`. *M.* Do it before any lighting work.
3. **Linear HDR target + G-buffer.** Stop tonemapping inside `shade_pixel`; defer ACES+gamma to a post pass;
   keep depth (`Hit.t`) + normal. *M.* Unblocks the whole post stack and a denoiser.
4. **BVH + cascaded SDFGI.** Tree-fold over the new bounds; cone-trace the field into an SH probe grid to
   replace constant ambient with real bounce light. *L.* MM3E's structural superpower.
5. **Ship the data layer.** Std-only scene file format + serde, the near-free march-step heatmap, and a band
   profiler. *M+S+S.* De-Rusts authoring, gives the best SDF-specific debug view for almost no code, and makes
   the renderer measurable — the on-ramp to choosing Fork A vs B with real data.
