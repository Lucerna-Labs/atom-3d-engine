# Independent exact 3D orientation oracle

`oracle.csv` supplies 6,890 homogeneous expansion points and 7,971 expected orientation signs. Each finite binary64 term is stored by its hexadecimal bits. Python `Fraction` sums those terms exactly and divides by the exact homogeneous denominator. The expected sign comes from an independent 24-term determinant of rows `[x, y, z, 1]`, equivalent to `det(a-d, b-d, c-d)`. Thus `d` below the oriented `abc` plane has positive sign.

The fixture covers:

- Every binary64 power-of-two exponent from -1074 through 1023, including determinants below underflow and above overflow.
- All 24 point permutations; independently positive and negative homogeneous scales; coordinates beyond finite binary64 range.
- Exactly coplanar quads under 16 deterministic arbitrary rational quaternion rotations and translations. These transforms are exactly orthogonal, rather than rounded trigonometric approximations. Positive and negative one-subnormal departures are included.
- Sixteen huge rational quad families with exact long-product cancellation, zero or one-subnormal departures, and collinear controls that must retain zero volume.
- Thirty-two signed mixed-exponent expansion quadruples, overflowing intermediate sums that cancel to the smallest subnormal, and a point using the maximum 1,024 input terms.

The first Rust run matched all 7,971 signs, including 1,200 exact zeros and 6,061 integer fallbacks. Additional Rust tests cover signed volume, independent denominator sign changes, repeated/equivalent points, one-subnormal departures from an oblique plane, exact budget replay, charged failed retries, and immutable input points.

This is additive to `../exact_geometry/`; the original 2D orientation and coordinate-comparison fixture is unchanged. It certifies predicate signs for these inputs, not mesh embedding, coplanar retessellation, native residuals, or delivery acceptance.

Regenerate from the Rust workspace root:

```sh
python3 mm3e-kit/tests/fixtures/exact_geometry_3d/generate.py
cargo test --offline -p mm3e-kit --test exact_geometry -- --nocapture
```

The seeds, exact point/query counts, sign convention, generator hash, and oracle hash are recorded in `manifest.json`.
