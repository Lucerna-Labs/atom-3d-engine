# Local speech analysis and editable lip animation

The editor can analyze recorded dialogue locally and turn the resulting mouth cues
into editable face, morph and joint animation. It preserves the original WAV and
raw recognizer evidence. Generated clips can be layered with independent body,
blink and camera clips, rendered as ordinary shots, and saved without a continuing
runtime dependency on the speech tool.

This is a first-pass timing workflow. Rhubarb recognizes speech and chooses coarse
mouth shapes; a supplied dialogue string is a recognition hint, **not forced
transcript alignment**. The character profile is an explicit artistic input. Passing
the operational checks does not establish recognition accuracy or final acting quality.
See the [upstream recognizer contract](https://github.com/DanielSWolf/rhubarb-lip-sync#recognizers).

## Backend setup

Bounded runtime analysis is currently implemented on Linux. Native curve creation,
editing and playback remain portable. A full Linux speech package includes Rhubarb
1.14.0, its required resources and complete upstream license notices at
`tools/rhubarb/`; the executable discovers that sibling directory automatically.
Use `--no-speech-backend` to skip discovery when only working with baked curves.
A broken automatically discovered backend is reported by `speech_backend_state`
without preventing unrelated authoring/rendering. An explicitly configured invalid
backend fails startup.

For a source checkout or lightweight package:

```sh
python3 scripts/install_speech_backend.py --destination .dependencies/rhubarb
cargo build --offline --release -p mm3e-editor
target/release/mm3e-editor --root MY_PROJECT --speech-backend .dependencies/rhubarb/rhubarb
```

In a lightweight binary package, the installer and its lock are in `setup/` instead
of `scripts/`. The destination must be new. `--archive FILE.zip` uses a matching
pinned offline archive; `--verify DIRECTORY` verifies an existing installation.
The installer supports Linux/Windows archives, but staging a Windows runtime is
not evidence of bounded Windows analysis support. It never changes system PATH.

Pins are computed from exact official HTTPS release archives and checked per file;
the upstream release did not supply a publisher signature/checksum. These are
reproducibility pins, not independent publisher authentication. The bundled
[upstream license notice](https://github.com/DanielSWolf/rhubarb-lip-sync/blob/v1.14.0/LICENSE.md)
contains the dependency/model notices as well as Rhubarb's MIT license.

The backend path is host startup configuration. Native scene files and JSON
commands cannot select executable code. The adapter starts the executable directly
without a shell, verifies binary/resource identities before and after use, captures
at most 2 MiB each of raw output and status, and kills/reaps the child on timeout or
capture failure. On Linux the direct recognizer child is also terminated if its
owning editor dies, including the startup race; arbitrary descendant processes
are outside this trusted-tool contract. The timeout bounds the recognizer process; preparation and resource
fingerprinting add local I/O time. Version probing has a separate five-second bound. Failed analysis
files remain in their owned directory for inspection.

The package also includes `examples/speech-quickstart.jsonl`. From a fresh extracted
package root, run `bin/mm3e-editor --root . < examples/speech-quickstart.jsonl`.
It creates a procedural face, analyzes the original example voice, maps speech,
adds independent blinks, renders a 29-frame test shot and saves the native project.
The test shot explicitly selects 2.9 seconds of the 2.9335-second source; it does
not claim to export the entire recording. Existing output paths must be kept or
moved before rerunning this example.

## Analyze the original audio

Import an original WAV normally, then analyze a fresh output directory:

```json
{"id":"audio","expected_revision":0,"command":{"op":"import_audio","request":{"id":"voice","path":"voice.wav"}}}
{"id":"backend","command":{"op":"speech_backend_state"}}
{"id":"analysis","command":{"op":"analyze_speech","request":{"audio":"voice","directory":"voice-analysis","recognizer":"english","dialogue_hint":"Hello. We make animated characters.","extended_shapes":"X","timeout_seconds":120}}}
```

Analysis leaves the document revision unchanged. It writes `input.wav`, optional
`dialogue.txt`, `raw.json`, `status.jsonl`, and finally `analysis.json`. The response
provides the analysis path/hash, source/backend identities, normalized cues,
clipped-sample count, analyzed duration and the recognizer's reported duration.
An existing directory is rejected. Failure never publishes a completed analysis
report or changes the authored document.

The full selected source range must contain 0.01–120 seconds. Optional `range` is
`{start_sample,frame_count}` in original simultaneous sample-frame indices. A
multichannel recording requires explicit `channel` selection; mono infers zero.
No channels are silently mixed. The preparation keeps the exact frame count and
sample rate, selects that channel, and quantizes signed PCM16 for the recognizer.
Finite float samples outside [-1,1] are clipped **only in this derived input** and
counted. Original source WAV bytes and later shot exports remain unchanged.

Use `recognizer:"phonetic"` for the backend's non-word-recognition mode. Dialogue
hints are accepted only with English recognition. `extended_shapes` selects a
nonrepeating subset of `GHX`, default `X`; the basic A–F shapes remain enabled.
Rhubarb's model has no confidence field in this output; the editor does not invent one.

Cues are contiguous half-open intervals in integer centiseconds (`start_cs`,
`end_cs`, `shape`). They must agree exactly with preserved raw JSON. The reported
end is the original selected duration truncated to 0.01 s. An unreported tail is
less than 10 ms and is exposed separately. Source sample boundaries can be located
as `range.start_sample + floor(cue_cs * sample_rate / 100)`; cue precision is still
centiseconds, not measured sample-accurate phoneme timing.

## Map cues onto a character

Each generated clip needs a `profile` describing actual poses for every used cue.
The included `examples/lip-sync-profile.json` supplies a simple A–F/X profile for
a procedural face rig named `face`. It is a calibration example, not a measured
human mouth model. Replace target IDs and pose values for your own character.

The profile format is `{"poses":[{"shape":"A","values":[...]}]}`. Each value is one
of the following types:

```json
{"type":"face","face":"face","channel":"jaw_open","value":0}
{"type":"morph","deformer":"head-skin","blendshape":"lip_seal","value":1}
{"type":"joint","joint":"jaw","rotation_degrees":[0,0,0],"translation":[0,0,0],"scale":1}
```

The face profile may also include the optional `gaze_x`, `gaze_y`, `brow_left`,
`brow_right` and `lip_seal` controls when a performance needs eye aim, asymmetric
brow action or a sealed consonant. Their ranges and head-local behavior are
validated by the same native face contract.

Facial and morph values must respect their real declared ranges; joints use native
world-rest pivots. Pose entries are sparse: omitted entries in the union of mapped
channels use the captured authored defaults. Missing used mouth shapes and duplicate
shape/channel entries are errors. Extended G/H shapes need their own calibrated
teeth/tongue poses if enabled. Shape descriptions are in the
[upstream mouth-shape guide](https://github.com/DanielSWolf/rhubarb-lip-sync#mouth-shapes).

Send `generate_lip_sync` with these request fields:

| Field | Meaning |
| --- | --- |
| `analysis_path` | Root-relative `analysis.json` from the previous step. |
| `analysis_sha256` | Optional exact file SHA from analysis; recommended to seal the handoff against changed bytes. |
| `clip` | Destination clip ID. |
| `profile` | Full pose dictionary described above. |
| `transition_seconds` | Maximum transition width, default 0.04; zero uses Step keys. |
| `replace` | Defaults false; replacing an existing clip must be explicit. |

The command requires `expected_revision` and accepts top-level `dry_run`. Preview
computes and validates the actual candidate without changing the document or writing
new analysis files. Success produces ordinary transform, facial and morph tracks
in a dedicated clip; other clips remain untouched. `replace:true` replaces the full
destination clip, so retain separate body/blink layers when regenerating speech.

Before generation, the editor verifies source identity, range, selected channel and
recomputed analysis PCM hash/clipped count. The same bounded read checks the optional
exact report SHA. Arbitrary files cannot silently substitute a different source range.
These checks detect accidental changes; they do not authenticate a report against a
same-filesystem actor who intentionally rewrites all its digests.

## Timing, editing and history

Positive-width transitions occur **before** each cue boundary, reaching the new
pose at the boundary. Width is limited to half both adjacent cue intervals so a
short closed-mouth cue keeps its full pose. Duplicate points are removed only from
exactly constant segments; there is no lossy decimation to evade a key budget.
Each native channel remains limited to 1,024 keys and all animation shares the
existing document budget. Choose a shorter analysis range if a dense result exceeds it.

Keys use the native f32 clock. Distinct times that collapse are rejected; inspection
reports cue-boundary rounding. The final pose is held through the exact selected
source duration, rounding the clip endpoint upward by at most one f32 ULP. The
unreported sub-centisecond tail is held, not relabeled as silence.

Use ordinary `edit_curve`, `put_clip`, `edit_layer`, IK and history operations to
revise the generated performance. `lip_sync_state {clip}` reports whether generated
channels/duration were edited and whether the original audio is still available.
Historical report/profile/defaults/settings are embedded under `clip.lip_sync` with
an integrity digest. Manual curve edits preserve this history; changed provenance
without an updated digest is rejected. Source removal or missing external reports
never prevents playback of valid baked curves. Metadata-only provenance changes
are excluded from physical cloth dependencies and evaluated-pose input signatures.

To combine speech with acting and blinks, use an assembly clip's layers as described
in [ANIMATION_LAYERS.md](ANIMATION_LAYERS.md). To deliver recorded audio with frames,
use the existing [dialogue-shot clock](DIALOGUE_SHOTS.md) and exact WAV sidecar export.
Audio placement must match the analyzed source range; creating a clip does not
implicitly create a shot or invent padding, resampling or a video mux.

## Verification and remaining limits

Local checks include real Rhubarb analysis of an original generated voice fixture,
an identical-length silence control using the same recognizer and hint, independent
stereo/range PCM checks, cue-boundary native geometry, body/blink isolation, editing,
29 rendered shot frames, exact selected original PCM and backend-free cold reload.
Separate tests exercise process deadlines/output caps, malformed reports, input
changes, metadata corruption, native key budgets and short closure transitions.

The voice fixture is synthetic. These are operational and data-integrity checks;
representative human speech, languages, accents, noisy dialogue and film acting
still need quality evaluation. The workflow does not supply forced word alignment,
automatic character pose calibration, voice generation, or final animation approval.
