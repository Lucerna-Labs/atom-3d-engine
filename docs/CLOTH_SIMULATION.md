# Loose cloth authoring and baking

Fitted shells and loose cloth are separate tools. `create_cloth_panel` creates real indexed
triangle geometry with physical thickness. `bake_cloth` evaluates fixed-step XPBD dynamics,
then stores native vertex positions alongside the authored rest surface. Rendering, queries,
film samples and USD field extraction use these actual evaluated triangles.

The following creates a hanging panel with two world-space attachments and an empty one-second
clip. Send each request as a separate JSONL line to a new editor session:

```json
{"id":"panel","expected_revision":0,"command":{"op":"apply","operations":[{"op":"put_clip","clip":{"id":"hold","duration":1}},{"op":"create_cloth_panel","request":{"id":"cape","origin":[0,1,0],"axis_u":[1,0,0],"axis_v":[0,0,1],"segments":[4,4],"width_m":0.8,"height_m":0.8,"thickness_m":0.004,"vertex_mass_kg":0.02,"pins":[{"vertex":0,"point":[-0.4,1,-0.4]},{"vertex":4,"point":[0.4,1,-0.4]}],"settings":{"iterations":12,"bend_compliance":0.05,"self_collision":true,"self_collision_thickness":0.006,"friction_coefficient":0.3}}}]}}
{"id":"bake","expected_revision":1,"command":{"op":"bake_cloth","request":{"id":"cape","clip":"hold"}}}
{"id":"measure","command":{"op":"cloth_state","id":"cape","animation":{"clip":"hold","time":0.5}}}
{"id":"image","command":{"op":"render","path":"cape.png","animation":{"clip":"hold","time":0.5}}}
```

The panel origin is its center. Unit perpendicular axes define its plane; vertex order is
rows along `axis_v`, then columns along `axis_u`. A pin with `target_object` interprets its
point in that object's local coordinates; an omitted target uses world coordinates. Pins
must meet the rest vertices at clip time zero. A pin does not silently snap the initial mesh.
Attachment motion can use object keys or a joint hierarchy. `collision_object_ids` selects
body fields explicitly, composing them in original document order with a Union seed.

`update_cloth_panel` replaces panel parameters while preserving its entity ID and order,
and clears its cache. `remove_cloth` removes the simulation asset and generated geometry.
Material changes use ordinary entity updates. Direct edits to generated triangle positions
are rejected; update the authoring asset or bake new motion instead.

Settings control edge stretch compliance, signed dihedral bending compliance, gravity,
damping, physical substeps, solver iterations and contact response. Compliance values are
solver parameters, not calibrated fabric presets. External contact uses vertex probes of
the chosen signed fields and numerical normals. `collision_thickness` must cover at least
half the rendered full thickness. Optional self-contact uses swept vertex–triangle and
edge–edge features along linear substep trajectories, deterministic candidate ordering,
inverse masses and local topology exclusions. `self_collision_thickness` is the minimum
layer gap and must cover full sheet thickness. `friction_coefficient` enables positional
Coulomb friction; external-surface friction currently assumes a stationary surface.

Every accepted physical frame must satisfy `max_penetration_m` and
`max_relative_edge_error`. Candidate/work limits fail atomically rather than skipping excess
contacts. The default tolerance values are engineering safeguards, not a garment-quality
specification. Pins remain exact; contradictory attachments can correctly make a bake fail.
Initial arbitrary intersections, one-ring exclusions, finite iterative convergence and
external vertex-only contact prevent a universal collision-free guarantee. There are no
external triangle-interior contacts, reaction forces, sewing constraints, pattern authoring
or measured woven-fabric material law yet.

Baking starts at zero. Duration must align with `fixed_dt`; limits are 10 seconds, 601
inclusive cache frames, 256 vertices per cloth, eight cloth assets and 320,000 cached vertex
samples per document. Constraint/field work has a separate estimate limit. The bake is an
atomic document mutation requiring `expected_revision`; `dry_run:true` computes and validates
without committing. In durable mode a completed cache is saved before acknowledgement.
The synchronous process does not yet provide in-flight progress/cancellation commands.

At arbitrary playback times, free vertices interpolate cached samples and pins follow the
exact evaluated attachment transforms. Playback rechecks external vertex penetration, edge
strain and enabled static self-feature clearance. This catches some bad intermediate poses,
but zero measured deficit does not prove global absence of intersections or certify an entire
time interval. `cloth_state` reports measured vertices, tolerance status and diagnostics even
when rendering would reject a bad sampled pose. Repair the source or reduce the step size
and rebake; weakening tolerances is not a physical fix.

Caches fingerprint the transitive physical dependencies: selected collision geometry, pin
targets, relevant clips, joint ancestors, face controls and garment body/opening sources.
Skinned/morphing collision surfaces also contribute their relevant deformation weights,
deltas, scalar tracks and joint ancestry. The actual deformed triangles are evaluated at
each physical substep before contact. A direct rigid local-point pin to a deforming surface
is rejected; use an ordinary joint-driven attachment object until vertex/barycentric
attachments have a defined contract. Object-scoped samples and selected USD exports can
inspect a body without requiring unrelated unfinished cloth caches.
Unrelated scenery, cameras, lighting and material edits preserve cache freshness. Dependent
changes invalidate playback until rebaked. Old v1/v2 caches still load as historical data;
the current single-panel v3 evaluator and sewn-panel v4 evaluator require their matching
bakes. Adding deformation support preserves unchanged prior v3/v4 cache fingerprints.
FNV fingerprints
detect accidental edits and are not cryptographic authentication.

See `tests/cloth_editor.rs`, `tests/cloth_dependencies.rs` and the kit's `tests/cloth.rs` for
numerical and regression coverage. `scripts/agent_cloth_acceptance.py` verifies the real
process, independently moving attachments, contact controls, visible cloth deformation with
a frozen-cloth/same-anchor control, restart persistence and forced-kill bake recovery.
