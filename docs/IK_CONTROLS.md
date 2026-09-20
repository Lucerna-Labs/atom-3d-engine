# Native two-bone IK authoring

`solve_ik` reaches a world-space target with an existing direct three-joint chain.
It measures the current root, middle and tip pivots at the requested clip time,
preserves both segment lengths, and writes native rotation keys for the root and
middle. It verifies those keys through the actual forward-kinematics evaluator and
compiles the resulting skin/face/garment geometry before committing.

This is bounded two-bone authoring, not a persistent IK constraint or a general
anatomical rig. Explicit [rotational joint limits and bounded constrained search](JOINT_LIMITS.md)
are available; self-collision avoidance, retargeting and full-body constraint solving
remain separate work. Pole and target positions are world-space
points in meters. Joints retain the existing world-rest pivot convention.

## Command

After creating the `reach` clip and the direct `shoulder -> elbow -> wrist` chain:

```json
{
  "id": "reach-wrist",
  "expected_revision": 1,
  "command": {
    "op": "solve_ik",
    "request": {
      "clip": "reach",
      "time": 0.5,
      "root_joint": "shoulder",
      "middle_joint": "elbow",
      "tip_joint": "wrist",
      "target": [0.55, 1.4, 0.2],
      "pole": [0, 1, 1],
      "unreachable": "reject",
      "tolerance_m": 0.0001,
      "tip_world_rotation_degrees": [25, -20, 35]
    },
    "dry_run": true
  }
}
```

`expected_revision` is required even for dry runs. With `dry_run:true`, the result
reports a verified candidate without changing the document, revision, undo history,
or durable project. Set it to `false` or omit it to commit. Failure is atomic.
Undo, redo and persistent project reload apply to the resulting native keys.

| Field | Contract |
| --- | --- |
| `clip`, `time` | Existing clip and authored seconds inside its duration; no implicit loop or time clamp. |
| `root_joint`, `middle_joint`, `tip_joint` | Distinct joints forming two direct parent links. The root may have an animated ancestor. |
| `target` | Requested world-space endpoint. |
| `pole` | World-space point selecting the middle joint's bend side and plane. |
| `unreachable` | `reject` by default; `clamp` explicitly projects onto the fixed-length reach interval. |
| `tolerance_m` | Forward-verification tolerance, default `0.0001`, allowed `0.000001..0.05` meters. It does not authorize stretching. |
| `tip_world_rotation_degrees` | Optional world joint-frame orientation using native `Rx*Ry*Rz` Euler degrees. Without it, existing tip-local keys stay untouched. |
| `new_track_easing` | Easing for newly created tracks only; existing track easing is preserved. |

The solver inserts or replaces a key at exactly `time`, retains other keys and
sampled local translation/scale, and preserves unrelated tracks and morph channels.
Adding a key affects interpolation in the neighboring intervals. It does not
guarantee target tracking between authored times. Descendants follow their edited
parents, so the operation drives both rigid bindings and continuous weighted skins.

## Reach and degeneracies

For measured lengths `L1` and `L2`, radial reach is
`abs(L1 - L2) .. L1 + L2`. `reject` fails outside it. `clamp` reports
`clamped:true`, the `requested_target`, `effective_target`, and separate
`requested_target_error_m` and `target_error_m`. The latter measures the native
forward-kinematics result against the effective endpoint. Root drift and measured
before/after segment lengths are included.

Collinear or coincident poles use the current bend or a deterministic perpendicular,
reported through `pole_fallback`. A target exactly at the root reports
`target_direction_fallback`. Equal-length chains can fold completely toward the
pole; with a coincident pole they retain the current first-bone direction. An
already-satisfied collinear inner or outer reach boundary retains the measured
current joints regardless of pole. Fully folded equal-length chains still allow
the pole to choose a bend direction.

The core uses f64 intermediates, robust antipodal quaternion corrections, and no
absolute minimum segment length. Nonfinite points, zero-length segments, and poses
whose f32 output cannot preserve segment lengths are explicit errors.

## Cloth dependency handling

IK edits may invalidate a related cloth cache. The edit can still succeed and reports
affected IDs in `stale_cloth_caches`; the old cache remains available for diagnosis.
`cloth_state` without animation reports `cache.fresh:false`. Animated use of stale
cloth fails until the caller explicitly runs `bake_cloth` for the affected asset and
clip. No automatic rebake or silent stale-cache playback occurs.

## Actual-process verification

Build the editor, then run the acceptance script with a new output directory:

```sh
cargo build --offline --release -p mm3e-editor
python3 scripts/agent_ik_acceptance.py --output artifacts/ik-acceptance-new
```

The script authors an 82-vertex continuous DQS limb with a dense morph, animated
ancestor, and rigid hand. It checks native joint pivots, skinned cap positions,
single-influence vertices, the weighted elbow ring, actual triangle/capsule field
queries, proper tip orientation, opposite pole choices and real PNG differences.
It also checks dry-run/rejection atomicity, clamp reporting, unrelated source/key
preservation, undo/redo, durable cold reload, and related-cloth stale/rebake behavior.
The output retains the transcript, project, images, binary/script fingerprints and
an `acceptance.json` report, including failures. `comparison.html` presents the actual
engine images after a successful run.

### Verified checkpoint, 2026-09-06

The [R2 process report](../artifacts/agent-ik-20260906-r2/acceptance.json) passes on
release SHA-256 `ef85b823a8eac88968409310b836faf224dfe7f9ef399d9c602726940840b152`.
The two pole choices reach the requested target with native errors of
`4.63e-7 m` and `3.22e-7 m`; both preserve the approximately `0.6 m` and `0.5 m`
segments. All 16 weighted elbow vertices differ from either rigid-bone placement.
The actual PNGs differ at 3,475 pixels. Undo/redo and cold reload reproduce exact
project, pose, mesh and PNG data. The related cache becomes stale after IK, rejects
animated use without producing a render file, and returns to a fresh state after
explicit rebaking; a second cold reload also reproduces its final PNG exactly.

The [comparison page](../artifacts/agent-ik-20260906-r2/comparison.html) shows the
native images. The earlier [R1 failure](../artifacts/agent-ik-20260906-r1/acceptance.json)
is preserved: the cloth fixture initially used static hand coordinates instead of
the animated hand's time-zero placement. The corrected fixture derives its panel
origin and axes from the actual time-zero wrist frame. No simulation or IK tolerance
was relaxed.
