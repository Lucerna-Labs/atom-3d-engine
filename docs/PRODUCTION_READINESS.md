# Film and animation production readiness

Current checkpoint, September 19: **not production ready**. Facial closure,
translated barycentric clothing pins, verified legacy native-file migration,
and geometry failure accounting are corrected. Full seal now closes at maximum
jaw opening; full blinks stay closed with raised brows. Affected cloth caches
are retained but require rebaking.

The engine, renderer and editor test run reports **818 passed, 3 failed and 6
ignored**, using `--no-fail-fast`. Formatting and strict all-targets Clippy pass.
The final executable `422b38b0b4f54ecee5957ad8926783128ee6d980d36b629fc2ca9a5e00f1f011` passes real-process facial, deformed-face,
animation-layer, sewn-clothing and speech workflows. Its 301 source-content
hashes, exact commands, logs and scope are in the
[September 19 checkpoint](../artifacts/production-corrections-20260919-r3/README.md).

The three original editor failures remain: flat-shoulder final embedding,
folded-sheet rounding refinement, and thin-sheet USD comparison-grid work.
The original textured face, cloth, material and constant-floor fixtures still
exceed their unchanged 64,000,000 structural arrangement cap. The active-normal
case passes only as an expected rejection. Frozen face/clothing inputs and
requests match the originals byte for byte. See the
[full textured result](../artifacts/textured-usd-acceptance-20260919-r3/acceptance.json).

The earlier September 10 representative-cache test log stopped at its first
failure; it did not establish that later editor test binaries passed. Its
2,115-entry source manifest was a path inventory, not content hashes. Earlier
checkpoint evidence remains archived below.

Current interchange work: [bounded textured USD delivery](TEXTURED_USD.md).
The previous packaged checkpoint implemented source-preserving image transport for compatible color/scalar materials,
face-corner UV transfer and no-overwrite bundle publication. Independent material,
schema and relocation checks passed on its supported fixtures. Its original face
export exceeded its vertex allowance resolving a closest-feature UV
discontinuity; a permitted periodic lift leaves 2.34944 texels against 0.25. The
original
[thin bent-cloth export challenge](THIN_SURFACE_DELIVERY.md) still fails native
meshing convergence; it is not replaced by a flat or thickened garment. Active
normal-map transfer, general chart handling, external-renderer appearance parity
and full film-production readiness remain open.

Unreleased geometry work now adds exact source-region coalescing, bounded
intersection validation and a source-to-final displacement certificate. The
reproduced overlapping-shell refinement defect rejects correctly, and the six
scalar/Boolean USD regressions pass with final intersection checks enabled.
The source-feature producer now passes the flat, tilted and folded extraction
tests. Bounded conditioning, exact coplanar retessellation and certified warped
patch flips let the frozen bundle pass native refinement with independently
checked residuals and outward faces. An optional exact degree-one radial proof
reduces the live bundle's fine-grid stage from 199,130,604 to 159,337,446 work
units, with the complete pairwise check retained as a counted fallback.
Caching triangle quality and reusing complete proofs across exactly matching
stage boundaries now lets both original constant-map bundle tests pass. Stale
snapshots trigger full validation; compaction reuse requires a verified index
bijection and bit-identical coordinates. The original native sheet now reaches
six refinement passes within budget but still fails face orientation. The full
original animated face/clothing challenge remains open, and this working source
is not a new release checkpoint.
[Current geometry evidence and remaining gates](THIN_SURFACE_DELIVERY.md).

Previous rig-limit work: [rotation limits and constrained IK](JOINT_LIMITS.md).
Final layered joint rotations now honor explicit hinge/cone/twist restrictions,
with rejection or reported projection. Feasible search, physical face/cloth
references, cache changes and frozen jobs pass focused checks. These constraints
do not constitute anatomical calibration or full film-production readiness.

Previous speech checkpoint: [local analysis and editable lip curves](SPEECH_LIP_SYNC.md).
Rhubarb-backed cue analysis, explicit character mappings, source preservation and
editable native curve generation are implemented. Spoken/silent controls and
rendered geometry/audio/reload checks pass locally; recognition accuracy, human
speech evaluation and final acting quality remain unproven. The full film goal
remains open.

Previous animation-composition checkpoint: [clip layers](ANIMATION_LAYERS.md).
Independent body, blink, mouth and camera clips now compose through native poses,
geometry, cloth simulation, shots and resumable rendering. Exact face/cloth split
comparisons, source edit isolation and cache freshness checks pass. Full film
production readiness and artistic/external-agent quality remain open.

Previous resumable-render checkpoint: [persistent render jobs](RENDER_JOBS.md). Frozen
native snapshots, bounded steps, frame-boundary cancellation and SHA-256 verification
are implemented for sequence and dialogue-shot exports. Actual process-death
checks cover seven storage boundaries and preserve exact image/audio output.
Full artistic/film-production readiness and external-agent quality remain open.

Previous normal-map filtering checkpoint: [contract and verification](NORMAL_FILTERING.md).
Optional isotropic slope-variance compensation is implemented; the source passes
462 workspace tests. An independently decoded editor HDR probe
confirms suppression of the false frontal glint, along with exact cold reload and
disabled controls. Directional/bimodal distributions still differ substantially from
the source reference, so the filter defaults off. Full film-production readiness
remains unproven.

Previous shared build: [material-map and relighting verification](MATERIAL_MAP_VERIFICATION.md).
453 workspace tests and actual face/cloth normal, scalar and emissive map checks pass.
Geometric coverage and caches remain stable while lighting changes. Broader film
quality, material calibration, normal-variance filtering and textured delivery remain open.

Previous shared build: [attached UV texture verification](TEXTURE_VERIFICATION.md).
439 workspace tests and real face/cloth texture, persistence, film and storage checks
pass. PNG assets and indexed-corner UVs now follow native deformation; broader material
shading and textured interchange remain open. Full film-production readiness is unproven.

Previous shared build: [storage-failure recovery verification](STORAGE_FAILURE_VERIFICATION.md).
419 workspace tests and actual error-injection/restart checks pass. Installation state
is now explicit, including ambiguous rename results and failures after no-clobber output
installation. These are specified I/O-error tests, not physical power-loss certification.
The full film-production goal remains open.

Latest preservation fix: [native-project size admission](PROJECT_BUDGET_VERIFICATION.md).
The shared build passes 411 workspace tests and rejects previously accepted-but-unsaveable
edits before acknowledgment. Near-limit native files, history, restart, and dialogue
regressions pass on the identified release. The full film-production goal remains open.

Previous shared build: [reference audio and shot-clock verification](DIALOGUE_VERIFICATION.md).
It passes 405 workspace tests and actual dialogue, IK, clothing and film-output
checks on one release binary. Explicit lip timing and audio delivery are implemented;
automatic speech alignment and the full production goal remain open. A remaining
transient-project size-preservation gap is recorded in that checkpoint.

Current work: [limb targets and shaped clothing patterns](IK_AND_PATTERN_VERIFICATION.md).
That checkpoint passes 373 workspace tests and both real-process workflows on one
identified build. It does not close the overall film-production goal.

Latest shared-build results: [continuous deformation verification](DEFORMATION_VERIFICATION.md).
Native LBS/DQS skinning, morph controls, a continuous blinking/mouth fixture and deformed-body
cloth contact are implemented. Independent delivery of a skinned mesh now passes with counted
BVH work and the unchanged aggregate limit. Full film-production readiness is still open.

The previous [sewn clothing and curved delivery verification](SEWING_AND_CURVED_DELIVERY_VERIFICATION.md)
records the earlier shared build.
The original required USD challenge now passes unchanged through the real exporter and
independent reader. Explicit sewn-panel construction and animation are implemented and
verified. Overall film-production readiness is still open.

The [earlier cloth contact and USD checkpoint](CONTACT_AND_USD_VERIFICATION.md) preserves
the failed extraction cases that led to the current implementation.

Earlier shared-build results: [film workflow verification](PRODUCTION_PASS_VERIFICATION.md).
Local CSG, procedural facial controls, fitted clothing, linear EXR, durable sessions and
canonical CPU rendering have since been implemented and tested. The film-production goal
remains open; the original audit below is retained as baseline evidence.

Audit date: September 6, 2026. Target: an autonomous character editor for film and animation,
including facial performance and clothing. This is an acceptance contract and a record of
observed gaps, not a certification. The project has an operational authoring and animation
foundation, but **the audited baseline is not production ready for film character work**.

This audit covers only this MM3E workspace. It examined the actual editor, renderer,
animation evaluator, persistence, import path, tests, and checked-in CI/release workflows.
The release binary was probed through JSONL without writing project or render files.
Implementation work is occurring alongside the audit; the baseline observations below
remain historical evidence even when a later change closes them. A gate is closed only by
identified implementation plus fresh acceptance evidence.

## Existing foundation

- Stable object IDs, strict request decoding, optimistic revisions, atomic edit batches,
  dry runs, and bounded undo/redo are implemented in
  [the editor](../mm3e-editor/src/lib.rs).
- Native projects embed analytic geometry, baked volume samples, joints and clips. Saves
  stage and synchronize a temporary file before installation; loads validate before
replacing the current document. See [storage](../mm3e-editor/src/storage.rs).
- Animation evaluates explicit times from rest, including quaternion rotation
  interpolation, parented rigid parts, and animated cameras. Pose, sample, pick and render
  share the evaluated scene. See [animation](../mm3e-editor/src/animation.rs).
- PNG sequence export preflights every requested pose, refuses existing output directories,
  preserves partial output on failure, and writes its manifest last. See
  [sequence export](../mm3e-editor/src/sequence.rs).
- The prior [animation validation record](AGENT_EDITOR_ANIMATION_VALIDATION.md) records
  104 passing tests and fresh-process pose/pixel persistence for a 19-part, 16-joint waving
  blockout. Its 25-frame run used **192×256 preview output with shadows and AO disabled**.
  That result establishes its tested behavior; it does not establish close-up facial
  quality, cloth behavior, shot throughput, or release packaging.

### Verified film-output progress after the baseline audit

The [film-output acceptance record](../artifacts/film-output-2026-09-06/acceptance.json)
identifies a later release binary that passed independent OpenEXR 3.4.5 decoding through
the actual JSONL render command. It delivered a 1920×1080 scene-linear f32 RGB EXR and
retained exact channel radiance values `[4, 8, 16]`, explicit Rec.709/D65 chromaticities and
linear-transfer metadata. Changing display exposure did not change the EXR radiance;
fresh-process project reload reproduced those values. A request for a display AOV in EXR
was rejected without an output file. The same run exported the inclusive 12-second/24-fps
range as 289 PNG frames. These close the three reproduced baseline output limitations for
the tested simple scene; they do not close the whole film-image or performance gate.

The new [package script](../scripts/package_editor.py) produced and checked the
[Linux editor archive](../artifacts/mm3e-editor-linux-x86_64-2026-09-06.zip), including
per-file SHA-256 verification and an extracted-binary posed render, native save and fresh
process load. It includes schemas, an executable JSONL example, documentation and dependency
notices. The checked-in CI workflow now includes the editor and reference-decoder acceptance
on Linux and Windows plus package artifacts. **Remote CI and Windows package execution have
not been verified by this local run.** The existing Windows viewer release workflow remains
unchanged. Final release evidence must identify the exact final build after further changes.

Alpha/compositing mattes, configured OCIO transformations, temporal rendering and complex
character-shot throughput remain open. The RGB EXR plate includes its opaque background;
it is not an RGBA compositing layer.

The [expanded film run](../artifacts/film-output-2026-09-06-exr-sequence/acceptance.json)
also independently decoded three EXR sequence frames, verified their declared linear
encoding and float-pixel fingerprints, and confirmed that unsupported display-pass EXR
sequences are rejected before reserving an output directory.

Durable `--project` mode subsequently passed
[actual process-crash acceptance](../artifacts/durable-project-2026-09-06-corrected/acceptance.json):
revisions and geometry survived two forced terminations, pixel output matched after restart,
competing persistent startup and transient saves were refused, and a real staging-file
collision preserved the old destination, the old live document and pre-existing crash
evidence. See the [durable project guide](DURABLE_PROJECTS.md). The storage cleanup bug found
during this work was fixed by removing only temporary files created by the current write.
This advances the preservation gate; power-loss tests, persistent history, network-filesystem
guarantees and injected post-rename directory-flush failures remain unverified.

## Ranked release gates

The ordering follows dependency and user-visible risk. Every row remains an open gate until
the listed evidence is recorded. Stylization may change visual targets; it does not remove
the requirement that the intended facial expressions, clothing and shots work reliably.

| Priority / gate | Required behavior | Acceptance evidence | Audited baseline |
| --- | --- | --- | --- |
| P0 — Preserve authored work | Invalid edits leave the complete document and its dependencies unchanged; saves survive interrupted writes; uncertain responses are recoverable; concurrent writers cannot silently lose work. | Negative tests for every new control and garment dependency; process-restart tests; interrupted-save and disk-full tests; documented conflict behavior tested with two processes. | Transactions and staged saves exist. History is process-local. No autosave/recovery journal or project lock; overwrite is last writer wins. |
| P0 — Facial performance | Independently control both eyelids, gaze, brows, jaw, mouth opening, lip seal, stretch, rounding and asymmetric expression; preserve attached facial detail under head/jaw motion. | Close-up neutral, full blink, partial blink, asymmetric blink, eye aim, lip seal, wide/open/round mouth and combined controls; geometry checks plus front/three-quarter/profile renders; no visible eye/lip intersections or holes. | Baseline animates complete rigid objects only; no facial controls or eyelid/lip geometry. |
| P0 — Coherent character deformation | Body and face remain connected through intended poses with stable volumes and normals. Corrective behavior is explicit and editable. | Head turns with expression; shoulders raised; elbows/knees bent; torso twist; extreme joint poses; continuity and penetration measurements plus visual review over time. | One rigid binding per part. Smooth CSG can join posed fields, but it is not continuous skin or a guarantee of anatomical deformation. |
| P0 — Clothing authoring and fit | Agents can create, size, edit and remove garments; author openings, thickness, layering, seams and material intent; garments follow their intended body region without cutting unrelated geometry. | At least a fitted top, lower garment and a loose garment on more than one body size; sleeves/neck/waist openings; rest and motion fit checks; garment replacement, body resizing, undo/redo and save/reload. | Generic solids and onion modifiers are available. No garment authoring contract, fit solver, isolated garment CSG, garment topology or cloth model. |
| P0 — Cloth motion and contact | Loose material drapes and moves as intended, respecting attachment constraints, gravity, collision thickness, layering and self-contact where required by the garment. | Draping at rest and motion tests for sleeve bends, walking, sitting, fast turns and stacked fabric; record penetration, stretch, seam errors and failure cases at the production time step. | Existing physics models rigid bodies; no cloth implementation or cloth acceptance. Rigid fitted shells alone do not close this gate. |
| P0 — Film image output | Render the agreed shot resolution without losing scene-linear dynamic range; preserve alpha and useful compositing passes; identify color spaces and display transforms. | HD and agreed final-resolution renders, HDR values above 1 retained in a floating-point image, reliable alpha edges and round-trip through an independent image reader/compositor. | Per-image cap rejects 1920×1080; only display-converted RGBA8 PNG/BMP output; no EXR or color-management configuration. |
| P0 — Temporal image quality | Geometry, normals, materials, shadows and highlights remain stable at frame and subframe times; camera and object motion support the intended shutter and focus behavior. | At least one facial close-up and one moving clothed full-body shot at final quality; inspect frame differences and playback; shutter/motion-blur and focus/DOF acceptance with declared tolerances. | Keyed motion exists; no editor motion-blur, shutter, depth-of-field or temporal-quality acceptance. Identical rerenders do not by themselves prove absence of flicker across motion. |
| P0 — Interchange and delivery | Deliver animated characters and clothing through a declared production format or an explicit end-to-end native pipeline; maintain identities, units, animation, geometry and materials. | Reopen delivered assets in an independent consumer; compare chosen poses, surfaces, camera, units and materials; document every baked/lossy field. | Native editor save/load exists. OBJ ingestion discards topology, UVs and material assignments. No asset export or external delivery acceptance. |
| P1 — Shot authoring | Frame-accurate shot ranges, clear timebase, clip editing/layering, curve controls, reference audio and repeatable lip timing; agents can revise a performance without rebuilding unrelated tracks. | Dialogue shot with blinks and expression over speech, camera edits, timeline range exports and restart; verify frames and audio alignment at the declared frame rate. | Seconds-based clips and one track per target exist. No audio/phoneme path, clip blending, retargeting or visual timeline. |
| P1 — Surface detail | Skin, eye, mouth and fabric materials have enough detail and light response for the intended framing; preserve patterns/material placement during deformation. | Controlled-light close-ups covering fabric texture/weave, skin and eyes; edit and reload material detail; review under multiple lights and poses. | Constant GGX material parameters plus a checker flag. No texture/UV/displacement workflow, skin scattering, eye optics or fabric shading contract. |
| P1 — Autonomous execution | Typed discoverable controls, explicit units/ranges/dependencies, bounded jobs, cancellation, resumable work, usable error recovery, and observations tied to the actual revision and pose. | An actual external agent client completes character creation, facial motion, clothing edits and render/revision recovery using the advertised interface; retain prompts, requests, responses and failures. | Strict JSONL process exists. Prior acceptance is a deterministic Python client. No actual LLM quality/efficiency benchmark, job progress/cancellation protocol, or MCP transport. MCP is an integration choice, not a replacement for tested task completion. |
| P1 — Performance and capacity | Complete representative film shots within agreed wall-time, memory and disk budgets; expensive operations remain interruptible and predictable. | Recorded hardware, binary/source identifiers, scene counts, quality, frames, timings, peak memory and output size for a facial close-up and clothed full-body shot, including worst-case assets. | Low-resolution preview evidence only. 240-frame and 32-million-pixel sequence caps; a synchronous render blocks further requests. |
| P1 — Distribution and continuous checks | Ship the editor actually being tested, with dependencies, docs, license notices and an install/run path on supported systems. | Clean-machine package launch; JSONL acceptance from packaged executable; CI includes editor tests and acceptance; release artifact identity matches tested build. | Checked-in CI omits the editor from clippy/build/test. Release workflow packages only the Windows GPU viewer. No editor release artifact. |

## Confirmed baseline boundary failures

The following probes ran against `target/release/mm3e-editor` on the audit date. Its SHA-256
was `030df3b11c4437ab2e7a6ff9d9543cc0539e3bdde513f3f81be564a3bde8fab5`.
The source was being extended concurrently; these results identify this binary, not future
builds. All three failures returned structured errors without producing files. The rejected
settings edit retained revision 0; creating the empty 12-second clip committed revision 1.

| Probe | Actual response | Consequence |
| --- | --- | --- |
| Apply full-quality settings at 1920×1080. | `render size must be 1..2048 per axis and at most 1048576 pixels` | The editor cannot currently deliver even this HD resolution through its supported settings. Raising limits must be paired with memory/time budgeting. |
| Render to `probe.exr`. | `render path must end in .png or .bmp` | A PNG renamed EXR would not solve this. Output must originate before display tone mapping and use an appropriate floating-point writer. |
| Export a valid 12-second clip from 0 through 12 at 24 fps. | `sequence exceeds 240 frames` | This inclusive range requires 289 frames. Long-shot execution needs a bounded chunk/resume contract or an appropriately budgeted larger export. |

The [CI workflow](../.github/workflows/ci.yml) explicitly lists kit, orchestrator and release
client packages for clippy/build/test and does not list `mm3e-editor`. Its workspace-wide
format step does not execute editor tests. The
[release workflow](../.github/workflows/release.yml) builds and copies `gpu_viewer.exe` only;
testing the source editor does not make it part of that package. These are source-confirmed
release gaps, independent of whether a workflow has recently run remotely. No remote CI run
or release publication was inspected in this audit.

## Small details that affect the architecture

1. **CSG isolation is necessary for garment openings and facial cavities.** The current
   document is one ordered global CSG fold. A subtractive mouth or neck opening can cut the
   accumulated body and other clothing. `group` is selection metadata and provides no
   geometric isolation. Independent local constructions need explicit scoped composition
   or complete standalone fields, with tests showing that nearby objects remain unchanged.
2. **A scalar query is not automatically a clearance measurement.** The editor correctly
   describes field samples as authored scalars rather than guaranteed Euclidean distances.
   Ellipsoid approximations, smooth blends and sampled fields retain those semantics.
   Cloth contact cannot silently treat every scalar as an exact penetration depth; validate
   the relevant field/gradient bounds and contact correction behavior.
3. **Small detail has a world-space tolerance budget.** `Scene::volume` raises the single
   scene-wide normal stencil from the coarsest registered volume; the stencil is not chosen
   per surface. Shading also uses minimum ray offsets of 0.01 m for shadows and 0.02 m for
   reflections. These are source-supported risks for eyelids, lips, seams and thin fabric,
   not yet demonstrated visible defects. Test close-ups with coarse imported scenery and
   multiple object scales before claiming fine-detail fidelity.
4. **Native snapshots are useful but incomplete recovery.** Up to 32 cloned documents remain
   in process memory, including embedded volume samples. There is no persistent edit log,
   autosave recovery, shared project lock or retry deduplication. A request ID correlates a
   response only. Production clients need an explicit recovery strategy for process loss,
   uncertain acknowledgements and competing writers.
5. **A valid rest document does not certify every animated time.** The existing evaluator
   checks requested poses and sequence preflight checks the scheduled frames. Interpolated
   cameras and composed transforms can still be invalid elsewhere. Subframe shutter samples,
   facial-control combinations and garment deformation must use the same validation path.
6. **Material ownership is not full geometric provenance.** A smooth blend chooses a
   dominant material; subtraction retains the base material. Picking one owner cannot prove
   which garment/cavity contributed to a surface. Agent repair tools need relevant local
   observations and dependency information.
7. **OBJ bake is a one-way representation change.** The parser reads positions and
   fan-triangulates faces; it ignores UVs, normals and material directives. Import preserves
   a baked scalar field, not editable source topology, garment panels or skin weights.
   Concave/nonplanar polygons and open meshes require explicit acceptance or rejection;
   successful parsing is not a topology certificate.
8. **Display output is not a compositing master.** The renderer already computes a float HDR
   buffer, but `post::resolve` applies exposure, an ACES-like curve and gamma before the editor
   writes an 8-bit image. The presence of an ACES-named function does not establish a
   configured production color pipeline. Preserve the scene-linear buffer and declare the
   source color space, display transform and alpha semantics separately.
9. **A deterministic screenshot is not a quality score.** Pixel hashes detect changes and
   support exact restart comparisons. They do not measure anatomy, acting quality, lip seal,
   clothing fit, temporal aliasing or aesthetic intent. Acceptance needs independent geometry
   measurements plus rendered review at the actual camera distance.

## Architecture direction without replacing MM3E

Keep the SDF engine and agent document as the authored core. Add explicit control semantics,
bounded local field construction, representation-appropriate deformation, and measured
observations. Do not replace the user's engine with another application merely to acquire
features. A film interchange or image writer can be a delivery boundary while native MM3E
data remains authoritative; generated delivery data must declare what is baked or lost.

For portable skeletal assets, the official
[OpenUSD UsdSkel schema](https://openusd.org/dev/api/_usd_skel__schemas.html) distinguishes
joint animation, skin influences and blend-shape animation, and specifies deformation
ordering. That is a useful interchange checklist; it does not mean those capabilities
already exist here or require MM3E to adopt a mesh renderer.

The official [OpenEXR technical introduction](https://openexr.com/en/latest/TechnicalIntroduction.html)
describes HDR and linear-light storage suited to downstream processing. The
[OpenColorIO roles documentation](https://lf-aswf.atlassian.net/wiki/spaces/OCIO/pages/11274955/Roles)
describes how a configuration declares its scene-linear working role. These support the
image-delivery gate; selecting or integrating the appropriate production configuration
still requires implementation and independent verification.

## Completion record required for a release claim

Record the exact source and packaged binary, accepted film style/framing, supported asset
limits, test results, full JSONL acceptance transcripts, representative final-resolution
shots, restart/interchange comparisons, temporal failures, runtime/memory measurements and
remaining exclusions. Preserve unsuccessful runs. Gate closures must describe precisely what
was demonstrated: a controllable blink does not certify anatomical tissue; a fitted animated
shirt does not certify cloth draping; an EXR writer does not certify a color-managed render
pipeline; passing unit tests does not certify the complete production workflow.
