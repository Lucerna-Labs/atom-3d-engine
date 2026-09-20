# Attached surface textures

The editor supports original PNG texture assets, independent indexed corner UVs,
planar UV generation, and opaque albedo modulation on native triangle surfaces.
UVs stay attached when the existing skin/morph or cloth paths replace vertex
positions while retaining the original topology. These controls operate through
the normal revision-checked JSONL transactions and native project persistence.

The later [material-map layer](MATERIAL_MAPS.md) adds roughness, metallic, emission
and tangent-space normal maps to these bindings. The albedo contract below remains
unchanged, while the albedo image itself is now optional when other maps are present.

Texture painting, displacement, anisotropic filtering,
automatic chart unwrapping and general textured interchange remain open.
Compatible composed surfaces have a bounded [textured USD path](TEXTURED_USD.md).
The current surface-detail layer is not a full film-production certification.

## Source images and interpretation

`import_texture` requires a source path and explicit `color_space:"srgb"` or
`"linear"`. This declaration controls interpretation in the renderer's linear
Rec.709/sRGB-primary working space, overriding PNG gamma/ICC metadata. No automatic
ICC or OCIO conversion occurs. Original PNG bytes, including metadata, remain
embedded as standard padded base64, with a SHA-256 identity. The original file can
be removed after import; `export_texture` recovers those exact bytes and separately
reports the declared interpretation.

Static PNG grayscale, grayscale-alpha, RGB, RGBA and indexed images are accepted;
palette and low-bit grayscale sources expand before sampling. Eight- and sixteen-bit
samples are retained at the decoder boundary rather than stripping to eight bits.
`texture_state.image.decoded_bit_depth` describes the expanded samples. Animated
PNG and malformed data are rejected. Text and ICC payloads are not interpreted by
the renderer.

The [PNG specification](https://www.w3.org/TR/png-3/) defines unassociated color
and linear alpha. Here RGB is decoded to linear light first, then premultiplied by
alpha for filtering. The material operation is explicitly:

```text
linear albedo = evaluated base albedo * (filtered premultiplied RGB + 1 - filtered alpha)
```

Opaque texels multiply the base color. Transparent texels retain it; hidden RGB
does not bleed into filtered edges. Alpha does not cut geometry or change film
coverage. A white base albedo displays the texture's opaque colors without tint.
The existing world-space checker, if enabled, remains part of the evaluated base
color and is separate from the attached UV map.

## A complete small example

Start a fresh editor from the inner workspace with `--root .`. Send each JSON
object on one line. The included calibration image is original generated test
art, not an approved character material.

The binary package includes it at `examples/uv-checker.png`; use that import path
when that directory is inside the chosen editor root.

```json
{"id":"image","expected_revision":0,"command":{"op":"import_texture","request":{"id":"print","path":"mm3e-editor/tests/fixtures/textures/uv-checker.png","color_space":"srgb"}}}
{"id":"surface","expected_revision":1,"command":{"op":"apply","operations":[{"op":"create","object":{"id":"panel","shape":{"type":"surface","vertices":[[0,0,0],[1,0,0],[1,1,0],[0,1,0]],"triangles":[[0,1,2],[0,2,3]],"thickness_m":0.01},"material":{"albedo":[1,1,1],"roughness":0.7}}},{"op":"project_uvs","request":{"id":"panel/uv","object":"panel","origin":[0,0,0],"axis_u":[1,0,0],"axis_v":[0,1,0],"meters_per_uv":[1,1]}},{"op":"bind_texture","binding":{"object":"panel","uv_set":"panel/uv","texture":"print","sampler":{"u":"clamp","v":"clamp","filter":"trilinear"}}},{"op":"set_camera","camera":{"eye":[0.5,0.5,2],"target":[0.5,0.5,0],"fov_degrees":35}},{"op":"set_settings","settings":{"width":128,"height":128,"quality":"full","spatial_aa":2,"shadows":false,"ao":false}}]}}
{"id":"uv","command":{"op":"uv_state","request":{"id":"panel/uv","triangles":[0,1]}}}
{"id":"texels","command":{"op":"texture_state","request":{"id":"print","uv":[[0.25,0.75],[0.75,0.25]],"lod":0}}}
{"id":"appearance","command":{"op":"material_state","request":{"points":[[0.25,0.75,0.005]],"footprint_m":0.001}}}
{"id":"preview","command":{"op":"render","path":"texture-preview.png"}}
{"id":"linear","command":{"op":"render","path":"texture-linear.exr"}}
{"id":"save","command":{"op":"save","path":"textured-project.json"}}
{"id":"source","command":{"op":"export_texture","request":{"id":"print","path":"original-texture.png"}}}
```

Use new output paths and the actual current revision in an existing session.
`import_texture` accepts `dry_run:true`; it validates the complete candidate without
committing. Replacing an existing image ID requires `request.replace:true`.

## UV identity and edits

`project_uvs` generates a planar map from the **original surface's local rest
coordinates**. Its origin and perpendicular unit axes are explicit. Each scale is
local meters per UV unit, in `1e-6..1e6`; optional `offset` defaults to `[0,0]`.
An entity's placement and subsequent deformation do not regenerate this map.
Planar projection is not an automatic unwrap of arbitrary curved geometry.

For authored charts, `put_uvs` accepts `id`, `object`, `values:[[u,v],...]` and
`corner_indices:[[i,j,k],...]`. There must be exactly one UV index triple per
original geometry triangle. UV corners are independent of geometry vertices, so
a chart seam can use different UVs without splitting cloth constraints or skin
vertices. The implementation records vertex count and a fingerprint of the ordered
triangle topology. Position-only changes retain the map; changed topology requires
updated UV data in the same transaction.

`uv_state` returns counts, topology identity and selected triangle UV triples.
Omitting `triangles` previews the first 32 and reports whether that preview is
truncated. `texture_state` optionally queries UV samples and reports premultiplied
linear RGBA, mip count, decoded mip bytes, requested LOD and actual sampled LOD.
`material_state` evaluates world points against the actual pose, returning material
owner ID, triangle, UV, footprint LOD, sampled LOD and final linear albedo.

`bind_texture` creates or replaces one albedo binding per object. `unbind_texture`,
`remove_uvs` and `remove_texture` are operations inside `apply`. Removing a referenced
asset is rejected unless its binding is also removed or updated. Specialized geometry
generators can validate intermediate states: when changing a garment's topology,
unbind/remove its UV set first, update geometry, then author/rebind UVs in the same
atomic batch. All failures preserve the previous document.

## Sampling and rendering contract

UV `v=0` is the lower image edge; PNG's top-down rows are handled once by the
sampler. `u` and `v` wrapping independently support `clamp`, `repeat` and `mirror`.
`nearest` and `bilinear` sample level zero. Default `trilinear` uses linear-light
area mipmaps, including odd-sized images, with `lod_bias` in `[-16,16]`.

Rendering estimates an isotropic projected ray footprint from camera sampling,
surface incidence, uniform placement scale and the current triangle's UV gradients.
The footprint propagates through reflection rays; GI uses its sampling cone.
This is an approximation, not an anisotropic filter or exact curved-surface ray
differential solution. Core minification tests compare it with a level-zero aliasing
control. Closest-sheet projection also defines color on shell rims and distance-space
rounding/onion surfaces. Twist/bend domain warps are rejected for textured bindings.

Object provenance follows the exact authored material-selection policy, without
inferring object identity from material IDs. Union and smooth union keep their
existing winner rules; subtraction projects the retained base owner's UVs. This
does not invent separately authored charts for newly cut boundaries.

Beauty, reflections, film output, GI and albedo observations share the resolver.
The editor uses checked rendering and refuses to write an image when appearance
evaluation fails. Low-level Rust clients should use the `*_checked` render/GI APIs;
legacy convenience wrappers fail explicitly rather than return fallback pixels.
Texture binding changes invalidate existing GI. Textured hybrid reprojection uses
a full render because its old G-buffer lacks texture/content/footprint provenance.

Visual UV/image edits do not invalidate cloth's physical cache. Historical snapshots
retain original PNG data while releasing unused decoded caches; undo can rebuild
those disposable buffers. GPU and legacy text export reject textures. The composed
field USD path requires explicit [UV transfer and material conversion](TEXTURED_USD.md)
and rejects unresolved seams or unsupported maps before creating output.

## Limits and evidence

Up to 32 images share 16 MiB of original PNG data and 16,777,216 source pixels;
each axis is at most 8192. Up to 512 UV sets share 1,048,576 values, also subject to
per-surface topology limits and the 64 MiB native-project admission budget. UV values
are finite f64 within ±1,000,000. Native projects preserve the full data; ordinary
JSONL command lines remain limited to 4 MiB. Planar generation avoids retransmitting
large generated arrays.

The [actual executable acceptance](../scripts/agent_texture_acceptance.py) checks a
continuous blinking/mouth fixture and simulated sewn panels with openings. It uses
independent UV/color calculations, compares actual rendered pixels, verifies cache
preservation, preserves failed attempts, and reloads every texture and image after
the input PNG is deleted. The calibration renders establish implemented behavior;
they do not establish artistic or anatomical production quality.
