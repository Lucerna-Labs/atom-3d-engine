# Reference audio and shot-clock checkpoint — September 6, 2026

The transient size-preservation gap identified below has since been fixed and
verified in the [native-project budget checkpoint](PROJECT_BUDGET_VERIFICATION.md).

Reference WAVs, integer-frame shots, rational frame rates, isolated mouth/morph
curve edits and matching image/audio delivery now work through the real editor.
**The full film-production goal remains open.** Lip timing in this checkpoint is
explicitly authored; it does not establish automatic speech alignment or acting quality.

The shared release binary SHA-256 is
`8fb21aa47070218548a909d1691167e0673e891f02419b56f36f9a264ca7b15b`.
It passes **405 workspace tests**, strict workspace/all-targets Clippy and formatting.
The ordinary invocation ignores six optional tests. The independent WAV-reader test
is one of those six and was also run successfully here; the other five retain their
existing opt-in status.

| Boundary | Evidence |
| --- | --- |
| Workspace tests | [Final log](../artifacts/dialogue-workspace-tests-2026-09-06-final.log) |
| Strict Clippy | [Final log](../artifacts/dialogue-clippy-2026-09-06-final.log) |
| Formatting | [Final log](../artifacts/dialogue-format-2026-09-06-final.log) |
| Release build | [Final log](../artifacts/dialogue-release-build-2026-09-06-final.log) |
| Real dialogue authoring, frames, WAVs and cold reload | [Final acceptance](../artifacts/dialogue-foundation-20260906-final/acceptance.json) |
| Independent PCM24/float32/float64/extensible WAV reading | [Reader acceptance](../artifacts/wave-independent-20260906-r1/independent-reader.json) |
| Existing IK and related-cloth workflow | [Regression acceptance](../artifacts/ik-dialogue-regression-20260906/acceptance.json) |
| Existing shaped clothing workflow | [Regression acceptance](../artifacts/pattern-dialogue-regression-20260906/acceptance.json) |
| Existing film/EXR/temporal rendering | [Regression acceptance](../artifacts/film-dialogue-regression-20260906/acceptance.json) |

## Authoring and preservation

Original WAV bytes and their SHA-256 identity remain embedded in the native project.
The parser retains supported PCM and floating-point samples, including finite float
values outside [-1,1]. Export copies the chosen sample bytes and format information;
it performs no resampling or gain conversion. Original ancillary RIFF metadata stays
in the embedded source but is omitted from trimmed exports where its offsets could
be misleading.

Native storage uses compact standard padded base64 and shared immutable buffers.
An 8 MiB source-byte budget occupies about 11 MiB of base64, instead of expanding
every byte into a separately indented JSON array item. Deserialization reconstructs
and validates the decoder and source hash. The referenced filesystem WAV can be
removed after import. Audio range overflow, malformed/replacement sources, invalid
references and removal of a referenced asset fail atomically.

`edit_curve` edits one facial or morph track while preserving the rest of the clip
and document. Tests check the actual resulting mouth cavity and deformed surface,
in addition to exact unrelated-key preservation. See the
[authoring guide](DIALOGUE_SHOTS.md).

## Frame and audio identity

The spoken reference fixture drives two explicitly authored 69-frame shots, at
24/1 and 24000/1001 fps. Integer sample-window calculations retain the phase of the
whole shot when rendering partial ranges. The fractional-rate five-frame slice
contains 3,337 samples; incorrectly restarting the rounding phase would deliver
3,336. The actual exporter preserves the correct bytes and matching image frames.

The final process run renders 153 shot frames and independently checks the input
recording, exact sample queries, waveform statistics, complete/partial WAV slices,
frame manifests and native f32 animation sample times. The mouth-interior material
changes from zero to 173 visible pixels, and the corresponding edit changes 1,918
pixels. A native field probe changes from -0.0435411 to +0.299406 when the mouth
opens. Both unchanged eye objects are fully occluded by the authored blink.

Thirteen rejected requests preserve the document and revision. After the imported
source file is removed and the process restarted, native data, audio observations,
shot schedules, and corresponding PNG/WAV files match exactly. A separate collision
regression proves that a failed audio-sidecar write retains completed image frames
and the conflicting file without installing a completion manifest.

The [detailed acceptance record](DIALOGUE_ACCEPTANCE.md) retains the earlier harness
failure: Python decimal key expectations initially differed from the engine's
documented f32 values. The correction explicitly converts only the expected edited
keys to f32; exact comparisons of the complete remaining document stay in force.

## Remaining production work

This is a reference-audio and authored-timing foundation. Automatic phoneme/viseme
alignment, semantic lip synchronization, production acting and facial anatomy,
audio mixing, final audiovisual container delivery, color management, detailed
surface materials and representative clothed film shots remain open.

A separate P0 preservation gap is visible in the current editor: transient
mutations skip the serialized-project size check that durable commits perform.
The shared audio cap cannot guarantee that a near-limit scene still fits the
64 MiB native-project limit. The next preservation change must reject an unsavable
candidate before acknowledging it and verify recovery at that boundary.

CI includes the dialogue process script on Linux and Windows. This record proves
local Linux execution; remote CI and Windows execution were not observed here.
