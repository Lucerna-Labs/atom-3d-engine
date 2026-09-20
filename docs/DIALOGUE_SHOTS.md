# Reference audio and frame-indexed dialogue shots

[Clip layers](ANIMATION_LAYERS.md) combine independently editable body, facial, morph and camera
performances through the existing animation and shot pipeline.

The editor can embed a real WAV reference, bind it to a shot with a rational frame
rate, inspect exact audio samples and waveform bins, edit one facial or morph
curve, and export native image frames with an exact WAV sidecar. The shot clock
connects frame numbers, animation seconds and source audio sample frames.

Lip timing can be authored explicitly or generated through the optional
[local speech-analysis workflow](SPEECH_LIP_SYNC.md). Generated mouth cues are not
forced transcript alignment. The editor does not generate voices, mix tracks,
change gain, resample audio, or pad missing samples. A waveform envelope describes
amplitude; it does not identify speech sounds.

## Embedded source audio

`import_audio` reads a WAV path relative to the editor root. It validates the
complete source before committing. `AudioAsset.data` stores **standard padded
base64 of the original WAV bytes**, including original RIFF metadata. It is a
JSON string, not a numeric byte array. Saving and loading reconstructs the
validated decoder and SHA-256 fingerprint from those bytes. Import does not leave
a dependency on the source filesystem path. History snapshots share immutable
audio storage.

`source_sha256` hashes the entire original WAV file. A sliced export has its own
`sha256`: the selected sample bytes and original format descriptor are retained,
while unrelated ancillary RIFF metadata is omitted and required frame-count
metadata is rebuilt. Original cue positions or sample loops are not copied into
a trimmed file as if they still described the new range.

| Accepted input | Details |
| --- | --- |
| Container | Complete little-endian RIFF/WAVE |
| Integer PCM | 8-, 16-, 24- or 32-bit containers; PCM8 is unsigned offset binary, other PCM widths are signed |
| IEEE float | 32- or 64-bit finite samples; values outside `[-1,1]` remain intact |
| Extensible format | Supported 40-byte PCM/float descriptors; valid-bit counts and channel mask are retained; unused low PCM bits must be zero |
| Sample rate | 8,000–192,000 Hz |
| Channels | 1–8 |
| Source-byte budget | 8 MiB total across all embedded audio assets, including source headers and ancillary chunks |
| Asset count | At most 16 |
| Parser limits | At most 1,024 top-level RIFF chunks and 23,040,000 sample frames per source, also subject to the shared byte cap |

The source-byte budget excludes base64 expansion: 8 MiB of source bytes occupies
about 11 MiB of base64 text. The sample-frame cap is a count, not a universal
120-second duration limit. Compressed encodings, RF64/BW64, big-endian RIFX,
segmented `wavl`/`slnt`, unknown format extensions, nonfinite float data, malformed
chunk lengths, duplicate format/data/fact chunks and inconsistent alignment are
rejected. See the implemented [WAV contract](../mm3e-kit/src/wave.rs).

A **sample frame** is one simultaneous sample for every channel. `start_sample`
and `frame_count` always count these interleaved frames, not individual channel
scalars or bytes.

## Quickstart with the checked-in speech reference

The binary package also includes this recording as `examples/dialogue-reference.wav`.
When running from an extracted package with that directory inside `--root`, use
that path in the import requests below.

From the inner workspace root, start a fresh transient editor:

```sh
cargo build --offline --release -p mm3e-editor
./target/release/mm3e-editor --root .
```

Send one complete JSON object per line. These requests assume a fresh document
at revision 0 and new output paths. If continuing another session, inspect its
actual revision and use that value for each mutation.

The checked-in fixture says “Hello. We make animated characters.” It was generated
once from original text using Flite; its
[provenance](../mm3e-editor/tests/fixtures/dialogue/provenance.json) records that
process. The editor and these requests consume the saved WAV and have no TTS
runtime dependency. The source is mono PCM16, 16,000 Hz, 46,936 sample frames.

First validate the import without committing, then import the same bytes. Both
requests require the current revision; the dry run leaves it at 0.

```json
{"id":"audio-preview","expected_revision":0,"command":{"op":"import_audio","request":{"id":"voice","label":"Reference dialogue","path":"mm3e-editor/tests/fixtures/dialogue/hello-reference.wav"},"dry_run":true}}
{"id":"audio-import","expected_revision":0,"command":{"op":"import_audio","request":{"id":"voice","label":"Reference dialogue","path":"mm3e-editor/tests/fixtures/dialogue/hello-reference.wav"}}}
```

The import response reports decoded metadata, `source_sha256` and `committed`.
Reusing an asset ID requires `request.replace:true`; replacement also validates
dependent shots, so shortening audio beyond an existing shot's range fails
atomically. Use the usual `undo`/`redo` after a committed import or replacement.

Create a small procedural face, a three-second clip and a 69-frame shot. These
facial controls demonstrate the shot API; the separate
[native surface deformation guide](DEFORMATION.md) covers continuous skin/morph
geometry. The blink keys below are manual authoring choices.

```json
{"id":"setup","expected_revision":1,"command":{"op":"apply","operations":[{"op":"create","object":{"id":"actor/head","shape":{"type":"ellipsoid","radii":[0.8,1,0.8]},"material":{"albedo":[0.65,0.4,0.26],"roughness":0.7}}},{"op":"create","object":{"id":"actor/left_eye","position":[0.28,0.2,0.78],"shape":{"type":"sphere","radius":0.15}}},{"op":"create","object":{"id":"actor/right_eye","position":[-0.28,0.2,0.78],"shape":{"type":"sphere","radius":0.15}}},{"op":"create_face","request":{"id":"face","character":"actor"}},{"op":"put_clip","clip":{"id":"performance","duration":3,"face_tracks":[{"face":"face","channel":"blink_left","keys":[{"time":0,"value":0},{"time":0.5,"value":0},{"time":0.6,"value":1},{"time":0.75,"value":0},{"time":3,"value":0}]}]}},{"op":"put_shot","shot":{"id":"line-1","clip":"performance","rate":{"numerator":24000,"denominator":1001},"start_frame":1001,"frame_count":69,"clip_frame_zero":1001,"audio":{"asset":"voice","start_sample":317}}},{"op":"set_camera","camera":{"eye":[0,0,4],"target":[0,0,0],"fov_degrees":38}},{"op":"set_settings","settings":{"width":128,"height":128,"quality":"preview","shadows":false,"ao":false}}]}}
```

`put_shot` creates or replaces one named shot. It leaves its referenced clip and
audio asset intact. The frame rate is the ratio `numerator / denominator`, not a
rounded decimal FPS. Read-only observations do not change the revision:

```json
{"id":"audio-detail","command":{"op":"audio_state","request":{"id":"voice","range":{"start_sample":317,"frame_count":16000},"bins":16,"samples":[317,8317,16317]}}}
{"id":"shot-detail","command":{"op":"shot_state","id":"line-1"}}
{"id":"frame-pose","command":{"op":"pose_shot","shot":"line-1","frame":1013}}
```

`audio_state` returns source metadata plus per-channel min/max/RMS bins and
optional exact sample values. Integer PCM observations are normalized to `[-1,1)`;
float observations retain their finite source values. Bins partition the requested
range without gaps. Explicit sample queries use absolute source indices and are
independent of that waveform range. The range defaults to the whole source;
`bins` defaults to 64 and supports 1–2,048, with at most 4,096 explicit sample
indices. Bin count never exceeds the number of sample frames in the range.

## Edit a single lip curve

An `edit_curve` operation replaces the matching track in place or appends a new
channel. Removing a missing track is an error. All other curves, their order,
camera keys, clips, geometry and audio remain unchanged. Existing clip validation
enforces key order, ranges and reference integrity before commit.

This manual jaw curve opens at clip time 0.5 seconds. Since the shot starts at clip
zero and source audio sample 317, that time corresponds to source sample
`317 + 0.5 * 16000 = 8317`. This relation is an authored timing anchor, not an
inferred word or phoneme boundary.

```json
{"id":"jaw-curve","expected_revision":2,"command":{"op":"apply","operations":[{"op":"edit_curve","request":{"clip":"performance","edit":{"op":"upsert_face","track":{"face":"face","channel":"jaw_open","easing":"linear","keys":[{"time":0,"value":0},{"time":0.5,"value":1},{"time":1,"value":0},{"time":3,"value":0}]}}}}]}}
```

| `edit.op` | Required fields |
| --- | --- |
| `upsert_face` | `track: {face, channel, keys: [{time, value}], easing}` |
| `remove_face` | `face`, `channel` |
| `upsert_morph` | `track: {deformer, blendshape, keys: [{time, weight}], easing}` |
| `remove_morph` | `deformer`, `blendshape` |

The outer request always names `clip`. `easing` defaults to linear and also
supports step, smooth step, ease in and ease out. Track keys still use native clip
seconds. An active curve overrides the corresponding default facial/morph control;
it does not add its weight to the default. `put_clip` remains a full clip
replacement, so use `edit_curve` when revising one facial or morph performance
channel. This curve-edit API does not itself perform clip blending, automatic retiming, or a cue
recognition system.

## Exact frame and audio clocks

A shot contains `id`, `clip`, `rate`, `start_frame`, `frame_count`,
`clip_frame_zero`, and optional `audio: {asset, start_sample}`. Frame numbers may be
negative or large integers. Clip time is computed from the exact integer
difference first:

```text
scheduled_time = (frame - clip_frame_zero) * rate.denominator / rate.numerator
sampled_time   = scheduled_time converted to native f32 animation time
```

The range is half-open: `[start_frame, start_frame + frame_count)`. Exactly
`frame_count` image frames are scheduled. The whole interval, including its
exclusive end, must fit the clip. Endpoint comparisons use the existing f32 clip
precision without an arbitrary epsilon or clamping; the computed f64 schedule is
retained in metadata. This accepts a rational 0.7-second endpoint against the
authored `0.7f32` duration, while rejecting a whole extra frame. Integer overflow,
out-of-range times and indistinguishable f32 samples are errors.

For whole-shot index `i`, source sample rate `S`, and audio in-point `A`, the
audio window is:

```text
[ A + floor(i       * S * rate.denominator / rate.numerator),
  A + floor((i + 1) * S * rate.denominator / rate.numerator) )
```

These integer windows are adjacent even when samples per video frame are
fractional. Every window must lie inside the embedded source. `shot_state`
reports absolute frame numbers, whole-shot indices, scheduled f64 and sampled
f32 times, audio windows, the overall audio range, source metadata, the floor
policy, and the fractional remainder at the final audio boundary. The remainder's
denominator is `rate.numerator`.

Partial selections keep the **original whole-shot index and floor phase**. In the
example, absolute frame 1003 is shot index 2. Its audio window begins at source
sample 1651 and has 668 samples. Selecting four frames starting there takes
`[1651,4321)`, or 2670 source sample frames. The floor phase is not restarted at
zero for the selected range.

There are at most 64 shots per document and 1–2,400 frames per shot. A rate must
be positive and between 0.001 and 240 FPS. Existing image, sequence-pixel and
render-sample budgets also apply. The older `render_sequence` API keeps its
documented inclusive seconds-based range; its endpoint convention is unchanged.

## Render images and the WAV sidecar

For bounded steps, cancellation and recovery after process interruption, use
[persistent render jobs](RENDER_JOBS.md) with `type:"shot"`. The synchronous exports
below retain their original behavior.

Both exports below are read-only with respect to the authoring document. They
create new output directories; existing destinations are rejected. The second
export is the four-frame partial selection described above.

```json
{"id":"full-shot","command":{"op":"render_shot","request":{"shot":"line-1","directory":"dialogue-full","format":"png"}}}
{"id":"partial-shot","command":{"op":"render_shot","request":{"shot":"line-1","directory":"dialogue-part","selection":{"start_frame":1003,"frame_count":4},"format":"png"}}}
{"id":"audio-slice","command":{"op":"export_audio","request":{"id":"voice","path":"voice-one-second.wav","range":{"start_sample":317,"frame_count":16000}}}}
{"id":"save","command":{"op":"save","path":"dialogue-project.json"}}
```

`render_shot` uses the actual scene renderer and accepts the existing PNG/EXR
formats, optional `view`, and `pass`. EXR and film-settings restrictions still
apply; see [film shots](FILM_SHOTS.md). With an audio placement it writes
`audio.wav`, containing precisely the selected source sample bytes, plus the
image sequence and `manifest.json`. A shot without audio writes images and a null
audio entry. A selected audio range with no source samples is rejected.

The manifest is written last. It identifies the shot, selected range and exact
rational rate. Use `shot.rate` as the authoritative rate; the inherited decimal
`fps` field is a convenience value. Each image entry retains its local output index, original
`shot_index`, absolute `shot_frame`, scheduled and sampled times, and
`audio_window`. Audio metadata records source/output hashes, sample range, rate
and channel count. The output is an image/WAV bundle; the editor does not mux a
video or mix audio. Failures preserve any already written files and report partial
output, without publishing a completed manifest.

`export_audio` writes a nonempty source-frame range as WAV independently of a
shot. Existing files require `request.overwrite:true`. It preserves sample bytes
and the validated source format, with the sliced-container metadata contract
described above. Exporting or inspecting audio never changes the document revision.

Save/load includes all original embedded audio. To reopen the example in a fresh
transient editor rooted at the same directory:

```json
{"id":"load","expected_revision":0,"command":{"op":"load","path":"dialogue-project.json"}}
{"id":"reopened-shot","command":{"op":"shot_state","id":"line-1"}}
```

The external input WAV is not needed after import. `apply` with `delete_shot`
removes a shot; `remove_audio` removes an asset. Removing referenced audio or a
referenced clip fails unless the dependent shots are also removed or updated
within the transaction. Native persistence, revision conflicts and root-relative
path rules are the same as other authoring operations.

## Verification scope

The [WAV tests](../mm3e-kit/tests/wave.rs) validate supported encodings, exact slice
bytes, malformed input and resource bounds. Editor tests cover
[embedded audio](../mm3e-editor/tests/audio.rs),
[rational clocks and partial sample windows](../mm3e-editor/tests/timeline.rs), and
[isolated facial/morph edits](../mm3e-editor/tests/curve_edit.rs).
The real-process harness is
[`agent_dialogue_acceptance.py`](../scripts/agent_dialogue_acceptance.py). It uses
the checked-in spoken fixture and explicitly authored sample cues, an independent
WAV reader, actual mouth field/image probes, full and partial frame exports, and
cold reload. Its recorded status and binary identity determine which process
results have passed; manually keyed mouth motion does not establish automatic
speech alignment or acting quality.

The [recorded spoken-reference acceptance](DIALOGUE_ACCEPTANCE.md) passed on
binary SHA-256
`8fb21aa47070218548a909d1691167e0673e891f02419b56f36f9a264ca7b15b`.
The [separate guide-example validation](../artifacts/dialogue-guide-examples-20260906-r1/validation.json)
ran all 13 JSON requests above verbatim against that executable. It verified the
69-frame export, the four-frame partial export and its `[1651,4321)` audio slice
using Python's independent WAV reader, then loaded the saved project after
removing the staged input WAV. The
[request/response transcript](../artifacts/dialogue-guide-examples-20260906-r1/transcript.jsonl)
is retained with those outputs.
