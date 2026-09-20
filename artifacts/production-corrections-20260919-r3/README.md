# Character-editor correctness checkpoint — September 19, 2026

**The editor is not production ready for film and animation.** This checkpoint repairs facial closure, translated clothing attachments, native-project compatibility, and geometry work accounting. The original geometry and textured-delivery gates remain open.

## Executable and provenance

- Release-profile executable: [mm3e-editor](mm3e-editor).
- SHA-256: `422b38b0b4f54ecee5957ad8926783128ee6d980d36b629fc2ca9a5e00f1f011`.
- [301 source-content hashes](source-sha256.json), [matching post-verification hashes](source-after-verification-sha256.json), [toolchain](toolchain.txt), and [commands, exit codes and timings](verification.json).
- The full three-crate test run is [preserved in r2](../production-corrections-20260919-r2/tests.log). After that run, only equivalent array-chunk iteration in the USD test parser changed to satisfy the current Clippy version. Runtime source hashes are unchanged; [the verified difference](full-suite-evidence.json) and [affected USD rerun](tests.log) are retained.
- r1 and r2 artifacts preserve earlier results and lint failures. No original challenge inputs, requests, limits or old failure logs were replaced.

## Implemented corrections

- Full lip seal closes the procedural aperture even at maximum jaw opening. Brow displacement fades during blinking, so a raised brow cannot reopen a fully closed blink. Neutral gaze preserves exact authored positions; nonzero gaze uses a translation-free vector transform.
- New zero face controls omit their serialized fields to preserve pre-extension neutral data. Native reads verify all seven generated parts against the current or exact September 10 generator before migrating. Authored controls, sources, materials and cache frames survive; edited or mixed derived geometry rejects atomically. Startup retains the original file and saved revision. Load reports migrated rig IDs and remains undoable.
- Barycentric garment attachments normalize accepted f32 weights in f64 and use difference-coordinate interpolation. Tests cover positive/negative translation, unequal morph displacements, transformed LBS/DQS subframes, pattern pins, rollback and reload. Authored weights remain intact. Attachment evaluation has an explicit 35-unit charge.
- Affected face-dependent and barycentric cloth caches require rebaking. Old cache frames are retained. Existing local/world attachments retain their compatibility behavior. Triangle indices still identify attachments: topology reorder stales caches, but rebaking without updating authored indices can retarget a pin.
- Plane-relation caches distinguish support masks, share symmetric keys, retain negative results, cap storage at 512, and charge actual lookup/evaluation work. The fixed edge solver's original eight-unit charge is restored. Triangle emission retains oriented corner/source correspondence without promising the previous cyclic anchor or byte ordering.
- Radial certification constructs exact vertex coordinates on demand. On the preserved failing flat mesh, unsuccessful radial work falls from 10,270,148 to 5,800,427 (4,469,721 saved). Topology, exact predicates, centre/ray candidates and certificate requirements remain mandatory. The full intersection fallback still fails.
- Counted BVH candidate-query failures retain completed traversal work. All four intersection consumers charge the query before subsequent candidate processing. The flat fixture now correctly retains the previously lost 143 units; failed inventories cannot publish partial proofs.

## Verification

| Crate | Passed | Failed | Ignored |
|---|---:|---:|---:|
| mm3e-kit | 420 | 0 | 1 |
| mm3e-orchestrator | 85 | 0 | 0 |
| mm3e-editor | 313 | 3 | 5 |
| Total | 818 | 3 | 6 |

The complete test command uses `--no-fail-fast`. Formatting and strict three-crate all-targets Clippy pass. This is the tested engine/renderer/editor scope, not a claim about every optional workspace package or skipped external test.

The final executable passes actual process workflows for [procedural facial controls](../agent-face-20260919-r3/acceptance.json), [deformed faces](../agent-deformed-face-20260919-r3/acceptance.json), [animation layers](../agent-animation-layer-20260919-r3/acceptance.json), [sewn clothing](../agent-sewing-20260919-r3/acceptance.json), and [speech-driven curves](../agent-speech-20260919-r3/acceptance.json). These cover rendered geometry, saved state and fresh-process reload at their documented boundaries. Focused new native migration and pin regressions are included in the full editor run.

## Preserved failures

| Gate | Current result |
|---|---|
| Frozen flat shoulders (`surface_feature_split`) | Final stored-mesh embedding exhausts its allowance. Total embedding work is 57,823,712 including radial work 5,800,427. |
| Folded native sheet (`surface_refine`) | Rounding repair exhausts the unchanged 8,000,000 budget in pass 1 at 3,976 vertices / 7,948 faces; the prior pass still reports 24 orientation failures. |
| Thin native sheet USD (`usd_delivery`) | Fine grid uses 175,091,859 of the 200,000,000 aggregate allowance; comparison-grid coplanar retessellation fails after 2,319,554 stage work. |
| Original textured face, cloth, material and constant-floor fixtures | All four still exceed the unchanged 64,000,000 structural arrangement cap. |

[Final textured acceptance](../textured-usd-acceptance-20260919-r3/acceptance.json) retains every failure. Frozen face and cloth inputs and generated requests match the original files byte for byte. Active-normal passes only as an expected rejection; successful active normal-map delivery and independent reader/appearance acceptance for the failed exports remain unproven.

All three editor failures were present in earlier comprehensive evidence. The September 10 representative-cache test invocation stopped after the first failure and never reached the other two binaries. Its apparent single-failure result was incomplete. The old 2,115-entry source list was a path inventory, not a content-hash manifest.

## Remaining work

Reduce measured extraction and refinement work under the original limits; complete final stored-mesh embedding; then pass fine-plus-comparison textured USD and independent reader checks. Facial controls remain procedural: gaze translates eyes/lids and brow controls change lid cuts. Anatomical acting quality, realistic fabric behavior, general tailoring, external renderer parity, and autonomous artist-level quality still require separate evidence.
