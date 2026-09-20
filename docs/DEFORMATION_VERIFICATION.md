# Continuous deformation checkpoint — September 6, 2026

Weighted skinning, morph-based facial controls, deformed-body cloth contact and independent
delivery of a skinned mesh now work through the actual editor. **The full film-production
goal remains open.** The current face is a functional geometry demonstration, not an
anatomically or artistically approved production character.

The release binary SHA-256 is
`4fef789db4ac2031031d97570bc6f16da300584c0c30f4a6eb0194a6533b4011`.
The shared source passes **334 workspace tests**, strict whole-workspace/all-targets Clippy,
and formatting. Five opt-in tests remain ignored by the ordinary workspace invocation.

| Boundary | Evidence |
|---|---|
| Workspace tests | [Full log](../artifacts/workspace-deformation-2026-09-06-r1.log) |
| Strict Clippy | [Full log](../artifacts/clippy-deformation-2026-09-06-r1.log) |
| Formatting | [Check log](../artifacts/format-deformation-2026-09-06.log) |
| Release build | [Build log](../artifacts/build-deformation-2026-09-06-r3.log) |
| Skinned mesh, images and independent USD reader | [Acceptance](../artifacts/agent-skinning-20260906-r7/acceptance.json) |
| Continuous face, clearance and reload | [Acceptance](../artifacts/deformed-face-final-20260906/acceptance.json) |
| Existing sharp-rim USD challenge | [Regression acceptance](../artifacts/usd-deformation-regression-20260906/acceptance.json) |

## Geometry and controls

The core evaluates rest-local morph displacement through the object's rest placement,
then skins its rest-world points with the existing joint deltas. Linear blend skinning
and rigid dual-quaternion skinning are explicit choices. The latter rejects nonunit
joint scale; it never silently falls back. Morph-only objects use a real no-skin path.
Source vertex order and triangle indices remain unchanged, and default/posed degenerate
surfaces are rejected before committing or producing partial output.

The actual 82-vertex, 160-triangle twist fixture includes a morph bulge before opposing
joint rotations. At its middle ring, DQS retains a radius near 0.100 m while LBS reaches
0.0342 m; the fixed-camera images differ at 6,301 pixels. Native positions agree with the
independent analytic formulas within 8.1e-8 m. Source data remains unchanged, returning
motion reproduces the rest pose, method-edit undo restores pixels, and reopening preserves
the project, evaluated vertices and PNG bytes. See the [DQS/LBS comparison](../artifacts/agent-skinning-20260906-r7/comparison.html).

`bind_surface`, `update_deformer`, `remove_deformer`, and `set_morph_weights` expose these
controls through atomic operations. The last command edits named scalar defaults without
resending dense delta arrays. Morph tracks share canonical time/easing with joint tracks
and override their corresponding defaults. Invalid weights, targets, channels, ownership,
default shapes and unsupported DQS scale are explicit errors. See the [authoring guide](DEFORMATION.md).

## Continuous blink and mouth motion

The native face control set now also exposes head-local `gaze_x`/`gaze_y`,
independent `brow_left`/`brow_right`, and `lip_seal` channels. Gaze moves both
eyes and their physical lids with the head pose; brow values alter each lid's
cut line; lip seal narrows an authored jaw opening without replacing the cavity.
These controls are optional fields, so version-1 rigs and clips load unchanged.
The focused facial test covers animated tracks, scene movement and save/reload
bit identity. They remain procedural controls rather than an anatomical rig.

The face fixture contains four deformed surfaces with 594 vertices and 960 triangles:
curved eyelids, a lip annulus and surrounding mouth skin. Eight adjacent lid targets
preserve a curved path around each eyeball during linear interpolation. Head and jaw
joints move the same geometry used by rendering and field queries.

Both eyes change from 1,791 visible owner pixels to zero on closure and return to 1,791.
The mouth interior grows from 58 to 1,463 visible pixels. A closed-boundary point changes
from -1.5 mm inside geometry to +30.49 mm of air after opening. All five reopened native
poses and PNG files match exactly. The [actual playback](../artifacts/deformed-face-final-20260906/deformed-face-playback.png)
comes from the editor's rendered frames.

An independent support-plane bound covers every eyelid triangle throughout every adjacent
target transition, including shell thickness. Minimum conservative clearance is 1.61 mm.
The release additionally checks 53,922 positions against the actual eye and lid fields at
33 times, with a sampled minimum shell clearance of 5.96 mm. This establishes the tested
path's clearance; it is not a general blendshape collision certificate.

## Clothing and scoped evaluation

Cloth bakes now evaluate relevant skin/morph geometry before every contact substep, retain
the required joint ancestors and scalar channels, and fingerprint those physical inputs.
LBS and DQS fixtures lift cloth to 0.363 m against a body field at 0.35 m, compared with
lower frozen/skin-only/morph-only controls. A scaled rest surface also produces the expected
0.723 m contact height. Relevant edits invalidate caches, while unrelated deformers,
lighting and camera edits preserve freshness. Authentic earlier v3/v4 cache fixtures remain
loadable and fresh without changing their stored geometry.

Scoped body samples and selected USD exports work with unrelated unbaked cloth in the
document. Full-scene rendering still rejects unfinished participating cloth. Pins on a
deformed surface now use an authored source triangle plus normalized barycentric weights;
the evaluated triangle is sampled at every physical substep and the saved cache replays it
after reload. Arbitrary local-point pins on a deformed surface remain rejected; ordinary
joint-driven attachment objects remain supported.

## Delivery and bounded work

The combined three-pose skinned-mesh export passes 16 OpenUSD schema validators. All
124,584 delivered vertices are queried against the native field, and 384 points per pose
are compared independently against the analytically deformed source triangles. Maximum
oracle disagreement is 6.26e-8; sampled convergence is at most 0.012494 m and field residual
at most 0.004253, within the unchanged 0.03 m / 0.012 limits.

The final fixture uses the recorded 40×48×40 grid and bounded box. Earlier grid placements
and the initial 48-cubed request exposed sampling and accounting limits; every attempted
request is indexed in the [preserved history](../artifacts/agent-skinning-20260906-history.json).
No source geometry, topology check or quality tolerance was weakened to close the final gate.

The old static estimate charged 264,077,760 units for the three individually valid poses
because it assumed every surface triangle was tested at every query. Actual scene/BVH
accounting admits the combined export at **99,739,354 units** under the unchanged 200M
limit. Bounding-box and triangle tests are charged before execution. An independent
[parity record](../artifacts/agent-skinning-20260906-r7/optimization-parity.json) confirms identical
USD point arrays and native PNG bytes before and after the accounting change. This is an
accounting correction, not a claimed wall-time speedup or geometry simplification.

Single-scalar export uses actual counted scene/BVH units. Multi-channel Boolean export
retains its static program estimate and counted arrangement work. The original sharp-rim
challenge still passes. Export remains an evaluated mesh cache, without editable UsdSkel
or glTF skin/morph interchange.

## Remaining production work

The native deformed object remains a hollow triangle-shell field with fixed world shell
thickness. There is no inferred solid body interior or anatomical tissue simulation.
Full anatomical rigs, teeth/tongue/eye optics, IK, clip blending/retargeting, speech timing,
broader tailoring, textures/UVs/subdivision, production color management, large-shot job
control, and artistic acceptance remain open. The new CI steps are checked in; remote CI
and Windows execution have not been observed locally.
