# Film workflow engineering pass — September 6, 2026

Historical checkpoint. See the newer [sewn clothing and curved delivery verification](SEWING_AND_CURVED_DELIVERY_VERIFICATION.md)
for the current shared build, additional completed checks and remaining production work.

**The production-readiness goal remains active. This is a verified engineering pass, not a
film-production release certification.** The user selected film and animation; the visual
acceptance target (stylized characters versus photorealistic digital humans) remains open.

The final shared build passes **147 workspace tests**, whole-workspace/all-targets Clippy with
warnings denied, and whole-workspace formatting. The normally opt-in Vulkan parity test was
also run and passed separately. It used llvmpipe, a software Vulkan adapter: 13,122 scalar
samples matched the CPU implementation with maximum absolute difference `4.77e-7`. This is
not discrete-GPU performance validation.

The release executable was then exercised at the actual JSONL/file boundaries:

| Capability | Verified result |
|---|---|
| Facial controls | Physical eyelid closure reduced each eye's visible pixel count from 334 to zero; mouth opening produced real cavity air; independent geometry survived local cuts |
| Facial animation/persistence | 25-frame performance, head attachment, undo/redo, removal and cold-reopen pixels passed |
| Clothing | Fitted shirt/trousers have separate geometry/materials, thickness, neckline/waist/hem/cuff openings and source-following animated poses |
| Garment fit and edits | 3,862 sampled surfaces across eight garment/pose combinations; parameter/material updates, source changes, rollback, undo/redo and cold-reopen pixels passed |
| Film image output | Independent OpenEXR 3.4.5 decoding recovered exact f32 `[4,8,16]` RGB, including 1920×1080 output; exposure changes did not alter the scene-linear plate |
| Shot sequences | A 12-second interval at 24 fps produced 289 inclusive PNG frames; three EXR sequence frames passed independent channel/value/hash checks |
| Durable projects | Writer exclusion, transient-save exclusion, acknowledged commits, invalid/dry-run isolation, forced-kill recovery, stale revisions and preserved staging failures passed |
| CPU renderer consistency | The older gray prototype path now uses the canonical engine, including real materials, lighting, reflections, CSG, volumes and GI; explicit worker counts and 1/4-worker pixel parity passed |
| Packaging | Archive payload checksums and the copied/extracted executable's schema, render, save and fresh-process reopen passed on Linux |

The numeric checks and final captures are in
[`artifacts/production-pass-2026-09-06/verified`](../artifacts/production-pass-2026-09-06/verified/summary.json).
That directory includes the exact [build/test records](../artifacts/production-pass-2026-09-06/verified/checks.json),
[source/executable hashes](../artifacts/production-pass-2026-09-06/verified/implementation-sha256.json),
[facial evidence](../artifacts/production-pass-2026-09-06/verified/face/acceptance.json),
[clothing evidence](../artifacts/production-pass-2026-09-06/verified/garments/summary.json),
[independent film decoding](../artifacts/production-pass-2026-09-06/verified/film/acceptance.json),
and [durable-session evidence](../artifacts/production-pass-2026-09-06/verified/durable/acceptance.json).

The verified binary SHA-256 is
`2e495b13f50761ac469334a40c5686702218929690840a44c18f00e94164d82b`.
Package and source snapshots remain separate from production approval. CI was updated and
locally checked; remote CI and Windows execution were not observed. The existing checkout
action's `v7` tag was verified against its public remote rather than guessed from older docs.

Visual inspection caught a defect that initial parameter/geometry assertions missed:
closed-mouth renders had distant streaks from an extremely flat approximate ellipsoid.
The mouth interior now uses exact rounded-box geometry, and an outside-head visibility
regression passes. Failed visual and test attempts are retained in their original artifact
directories. Passing hashes alone were not treated as visual quality approval.

Open release gates include actual fabric draping, stretch, seams, friction and self-collision;
continuous skin deformation and detailed facial anatomy; teeth/tongue and speech alignment;
alpha/data AOVs, production color management and temporal/shutter quality; production asset
interchange; stress/throughput gates; persistent undo history; and approval against the
chosen artistic target. Fitted shells do not certify cloth dynamics, and procedural lip
controls do not certify anatomical acting quality. The current authored facial/body fixtures
remain visibly simple stylized characters.

Use [facial controls](FACIAL_CONTROLS.md), [garment authoring](GARMENTS.md),
[animation](AGENT_EDITOR_ANIMATION.md), and [durable projects](DURABLE_PROJECTS.md) for the
implemented command contracts. The [readiness audit](PRODUCTION_READINESS.md) retains the
broader film release requirements and original failure evidence.
