# Recreate the agent editor checkpoint

This repository includes all engine/editor source changes, Cargo.lock, acceptance
scripts, and the original unit/integration test fixtures. It retains a **4.3 MB
artifact sample** instead of the 6.6 GB local output archive. No Git LFS or archived
executable is required to rebuild it.

## What is retained

- Embedded native face and clothing projects, including their textures and
  geometry; both the material-map and textured-export versions are included.
- Original frozen face/clothing export inputs and complete request JSON. The
  request bounds, resolutions, tolerances and work limits are unchanged.
- A small speech project and analysis needed by the joint-limit script's defaults.
- The latest compact reports, source-content hashes, toolchain record, full test
  log, and five example face/clothing images.
- Test-specific geometry, WAV/texture, and legacy native-project fixtures under
  `mm3e-kit/tests/fixtures` and `mm3e-editor/tests/fixtures`.

`reproduction-manifest.json` hashes every retained artifact input/report. The
reproduction script checks those hashes before doing work. The manifest is not
an inventory of omitted local outputs. The historical source hashes and binary
hash identify the measured September 19 build; newly built executables can have
different hashes on other toolchains, platforms or source-directory paths.

## Build and run

The recorded Linux toolchain was Rust/Cargo 1.98.1 and Python 3.12. Use a Rust
installation with Cargo and a C linker. Cargo.lock fixes Rust package resolution;
a fresh machine needs network access to download the locked dependencies.

From the repository root:

```sh
cargo build --locked --release -p mm3e-editor
target/release/mm3e-editor --help
```

The JSONL protocol and examples are in [the editor guide](../mm3e-editor/README.md).
On Windows, the executable is `target/release/mm3e-editor.exe`.

For the complete checkpoint workflow, install the optional independent readers
and pinned speech runtime (the installer verifies its downloaded runtime):

```sh
python3 -m venv .dependencies/reproduction-python
.dependencies/reproduction-python/bin/python -m pip install OpenEXR==3.4.5 usd-core==26.8 'numpy>=1.24,<3'
python3 scripts/install_speech_backend.py --destination .dependencies/reproduction-speech
.dependencies/reproduction-python/bin/python scripts/reproduce_checkpoint.py \
  --output artifacts/recreated-checkpoint \
  --speech-backend .dependencies/reproduction-speech/rhubarb
```

These shell paths are for Linux. Windows venv executables are under `Scripts`,
and the pinned Windows speech executable is `rhubarb.exe`. The original real
speech checkpoint was measured on Linux; this package does not claim a fresh
Windows acceptance result.

The runner builds the release editor, runs every engine/renderer/editor test
binary with `--no-fail-fast`, then runs the facial, deformed-face, layered
animation, sewn-clothing, speech and frozen textured-USD workflows. It continues
after failed checks, writes each full log and a `reproduction.json`, and exits
nonzero if any check fails. Existing output directories are never overwritten;
choose a new `--output` for another run.

For a quicker render-only investigation, use `--skip-tests`; to reuse an existing
binary, add `--skip-build --editor /absolute/path/to/mm3e-editor`. These omissions
are recorded as skipped checks. Without `--speech-backend`, speech is explicitly
skipped. External USD/EXR readers are needed when an export reaches independent
validation; their absence is never a successful reader result.

Other workflows can be regenerated with the committed `scripts/agent_*_acceptance.py`
programs. The CI workflow records their dependency order and explicit arguments,
including fresh material-map and speech output paths for downstream checks.
Packaging is provided by `scripts/package_editor.py`. Generated executables,
archives, render sequences, scratch databases, large diagnostic transcripts,
compiler caches and downloaded dependencies remain local and are ignored.
Selected tracked evidence under `artifacts/` remains versioned despite that ignore
rule; adding another curated file deliberately requires `git add -f`.

## Expected unresolved results

The retained checkpoint is **not production ready**: 818 tests passed, three
failed and six were ignored. The failures are flat-shoulder final embedding,
folded-sheet refinement, and thin-sheet USD delivery. All five focused process
workflows passed, but the original textured face, cloth, material and constant-floor
exports exceeded their unchanged structural work caps. The active-normal case
passed only as an expected rejection. The reproduction command currently exits
nonzero for these failures; it does not conceal or waive them.

See [the current checkpoint report](production-corrections-20260919-r3/README.md)
and [readiness status](../docs/PRODUCTION_READINESS.md). Historical documents may
link to additional local-only evidence or binaries that are deliberately not
included in this small reproduction package.

The [publication verification](publication-verification-20260920.json) rebuilt
and ran these checks from an export containing only the staged Git files. It
reproduced the 818/3/6 test counts, all five passing process workflows and all
four original textured-export failures. Dependency caches and the installed
pinned speech runtime were reused explicitly; untracked project fixtures were
not available in the exported tree.
