# Cloth contact and USD engineering checkpoint — September 6, 2026

Historical checkpoint. The newer [sewn clothing and curved delivery record](SEWING_AND_CURVED_DELIVERY_VERIFICATION.md)
shows the original export challenge passing unchanged, while the overall production goal
remains open. The failures and binary below are retained as evidence of this earlier state.

**Production readiness remains open. The required USD acceptance suite is partial and
returns failure.** This checkpoint records completed mechanisms and an unresolved geometry
delivery case; it does not approve a film-production release.

The shared release binary SHA-256 is
`924a3bbef7b639856e174cc58d9987c348ca325331f8cb6e3ba6de1df756858e`.
Its source passes **256 workspace tests**, whole-workspace/all-targets Clippy with warnings
denied, and formatting. Five opt-in tests are ignored by the ordinary workspace invocation.
The separate USD writer suite, including the independent OpenUSD reader, passes all four
tests and runs 16 schema validators. These are test counts, not artistic acceptance.

Check records:

- [Workspace tests](../artifacts/workspace-contact-usd-2026-09-06-r2.log)
- [Strict Clippy](../artifacts/clippy-contact-usd-2026-09-06-r3.log)
- [Format check](../artifacts/fmt-contact-usd-2026-09-06-check.log)
- [Release build](../artifacts/build-contact-usd-2026-09-06-r2.log)
- [Independent writer validation](../artifacts/openusd-writer-contact-2026-09-06.log)

## Completed cloth checks

The solver now has bounded swept vertex–triangle and edge–edge self-contact, inverse-mass
response, exact pins, and positional Coulomb friction. Initial and fractional cached poses
are measured for configured feature clearance and rejected when they exceed authored
tolerances. Tests include clear endpoint poses whose interpolated middle pose overlaps,
unchanged state on budget failures, connected folds, high-speed controls and geometry
invariants. These do not establish general intersection-free motion; external contact is
still discrete vertex/field contact and external friction assumes stationary geometry.

Bakes retain only transitive collision and pin dependencies and reuse the unposed compiled
scene. Non-neutral time-zero keys, joint ancestors, face controls and fitted-garment sources
are covered by regression tests. Unrelated camera, material, lighting and scenery changes
preserve freshness; physical dependency changes invalidate it.

The measured 49-vertex core fixture dropped from 5,785,920 to 1,161,600 collision work units
over 12 physical substeps, with identical candidates and positions. Caps and physics settings
were unchanged. The [original full one-second JSONL request](../artifacts/cloth-doc-performance-replay-2026-09-06/acceptance.json),
which previously exceeded the 100-million cumulative budget, now passes unchanged at
25,044,464 charged total work units, including 23,235,824 self-contact work units. The
[original failure](../artifacts/cloth-doc-example-2026-09-06/acceptance.json) remains preserved.

The [real cloth process acceptance](../artifacts/cloth-agent-acceptance-2026-09-06-e/acceptance.json)
also passes: 49 vertices, 61 frames, floor-contact versus no-contact control, atomic bad
bakes, stale-cache rejection, cold-reopen geometry/pixels, forced-kill recovery, and exact
rebake replay. A frozen-cloth control holds the camera and moving anchor at the identical
sampled pose; 21 visible cloth probes differ in ownership or depth, and the rendered image
changes. This establishes visible cloth deformation independently of anchor motion.

## Independently read USD output

`export_usd` now runs through the actual agent process. It extracts the selected composed
field, checks two resolutions and independent surface points, and atomically installs a
new USDA file. Units, identities, topology, constant materials, IOR and the sampled camera
are carried into the external mesh cache. Native edit operators, skeletons, UVs, simulation
state, lights and color-management configuration are explicitly outside this delivery.

The [latest executable acceptance](../artifacts/usd-acceptance-20260906-r7/acceptance.json)
independently passes smooth CSG/cavity geometry, isolated object selection and an animated
thickened sheet. It checks the actual OpenUSD point/topology/material arrays, all vertices
and each face's midpoint/centroid against native and independent analytic fields, normals,
extents, camera optics, a finite perspective matrix, material bindings, cold-reopen identical
bytes, and 13 rejected exports with unchanged native state and output inventory.

Three thickened-sheet frames contain 4,212/4,144/4,150 vertices and 8,420/8,284/8,296
triangles. Every frame has one closed component. The half-second case previously produced
31 open edges. Exact coincident vertices connected by collapsed faces are now shared, and
an adjacent triangle is split at an existing exactly collinear point. Face winding uses
tetrahedron sign topology. No epsilon welding, hole caps or component pruning is used.
The [rounding study](../artifacts/moving-sheet-rounding-repair-20260906.jsonl) and regression
tests preserve both the original failures and the corrected six fine/coarse cases.

## Required export case still failing

The original sharp open-rim subtraction remains required with its unchanged bounds,
64-cubed resolution, 0.075-meter sampled-convergence tolerance and 0.025 scalar tolerance.
It fails at time zero with 0.085295945-meter sampled deviation, localized in the response
near `[0.5333692, 0.23677056, 0.0076174303]`. No requested USDA is installed. The overall
script reports `status: partial`, `passed: false`, and exits 1. Simpler accepted geometry
cannot make this production gate green; the CI invocation intentionally preserves that gate.

An [independent sampling study](../artifacts/open-rim-sampling-study-20260906.jsonl) compares
27 unchanged geometry/time/grid cases. Extra near-rim components already exist in the
negative sample graph and are not unused mesh vertices. Tested uniform resolutions through
160 cells do not resolve the correct component count consistently across all three poses.
The extraction must preserve thin Boolean boundaries within cells and pass this same
challenge before this gate can close. Increasing a global grid or deleting tiny components
has not established the required result.

Further production work includes garment patterns and seams, robust external cloth contact,
continuous skin and facial deformation, detailed mouth/eye anatomy, audio timing, surface
detail and color management, larger-scene job control and throughput, and artistic review.
Remote CI and Windows execution have not been observed in this local checkpoint.
