# Native garment outlines and holes

`preview_pattern_panel` triangulates a named 2D outline and its holes without
changing the document revision. `create_pattern_cloth` and `update_pattern_cloth`
are operations inside the root protocol's atomic `apply` command. They assemble
the triangulated panels into native cloth with explicit pins and ordered seams.

The implementation accepts concave outlines, including an open U-shaped neckline,
and closed interior holes such as armholes. The resulting `Shape::Surface` retains
the actual triangles and open boundaries. Holes contain no cap triangles; the
native distance field adds the authored shell thickness and rounded boundary rims.
Cloth playback changes vertex positions while preserving that topology.

## Coordinates, identity and triangulation

A `PatternPanel` defines an `origin`, perpendicular unit `axis_u` and `axis_v`,
one named `outer` loop, and zero or more named `holes`. Each loop contains named
controls with `position: [u, v]` in metres. Arrays describe closed loops without
repeating the first control at the end. Loop IDs must be unique within a panel;
control IDs must be unique across all loops in that panel. Panel IDs must be unique
within the garment.

Either input winding is accepted. Preview reports the outer `boundary_loops`
counterclockwise and hole loops clockwise in the `(u, v)` plane. Every triangle
is counterclockwise. The first authored control remains the first vertex of its
reported loop. Original controls occupy an unchanged prefix of `points`: outer
controls first, followed by each hole's controls in caller order. Named
`control_vertices` map those IDs to final **local** vertex indices.

The kernel validates contours, connects holes using visible bridges, clips ears,
and optionally bisects the longest edges. An interior edge split updates both
incident triangles; a boundary split also updates its reported boundary loop.
`max_edge_m` bounds every edge of the final 2D triangulation, including interior
diagonals. Omitting it, or supplying `null`, disables refinement. Changing the
outline or refinement target can change appended vertex IDs and seam chains, so
preview the final panel definition before selecting those chains.

Self-intersections, overlapping or touching contours, repeated controls, edge
backtracking, exterior holes, and nested holes are rejected. Normalized f64
predicates reject distinct contour features within `1e-12` of the normalization
scale as too close. Unsupported numerical precision, work exhaustion, or vertex
exhaustion returns an error without a partial mesh or reduced-detail substitute.
Straight-edge controls are retained.

Authored 2D positions, retained triangulation points, and `max_edge_m` use f64.
The editor enables `serde_json`'s `float_roundtrip` feature so JSON project reload
preserves these floating-point values exactly. Placement produces the native
engine's f32 world vertices using `origin + axis_u * u + axis_v * v`; compilation
also validates the resulting native surface.

## Preview a panel

Send one JSON request per line to `mm3e-editor --root DIRECTORY`. The formatted
examples below are complete JSON objects; compact each object onto one line when
sending it over the process protocol. Preview requires no `expected_revision`.

```json
{
  "id": "preview-left",
  "command": {
    "op": "preview_pattern_panel",
    "panel": {
      "id": "left",
      "origin": [0, 0.6, 0],
      "axis_u": [1, 0, 0],
      "axis_v": [0, 1, 0],
      "outer": {
        "id": "outline",
        "points": [
          {"id": "bottom-left", "position": [-0.4, 0]},
          {"id": "bottom-seam", "position": [0, 0]},
          {"id": "top-seam", "position": [0, 0.8]},
          {"id": "top-left", "position": [-0.4, 0.8]}
        ]
      },
      "holes": [{
        "id": "opening",
        "points": [
          {"id": "opening-0", "position": [-0.3, 0.35]},
          {"id": "opening-1", "position": [-0.15, 0.35]},
          {"id": "opening-2", "position": [-0.15, 0.55]},
          {"id": "opening-3", "position": [-0.3, 0.55]}
        ]
      }],
      "max_edge_m": null
    }
  }
}
```

The response's `result` is a `PatternPanelMesh`: `panel`, `points`, `triangles`,
named `boundary_loops`, named `control_vertices`, `vertex_offset`,
`triangle_offset`, and `triangulation_work`. Standalone preview offsets are zero.
The example retains `bottom-seam` at local index `1` and `top-seam` at `2`; its
unrefined right boundary is the contiguous chain `[1, 2]`.

## Create with explicit seams and named pins

This complete request creates two panels on a fresh revision-zero document. The
left panel uses the previewed outline and hole. Its right edge and the right
panel's left edge coincide initially. Both supplied seam chains run bottom to top.

```json
{
  "id": "create-pattern",
  "expected_revision": 0,
  "command": {
    "op": "apply",
    "operations": [{
      "op": "create_pattern_cloth",
      "request": {
        "id": "garment",
        "label": "Two-panel garment with opening",
        "group": "wardrobe",
        "panels": [
          {
            "id": "left",
            "origin": [0, 0.6, 0],
            "axis_u": [1, 0, 0],
            "axis_v": [0, 1, 0],
            "outer": {
              "id": "outline",
              "points": [
                {"id": "bottom-left", "position": [-0.4, 0]},
                {"id": "bottom-seam", "position": [0, 0]},
                {"id": "top-seam", "position": [0, 0.8]},
                {"id": "top-left", "position": [-0.4, 0.8]}
              ]
            },
            "holes": [{
              "id": "opening",
              "points": [
                {"id": "opening-0", "position": [-0.3, 0.35]},
                {"id": "opening-1", "position": [-0.15, 0.35]},
                {"id": "opening-2", "position": [-0.15, 0.55]},
                {"id": "opening-3", "position": [-0.3, 0.55]}
              ]
            }],
            "pins": [{
              "vertex": {"type": "control", "id": "top-left"},
              "point": [-0.4, 1.4, 0]
            }]
          },
          {
            "id": "right",
            "origin": [0, 0.6, 0],
            "axis_u": [1, 0, 0],
            "axis_v": [0, 1, 0],
            "outer": {
              "id": "outline",
              "points": [
                {"id": "bottom-seam", "position": [0, 0]},
                {"id": "bottom-right", "position": [0.4, 0]},
                {"id": "top-right", "position": [0.4, 0.8]},
                {"id": "top-seam", "position": [0, 0.8]}
              ]
            },
            "pins": [{
              "vertex": {"type": "control", "id": "top-right"},
              "point": [0.4, 1.4, 0]
            }]
          }
        ],
        "seams": [{
          "panel_a": "left", "chain_a": [1, 2],
          "panel_b": "right", "chain_b": [0, 3],
          "rest_length_m": 0, "compliance": 0
        }],
        "thickness_m": 0.002,
        "vertex_mass_kg": 0.02,
        "settings": {"max_seam_error_m": 0.001},
        "material": {"albedo": [0.6, 0.1, 0.08], "roughness": 0.75}
      }
    }]
  }
}
```

Each seam pairs `chain_a[i]` with `chain_b[i]`. Chains use final panel-local
indices, have equal lengths of at least two vertices, and follow contiguous
boundary edges without repeated vertices. Refined boundary vertices must appear
when traversing their edges. The editor never sorts or automatically reverses
either chain, including after loop winding normalization. Supply the desired
partner order explicitly. Duplicate stitch pairs and same-panel seams are
rejected. Seams constrain separate vertices without welding them.

Pins accept either `{"type":"control","id":"top-left"}` or
`{"type":"index","index":3}` as their `vertex`. A missing or null
`target_object` makes `point` a fixed world-space target. With `target_object`,
`point` is in that object's local space and follows its evaluated transform and
joint ancestry. Initial targets must meet the authored rest vertices within the
cloth tolerance; baking does not silently snap the initial geometry into place.

## Inspect, edit and retain the native recipe

```json
{"id":"inspect-pattern","command":{"op":"cloth_state","id":"garment"}}
```

`cloth_state` returns retained `pattern` metadata, native vertices and triangles,
and global `stitch_pairs`. Each retained panel mesh has explicit vertex and
triangle offsets into the garment arrays. A local seam or pin index becomes a
global index by adding that panel's `vertex_offset`.

For replacement, use `update_pattern_cloth` inside `apply` with the same complete
request structure and existing garment ID. Supply the current
`expected_revision`, re-preview edited panels, and explicitly select the new
final seam chains. Successful replacement preserves document order and clears the
previous bake cache. Failed validation preserves the prior document, revision,
and cache. Ordinary `undo` and `redo` retain the native states. Use `remove_cloth`
to remove the asset.

`ClothAsset.pattern` retains `triangulation_algorithm`, the complete named panel
recipes and placements, explicit seams, `vertex_mass_kg`, the final meshes and
identity mappings, and total `triangulation_work`. Its triangulation identifier is
`planar-outlines-holes-midpoint-v1`. Validation rebuilds the recipe deterministically
and requires exact agreement with retained topology, controls, loops, work,
rest vertices, pins, and masses. Saving these fields preserves the actual authored
recipe and final mesh; inconsistent retained provenance is rejected on compilation.

Pattern cloth caches use `xpbd-dihedral-outline-pattern-v5`. The source fingerprint
includes the complete pattern metadata along with physical cloth inputs and
relevant animated collision/attachment sources. FNV-1a64 detects accidental
changes; it is not a cryptographic signature. Stale caches require a new
`bake_cloth`. The existing [cloth simulation workflow](CLOTH_SIMULATION.md) covers
clips, baking, pose inspection, and rendered playback. Sampled animated states
still enforce the authored seam and physical tolerances.

## Bounds and implemented scope

| Bound | Editor contract |
| --- | --- |
| Pattern panels | 1–16 per garment |
| Vertices | 256 total across all panels, including refinement |
| Triangulation work | 1,000,000 total across all panels in one garment assembly |
| Cloth assets | 8 total in the document, shared with other cloth authoring paths |
| Seam chains / stitch pairs | At most 256 chains / 1,024 unique pairs per garment |
| 2D coordinates | Finite values in `[-20, 20]` metres |
| `max_edge_m` | Optional finite value in `[0.00001, 40]` metres |
| Panel axes | Unit length and perpendicular within `1e-5` |
| `thickness_m` | `[0.0005, 0.05]` metres, subject to collision-thickness validation |
| `vertex_mass_kg` | `[0.0001, 100]` kilograms before pinning |

Each standalone preview receives the full 256-vertex and 1,000,000-work limits.
Assembly gives each successive panel only the remaining garment budget; separately
successful previews can therefore exceed the combined limit. Existing bake-work,
cached-vertex, frame, and physical-step limits also apply.

This implements outline triangulation, holes, controlled refinement, named pins,
explicit inter-panel sewing, and native cloth dynamics. Darts, seam allowances,
grading, inferred seam correspondence, and a measured physical weave model are
not implemented. Current cloth collision checks do not certify every possible
surface intersection or continuous arbitrary motion.

## Focused verification

```sh
cargo test --offline -p mm3e-kit --test pattern
cargo test --offline -p mm3e-editor --test pattern_cloth
```

The [kernel tests](../mm3e-kit/tests/pattern.rs) cover L and U outlines, two armholes,
six curved holes with varied order and winding, independent area and coverage,
boundary incidence and Euler topology, native `TriangleSurface` acceptance,
refined seam chains, exact work/vertex limits, and numerical failure cases.
The [editor tests](../mm3e-editor/tests/pattern_cloth.rs) cover preview identity,
native sewn/unsewn dynamics and hole sampling, atomic invalid updates, explicit
reversed correspondence, retained-provenance rejection, undo, and cold reload.
The API definitions are in [editor pattern authoring](../mm3e-editor/src/pattern.rs),
[root protocol](../mm3e-editor/src/protocol.rs), and the
[triangulation kernel](../mm3e-kit/src/pattern.rs).

## Actual process and image acceptance

```sh
cargo build --offline --release -p mm3e-editor
python3 scripts/agent_pattern_acceptance.py \
  --editor target/release/mm3e-editor \
  --output artifacts/my-pattern-acceptance
```

The script authors a concave neckline, one named closed arm opening in each of
two panels, explicit paired seam chains, and named control pins following an
animated shoulder joint. It uses the actual preview, create, update, bake, sample,
pick, render, save and load operations. The source pattern has 106 vertices and
166 triangles within the existing garment limits; all final edges are at most
0.2 metres. The 2D material area is 0.56174588 square metres, independently checked
against the polygon area minus its explicit holes.

At the measured posed frame, the sewn boundary remains coincident; the otherwise
identical unsewn panel separates by 0.37209 metres. That falling control also
executes 687 actual support-contact projections. The two sewn opening areas remain
0.0156247 and 0.0156282 square metres. Their centers are more than 60.9 mm from the
native triangle shell, and five actual camera rays through each opening avoid the
garment material in every displayed pose. A separate filled-hole control contains
actual cap geometry at those locations: its field is negative and camera rays hit
the garment. Removing the holes changes 966 rendered pixels; removing the seams
changes 20,288 pixels in the posed frame. The concave neckline is also queried as
open native geometry.

All three cases retain their actual rest, intermediate and posed PNGs, full native
states, source recipes, preview mappings, bake diagnostics and complete JSONL
transcript. Invalid density, crossing-outline and noncontiguous-seam edits preserve
the document and revision. A valid pattern update clears its cache; undo restores
the exact prior pattern and bake. Each case is saved and loaded in a new process,
which reproduces its retained topology, cache, sampled state and posed PNG bytes.
The report records SHA-256 hashes of the executable and script and rejects a
binary that changes during the run. Failed runs retain their evidence in the
requested output directory.

The [final release acceptance](../artifacts/pattern-release-20260906-final-r2/acceptance.json)
passed on binary SHA-256
`ef85b823a8eac88968409310b836faf224dfe7f9ef399d9c602726940840b152`.
Its [posed sewn garment](../artifacts/pattern-release-20260906-final-r2/sewn-openings/posed.png),
[unsewn control](../artifacts/pattern-release-20260906-final-r2/unsewn-control/posed.png),
and [saved editable pattern](../artifacts/pattern-release-20260906-final-r2/sewn-openings/native-pattern.mm3e-agent.json)
are retained with the report. The earlier
[f64 cold-reload failure fixture](../artifacts/pattern-f64-pre-fix-repro/README.md)
is preserved separately.
