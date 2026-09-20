# Joint rotation limits and constrained IK

Native joints support explicit hinge and swing/twist rotation limits. Limits are
applied to the final local rotation **after clip layers combine and before parent
forward kinematics**. They affect rigid bindings, weighted skin, procedural
attachments, cloth dependency motion, shots and render jobs through the shared pose
evaluator. They do not infer anatomical ranges or constrain translation/scale.

## Configure a joint

Inside an ordinary Apply transaction:

```json
{"op":"set_joint_limit","id":"elbow","limit":{"type":"hinge","axis":[0,0,1],"min_degrees":-145,"max_degrees":0,"mode":"reject"}}
{"op":"set_joint_limit","id":"shoulder","limit":{"type":"swing_twist","axis":[1,0,0],"swing_degrees":100,"twist_min_degrees":-75,"twist_max_degrees":75,"mode":"project"}}
{"op":"clear_joint_limit","id":"elbow"}
```

The field is also available as `rotation_limit` in `set_joints`. Sparse operations
change only the selected joint's limit and retain hierarchy, bindings and pivots.
They use normal revision checks, atomic batches, dry runs and undo/redo.

Axes are finite, nonzero directions in the joint's **world-rest coordinate system**
before parent motion. The math normalizes them; changing the axis sign reverses
positive twist. Twist bounds must be ordered, lie within -180..180 degrees and
contain zero. The rest pose therefore remains admissible. Cone swing is 0..180
degrees. A hinge locks swing to zero while allowing its declared twist interval.
These example bounds are authoring choices, not human anatomical specifications.

`mode:"reject"` is the default. An evaluated violation returns a named joint/clip
error before geometry output. `mode:"project"` explicitly applies a bounded
rotation during evaluation and reports the change. Authored keys remain unchanged.
The projected delta is rebuilt around the same rest pivot, preserving sampled
translation and uniform scale at native precision; it is not a basis replacement
with a stale positional offset. Pose diagnostics include measured local-pivot drift.

Static definition validation does not require every stored clip to stay inside a
limit. An 80-degree source clip at half weight can produce an admissible 40-degree
assembly; rejecting the source during blending would be incorrect. Two additive
30-degree layers instead produce a final 60-degree request. Local direct keys still
override their inherited channels before the limit is evaluated. An illegal stored
performance remains authorable so agents can inspect and repair it.

## Inspect and repair

```json
{"id":"limits","command":{"op":"joint_limit_state","animation":{"clip":"reach","time":0.5}}}
{"id":"pose","command":{"op":"pose","animation":{"clip":"reach","time":0.5}}}
```

`joint_limit_state` reports requested swing/twist, angular correction, configured
limits, projected matrix/Euler values and `would_reject`/`would_project`. It does
not modify geometry or keys and can inspect violations in Reject mode. Successful
`pose` results include `joint_limits` with `applied` for actual evaluation changes.
Use the projection as a reviewable authoring reference or explicitly select Project
mode; neither silently overwrites existing animation.

Angles are projective quaternion rotations, not accumulated turns. Opposite
quaternion signs represent the same rotation. Twist is reported on a principal
branch, while cyclic endpoint comparison handles equivalent +180/-180 rotations.
Compliance uses SO(3) angular error, not a naive comparison of displayed Euler
components.

The separated projection factors `q = swing * twist`, limits swing along its
existing axis, selects the nearest cyclic twist endpoint and recomposes. It is
**not a global nearest SO(3) projection** for coupled cone/twist restrictions.
At a perpendicular half-turn, twist is undefined; a deterministic zero-twist gauge
is used and `singular_twist` is reported. Projection is stateless: it does not add
winding, velocity limits, history or smoothness across branch cuts. Use valid
curves and temporal review rather than treating hard projection as automatic acting.

Native f32 quaternion and matrix readback can destabilize a tight boundary near a
half-turn. The kernel checks both representations, permits at most eight bounded
inward repairs, and reports `quantization_adjusted` plus the additional correction.
Repairs cannot add more than 0.01 radians; unrepresentable cases reject explicitly.
The compliance tolerance itself remains `32 * f32::EPSILON` radians. These limits
are numerical contracts, not claims that a quantization-adjusted pose is artistically
acceptable. Ordinary admitted poses retain their original evaluated matrices.

The terminology is related to established
[swing/twist joint limits](https://nvidia-omniverse.github.io/PhysX/physx/5.2.1/docs/Joints.html).
MM3E's rest-frame, projection, diagnostics and persistence rules here are its own;
no PhysX dependency or compatibility is implied.

## Constrained two-bone authoring

`solve_ik` retains its existing radial `unreachable:reject|clamp` behavior. When a
selected root/middle/tip joint has a rotational limit, it additionally uses a bounded
search over feasible local rotations. New fields are:

| Field | Meaning |
| --- | --- |
| `limit_policy` | `reject` by default; `best_feasible` explicitly accepts a residual pose. |
| `limit_evaluations` | Maximum candidate evaluations, default 4096, range 1..32768. |

The geometric solver first supplies the radial effective target and the middle
point selected by the pole. Search keeps the parent, pivots, sampled translations
and scales fixed. It considers projected current/analytic seeds, local rotation
trials and bone-axis twists. Optional tip-world orientation participates in each
candidate evaluation instead of being clamped only afterward.

Reject mode requires the native result to meet endpoint, pole-selected middle and
optional orientation tolerances. Exhausting or stalling a local search means **no
verified solution was found**, not proven infeasibility. `best_feasible` returns the
best finite feasible candidate visited, with `limit_search.within_tolerance`,
`approximation_accepted`, evaluation count, budget status and measured residuals.
It does not claim a globally nearest pose. Both policies verify root position,
segment lengths, final limits and faithful native Euler-key reconstruction.

`clamped` and `effective_target` keep their original radial meaning.
`target_error_m`, `middle_error_m` and `tip_orientation_error_degrees` report actual
remaining errors. In an explicitly accepted approximation these can exceed target
tolerances; success then means the feasible keys were authored, not that the exact
request was reached. The solver preserves source clips and inherited local
translation/scale, including layered inputs. Existing tip keys remain untouched
when no tip orientation was requested.

The search is a deterministic projected pattern search, not an implementation of
SDLS. The broader distinction between numerical IK solutions and unreachable-target
behavior is discussed in [Buss and Kim's IK research](https://mathweb.ucsd.edu/~sbuss/ResearchWeb/ikmethods/).
Full-body solving, retargeting, collision avoidance and anatomical calibration are
separate work.

## Cloth, persistence and output boundaries

Relevant joint limits are part of cloth motion dependencies. Changing one preserves
the old cache as stale evidence and requires rebaking; an unbound unrelated joint
limit does not stale the cloth. The existing rest-pin condition still applies at
clip time zero. A first IK key at a later time holds backward to zero, so author a
neutral initial key or deliberately refit/rebind the rest garment when needed.

Changing limits invalidates a previously captured animation evaluation's input
signature. New native projects retain the metadata; absent limits are omitted so
older project encodings and flat cloth fingerprints remain compatible. Render jobs
freeze the limits in their native snapshot and resume with that snapshot even if
live authoring changes afterward.

Sequence/shot/job preflight checks every requested frame and shutter sample before
reserving output. It catches later-frame violations, not only valid endpoints.
This sampling does not prove continuous-time compliance between all requested
samples; Project or Reject evaluation remains authoritative whenever a pose is used.

## Verification and remaining scope

Core tests include independent hinge/cone references, 6,000 random cases, cyclic
endpoints, sign invariance, singularities, and quantized native readback. Retained
failed cases document the near-pole quantization defect and its bounded repair.
Editor tests cover final-layer limits, nonzero pivots/parents, actual weighted jaw
fields, cloth contacts/caches, IK history/reload, late-frame preflight and frozen jobs.

Real-process acceptance compares continuous face and cloth motion with independently
authored capped references, tests generated speech on actual jaw skin, and verifies
constrained IK success/rejection/explicit approximation. Reported small image
mismatches against manual reference keys reflect their different f32 paths; exact
cold rerenders and unconstrained/admitted behavior are checked separately.

This completes explicit rotational-limit authoring and evaluation. It does not
certify anatomical tissue, unrestricted solver convergence, global optimality or
film-production readiness.
