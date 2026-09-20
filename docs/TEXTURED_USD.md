# Textured composed-surface USD delivery

`export_usd` can package supported texture bindings with the evaluated composed
surface. It retains thickened geometry and selected CSG operations. This is a
polygon cache with PreviewSurface materials, not an editable native rig or a
guarantee of matching renderer output.

Add this explicit option to an otherwise valid `export_usd` request:

```json
"texture_delivery": {
  "filtering": "reader_defined",
  "max_uv_error_texels": 0.25,
  "max_refinement_passes": 8
}
```

The option records a real interchange limitation: USD's basic texture nodes do
not carry native nearest/bilinear/trilinear selection, footprint estimation or
LOD bias. The native project and its sampler settings remain unchanged. Every
converted map's original sampler is included in the delivery report. No
pixel-equivalence claim follows from matching UVs, images or shader parameters.

## Geometry and UVs

The exporter meshes the full selected scalar field in document order. UV transfer
operates on that extracted surface; it does not replace a thick shell with its
midsurface or omit subtractors. The transfer uses the posed source geometry and
material owner. UVs are authored per face corner in `primvars:st`, so opposite
sides of a seam can carry different coordinates at the same geometric position.

Seam cuts and bounded conforming refinement precede the final geometric checks.
The final f32 UV coordinates are checked against sampled source coordinates in
texel units, respecting only periods preserved by every bound map: Repeat-only
uses period 1; any Mirror map without Clamp uses period 2; any Clamp map disables
period reduction on that axis. Thus 0 and 1 sample identically for Repeat, but
an interpolated seam crossing through 0.5 still fails. Stored corner values stay
unwrapped. Raw and wrap-equivalent errors are reported separately. A failed face
may choose a coherent periodic branch only when its interior source triangle also
ties the native boundary winner at the affected corner. Integer-period shifts
are counted and every original validation probe is checked again. This does not
shorten ordinary UV spans or erase deliberately repeated tiles.
When an inward boundary sample cannot be represented, the original boundary
winner is retained only after an exhaustive, counted check that every source
triangle tied at native distance precision has compatible UV phase. This is a
check of those native candidates, not a proof about unobserved continuum sides.
Ambiguous/unresolved seams, work limits, output limits or failed
geometric/UV checks reject the export before files are reserved. Sampling does
not prove a global error bound or discover every missing feature. The report
distinguishes original extraction measurements from attribute-transfer work.

There is no proximity snapping. Identical f32 positions are merged only within
connected degenerate geometry. Distinct collinear points are retained by splitting
neighboring edges at the existing points. If originally closed input acquires a
boundary, repair requires each affected connected boundary component to be exactly
collinear; its edges receive the same ordered subdivisions. No vertices move and
no nonzero-area caps are added. This never performs a global weld of unrelated
touching components. Closed oriented edges and connected vertex links are checked
after transfer; an unrepaired degenerate or nonmanifold result is an error. These
indexed topology checks do not certify freedom from arbitrary geometric
self-intersection.

The existing geometry limits remain in force, including at most 60 inclusive
frames and aggregate geometry/field-work limits. Texture transfer consumes the
same work allowance. Encoded image assets have an additional 256 MiB total cap.

## Material conversion

Original PNG bytes are retained in content-addressed `source-<sha256>.png` files.
Color maps also get lossless f32 RGB EXR derivatives. Color-space declarations
override PNG metadata exactly as they do in the native renderer.

| Native role | Delivered value |
| --- | --- |
| Albedo | Decode to linear light, then `RGB * alpha + (1 - alpha)` in an opaque EXR; constant material albedo is shader scale. |
| Emission | Decode to linear light, then `RGB * alpha` in an opaque EXR; constant emission is shader scale. |
| Metallic | Original PNG, raw channels, selected R/G/B/A output and constant material scale. |
| Roughness | Original raw channel and scale if its entire range stays above the native 0.04 floor; constant 0.04 if its entire range stays below it. |
| Normal with strength zero | Geometric normal; original source image retained and the disabled contribution recorded. |

Native alpha modulates opaque color; it never becomes USD opacity. These color
conversions commute with linear filtering mathematically, subject to f32 rounding
and the reader's actual reconstruction filter. Packed scalar channels remain
independent of alpha. Wrap modes are explicitly clamp, repeat or mirror; no
source-metadata fallback is used.

Roughness that crosses the 0.04 floor is rejected: clamping individual texels
before filtering would change native results. Active tangent normal maps are
also rejected until source-to-output tangent-frame transfer is implemented.
Neither map is silently dropped. Native variance compensation, displacement,
renderer BRDFs, lighting/environment and color-management configuration are not
delivered by this path.

## Files and recovery

For a requested `shot.usda`, the exporter reserves a sibling
`mm3e-assets-<sha256-of-request-path>` directory. All source and derived images
are written without overwrite, followed by `manifest.json`. The USDA layer is
installed last. The manifest records every image's size/hash and the expected
layer hash; a manifest by itself is not proof that layer installation completed.

Move the USDA file and its sibling asset directory together. Shader asset paths
are relative to the layer and contain no machine-specific absolute paths.
Failures preserve partial assets and report their location. Existing output or
asset directories are never replaced; choose a new export path for a retry.
Installation uncertainty remains explicit through the existing storage errors.

The public Rust `encode_with_assets` API returns the layer, report, asset map and
relative asset-directory name. The byte-only `encode` API rejects textured
deliveries so callers cannot accidentally discard required images.

## Interchange references and verification boundary

The representative animated face and bent-cloth challenge is still **open**.
The unchanged `face/lips` export now passes the captured barycentric and
degenerate-split failures, but exhausts its vertex allowance while resolving
closest-feature UV discontinuities. Its original U-repeat seam has an exact
periodic transition; applying the permitted shift still leaves a measured
2.34944-texel discrepancy on a clamped V coordinate, against the requested 0.25.
The evidence is retained in
`artifacts/delivery-uv-face-replay-20260907-r4/permitted-lift-residual.json`.
The [thin-cloth extraction failure](THIN_SURFACE_DELIVERY.md) is also retained.
Neither challenge is replaced by the passing tile or cylindrical fixtures, and
no geometry, quality threshold or work cap is relaxed to claim success.

The graph uses the official [UsdPreviewSurface and texture specification](https://openusd.org/release/spec_usdpreviewsurface.html)
and [USD rendering guide](https://openusd.org/release/user_guides/render_user_guide.html).
Reader/schema validation, decoded image checks and sampled value comparisons are
separate from an external renderer's visual result. A final film-renderer
comparison remains required before asserting appearance parity.
