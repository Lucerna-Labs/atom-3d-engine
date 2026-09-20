# Animation verification — 2026-09-06

Animation was implemented in the existing MM3E agent editor and exercised through its actual
JSONL executable and CPU rendering pipeline. The final release run passed.

| Boundary | Result |
|---|---|
| Tests across kit, orchestrator and editor | **104 passed, 0 failed**: 38 kit, 39 orchestrator, 27 editor |
| Strict Clippy, all targets in the affected three crates | Passed |
| Formatting check for the same crates | Passed |
| Release executable | Built successfully |
| Character animation | 19 character parts bound to 16 joints; one persistent waving clip |
| Geometry evidence | Animated hand center lies inside its posed field and outside its rest field |
| Parent/child motion | Rotated/scaled ancestors correctly carry child pivots and bound offset geometry |
| Output | 25 PNG frames for an inclusive 0–2 second interval at 12 fps; 24 distinct pixel fingerprints |
| Playback artifact | 24-frame looping APNG assembled losslessly from the real rendered frames; duplicate endpoint omitted |
| Persistence | Fresh process reopens the saved rig/clip and reproduces the same pose and PNG pixels |
| Observation isolation | Sampling, picking and rendering preserve the authored document, revision and pending redo |
| Failure behavior | Invalid clip/hierarchy edits roll back; invalid intermediate poses fail export preflight without output |
| Compatibility | Older version-1 projects with no animation fields load with empty clips/joints |

Commands used from the inner workspace:

```sh
cargo test --offline -p mm3e-editor -p mm3e-kit -p mm3e-orchestrator
cargo clippy --offline -p mm3e-editor -p mm3e-kit -p mm3e-orchestrator --all-targets -- -D warnings
cargo fmt -p mm3e-editor -p mm3e-kit -p mm3e-orchestrator --check
cargo build --offline --release -p mm3e-editor
python3 scripts/agent_animation_acceptance.py --output artifacts/agent-animation-2026-09-06-final
```

The final acceptance run took approximately **3.28 seconds** at 192×256 using the preview
preset, with shadows and AO disabled. This is a single local workflow measurement including
rendering and diagnostic queries, not a real-time performance guarantee.

The independent geometry checks observed approximately 0.917 m of left-wrist movement,
0.425 m of elbow movement, and 1.056 m of hand-center movement between the chosen poses.
The unbound pedestal retained its transform. The posed hand field was negative at its
evaluated center (`-0.03420`), whereas the same point was outside the rest hand (`0.96666`).
These are engine field samples, not a general Euclidean distance accuracy claim.

Static snapshots at rest and mid-wave were visually inspected: the arm rises from the
shoulder, the elbow and hand follow, and the head turns. The character remains a rounded
blockout with mitten-like hands. This work evaluates rigid SDF part motion; it does not
validate weighted skinning, anatomical deformation, IK, collisions, physics, clip blending,
retargeting, a visual timeline, or GPU animation performance.

The main correctness corrections were preserving the exact rest rotation basis for idle
bindings and returning authored key values directly at key times. The latter also prevents
step interpolation from introducing arithmetic error (for example between scale values
1000 and 0.001). Regression tests cover those cases, shortest quaternion arcs, exact loop
boundaries, negative time, explicit pivots, camera interpolation and overrides, dangling
targets, cycles, export limits and output collisions.

Final artifacts:

- [Animated playback](../artifacts/agent-animation-2026-09-06-final/wave-playback.png)
- [Native project with rig and clip](../artifacts/agent-animation-2026-09-06-final/hero-animation.mm3e-agent.json)
- [Acceptance results](../artifacts/agent-animation-2026-09-06-final/acceptance.json)
- [Actual request/response transcript](../artifacts/agent-animation-2026-09-06-final/transcript.jsonl)
- [Frame manifest](../artifacts/agent-animation-2026-09-06-final/frames/manifest.json)
- [Evaluated poses](../artifacts/agent-animation-2026-09-06-final/poses.json)
- [Source/executable SHA-256 manifest](../artifacts/agent-animation-2026-09-06-final/implementation-sha256.json)

The earlier successful run remains under `artifacts/agent-animation-2026-09-06`. The final
run was repeated after rebuilding the release executable with the exact-key interpolation
fix. The final APNG SHA-256 is
`a1eb71f30d9bed22610f4bcba9509c7f46be7c584e3754b1d784823e63bb95ef`.
