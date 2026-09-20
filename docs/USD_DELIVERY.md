# Evaluated USD delivery

Selected objects with texture bindings require the explicit bounded
[textured USD delivery](TEXTURED_USD.md) option. Unsupported material conversions,
unresolved UV seams and failed geometry checks reject before output; source
images and native projects are retained.

`export_usd` writes an evaluated polygon mesh cache and camera to a new `.usda` file.
It meshes the actual selected scalar field, preserving the document's CSG order and cuts.
The native project remains the editable source. Export does not change its revision.

The [Boolean boundary path](BOOLEAN_BOUNDARIES.md) retains constituent hard cuts within
cells, then corrects the delivered mesh against source curves with bounded local
refinement. Its half-resolution comparison remains affine. Single opaque scalar fields
retain the ordinary tetrahedral extractor. Smooth subtrees are explicitly reported as
opaque channels, and failed Boolean extraction is not replaced by a weaker fallback.

For an existing `ball` entity and one-second `move` clip:

```json
{"id":"deliver","command":{"op":"export_usd","request":{"path":"ball-motion.usda","object_ids":["ball"],"bounds_min":[-1,-1,-1],"bounds_max":[2,2,2],"resolution":[48,48,48],"max_surface_error_m":0.03,"max_field_residual":0.01,"clip":"move","start_seconds":0,"end_seconds":1,"frames_per_second":24}}}
```

Choose bounds and tolerances for the actual object. Bounds must contain the complete sampled
surface at every delivered pose. A crossing at the sampling boundary is an error, not a
silently capped or clipped mesh. Omitting `clip` exports the authored rest pose at time zero.
The end time must land on the requested frame grid, with at most 60 inclusive frames.
Times are seconds in the engine and seconds multiplied by fps in USD. Only the authored
samples are validated; changing-topology interpolation by an external consumer is not certified.

Selection is explicit and must be nonempty with unique IDs. Its first object in document
order must be a Union seed. The selection is composed in that order regardless of request
ID ordering. Unselected subtractors do not participate. A native triangle sheet is meshed
as its full thickened field, including the round rim; this command never substitutes the
zero-thickness midsheet or a primitive proxy.

Selected skinned and morphed surfaces use their actual evaluated native triangle fields.
Unrelated unfinished cloth caches do not block a selected-body export. The result remains
a baked mesh cache; it does not export an editable skeleton or blendshape system.

For a single scalar field, extraction and validation now use counted scene/BVH work.
Every visited bounding-box and triangle test is charged before it executes. This avoids
charging every source triangle when the acceleration structure prunes most of them.
Distances and materials remain identical to the authored field, and the aggregate limit
remains 200 million work units. Boolean multi-channel extraction retains its prior static
program estimate plus counted arrangement work. These mode-specific units are not CPU
instructions or a wall-time guarantee; the response identifies the mode and query totals.

Two independent accuracy controls are required:

- `max_surface_error_m` limits sampled bidirectional distances between the requested mesh
  and a second extraction at half the cell resolution. Vertices, edge midpoints and face
  centroids are tested against the other mesh's triangles, in world meters. The two grids
  must also agree on connected-component count.
- `max_field_residual` limits absolute native scalar values at the final mesh's vertices,
  edge midpoints and centroids. The native scalar is not always exact Euclidean distance;
  this limit must not be interpreted as a geometric error certificate.

These finite checks can still miss a feature unresolved by both grids. They do not prove a
global Hausdorff bound, manifold vertex neighborhoods, or freedom from all intersections.
Thin fabric and small facial features need sufficiently tight bounds and high resolution;
an accepted coarse character mesh does not establish close-up quality. Resolution is an
even number from 8 through 128 cells on each axis. Each mesh must fit the triangle BVH's
65,536-vertex/131,072-triangle limits. The whole delivery is limited to 500,000 vertices,
1,000,000 triangles, 200 million combined estimated scalar-program and counted Boolean
work units, and 512 MiB encoded output. Boolean extraction also limits scalar storage and
active planes; the response reports extra queries, corrected/added vertices and maximum
displacement separately from the requested base grid.

The file contains world-space points, triangles, flat face normals, extents, source entity
IDs, time-varying material subsets and camera transforms. Subdivision is disabled. Each
triangle takes the native material owner at its centroid; material boundaries inside that
face are approximated. Constant scene-linear Rec.709/D65 albedo, emission, roughness,
metalness and dielectric IOR are translated to UsdPreviewSurface. IOR preserves the native
dielectric F0 (`0.08 * specular`). Procedural checker and artistic reflection parameters
are rejected, so an unsupported material cannot silently change at delivery.

Camera filmback and focal length follow [OpenUSD's camera unit conventions](https://openusd.org/release/api/class_usd_geom_camera.html).
Field of view, pose, aspect ratio, focus distance, aperture and shutter are authored.
`camera_near_clip_m` defaults to 0.0001 meters (range 0.000001..1); this is the consumer's
explicit near plane, while native tracing starts at the eye. The far plane follows the
native march limit. The file does not deliver lights, environment, a color-management
configuration, textures/UVs, skeletons, skin weights, SDF operators or cloth solver state.

The parent output directory must exist. All frames, validation and encoding finish before
staged, synchronized, no-clobber installation of the file. An existing path or invalid late
pose leaves it untouched. The response retains convergence/scalar measurements per frame,
source IDs and a native-document FNV fingerprint for accidental-change detection.

Independent reader tests are in `tests/usd_writer.rs`; operational process checks are in
`scripts/agent_usd_acceptance.py`. Writer schema checks and a real exported character's
artistic approval are different acceptance boundaries.
