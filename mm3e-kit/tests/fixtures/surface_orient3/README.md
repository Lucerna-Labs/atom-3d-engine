# Stored-f32 orientation oracle

This additive fixture contains 4,298 quadruples of original finite f32 coordinates, encoded by their hexadecimal bits. The generator uses Python `Fraction` and the 24-term homogeneous determinant to produce expected signs independently of the runtime difference-based expansion. Rust compares every sign against both the filtered stored-mesh predicate and its forced exact fallback, then against the separately verified rational `exact_geometry::orient3` implementation.

Coverage includes every f32 power-of-two exponent from -149 through 127, all 24 permutations of cancellation families, mixed random finite bit patterns, oblique coplanar planes with one-subnormal or adjacent-f32 departures, and large integer unit-volume matrices. The first-product residual families use exact differences with enough significant bits that their pairwise product requires an FMA remainder.

The tests deliberately remove each of the three exactness obligations. Dropping coordinate-difference tails produces 882 wrong signs; dropping first-product residuals produces 64; dropping third-product residuals produces 576. A naive rounded f64 difference determinant disagrees with the independent fixture in 1,335 cases. The correct implementation matches all signs, including 480 exact zeros; 2,268 cases take its natural exact fallback, and all 4,298 also exercise the forced fallback.

For finite f32 coordinates, an exact difference component is a multiple of 2^-149 with magnitude below 2^129. All degree-three products and exact FMA remainders have least bits no lower than 2^-447 and magnitude below 2^387. Thus the TwoDiff/TwoProduct expansion neither underflows nor overflows binary64. Both parts of the first product and both remainders of their third-factor products must be accumulated.

Regenerate from the Rust workspace root:

```sh
python3 mm3e-kit/tests/fixtures/surface_orient3/generate.py
cargo test --offline -p mm3e-kit --lib surface_intersections::orient3_tests -- --nocapture
```

The fixture validates orientation arithmetic; it does not by itself certify a complete exported mesh. The measured complete-mesh replay is recorded separately under `artifacts/orient3-differences-20260910-r1/`.
