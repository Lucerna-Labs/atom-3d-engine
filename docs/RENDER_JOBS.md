# Persistent render jobs

Use a render job when a sequence should survive process interruption or run in
bounded steps. Creating a job freezes a native project snapshot, the output plan,
and the exact renderer executable identity. Subsequent authoring edits do not
change its scene. Both inclusive seconds sequences and integer-frame dialogue
shots use the existing renderer and image/audio formats.

A job does not launch a background worker. An agent calls `step_render_job` until
it receives `status:"complete"`, or cancels/resumes it as needed. The authoring
revision stays unchanged. `expected_revision` is optional on these commands and,
when supplied, retains the usual conflict check; use it at creation to ensure the
intended scene revision is captured.

## Commands

After authoring a clip called `move`, send one JSON object per line:

```json
{"id":"create","command":{"op":"create_render_job","request":{"type":"sequence","directory":"motion-job","clip":"move","start":0,"end":1,"fps":24,"format":"png"}}}
{"id":"step","command":{"op":"step_render_job","directory":"motion-job","max_frames":4}}
{"id":"state","command":{"op":"render_job_state","directory":"motion-job"}}
{"id":"cancel","command":{"op":"cancel_render_job","directory":"motion-job"}}
{"id":"resume","command":{"op":"resume_render_job","directory":"motion-job"}}
{"id":"continue","command":{"op":"step_render_job","directory":"motion-job","max_frames":4}}
{"id":"verify","command":{"op":"render_job_state","directory":"motion-job","verify_outputs":true}}
```

For an authored dialogue shot, use its exact shot and optional partial selection:

```json
{"id":"create-shot","command":{"op":"create_render_job","request":{"type":"shot","directory":"dialogue-job","shot":"line-1","selection":{"start_frame":1003,"frame_count":4},"format":"exr"}}}
```

All fields from the corresponding `render_sequence` or `render_shot` request,
including view/pass overrides, retain their existing meanings. The creation
preflight validates the complete requested range and its aggregate budgets before
reserving a new directory. Existing directories are rejected, including abandoned
partial preparations; they are preserved for inspection.

A step defaults to one frame and accepts `max_frames:1..32`. The count includes a
recovered pending frame. Job-wide limits remain 2,400 frames, 8 billion pixels and
32 billion primary pixel samples. A frame can still be expensive: the step count
is not a wall-clock deadline, and cancellation does not interrupt a renderer in
the middle of a frame. Snapshot decoding and prefix verification also take time.

## Progress and control

The response includes `status`, `completed_frames`, `total_frames`, `pending_frame`,
`source_revision`, snapshot/executable/plan SHA-256 identities and `manifest_path`.
`pending` means there is unfinished work; it does not claim a worker is currently
running. A pending frame can already have rendered bytes awaiting publication or
progress acknowledgement. `complete` reflects the committed job state.

`render_job_state` reads committed metadata without taking the progress writer lock
by default. `outputs_verified:false` distinguishes that observation from a byte
check. With `verify_outputs:true`, it acquires the progress lock and checks the
snapshot, published frames and, when complete, the WAV and manifest bytes. Competing
progress/verification writers receive the existing `project_locked` failure.

A separate control lock lets another editor process request cancellation during a
step. The worker checks the flag between frames and before finalization. An
in-flight frame may complete; a completion racing with cancellation may finish.
Cancellation preserves committed frames and pending recovery data. Resume clears
the cancellation flag; the next explicit step performs work. It does not spawn a
worker. A cancelled job can still be inspected and byte-verified.

## Recovery and delivery

The job directory contains an immutable `job.json`, a normal native `snapshot.json`,
atomic `state.json` and `control.json`, persistent lock sidecars, and a private
`.render-staging` directory. `job.json` is installed last during creation, so an
interrupted preparation cannot appear as an initialized job.

For each frame, the editor reserves a new staging attempt, renders and synchronizes
its bytes, then commits a pending record with their encoded SHA-256 and length. Renderer observation metadata has
a separate digest, so corruption of camera, film or pixel-fingerprint fields is
rejected before reuse as well.
It installs the frame with an atomic no-clobber hard link, synchronizes the output
directory on Unix, and commits progress. After process death, a pending frame can
be recovered from its recorded staging identity or matching already published
bytes. A staged image lacking a pending record is preserved and rendered again;
its provenance was never committed. Recovery data and staging hard links are
retained, including abandoned attempts. Hard links do not duplicate image payload
storage on the same filesystem.

A step verifies previously committed frame bytes before reuse. This detects
external corruption but entails reading the completed prefix on each step; choose
a suitable frame batch for long shots. Finalization verifies images again, installs
any prepared WAV, and writes `manifest.json` last. A lost completion acknowledgement
can be retried idempotently. Foreign or corrupted files are never overwritten to
make a retry succeed. Errors preserve their evidence.

Completed output retains the normal `frame_0000.png`/`.exr` names, optional
`audio.wav` and sequence/shot manifest. Frame entries add encoded-file hashes and
lengths; the manifest adds source-revision, snapshot, plan and executable identities.
Shot entries retain absolute frame numbers, whole-shot indices and source PCM
windows. Splitting work into steps never resets fractional audio sample phase.

Keep the original executable to continue a job after an engine update. A different
binary is explicitly rejected by `step_render_job`; use it only to inspect the job
or create a new job. Jobs are bound to their original canonical directory, so moving
the directory is not a supported resume operation. Native snapshots contain embedded
source PNG/WAV data and can be loaded as ordinary projects for new work.

These guarantees use the same cooperating-process, local-filesystem model as
[durable projects](DURABLE_PROJECTS.md). Lock sidecars are never deleted; the OS
releases locks when a process exits. They do not provide a hostile-filesystem
sandbox, physical power-loss certification, render-farm scheduling or guaranteed
cancellation latency. The full film-production goal remains open.

## Verification

Editor integration tests compare actual PNG pixels and EXR channels to synchronous
exports, preserve exact rational-rate audio windows, check frozen snapshots across
live edits/restarts, and reject corruption, collisions and invalid destinations.
The real-process client additionally tests interruption, prefix reuse, competing
workers and frame-boundary cancellation.

The Linux-only acceptance interposer terminates the actual editor at seven storage
boundaries: before/after pending-intent commit, before/after first-frame publication,
after WAV publication, after manifest publication and after final state commit
before response. Fresh processes must recover and match uninterrupted image/audio
bytes while preserving previously published frame identities and modification times.
The test interposer is never part of the production binary.

```sh
python3 scripts/agent_render_job_acceptance.py --editor target/release/mm3e-editor --output artifacts/render-jobs-new-run
python3 scripts/agent_render_job_fault_acceptance.py --editor target/release/mm3e-editor --output artifacts/render-job-faults-new-run
```

The fault check requires Linux and a C compiler. CI configuration includes platform
runs and retained artifacts; local success alone does not verify remote CI or Windows.
