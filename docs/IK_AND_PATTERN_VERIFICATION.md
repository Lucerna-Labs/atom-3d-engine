# Limb targets and clothing patterns — September 6, 2026

**This checkpoint is verified. The editor is not yet production ready for film.**

The new authoring layer adds world-space limb targets and named planar clothing
outlines with holes, explicit seams and retained topology. These capabilities build
on the [verified continuous deformation layer](DEFORMATION_VERIFICATION.md).

## Shared-build evidence

- [Workspace tests](../artifacts/ik-pattern-workspace-tests-2026-09-06-final.log):
  373 passed, zero failed, five optional tests ignored.
- [Strict workspace/all-targets Clippy](../artifacts/ik-pattern-clippy-2026-09-06-final.log): passed.
- [Focused editor regression](../artifacts/ik-pattern-final-focused-2026-09-06-r1.log):
  real forward-kinematics target reaching, scaled parent rigs, explicit unreachable
  policies, atomic failures, dry run, undo/redo and pattern save/load passed.
- [Final release pattern run](../artifacts/pattern-release-20260906-final-r2/acceptance.json):
  actual JSONL authoring, baking, rendered openings, sewn/unsewn/filled-hole controls
  and cold reload passed. Its exact binary SHA-256 is
  `ef85b823a8eac88968409310b836faf224dfe7f9ef399d9c602726940840b152`.

The pattern fixture has 106 vertices and 166 triangles. Explicit stitches retain
the two panels together while the unsewn control separates by 0.372 m. Native
queries and rendered rays retain both arm openings; a filled-hole control changes
966 pixels. All three cases reproduce cached geometry and PNG bytes after reload.
The [posed image](../artifacts/pattern-release-20260906-final-r2/sewn-openings/posed.png)
is a functional pattern assembly, not an approved fitted production garment.

Two regression failures led to fixes: JSON f64 round-trip precision is now enabled
to preserve retained pattern geometry, and Euler key extraction reconstructs a
consistent rotation in f64 near gimbal lock. Failed logs remain in artifacts.

## Limb targets and actual deformation

The [real-process IK acceptance](../artifacts/agent-ik-20260906-r2/acceptance.json)
uses the same binary. Its 82-vertex DQS limb includes a dense morph, an animated
ancestor and a rigid hand. Opposite pole requests achieve endpoint errors of
4.63e-7 m and 3.22e-7 m, retain measured segment lengths, and change 3,475 rendered
pixels. All 16 blended elbow vertices differ from either purely rigid bone pose.
Native field queries confirm the evaluated triangle and hand geometry. See the
[rendered comparison](../artifacts/agent-ik-20260906-r2/comparison.html) and
[IK authoring contract](IK_CONTROLS.md).

Dry runs, unreachable rejection, explicit clamp reporting, unrelated source/key
preservation, undo/redo and durable reload are verified. Editing a limb with a
related baked cloth reports the same stale cache as `cloth_state`; stale animated
reads and rendering fail without output. Explicit rebaking succeeds, and a cold
restart reproduces the document, pose, mesh, cloth cache and PNG bytes exactly.

## Failure history and limits

The first full workspace run exposed the Euler conversion bug; the initial
threshold-only correction still failed near gimbal lock. Consistent f64 rotation
reconstruction passes 81 angle combinations with a 5e-6 matrix tolerance. An
independent 250,000-angle review found a maximum error of 2.027e-6.

Core review also found an already-reached collinear endpoint rejected at extreme
segment ratios. Its measured pose is now retained at this unique reach boundary;
the exact huge-root/tiny-distal regression passes with three different poles.
Input guards reject excessive hole, pin, seam and chain counts before costly set
construction. Four adversarial editor tests supplement the authoring tests.

Earlier logs and the [failed pattern reload fixture](../artifacts/pattern-f64-pre-fix-repro/README.md)
remain intact. The [first IK process run](../artifacts/agent-ik-20260906-r1/acceptance.json)
passed limb checks but rejected the cloth fixture's mismatched rest attachment.
The corrected fixture derives its placement from the actual animated hand at time
zero; no solver or contact tolerance was relaxed.

CI now includes both process acceptance scripts on Linux and Windows. Local Linux
execution is proven; remote CI and Windows execution have not been observed here.

## Scope and remaining film requirements

IK authors ordinary keys at one requested time. It does not constrain the target
between keys or enforce anatomical joint limits. Pattern controls support planar
polygon outlines and holes; see [the pattern contract](PATTERNS.md). They do not
implement darts, seam allowances, grading, automatic fit or calibrated fabric.

The [film release gates](PRODUCTION_READINESS.md) remain open, including production
facial performance, representative clothed character poses and shots, surface
detail, audio/lip timing, color management and actual external-agent task quality.
