# Captured exact source-contraction challenges

These complete post-conformance, post-coalescing source meshes come from the frozen flat and folded support-enclosure fixtures in `artifacts/coalesced-edge-eligibility-20260907-r1`. Production source SHA: `0ffc50961d11e585ca841ccf69d4782fcbcaa11c36d9d1cb4977b7e4e66e04e1`.

Each coordinate is reconstructed by exact rational elimination of the original tetrahedron/basis equations, checked against the independent oracle's exact coordinates, then enclosed by the nearest binary64 lower/upper neighbors. Exactly representable coordinates use a singleton interval. These are not raw captured floating-point hints or widened f32 brackets.

The CSV format is dependency-free: `v,original_id,lo_x,lo_y,lo_z,hi_x,hi_y,hi_z` and `t,local_a,local_b,local_c`. Vertex indices are compacted; original IDs are retained. Triangle row order is unchanged. Flat has 2,511 vertices/5,018 triangles, and original edge 0–5 maps to local 0–3. Folded has 1,946 vertices/3,888 triangles, and original edge 510–846 maps to local 410–692. The JSON mirrors, exact-oracle comparison, and execution logs are in `artifacts/source-contraction-captured-20260907-r1`.

`source_contraction_captured.rs` exercises both directions on both complete meshes. Independent exact rational clipping found no nonadjacent swept or forbidden affected-final contacts in these cases. The test requires production admission; an unresolved interval rejection remains a failed admission challenge, not evidence of a false acceptance. Admission does not certify adjacent swept trajectories, final f32 embedding, topology, native residuals, or UV fidelity. The flat nearest-f32 degeneracy is separately preserved.

To regenerate from the frozen evidence, run from the workspace root:

```sh
python3 mm3e-kit/tests/fixtures/source_contraction_captured/generate.py --capture-directory artifacts/coalesced-edge-eligibility-20260907-r1 --json-output artifacts/source-contraction-captured-20260907-r1
```

`manifest.json` records source/oracle hashes, output hashes, dimensions, and interval-width statistics. Regeneration should reproduce the fixture bytes exactly.
