# Composable character animation

Clips can now assemble independent body, blink, mouth, morph and camera animation
through ordered `layers`. Source clips remain editable assets; composing them does
not copy or bake their keys. Layered clips use the existing pose, deformation,
render, dialogue-shot, cloth-bake and persistent-render-job paths.

A clip evaluates its layers in listed order, then applies its own authored channels
as final overrides. This makes direct curve and IK edits authoritative. To blend
an existing body performance with additional motion on the same channel, put the
body clip in the first layer of a new assembly clip instead of duplicating its
tracks into the assembly.

## Author and revise

From an editor with a `ball` object, this complete clip example combines an
independent motion with an additive offset. Use existing character joint/morph
channels in the same way.

```json
{"id":"clips","expected_revision":1,"command":{"op":"apply","operations":[{"op":"put_clip","clip":{"id":"walk","duration":2,"tracks":[{"target":{"type":"object","id":"ball"},"keys":[{"time":0},{"time":2,"translation":[1,0,0]}]}]}},{"op":"put_clip","clip":{"id":"lift","duration":1,"tracks":[{"target":{"type":"object","id":"ball"},"keys":[{"time":0},{"time":1,"translation":[0,0.2,0]}]}]}},{"op":"put_clip","clip":{"id":"take","duration":2,"layers":[{"id":"body","clip":"walk"},{"id":"accent","clip":"lift","mode":"additive","start":0.5,"end":1.5,"weight":0.5}]}}]}}
{"id":"pose","command":{"op":"pose","animation":{"clip":"take","time":1}}}
{"id":"revise","expected_revision":2,"command":{"op":"apply","operations":[{"op":"edit_layer","request":{"clip":"take","action":{"op":"upsert","layer":{"id":"accent","clip":"lift","mode":"additive","start":0.5,"end":1.5,"weight":0.8}}}}]}}
```

Revision numbers above assume the object was authored in revision 1. Use the actual
revision returned by your editor. `edit_layer` runs inside the normal atomic Apply
transaction with dry-run and undo/redo. `upsert` replaces a matching layer in place
or appends a new ID. `remove` takes `id`; `move` takes `id` and the zero-based final
`index`. Other clips, layers and locally authored channels remain intact. Reordering
changes composition and can change the pose.

`pose` reports `layer_samples` for layered clips, including parent/source clocks,
weight and active contribution. Static graph validation still checks missing
references and cycles in disabled, zero-weight and masked-out layers. Deleting a
referenced clip requires repairing its incoming references; both edits can be in
one atomic batch.

## Layer fields

| Field | Meaning |
| --- | --- |
| `id`, `clip` | Unique layer ID within its parent and source clip ID. |
| `start`, `end` | Inclusive active window in parent-clip seconds. Start defaults to 0; omitted end uses the parent duration. |
| `source_start`, `time_scale` | Source clock offset and positive speed, default 0 and 1. Speed range is 0.001–1000. |
| `playback` | Source `clamp` or `loop`, default clamp. |
| `weight` | Constant factor in 0–1, default 1. |
| `weight_keys`, `weight_easing` | Optional `{time,value}` keys in parent-clip time, each value in 0–1; their sampled value multiplies the constant factor. |
| `mode` | `override` (default) or `additive`. |
| `reference_time` | Optional source-clip time for additive reference pose; omitted means authored rest defaults. Invalid on override layers. |
| `mask` | Optional list of channels. Omitted means all authored source channels; an empty list means none. |
| `enabled` | Defaults true. Disabling does not delete its sources or keys. |

Source time is `(parent_time - start) * time_scale + source_start`, evaluated in
f64 from the actual parent f32 time, then clamped or looped to source duration and
converted to the native f32 sample. Negative source offsets work with both playback
modes. Each nested clip has its own clock. The root retains the original clamp/loop
and rational-shot scheduling contract. A layer's inclusive window may produce a
hard boundary; author weight keys for an intentional fade.

Masks select whole sparse channels:

```json
[
  {"type":"joint","id":"hero/head"},
  {"type":"object","id":"prop"},
  {"type":"face","face":"hero-face","channel":"blink_left"},
  {"type":"morph","deformer":"head-skin","blendshape":"mouth_open"},
  {"type":"camera"}
]
```

A joint/object channel contains its full translation, rotation and uniform scale.
An absent source channel has no influence on previous channels. A joint mask does
not freeze descendants: ordinary parent motion still carries child joints and
bound geometry. Per-axis masks, retargeting and nonuniform scale are not added here.

## Blend contract

Override interpolates canonical-rest-anchor translation and uniform scale, with
shortest-path quaternion interpolation for rotation. Joint anchors are the existing
world-rest pivots; object anchors are their rest positions. Explicit object-track
pivots remain part of the source delta. Blending sources with different pivots
blends the target anchor positions; it does not invent an interpolated pivot.

Additive uses the source minus its reference for anchored translation and scalar
channels. With unit quaternions, `delta = inverse(reference) * source`, followed by
`result = base * slerp(identity, delta, weight)`. This is a local rotational delta;
order matters. Uniform scale is `base * ((1-weight) + weight * source/reference)`.
Ratio and scalar arithmetic use f64 intermediates before the native f32 result to
avoid cancellation across valid scale ranges. Exact zero/full-weight and equal-pose
cases bypass unnecessary interpolation.

Face and morph channels use their authored defaults when a prior/reference channel
is absent. Final values must remain inside the channel's declared range; combined
values are rejected instead of silently clamped. Camera eye/target/FOV values blend
numerically using the rest camera as the default reference; up remains the existing
rest hint. A collapsed eye/target or invalid FOV is rejected before output.

Locally authored final channels and masked-out channels are excluded from inherited
value evaluation. This keeps unused overflowing layers from defeating a valid local
override. Source graphs remain statically validated. Animation reuse in deformation
also checks clip, time, authored-input signature and a private payload signature;
changing public evaluation times or joint transforms invalidates reuse.

Masks and override/additive modes are also described in the
[Unity animation-layer manual](https://docs.unity3d.com/6000.0/Documentation/Manual/AnimationLayers.html)
and [Unreal animation-sequence documentation](https://dev.epicgames.com/documentation/en-us/unreal-engine/animation-sequence-editor-in-unreal-engine).
MM3E's sparse channels, world-rest anchors, local-override priority and reference
rules above are its explicit native contract, not a claim of format compatibility.

## Cloth, IK and persistence

Cloth baking projects the complete reachable layer graph onto its collision/pin
geometry dependencies, including relevant joint ancestors and face/morph inputs.
Contributing child motion, clocks, weights, references and order participate in
cache freshness. Unrelated camera/facial branches are removed from the physical
projection. Facial animation still matters when a selected collider or pin depends
on it. Physical edits preserve the old cache as stale evidence and require rebaking
before playback; they do not silently reuse it or erase it.

IK writes direct keys in the destination clip and preserves sampled translation
and scale inherited from its layers. Source clips remain unchanged. The existing
solver's joint-limit and constraint limitations still apply.

Native version-1 files omit empty `layers`, retaining old flat-clip encodings and
verified v3/v4 cloth-cache fingerprints. Snapshots/jobs freeze the entire graph.
There are at most 16 layers per clip, graph depth 8 and 1,024 expanded clip
sampling calls, including additive reference samples. Layer weight keys share the
existing 65,536-key document budget. These are finite structural limits, not a
performance guarantee for every legal scene.

## Evidence and remaining scope

Integration tests cover actual object/joint poses, face/morph fields and vertices,
camera pixels, exact endpoints, clocks/masks, sparse edits/history, additive scale
range, changed evaluation payloads, physical cloth contact and old cache formats.
Before-fix logs retain both the additive-scale cancellation and shadowed-overflow
failures. Real-process acceptance separately checks split character performances,
source edits, cold reload and layered cloth rebaking/render jobs.

This adds animation composition. It does not provide automatic speech alignment,
anatomical rig limits, a state-machine editor, retargeting or film-quality acting
approval. Full production readiness remains unproven.
