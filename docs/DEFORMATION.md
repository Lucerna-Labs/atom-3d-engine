# Native surface deformation

The editor can deform an authored `Surface` through linear blend skinning, rigid
dual quaternion skinning, dense blendshape position deltas, or morphs without a
skeleton. The evaluated vertices replace the native `TriangleSurface` used by
the renderer and field queries. Triangle indices remain unchanged. The source
object, rest transform, weights and deltas remain durable authoring data.

This is a CPU geometry path in the existing field renderer. An evaluated surface
still has the native field `distance_to_triangles - half_thickness`; it is not
voxelized or substituted with a separate preview mesh. Open boundaries stay open.
This is a hollow triangle shell, with no inferred solid interior or tissue physics.

## Coordinate and evaluation contract

For source vertex `p`, source-local blendshape deltas `d[k]`, and scalar weights
`m[k]`, evaluation proceeds in this order:

1. Add the weighted deltas in the source object's rest-local coordinates:
   `p_morph = p + sum(m[k] * d[k])`.
2. Apply the object's authored rest position, rotation and uniform scale.
3. Skin the resulting rest-world position with the selected joint deltas.
4. Build the native triangle field at the resulting world-space vertices and
   set the compiled object's transform to identity.

Animation joints use world-rest pivots. Their evaluated transforms are deltas
from rest, including ancestor motion. These are the transforms used by skinning;
they are not raw local joint transforms. A deformer has an explicit joint-ID
palette, and influence indices address that palette rather than document order.

`linear_blend` forms the weighted sum of transformed positions. The core only
normalizes the accepted f32 weight-sum roundoff within `1e-6`. It rejects incorrect
sums, duplicate indices, zero/negative weights and nonfinite inputs.

`dual_quaternion` blends rigid transforms with hemisphere correction. It requires
unit scale on every palette joint and its ancestors in every authored clip.
Evaluated nonunit scale is also rejected; there is no fallback to linear skinning.
An object's rest scale is applied before skinning and can be nonunit.

With `joints: []` and `weights: []`, the core executes its explicit no-skin path.
It does not invent a skeleton or an identity influence. Dense morph deltas and
the object rest transform still apply.

The rendered shell thickness is the source `thickness_m` multiplied by the source
object's rest scale. This is a fixed world-space shell thickness during the pose;
linear skinning with animated bone scale does not model thickness or tissue-volume
changes. The object must have neutral mirror, elongate, round and onion modifiers.

## Authoring operations

These operations appear inside the existing atomic `apply.operations` array.
The request envelope still requires the current `expected_revision` for a
mutation. A failed edit leaves the document, revision and undo history unchanged.

| Operation | Fields | Effect |
| --- | --- | --- |
| `bind_surface` | `request.deformer` | Bind an existing native `Surface` object. |
| `update_deformer` | `request.id`, optional `method`, `joints`, `weights`, `blendshapes` | Replace the specified authoring arrays or method atomically. |
| `set_morph_weights` | `id`, `weights: [{id, weight}]` | Edit specified default scalar controls without resending dense deltas. |
| `remove_deformer` | `id` | Remove the deformer and its scalar animation channels; retain the authored source surface. |

For example, after creating `surface` and the `root`/`elbow` joints, a binding for
a three-vertex source can be authored as:

```json
{
  "op": "bind_surface",
  "request": {
    "deformer": {
      "id": "skin",
      "object": "surface",
      "method": "linear_blend",
      "joints": ["root", "elbow"],
      "weights": [
        [{"joint": 0, "weight": 1}],
        [{"joint": 0, "weight": 0.5}, {"joint": 1, "weight": 0.5}],
        [{"joint": 1, "weight": 1}]
      ],
      "blendshapes": [{
        "id": "lift",
        "deltas": [[0, 0, 0], [0, 0.05, 0], [0, 0.1, 0]],
        "weight": 0,
        "min_weight": 0,
        "max_weight": 1
      }]
    }
  }
}
```

There must be one influence row and one delta per source vertex. Skin rows have
1–8 distinct positive influences. Morph deltas are displacement vectors, not
absolute target positions. All morphs are validated, including those whose weight
is zero. IDs within a blendshape list are distinct.

Each blendshape has an authored default `weight` and a permitted
`[min_weight, max_weight]` interval; defaults are `0`, `0`, and `1`. The global
supported scalar range is `[-1000, 1000]`. Independent controls need not sum to
one. The caller must explicitly author any relationships between controls.

The default face controls in the acceptance project can be edited with:

```json
{
  "op": "set_morph_weights",
  "id": "face/left-lid",
  "weights": [{"id": "blink-1", "weight": 0.5}]
}
```

Inspect that default edit without an animation sample. An active scalar track
overrides the corresponding default weight; it does not add to it. A valid default
shape is checked before committing. A pose that later collapses a triangle fails
evaluation before a partial scene or output is produced.

The procedural face rig accepts the same editable scalar-track mechanism for
`blink_left`, `blink_right`, `jaw_open`, `lip_round`, `lip_wide`, `smile`,
`gaze_x`, `gaze_y`, `brow_left`, `brow_right` and `lip_seal`. Gaze is a bounded
head-local eye offset, brow controls are independent per side, and lip seal
reduces an open cavity while preserving the authored jaw control. These optional
fields are backward compatible with version-1 face rigs; they do not claim
anatomical skin, teeth, tongue or eye optics.

## Scalar animation and inspection

`Clip.morph_tracks` contains channels of this form:

```json
{
  "deformer": "skin",
  "blendshape": "lift",
  "easing": "smooth_step",
  "keys": [{"time": 0, "weight": 0}, {"time": 1, "weight": 1}]
}
```

Key times are strictly increasing and inside the clip duration. Missing targets,
duplicate channels and out-of-range scalar values are errors. Supported easing is
`step`, `linear`, `smooth_step`, `ease_in`, or `ease_out`, using the engine's existing
scalar tracks. Morph and transform tracks use the same canonical clamp/loop time.

`deformer_state` returns actual world-space vertices, unchanged triangle indices,
evaluated time, scalar controls, vertex bounds, maximum displacement and shell
thickness. For example:

```json
{
  "op": "deformer_state",
  "id": "face/left-lid",
  "animation": {"clip": "expression", "time": 0.237, "playback": "clamp"}
}
```

`pose` also includes compact deformation summaries. Rendering, picking, object
field sampling and cloth collision use the refreshed native fields. Sampling does
not bake source geometry or advance editor history.

An object can have only one geometry owner. A deformed object cannot also be a
cloth asset or generated facial/fitted-garment object, have a rigid joint-object
binding, or have an object transform track. Animate the deformer through its
joints and scalar channels. Material edits remain independent. Rest geometry and
rest-transform edits are accepted when topology, weights and deltas remain valid.
Remove the deformer before deleting its source object.

Cloth collision dependencies include the relevant deformation data and tracks, so
an affected bake becomes stale after a relevant edit. A cloth pin may follow a
deformed surface by naming its source `target_triangle` and normalized
`barycentric` weights; the triangle index remains attached to the authored source
topology and is evaluated at every physical substep. Its retained `point` field
is ignored for this route. An arbitrary local-point pin
on a deformed object is still rejected. Existing ordinary object and joint-driven
attachments remain available.

The document limits are 64 deformers, 262,144 total deformed vertices, 1,048,576
dense morph deltas, and 4,194,304 counted vertex/influence/delta operations per
evaluation. An individual asset supports 65,536 vertices, 256 palette joints,
64 blendshapes and eight influences per vertex. A clip supports 2,048 morph tracks;
their keys share the existing document animation-key budget.

## Continuous face acceptance

Run the real executable and preserve a fresh artifact directory:

```bash
cargo build --offline --release -p mm3e-editor
python3 scripts/agent_deformed_face_acceptance.py \
  --editor target/release/mm3e-editor \
  --output artifacts/my-deformed-face-acceptance
```

The script authors four actual native surfaces: two curved eyelids, a lip annulus,
and surrounding mouth skin. They contain 594 vertices and 960 triangles. The eyes
remain native spheres. A native head with a local mouth cut and a dark interior
provides independently measurable occlusion. Head and jaw joints carry the same
geometry shown in the PNGs.

Each lid uses eight adjacent curved targets and explicit piecewise linear hat
weights. This avoids taking one long straight chord from the open lid to the
closed lid through the eyeball. The script checks an independent support-plane
bound for every triangle over every adjacent transition: the triangle lies within
the convex hull of its six endpoint vertices. The minimum projection of those
vertices onto a unit support direction bounds the distance to the sphere center.
The actual shell thickness is included. Shared rigid head motion preserves this
bound. This fixture's minimum conservative clearance is 1.61 mm over the entire
motion, not only at sampled endpoints.

The release run additionally measures the actual engine fields at 33 times:
eyelid vertices, unique-edge midpoints and triangle centroids are queried against
both the posed eye and the native lid itself. The sphere field is cross-checked
independently from the returned pose. This is 53,922 geometric probe positions,
each tested against both fields. The sampled minimum shell clearance is 5.96 mm.

The five frames at `0`, `0.2`, `0.4`, `0.6` and `0.8` seconds show closing and
opening, a 12-degree head turn and an 8-degree jaw rotation. In the recorded
acceptance, each eye has 1,791 visible owner pixels at rest, zero when closed,
and 1,791 on return. Mouth-interior pixels grow from 58 to 1,463. A point on the
closed mouth boundary changes from `-1.5 mm` inside the actual field to `+30.49 mm`
clear air after opening. The return PNG is identical to the initial PNG.

The run records the executable and script SHA-256 digests and rejects an executable
that changes during acceptance. It preserves the full JSONL transcript, authored
project, numerical clearance
records, native vertices for every displayed pose, five PNGs and a lossless animated
PNG assembled from the actual images. It checks malformed dense-edit rollback,
scalar edit/undo/redo, unchanged authored objects, and a fresh editor process
reproducing all five native poses and PNG files exactly. A failure is recorded in
`acceptance.json`; rerun in another output directory rather than replacing evidence.

The recorded [release acceptance](../artifacts/deformed-face-final-20260906/acceptance.json)
passed on binary SHA-256
`4fef789db4ac2031031d97570bc6f16da300584c0c30f4a6eb0194a6533b4011`.
Its [actual rendered playback](../artifacts/deformed-face-final-20260906/deformed-face-playback.png)
and [saved editable project](../artifacts/deformed-face-final-20260906/deformed-face.mm3e-agent.json)
are retained alongside the measurements.

This face is an agent-designed continuous geometry demonstration. It does not
certify anatomical fit, tissue behavior, production character quality, arbitrary
blendshape collision safety, teeth, tongue or speech synchronization. The lid
clearance proof relies on this fixture's adjacent targets and shared rigid eye/head
motion; it is not a general deformation collision solver.

## Implementation and verification

The core is in `mm3e-kit/src/deform.rs`; editor authoring, validation and actual
surface refresh are in `mm3e-editor/src/deform.rs`. Analytic integration tests in
`mm3e-editor/tests/deformation.rs` independently check bending, the distinction
between LBS and DQS cross sections, object-rest/morph/bone ordering, morph-only
evaluation, invalid ownership/weights/channels, collapsed poses and transaction
persistence. The face script is additional process and image evidence.

The mathematical background for rigid DQS is Kavan and colleagues' work on
[dual quaternion skinning](https://users.cs.utah.edu/~ladislav/dq/index.html).
The explicit morph-before-skin order is also described in the
[glTF 2.0 morph-target specification](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#morph-targets).
These are mathematical references; this document does not assert glTF skin/morph
import or export support.
