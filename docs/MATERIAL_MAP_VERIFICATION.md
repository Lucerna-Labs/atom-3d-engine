# Material maps and relighting verification

Roughness, metallic, emissive and tangent-space normal maps now operate through
the native editor and renderer on animated face and sewn-cloth surfaces. The
material-map profile is described in [the authoring guide](MATERIAL_MAPS.md).
**Full film-production readiness remains open.** The rendered fixtures are
calibration surfaces, not approved anatomy or calibrated skin/fabric materials.

The shared release SHA-256 is
`fd1d3c81422223b703e47fe05dd2a51b508eefc04db83c2a3c4919039680f0e6`.
It passes **453 workspace tests**, strict workspace/all-targets Clippy and formatting.
Six existing opt-in tests remain ignored by the ordinary test invocation.

| Boundary | Evidence |
| --- | --- |
| Workspace tests | [Final log](../artifacts/material-map-tests-2026-09-06-final.log) |
| Strict Clippy | [Final log](../artifacts/material-map-clippy-2026-09-06-final.log) |
| Formatting | [Final check](../artifacts/material-map-format-2026-09-06-final.log) |
| Final release build | [Build log](../artifacts/material-map-build-2026-09-06-final-r2.log) |
| Native material maps, relighting and restart | [Final acceptance](../artifacts/material-maps-20260906-final/acceptance.json) |
| Nine guide examples and independent HDR readback | [Guide validation](../artifacts/material-map-guide-20260906-r1/validation.json) |
| Existing film/EXR behavior | [Regression](../artifacts/film-material-map-regression-20260906/acceptance.json) |
| Existing storage error/restart recovery | [Regression](../artifacts/storage-material-map-regression-20260906/acceptance.json) |
| Existing dialogue workflow | [Regression](../artifacts/dialogue-material-map-regression-20260906/acceptance.json) |

## Geometry stays authoritative

Normal maps derive their frames from current posed triangle geometry and UVs.
Tests cover tangent direction, green-axis reversal, strength zero, mirrored UVs,
mirrored geometry/domain mapping, back faces, and shell-rim fallback. Invalid UV
frames, wrong color/data interpretations and lower-hemisphere inputs are explicit
errors. Standard eight-bit normal-map quantization is retained rather than silently
reinterpreting its center value.

Shading normals affect lighting and reflection direction. Geometric normals retain
control of depth, coverage, raw normal AOVs, AO and ray origins. An independent test
places a light below the opaque geometric hemisphere while the mapped normal faces
it; the surface correctly remains unlit. Strength zero restores the exact original
film frame, including geometric data.

Scalar and normal data use independent channels, including alpha, without color's
premultiplication. Roughness and metallic multiply their existing base factors;
emission multiplies filtered premultiplied color by the HDR emissive factor.
Bindings can omit albedo entirely. Older albedo-only bindings continue to load and
render through the same API.

## Real relighting and deformation

The executable harness reuses the existing continuous face and sewn-panel fixtures,
including their geometry and prior texture checks. It applies material maps, moves
through three poses, and compares normals/scalars against independent calculations.

| Fixture | Material probes | Maximum normal-vector error | Maximum channel error | Normal-map versus flat-control changed pixels, two lights |
| --- | ---: | ---: | ---: | ---: |
| Morphing face | 296 | 1.222e-7 | 4.908e-8 | 8,102 / 8,112 |
| Simulated sewn garment | 84 | 1.300e-7 | 4.283e-8 | 16,467 / 16,442 |

The two normal-strength controls preserve exact geometry-owner coverage. Material
and lighting edits retain the complete cloth cache, which remains fresh. Invalid
normal-map interpretation is rejected without changing the document or revision.
After the map files are deleted and the process restarted, native data, source PNGs
and rendered PNG bytes match exactly.

The guide's EXR is independently decoded and matches the renderer's declared linear
float fingerprint. Its highest channel is 7.0605936, proving values above one remain
in the output. Relevant images are the [mapped garment](../artifacts/material-maps-20260906-final/cloth/right-normal.png)
and its [strength-zero control](../artifacts/material-maps-20260906-final/cloth/right-flat.png).

## GI correction and limits

A [preserved failing regression](../artifacts/material-map-gi-before-2026-09-06.log)
showed identical gathered light for mapped metalness zero and one. GI gathering had
used only a Lambert term. It now evaluates the resolved material BRDF; the corrected
test and existing GI checks pass. Gathering remains a probe approximation without
direct shadow tracing during the bake.

Normal maps are filtered by vector averaging and normalization. This does not retain
unresolved normal variance, so variance-aware roughness filtering remains open.
Per-triangle frames are not MikkTSpace vertex tangents. Displacement, anisotropic
filtering, measured skin/fabric scattering, texture painting and chart-aware textured
USD transfer are also unfinished. CI includes this process test on Linux and Windows;
remote CI and Windows execution were not observed locally.
