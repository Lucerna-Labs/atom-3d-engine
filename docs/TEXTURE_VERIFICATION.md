# Attached UV texture checkpoint — September 6, 2026

The editor now imports original PNG assets, creates planar or indexed-corner UVs,
and renders attached albedo textures through native skin/morph and cloth motion.
**Full film-production readiness remains open.** The checker imagery is calibration
art; it is not an approved character design or measured fabric material.

The final shared release SHA-256 is
`5ffa9b0ac666bebc4401434963873edb82f14808119c6580ca89f1cc80636466`.
It passes **439 workspace tests**, strict workspace/all-targets Clippy and formatting.
Six existing opt-in tests are ignored by the ordinary invocation.

| Boundary | Evidence |
| --- | --- |
| Workspace tests | [Final source run](../artifacts/texture-workspace-tests-2026-09-06-r2.log) |
| Strict Clippy | [Final source check](../artifacts/texture-clippy-2026-09-06-r4.log) |
| Formatting | [Check](../artifacts/texture-format-2026-09-06-final.log) |
| Release build | [Build](../artifacts/texture-build-2026-09-06-r2.log) |
| Textured face and sewn cloth through JSONL | [Final acceptance](../artifacts/texture-attachment-20260906-final/acceptance.json) |
| Nine copyable authoring-guide requests and independent EXR readback | [Guide validation](../artifacts/uv-texture-guide-20260906-r1/validation.json) |
| Existing film/EXR/temporal output | [Regression](../artifacts/film-texture-regression-20260906/acceptance.json) |
| Existing storage error/restart recovery | [Regression](../artifacts/storage-texture-regression-20260906/acceptance.json) |
| Existing reference-audio dialogue | [Regression](../artifacts/dialogue-texture-regression-20260906/acceptance.json) |

## Actual attachment and source preservation

The face fixture has independently controlled curved eyelids and continuous lip/mouth
surfaces, driven by the existing head/jaw skeleton and morphs. Texture sampling is
evaluated against the current triangle vertices, not their original world positions.
Lips and surrounding skin use periodic charts with additional UV seam values, without
adding or splitting physical vertices. Independent calculations check 467 owned
face probes across three poses; maximum UV disagreement is `3.664e-15` and maximum
linear RGBA disagreement is `5.433e-8`.

The sewn garment retains its actual neckline, arm openings, stitches and baked motion.
Its 126 probes agree within `3.387e-15` UV and `6.939e-8` linear RGBA. Texture/UV edits
preserve the complete physical cache; rebaking with visual bindings present reproduces
the original cache exactly. The private collision projection excludes visual bindings.

At the same camera and pose, textures change 7,188 face pixels and 16,103 garment
pixels. Material-owner coverage remains unchanged, verifying that alpha affects only
color modulation rather than geometry. The returning facial pose reproduces its first
textured PNG. After save, source-PNG deletion and a new process, original PNG bytes,
native data and all six rendered poses match exactly.

Actual calibration images:

- [Morphed face](../artifacts/texture-attachment-20260906-final/face/textured-1.png)
- [Animated sewn garment](../artifacts/texture-attachment-20260906-final/cloth/textured-1.png)
- [Face project](../artifacts/texture-attachment-20260906-final/face/textured.json)
- [Garment project](../artifacts/texture-attachment-20260906-final/cloth/textured.json)

## Filtering, ownership and failure behavior

Core tests verify top-down PNG/lower-left UV orientation, negative wrapping and
mirror boundaries, premultiplied linear filtering without hidden-color fringes,
and area-preserving mipmaps for odd dimensions. Renderer tests check objects sharing
one material ID, subtraction/smooth-union ownership, placed and replaced posed
surfaces, actual reflection and GI color, and consistent beauty/film/albedo results.
A deliberately unfiltered level-zero control demonstrates minification aliasing;
the default mip path converges to the expected checker mean.

Editor tests cover PNG8/PNG16, palette transparency, low-bit grayscale, explicit
APNG rejection, CRC errors, planar UV generation, selected-triangle inspection,
stale topology, dependency failures, original-byte export, cache release and undo.
An unsupported runtime precision case returns an error without creating a PNG or
changing the authored document. Checked render/GI APIs never expose the internal
discarded failure buffer; legacy convenience APIs fail explicitly.

The first [process attempt](../artifacts/texture-attachment-20260906-r1/acceptance.json)
passed face sampling and rendering, then stopped because the harness indexed the
PNG decoder's two-item return as though it had three items. The [corrected attempt](../artifacts/texture-attachment-20260906-r2/acceptance.json)
preserves every geometry/color assertion and earlier failure evidence. No geometry,
texture tolerance or physical-cache assertion was relaxed.

## Declared limits

See the [authoring contract](UV_TEXTURES.md) for source/pixel/UV limits, explicit
color interpretation, alpha modulation and local-rest planar projection. Filtering
uses an isotropic projected-ray-footprint approximation. It does not implement
anisotropic filtering or exact curved-surface ray differentials. Textured hybrid
reprojection rerenders fully because its cache lacks the required appearance state.

Texture painting, normal/displacement maps, automatic unwrapping, measured skin/fabric
shading and production visual approval remain open. GPU and legacy text paths reject
textures. Selected textured composed-field USD export is rejected before output until
proper chart-aware transfer exists. Native projects and PNG/EXR renders preserve the
implemented appearance. CI includes the process check on Linux and Windows; remote
CI and Windows execution were not observed in this local run.
