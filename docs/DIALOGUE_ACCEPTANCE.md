# Reference-audio and dialogue-shot acceptance

The actual JSONL harness is
[`scripts/agent_dialogue_acceptance.py`](../scripts/agent_dialogue_acceptance.py).
It uses the checked-in original spoken fixture
[`hello-reference.wav`](../mm3e-editor/tests/fixtures/dialogue/hello-reference.wav)
and its [provenance](../mm3e-editor/tests/fixtures/dialogue/provenance.json).
The recording says “Hello. We make animated characters.” It contains 46,936 mono
PCM16 sample frames at 16,000 Hz. Its source SHA-256 is
`c17de07edc5c105d80b880be27ffdef8f60011e7778d8bf09101b7e37c550f57`.
The fixture was generated locally using the installed CMU Flite voice; running
this harness requires no speech-generation service, Flite installation, or network.

This acceptance covers embedded reference audio, explicit rational shot timing,
manual mouth curves, actual native facial geometry, and matching image/audio
delivery. It does not demonstrate automatic phoneme recognition or alignment,
semantic lip synchronization, acting quality, anatomical facial deformation,
audio mixing, or an audio/video container encoder. The visual fixture uses the
implemented procedural SDF head, eyelids, lips and mouth cavity.

## Reproduce

From the inner workspace root:

```sh
cargo build --offline --release -p mm3e-editor
python3 -B scripts/agent_dialogue_acceptance.py \
  --editor target/release/mm3e-editor \
  --output artifacts/dialogue-acceptance-NEW
```

The output directory must be new. The harness stages an exact executable copy
inside that directory, so its authoring and cold-reload processes use the same
binary even if another build runs concurrently. It retains the binary identity,
every dispatched JSON request and received response, stderr, native project,
PCM exports, PNG frames, manual cues, and failures. The checked-in source recording
is never deleted; only the imported copy inside the new artifact directory is
removed for the reload test.

Python's standard `wave` reader independently decodes exported WAV files. Exact
PCM bytes and queried sample values are compared with the original recording;
waveform minima, maxima and RMS values are recomputed independently. PNGs are
decoded with the existing independent CRC-checked decoder supporting all five
scanline filters. Frame comparisons use both pixel checks and encoded-byte equality.

## Explicit timing contract exercised

Both shots begin at absolute frame 1001, use that same frame as clip time zero,
contain exactly 69 frames, and place source sample 317 at shot index zero.
The clip duration is three seconds. The tested frame rates are `24/1` and
`24000/1001`, with integer arithmetic determining each audio window:

```text
start_sample(i) = 317 + floor(i * 16000 * denominator / numerator)
end_sample(i)   = 317 + floor((i + 1) * 16000 * denominator / numerator)
```

The harness checks every complete-shot boundary for adjacency, every scheduled
clip time, the exact f32 sample sent to animation, and `pose_shot` agreement with
the ordinary pose evaluator. Each partial selection starts at absolute frame
1002 and includes five frames. This is complete-shot index one, so it exposes a
one-sample error if an exporter resets the audio floor phase at a partial range.

| Rate / selection | Source start | Source end, exclusive | PCM sample frames |
| --- | ---: | ---: | ---: |
| `24/1`, complete shot | 317 | 46,317 | 46,000 |
| `24/1`, five-frame partial | 983 | 4,317 | 3,334 |
| `24000/1001`, complete shot | 317 | 46,363 | 46,046 |
| `24000/1001`, five-frame partial | 984 | 4,321 | 3,337 |

The image-sequence manifests retain complete-shot frame indices and corresponding
source sample windows. Every partial image must equal the matching complete-shot
image; `audio.wav` must contain exactly the independently calculated source byte
slice. No resampling or padding is permitted by these checks.

Manual cue anchors are source samples `317`, `8317`, `16317`, `24317`, `36317`,
`44317`, and `46317`. Their jaw and lip-round values are explicit fixture data,
recorded in `manual-cues.json` and `authored-curves.json`. They are not labeled
phonemes or claimed word boundaries. Sparse `edit_curve` operations must preserve
the complete document except the chosen curve, including blink, smile, camera,
other clips, source geometry and embedded audio. A later jaw retime and a
camera-only edit exercise the same preservation contract and undo restoration.

## Shared-release result

The [corrected run report](../artifacts/dialogue-foundation-20260906-r2/acceptance.json)
passed on the staged release binary with SHA-256
`8fb21aa47070218548a909d1691167e0673e891f02419b56f36f9a264ca7b15b`.
The binary hash was unchanged after the run. The measured run took 10.304 seconds
and rendered 153 shot frames: two complete 69-frame shots, two five-frame partial
exports, and five cold-reload frames. Separate cue renders verify mouth and blink
behavior at the authored times.

The mouth-interior material increased from zero to 173 visible pixels. The sparse
mouth edit changed 1,918 pixels at the same camera/time. A fixed native head-field
probe changed from `-0.0435411` to `+0.299406`, proving the opened region contains
air; the closed poses restored the negative field value. The blink frame fully
occluded both unchanged eye objects. Neutral and open-mouth renders were also
visually inspected.

Thirteen rejected requests preserved the document and revision, covering duplicate
audio import, truncated replacement audio, removal of referenced audio, invalid
shot counts/rates/ranges/references, duplicate curve key times, invalid render
selections, an out-of-range WAV export, and missing mutation revision. Invalid
render selections and WAV export produced no destination output.

After saving, deleting the imported `input.wav`, and starting a fresh process,
the native document, audio inspection and both shot schedules matched exactly.
The complete recording exported identically from embedded project data. The
fractional-rate partial WAV and all five corresponding PNG files were byte-identical
to the original exports.

Evidence:

- [Full JSONL transcript](../artifacts/dialogue-foundation-20260906-r2/transcript.jsonl)
- [Native project with embedded audio](../artifacts/dialogue-foundation-20260906-r2/dialogue.json)
- [24 fps manifest](../artifacts/dialogue-foundation-20260906-r2/dialogue-24/manifest.json)
  and [reference-audio slice](../artifacts/dialogue-foundation-20260906-r2/dialogue-24/audio.wav)
- [Fractional-rate partial manifest](../artifacts/dialogue-foundation-20260906-r2/dialogue-23976-partial/manifest.json)
  and [exact partial WAV](../artifacts/dialogue-foundation-20260906-r2/dialogue-23976-partial/audio.wav)
- [Silent rendered APNG](../artifacts/dialogue-foundation-20260906-r2/dialogue-playback.png),
  [neutral frame](../artifacts/dialogue-foundation-20260906-r2/cue-rest-start.png),
  and [open-mouth frame](../artifacts/dialogue-foundation-20260906-r2/cue-open-a.png)
- [Cold-reload partial manifest](../artifacts/dialogue-foundation-20260906-r2/cold-partial/manifest.json)

The [first attempt](../artifacts/dialogue-foundation-20260906-r1/acceptance.json)
is preserved as a failed harness check. Audio decoding and export had passed, but
the Python expectation compared decimal `0.7` with the editor's native f32 value
`0.699999988079071`. Document comparison showed only the requested jaw curve had
changed. The corrected harness explicitly converts the selected expected keys to
IEEE f32 and retains exact whole-document comparison; it does not weaken the
preservation assertion or erase the earlier failure.
