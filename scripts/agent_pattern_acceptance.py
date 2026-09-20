#!/usr/bin/env python3
"""Actual JSONL outline/hole pattern authoring, sewing, simulation and cold-reload proof.

Creates concave garment panels with explicit arm holes and a neckline. Native triangle
fields, image ownership and opening area are checked; an unsewn assembly and a filled-hole
mesh are independent negative controls. This is not measured pattern CAD or fabric fitting.
"""
import argparse
import copy
import hashlib
import json
import math
from pathlib import Path
import time
import traceback

from agent_animation_acceptance import Client, by_id, write_json
from agent_sewing_acceptance import png_pixels, pixel_difference

ROOT = Path(__file__).resolve().parents[1]
EYE = [0.0, 1.05, 2.2]
FOV = 38.0
SIZE = 256
FRAMES = [("rest", 0.0), ("middle", 0.15), ("posed", 0.3)]


def sha256(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def loop(identifier, points):
    return {"id": identifier, "points": [{"id": f"{identifier}-{i}", "position": point}
                                         for i, point in enumerate(points)]}


def recipe(holes=True):
    left = [[-.4, 0], [0, 0], [0, .64], [-.08, .66], [-.13, .8], [-.32, .8], [-.4, .6]]
    panels = []
    for side, factor in [("left", 1), ("right", -1)]:
        outer = [[x * factor, y] for x, y in left]
        opening = [[(-.27 + .065 * math.cos(2 * math.pi * i / 8)) * factor,
                    .50 + .085 * math.sin(2 * math.pi * i / 8)] for i in range(8)]
        panels.append({"id": side, "origin": [0, .9, 0], "axis_u": [1, 0, 0], "axis_v": [0, 1, 0],
                       "outer": loop("perimeter", outer), "holes": [loop("arm-opening", opening)] if holes else [],
                       "max_edge_m": .2,
                       "pins": [{"vertex": {"type": "control", "id": f"perimeter-{i}"},
                                 "target_object": "anchor", "point": [left[i][0], 0, 0]} for i in (4, 5)] if side == "left" else []})
    return {"id": "garment", "label": "Concave two-panel garment with arm openings", "group": "wardrobe",
            "panels": panels, "seams": [], "thickness_m": .002, "vertex_mass_kg": .02,
            "collision_object_ids": ["support"],
            "settings": {"iterations": 48, "substeps": 4, "bend_compliance": .02, "max_seam_error_m": .001},
            "material": {"albedo": [.68, .08, .06], "roughness": .75}}


def source_operations():
    return [{"op": "create", "object": {"id": "anchor", "position": [0, 1.7, 0],
                                           "shape": {"type": "sphere", "radius": .015}}},
            # Floor below the panels is an actual selected collision field. The control
            # reaches it during its fall; source contact is separately measured below.
            {"op": "create", "object": {"id": "support", "shape": {"type": "plane", "normal": [0, 1, 0], "offset": -.55},
                                           "material": {"albedo": [.12, .16, .19], "roughness": .9}}},
            {"op": "set_joints", "joints": [{"id": "shoulder", "pivot": [0, 1.7, 0], "objects": ["anchor"]}]},
            {"op": "put_clip", "clip": {"id": "pose", "duration": .3, "tracks": [
                {"target": {"type": "joint", "id": "shoulder"}, "keys": [{"time": 0}, {"time": .3, "rotation_degrees": [0, 10, 8]}]}]}},
            {"op": "set_camera", "camera": {"eye": EYE, "target": [0, EYE[1], 0], "fov_degrees": FOV}},
            {"op": "set_settings", "settings": {"width": SIZE, "height": SIZE, "quality": "full",
                                                   "spatial_aa": 2, "shadows": False, "ao": False}}]


def sample(seconds):
    return {"clip": "pose", "time": seconds}


def ordered_seam(preview):
    """Author this fixture's known u=0 seam from bottom to top on BOTH panels.

    The explicit chosen direction is part of the fixture, not an editor guess. Only
    reported outer-boundary indices are selected; every final edge vertex is retained.
    """
    boundary = next(loop for loop in preview["boundary_loops"] if loop["id"] == "perimeter")
    indices = [i for i in boundary["vertices"] if abs(preview["points"][i][0]) < 1e-12]
    return sorted(indices, key=lambda i: preview["points"][i][1])


def prepare(client, request, sewn):
    previews = [client.call({"op": "preview_pattern_panel", "panel": panel}) for panel in request["panels"]]
    a, b = [ordered_seam(p) for p in previews]
    assert len(a) == len(b) >= 2
    assert [previews[0]["points"][i] for i in a] == [previews[1]["points"][i] for i in b]
    request["seams"] = [{"panel_a": "left", "chain_a": a, "panel_b": "right", "chain_b": b,
                         "rest_length_m": 0, "compliance": 0}] if sewn else []
    return previews


def signed_area(points):
    return sum(a[0] * b[1] - a[1] * b[0] for a, b in zip(points, points[1:] + points[:1])) / 2


def check_preview(panel, mesh):
    expected = abs(signed_area([p["position"] for p in panel["outer"]["points"]]))
    expected -= sum(abs(signed_area([p["position"] for p in hole["points"]])) for hole in panel["holes"])
    actual = 0
    for triangle in mesh["triangles"]:
        points = [mesh["points"][i] for i in triangle]
        area = signed_area(points)
        assert area > 0
        actual += area
        for a, b in zip(points, points[1:] + points[:1]):
            assert math.dist(a, b) <= panel["max_edge_m"] + 1e-12
    assert abs(actual - expected) < 1e-12, (actual, expected)
    for boundary in [panel["outer"], *panel["holes"]]:
        for point in boundary["points"]:
            assert mesh["points"][mesh["control_vertices"][point["id"]]] == point["position"]
    assert len(mesh["boundary_loops"]) == 1 + len(panel["holes"])
    return {"area_m2": actual, "vertices": len(mesh["points"]), "triangles": len(mesh["triangles"]),
            "boundary_loop_names": [b["id"] for b in mesh["boundary_loops"]], "work": mesh["triangulation_work"]}


def cross(a, b):
    return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]


def opening_geometry(state, mesh):
    boundary = next(loop for loop in mesh["boundary_loops"] if loop["id"] == "arm-opening")
    points = [state["vertices"][i + mesh["vertex_offset"]] for i in boundary["vertices"]]
    center = [sum(p[axis] for p in points) / len(points) for axis in range(3)]
    area = [0.0, 0.0, 0.0]
    for a, b in zip(points, points[1:] + points[:1]):
        edge = cross([a[i] - center[i] for i in range(3)], [b[i] - center[i] for i in range(3)])
        area = [x + y for x, y in zip(area, edge)]
    return center, math.sqrt(sum(v*v for v in area)) / 2


def project(point):
    depth = EYE[2] - point[2]
    factor = SIZE / (2 * math.tan(math.radians(FOV) / 2) * depth)
    return [SIZE/2 + point[0] * factor, SIZE/2 - (point[1]-EYE[1]) * factor]


def query(client, identifier, points, seconds):
    return client.call({"op": "sample", "id": identifier, "points": points, "animation": sample(seconds)})["samples"]


def seam_gap(state, request, meshes):
    a, b = meshes
    # The same explicit correspondence is evaluated even in the no-seam negative control.
    chain_a, chain_b = ordered_seam(a), ordered_seam(b)
    return max(math.dist(state["vertices"][i + a["vertex_offset"]], state["vertices"][j + b["vertex_offset"]])
               for i, j in zip(chain_a, chain_b))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--editor", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    executable, root = args.editor.resolve(strict=True), args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    report = {"status": "running", "binary": str(executable), "binary_sha256": sha256(executable),
              "script_sha256": sha256(Path(__file__).resolve()), "cases": {},
              "limits": "Native concave planar outline/hole and explicit sewing proof; no darts, allowances, grading, measured weave or general garment fitting certification."}
    started, client, pixels = time.monotonic(), None, {}
    try:
        with (root / "transcript.jsonl").open("w") as transcript:
            for name, sewn, holes in [("sewn-openings", True, True), ("unsewn-control", False, True), ("filled-hole-control", True, False)]:
                case_root = root / name
                case_root.mkdir()
                client = Client(executable, case_root, transcript, name)
                client.apply(source_operations())
                requested = recipe(holes)
                revision = client.revision
                previews = prepare(client, requested, sewn)
                assert client.revision == revision, "preview changed editor history"
                checks = [check_preview(panel, mesh) for panel, mesh in zip(requested["panels"], previews)]
                assert sum(check["vertices"] for check in checks) <= 256
                write_json(case_root / "requested-pattern.json", requested)
                write_json(case_root / "preview-meshes.json", previews)
                client.apply([{"op": "create_pattern_cloth", "request": requested}])
                authored = client.call({"op": "get_document"})
                asset = authored["cloths"][0]
                for retained, source in zip(asset["pattern"]["panels"], requested["panels"]):
                    assert retained["id"] == source["id"] and retained["outer"] == source["outer"] and retained["holes"] == source["holes"]
                    assert math.dist(retained["origin"], source["origin"]) < 1e-6
                case = {"preview": checks, "frames": {}, "seams": requested["seams"]}
                report["cases"][name] = case
                # Invalid density, crossing outline and noncontiguous seam edits are atomic.
                if name == "sewn-openings":
                    failures = []
                    for kind in ("too-dense", "crossing-outline", "noncontiguous-seam"):
                        bad = copy.deepcopy(requested)
                        if kind == "too-dense":
                            bad["panels"][0]["max_edge_m"] = .001
                        elif kind == "crossing-outline":
                            bad["panels"][0]["outer"]["points"][1], bad["panels"][0]["outer"]["points"][4] = bad["panels"][0]["outer"]["points"][4], bad["panels"][0]["outer"]["points"][1]
                        else:
                            bad["seams"][0]["chain_a"] = bad["seams"][0]["chain_a"][::2]
                            bad["seams"][0]["chain_b"] = bad["seams"][0]["chain_b"][::2]
                        previous_revision = client.revision
                        failures.append(client.apply([{"op": "update_pattern_cloth", "request": bad}], expect_ok=False))
                        assert client.revision == previous_revision and client.call({"op": "get_document"}) == authored
                    case["invalid_edits"] = failures
                baked = client.call({"op": "bake_cloth", "request": {"id": "garment", "clip": "pose"}}, True)
                case["bake"] = baked
                assert client.call({"op": "get_document"})["objects"] == authored["objects"]
                states = {}
                for frame, seconds in FRAMES:
                    state = client.call({"op": "cloth_state", "id": "garment", "animation": sample(seconds)})
                    states[frame] = state
                    assert state["sample_within_bake_tolerances"]
                    assert state["triangles"] == asset["triangles"]
                    assert state["pattern"] == asset["pattern"]
                    values = query(client, "garment", state["vertices"], seconds)
                    assert max(point["value"] for point in values) < -0.00099
                    contacts = query(client, "support", state["vertices"], seconds)
                    assert min(point["value"] for point in contacts) >= .001, "actual cloth crossed its selected support field"
                    rendered = client.call({"op": "render", "path": f"{frame}.png", "animation": sample(seconds)})
                    pixels[(name, frame)], png = png_pixels(case_root / f"{frame}.png")
                    assert png["red_garment_pixels"] > 300
                    info = {"render": rendered, "png": png, "seam_gap_m": seam_gap(state, requested, asset["pattern"]["meshes"]),
                            "minimum_support_field_m": min(point["value"] for point in contacts)}
                    case["frames"][frame] = info
                    if sewn:
                        assert info["seam_gap_m"] < .001
                    if holes:
                        openings = []
                        for panel, mesh in zip(requested["panels"], asset["pattern"]["meshes"]):
                            center, area = opening_geometry(state, mesh)
                            expected = abs(signed_area([p["position"] for p in panel["holes"][0]["points"]]))
                            assert area > .5 * expected, (name, frame, area, expected)
                            value = query(client, "garment", [center], seconds)[0]["value"]
                            assert value > .025, (name, frame, "hole capped", value)
                            x, y = project(center)
                            hits = [client.call({"op": "pick", "x": x+dx, "y": y+dy, "animation": sample(seconds)}) for dx, dy in [(0,0),(-1,0),(1,0),(0,-1),(0,1)]]
                            assert all(hit.get("material_owner_id") != "garment" for hit in hits), (name, frame, hits)
                            openings.append({"panel": panel["id"], "center": center, "area_m2": area,
                                             "rest_area_m2": expected, "native_field_m": value, "hole_rays": hits})
                        info["openings"] = openings
                    write_json(case_root / f"{frame}-native-state.json", state)
                    write_json(root / "acceptance.json", report)
                assert pixel_difference(pixels[(name,"rest")], pixels[(name,"posed")]) > 300
                if name == "sewn-openings":
                    # Concave neckline is an authored outer opening, not a capped polygon.
                    neck = query(client, "garment", [[0,1.65,0]], 0)[0]["value"]
                    assert neck > .04, neck
                    case["rest_neckline_field_m"] = neck
                    cached = client.call({"op": "get_document"})
                    update = copy.deepcopy(requested)
                    update["panels"][0]["holes"][0]["points"][0]["position"][0] += .002
                    prepare(client, update, True)
                    client.apply([{"op": "update_pattern_cloth", "request": update}])
                    assert client.call({"op": "get_document"})["cloths"][0]["cache"] is None
                    client.call({"op": "undo"}, True)
                    assert client.call({"op": "get_document"}) == cached
                    case["pattern_update_invalidates_cache_and_undo_restores_exact"] = True
                saved = client.call({"op": "get_document"})
                client.call({"op": "save", "path": "native-pattern.mm3e-agent.json"})
                client.close()
                client = Client(executable, case_root, transcript, name + "-cold-reopen")
                client.call({"op": "load", "path": "native-pattern.mm3e-agent.json"}, True)
                assert client.call({"op": "get_document"}) == saved
                reopened = client.call({"op": "cloth_state", "id": "garment", "animation": sample(.3)})
                assert reopened == states["posed"]
                client.call({"op": "render", "path": "reopened-posed.png", "animation": sample(.3)})
                assert (case_root/"reopened-posed.png").read_bytes() == (case_root/"posed.png").read_bytes()
                case["cold_reopen_native_topology_cache_and_pixels_exact"] = True
                # The filled negative control must actually occupy the rest hole centers.
                if name == "filled-hole-control":
                    values = query(client, "garment", [[-.27,1.4,0],[.27,1.4,0]], 0)
                    assert all(v["value"] < -.00099 for v in values), values
                    case["filled_hole_center_fields"] = values
                    for center in [[-.27,1.4,0],[.27,1.4,0]]:
                        x,y=project(center)
                        hit=client.call({"op":"pick","x":x,"y":y,"animation":sample(0)})
                        assert hit.get("material_owner_id") == "garment", hit
                client.close()
                client = None
            a, b, c = [report["cases"][name] for name in ("sewn-openings", "unsewn-control", "filled-hole-control")]
            assert a["frames"]["posed"]["seam_gap_m"] < .001 < .1 < b["frames"]["posed"]["seam_gap_m"]
            assert b["bake"]["diagnostics"]["contact_projections"] > 0, "support contact was never exercised"
            assert sum(p["area_m2"] for p in c["preview"]) > sum(p["area_m2"] for p in a["preview"]) + .02
            report["sewn_vs_unsewn_changed_pixels"] = pixel_difference(pixels[("sewn-openings","posed")],pixels[("unsewn-control","posed")])
            report["holes_vs_filled_changed_pixels"] = pixel_difference(pixels[("sewn-openings","rest")],pixels[("filled-hole-control","rest")])
            assert report["sewn_vs_unsewn_changed_pixels"] > 300 and report["holes_vs_filled_changed_pixels"] > 100
            assert sha256(executable) == report["binary_sha256"], "binary changed during acceptance"
            report["status"] = "passed"
    except Exception as error:
        report.update({"status":"failed","error":str(error),"traceback":traceback.format_exc()})
        raise
    finally:
        if client is not None: client.abort()
        report["elapsed_seconds"] = time.monotonic() - started
        write_json(root / "acceptance.json", report)
    print(json.dumps({k:report[k] for k in ("status","elapsed_seconds","sewn_vs_unsewn_changed_pixels","holes_vs_filled_changed_pixels")},indent=2))


if __name__ == "__main__":
    main()
