# Independent exact-geometry sign oracle

`oracle.csv` stores finite binary64 expansion terms as hexadecimal bit patterns. Python `Fraction` computes each expansion sum and homogeneous division exactly, then supplies expected projected orientation and coordinate-comparison signs. The Rust tests consume the stored expectations without sharing arithmetic implementation with the producer.

Coverage includes 4,731 points and 14,594 predicate queries:

- Every power-of-two exponent from -1074 through 1023, including subnormal, normal, underflow-prone, and overflow-prone determinants.
- Overflowing intermediate expansion sums that cancel to a genuine subnormal, exactly 1,024 terms, negative homogeneous scales and denominators, and coordinates beyond finite binary64 range.
- 128 deterministic signed carry/borrow triples and 24 families of large rational collinear coordinates with zero or one-subnormal signed offsets. The latter force cancellation across long integer products.
- All six point permutations and all six ordered axis pairs for the signed-carry, near-line, and captured-source triples.
- Original flat cofactor points (2,3,24), (24,4,18), and the tiny positive facet (0,18,5), plus folded long-collinear points (510,846,530). Exact coordinates are reconstructed from the original captured tetrahedron/basis equations, then encoded as homogeneous integer expansions with a common power-of-two scale. They are not rounded coordinate hints.

The final run matched all 9,553 orientations and 5,041 comparisons, including 5,598 integer fallbacks. These cases exercise the public 2D orientation and coordinate APIs; they do not establish 3D orientation or mesh embedding.

Regenerate from the workspace root:

```sh
python3 mm3e-kit/tests/fixtures/exact_geometry/generate.py --capture-directory artifacts/coalesced-edge-eligibility-20260907-r1
```

The seed, exponent sweep, source hashes, dimensions, and oracle hash are fixed in `generate.py` and `manifest.json`. Independent Rust tests additionally verify invalid inputs, normalized denominator equivalence, immutable point clones, exact work-budget replay, one-less failures, and cumulative charging of rejected constructors.
