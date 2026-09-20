# Storage failure and recovery verification

The editor now tracks installation state explicitly. A staging failure is different
from an indeterminate installation call or a failure after a successful installation.
Project recovery no longer depends on matching a human-readable error-message prefix.
The full film-production goal remains open.

The final shared build is
`770154fd3454f3893d6137f795bfbe3a4554f63dfa41865bcd7725e19675f802`.
It passes **419 workspace tests**, strict workspace/all-targets Clippy and formatting.
Six optional tests remain ignored by the ordinary invocation. This build also includes
the independently tested [surface interpolation-coordinate foundation](SURFACE_ATTRIBUTES.md);
editor UVs and texture rendering remain unimplemented.

| Shared-build check | Evidence |
| --- | --- |
| Workspace tests | [Final log](../artifacts/storage-surface-tests-2026-09-06-final.log) |
| Strict Clippy | [Final log](../artifacts/storage-surface-clippy-2026-09-06-final.log) |
| Formatting | [Final check](../artifacts/storage-surface-format-2026-09-06-final.log) |
| Release build | [Final log](../artifacts/storage-surface-build-2026-09-06-final.log) |
| Eight real-executable fault/control cases | [Final report](../artifacts/storage-fault-final-20260906/acceptance.json) |
| Near-limit project admission and reload | [Regression](../artifacts/project-budget-storage-regression-20260906/acceptance.json) |
| Existing process-crash and writer-lock checks | [Regression](../artifacts/durable-storage-regression-20260906/acceptance.json) |
| Reference audio and dialogue shots | [Regression](../artifacts/dialogue-storage-regression-20260906/acceptance.json) |

## Actual error-return injection

The Linux [rename manual](https://man7.org/linux/man-pages/man2/rename.2.html)
documents an NFS case where a failed return can follow an already completed rename.
The [fsync manual](https://man7.org/linux/man-pages/man2/fsync.2.html) distinguishes
file synchronization from synchronizing its directory entry. These contracts motivate
separate installation and completion states; the test below does not certify NFS.

The [Linux acceptance script](../scripts/agent_storage_fault_acceptance.py) compiles
a [test-only interposer](../scripts/storage_fault_shim.c) and runs the actual editor.
It scopes interception to fresh case directories, an exact destination, current-process
staging names and an explicit arming marker. The production editor has no injection
flags or test branches. Logs distinguish real syscall results from injected returns.
Process maps independently confirm that the interposer is loaded in the fault session
and absent after restart.

This tests recovery from specified I/O error returns. It is **not a physical power-loss
test or a filesystem durability certificate**. Linux `/proc`, dynamic ELF loading and
a C compiler are required; this test is not a Windows storage certification.

The [preserved baseline](../artifacts/storage-fault-baseline-20260906-r2/acceptance.json)
uses release `f7289b39ebe66a290b9d5387c6f067a2410d3e929d165e0a7b9b02e2b2a1dcc2`.
The initial [typed-state result](../artifacts/storage-fault-typed-20260906-r1/acceptance.json)
uses `60c936ec10d49b2978f00ae247c6778973310ae403856f08a79c4b0bee4e12d6`.
These two runs have identical harness, C-source and compiled interposer hashes.
Only the editor binary differs.

| Case | Observed operation and required result |
| --- | --- |
| Normal durable commit | Real write, file sync, rename and directory sync succeed; proposed revision is acknowledged and survives restart. |
| Normal no-clobber output | Real hard link and cleanup succeed; expected rendered PNG remains. |
| Staged write ENOSPC | 37 real staging bytes are written before the injected failure; old project, live state and history remain exact, own temporary output is cleaned, and a disarmed retry succeeds. |
| File-sync EIO | Failure occurs before rename; old file/state/history remain exact and the session stays usable. |
| Directory-sync EIO | Rename actually succeeds first; response is `commit_uncertain`, subsequent commands require recovery, and restart recovers the proposed revision. |
| Rename error before execution | No rename occurs, but its failed return is treated as indeterminate; restart recovers the previous revision. |
| Rename error after real success | Rename actually installs the proposed file before EIO is returned; the session requires recovery and restart recovers the proposed revision. |
| No-clobber cleanup failure | Hard link actually installs the output before staging-link removal fails; response is `output_installed_uncertain`, the complete expected PNG remains, and the unchanged project session remains usable. |

The baseline reported the rename-after-success case as ordinary `io`, leaving disk
at revision 4 while the session remained usable at revision 3. It also returned an
ordinary error after the no-clobber PNG had already been installed. The new typed
visibility prevents either response from implying rollback.

The corrected persistent uncertainty cases block eight command types with
`recovery_required`. Restart reads the actual installed file rather than writing
the older in-memory document back to disk. Complete project bytes, revisions, poses,
PNG bytes, history availability, retained unrelated evidence and temporary-file
cleanup are inspected through the real executable and filesystem.

## Reproduce

```sh
python3 -B scripts/agent_storage_fault_acceptance.py \
  --editor target/release/mm3e-editor \
  --output artifacts/storage-fault-NEW
```

The output directory must be new, and the environment must not already set
`LD_PRELOAD`. Each report retains executable/interposer/source hashes, compiler
identity, loaded-library evidence, complete JSONL requests/responses, syscall logs,
actual project files, rendered images and restart results. Baseline mode preserves
the old behavior rather than weakening the fixed-mode assertions.
