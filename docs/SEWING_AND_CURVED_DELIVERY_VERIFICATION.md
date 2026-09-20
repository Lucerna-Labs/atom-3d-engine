# Sewn clothing and curved delivery checkpoint — September 6, 2026

Historical checkpoint. See the newer [continuous deformation verification](DEFORMATION_VERIFICATION.md)
for weighted skinning, morph-based face motion, deformed cloth contacts and current checks.

The previously failing sharp-rim USD export now passes the unchanged executable/independent
reader acceptance. Sewn-panel authoring and animation also pass real-process checks.
**The overall film-production goal remains open.** These results close specific engineering
gates; they do not establish complete character anatomy, performance quality or a studio release.

The verified release binary SHA-256 is
`85379e864084edb2a5ffea210dafefcfdea8959f915b6af11d3fa2e289288677`.
The source passes **293 workspace tests**, whole-workspace/all-targets Clippy with warnings
denied, and formatting. Five opt-in tests are ignored in the ordinary workspace invocation.
The independent reader checks below are additional real-file validation, not an assumption
derived from the Rust test count.

| Check | Evidence |
|---|---|
| Workspace tests | [Full log](../artifacts/workspace-boolean-sewing-2026-09-06-r4.log) |
| Strict Clippy | [Full log](../artifacts/clippy-boolean-sewing-2026-09-06-r5.log) |
| Formatting | [Check log](../artifacts/format-boolean-sewing-2026-09-06-r3.log) |
| Release build | [Build log](../artifacts/build-boolean-sewing-2026-09-06-r6.log) |
| Independent USD delivery | [Passed acceptance](../artifacts/usd-boolean-acceptance-20260906-r4/acceptance.json) |
| Sewn panels | [Passed acceptance](../artifacts/sewn-panels-acceptance-20260906-r4/acceptance.json) |
| Existing cloth workflow | [Passed acceptance](../artifacts/cloth-agent-acceptance-2026-09-06-g/acceptance.json) |
| Prior v3 cloth cache | [Fresh, unchanged native file](../artifacts/v3-cloth-compatibility-2026-09-06-r2/acceptance.json) |

## Real sewn-panel construction

`create_sewn_cloth` and `update_sewn_cloth` retain independently placed rectangular panel
recipes, stable panel IDs and explicit ordered boundary correspondence. Stitch constraints
act on separate vertices with inverse-mass response and compliance. Rest geometry is
preserved; pins follow their evaluated object and joint ancestry. Solver and sampled-pose
checks enforce actual seam length error, including the initial pose. A checksum or cached
diagnostic cannot override a bad evaluated seam.

The release fixture joins two panels with a 1 mm tolerance. After gravity and attachment
motion, the measured seam gap is `1.862645149e-9 m`; the unsewn control separates by about
1.13 m. The fixed-view images differ at 2,346 pixels. The [joined image](../artifacts/sewn-panels-acceptance-20260906-r4/sewn/frame-030.png)
and [unsewn control](../artifacts/sewn-panels-acceptance-20260906-r4/no-seam-control/frame-030.png)
show actual rendered geometry. Cold reopening reproduces document, pose and PNG bytes.
An explicit compliance edit invalidates the old cache, undo restores it, and redo plus
rebake changes both the source fingerprint and measured seam response. Invalid chains and
updates preserve revision, native project and prior cache.

The core additionally checks separated-panel assembly, compliance/mass/pin behavior,
checkpoint replay, budget failures after real corrections, and direct sewn-pair contact
exclusions without transitive suppression. Existing empty-seam dynamics and old v3 cache
fingerprints remain compatible. See [the complete authoring guide](SEWN_PANELS.md).

This is rectangular panel assembly, with one material per garment and current mesh/work
limits. Arbitrary outlines, darts, seam allowances, grading and calibrated fabric models
remain open requirements.

## Curved Boolean delivery

The [boundary extraction design](BOOLEAN_BOUNDARIES.md) preserves individual hard-Boolean
fields within cells, corrects the delivered mesh against source curves, and refines narrow
triangles consistently. The half-resolution affine mesh remains the comparison reference.
Equivalent positive subtraction and negative-field intersection now produce identical
coarse and final geometry across the original three frames. Polygon diagonals use a
geometry-based quality rule and canonical outward ordering, so representation does not
arbitrarily choose a poor triangulation.

The original required request remains a 64-cubed base grid, 32-cubed comparison, three
samples at 0/0.5/1 seconds, 0.075 m sampled mesh-deviation tolerance and 0.025 native scalar
tolerance. The independent OpenUSD reader now verifies all three delivered frames:

| Time | Vertices | Triangles | Closed components | Sampled deviation (m) | Maximum native scalar residual | Backwards faces |
|---:|---:|---:|---:|---:|---:|---:|
| 0 | 28,608 | 57,208 | 2 | 0.006728693 | 0.001683965 | 0 |
| 0.5 | 28,478 | 56,948 | 2 | 0.007790488 | 0.001522496 | 0 |
| 1 | 28,466 | 56,924 | 2 | 0.009204707 | 0.001683950 | 0 |

The reader checks 769,872 vertices/face-center/edge-midpoint samples against both the
native engine and independent analytic fields. Every delivered face also passes the
unchanged ±1 mm normal probe with 1e-5 scalar tolerance. Sixteen OpenUSD schema validators,
units, extents, material bindings, physical camera parameters and finite projection
matrices pass. Smooth CSG/cavity, isolated-body and animated thickened-sheet cases remain
passing, along with cold-reopen equality and 13 rejected-export checks.

Only the middle original frame needs local subdivision: six added vertices and twelve
added triangles. This is recorded separately from the requested grid. Extra field queries
are charged before execution, with structural work and scalar-program cost reported
separately. Source validation also rejects malformed primitives, transforms, buffers,
domain values and invalid arithmetic that floating-point min/max could otherwise hide.

## Preserved failures and remaining release work

The earlier [scalar-sampling failures](CONTACT_AND_USD_VERIFICATION.md), the first
[Boolean export with 125 backwards faces](../artifacts/usd-boolean-acceptance-20260906-r1/acceptance.json),
and the [representation-sensitive final-frame failure](../artifacts/usd-boolean-acceptance-20260906-r2/acceptance.json)
remain available. Their geometry, requests, replies and tolerances were not overwritten.
The original challenge was retained throughout and now passes rather than being replaced
by a simpler case.

The new method still has explicit singular-support, opaque-field, subgrid-feature and
resource limits. It does not certify every surface intersection, between-frame topology
or external-renderer image parity. Production work remains on continuous skin/facial
deformation, detailed mouth/eye anatomy, IK/blending/retargeting, audio timing, tailoring,
material/UV detail and color management, larger-scene execution/cancellation/resume, and
artistic acceptance. The CI workflow includes the new checks; remote CI and Windows runs
have not been observed locally.
