# Engine and editor ownership

`Lucerna-Labs/atom-3d-engine` owns the standalone renderer: mm3e-kit,
mm3e-orchestrator, CPU/GPU backends, examples, tests, and renderer releases.
The core libraries remain std-only.

`Lucerna-Labs/atom-3d-engine-editor` owns its existing native editor plus the
optional agent workspace, authoring scripts, native-project fixtures, and film
readiness evidence. The agent workspace consumes an exact engine Git revision.

The combined c6c5f3f publication was sent to this repository by mistake. The
correction removes only editor files introduced by that commit from the engine
Git tree. Each was copied and hash-verified in the editor checkout first. Local
recovery files and Git history are retained; there is no force push.

Geometry, animation, skinning, cloth mechanism, texture sampling and renderer
improvements remain here. Agent project state, JSON commands, saved editor
projects, shot jobs, and export policy belong to the editor repository.

```sh
cargo build --locked --release -p mm3e-kit -p mm3e-orchestrator -p mm3e-cpu-parallel
cargo test --locked -p mm3e-kit -p mm3e-orchestrator -p mm3e-cpu-parallel --tests --no-fail-fast
```

Core tests do not certify film readiness. The separate editor retains its known
three geometry-test failures and four original textured-export failures.

Separation verification on Linux, September 20: 513 engine/renderer/CPU/release-
client tests passed, zero failed, one optional test ignored. Formatting, strict
all-targets Clippy for those packages, and release library builds passed. Cargo
registry versions were retained. GPU runtime and Windows execution are not
claimed by this local verification.
