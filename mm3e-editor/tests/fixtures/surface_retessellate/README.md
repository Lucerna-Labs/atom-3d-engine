# Original conditioned bundle reference

`conditioned-bundle.json` retains every position and triangle from the original
certified endpoint-conditioned USD bundle. Each preserved original face row is
represented as a singleton ancestor set. No geometric or request changes were
made during extraction. `provenance.json` records both source and fixture hashes.

The positive test requires both observed locked pairs, 4470/4471 and 6225/6226,
to be retessellated while retaining the complete original parent sets. Exact
work replay and one-unit-short rejection are required on this same input.
This fixture tests the exact coplanar patch stage, not native/UV delivery.

`exact-retessellated-bundle.json` is the unchanged coordinate/triangle/ancestor
output of the exact coplanar stage. `bounded-provenance.json` binds its source
and extracted fixture hashes. The additive warped test requires all four
remaining noncoplanar slivers, full ancestor sets, composed per-face bounds,
exact reported-work replay and one-unit-short rejection on that same input.
