# Open delivery gate: thin, bent cloth

The textured delivery run exposed a pre-existing scalar-meshing limitation. The
native clothing simulator and renderer retain the garment; export rejects it
when extraction cannot establish the requested geometry checks. Do not replace
the garment, thicken its authored geometry, relax the quality limits or mark this
gate complete from a static flat-panel export.

## Current working state — September 10, 2026

The source-feature producer now passes all nine convex extraction tests, including
the original flat, tilted and folded positives. A fresh check is recorded in
`artifacts/meshing-local-representation-convex-20260910-review-r1.log`.
This is an extraction result; it does not by itself establish native refinement
or delivery.

The last real JSONL focused oracle rejects all three cases during refinement
on provisional binary `25dc778d7c61c5d42dfa25c6d8b4d9795728cadd84e9cfeaf0a725d23dfc49b5`.
See `artifacts/surface-feature-oracle-current-20260910-r1/acceptance.json`.
That binary predates the conditioning and retessellation integration described
below. No new release checkpoint is established.

An explicit stored-mesh conditioning prototype removes 158 short-edge
obligations through 150 certified endpoint contractions in the original frozen
bundle. Its maximum displacement is 84.3 nm, with complete surviving-face
ancestry. Prepared topology and a prepared source-contraction index reduce its
work from 131.43 million to 27.34 million without changing output geometry,
ancestry or contraction decisions. Evidence is in
`artifacts/surface-condition-source-index-20260910-r1/`. Conditioning is now
integrated, but the complete USD pipeline still fails its shared work gate.

Native replay also exposed long slivers with millimeter edge lengths and
nanometer altitudes. Exact coplanar flips preserve their complete patches;
additional warped flips use an outward bounded patch-separation certificate.
Both old faces remain ancestors of each replacement face. The general warped
operator makes 82 flips with a 72.58 nm cumulative surface bound on the frozen
bundle. Native refinement then passes in two passes with 16,464 vertices and
32,924 faces; independent residual and outward-normal checks also pass.
See `artifacts/surface-retessellate-bounded-20260910-r1/` and
`artifacts/general-warped-native-20260910-r1/`.

The final refiner now attempts the sufficient [radial embedding proof](RADIAL_EMBEDDING.md)
under a fixed 16-million attempt cap. Any unsuccessful attempt is charged before
the complete pairwise fallback uses the remaining shared budget. Its result is
bound to the exact arrays checked. The real frozen refined mesh needs 7,828,554
proof units, versus 47,711,715 for the full pair check. Success, overlap rejection,
disjoint-component fallback, exclusive work accounting and exact-budget replay
pass, along with strict editor Clippy.

The live bundle fine-grid stage now costs 159,337,446 units, down from
199,130,604, but the comparison grid still exhausts the unchanged 200-million
aggregate allowance. The original native sheet spends 197,277,014 units in
preprocessing and fails during refinement. These failures are preserved in
`artifacts/radial-usd-bundle-20260910-r1.log` and
`artifacts/radial-usd-sheet-20260910-r1.log`. They remain required positive gates.

Subsequent quality caching preserves every frozen coordinate, triangle, ordered
flip certificate, ancestor and per-face bound while reducing the exact stage
from 37,940,390 to 29,327,872 units and the bounded stage from 44,612,887 to
30,659,079. Opaque immutable embedding snapshots now permit counted exact
comparison at unchanged boundaries. Producer compaction can update a snapshot
only through a verified used-index bijection with identical coordinate bits and
unchanged ordered face images. Neither a public report nor an empty caller list
can issue a proof. The certified conditioner checks its actual compact output.

Both original constant-map bundle tests now pass in
`artifacts/certified-chain-usd-bundle-20260910-r1.log`, including publication,
manifest checks, conflicts and file preservation. The original native sheet
still fails in `artifacts/certified-chain-usd-sheet-20260910-r1.log`: preprocessing
costs 147,037,846 units and refinement uses 33,617,068, but face orientation
remains invalid after six passes despite passing scalar accuracy. A midpoint
rounding correction removes the first pair of folds but does not yet close this
gate. Full original animated face/clothing delivery and independent reader
acceptance remain required; no release-readiness claim follows from the bundle
tests.

A new real-protocol run uses immutable binary
`b4cba1ca076636053217cdaf4f91758446ae3b6a2cc4ebad822ae14adace8c8d`
from 268 captured source files, all rehashed after its isolated release build.
The focused flat, tilted and UV cases still exceed the refinement vertex bound.
The primary original face, cloth, material and constant-floor cases exhaust
source extraction work before reaching the improved preprocessing chain. The
active-normal rejection control passes. Inputs and requests retain their
original identity; this is a failing verification checkpoint, not a release.
See `artifacts/embedding-reuse-protocol-build-20260910-r1/verification-summary.json`,
`artifacts/surface-feature-oracle-current-20260910-r2/acceptance.json`, and
`artifacts/textured-usd-acceptance-20260910-r1/acceptance.json`.

The full textured acceptance harness now independently enforces the reported
200-million work limit and exclusive ledger sum, and can compare every request
field and source byte against the original challenge with `--challenge-root`.
Contract-positive and tampering controls pass; they do not prove engine delivery.

The retained input is
`artifacts/material-maps-normal-filter-20260907-opt-in/cloth/textured.json`.
It contains the original `garment` with 2 mm thickness and the cached `pose`
animation. Required acceptance keeps its three times 0, 0.15 and 0.3 seconds,
0.015 m sampled grid-deviation limit, 0.003 authored-scalar limit and 0.25-texel UV
interpolation limit. The export must retain the complete thickened field and
its animated texture, with independent witnesses for both shell sides.

## Observed failures

On release `4d1447272d9041cba8281307f738305361b4253e6e4e5c1afd2cdec86bdc5476`,
the shared-bounds `[96,96,128]` request used 189.62 million work units for rest
alone: 82.39 million field work and 107.23 million UV work. It produced 110,956
triangles. The three-frame request exhausted the existing aggregate 200 million
work allowance before its second frame completed.

Independent one-frame requests revealed the further problem that was hidden by
that exhaustion: at 0.15 s, fine extraction found one connected component while
the comparison found 16; at 0.3 s the counts were one and 176. Reducing Z to 64
cells made the posed failures worse. See:

- `artifacts/textured-usd-cloth-work-20260907-r1/work-report.json`
- `artifacts/textured-usd-cloth-anisotropic-20260907-r1/diagnostics.json`
- `artifacts/textured-usd-acceptance-20260907-r2/acceptance.json`

The later fixed-owner optimization removes unnecessary material-owner field
traversals where ownership follows directly from the CSG structure. It does not
solve thin-sheet extraction. Do not infer successful animated export from its
lower work count.

## Why a denser or rotated grid is insufficient

The native field is distance to the source triangles minus half-thickness. A
lattice cell can intersect both shell sides while every sampled corner lies
outside. Affine interpolation of the already-composed scalar then loses the
entire piece. Curved clothing also prevents one global orientation from making
every local sheet nearly parallel to the sampling plane.

For this input, the three-frame world bounds span about 0.893, 0.884 and 0.162 m.
The last pose's smallest principal-axis span is still about 0.0956 m, against
0.002 m physical thickness. Orientation can improve work allocation but does
not establish feature preservation.

## Source feature extraction in progress

The working source now includes a bounded triangle-box query, a compiler for
cell-local source features, a convex union extractor and native-field refinement.
The editor routes eligible affine native surfaces through this implementation.
It has not passed the required delivery checks and is not a release checkpoint.

The current extractor uses triangle prisms and containing 14-plane capsule
enclosures as its initial geometry. The original rounded native field remains
the projection and validation reference. A containing enclosure by itself does
not establish correct rounded edges or corners.

The unresolved boundary is conversion of constrained intersections into f32 mesh
coordinates. Different positive-area source facets can have vertices closer than
one representable coordinate step. Merging vertices just because their rounded
positions match changed topology in a retained folded-sheet diagnostic. The
working implementation therefore retains source constraints and computes exact
coordinate rounding candidates before choosing shared mesh positions.

An independent rational-arithmetic oracle checks the rounding candidates,
including exact representable values, halfway cases and small offsets from large
coordinates. This numerical check passes; the mandatory flat, tilted and folded
geometry fixtures still reject under the current coordinated rounding search.
See `artifacts/exact-f32-bracketing-oracle-20260907-r1/` and
`artifacts/meshing-local-convex-20260907-r14.log`. Failure of that bounded search
does not prove that no valid shared rounding exists.

The source-feature route also currently regresses three existing delivery tests:
the thickened native sheet and both constant-map bundle tests fail at the same
rounding boundary. Six other USD delivery tests pass. The retained logs are
`artifacts/source-feature-existing-usd-regressions-20260907-r1.log` and
`artifacts/source-feature-existing-usd-texture-regressions-20260907-r1.log`.
Restoring these existing positives is mandatory alongside the new thin-surface
checks; the previous release package remains the last verified checkpoint.

The original animated face and garment acceptance requirements above remain
unchanged. Neither passing a rounding oracle nor accepting a simpler enclosure
closes their geometry, texture, relocation or reload gates.

### Next bounded geometry investigation

Retaining each incoming polygon can allow different internal triangulation, but
it cannot eliminate a genuinely short polygon boundary. If exact retriangulation
and shared rounding cannot resolve such a boundary, any simplification must be an
explicit geometry operation with its own evidence. Matching rounded coordinates
is not sufficient authorization to identify two vertices.

An exact audit now shows that the captured flat and folded blockers have adjacent
regions whose union removes the troublesome shared edge without changing the
surface. The relevant pairs occupy the same extraction cell and have equal exact
affine planes. Their implementation must preserve junctions needed by neighboring
regions. See `artifacts/polygon-coalescing-audit-20260907-r1/exact-analysis.json`.
This is a verified local construction opportunity, not a passing mesh or export.

The first implementation of exact coalescing passes the six basic convex controls
and removes those captured internal edges. The required flat, tilted and folded
fixtures still fail: the flat/folded cases advance to short retained boundaries,
while the tilted case reaches the unchanged work cap. These outcomes are retained
in `artifacts/meshing-local-convex-20260907-r15.log`. The next investigation is an
explicit representation policy whose cumulative displacement and conversion bound
must fit within 10% of the existing stricter native residual target. Exact mode
remains separate; no physical or texture acceptance limit is increased.

A candidate half-edge collapse must start from the conformed source mesh, pass
the link condition, preserve oriented vertex neighborhoods and components, and
introduce no triangle intersections. Topology and geometric embedding are
separate requirements; the [CGAL simplification manual](https://doc.cgal.org/latest/Surface_mesh_simplification/)
also distinguishes topology checks, normal-change filters and geometric envelopes.
No simplifier has passed these requirements in this working checkpoint.

For an endpoint collapse, corresponding points in each retained triangle move by
at most the endpoint displacement: their displacement is the moved vertex's
barycentric weight times that displacement. A two-sided surface bound additionally
requires checking that every removed triangle maps to edges or vertices retained
in the resulting complex and that each new triangle has a predecessor. Summing
accepted local bounds gives a conservative cumulative bound; conversion into f32
requires its own additional bound. This prospective certificate must stay below a
separate allowance within the existing radius-relative accuracy target. It cannot
replace native rounded-field checks, shell witnesses or source-attached UV checks.

The working `mesh_correspondence` helper uses a tighter direct certificate when
the complete ancestry is retained. Freeze the triangulation after exact region
coalescing. Map every original vertex, including retained alias witnesses, directly
to its final stored vertex. Every original face must map to a final cyclically
oriented face, edge or vertex; every final face must have an original predecessor.
Those coverage conditions give a two-sided barycentric correspondence, bounded
by the maximum original-to-final vertex displacement. Long contraction chains
therefore cannot hide drift, while disjoint contractions need not repeatedly add
the same rounding uncertainty. The helper checks outward-rounded coordinate
enclosures and does not certify topology, intersections or native/texture error.
Its integration into the representation policy remains in progress.

### Refinement overlap regression

A separate constructed input demonstrates why a geometric intersection check is
necessary after native projection. Two initially disjoint concentric closed
octahedral meshes at L1 radii `2R` and `3R` project onto the spherical offset of the
native triangle `[(0,0,0),(.1R,0,0),(0,.1R,0)]`, with `R=1/1024 m`.
The previous refiner returns success with two closed indexed components, but the
output contains 2,048 exactly coincident cross-component face pairs. Its sampled
native residual is still below `.005R`.

The exact input, output and process log are retained in
`artifacts/surface-refine-intersection-audit-20260907-r1/`. This proves a generic
projection defect; it does not establish that the one-component support producer
creates the same input. Refinement acceptance now uses a bounded triangle-
intersection validator for crossings, coplanar area overlap and intersections
beyond a legitimately shared indexed vertex or edge. It uses conservative BVH
queries and exact f32 orientation predicates under the same aggregate work cap.

The regression now rejects the two-shell input for the specific geometric
intersection, while a single-shell control still passes the original `.005R`
native residual target. That positive also independently checks oriented edge
incidence, Euler characteristic and duplicate geometric faces. Its positions,
topology and ancestry replay exactly at reported work; one fewer work unit rejects.
Both tests pass in
`artifacts/surface-refine-intersection-regressions-20260907-r1.log`.
This closes the reproduced overlap acceptance defect, not the outstanding thin
surface extraction or full film-delivery gates.

USD delivery also checks its final stored mesh, after any geometry changes made
by UV transfer. An unchanged refined mesh reuses its already charged check; other
routes charge a fresh check against the same aggregate allowance. The per-frame
`embedding_validation` report distinguishes these cases. The six existing scalar
and Boolean USD tests still pass with the final check enabled, as recorded in
`artifacts/usd-final-embedding-regressions-20260907-r1.log`. That run explicitly
excludes the separately recorded failing native-sheet test; it is not a full
delivery-suite pass.

## Implementation requirements and rationale

A source triangle thickened by a sphere is the union of its triangular prism
(two parallel offset planes and three edge half-planes) and the capsules along
its three edges. This set decomposition preserves both slab boundaries before
scalar interpolation. The native distance oracle must remain the final geometry
reference; the prism's max-of-planes scalar is not globally Euclidean distance.

The existing hard-Boolean arrangement kernel can preserve separate affine leaf
boundaries even if the composed field has no negative corner sample. Its current
global channel interface is unsuitable for a garment: it caps 128 channels,
stores all lattice-node/channel samples, and uses local channel IDs in shared
vertex keys. A production implementation needs:

1. A bounded source-triangle AABB query, expanded by physical shell radius.
2. Cell-local prism/capsule feature sets with stable global feature IDs across
   neighboring cells and tetrahedra; local slot IDs must not become weld IDs.
3. Shared boundary sampling, counted local arrangement work and unchanged
   closed/oriented topology checks.
4. Native-source projection/refinement and existing scalar, bidirectional
   deviation and component checks for curved caps and overlapping triangles.
5. The original three-frame garment challenge, two-sided shell witnesses,
   relocated texture assets and cold native reload on one identified executable.

These requirements remain the acceptance contract for the working implementation.
No global work cap or quality threshold has been increased to make the recorded
failures pass.

The September 10 kernel checkpoint adds exact, bounded reuse inside each affine
tetra context. Repeated proportional-plane decisions are cached only after the
full f32 relation is evaluated; local-support products still run when two planes
coincide only on a face or edge. Two-coordinate affine intersections use a fixed
elimination path whose returned weights match the general solver bit-for-bit in
10,000 deterministic cases. The cache and edge path pass all 85 kit library
tests, all 9 convex integration tests and strict all-targets Clippy. They reduce
the live face/garment prefix cost, but the original face and clothing exports
still reach the unchanged 64,000,000 structural cap; this remains an open
production gate.
