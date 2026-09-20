# Fitted character garments

The editor creates a vest, short-sleeve top or trousers as **separate, hollow SDF geometry** with its own stable object ID and material. Necklines, armholes, hems, waist openings and cuffs are local Boolean cuts: they do not cut the character. Garments follow the evaluated body, including articulated motion, and persist with the project.

This is fitted geometry for the named humanoid recipe. It does **not** simulate fabric, gravity, inertia, stretch, wrinkles, stitching, drape or cloth self-collision. Film cloth simulation and detailed tailoring remain separate production requirements.

Use the separate [loose cloth](CLOTH_SIMULATION.md) and [sewn panel](SEWN_PANELS.md)
operations for implemented cloth dynamics and explicit stitching. The fitted-garment
recipe itself remains a source-following shell.

## Create and edit

A character created by `create_humanoid` supplies 17 physical capsule/ellipsoid parts. Clothing refers to those explicit IDs in authored CSG order. Eyes and generated facial details are excluded from the clothing collision body; an evaluated mouth cavity in the source head is respected.

Send one request per line to `mm3e-editor --root DIRECTORY`:

```json
{"id":"shirt","expected_revision":1,"command":{"op":"apply","operations":[{"op":"create_garment","request":{"id":"hero/shirt","character":"hero","style":"short_sleeve_top","clearance_m":0.012,"thickness_m":0.004,"material":{"albedo":[0.68,0.08,0.045],"roughness":0.85}}}]}}
```

Use the current revision returned by the editor. A garment operation participates in the same atomic transaction, undo and redo history as other edits.

| Parameter | Meaning |
| --- | --- |
| `id` | Unique stable generated object ID; required. |
| `character` | Prefix of the complete named humanoid body; required. |
| `style` | `vest`, `short_sleeve_top` or `trousers`; required. |
| `clearance_m` | Requested offset from the selected body field, `0.0001..0.25`; required. |
| `thickness_m` | Nominal full shell field-band width, `0.0005..0.10`; required. |
| `material` | Independent surface material. Omitting it uses the standard default surface. |

For the same ID, `update_garment` takes the same complete `request` and replaces its parameters without changing object order or identity. Supply the desired material again when updating the full recipe. A material-only edit can use the ordinary `update` operation with `patch.material`. `remove_garment` takes `id` and removes both geometry and its recipe metadata.

Source-body shape and transform edits regenerate the garment. Direct transforms, geometry modifiers, shape replacements, independent garment animation tracks and joint bindings are rejected: garment motion already follows its sources. The source parts must remain unmodified capsules/ellipsoids; unsupported modifiers fail explicitly. Existing source joint and object animation still works.

The current style recipes use fixed openings derived from body geometry: sleeve cuffs at 65% of the upper arm, trouser cuffs at 94% of the shin, the top hem below the abdomen center, and the trouser waist above the pelvis center. These are procedural fitted styles, not editable sewing patterns.

## What the geometry means

For covered body field `d`, clearance `c` and nominal thickness `t`, the initial garment is:

```text
abs(d - (c + t/2)) - t/2
```

A local subtraction then excludes the entire selected physical body expanded by `c`. This handles the smooth-union bulges and body parts outside the garment's coverage selection. Local source-relative cutters open the neck, hem and cuffs. The result is a single continuous CSG garment field with independent material ownership.

Thickness and clearance remain world-space values during source scaling. Ellipsoids and smooth combinations provide approximate distance fields; the parameters are not a promise of exact Euclidean thickness everywhere. Clearance cuts can thin or remove material where another body part intrudes. An extreme pose can change garment connectivity. There is no fabric solver to preserve panel shape or stretch limits.

## Inspect fit at a pose

```json
{"id":"fit","command":{"op":"garment_fit","id":"hero/shirt","animation":{"clip":"dressed_motion","time":1.0},"samples_per_source":64}}
```

Omit `animation` for the rest state. `samples_per_source` defaults to 64 and accepts `16..256`.

The tool projects deterministic directions from covered source parts onto the garment's middle field surface, then brackets inner and outer garment boundaries against the actual evaluated geometry. The report includes:

- Probe count, retained middle-surface points, rejected opening/occluded directions and unbracketed searches.
- Accepted inner/outer surface sample count and minimum body-field clearance.
- Penetration count, clearance-violation count, tolerance and up to 32 failing sample locations.
- `sampled_fit_pass`, which requires at least one surface sample and no measured violation.

`sampled_fit_pass` is a finite sampled result. It does not certify full surface coverage, inter-garment separation, collision with other characters or scenery, self-collision, or motion between sampled times. A layered outfit needs additional checks; this tool compares each garment to its specified body sources only.

`render`, `pick` and `sample` operate on the generated garment's real field and return its stable material identity. `save`/`load` preserve both the recipe and its derived rest CSG snapshot; pose evaluation rebuilds independently and does not accumulate deformation.

## Reproduce the boundary check

```sh
cargo test --offline -p mm3e-editor --test garments
cargo test --offline -p mm3e-editor garment::tests
cargo build --offline --release -p mm3e-editor
python3 scripts/agent_garment_acceptance.py --output artifacts/garment-check-new
```

Use a new output directory. The script retains its full request/response transcript, creates a dressed character, measures rest and animated fit, verifies visible garment materials, checks atomic rejection of direct generated transforms, and saves/reopens in a fresh process. It requires the reopened animated PNG to be byte-identical. Outputs are actual editor renders, not illustrations.
