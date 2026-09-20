# Agent editor verification — 2026-09-06

The first agent-control layer was built and exercised against the existing MM3E CPU engine on
Linux. The final release executable completed the character acceptance run. This record covers
that first layer, not a finished visual editor or production character pipeline.

| Check | Result |
|---|---|
| Core baseline before implementation | 63 tests passed |
| Final kit, orchestrator and editor tests | **77 passed, 0 failed**: 32 kit, 35 orchestrator, 10 editor |
| Clippy, all targets of the three affected crates, warnings denied | Passed |
| Formatting check for the three affected crates | Passed |
| Release executable build | Passed |
| Actual JSONL subprocess: create, edit, render, pick, save, restart, reload | Passed; edited pixels differ and reloaded pixels are identical |
| Atomic failure, dry run, revision conflict, undo/redo | Passed; rejected requests leave state unchanged |
| Malformed/oversized JSONL recovery | Passed; next valid request remains synchronized |
| Native volume persistence | Passed; typed state and scalar samples survive save/load |
| Real OBJ cube import | Passed after the shared-edge parity correction |
| Triangle crossing | Shared diagonal, reversed winding, common vertex, and small projection regressions passed |
| Release character acceptance | **Passed in 5.445 seconds** in this one local run |

Commands run from the inner workspace:

```sh
cargo test --offline -p mm3e-editor -p mm3e-kit -p mm3e-orchestrator
cargo clippy --offline -p mm3e-editor -p mm3e-kit -p mm3e-orchestrator --all-targets -- -D warnings
cargo fmt -p mm3e-editor -p mm3e-kit -p mm3e-orchestrator --check
cargo build --offline --release -p mm3e-editor
python3 scripts/agent_character_acceptance.py --output artifacts/agent-editor-2026-09-06-final
```

The character client created 19 named body parts plus a pedestal. Seven measurement-driven
edits changed the chest's X radius while preserving its other radii and every other entity.
At the requested landmark, the final object-scoped authored field residual was
`-0.00011717575398506597`, within the configured `0.0002` threshold. This is a field-residual
check, not an anatomical-quality or general distance-accuracy claim.

The run includes front images before/after, three-quarter, side, back, normal and depth passes,
then a front image after restarting and loading the native project. The after/reloaded PNG
files were byte-identical. Their SHA-256 was
`411bc78cc06f94252a5d7df4011545022b4098e83e1c1f58f1fcec92c50374e1`.
The final saved session revision was 10.

At 384×512 using the full preset, the five beauty render measurements were approximately
252–276 ms each in this run. The complete run time also includes diagnostics, PNG encoding,
state checks and process work. These are single-run measurements on the current host, not
an FPS guarantee or a comparison with GPU rendering.

Visual inspection confirmed a recognizable, editable humanoid blockout in the front and
three-quarter images, with corresponding normals. The asset has simple rounded anatomy,
mitten-like hands, no detailed face or rig, visible joint transitions, and shadow speckling near
the feet. Those are quality limitations to address in later modeling and rendering work.

The evidence is stored in
[`artifacts/agent-editor-2026-09-06-final`](../artifacts/agent-editor-2026-09-06-final/acceptance.json):

- [`acceptance.json`](../artifacts/agent-editor-2026-09-06-final/acceptance.json): results, residual trajectory and timings.
- [`transcript.jsonl`](../artifacts/agent-editor-2026-09-06-final/transcript.jsonl): actual requests and responses.
- [`capabilities.json`](../artifacts/agent-editor-2026-09-06-final/capabilities.json): executable-generated schemas and scope.
- [`hero.mm3e-agent.json`](../artifacts/agent-editor-2026-09-06-final/hero.mm3e-agent.json): reopenable native project.
- [`implementation-sha256.json`](../artifacts/agent-editor-2026-09-06-final/implementation-sha256.json): source and executable identities.
- [`three-quarter.png`](../artifacts/agent-editor-2026-09-06-final/three-quarter.png): rendered blockout.

Failure history remains available. The original
[`agent-editor-2026-09-06`](../artifacts/agent-editor-2026-09-06/acceptance.json) run failed because
the combined field at the chest probe was owned by an overlapping arm. Adding an explicit
object-scoped observation resolved that ambiguity; the
[`object-scope`](../artifacts/agent-editor-2026-09-06-object-scope/acceptance.json) run then passed.
The subsequent OBJ test found a genuine double-counted crossing defect in the original bake;
the half-open projected edge mechanism corrected it. An earlier persistence test expectation
also compared differently formatted f32/f64 JSON numbers; that assertion was corrected to
check typed round-trip state, exact scalar observations, and unchanged saved bytes.

GPU execution, Windows viewers, updater behavior, LLM planning, anatomical quality, animation
deformation, mesh export and a visual editor were not validated by this run. The core's legacy
text writer still omits volumes; the new native project format avoids that writer. The full
workspace's unrelated targets were not part of the three-crate validation above.
