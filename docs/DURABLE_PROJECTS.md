# Durable editor projects

Project and native save paths must end in `.json` (including `.mm3e-agent.json`), keeping
them distinct from render destinations. All editor native saves use the same OS locks.

Use `--project` when an agent should continue editing the same native project across process
restarts. The editor acquires an OS writer lock and saves every completed mutation before
acknowledging its new revision. The ordinary mode without `--project` remains a transient
session that needs explicit `save` commands.

From the workspace containing `Cargo.toml`, create a project directory and start the editor:

```sh
cargo build --locked --release -p mm3e-editor
mkdir -p artifacts/my-character
target/release/mm3e-editor --root artifacts/my-character --project hero.mm3e-agent.json
```

The root and parent directories must already exist. The project path must be relative to
`--root`, with no traversal components. An existing project is validated and reopened;
otherwise a new empty native project is persisted at revision 0. A corrupt or unsupported
project fails startup without replacing its contents.

Send one JSON request per input line. Inspect the current revision first:

```json
{"id":"state","command":{"op":"inspect"}}
{"id":"create","expected_revision":0,"command":{"op":"apply","operations":[{"op":"create_humanoid","id":"hero","height":1.8,"build":1.0,"head_scale":1.0}]}}
```

The creation example applies only to a new empty project at revision 0. Use the actual
revision from the response when continuing an existing project. After a successful mutation,
the native file contains the complete authored document and acknowledged revision, including
all supported geometry, rig, animation, facial and clothing data. Opening the same project
in a fresh process restores that revision; it does not reset it to zero.

## Edits, history and saves

`apply`, `undo`, `redo`, `load`, `import_obj`, `import_audio`, `solve_ik` and `bake_cloth` persist their candidate document and next
revision before changing live state. Validation and storage errors before installation
preserve the previous live document and installed project. A dry run does not write a new revision.
Observation commands do not increment revisions. Image and sequence commands still create
their requested output files.

Undo/redo history is **session-local**. The currently selected state resulting from undo or
redo is durable; the surrounding history is not saved. Restarting therefore restores the
latest document and revision with empty undo/redo lists.

`save` remains useful for explicit snapshots. Saving to the active project's own path with
`overwrite:true` reuses its held lock. Saving to another destination acquires that destination's
writer lock for the duration of the write. This also applies to `save` from a transient
session, so a transient editor cannot overwrite a project held by another editor process.
Existing destinations require `overwrite:true`. A snapshot does not change the active
project path or the editor revision.

`load` reads a native document and commits it to the **current** persistent project at the
next revision. It does not switch which file `--project` is bound to. Start a different
process with the desired `--project` path when switching projects.

## Native project size and admission

Every accepted snapshot, including transient edits and dry runs, must fit the same
64 MiB canonical saved-project budget. Counting uses the exact pretty JSON encoder,
including the native envelope, original embedded audio, and escaped/UTF-8 text.
Transient checks count without allocating a saved-file buffer; collecting writes
stop before exceeding the quota. A rejected candidate never reaches the disk writer
or changes the live document, revision, undo/redo lists or existing project file.

Up to 19 bytes inside the limit are reserved for the revision number to grow to its
maximum 20 decimal digits. The saved JSON itself is unchanged. Reserving these bytes
keeps newly admitted history snapshots saveable across revision digit boundaries.

```json
{"id":"native-space","command":{"op":"project_budget"}}
```

This read-only command returns `encoded_bytes`, `revision_reserve_bytes`,
`limit_bytes` and `remaining_bytes`; the first, second and fourth sum to the limit.
Use a dry run to measure whether a proposed edit is admissible. `project_size_limit`
is an ordinary rejection, not a poisoned or uncertain commit; observations and
subsequent valid edits remain usable.

The raw input-file limit is not a guarantee of canonical admission: compact JSON
below 64 MiB can expand beyond the saved-project budget. Existing canonical files
within the final 19 bytes of the old limit can also require reduction before the
new admission policy accepts them. Rejected sources are left unchanged; the editor
does not silently remove data or reformat files in place to make them fit.

## Locks and crash recovery

The editor holds a nonblocking OS lock on an adjacent file named
`.mm3e-project-lock-<identifier>`. The sidecar remains present after normal exit or a crash.
Its existence does not indicate a stale lock; the operating system owns the live lock and
releases it when the process/handle closes. **Do not delete or replace sidecars while editors
may be running.** Deleting a locked inode can defeat coordination. Sidecar names are reserved
and cannot be used as project destinations.

A second persistent writer fails startup. A competing `save` reports `project_locked`.
Canonical parent resolution makes supported directory aliases share the same lock. These
locks coordinate participating editor processes; unrelated tools that bypass the protocol
can still change files. The existing local-root checks reject escapes and project/lock-file
symlinks, but they are not a sandbox against a hostile process changing the filesystem.

After an interrupted request or lost response, reopen the project and inspect its revision
and document before retrying. Request IDs correlate responses; they are not deduplication
keys. An interruption may occur after a complete revision reached disk but before the agent
received its acknowledgement.

Writes synchronize staged bytes, replace the project file and synchronize the containing
directory where supported. A staging write or file-synchronization failure before installation
retains the old file. Installation-call errors are treated as indeterminate: a failing return
does not prove that the destination remained unchanged. If installation succeeds but later
cleanup or directory synchronization fails, the destination is known to have been installed,
while complete write durability has not been confirmed.

For the active durable project, either uncertainty returns `commit_uncertain` and blocks
subsequent session commands with `recovery_required`. Close/reopen and inspect the actual
project; restart may recover the old or proposed revision, depending on what reached disk.
The editor does not roll back a possibly installed file using its older in-memory document.
Installation state is tracked explicitly; human-readable error text does not determine it.

For ordinary render/export outputs, `output_state_uncertain` means installation itself is
indeterminate, and `output_installed_uncertain` means installation succeeded before a later
step failed. Inspect the destination before retrying. Saving a separate snapshot can also
report `commit_uncertain` for that destination without invalidating the active project's
unchanged state. No-clobber writes use a hard link: failure to remove the owned staging link
after its installation can leave a complete output even though the request reports an error.

Pre-existing temporary files are preserved on staging-name collisions. They are not treated
as the current writer's files and are not silently removed. Failed outputs and crash evidence
remain available for inspection.

The [storage-failure acceptance](STORAGE_FAILURE_VERIFICATION.md) now exercises real
staging writes, file sync, installation and cleanup under targeted error-return injection.
It verifies both possible disk states after an ambiguous rename result and cold restart.

## Verification and limits

Run the actual executable acceptance with a new output directory:

```sh
python3 scripts/agent_durable_acceptance.py --binary target/release/mm3e-editor --output artifacts/my-durable-acceptance
```

The script retains its request/response transcript, process stderr, saved projects, rendered
comparison images, intentionally preserved temporary collisions, and `acceptance.json` or
`failure.json`. It checks acknowledged revisions on disk, dry-run/invalid-edit isolation,
writer exclusion including transient saves, undo/redo persistence, forced process termination
and reopening, geometry and image equality, stale revisions, and real staging-collision
failures without loss of the original file or live document.

The September 6, 2026 [CLI acceptance record](../artifacts/durable-project-2026-09-06-corrected/acceptance.json)
passed those checks, restoring revision 5 after two forced terminations and reproducing the
same rendered pixels. Its full transcript and retained staging-collision files are beside
the result. The [earlier failed test run](../artifacts/durable-project-2026-09-06/failure.json)
is preserved: the client initially compared short serialized f32 decimals with JSON
observations promoted to f64. The corrected client compares exact authored f32 values;
it does not substitute a loose geometric tolerance.

The ProjectStore unit tests also exercise OS lock release after killing a child process,
symlink confinement, parent aliases and document-size rejection. Standard-library locks
require Rust 1.89 or later. Local process-crash verification is not a power-loss test across
all storage devices or a guarantee for every network filesystem. Persistent history,
multi-user collaboration and protection against external nonparticipating writers remain
separate capabilities.
