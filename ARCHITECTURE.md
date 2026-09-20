# MM3E architecture

MM3E is a dependency-free, std-only Rust 3-D engine built on one idea: **everything decomposes
into eight root atoms and recomposes** — *composition over cracking*. It is the 3-D elevation of
MMPE (the 2-D "mechanical math primitive engine"): where MMPE rasterized 2-D signed-distance
fields with a `scan`-convert, MM3E sphere-traces 3-D signed-distance fields. Same atoms, same
zero-dependency spirit, same strict two-crate split.

## The two crates

| Crate | Role | Contains |
|---|---|---|
| **`mm3e-kit`** | *all mechanism, no policy* | the 8 atoms, vectors/matrices/quaternions, SDF primitives + CSG + domain operators, the pinhole camera, the sphere tracer (normals, soft shadows, AO), the light-transport math, color/material, the framebuffer + BMP encoder |
| **`mm3e-orchestrator`** | *all policy, no mechanism* | the scene graph, the world-field closure, lighting, reflections, fog, GI baking, post-processing, render modes, animation, the scene file format, the render loop |

The rule is absolute: the kit never decides *what* to draw; the orchestrator never computes a
distance, a normal, or a tone-map itself. If a primitive in the kit grows an `if` that makes a
value judgement, that `if` belongs in the orchestrator.

## The eight root atoms

`scan · hash · fold · project · scale · compare · combine · order`

They are defined once in `mm3e_kit::atoms` and specialized everywhere:

- **scan** — the pixel grid → rays; the probe grid while baking GI.
- **hash** — the procedural checker floor; lattice jitter.
- **fold** — sphere tracing (reduce ray steps → a hit); folding objects → the world field; folding lights → radiance.
- **project** — every dot product: camera basis, `Mat3·v`, `n·l`, the half-vector.
- **scale** — vector normalization; the perspective spread; the per-object distance scale.
- **compare** — every SDF (a distance); the gradient samples for normals.
- **combine** — the GGX BRDF; alpha-over; smooth-min CSG; bloom's Gaussian; fog.
- **order** — lights brightest-first; the painter's algorithm in MMPE's 2-D ancestor.

## Data flow (one beauty frame)

```
Scene + Camera
   │  scene.world()  ── builds Fn(Vec3)->Field  (bounded fold over objects, per thread)
   ▼
render()  ── scoped std threads, one row-band each
   │
   ├─ for each pixel, AA-supersample:
   │     camera.ray ──project──► Ray
   │     Marcher::march ──fold──► Hit (pos, normal, mat, steps)
   │     trace():
   │        ambient = IBL(sky) + GI(probe volume) + floor        ── combine
   │        for each light (ordered): GGX brdf · soft_shadow · falloff
   │        + emissive, + recursive mirror reflection (Fresnel)
   │        + distance fog
   │     ► linear-HDR Vec3
   │
   ▼
post::resolve()  ── bloom (bright-pass + separable Gaussian) → exposure → ACES → gamma
   ▼
Framebuffer ──► BMP / u32
```

In a debug AOV mode (`Normal`, `Depth`, `Ao`, `Steps`, `Albedo`) the per-pixel path writes the
chosen variable straight to display and skips the post stack.

## Module map

`mm3e-kit/src`
- `atoms` (in `lib.rs`) — the canonical eight.
- `vec` — `Vec3`, `Mat3`, `Transform` (rigid + uniform scale), `Quat` (slerp for animation).
- `triangle` — projected, half-open triangle crossings used by mesh-to-SDF parity sampling.
- `sdf` — `Field`; primitives (sphere, box, rounded box, torus, cylinder, capsule, cone,
  ellipsoid, octahedron, hex prism, plane); CSG (union/intersect/subtract + smooth variants);
  domain operators (round, onion, elongate, repeat, twist, bend, mirror).
- `camera` — look-at frame + primary-ray generation.
- `march` — `Ray`, `Hit`, `Marcher` (sphere trace, tetrahedron-gradient normals, soft shadows, AO).
- `shade` — Lambert, Cook-Torrance GGX BRDF, Fresnel, sky/sky-diffuse, ACES, gamma, fog.
- `color` — `Rgba`, `Material` (albedo, metallic, roughness, reflectivity, specular, emissive, checker).
- `framebuffer` — RGBA buffer, alpha-over, BMP + u32 encoders.

`mm3e-orchestrator/src`
- `lib.rs` — `Prim`, `Modifiers`, `Object`, `Combine`, `Light`, `Scene`, `RenderMode`,
  `render`, `trace`, `orbit_camera`, GI baking entry point.
- `gi` — the SDF global-illumination irradiance probe volume (ambient cube + trilinear lookup).
- `post` — the linear-HDR resolve pass (bloom, exposure, ACES, gamma).
- `anim` — keyframe `Track`s, easing, `Lerp`, `ping_pong`.
- `scene_io` — the `.mm3e` text scene format (serializer + parser).

## Concurrency

The optional `mm3e-editor` crate adds the authoring document and transactional JSON command
service. It compiles stable, named entities into the existing scene and calls the existing
renderer. Schema and PNG dependencies stay in this outer layer. `Scene::sample_authored` and
`Scene::sample_object` expose unpruned scalar observations for tools, separately from the
rendering acceleration bounds. See [the editor guide](mm3e-editor/README.md) for the command
contract and [the design report](docs/AGENT_EDITOR_FIRST_PRINCIPLES.md) for the extension path.

Editor animation stores rest-world joint pivots, object bindings and named clips in that same
document. `Document::compile_at` first compiles rest geometry, then evaluates keyframes using
the orchestrator's interpolation and the kit's quaternion/transform mechanisms. Pose inspection,
geometry observations and renders consume independent evaluated scenes without editing the
rest document. [Timed sequence export](docs/AGENT_EDITOR_ANIMATION.md) uses the same render path.

Rendering and GI baking both fan out over scoped `std::thread` bands — no external crate. Results
are stitched in deterministic order, so a frame is identical regardless of thread count (a test
enforces this).

## Deliberate boundaries

MM3E is a **real-time-capable CPU SDF/raymarching engine**. It deliberately is *not* a GPU mesh
rasterizer. Geometry is analytic signed-distance fields (which makes GI, soft shadows, AO, and
collision queries fall out of the field for free); there is no triangle pipeline. The roadmap
(`ROADMAP.md`) tracks the path to a GPU backend and mesh ingestion for those who want them.
