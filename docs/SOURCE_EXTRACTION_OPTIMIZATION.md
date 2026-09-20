# Source extraction work

## September 19 accounting correction

The relation-cache and fixed edge-solver work totals recorded for September 10
are historical measurements, not valid current work claims. Review found that
the fixed edge solve retained the general solver's Gaussian arithmetic while
charging four instead of eight units. The charge is restored to eight. This
correction preserves the original resource caps.

Full-plane and local-support relation queries now share one bounded cache keyed
by the support face mask and the unordered channel pair. A full-plane negative
result does not decide whether two planes coincide only on an edge or face.
Both positive and negative local results are cached with their own support mask.
Lookup, support construction, pivot selection, product comparisons, and
insertion have explicit precharges. A cache hit no longer hides repeated local
product arithmetic. The cache stops retaining new entries at 512 and continues
using the same charged exact predicate. Context input fields are private to the
module, and callers supply immutable sample slices through its constructor.

The direct triangle path preserves oriented corner order modulo cyclic rotation;
it does not promise the same legacy anchor or output byte ordering. A finite-f32
planar regression fixture demonstrates distinct cyclic legacy quality scores and
checks positions, source nodes/weights, affine ancestry, constraints, and global
support IDs through actual polygon emission in both orientations and all three
starting rotations. Final exported UV/texture acceptance remains a separate gate.

Verification in `artifacts/geometry-accounting-20260919-r1/`:

- `focused-tests-r1.log`: eight new cache, work-budget, and corner-provenance tests pass.
- `kit-library-r2.log`: all 94 current kit library tests pass.
- `convex-integration-r2.log`: all nine convex producer tests pass.
- `clippy-r2.log`: strict kit all-targets Clippy passes.

The focused checks include opposite-sign and noncoincident rows, local-only
coincidence, finite-f32 extremes, cache saturation, exact-budget success, and
one-unit-short rejection. Fresh original export measurements are required before
claiming any net work improvement or production readiness.

## September 10 checkpoint

The original material request completes its 221,184 lattice cells but initially
exhausts the independent 64-million structural allowance while constructing exact
coordinate rounding pools. Its callback work is 16,933,158; those units remain
part of the separate, unchanged 200-million aggregate allowance. The source,
request, grid, native field and quality limits are unchanged.

A frozen charge-site profile attributes 51,674,133 units to lattice generation
and emission. Large contributors include polygon fan selection, repeated
intersection solves, exact support reduction and source-plane construction.
The profile is diagnostic instrumentation; its overhead is not production work.
See `artifacts/material-lattice-work-20260910-r1/`.

A bounded per-tetrahedron vertex-cache prototype increased work to 59,682,244
units and was rejected. Its results and the preceding compilation failure are
retained in the `r2` and `r3` directories. No runtime cache was added to the engine.

The implemented endpoint shortcut uses a simpler fact: a nonzero affine equation
on an edge with one exactly zero endpoint has its unique root at that endpoint.
It reaches the same rank-one recursive construction as the previous Gaussian
solve and exact cofactor reduction. The full set of plane constraints remains
attached, including residual rejection for another inconsistent constraint.
No small nonzero sample is classified as zero.

On the unchanged material input, the shortcut reduces lattice work to 47,108,478
units, saving 4,565,655. A frozen reference verifies 73,344 returned-bit comparisons
across every finite f32 exponent, both endpoint positions, signed zeros, all six
support edges and changed global node ordering. Constraint rejection and exact
work-budget replay controls pass. All nine convex producer tests and strict kit
Clippy pass. See `artifacts/exact-endpoint-optimization-20260910-r1/summary.json`.

Convex triangle and quad fan scoring also reuses rounded positions, ordered edge
vectors and squared lengths in fixed stack storage. Every score retains the
previous arithmetic order and fan tie break; larger polygons and the general
Boolean path retain their existing calculation. The charge changes from
`16 * n * n` to `12 * n * n` to reflect removed repeated vector operations.
350,000 score-bit comparisons pass in both debug and release builds, as do
zero/degenerate controls, all nine convex producer tests and strict Clippy.
The independent full-material comparison saves exactly 3,043,112 additional
units and preserves all 28,920,840 bytes of pre-pool geometry, sources, polygons,
constraints and provenance. See `artifacts/material-lattice-work-20260910-r5/README.md`.
That isolated run still exhausts the unchanged structural cap later.

The September 10 path additionally treats a three-vertex convex facet as a
triangle directly: it keeps the outward winding and rotates to the smallest
stable vertex key without evaluating equivalent fan anchors. Quad and larger
facets retain the quality comparison and tie break, so this reduction cannot
change a diagonal or texture chart.

These optimizations do not establish successful film delivery. The original
face, clothing and material exports must still pass through native refinement,
complete final embedding checks, convergence comparison, UV transport and the
independent reader under their original limits.

The September 10 tree added the following changes, whose accounting was corrected above. A per-context
proportional-relation cache reuses an exact full-plane relation while retaining
the local-support product check for planes that only coincide on a face or edge.
The same cache serves convex representative-plane canonicalization, charging
each actual pair comparison instead of a squared crossing-count estimate.
An affine feature solve with a two-coordinate support now uses the same fixed
Gaussian elimination arithmetic without the temporary selected-plane vector.
The edge solver matches the general solver bit-for-bit over 10,000 deterministic
cases, including the endpoint controls; the relation cache replays the same
exact zero decision. All 85 kit library tests, the nine convex integration tests,
kit all-targets Clippy passed. That editor invocation stopped at the preserved
flat-shoulder embedding-budget failure; it did not run the later test binaries.
The September 19 no-fail-fast run also retains the preexisting folded-sheet
refinement and thin-sheet USD failures.

The September 10 release build `22d7498ccdc8fa91891600b2916a6eb5c249a47e163c0ea516c94339b6677ff3`
still rejects the unchanged face and garment fine-grid exports at the 64,000,000
structural cap. Material and constant-floor now reach the representation phase
before exhausting that same cap; the active-normal rejection control passes.
The complete acceptance record is
`artifacts/textured-usd-acceptance-20260910-r11/acceptance.json`. A local
cross-cell canonicalization experiment was rejected because it changed the
existing convex topology tests, and remains diagnostic only.
