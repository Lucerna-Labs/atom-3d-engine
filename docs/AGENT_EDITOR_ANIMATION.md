# Agent editor animation

[Clip layers](ANIMATION_LAYERS.md) combine independently editable body, facial, morph and camera
performances through the existing animation and shot pipeline.

The editor can author named motion clips, bind character parts to a joint hierarchy,
inspect any sampled pose, render that pose, and export a timed PNG sequence. The animation
data is part of the native project and participates in atomic edits, undo, redo, save, and
load. Agents control animation through the existing JSON-lines executable.

The [verification record](AGENT_EDITOR_ANIMATION_VALIDATION.md) includes the 104-test result,
real articulated-character acceptance run, native project, frame manifest and playback artifact.

## Run a small animation

Run these commands from the workspace containing `Cargo.toml`. Each input line is one
complete request. Use a fresh output directory when repeating the export.

```sh
cargo build --offline --release -p mm3e-editor
mkdir -p artifacts/animation-quickstart
target/release/mm3e-editor --root artifacts/animation-quickstart <<'JSONL'
{"id":"author","expected_revision":0,"command":{"op":"apply","operations":[{"op":"set_settings","settings":{"width":96,"height":96,"quality":"preview","shadows":false,"ao":false}},{"op":"create","object":{"id":"ball","shape":{"type":"sphere","radius":0.3},"position":[0,0.8,0]}},{"op":"put_clip","clip":{"id":"slide","duration":1,"tracks":[{"target":{"type":"object","id":"ball"},"easing":"smooth_step","keys":[{"time":0},{"time":1,"translation":[1,0,0]}]}]}}]}}
{"id":"midpoint","command":{"op":"pose","animation":{"clip":"slide","time":0.5}}}
{"id":"frames","command":{"op":"render_sequence","request":{"directory":"slide-frames","clip":"slide","start":0,"end":1,"fps":4}}}
JSONL
```

The successful authoring request commits revision 1. The midpoint pose reports the ball
at `[0.5, 0.8, 0]`. The sequence contains five PNGs sampled at 0, 0.25, 0.5, 0.75, and 1
second, plus `slide-frames/manifest.json`. Pose inspection and export keep revision 1.
The `describe` command returns the generated request/document schemas and current limits.

## Authoring and observation commands

Place these authoring operations inside an `apply` request with the current
`expected_revision`. An entire batch either commits or fails without changing authored
state. `dry_run:true` validates the candidate without committing it.

| Operation | Fields and effect |
| --- | --- |
| `put_clip` | `clip`: create or replace a named clip, including all its keys |
| `delete_clip` | `id`: remove an existing clip |
| `set_joints` | `joints`: replace the complete hierarchy and object bindings |
| `rig_humanoid` | `id`: add a hierarchy for an existing named 19-part humanoid blockout |

Deleting an object or joint while retaining a referencing track/binding is rejected.
Update the dependent clips and bindings in the same transaction. `put_clip` replaces the
whole clip; it does not merge individual keys. `rig_humanoid` adds joints and rejects
duplicates through validation; use `set_joints` to replace or adjust an existing rig.

An animation sample has the form:

```json
{"clip":"slide","time":0.5,"playback":"clamp"}
```

| Command | Animation behavior |
| --- | --- |
| `pose` | Required `animation`; returns evaluated object positions, rotation bases, scales, world joint pivots, and camera |
| `sample` | Optional `animation`; evaluates world-space scalar queries at that pose |
| `pick` | Optional `animation`; ray-picks the posed geometry, using its evaluated camera unless `view` overrides it |
| `render` | Optional `animation`; produces a PNG/BMP with posed geometry and camera |
| `render_sequence` | Required `request` containing clip, time range, frame rate, and output directory |

For example, to render a still after authoring the example clip:

```json
{"id":"still","command":{"op":"render","path":"slide-middle.png","animation":{"clip":"slide","time":0.5},"pass":"normal"}}
```

Sampling always starts from the authored rest scene. Calls do not accumulate motion or
advance a shared playback cursor. Requests can sample times out of order. Omitting
`animation` uses the rest scene, even after a posed observation. These observations do not
edit the document, increment its revision, or add undo snapshots; render commands create
their requested files.

`playback` defaults to `clamp`: times below zero hold the clip start and times beyond its
duration hold its end. `loop` uses Euclidean modulo, so negative times wrap and exactly one
duration maps to zero. Sample times must be finite and within -86,400 to 86,400 seconds.
Tracks hold their first or last key outside their own keyed interval within the clip.

## Motion and interpolation

A clip has `id`, `duration`, optional `tracks`, optional `camera_keys`, and optional
`camera_easing`. A motion track has a typed `target`, `keys`, optional `easing`, and an
optional object-only `pivot`:

```json
{
  "target":{"type":"joint","id":"hero/left_shoulder"},
  "easing":"smooth_step",
  "keys":[
    {"time":0},
    {"time":0.5,"rotation_degrees":[0,0,60]},
    {"time":1}
  ]
}
```

Targets are `{"type":"object","id":"..."}` or `{"type":"joint","id":"..."}`.
Each transform key contains a `time` and rest-pose deltas: `translation` defaults to
`[0,0,0]`, `rotation_degrees` to `[0,0,0]`, and uniform `scale` to 1. Omitted values are
these defaults, not values inherited from the preceding key. A clip may have only one
track per typed target.

Translations and scales interpolate numerically. Euler angles use the engine's
`Rx * Ry * Rz` convention, convert to normalized quaternions, and interpolate along the
shortest rotation arc. A pair of 0-degree and 360-degree keys does not encode a full spin;
insert intermediate keys, for example 0, 120, 240, and 360 degrees.

The track's `easing` applies to each segment's normalized time `u` before interpolation:

| Value | Segment behavior |
| --- | --- |
| `linear` (default) | `u` |
| `step` | Hold the left key until the next key's exact time |
| `smooth_step` | `u*u*(3-2*u)` |
| `ease_in` | `u*u` |
| `ease_out` | `1-(1-u)*(1-u)` |

Key times must be finite, strictly increasing, distinct, and inside `[0, duration]`.
Each transform track has 1–1,024 keys. Clip durations are 0.001–3,600 seconds. Transform
key scales are 0.001–1,000. The document supports at most 256 joints, 64 clips, 512 transform
tracks per clip, and 65,536 total transform/camera keys. Unknown JSON properties are rejected.

## Character joints and coordinate conventions

The world remains right-handed, Y-up, measured in meters, with character front along +Z.
A joint contains `id`, optional `parent`, a `pivot` in **world coordinates at rest**, and
an `objects` list. These pivots are not offsets from the parent. The hierarchy must be
acyclic and every referenced parent and object must exist. Each object may bind to at most
one joint. Joint IDs and object IDs belong to distinct typed target domains.

For a rest-world point `x`, a motion delta acts as:

```text
D(x) = pivot + translation + rotation * ((x - pivot) * scale)
posed_joint = posed_parent composed with D_joint
posed_object = posed_binding_joint composed with D_object composed with rest_object
```

Key translations and rotation axes refer to the world at rest, before ancestor motion.
Children inherit their ancestors' motion. Joint tracks use the joint's pivot and reject a
non-null track `pivot`. Object tracks default to the object's rest position; a supplied
track `pivot` is also in rest-world coordinates. Untracked transforms contribute identity
deltas. An object may have its own track while bound to an animated joint.

For an existing `hero` created by `create_humanoid`, bind it with:

```json
{"id":"rig","expected_revision":1,"command":{"op":"apply","operations":[{"op":"rig_humanoid","id":"hero"}]}}
```

Use the actual current revision in a live session. The helper derives pivots from the
current named parts and binds all 19 objects to 16 joints: root, spine, neck, head, and
left/right shoulder, elbow, wrist, hip, knee, and ankle. It preserves the existing geometry.
For a custom character, author the equivalent hierarchy with `set_joints`.

Bindings move complete SDF parts rigidly, with uniform scale. There are no weighted mesh
skin influences or continuous tissue deformation. Smooth CSG blending still combines the
posed fields according to their existing object order. Group labels remain selection
metadata and do not create hierarchy relationships.

## Camera animation and overrides

`camera_keys` use absolute `eye`, `target`, and `fov_degrees` values with a `time`. They use
the document camera's `up` hint and the clip's `camera_easing`; an empty camera-key list
keeps the rest camera. Eye/target positions interpolate componentwise and field of view
interpolates numerically.

`render`, `pick`, and the sequence request accept an optional `view` object with `eye`,
`target`, `up`, and `fov_degrees`. A supplied view selects that observation's camera after
the animated pose is evaluated. It does not edit the camera track or document. A malformed
evaluated camera still fails pose validation before the override is applied.

Render observations report the actual compiled camera: its target is one unit along the
normalized forward direction, so that target's coordinates may differ from the authored
look-at target while describing the same view. Sequence manifests record nullable
`view_override` at the top level and the actual evaluated `view` on each frame.

## Frame sequence output and failures

`render_sequence` takes the nested `request` shown in the quickstart. Required fields are
`directory`, `clip`, `start`, `end`, and `fps`. Optional fields are `playback` (default
`clamp`), `pass` (default `beauty`), and `view`. Passes are `beauty`, `normal`, `depth`, `ao`,
`steps`, and `albedo`.

Times must be finite with `0 <= start <= end`; `fps` must be finite and positive. Frame
indices start at zero, with `scheduled_time = start + index / fps`, stopping when the next
scheduled time exceeds `end`. The range is inclusive when an endpoint lies on that grid:
0–2 seconds at 12 fps yields 25 frames. Equal start/end yields one frame. A non-grid end
does not add an extra irregularly timed frame. Loop playback includes the grid endpoint
too; a sample exactly at the clip duration wraps to its start pose.

Scheduling uses f64 arithmetic from the request's f32 values. The manifest distinguishes
this `scheduled_time` from `time`, the f32 value actually supplied to animation before
clamp/loop mapping. Ranges whose neighboring frame times become identical at f32 precision
are rejected. The exporter accepts at most 240 frames and 32,000,000 total configured
pixels across those frames; the existing per-image size limits also apply.

The directory must be new, relative to the editor root, and have an existing parent.
There is no sequence `overwrite` option. Root confinement rejects traversal and symlinks
that resolve outside the root. Directory creation and each output-file installation refuse
to replace existing entries. These local checks do not provide a sandbox against a hostile
process concurrently changing the filesystem.

All requested poses and budget limits are checked before creating the output directory.
This matters because structurally valid endpoints can yield an invalid intermediate pose:
camera interpolation can make eye and target coincide, and ancestor scale composition can
exceed evaluated-transform limits. `validate` checks authoring invariants and the rest
scene; it does not certify every time on a continuous trajectory. `pose` checks the requested
sample, and sequence preflight checks every sample scheduled for that export.

Success writes `frame_0000.png`, subsequent numbered PNGs, and finally `manifest.json`.
The response includes `manifest_path`, `directory`, `frame_count`, dimensions, pixel count,
and first/last sample times. The manifest records the clip/range/settings, exact timestamps,
final absolute frame paths, actual camera views, and two noncryptographic fingerprints:

- Per-frame `rgba_fnv1a64` hashes rendered RGBA8 pixels, not encoded PNG bytes.
- `document_fnv1a64` hashes `serde_json::to_vec` of the complete authored document.

The manifest is written last. The directory is reserved before rendering; export is not a
single atomic directory replacement. If a render or file write fails after reservation,
the error reports the number of completed PNG frames and identifies the preserved directory,
which may contain partial output. Use a new directory for a retry. A final write can report
an error after installation if directory durability cannot be confirmed; inspect retained
files instead of assuming the directory is empty.

## Native project compatibility and verification

The native project wrapper remains `mm3e-agent-project-v1`, and the document version remains
1. This editor reads earlier version-1 documents with missing `joints` and `clips` as empty
arrays. New saves include the full rig and clips alongside rest geometry, settings, lights,
and camera. No sampled pose replaces the authored rest state. This is compatibility for
reading older projects in the updated editor; older binaries may reject the new fields.
Undo/redo history remains session-local and is not saved.

The focused sequence tests passed on September 6, 2026:

```sh
cargo test --offline -p mm3e-editor sequence::tests -- --nocapture
```

Result: 8 passed, 0 failed. These cover inclusive timing, non-grid/single-frame ranges,
invalid timing and precision, frame and pixel budgets, strict schema defaults, preservation
of existing directories, and symlink-parent confinement. They are not a character-quality
or real-time performance benchmark.

The three quickstart JSON requests were also run against the current debug executable in
a temporary output root: all succeeded at revision 1, the five PNGs had five distinct pixel
fingerprints, the sampled midpoint matched the expected transform, and the manifest contained
the evaluated cameras and full-document fingerprint. The temporary outputs were removed after
verification.

This layer provides direct object motion, hierarchical rigid character posing, camera
animation, and rendered frame sequences. Weighted skinning, inverse kinematics, motion
retargeting, clip blending, anatomy/aesthetic evaluation, audio synchronization, and a visual
timeline are not implemented by these APIs.
