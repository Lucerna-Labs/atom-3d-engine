# Geometry delivery and Boolean boundaries

The engine keeps its native signed-field renderer. Geometry delivery derives a polygon
cache from the same evaluated scene while retaining information that would be lost by
sampling only the final scalar.

## Why combining samples first can lose a feature

Interpolation does not generally commute with minimum or maximum. Consider two linear
fields on a one-dimensional cell: A goes from -2 to 1; B goes from 1 to -2. Their
intersection is `max(A,B)`:

| Position | Interpolated A | Interpolated B | Combine after interpolation | Interpolate combined endpoint samples |
|---|---:|---:|---:|---:|
| Left endpoint | -2 | 1 | 1 | 1 |
| Midpoint | -0.5 | -0.5 | -0.5 | 1 |
| Right endpoint | 1 | -2 | 1 | 1 |

Negative values are inside. A real interior exists between the endpoints, but sampling
the combined values and interpolating them erases it. The same information loss created
small disconnected fragments around the original spherical subtraction rim. An independent
study found the fragments already present in the combined sample graph; they were not
unused output vertices. Increasing a uniform grid did not consistently repair this case.

Research on [feature-sensitive extraction](https://www.graphics.rwth-aachen.de/media/papers/feature1.pdf)
and [Hermite-data contouring](https://www.cs.rice.edu/~jwarren/papers/dualcontour.pdf) motivates
retaining boundary and normal information during conversion. The current implementation
uses constituent affine planes and explicit source queries, rather than those papers'
complete extraction algorithms.

## Preserving the native operations

`mm3e_orchestrator::meshing::BooleanField` lowers a compiled immutable scene into scalar
channels and a Boolean expression. Rendering and meshing share the object's domain mapping.
The lowering preserves nested transform order, positive scale, offset, negation, hard
union/intersection/subtraction and shells. For finite inputs, scales and offsets distribute
through min/max; negation exchanges the two; a shell is `max(f,-f)-thickness`.

Distance operations retain their original f32 order. Smooth-union subtrees remain opaque
channels evaluated with the full original expression, while later hard cuts stay separate.
Native triangle surfaces and volumes use their actual registered geometry. Scalar parity
tests compare this program against the renderer's unpruned field across transforms, domain
warps, nested CSG, shells, smooth blends, surfaces and volumes. Malformed transforms,
primitives, buffers, domain parameters and query coordinates are rejected; floating-point
min/max must not mask a NaN into plausible geometry.

The kit interpolates each channel separately within a tetrahedron, partitions candidate
boundary polygons by the other active planes, and classifies the resulting pieces through
the Boolean expression. Shared simplex/constraint identities determine intersection
vertices across cells. Equal sampled channels receive consistent treatment. Strict edge
incidence and vertex-link checks reject nonmanifold contacts, including point pinches that
an edge-only check would miss.

## Correcting the curved surface

A closed affine mesh can still place a small facet on the wrong side of an actual curved
cut. This occurred in 125 body facets near the rim, despite passing topology and ordinary
residual checks. Delivery therefore enables a bounded source correction for the final mesh:

1. Preserve each vertex's boundary provenance and nearby affine scalar coordinates.
2. Query the original source fields and move vertices toward the actual curved boundaries
   while retaining the local relation to neighboring boundaries.
3. Check source-field values on either side of each face normal. Subdivide marked triangle
   edges and their incident neighbors consistently, and correct new vertices against the
   same source. Faces are not flipped to hide an error.
4. Recheck geometry, topology, bounds and requested surface tolerances before encoding.

Newton steps use actual representable f32 probe coordinates and their spans; a requested
offset that disappears at large world coordinates is not used as a denominator. Iteration,
refinement, callback and output bounds can reject an unresolved case. Extra source queries,
vertex displacement and added geometry are reported. The fine delivery mesh is corrected;
the unchanged half-resolution affine mesh is a comparison reference. Their measured
bidirectional distance includes both sampling and correction differences.

The editor currently uses an explicit 0.001-meter normal probe and 1e-5 scalar comparison
tolerance for this correction. These are observable checks at a declared scale, not a
universal curvature or normal certificate. The kit exposes that policy through options.

## Bounded work and remaining limits

The Boolean path supports 128 channels, 32 active planes per tetrahedron and 33,554,432
stored scalar values (128 MiB). Expression depth and node limits, per-extraction work,
mesh size and the editor's aggregate delivery budget are enforced separately. Additional
callbacks are charged before execution using the caller's scalar-program cost. A failed
quality or resource check returns an error before installing the requested output file.

This does not resolve every opaque field. Subgrid features inside an opaque channel,
singular or incompatible supports, some grid-aligned all-zero tetrahedra, global geometric
self-intersections and arbitrary topology changes remain limitations. The half-grid test
and source residuals are finite observations, not a certified Hausdorff bound. No component
pruning, primitive substitution or automatic fallback hides a failed Boolean extraction.

The same engineering principles appear elsewhere in the editor: stitches preserve panel
identity while imposing geometric constraints; cache fingerprints retain relevant physical
dependencies; independent reader and no-seam controls test actual outcomes; atomic writes
keep incomplete work from becoming an acknowledged asset. Each mechanism is verified at
its own boundary rather than inferred from a successful render or a passing schema check.
