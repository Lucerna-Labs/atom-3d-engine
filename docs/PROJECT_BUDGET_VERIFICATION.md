# Native project-size admission verification

The previous transient editor could accept a valid compact project, or an edit
within the 4 MiB JSONL request limit, whose complete pretty-printed native project
exceeded the 64 MiB save limit. The subsequent save failed. Admission now checks
the same canonical Saved representation before accepting the candidate, including
up to 19 bytes reserved for future revision-number growth.

The shared build passes **411 workspace tests**, strict workspace/all-targets Clippy
and formatting; six existing opt-in tests are ignored by the ordinary invocation.
See the [test log](../artifacts/project-budget-workspace-tests-2026-09-06-final.log),
[Clippy log](../artifacts/project-budget-clippy-2026-09-06-final.log),
[format check](../artifacts/project-budget-format-2026-09-06-final.log), and
[release build](../artifacts/project-budget-release-build-2026-09-06-final.log).
The full film-production goal remains open.

## Real-process baseline and corrected behavior

The [baseline report](../artifacts/agent-project-size-baseline-20260906-r1/run/acceptance.json)
preserves the actual failure using a staged old executable with SHA-256:

`8fb21aa47070218548a909d1691167e0673e891f02419b56f36f9a264ca7b15b`

The [corrected report](../artifacts/agent-project-budget-20260906-r1/acceptance.json)
uses the same fixture bytes through a new executable with SHA-256:

`f7289b39ebe66a290b9d5387c6f067a2410d3e929d165e0a7b9b02e2b2a1dcc2`

The fixture contains four valid independent planar triangle surfaces. Full grids
have 65,536 vertices and 130,050 triangles each, within the existing per-surface
limits. Three full grids plus a 231×231 fourth grid provide the near-limit valid
case; replacing the fourth grid with another full grid provides the oversized case.
The test does not bypass geometry validation or raise any limits.

| Measured boundary | Result |
| --- | --- |
| Near-limit compact source | 13,943,018 bytes |
| Near-limit actual native save | 66,906,602 bytes |
| Reserved future revision bytes | 19 |
| Remaining admitted capacity | 202,243 bytes |
| Oversized compact source | 14,641,788 bytes, below the 64 MiB input-file limit |
| Oversized replacement request | 3,660,006–3,660,008 bytes, below the 4 MiB JSONL request limit |
| Accepted edited native save | 66,906,622 bytes |

On the old executable, both the oversized Load and replacement edit succeeded,
then Save failed with `serialized project exceeds64MiB`. The prior last-valid
save remained byte-identical. This failure and its last-valid file remain preserved.

On the corrected executable, transient and persistent Load, Apply, and Apply dry-run
reject the oversized candidates with `project_size_limit`. The complete prior
serialized document, revision, undo/redo availability, and durable file bytes remain
unchanged. An admissible dry run also leaves these unchanged. An existing redo entry
still works after rejection, and undo restores the complete original document.

The accepted near-limit edit saves and reloads in a new transient process with an
identical document digest and actual rendered PNG bytes. Persistent rejection and
a separate persistent cold reopen also retain exact project bytes, reported budget,
and rendered PNG bytes. The old and new last-valid document digests match, confirming
that the fix did not require changing the geometry or native document encoding.

Baseline reproduction took 6.41 seconds and the corrected process checks took
10.47 seconds on this host; these are verification-run timings, not capacity claims.
The complete JSONL requests/responses and file hashes are retained with each report.

## Reproduction

The [process harness](../scripts/agent_project_budget_acceptance.py) requires a new
output directory. Preserve an old executable before rebuilding when reproducing
the baseline:

```sh
python3 scripts/agent_project_budget_acceptance.py \
  --editor artifacts/agent-project-size-baseline-20260906-r1/mm3e-editor-baseline \
  --mode baseline --output artifacts/project-budget-baseline-new

python3 scripts/agent_project_budget_acceptance.py \
  --editor target/release/mm3e-editor --mode fixed \
  --fixtures artifacts/project-budget-baseline-new/fixtures \
  --output artifacts/project-budget-fixed-new
```

`--fixtures` reuses verified fixture bytes inside the new editor root. Omitting it
generates the same class of valid geometry fixture. The harness retains a few large
snapshots and hashes saved document bytes rather than requesting enormous
`get_document` responses. Separate native tests cover exact capacity, one-byte
overflow, escaping, startup rejection, and revision-digit boundaries.

## Shared encoder and compatibility

The [native encoder](../mm3e-editor/src/native.rs) counts and collects the same
pretty-printed Saved envelope. Counting admission does not allocate output bytes;
collecting serialization rejects a write before exceeding its quota. The reserve
is exactly 20 minus the current revision's decimal digit count, including revision
zero. This makes newly admitted snapshot capacity independent of later revision
growth while preserving the original saved-file encoding.

`project_budget` reports actual encoded size, reserved revision bytes, total limit
and remaining capacity. Transient/durable edits, all four dry-run command paths,
loads, undo/redo and existing-project startup use the common policy. Quota failures
occur before installed-file writes and do not poison the current project session.
Public tests also exercise the last valid u64 revision and verify that subsequent
mutation rejection leaves inspection and geometry queries available.

Raw files below 64 MiB are not necessarily admissible after canonical formatting.
An older canonical file within the final 19 bytes of the old limit may also need
reduction before loading under the new reserve policy. Rejected files remain
unchanged; no data is silently discarded or rewritten to fit. This is an explicit
compatibility edge, not evidence that the source file itself was corrupted.

The same release passes the existing [process-crash/locking workflow](../artifacts/durable-budget-regression-20260906/acceptance.json)
and [reference-audio dialogue workflow](../artifacts/dialogue-budget-regression-20260906/acceptance.json).
CI includes the new size-admission process script on Linux and Windows; remote CI
and Windows execution have not been observed locally.
