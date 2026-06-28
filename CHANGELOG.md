# Changelog

All notable changes to MM3E are documented here.

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
