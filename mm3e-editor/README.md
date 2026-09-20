# MM3E agent editor

[Joint rotation limits](../docs/JOINT_LIMITS.md) constrain evaluated rigs and support
bounded IK authoring with explicit approximation diagnostics.

[Local speech analysis](../docs/SPEECH_LIP_SYNC.md) can generate editable mouth curves
from recorded audio using an explicit face/morph/joint pose profile.

[Clip layers](../docs/ANIMATION_LAYERS.md) combine independently editable body, facial, morph and camera
performances through the existing animation and shot pipeline.

[Persistent render jobs](../docs/RENDER_JOBS.md) provide frozen scene snapshots, bounded
steps, cancellation and verified reuse of completed frames after process interruption.

A headless editor with typed JSON commands, stable object IDs, reversible transactions and
image/geometry feedback from the actual MM3E engine. Run it on Linux without a window. The
library is reusable by a future visual editor or an agent transport such as MCP.

The editor now also supports [animation](../docs/AGENT_EDITOR_ANIMATION.md): named clips,
object/camera keyframes, rigid joint hierarchies, a humanoid rig helper, time-specific geometry
observations, and frame sequence export. Rig and clip edits use the same transactions and
native project persistence as geometry.

Film-workflow additions include [eyelid and lip controls](../docs/FACIAL_CONTROLS.md),
[continuous surface skinning and morph controls](../docs/DEFORMATION.md),
[limb target authoring](../docs/IK_CONTROLS.md),
[fitted garments and fit checks](../docs/GARMENTS.md), [loose cloth dynamics](../docs/CLOTH_SIMULATION.md),
[sewn panel construction](../docs/SEWN_PANELS.md),
[concave outline and hole patterns](../docs/PATTERNS.md),
[evaluated USD delivery](../docs/USD_DELIVERY.md), [textured USD delivery](../docs/TEXTURED_USD.md), [film shots and EXR channels](../docs/FILM_SHOTS.md),
[reference audio and dialogue shots](../docs/DIALOGUE_SHOTS.md),
[attached UV textures](../docs/UV_TEXTURES.md),
[surface material maps](../docs/MATERIAL_MAPS.md), and
[durable locked projects](../docs/DURABLE_PROJECTS.md). Film production remains an active goal;
see [the release gates](../docs/PRODUCTION_READINESS.md).

Dialogue shots retain an explicit rational frame rate, integer start frame and frame count,
with sample-exact audio windows that keep their original phase during partial exports.
Imported source WAVs are embedded as standard padded base64; up to 16 assets share an 8 MiB
source-byte budget. Shot export writes actual PNG/EXR frames and an exact WAV sidecar when
reference audio is attached. Face and morph curves remain explicitly authored and individually
editable. Automatic speech alignment, voice generation and audio mixing are not implemented.

From the inner workspace root:

```sh
cargo build --release -p mm3e-editor
./target/release/mm3e-editor --root .
```

For work that must survive restart, use
`./target/release/mm3e-editor --root . --project character.mm3e-agent.json`. Native save/project
paths require `.json`, keeping them distinct from render destinations. Omit `--project` for
a transient session.

The root directory must exist. Send one JSON object per stdin line. Each line produces one JSON
response on stdout; stderr is reserved for process errors. Blank lines are ignored. EOF exits.

```json
{"id":"schema","command":{"op":"describe"}}
{"id":"create","expected_revision":0,"command":{"op":"apply","operations":[{"op":"create_humanoid","id":"hero","height":1.8,"build":1.0,"head_scale":1.15}]}}
{"id":"inspect","command":{"op":"inspect","id":"hero/head"}}
{"id":"preview","command":{"op":"render","path":"hero.png"}}
{"id":"save","command":{"op":"save","path":"hero.mm3e-agent.json"}}
```

`describe` returns generated JSON Schemas for requests and documents, runtime limits, supported
semantics and missing capabilities. Unknown properties are rejected. The default camera can
be replaced by `set_camera`; a render's optional `view` override does not edit the document.

| Command | Behavior |
|---|---|
| `inspect` | State, settings and compact objects; optional `id` selects a part; volume samples are summarized |
| `get_document` | Complete supported authoring document, including volume samples |
| `project_budget` | Exact canonical native-project bytes, revision headroom and remaining space without returning scene data |
| `import_texture`, `texture_state`, `export_texture` | Import an explicitly interpreted PNG, sample its linear filtered colors, or recover its original bytes |
| `apply` with `project_uvs` / `put_uvs` / `bind_texture` | Generate planar UVs or supply independent corner charts and attach an albedo texture |
| `uv_state`, `material_state` | Inspect selected UV triangles or actual posed surface UV/color samples |
| `apply` | Atomic list of create/update/delete/translate/camera/settings/lights/humanoid operations; optional `dry_run` |
| `undo`, `redo` | Restore a session snapshot while increasing the revision |
| `sample` | Unpruned scalar values for world-space `points`; optional `id` samples one object before scene-wide CSG |
| `pick` | Cast through image coordinates `x,y`; returns position, normal and material owner when a hit is found |
| `render` | Write display PNG/BMP passes or genuine scene-linear beauty RGB EXR with visibility/timing metrics |
| `validate` | Check document invariants and compilation into an engine scene |
| `pose` | Inspect object transforms, joint pivots and camera at an explicit clip time |
| `solve_ik` | Author verified rotation keys for a direct three-joint chain using a world-space tip target and bend pole; optional dry run |
| `render_sequence` | Export a bounded inclusive time range as PNG/EXR frames and a manifest |
| `import_audio` | Import an embedded reference WAV; requires current revision even for `dry_run`; replacing an asset requires `replace:true` |
| `audio_state` | Inspect audio metadata, bounded waveform bins and exact requested source samples without editing |
| `export_audio` | Write an exact source-sample range as WAV; existing files require `overwrite:true` |
| `shot_state` | Inspect an authored shot's rational clock, absolute frame numbers and source-audio windows |
| `pose_shot` | Evaluate the native pose at one absolute frame of an authored shot |
| `render_shot` | Export all or selected shot frames to a new directory, with a manifest and attached reference-audio WAV slice |
| `apply` with `put_shot` | Create or replace a shot referring to an existing clip and optional embedded audio |
| `apply` with `edit_curve` | Upsert or remove one face/morph track while preserving other tracks, keys and camera data |
| `apply` with `delete_shot` / `remove_audio` | Remove a shot or an audio asset; remaining references must stay valid |
| `face_state` | Inspect neutral or time-sampled eyelid and lip controls |
| `garment_fit` | Measure garment surfaces against scoped evaluated body geometry |
| `bake_cloth` | Atomically bake loose cloth into native triangle caches; requires expected revision |
| `cloth_state` | Inspect sampled cloth vertices, cache freshness and contact/strain diagnostics |
| `deformer_state` | Inspect actual world-space skinned/morphed vertices, controls and bounds |
| `apply` with `bind_surface` / `update_deformer` / `set_morph_weights` | Bind a native surface to joints and morphs, or edit its controls atomically |
| `apply` with `create_sewn_cloth` / `update_sewn_cloth` | Author or replace a garment from explicit panel recipes and ordered stitch chains |
| `preview_pattern_panel` | Triangulate a planar outline with holes; return named control mappings and final boundary indices without editing |
| `apply` with `create_pattern_cloth` / `update_pattern_cloth` | Author or replace cloth from concave outline/hole patterns and explicit seam chains |
| `export_usd` | Write selected composed geometry and camera as a bounded, validated USD mesh cache |
| `save`, `load` | Persist/reopen a versioned native project with all supported geometry embedded |
| `import_obj` | Bake local OBJ into SDF geometry using `id`, `path`, `resolution` (4..96), and positive `padding` |

Every mutation (`apply`, `undo`, `redo`, `load`, `import_obj`, `bake_cloth`, `solve_ik`,
`import_audio`, `import_texture`, including supported dry runs) requires the current `expected_revision`.
Missing or stale revisions fail without changing state. A response
contains `id`, `ok`, current `revision`, and either `result` or a structured `error`. Request IDs
correlate responses; they are not idempotency keys. After an uncertain response, inspect the
state before retrying. Revisions are monotonic within a session; durable `--project` sessions
restore the persisted revision after restart. Transient `load` is a new local mutation.

Transient and durable edits, loads, undo/redo and dry runs share the 64 MiB canonical
native-project budget. Up to 19 bytes are reserved for the revision counter to grow
to 20 digits, keeping admitted snapshots saveable across revision digit changes.
`project_budget` reports `encoded_bytes`, `revision_reserve_bytes`, `limit_bytes`
and `remaining_bytes`. A `project_size_limit` error preserves the current state,
revision, history and existing files. A compact input file below 64 MiB can still
be rejected if its canonical saved representation exceeds this budget; an older
canonical file within the final 19 bytes of the limit can also require reduction.
Rejected input files are preserved unchanged. Ordinary save formatting is unchanged.

A targeted edit and object-scoped observation:

```json
{"id":"edit","expected_revision":1,"command":{"op":"apply","operations":[{"op":"update","id":"hero/chest","patch":{"shape":{"type":"ellipsoid","radii":[0.285,0.234,0.1404]}}}]}}
{"id":"measure","command":{"op":"sample","id":"hero/chest","points":[[0.285,1.287,0]]}}
{"id":"undo","expected_revision":2,"command":{"op":"undo"}}
```

Geometry uses meters, a right-handed Y-up world, and a character facing +Z. Rotations are Euler
degrees using the engine's `Mat3::from_euler` convention. Object order defines the existing
global CSG fold. Group labels do not imply transform parenting or CSG isolation. The first
object must use union. Entity IDs remain stable during edits; callers choose IDs when creating
entities. `translate` rejects duplicate or missing IDs, and a whole batch rolls back on failure.

`set_settings`, `set_camera`, `set_lights`, and nested values in an update patch **replace** their
specified values. Omitted fields inside a replacement structure use its documented/schema
defaults. To preserve all other values, inspect first and supply the complete current structure.
Top-level omitted fields in an entity update patch remain unchanged.

File paths must be relative to `--root`, without `.` or `..` components. Parent directories must
exist. Render/save replacement requires `overwrite:true`; USD export always requires a new
file. Saves stage and flush data before file
installation. Local root checks are not a security sandbox against concurrent filesystem
attackers. Durable `--project` sessions hold an OS writer lock also respected by native saves.
Document load is validated
before state replacement. Undo/redo history remains in the current process, up to 32 snapshots.

Native save/load covers the supported document, including baked SDF samples. This first layer
does not import/export the legacy `.mm3e` text format, whose writer omits volumes. OBJ import
retains the baked shape, not original mesh topology, UVs or material assignments. It uses the
existing engine parser and bake and does not certify the original mesh's topology.

`sample` reports an **authored scalar**, not guaranteed Euclidean signed distance. The engine's
ellipsoid, blended, and sampled fields retain their approximations. Sampling a named object
ignores scene-wide CSG for that observation, so another overlapping part cannot replace it.
Picking reports material ownership; a subtraction keeps the base material and a smooth blend
chooses a dominant material. This is not complete geometric provenance through a CSG tree.

The authoring layer supports all eleven analytic primitives, sampled volumes and bounded local CSG,
union/smooth union/subtraction, transforms, materials, mirror/elongate/round/onion modifiers.
Twist, bend and infinite repeat are not exposed until their contracts and budgets are suitable
for agent editing. The original engine still has those operators. Persistent IK constraints, automatic UV unwrapping,
production retopology, anatomy/aesthetic evaluation and a visual editor UI remain
unimplemented. Evaluated mesh-cache delivery is available through `export_usd` with declared
sampling and geometry limits.
The 19-part humanoid recipe creates editable geometry; `rig_humanoid` separately binds it to
16 joints for rigid part animation. Procedural facial controls and fitted garments extend
it. Authored native surfaces additionally support LBS, rigid DQS and morph deformation.
The evaluated surface retains its hollow-shell field contract; anatomical tissue mechanics
are not inferred. Loose cloth and sewn-panel dynamics
use the separate native cloth solver; they do not establish a calibrated fabric material model.

Rendering is CPU-based in this first layer. `full` is an engine quality preset, not an exactness
guarantee. Center-ray hit coverage, material-owner counts, bounds and step statistics are
diagnostics, not character quality scores. A no-hit result can also mean the trace budget was
exhausted. Render responses carry their actual revision, view, quality, timing and a deterministic
noncryptographic RGBA fingerprint. Readback/diagnostic time is included in `total_ms`.

Budgets now allow 16,777,216 pixels (8192 per axis), 2400 sequence frames and 8,000,000,000
aggregate sequence pixels, written one frame at a time. Set sequence `format:"exr"` for linear
RGB plates. EXR bypasses exposure/tone-map/gamma and records Rec.709/sRGB primaries with D65.
Film settings enable coverage alpha, depth/normal/material channels, depth of field and
sampled shutter motion; see [film shots](../docs/FILM_SHOTS.md). Display-mapped diagnostics
are rejected for EXR.
EXR uses `linear_rgb_fnv1a64`, with a null display `rgba_fnv1a64`.

Run the repeatable, measurement-driven client against the release executable:

```sh
python3 scripts/agent_character_acceptance.py --output artifacts/my-fresh-acceptance-run
```

The output directory must be new. This creates a character, adjusts chest width using engine
measurements, renders multiple views and debug passes, checks rollback/dry run/undo/redo, saves,
restarts the executable, and checks identical saved-state pixels. It records full requests and
responses, intermediate residuals and failures. This validates a deterministic tool client;
it does not benchmark an LLM's planning or anatomical skill.

```sh
cargo test -p mm3e-editor -p mm3e-kit -p mm3e-orchestrator
cargo clippy -p mm3e-editor -p mm3e-orchestrator --all-targets -- -D warnings
```

The broader design and source-backed cross-domain exploration are in
[the first-principles report](../docs/AGENT_EDITOR_FIRST_PRINCIPLES.md).
The [verification record](../docs/AGENT_EDITOR_VALIDATION.md) links the rendered character,
native project, complete request transcript, measured results, and preserved failed runs.
