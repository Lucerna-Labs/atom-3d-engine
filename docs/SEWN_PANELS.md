# Native sewn-panel clothing

`create_sewn_cloth` and `update_sewn_cloth` are atomic editor operations inside
`apply`. They author one simulated garment from independently placed rectangular
triangle panels and explicit ordered boundary stitches. The original panel IDs,
placement, grid topology, pin definitions, and stitch chains remain in the native
project. A seam constrains pairs of separate vertices; it never welds or replaces
the panels' rest geometry.

This implements rectangular sewn-panel assembly. Arbitrary pattern outlines,
darts, cutting, seam allowances, grading, and measured woven-fabric constitutive
behavior remain outside this implementation.

## Correspondence and attachments

Each panel uses local vertex numbering: `index = v * (segments_u + 1) + u`.
Every seam chain must follow contiguous boundary edges without repeated vertices.
The two chains must have equal counts of at least two vertices. Pair `chain_a[i]`
with `chain_b[i]`; the editor does not sort or reverse the chains. To reverse the
correspondence, supply the reversed chain explicitly. Duplicate stitch pairs,
missing panel IDs, interior edges, skipped boundary edges, and same-panel darts
are rejected.

Pins use the existing cloth attachment contract. A pin without `target_object`
uses a fixed world-space point. A pin with `target_object` uses that object's local
point and follows its evaluated transform, including its complete joint ancestry.
For a continuously deformed surface, supply `target_triangle` and normalized
`barycentric` weights together with `target_object`; the authored triangle index
and weights are evaluated against the deformed vertices at every substep and at
requested playback times between baked frames. Finite nonnegative weights must
sum to one within 1e-6, checked in f64. Evaluation normalizes that rounding
residual and uses f64 difference coordinates before rounding the world point to
f32, preserving attachments when the character is translated. The authored
weights remain unchanged. Each barycentric evaluation charges 35 scalar
arithmetic and validation operations against the bake work budget. Caches baked with the older
unnormalized attachment calculation require rebaking; historical world/local
pin caches retain their original fingerprints.

The required `point` field remains part of the pin record for compatibility and is
ignored by this barycentric route. A local point alone is rejected for a
deformed target.

The same optional fields are available on `PatternPin` for outline-pattern
garments. They are retained in the panel recipe and become the cloth pin's
source attachment when the pattern is assembled.
Initial pin targets must meet their authored rest vertices exactly within the
existing numerical tolerance. No initial geometry snapping or hidden warmup occurs.

## Complete example

On a fresh editor document, send this JSON request as one line:

```json
{
  "id": "author-garment",
  "expected_revision": 0,
  "command": {
    "op": "apply",
    "operations": [
      {"op": "put_clip", "clip": {"id": "hang", "duration": 0.5, "tracks": []}},
      {
        "op": "create_sewn_cloth",
        "request": {
          "id": "garment",
          "label": "Two-piece hanging garment",
          "group": "wardrobe",
          "panels": [
            {
              "id": "left",
              "origin": [-0.2, 1, 0],
              "axis_u": [1, 0, 0],
              "axis_v": [0, 1, 0],
              "segments": [2, 3],
              "width_m": 0.4,
              "height_m": 0.6,
              "pins": [
                {"vertex": 9, "point": [-0.4, 1.3, 0]},
                {"vertex": 11, "point": [0, 1.3, 0]}
              ]
            },
            {
              "id": "right",
              "origin": [0.2, 1, 0],
              "axis_u": [1, 0, 0],
              "axis_v": [0, 1, 0],
              "segments": [2, 3],
              "width_m": 0.4,
              "height_m": 0.6
            }
          ],
          "seams": [
            {"panel_a": "left", "chain_a": [2, 5, 8, 11],
             "panel_b": "right", "chain_b": [0, 3, 6, 9],
             "rest_length_m": 0, "compliance": 0}
          ],
          "thickness_m": 0.002,
          "vertex_mass_kg": 0.02,
          "settings": {"iterations": 48, "substeps": 4,
                       "bend_compliance": 0.02, "max_seam_error_m": 0.001},
          "material": {"albedo": [0.6, 0.1, 0.08], "roughness": 0.75}
        }
      }
    ]
  }
}
```

Then bake and inspect the evaluated garment:

```json
{"id":"bake","expected_revision":1,"command":{"op":"bake_cloth","request":{"id":"garment","clip":"hang"}}}
{"id":"state","command":{"op":"cloth_state","id":"garment","animation":{"clip":"hang","time":0.5}}}
{"id":"image","command":{"op":"render","path":"garment.png","animation":{"clip":"hang","time":0.5}}}
```

`cloth_state` returns the retained `sewing` definition, global `stitch_pairs`,
actual evaluated positions, actual seam-length error, and whether the sampled
pose meets all authored tolerances. Baking measures seam error after every real
solver substep. Rendering independently rechecks seam lengths after interpolation
and exact pin evaluation; cached diagnostic summaries are not accepted as proof
that the evaluated pose is valid.

The seam tolerance also applies to the initial assembly pose. A garment whose
seam edges already coincide can use a tight tolerance from frame zero. A separate
assembly demonstration may explicitly permit a larger initial gap, but that
does not establish a tightly closed delivery. Tests cover both cases separately.

## Editing, durability, and limits

`update_sewn_cloth` takes the complete replacement request, preserves garment ID
and document order, and clears its previous bake cache atomically. Failed updates
preserve the entire prior project and cache. Editor undo/redo retain those native
states. Changes to stitch definitions, physical settings, attachment geometry, or
relevant ancestor animation invalidate playback until a new bake succeeds.
Existing single-panel requests and cache formats remain supported.

Each garment supports 2–16 panels, 256 total vertices, 256 seam chains, and 1,024
unique stitch pairs. The existing eight-asset, cached-vertex, physical-step, and
work budgets still apply; seam constraints are included in charged solver work.
One garment has one material and group. Enabled self-contact uses the core's
explicit sewn-pair exclusions; this is not a claim of globally intersection-free
cloth or complete arbitrary-motion collision certification.

## Actual-process acceptance

```sh
cargo build --offline --release -p mm3e-editor
python3 scripts/agent_sewing_acceptance.py --executable target/release/mm3e-editor \
  --output artifacts/sewn-panels-acceptance-NEW
```

The output directory must be new. The harness preserves all requests/replies,
native projects, requested garment recipes, independent CRC-checked PNG pixels,
three evaluated poses, a visible no-seam control, cold-reload comparisons, invalid
update rollback, and an explicit seam-compliance edit with undo/redo and rebake.
Its JSON report identifies the exact binary and records failures as failures.
