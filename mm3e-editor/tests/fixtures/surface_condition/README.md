# Frozen original bundle conditioning reference

`bundle-input.json` is an exact copy of the original initial refiner mesh from
`artifacts/texture-refine-collapse-diagnostic-20260907-r1/refine-input-0.json`.
SHA256: `04bf38cafd0eb3492f8add58797958803593a9f7b74b9509480afcfbecb97b5b`.

It contains 3336 stored f32 vertices and 6668 triangles, including the original
face 190 whose 59.6–84.3nm edges obstruct native projection. The producing
fixture is the unchanged ±0.6 XY quad with full thickness 0.2 and 24³ grid.
This reference is the stored input geometry, not original rational/cofactor
source coordinates. The conditioning certificate uses this explicit boundary;
producer and conditioning error bounds must be composed before delivery.

The test retains all original coordinates and constraints. It allows only
contractions to existing endpoints, checks source/topology/embedding and full
original-to-final correspondence, and verifies exact-work replay and rejection
with one fewer work unit. Isolated conditioning is not complete USD/native/UV
acceptance. The test allowance is 49µm, reserving 1µm of the original 50µm
producer representation allowance; its actual displacement bound is reported.

`expected-prepared.json` freezes the prior validated output's geometry and
ancestry independently of work counters. Hash byte encodings are little-endian
f32 bits for XYZ positions, u32 indices for triangles/vertex maps, u64 values for
original face rows, and `(u32 removed, u32 retained, f64 displacement bits)` for
each ordered contraction. The baseline source output hash is recorded. This
checks that replacing repeated source scans with a prepared index preserves
all 150 contraction choices, 3186 vertices, 6368 faces and both ancestry maps.
