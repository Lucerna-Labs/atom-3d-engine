# Native surface interpolation coordinates

The core `TriangleSurface::closest_hit_bounded(point, max_work)` method returns a
nearest-surface observation with original triangle identity and interpolation
coordinates. This is a geometry foundation for attached surface detail and future
barycentric attachments. The later [UV texture layer](UV_TEXTURES.md) now uses these
coordinates in editor authoring and CPU rendering.

`SurfaceHit` contains:

- `distance`: the existing triangle-distance-minus-half-thickness field value.
- `closest_point`: the computed point on the authored midsurface, retained as three
  f64 coordinates with the documented numerical reconstruction tolerance.
- `normal`: the existing shell normal, or the original triangle's winding normal
  on the midsurface where the distance gradient is undefined.
- `triangle`: index in the original triangle array; equal-distance ties retain
  the existing lowest-original-index ordering.
- `barycentric`: finite, nonnegative, normalized weights in that triangle's original
  `[a, b, c]` corner order.
- `work`: the existing BVH and triangle-test count plus one precharged winner-attribute
  evaluation. Exhaustion returns an error without a partial hit.

Coordinates use exactly the frame in which the `TriangleSurface` was constructed.
For an ordinary local surface they are local coordinates; for a deformed or cloth
surface constructed from posed world vertices they are world coordinates. The
method does not infer an enclosing object's transform.

```rust
use mm3e_kit::{surface::TriangleSurface, Vec3};

fn main() -> Result<(), String> {
let surface = TriangleSurface::new(
    vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)],
    vec![[0, 1, 2]],
    0.01,
)?;
let hit = surface.closest_hit_bounded(Vec3::new(0.25, 0.25, 0.2), 3)?;
assert_eq!(hit.triangle, 0);
assert_eq!(hit.barycentric, [0.5, 0.25, 0.25]);
Ok(())
}
```

The existing distance, normal, traversal and bounded-distance work contracts remain
unchanged. The attribute calculation recovers the winner's feature from the
original query with the same plane/edge ordering: edge weights use the clamped
edge parameter, while interior weights use relative-vector cross areas. It does
not reconstruct small edge weights from a rounded absolute world-space point or
use a nearly cancelling Gram determinant. It normalizes boundary roundoff
within 128 f64 epsilons and checks reconstruction against the original traversal
point. The relative scale includes the triangle's longest edge and each coordinate's
magnitude; it has no absolute world-unit tolerance floor. Unsupported precision,
nonfinite queries and unrepresentable distances are explicit errors.

Eight [independent tests](../mm3e-kit/tests/surface_attributes.rs) cover analytic
interior/edge/corner cases, both shell sides, winding and deterministic ties, thin
triangles down to `f32::MIN_POSITIVE`, and 450 curved-mesh queries at three scales
against a separate least-squares/segment brute-force oracle. LBS and DQS tests
confirm that original corner parameters remain attached after actual vertex
deformation. Additional regressions retain harmless zero-plane projection residue
and reject a gross reconstruction loss at an extreme query scale. Successful
distance/normal bits and prior work counts are checked directly.

## Appearance integration

UV seams must be represented independently of geometric topology, so adding a seam
does not split a cloth constraint or skin vertex. An indexed UV triple per original
triangle can interpolate through the returned barycentric weights after deformation.
Attribute edits also need topology validation and must not invalidate physical
cloth caches merely because a visual map changed.

The renderer needs explicit object provenance alongside material selection. Generic
scenes can share material IDs, and subtraction retains the base material even when
the cut determines the boundary. Inferring object identity as `material - 1` is not
a general renderer contract. Beauty, reflections, film output, GI and the albedo
pass must share the eventual material-at-hit resolver.

Texture decoding, explicit color-space interpretation, filtering/mip selection,
native persistence and independent rendered verification are implemented by the
[UV texture layer](UV_TEXTURES.md). The
existing USD exporter remeshes the composed field, so original UV corner indices
cannot simply be copied onto its different topology. Texture delivery needs actual
attribute transfer or an explicitly different retained-surface export contract.
