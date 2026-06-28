# MM3E — Mechanical Math 3-D Engine

A **dependency-free, real-time-capable 3-D engine** built from pure math primitives, in std-only
Rust. No GPU, no Vello, no `image`, no math crate, no windowing crate — it rolls its own vectors,
matrices, quaternions, signed-distance fields, sphere tracer, global illumination, post-processing,
animation, scene format, an interactive window, and 24-bit BMP encoder.

MM3E is the **3-D elevation of [MMPE](https://github.com/Rekonquest/mm3e)** — where the 2-D engine
rasterized 2-D signed-distance fields with a `scan`-convert, this one sphere-traces 3-D
signed-distance fields. Same eight root atoms, same zero-dependency spirit, same strict
kit/orchestrator split.

![Global illumination demo](gi_demo.png)
![Showcase](showcase.png)
![Feature gallery](gallery.png)

*Top: a Cornell-style room with real GI color-bleed (red/blue walls tinting the white sphere),
baked from the SDF probe volume. Middle: PBR metals, emissive bloom, soft shadows. Bottom: the
primitive zoo plus domain operators (twist, onion, round, smooth-union). All pure-math raymarched.*

## What it is (and isn't)

MM3E is a **best-in-class real-time SDF / raymarching engine + renderer**. Geometry is analytic
signed-distance fields, which makes global illumination, soft shadows, ambient occlusion, and CSG
fall out of the field essentially for free — the things a mesh engine voxelizes or approximates to
fake. It deliberately is **not** a GPU triangle rasterizer like Unreal/Unity/Godot; it has no mesh
pipeline. See [ROADMAP.md](ROADMAP.md) for the honest gap analysis and the path to a GPU backend.

## The doctrine

Everything decomposes into the **eight root atoms** and recomposes — *composition over cracking*:

`scan · hash · fold · project · scale · compare · combine · order`

Sphere tracing is `fold` (reduce ray steps to a hit); every SDF is `compare` (a distance); the GGX
BRDF, fog, smooth-min CSG, and bloom are all `combine`; the camera basis and every normal are
`project`. See [ARCHITECTURE.md](ARCHITECTURE.md).

## Architecture — two crates, strictly split

- **`mm3e-kit`** — *all mechanism, no policy.* The atoms, math (`vec`), SDF primitives + CSG +
  domain operators (`sdf`), the pinhole `camera`, the sphere tracer (`march`), light-transport
  math (`shade`), `color`/`Material`, and the `framebuffer`. Nothing here decides *what* to draw.
- **`mm3e-orchestrator`** — *all policy, no mechanism.* The scene graph, the world-field closure,
  lighting, reflections, fog, GI baking (`gi`), the post stack (`post`), render modes, animation
  (`anim`), and the scene file format (`scene_io`). It drives the kit; it never rasterizes a pixel.

## Features

**Geometry** — 11 analytic SDF primitives (sphere, box, rounded box, torus, cylinder, capsule,
cone, ellipsoid, octahedron, hex prism, plane); CSG union/intersect/subtract + smooth variants;
domain operators (round, onion, elongate, infinite repeat, twist, bend, mirror); conservative
bounding-sphere pruning of the world field.

**Shading & lighting** — Cook-Torrance GGX PBR (metallic-roughness); diffuse image-based lighting
from the sky; **SDF global illumination** (baked irradiance probe volume); directional + point +
**area/sphere lights** with inverse-square falloff and soft shadows; recursive mirror reflections;
gradient normals; ambient occlusion; HDR sky + sun; distance fog.

**Pipeline** — linear-HDR scene-color target → post pass (bloom, exposure, ACES tone-map, gamma);
supersampled anti-aliasing; debug AOVs (normals, depth, AO, albedo, march-step heatmap);
multithreaded rendering via scoped std threads (deterministic, ≈11× on 24 cores).

**Systems** — keyframe animation with easing and quaternion slerp; a `.mm3e` text scene format
(serializer + parser, round-trip tested); a real-time interactive viewer (raw Win32/GDI, no crate).

**Engineering** — 19-test suite, GitHub Actions CI (fmt + clippy `-D warnings` + build + test on
Linux & Windows), zero external dependencies.

## Run

```sh
cargo run -p mm3e-orchestrator --example spheres    --release   # hero scene
cargo run -p mm3e-orchestrator --example showcase   --release   # every primitive + CSG mode
cargo run -p mm3e-orchestrator --example gallery    --release   # primitive zoo + domain ops
cargo run -p mm3e-orchestrator --example gi_demo    --release   # global illumination color bleed
cargo run -p mm3e-orchestrator --example aov        --release   # debug passes (normals/steps/…)
cargo run -p mm3e-orchestrator --example scene_file --release   # write + load a .mm3e scene
cargo run -p mm3e-orchestrator --example animate    --release 24 # 24 animation frames
cargo run -p mm3e-orchestrator --example viewer     --release   # live interactive window (Windows)
```

Each renderer writes a `.bmp` next to the workspace and prints its absolute path. The viewer opens
an orbit-able window — arrow keys or left-drag to orbit, `W`/`S` to zoom, `Esc` to quit.

## Scene format

`scene_io::serialize` / `parse` read and write a line-oriented `.mm3e` document — settings, camera,
materials, lights, and objects (primitive + transform + CSG mode + domain modifiers). Editing a
scene no longer means recompiling Rust:

```
size 800 450
sun 0.5 0.7 0.4
cam 0 0.8 8  0 0.8 0  0 1 0  50
mat 1 1 1 0 0.6 0 0.15 0 0 0 1          # checkered floor
mat 0.95 0.72 0.28 1 0.18 0.5 0.5 0 0 0 0   # gold metal
obj plane 0 1 0 0 pos 0 0 0 basis 1 0 0 0 1 0 0 0 1 scale 1 mat 0 combine union
obj sphere 1 pos -1.4 1 0 basis 1 0 0 0 1 0 0 0 1 scale 1 mat 1 combine union
```

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) — the two crates, the eight atoms, data flow, module map.
- [ROADMAP.md](ROADMAP.md) — honest competitive gap analysis and the path forward (GPU, meshes).
- [CHANGELOG.md](CHANGELOG.md) — release history.

## License

MIT — see [LICENSE](LICENSE).
