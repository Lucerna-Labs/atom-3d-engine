# Native triangle surfaces

`mm3e_kit::surface::TriangleSurface` retains the original indexed triangle surface and
constructs a nearest-triangle BVH. Its field is the minimum Euclidean distance to those
triangles minus a positive half thickness. This produces a two-sided physical shell with
rounded open rims. It does not voxelize the source, guess a solid interior, or close garment
openings. Overlapping thickened triangles combine through minimum distance; self-intersection
handling remains the responsibility of authoring and cloth simulation.

```rust
let surface = TriangleSurface::new(vertices, triangles, half_thickness)?;
let id = scene.surface(surface)?;
scene.add(Object::new(Prim::Surface { id }, transform, material_id));
```

The core constructor receives **half thickness**. The editor's `thickness_m` is full
physical thickness and is divided by two once when compiling this primitive. One surface
entity compiles to one scene object; object placement, material IDs, picking, and authored
scalar queries retain their existing meaning.

Geometry is immutable after construction. `vertices()`, `triangles()`, `half_thickness()`,
`bounds()`, `bound()`, `distance()` and `normal()` expose its validated data and observations.
Clones share the geometry and BVH; a deformed sample constructs new geometry without changing
the previous snapshot. Scene field acceleration encloses the shell thickness. CPU rendering,
film outputs, lighting, shadows and scene queries use the same native surface field.
Surface scenes explicitly use finite-difference world normals instead of the analytic dual
path. The standalone surface normal helper resolves the closest geometric feature directly;
at the medial surface its gradient is undefined and the helper returns the triangle winding
normal.

`distance_bounded(point, max_work)` computes the same field with a strict query budget.
It returns the actual count of AABB and triangle distance tests. Every test is charged
before execution; exhaustion returns an error rather than a partial or proxy distance.
The ordinary renderer's distance/normal queries retain their existing results.

Construction rejects nonfinite vertices, nonpositive/nonfinite thickness, invalid indices,
degenerate faces, duplicate faces, and edges with more than two incident triangles. Open
borders, disconnected pieces and either winding are accepted. This validates indexed
geometry, not a claim of collision-free manifold cloth. Limits are 65,536 vertices and
131,072 triangles. A median-split BVH has bounded depth; triangle calculations use f64 over
the original f32 coordinates and return f32 distances. Finite shell bounds are required.

GPU support is explicit: `GpuRenderer::compile_checked` and
`wgsl::build_shader_checked` reject placed triangle surfaces with a CPU-renderer diagnostic
before shader generation or GPU allocation. Compatibility wrappers fail loudly. No missing
surface, primitive replacement, or volume approximation is produced. Legacy `.mm3e` text
serialization also rejects native surface geometry; use the editor's durable project format.

Acceptance tests cover both physical sides, shell interiors, round open borders, sharp folds
without cracks, original topology/clone preservation, BVH versus brute-force triangle
distance, validation failures, transformed material-aware observations, scene acceleration
parity, GPU/legacy rejection, and real bent-sheet rendering with continuous coverage, changed
depth/normals and deterministic worker results. These tests establish native surface
rendering; they do not certify cloth simulation, garment fit or overall film-production quality.
