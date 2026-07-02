# Changelog

All notable changes to MM3E are documented here.

## [0.8.0] — mesh ingestion: OBJ → SDF bake, on the CPU and the GPU

- **Triangle meshes now flow through the engine as fields** — the roadmap's "deliberate bridge
  to art content", done the doctrine-preserving way. `mesh::parse_obj`/`load_obj` (std-only,
  line-oriented, line-numbered errors, fan triangulation, negative/relative indices) reads a
  mesh; `mesh::bake_sdf` converts it once into exact signed distances on a grid: nearest-triangle
  distances via a triangle AABB tree (Ericson closest-point, degenerate-tri filtered), sign by
  ray-crossing parity per grid row (watertight input; the crossing plane is epsilon-nudged so
  shared-edge hits cannot double-count), multithreaded over z-slabs. A 9,600-triangle torus knot
  bakes to a 185×95×192 grid in ~1 s and its field is Lipschitz-clean (0 adjacent-pair
  violations — the sharp test for parity/sign bugs).
- **`SdfVolume`** (`mm3e-kit/src/volume.rs`, mechanism): trilinear sampling inside the grid box;
  outside, the provably conservative `max(box_dist, d(clamped) − box_dist)` (true distance
  bounds both ways, so the tracer never overshoots). **`Prim::Volume { id }`** references
  `Scene::volumes` the way materials work, so `Prim` stays `Copy`; baked meshes participate in
  CSG (`mesh_demo` smooth-unions an analytic sphere into the knot), GI, physics, and the BVH
  plan unchanged. Bit-exact world/linear-fold equivalence holds with volumes in the scene
  (tested). Volume scenes fall back to tetrahedron normals (`is_dual_safe` excludes them).
- **On the GPU too**: volumes upload as a concatenated storage buffer (binding 3) and the shader
  ports `SdfVolume::sample` exactly. Measured parity on a baked-sphere scene: **mean 0.153/255**
  — the same fidelity as the analytic scenes.
- **`Marcher.normal_h`** (new mechanism knob, default 0.0009 = old behavior exactly): a sampled
  field's trilinear surface wobbles at cell scale, so volume scenes widen the normal stencil to
  ~½ cell and the shadow/reflection lift-offs and AO base offset scale with it (all bit-identical
  for analytic scenes via `max(historical, …)`). Also baked into the WGSL twin.
- `scene_io::serialize` writes an explicit comment for volume objects (grid data is not
  text-serializable) instead of a broken directive.

## [0.7.0] — BVH tree-fold, GPU parity + GI, deliberate adapter selection

- **BVH over union runs** (`mm3e-orchestrator/src/accel.rs`) — the roadmap's top item, built to a
  hard constraint: the accelerated world field is **bit-identical** to the linear fold (proven
  point-by-point over randomized scenes in `tests/engine.rs`; freshly rendered `spheres.bmp` /
  `gi_demo.bmp` are byte-identical to their pre-BVH commits). Segments preserve CSG order
  (Smooth/Subtract stay ordered; consecutive plain-Union objects gather through a median-split
  sphere BVH with earliest-index tie-breaks); unbounded objects evaluate first and seed the prune
  bound, so a nearby floor plane discards the whole tree in one sphere test. Measured
  (`bvh_bench`): 1.5–2.3× fewer field-eval ns on small scenes, 1.2–1.8× on dense grids to 257
  objects; never loses, so it is always on.
- **GPU adapter enumeration + deliberate selection** — `list_adapters()`,
  `GpuRenderer::with_adapter(substring)`, and the `MM3E_GPU_ADAPTER` env var (report next-step
  #1). Validated across the fleet: RTX 5070 Ti 604.5 fps @ 960×540; **Intel Arc A380** runs the
  same scene at 55.4 fps with slightly *tighter* CPU parity than NVIDIA.
- **CPU↔GPU parity, measured and fixed** (`examples/gpu_parity.rs`): the WGSL twin now matches
  the CPU reference to mean ≤0.2/255 on parity-hostile scenes (was 1.6–2.4/255). Fixes: the
  emissive/Fresnel/fog blend order in `shade()` (reflected legs now inherit fog transmittance;
  emissive never dims at grazing angles), marcher budgets baked from `scene.marcher` (was
  hardcoded), the CPU's stochastic shadow schedule ported (also **+54% GPU fps** — 391.8 → 604.5),
  the checker's 64-bit FNV-1a hash tint ported bit-exactly to WGSL on u32 pairs,
  `scene.shadows`/`scene.ao` toggles compiled in (were silently ignored), material ids wrapping
  modulo `materials.len()` like the CPU, `Smooth(k≤0)` and negative elongate/scale codegen guards,
  and `f()` no longer sign-flips −inf.
- **Baked GI renders on the GPU** — `Scene::bake_gi`'s ambient cubes upload as a storage buffer
  and the shader ports `GiVolume::sample` exactly (trilinear per face + n² ambient-cube combine).
  Cornell-style parity: mean 0.191/255 vs the CPU. The SDF superpower is now on the fast path.
- **Renderer validation harness** (`examples/render_validate.rs`, report next-steps #3/#4): every
  marcher config vs a conservative ground truth — field evals, hit/silhouette agreement, depth
  error, normal error, material mismatch. Safe configs hold ≥99.4% agreement at ~2.3 mm depth
  error; the LOD dial's fidelity price is now measured, not anecdotal.
- **Example hygiene**: `game.rs` runs fixed-timestep physics behind a real frame clock (was
  dt=1/60 per unpaced frame — a 600 fps GPU played 10× fast); game + viewer input is focus-gated
  (`GetAsyncKeyState` is system-global); `MAX_DYN` deduplicated to one constant; GPU readback map
  failures panic with a clear message; `compile()` validates resolution.

## [0.6.5] — marcher tunneling fix (correctness over borrowed speed)

- **Fixed a real tunneling bug in `Marcher::march_with`.** The overlap safety guard was gated on
  `omega > 1.0`, so after one over-relaxation failure decayed omega, secant (up to `4·d`) and
  subitize steps kept firing with **no safety net**. Tunneled steps accepted hits **buried inside
  surfaces** — 2% of all hits on the profiler scene (worst 0.17 world units deep; 5× the whole
  shell thickness on the thin-shell scene). This was the root cause of the chaotic thin-shell
  self-shadowing documented in the dual-wiring tests, and it is visible as shadow acne on
  `gi_demo`'s sphere and walls in pre-fix renders.
- **The guard is now unconditional, with midpoint gap verification.** The two-sphere overlap test
  is pessimistic at grazing incidence (a good near-surface landing has a tiny sphere), so a tripped
  step first spends one midpoint sample: if the midpoint's safe sphere covers the whole skipped
  gap, the gap is provably surface-free and the landing stands. Only genuinely unverifiable gaps
  retreat to the last safe frontier, and each boost mechanism (omega / subitize / secant) decays
  only for its **own** misfires. The rescue also keeps omega alive through spurious grazing
  failures: **−5% march evals vs. plain Keinert decay**. New regression tests: the buried-hit
  invariant (`kit.rs`), plus `examples/overshoot_probe.rs` (0 buried hits post-fix, was 2,155).
- **Honest re-measurement demoted two earlier "wins."** With landings forced to verify instead of
  tunnel: **secant costs +5% march evals** (was "−14%") — now a default-off `Marcher.secant` knob,
  preserved as a primitive per the renderer report's classification doctrine; **subitize** pays
  only in its real niche (−8% evals on miss-heavy/sky framings, +8% cost on hit-dominated ones —
  the old −11%…−23% claims were largely tunneling artifacts; see `examples/subitize_econ.rs`).
  The engine test now asserts the true claims.
- **Cost of correctness:** +4.5% total field evals on the profiler scene; `balanced`/`fast`
  quality tiers unchanged. 93% of the `spheres` frame is bit-identical; the diffs are the
  corrected pixels.
- **`is_dual_safe` hoisted out of the per-ray path** — it walks every object and was being called
  per ray *and per reflection bounce* despite its "once per render" contract. Now computed once in
  `render`/`render_gbuffer`/`reproject_hybrid`/`render_checkerboard` and threaded through.
- **`scene_io::parse` validates untrusted input**: rejects degenerate/absurd sizes (u32-wrap
  risk), zero sun, coincident camera eye/target, up parallel to view, out-of-range FOV, hostile
  marcher budgets (a 4-billion-step file was an effective per-ray hang), and out-of-range material
  indices — each with a line-numbered message instead of a NaN frame or a silent wrong render.

## [0.6.4] — stochastic soft shadows (the `hash` atom enters the renderer)

- **`Marcher::soft_shadow` is now blue-noise stochastic** — the cross-domain primitive finder kept
  pointing at the one bridge the engine was missing, `hash → graphics` (procedural/stochastic
  sampling), and `hash` is literally the only one of the eight root atoms the marcher never used.
  Brought it in: jitter the shadow march's start by a hash of the ray origin and take coarser steps,
  so the per-ray jitter dithers the coarser penumbra across pixels instead of banding.
- **Validated on the real engine.** Shadows were the single biggest cost (51% of field-evals).
  Measured against the secant+LOD baseline: **shadow-phase field-evals −25.3%, total −13.8%**
  (7,633,504 → 6,579,880), **29.4 → 31.2 fps**, all 32 tests green. Image cost: mean Δ ~0.04–0.06/255
  (spheres 0.035, showcase 0.058), max ~130 at a few penumbra-edge pixels, ~0.1% of pixels changed
  by >16 — edge-localized, the same character as the secant change, and deterministic (hash of
  position) so renders stay stable frame-to-frame.
- This is the third validated cross-domain transfer to land in the engine (after secant and LOD),
  and the largest single eval cut — it makes the dominant shadow phase a quarter cheaper.

## [0.6.3] — screen-footprint LOD (a validated graphics→marcher glue, dialed by policy)

- **The cross-domain glue finder was deepened** (`xdsim --mode binding`) to weight a primitive's
  *adapter signature* (does it convert between representations) and *engine-relevance* (does it reach
  the marcher's domains), so the top hits are glue we can actually test. It surfaced graphics
  **level-of-detail** as engine-applicable glue.
- **Validated on the real marcher, then merged as a policy-dialed knob.** Adding a screen-footprint
  term to the hit tolerance (`eps += lod_footprint · t`) lets distant / sub-pixel geometry resolve in
  fewer steps. Measured against the secant-merged baseline: **−7.2% / −11.6% / −16.2% field-evals** at
  footprint 0.003 / 0.006 / 0.010, for an image mean Δ of **1.3% / 2.3% / 3.5%**, 32 tests green at
  each. Unlike secant (near-free), LOD **charges a real currency it names — image fidelity** — so it
  is a speed/quality lever, not a default.
- Merged as `Marcher.lod_footprint` (**mechanism**, default `0.0` = exact, render path byte-identical)
  wired into the `Quality` presets (**policy**): `fast`/moving `0.006` (≈−12%, imperceptible in
  motion), `balanced` `0.002`, `full`/still `0.0` (exact), interpolated by `Quality::lerp` as a frame
  converges. The marcher carries the mechanism; the orchestrator decides when to spend the currency.

## [0.6.2] — secant root refinement (a validated cross-domain transfer)

- **`Marcher::march` now does secant / regula-falsi root refinement near the surface** — a primitive
  borrowed from numerical optimization (control-numerical-opt). When the march gets close to a
  surface, it estimates the field's slope `dd/dt` from the last two samples and steps toward the
  predicted root instead of by the raw safe distance. On grazing rays the slope is shallow, so the
  secant step *exceeds* the sphere step — exactly where sphere tracing crawls — capped at `4·d` with
  the existing over-relaxation overlap test as the safety net, and gated to Lipschitz fields
  (`step_scale >= 1.0`) like over-relaxation.
- **How it was found:** surfaced as a candidate by the cross-domain primitive simulator, then
  validated empirically — not by type-checking, but by measuring the conserved currency it charges.
  Measured on the profiler scene (480×270): **march-phase field-evaluations −14.3%, total −5.3%**
  (8,063,874 → 7,633,504), **28.6 → 29.4 fps**, with a **0.19% mean image change** (sub-pixel
  silhouette shift only) and all 32 tests green. A combined `secant + coarser-eps` variant reached
  −6.5% but doubled the edge perturbation, so the eps coarsening was left out as a separate quality
  knob rather than a default.

## [0.6.1] — GPU resolution sweep (the CPU-vs-GPU "why" answered with numbers)

- **`GpuScene::render_only` + `GpuRenderer::wait_idle`**: a no-readback dispatch path. The existing
  render copies the whole frame back over PCIe every call (≈33 MB/frame at 4K), which is what an
  off-screen BMP/AOV save needs but *not* what an on-screen game does — a game presents the GPU
  texture directly. `render_only` measures pure shading throughput; `wait_idle` fences a batch.
- **`examples/gpu_resolution`**: renders the same GGX-PBR scene (soft shadows + reflections,
  aa=2, 3 bounces) at 540p → 1080p → 1440p → 4K and prints render + readback fps. Measured on an
  RTX 5070 Ti: **540p 606 fps, 1080p 167 fps, 1440p 96 fps, 4K 43 fps** (render-only). This is the
  *same* SDF raymarch the CPU runs at ~2 fps — the gap is lane count (thousands of GPU lanes vs a
  handful of CPU cores), not the algorithm. It directly answers why integrated graphics can drive
  4K (rasterizing pre-built triangles on dedicated hardware) while the CPU SDF path cannot
  (solving a distance field by marching, hundreds of evals per pixel).
- **`gpu_viewer` is now resolution-selectable**: pass `480p` / `720p` / `1080p` / `1440p` / `4k`
  (or `WxH`), e.g. `cargo run -p mm3e-gpu --example gpu_viewer --release -- 4k`, and the live fps
  shows in the title bar (`SetWindowTextW`, refreshed twice a second) — so the sweep numbers are
  watchable interactively, not just a printed table. Launched at 3840×2160 on an RTX 5070 Ti.
  (The Win32/GDI viewer reads each frame back to present it, so its fps reflects render + readback;
  the render-only ceiling is in `gpu_resolution`.)
- **`examples/cpu_vs_gpu`**: the same scene / camera / shading rendered on **both** paths at matched
  resolutions, side by side with the speedup factor. Measured (24-thread CPU vs RTX 5070 Ti):
  480p **2.7 → 725 fps (271×)**, 1080p **0.51 → 165 fps (326×)**, 4K **~0.13 → 43 fps (337×)**.
  The GPU's WGSL is codegen'd from the same world field the CPU marches, so this isolates the one
  variable that differs — parallel hardware lanes — and quantifies it: the algorithm is identical;
  only the lane count separates "a couple of frames" from "real-time 4K".

## [0.6.0] — profiling + engine-native frame generation

- **CPU profiler** (`examples/cpu_profile`): differential timing (toggle a feature, measure the
  delta) + per-phase field-eval counts. It proved the renderer is march-bound and *not* wasteful —
  the bounding-sphere prune already makes shadow rays cheap-per-eval, and shading runs only after a
  confirmed hit. Primary march + shading is ~85% of the frame; shadows 18%; AO/reflections ~0%.
- **Monomorphized field dispatch**: `Marcher::{march,normal,soft_shadow,ambient_occlusion}` and
  `trace`/`shade_pixel` are generic over the field type (`F: Fn + ?Sized`); the beauty path builds
  the concrete world closure per band so the ~8M evals/frame inline the primitive loop instead of
  calling through `&dyn Fn` (~4%).
- **Checkerboard render** (`render_checkerboard`): trace half the pixels, fill the rest from
  neighbours (~2× for the moving phase).
- **Engine-native frame generation** — **reprojection** (`reproject` module + `render_gbuffer`).
  A real frame captures colour + depth + object tags; cheap fake frames warp it into the new camera
  using the depth the engine already computes (forward warp + z-buffer + row hole-fill).
  Deterministic and testable, not a black box.
  - **Level 1 (camera):** the whole frame reprojects to the new camera.
  - **Level 2 (object motion vectors):** each pixel is tagged with the moving object it belongs to,
    and those pixels are additionally shifted by that object's world-space motion — so dynamic
    objects warp correctly, not just the camera.
  - **Level 3 (hybrid partial rerender):** `reproject_hybrid` rerenders only the disocclusion
    holes for real (raymarches those few pixels) instead of smearing them, so newly revealed
    surfaces are correct — verified closer to ground truth than the cheap fill.
  - **Wired into the live viewer:** during camera motion it renders a real G-buffer every 3rd frame
    and reprojects the rest; when still it drops reprojection and refines to a full-quality frame.
  - Measured (`examples/reproject`, 640×360): **fake frames ~11× cheaper** than a real frame
    (≈5 ms vs ≈50 ms), lifting displayed fps from ~20 to ~50. Physics/input/logic stay honest at
    full rate; only the raymarched image is generated less often.

## [0.5.0] — CPU performance + adaptive rendering

The CPU bottleneck is per-pixel ray-march cost, not threads, so the wins are algorithmic and
adaptive (the CPU path becomes a progressive previewer, not a brute-force real-timer).

- **Enhanced sphere tracing** (Keinert over-relaxation) in `Marcher::march`: step by `1.4·distance`
  and back off only when two safe spheres fail to overlap — fewer steps on the empty horizon/grazing
  rays that dominate cost, with the hit point unchanged.
- **Distance-adaptive soft shadows**: the shadow march cap now grows with `t` (fine near the caster,
  big leaps in open space) instead of a fixed small cap — the single biggest CPU win.
- Removed a **per-pixel `Vec` allocation** in the light loop (lighting is additive, so the old
  brightest-first sort was pure waste).
- **Configurable budgets**: `Marcher.shadow_steps` and `Marcher.ao_samples` (AO off at 0).
- **`Quality` presets** (`fast`/`balanced`/`full` + `lerp`): a knob bundle an adaptive renderer
  interpolates between.
- **Adaptive CPU viewer** (`examples/viewer.rs`): low-res `fast` preset while the camera moves with
  **dynamic resolution** auto-tuned to a frame budget, then **progressive refinement** to a
  full-quality still when it holds — upscaled to the window, with an FPS/mode HUD.
- **Bitmap-font primitive** (`mm3e_kit::font`) + `Framebuffer::to_rgba8` for HUD overlays.
- Measured (24 threads): full still **1.19 → 2.1 fps**; new `fast` preview **68.5 fps** at 320×180
  (vs 14.65 fps before), so the viewer is smooth while moving and converges to a clean still.

## [0.4.0] — physics + a playable game

- **SDF-native physics** (`mm3e_orchestrator::physics`): rigid sphere bodies colliding against the
  world field. The field is the collision oracle — distance gives penetration, its gradient gives
  the contact normal — so collision detection is closed-form. Gravity, sub-stepped integration,
  restitution/friction, grounded detection, sphere–sphere contacts. Two regression tests.
- **Dynamic spheres on the GPU**: the WGSL kernel unions up to 12 uniform-driven spheres into the
  baked field, so moving objects render without recompiling the shader.
- **Particle system** (`mm3e_orchestrator::particles`): short-lived sparks that integrate under
  gravity + drag and render as the same dynamic spheres — particles are just more primitives.
- **`examples/game.rs`**: a playable game — roll a ball around an SDF obstacle course (gravity,
  jump, collisions, follow camera, loose balls to bump), **collect gold orbs to score** with a
  particle burst on pickup and a win condition. GPU-rendered in real time, verified on an RTX 5070 Ti.
- The GPU dynamic-sphere budget is 24 (player + balls + collectibles + particles).
- `Scene::field()` is now public so physics/tools can query the same `Fn(Vec3) -> Field` the
  renderer marches.

## [0.3.0] — GPU backend

- **`mm3e-gpu`**: a real-time GPU backend. It codegens the SDF world field into a WGSL compute
  shader (the GPU twin of the CPU `world()` closure) and runs it on **wgpu** (Vulkan/Metal/DX12).
  Measured **~430 fps at 960×540 on an RTX 5070 Ti** — roughly 400× the CPU path — with a
  pixel-faithful image. The camera rides in a uniform, so a fixed scene compiles once and a
  real-time loop only re-uploads the camera.
- Ported to WGSL: all 11 primitives, union/smooth/subtract CSG, the round/onion/twist/bend/mirror/
  repeat/elongate domain operators, GGX PBR, soft shadows, AO, IBL ambient, reflections, and fog.
- Examples: `gpu_probe` (adapter check), `gpu_render` (GPU render to BMP + fps benchmark), and
  `gpu_viewer` (a real-time GPU window via Win32/GDI present — no windowing crate).
- The core crates (`mm3e-kit`, `mm3e-orchestrator`) remain **zero-dependency**; wgpu is isolated to
  the opt-in `mm3e-gpu` crate. CI builds and lints the GPU crate on Windows.

## [0.2.0] — the complete engine

A large feature pass turning the renderer into a full SDF engine, all within the zero-dependency,
std-only doctrine.

### Rendering
- **Cook-Torrance GGX PBR** (metallic-roughness workflow: GGX NDF, height-correlated Smith
  visibility, spectral Schlick Fresnel, energy-conserving diffuse), replacing Blinn-Phong.
- **Diffuse image-based lighting** from the procedural sky (cosine-weighted hemisphere irradiance).
- **SDF global illumination** — a baked irradiance probe volume (ambient cube + trilinear lookup)
  giving real one-bounce color bleed, parallel-baked across cores.
- **Area / sphere lights** with inverse-square falloff and distance-widened soft shadows.
- **Linear-HDR pipeline** with a real post pass: bloom (bright-pass + separable Gaussian),
  exposure, ACES tone-map, gamma.
- **Debug AOV render modes**: normals, depth, ambient occlusion, albedo, and a march-step heatmap.

### Geometry
- New primitives: **cone, ellipsoid, octahedron, hex prism** (added to sphere, box, rounded box,
  torus, cylinder, capsule, plane).
- **Domain operators**: round, onion (shell), elongate, infinite repeat, twist, bend, mirror.
- **Smooth subtract / smooth intersect** CSG, alongside the existing smooth union.
- Conservative bounding-sphere pruning of the world field (O(1) per far object).

### Systems
- **Animation**: keyframe `Track`s with easing and `Quat` slerp; the engine stays stateless, so
  animation is "evaluate tracks → rebuild scene → render".
- **Scene file format** (`.mm3e`): a hand-rolled text serializer + parser with a stable round-trip.
- **Real-time interactive viewer** (`examples/viewer.rs`): a live orbit window via raw Win32/GDI
  FFI — no windowing crate.

### Engineering
- 19-test suite (kit math/SDF/CSG + orchestrator round-trip/determinism/GI/animation).
- GitHub Actions CI (fmt, clippy `-D warnings`, build, test) on Linux and Windows.
- New examples: `gallery`, `gi_demo`, `aov`, `scene_file`, `animate`, `viewer`.

## [0.1.0] — initial release

- Dependency-free 3-D SDF raymarcher: the 3-D elevation of MMPE (2-D SDF → 3-D SDF + sphere
  tracing), built on the eight root atoms with a strict kit/orchestrator split.
- Analytic SDF primitives + CSG (incl. smooth union), sphere tracing, gradient normals, soft
  shadows, ambient occlusion, recursive mirror reflections, procedural sky + fog, supersampled AA.
- Multithreaded CPU rendering via scoped std threads; self-rolled BMP encoder.
- Examples: `spheres`, `showcase`, `turntable`.
