#!/usr/bin/env python3
"""Actual JSONL authoring and rendering of continuous native eyelid and lip surfaces.

Agent-designed geometry proof, not a measured anatomical or production facial rig.
Preserves the complete transcript, source project, poses, images and failures in a new
directory. Uses no fixture renderer, placeholder meshes, or image substitutions.
"""
import argparse
import copy
import hashlib
import json
import math
from pathlib import Path
import time
import traceback

from agent_animation_acceptance import Client, by_id, make_playback, write_json
from agent_sewing_acceptance import png_pixels, pixel_difference

ROOT = Path(__file__).resolve().parents[1]
HEAD = [0.0, 1.4, -0.16]
HEAD_RADIUS = 0.42
EYE_RADIUS = 0.1
LID_RADIUS = 0.11
LID_THICKNESS = 0.003
SKIN = {"albedo": [0.63, 0.37, 0.25], "roughness": 0.75}
LIP = {"albedo": [0.44, 0.065, 0.075], "roughness": 0.65}
EYES = {"left": [-0.18, 1.55, 0.19], "right": [0.18, 1.55, 0.19]}
TIMES = [round(i * 0.05, 8) for i in range(17)]
FRAMES = [("open", 0.0), ("half-closing", 0.2), ("closed-open-mouth", 0.4),
          ("half-opening", 0.6), ("return", 0.8)]


def add(a, b):
    return [x + y for x, y in zip(a, b)]


def sub(a, b):
    return [x - y for x, y in zip(a, b)]


def mix(a, b, t):
    return [x + (y - x) * t for x, y in zip(a, b)]


def length(v):
    return math.sqrt(sum(x * x for x in v))


def normalize(v):
    norm = length(v)
    assert norm > 0
    return [x / norm for x in v]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def grid_triangles(rows, columns):
    result = []
    for row in range(rows - 1):
        for col in range(columns - 1):
            a = row * columns + col
            b = a + 1
            c = a + columns
            d = c + 1
            result.extend([[a, c, b], [b, c, d]])
    return result


def annulus_triangles(rows, columns):
    result = []
    for row in range(rows - 1):
        for col in range(columns):
            nxt = (col + 1) % columns
            a, b = row * columns + col, row * columns + nxt
            c, d = (row + 1) * columns + col, (row + 1) * columns + nxt
            result.extend([[a, c, b], [b, c, d]])
    return result


def lid_geometry(closure):
    rows, columns = 9, 17
    minimum = 0.28 - 1.80 * closure
    vertices = []
    for row in range(rows):
        latitude = minimum + (1.52 - minimum) * row / (rows - 1)
        for col in range(columns):
            longitude = -1.52 + 3.04 * col / (columns - 1)
            vertices.append([LID_RADIUS * math.cos(latitude) * math.sin(longitude),
                             LID_RADIUS * math.sin(latitude),
                             LID_RADIUS * math.cos(latitude) * math.cos(longitude)])
    return vertices, grid_triangles(rows, columns)


def continuous_lid_clearance():
    """Conservative support-plane bound over every triangle and every time interval.

    A triangle during an adjacent morph transition lies inside the convex hull of its
    six endpoint vertices. Its projection onto a fixed unit support direction is at
    least the minimum of those six projections. Distance to the sphere center is no
    smaller than that positive projection. Head motion is shared rigid motion, so the
    bound survives it. Subtract the actual lid shell radius as well as eyeball radius.
    """
    shapes = [lid_geometry(k / 8)[0] for k in range(9)]
    triangles = lid_geometry(0)[1]
    clearance = math.inf
    for a, b in zip(shapes, shapes[1:]):
        for triangle in triangles:
            hull = [shape[index] for shape in (a, b) for index in triangle]
            normal = normalize([sum(p[axis] for p in hull) for axis in range(3)])
            clearance = min(clearance, min(dot(point, normal) for point in hull)
                            - EYE_RADIUS - LID_THICKNESS / 2)
    assert clearance > 0.001, clearance
    return clearance


def lip_geometry(opening):
    rows, segments = 3, 32
    vertices, jaw_weights = [], []
    for row in range(rows):
        radial = row / (rows - 1)
        a = 0.11 + (0.15 - 0.11) * radial
        inner_b = 0.005 + 0.040 * opening
        outer_b = 0.018 + 0.043 * opening
        b = inner_b + (outer_b - inner_b) * radial
        for segment in range(segments):
            angle = 2 * math.pi * segment / segments
            vertices.append([a * math.cos(angle), 1.34 + b * math.sin(angle),
                             0.284 + 0.010 * math.sin(math.pi * radial)])
            jaw_weights.append(0.65 * max(0, -math.sin(angle)))
    return vertices, annulus_triangles(rows, segments), jaw_weights


def mouth_skin_geometry(opening):
    rows, segments = 6, 32
    vertices, jaw_weights = [], []
    for row in range(rows):
        radial = row / (rows - 1)
        for segment in range(segments):
            angle = 2 * math.pi * segment / segments
            outer = [0.24 * math.cos(angle), 1.34 + 0.18 * math.sin(angle)]
            outer_z = HEAD[2] + math.sqrt(HEAD_RADIUS ** 2 - outer[0] ** 2 - (outer[1] - HEAD[1]) ** 2)
            inner = [0.135 * math.cos(angle), 1.34 + (0.013 + 0.039 * opening) * math.sin(angle), 0.283]
            # Outer border follows the spherical head. Inner border overlaps the actual lip.
            vertices.append(mix(inner, [*outer, outer_z], radial))
            jaw_weights.append(0.65 * max(0, -math.sin(angle)) * (1 - radial) ** 2)
    return vertices, annulus_triangles(rows, segments), jaw_weights


def influence_rows(jaw_weights):
    return [[{"joint": 0, "weight": 1 - w}, {"joint": 1, "weight": w}]
            if w > 1e-8 else [{"joint": 0, "weight": 1.0}] for w in jaw_weights]


def object_surface(identifier, vertices, triangles, material, position=None, thickness=0.003):
    return {"id": identifier, "position": position or [0, 0, 0],
            "shape": {"type": "surface", "vertices": vertices, "triangles": triangles,
                      "thickness_m": thickness}, "material": material}


def scene_recipe():
    cavity_cut = {"type": "transform", "position": [0, -0.06, 0.41],
                  "shape": {"type": "primitive", "shape": {"type": "round_box",
                            "half_extents": [0.13, 0.11, 0.14], "radius": 0.015}}}
    objects = [{"id": "face/head", "position": HEAD, "material": SKIN,
                "shape": {"type": "csg", "expression": {"type": "subtract",
                          "a": {"type": "primitive", "shape": {"type": "sphere", "radius": HEAD_RADIUS}},
                          "b": cavity_cut}}},
               {"id": "face/mouth-interior", "position": [0, 1.34, 0.115],
                "shape": {"type": "box", "half_extents": [0.127, 0.105, 0.04]},
                "material": {"albedo": [0.026, 0.006, 0.009], "roughness": 0.85}},
               {"id": "face/nose", "shape": {"type": "capsule", "a": [0, 1.51, 0.24],
                                             "b": [0, 1.42, 0.285], "radius": 0.035}, "material": SKIN}]
    deformers, morph_tracks = [], []
    for side, center in EYES.items():
        eye_id, lid_id = f"face/{side}-eye", f"face/{side}-lid"
        objects.append({"id": eye_id, "position": center,
                        "shape": {"type": "sphere", "radius": EYE_RADIUS},
                        "material": {"albedo": [0.72, 0.84, 0.92], "roughness": 0.3}})
        vertices, triangles = lid_geometry(0)
        objects.append(object_surface(lid_id, vertices, triangles, SKIN, center, LID_THICKNESS))
        morphs = []
        for k in range(1, 9):
            target, _ = lid_geometry(k / 8)
            morph_id = f"blink-{k}"
            morphs.append({"id": morph_id, "deltas": [sub(b, a) for a, b in zip(vertices, target)]})
            # Hat weights interpolate adjacent curved targets, never a long unsafe chord.
            morph_tracks.append({"deformer": lid_id, "blendshape": morph_id, "easing": "linear",
                                 "keys": [{"time": t, "weight": 1 if i in (k, 16 - k) else 0}
                                          for i, t in enumerate(TIMES)]})
        deformers.append({"id": lid_id, "object": lid_id, "joints": ["face/head-joint"],
                          "weights": [[{"joint": 0, "weight": 1}] for _ in vertices], "blendshapes": morphs})
    for identifier, geometry, material in [("face/lips", lip_geometry, LIP),
                                           ("face/mouth-skin", mouth_skin_geometry, SKIN)]:
        vertices, triangles, jaw_weights = geometry(0)
        target, _, _ = geometry(1)
        objects.append(object_surface(identifier, vertices, triangles, material))
        deformers.append({"id": identifier, "object": identifier,
                          "joints": ["face/head-joint", "face/jaw-joint"],
                          "weights": influence_rows(jaw_weights),
                          "blendshapes": [{"id": "mouth-open", "deltas": [sub(b, a) for a, b in zip(vertices, target)]}]})
        morph_tracks.append({"deformer": identifier, "blendshape": "mouth-open",
                             "keys": [{"time": 0, "weight": 0}, {"time": 0.4, "weight": 1},
                                      {"time": 0.8, "weight": 0}]})
    rigid_objects = [obj["id"] for obj in objects if obj["shape"]["type"] != "surface"]
    joints = [{"id": "face/head-joint", "pivot": HEAD, "objects": rigid_objects},
              {"id": "face/jaw-joint", "parent": "face/head-joint", "pivot": [0, 1.33, 0.08]}]
    tracks = [{"target": {"type": "joint", "id": identifier}, "keys": [
        {"time": 0}, {"time": 0.4, "rotation_degrees": angles}, {"time": 0.8}]
        } for identifier, angles in [("face/head-joint", [0, 12, 0]), ("face/jaw-joint", [8, 0, 0])]]
    clip = {"id": "expression", "duration": 0.8, "tracks": tracks, "morph_tracks": morph_tracks}
    return objects, joints, deformers, clip


def animation(t):
    return {"clip": "expression", "time": t}


def sha256(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def transform_point(pose_object, rest_point, rest_position):
    local = sub(rest_point, rest_position)
    return [pose_object["position"][axis] + pose_object["scale"] * sum(
        local[col] * pose_object["basis"][col][axis] for col in range(3)) for axis in range(3)]


def probe_batches(client, identifier, points, t):
    values = []
    for start in range(0, len(points), 512):
        command = {"op": "sample", "points": points[start:start + 512], "animation": animation(t)}
        if identifier is not None:
            command["id"] = identifier
        values.extend(client.call(command)["samples"])
    return values


def lid_probes(state):
    vertices = state["vertices"]
    points, edges = list(vertices), set()
    for a, b, c in state["triangles"]:
        for i, j in [(a, b), (b, c), (c, a)]:
            edge = tuple(sorted((i, j)))
            if edge not in edges:
                edges.add(edge)
                points.append(mix(vertices[i], vertices[j], 0.5))
        points.append([(vertices[a][axis] + vertices[b][axis] + vertices[c][axis]) / 3 for axis in range(3)])
    return points


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--editor", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    executable = args.editor.resolve(strict=True)
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    started = time.monotonic()
    report = {"status": "running", "kind": "actual JSONL native face deformation acceptance",
              "executable": str(executable), "binary_sha256": sha256(executable),
              "script_sha256": sha256(Path(__file__).resolve()), "backend": "MM3E CPU", "renders": {},
              "limits": "Agent-designed continuous geometry proof; no anatomical fitting, tissue simulation, teeth, tongue, speech alignment or production quality certification."}
    write_json(root / "acceptance.json", report)
    client = None
    try:
        objects, joints, deformers, clip = scene_recipe()
        report["continuous_lid_shell_clearance_bound_m"] = continuous_lid_clearance()
        report["continuous_clearance_scope"] = "All points of every retained lid triangle over every adjacent piecewise linear morph interval, including shell thickness; conservative support of the six endpoint vertices; common rigid eye/head motion preserves the bound."
        report["authored_native_vertices"] = sum(len(obj["shape"].get("vertices", [])) for obj in objects)
        report["authored_native_triangles"] = sum(len(obj["shape"].get("triangles", [])) for obj in objects)
        with (root / "transcript.jsonl").open("w") as transcript:
            client = Client(executable, root, transcript, "author")
            write_json(root / "capabilities.json", client.call({"op": "describe"}))
            operations = [{"op": "create", "object": obj} for obj in objects]
            operations.append({"op": "set_joints", "joints": joints})
            operations.extend({"op": "bind_surface", "request": {"deformer": asset}} for asset in deformers)
            operations.extend([
                {"op": "put_clip", "clip": clip},
                {"op": "set_settings", "settings": {"width": 256, "height": 256, "quality": "full",
                 "spatial_aa": 2, "shadows": False, "ao": False, "exposure": 1.0}},
                {"op": "set_camera", "camera": {"eye": [0, 1.42, 1.9], "target": [0, 1.42, 0.04], "fov_degrees": 29}},
            ])
            client.apply(operations)
            document = client.call({"op": "get_document"})
            write_json(root / "authored-document.json", document)
            client.call({"op": "validate"})
            # Both malformed dense edits and scalar edits reach the actual transaction API.
            revision = client.revision
            bad = copy.deepcopy(deformers[0]["blendshapes"])
            bad[0]["deltas"].pop()
            client.apply([{"op": "update_deformer", "request": {"id": deformers[0]["id"], "blendshapes": bad}}], expect_ok=False)
            assert client.revision == revision and client.call({"op": "get_document"}) == document
            client.apply([{"op": "set_morph_weights", "id": deformers[0]["id"],
                           "weights": [{"id": "blink-1", "weight": 2}]}], expect_ok=False)
            assert client.revision == revision and client.call({"op": "get_document"}) == document
            rest = client.call({"op": "deformer_state", "id": deformers[0]["id"]})
            client.apply([{"op": "set_morph_weights", "id": deformers[0]["id"],
                           "weights": [{"id": "blink-1", "weight": 0.5}]}])
            changed = client.call({"op": "get_document"})
            assert client.call({"op": "deformer_state", "id": deformers[0]["id"]})["vertices"] != rest["vertices"]
            assert changed["objects"] == document["objects"]
            client.call({"op": "undo"}, True)
            assert client.call({"op": "get_document"}) == document
            client.call({"op": "redo"}, True)
            assert client.call({"op": "get_document"}) == changed
            client.call({"op": "undo"}, True)
            assert client.call({"op": "deformer_state", "id": deformers[0]["id"]}) == rest
            report["invalid_edit_rollback_and_valid_morph_undo_redo"] = True

            decoded, states = {}, {}
            for name, seconds in FRAMES:
                states[name] = {asset["id"]: client.call({"op": "deformer_state", "id": asset["id"], "animation": animation(seconds)}) for asset in deformers}
                write_json(root / f"{name}-native-geometry.json", states[name])
                rendered = client.call({"op": "render", "path": f"{name}.png", "animation": animation(seconds)})
                decoded[name], png_info = png_pixels(root / f"{name}.png")
                # Decoder's garment-specific color count is irrelevant to this face fixture.
                png_info.pop("red_garment_pixels")
                rendered["independent_png"] = png_info
                report["renders"][name] = rendered
                write_json(root / "acceptance.json", report)
            eye_ids = [f"face/{side}-eye" for side in EYES]
            owner_pixels = {name: rendered["metrics"]["visible_material_owner_pixels"] for name, rendered in report["renders"].items()}
            for eye in eye_ids:
                assert owner_pixels["open"].get(eye, 0) > 0, (eye, owner_pixels)
                assert owner_pixels["closed-open-mouth"].get(eye, 0) == 0, (eye, owner_pixels)
                assert owner_pixels["return"].get(eye, 0) == owner_pixels["open"].get(eye, 0)
            cavity = "face/mouth-interior"
            assert owner_pixels["closed-open-mouth"].get(cavity, 0) > owner_pixels["open"].get(cavity, 0) + 20, owner_pixels
            assert decoded["open"] == decoded["return"]
            assert states["open"] != states["closed-open-mouth"]
            for identifier in states["open"]:
                assert states["open"][identifier]["vertices"] == states["return"][identifier]["vertices"]
                assert states["open"][identifier]["triangles"] == states["closed-open-mouth"][identifier]["triangles"]
            report["changed_pixels_open_to_closed"] = pixel_difference(decoded["open"], decoded["closed-open-mouth"])
            assert report["changed_pixels_open_to_closed"] > 300

            # Actual native eyelid vertices, unique-edge midpoints and triangle centroids
            # are queried against the actual posed eyes throughout closure and reopening.
            clearance_rows, native_points = [], 0
            for index in range(33):
                seconds = round(index * 0.025, 8)
                pose = by_id(client.call({"op": "pose", "animation": animation(seconds)})["objects"])
                for side in EYES:
                    lid_id, eye_id = f"face/{side}-lid", f"face/{side}-eye"
                    state = client.call({"op": "deformer_state", "id": lid_id, "animation": animation(seconds)})
                    points = lid_probes(state)
                    eye_values = probe_batches(client, eye_id, points, seconds)
                    clearance = min(value["value"] for value in eye_values) - LID_THICKNESS / 2
                    assert clearance > 0.001, (seconds, side, clearance)
                    # Independently compare sphere-distance math with the queried native eye.
                    center = pose[eye_id]["position"]
                    mismatch = max(abs(value["value"] - (math.dist(point, center) - EYE_RADIUS))
                                   for point, value in zip(points, eye_values))
                    assert mismatch < 2e-6, (seconds, side, mismatch)
                    own = probe_batches(client, lid_id, points, seconds)
                    assert max(value["value"] for value in own) < -LID_THICKNESS / 2 + 2e-6
                    native_points += len(points)
                    clearance_rows.append({"time": seconds, "lid": lid_id, "points": len(points),
                                           "minimum_shell_clearance_m": clearance, "sphere_check_error_m": mismatch})
                write_json(root / "clearance-probes.json", clearance_rows)
            report["native_surface_probe_count"] = native_points
            report["actual_eye_shell_clearance_min_m"] = min(row["minimum_shell_clearance_m"] for row in clearance_rows)
            report["clearance_sample_times"] = 33
            report["eye_owner_pixels"] = {name: {eye: owner_pixels[name].get(eye, 0) for eye in eye_ids} for name, _ in FRAMES}
            report["cavity_owner_pixels"] = {name: owner_pixels[name].get(cavity, 0) for name, _ in FRAMES}

            # A real rest boundary point moves into clear air inside the opened mouth.
            rest_boundary = states["open"]["face/mouth-skin"]["vertices"][8]
            air_values = []
            for seconds in (0.0, 0.4):
                pose = by_id(client.call({"op": "pose", "animation": animation(seconds)})["objects"])
                point = transform_point(pose["face/head"], rest_boundary, HEAD)
                value = probe_batches(client, None, [point], seconds)[0]["value"]
                air_values.append({"time": seconds, "point": point, "world_field": value})
            report["mouth_boundary_to_air"] = air_values
            assert air_values[0]["world_field"] < -0.001
            assert air_values[1]["world_field"] > 0.008, air_values
            assert client.call({"op": "get_document"}) == document
            client.call({"op": "save", "path": "deformed-face.mm3e-agent.json"})
            client.close()
            client = None

            # Cold process, native document loader, the same actual geometry and PNG bytes.
            client = Client(executable, root, transcript, "cold-reopen")
            client.call({"op": "load", "path": "deformed-face.mm3e-agent.json"}, True)
            assert client.call({"op": "get_document"}) == document
            for name, seconds in FRAMES:
                for asset in deformers:
                    assert client.call({"op": "deformer_state", "id": asset["id"], "animation": animation(seconds)}) == states[name][asset["id"]]
                client.call({"op": "render", "path": f"reopened-{name}.png", "animation": animation(seconds)})
                assert (root / f"reopened-{name}.png").read_bytes() == (root / f"{name}.png").read_bytes()
            client.close()
            client = None
            assert sha256(executable) == report["binary_sha256"], "executable changed during acceptance; rerun with one fixed build"
            report["cold_reopen_native_geometry_and_all_five_pngs_exact"] = True
            report["source_objects_unchanged_by_deformation"] = True
            report["five_pose_playback"] = make_playback([root / f"{name}.png" for name, _ in FRAMES[:-1]], root / "deformed-face-playback.png", 5)
            report["status"] = "passed"
    except Exception as error:
        report.update({"status": "failed", "error": str(error), "traceback": traceback.format_exc()})
        raise
    finally:
        if client is not None:
            client.abort()
        report["elapsed_seconds"] = time.monotonic() - started
        write_json(root / "acceptance.json", report)
    print(json.dumps({key: report[key] for key in ["status", "elapsed_seconds", "eye_owner_pixels", "cavity_owner_pixels", "actual_eye_shell_clearance_min_m"]}, indent=2))


if __name__ == "__main__":
    main()
