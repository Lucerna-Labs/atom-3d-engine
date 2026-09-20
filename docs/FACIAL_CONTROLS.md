# Facial controls

The editor creates physical eyelid shells, upper/lower lip curves, a local mouth cavity,
and a recessed interior for an existing humanoid. Controls and animation tracks are native
project data. Frame evaluation preserves the authored head, eyes, document and undo history.

Create the humanoid and optional body rig, then add its face using the current revision:

```json
{"id":"face","expected_revision":1,"command":{"op":"apply","operations":[{"op":"create_face","request":{"id":"hero/face","character":"hero"}}]}}
```

Sources must be `hero/head` (an unmodified ellipsoid) and `hero/left_eye`, `hero/right_eye`
(hard-union spheres). Eyes must share the head's joint binding, or all three must be unbound.
The generator follows current source dimensions and transforms, including supported edits.

| Control | Range | Geometric effect |
|---|---|---|
| `blink_left`, `blink_right` | 0–1 | Shell lids close around unchanged eyeballs |
| `jaw_open` | 0–1 | Open a head-local cavity and separate lip curves |
| `lip_round` | 0–1 | Narrow and protrude the lip outline |
| `lip_wide` | -1–1 | Narrow or widen the mouth |
| `smile` | -1–1 | Lower or raise its corners |
| `gaze_x`, `gaze_y` | -1–1 | Translate both eyes and lids horizontally/vertically in head-local coordinates |
| `brow_left`, `brow_right` | -1–1 | Raise or lower each side's lid cut independently; fade the offset to zero at a full blink |
| `lip_seal` | 0–1 | Reduce cavity/lip aperture; at 1 close it fully while preserving the authored `jaw_open` value |

Set neutral controls with `set_face_controls`. This replaces the control set; omitted values
default to zero. Zero-valued gaze, brow and seal fields are omitted when serializing to
preserve older neutral face data and cache inputs; consumers should apply their documented
zero defaults. Generated-part materials remain editable, while shapes/transforms/labels
are generator-owned and reject direct edits rather than silently discarding them.

```json
{"id":"expression","expected_revision":2,"command":{"op":"apply","operations":[{"op":"set_face_controls","id":"hero/face","controls":{"blink_left":1,"blink_right":1,"jaw_open":0.7,"lip_round":0.5}}]}}
{"id":"state","command":{"op":"face_state"}}
```

Animate controls through `face_tracks` alongside normal transform/camera tracks:

```json
{"id":"blink","expected_revision":3,"command":{"op":"apply","operations":[{"op":"put_clip","clip":{"id":"performance","duration":1,"face_tracks":[{"face":"hero/face","channel":"blink_left","easing":"smooth_step","keys":[{"time":0,"value":0},{"time":0.15,"value":1},{"time":0.3,"value":0}]},{"face":"hero/face","channel":"blink_right","easing":"smooth_step","keys":[{"time":0,"value":0},{"time":0.15,"value":1},{"time":0.3,"value":0}]},{"face":"hero/face","channel":"jaw_open","keys":[{"time":0,"value":0},{"time":0.5,"value":0.8},{"time":1,"value":0}]}]}}]}}
{"id":"frame","command":{"op":"render","path":"performance.png","animation":{"clip":"performance","time":0.15}}}
```

Key times obey the existing interpolation rules, with one track per face/channel. `face_state`
accepts optional `animation:{clip,time,playback}` and reports effective controls and head-local
mouth dimensions. `pose`, `sample`, `pick`, `render` and sequences evaluate the same geometry.

Head motion carries eyes, lids, lips and interior, including direct head-object tracks.
Blinking never changes eye radius, scale, material or a visibility flag. Gaze offsets are
head-local translations and carry through head motion; they do not rotate eyeballs or
move an iris independently. Brow controls move lid cuts, without creating eyebrow geometry,
and fade during blinking so the lids meet at full closure. Mouth subtraction affects only
the head; full lip seal closes the procedural cavity and lip gap without replacing the
authored jaw control. Overlapping independent objects survive. `remove_face` deletes
the definition and seven generated objects, preserving the source head/eyes. Remove
dependent face tracks in the same transaction when required.

This is a procedural facial rig. `jaw_open` controls cavity/lip aperture, not an anatomical
jaw hinge or facial skin solver. Teeth, tongue, weighted skin deformation, corrective anatomy
and automatic audio alignment remain film-production requirements.
Head/cavity ellipsoids retain the engine's documented field approximation. These controls
alone do not establish film-quality anatomy.

The [acceptance script](../scripts/agent_face_acceptance.py) tests actual eye occlusion,
mouth air, independent-object survival, animation, history and reopening. The
[final evidence](../artifacts/agent-face-2026-09-06-final/acceptance.json) includes a
[25-frame preview](../artifacts/agent-face-2026-09-06-final/facial-playback.png).
Visual review caught and corrected phantom mouth-interior streaks; the earlier failed
visual result remains preserved in its original artifact directory.

September 19 regression coverage in `mm3e-editor/tests/facial.rs` checks full seal at
maximum jaw opening, blink closure across both brow extremes, translated neutral eye
coordinates at exact f32 precision, gaze vector mapping under rotation/scale, independent
invalid controls and keys, and legacy default serialization. The original failing cases
are preserved in `artifacts/face-contract-regressions-20260919-before.log`.

All 12 facial regressions pass in `artifacts/face-contract-regressions-20260919-after-r2.log`.
The actual JSONL checks in `artifacts/face-contract-jsonl-20260919-r1.log` verify sealed
head geometry, closed lid geometry, translated neutral eye coordinates and atomic invalid
control rejection; the log records the tested binary hash. This closes these control defects,
not the outstanding film-production delivery gates.

Relevant cloth caches now carry a facial-evaluation version when they depend on the newer
controls. Caches made by the September 10 intermediate build may become stale even at
neutral controls because that build serialized all five new zero fields. Existing cache
frames are retained; rebake affected cloth against the corrected evaluated geometry.

Native `load` and `--project` startup recognize the September 10 generated face format.
Before migrating a rig, every generator-owned field of all seven stored parts must match
either the current generator or the exact September 10 generator. Edited or partially
regenerated parts reject; independent material edits remain intact. Migration preserves
authored controls, source objects, clips and baked cache frames, then applies the ordinary
full document validation. The `load` response lists `migrated_faces`, and its normal atomic
commit and undo/redo rules apply. Startup preserves the saved revision and original file
bytes; corrected geometry is written on the next explicit save or committed edit.

`tests/fixtures/face_controls_legacy_20260910.json` is an authentic native file produced
by release r9, including nonzero facial controls and a baked face-dependent cloth cache.
Its producer transcript and hashes are retained in
`artifacts/face-legacy-migration-20260919-r2`. Compatibility tests cover both read boundaries,
current-file idempotence, source-byte and material preservation, stale retained caches,
and rejection without changing state, history or files. Frozen render-job snapshots do
not migrate: their snapshot and executable hashes retain the original job contract.

The three native compatibility tests pass in
`artifacts/face-legacy-migration-20260919-after-r2.log`; dependency-inclusive strict
Clippy passes in `artifacts/face-legacy-migration-clippy-20260919-r1.log`. A real-process
comparison against the preserved r9 binary passes in
`artifacts/face-legacy-migration-process-20260919-r2/acceptance.json`.
