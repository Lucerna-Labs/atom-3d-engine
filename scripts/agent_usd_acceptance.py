#!/usr/bin/env python3
"""Real-process USDA delivery acceptance with an independent OpenUSD reader.

Run after building mm3e-editor. Install usd-core into a temporary directory and
pass --reader-path if it is not already importable. Requires a NEW output folder.
Preserves dispatched JSONL, replies, errors, native projects, USDA and a JSON report.
This verifies evaluated composed-field polygon caches, including surface thickness;
it does not claim editable SDF/rig/simulation interchange or renderer appearance parity.
Scalar field residual and sampled mesh-convergence distance are different quantities.
"""

import argparse
import hashlib
import json
import math
from pathlib import Path
import select
import struct
import subprocess
import sys
import time
import traceback


ROOT = Path(__file__).resolve().parents[1]
SAMPLE_TIMES = [0.0, 0.5, 1.0]
FPS = 2.0
MATERIALS = {
    "body": {"albedo": [0.32, 0.08, 0.02], "roughness": 0.37, "metallic": 0.0, "specular": 0.25},
    "satellite": {"albedo": [0.06, 0.3, 0.5], "roughness": 0.23, "metallic": 0.8, "specular": 0.7,
                  "emissive": [1.5, 0.01, 0.0]},
    "cutter": {"albedo": [0.04, 0.25, 0.09], "roughness": 0.6, "metallic": 0.0, "specular": 0.5},
    "sheet": {"albedo": [0.3, 0.14, 0.04], "roughness": 0.7, "metallic": 0.0, "specular": 0.5},
}
FILM = {"aperture_radius_m": 0.006, "focus_distance_m": 4.0,
        "shutter_open_seconds": -0.01, "shutter_close_seconds": 0.01, "shutter_samples": 2}
SHEET_POINTS = [[-0.4, -0.3, 0], [0.4, -0.3, 0], [0.4, 0.3, 0], [-0.4, 0.3, 0]]
SHEET_TRIANGLES = [[0, 1, 2], [0, 2, 3]]


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2, allow_nan=False) + "\n")


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def close(actual, expected, tolerance=2e-5):
    if isinstance(expected, (list, tuple)):
        assert len(actual) == len(expected), (actual, expected)
        for a, b in zip(actual, expected):
            close(a, b, tolerance)
    else:
        assert math.isclose(actual, expected, abs_tol=tolerance, rel_tol=tolerance), (actual, expected)


def sub(a, b):
    return [x - y for x, y in zip(a, b)]


def add(a, b):
    return [x + y for x, y in zip(a, b)]


def scale(v, amount):
    return [x * amount for x in v]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def length(v):
    return math.sqrt(dot(v, v))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def normalize(v):
    return scale(v, 1.0 / length(v))


def f32(value):
    return struct.unpack("f", struct.pack("f", value))[0]


def sphere_field(point, center, radius):
    return length(sub(point, center)) - radius


def composed_field(point, seconds, cutter_offset=0.0, satellite_origin=1.2):
    body = sphere_field(point, [0.2 * seconds, 0, 0], 0.6)
    satellite = sphere_field(point, [satellite_origin + 0.35 * seconds, 0, 0], 0.25)
    cutter = sphere_field(point, [cutter_offset + 0.2 * seconds, 0, 0], 0.3)
    return max(min(body, satellite), -cutter)


def sheet_field(point, seconds):
    # The two authored triangles form a rectangle. Its thickened surface is the
    # Euclidean distance to that rectangle minus half the physical thickness.
    p = sub(point, [0.12 * seconds, 0.06 * seconds, 0])
    return length([max(abs(p[0]) - 0.4, 0), max(abs(p[1]) - 0.3, 0), p[2]]) - 0.06


class Client:
    def __init__(self, executable, root, transcript, session):
        self.root = root
        self.session = session
        self.transcript = transcript
        self.sequence = 0
        self.revision = 0
        self.stderr = (root / f"{session}.stderr.log").open("w")
        self.process = subprocess.Popen(
            [str(executable), "--root", str(root), "--project", "project.json"],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=self.stderr, text=True,
        )
        self.call({"op": "inspect"})

    def call(self, command, mutation=False, ok=True):
        self.sequence += 1
        request = {"id": f"{self.session}-{self.sequence}", "command": command}
        if mutation:
            request["expected_revision"] = self.revision
        self.transcript.write(json.dumps({"request": request}, allow_nan=False) + "\n")
        self.transcript.flush()
        started = time.monotonic()
        self.process.stdin.write(json.dumps(request, allow_nan=False) + "\n")
        self.process.stdin.flush()
        if not select.select([self.process.stdout], [], [], 300)[0]:
            raise TimeoutError(f"No editor reply within 300 seconds: {request['id']}")
        line = self.process.stdout.readline()
        if not line:
            raise RuntimeError(f"Editor exited during {request['id']}; see stderr log")
        response = json.loads(line)
        self.transcript.write(json.dumps({"response": response, "elapsed_seconds": time.monotonic() - started}) + "\n")
        self.transcript.flush()
        assert response["id"] == request["id"], response
        if ok is not None:
            assert response["ok"] == ok, response
        self.revision = response["revision"]
        return response["result"] if ok else response

    def apply(self, operations):
        return self.call({"op": "apply", "operations": operations}, mutation=True)

    def close(self):
        self.process.stdin.close()
        self.process.wait(timeout=30)
        self.stderr.close()
        assert self.process.returncode == 0, self.process.returncode

    def abort(self):
        if self.process.poll() is None:
            self.process.kill()
            self.process.wait(timeout=30)
        self.stderr.close()


def animation(seconds):
    return {"clip": "move", "time": seconds}


def track(identifier, translation):
    return {"target": {"type": "object", "id": identifier}, "keys": [
        {"time": 0}, {"time": 1, "translation": translation},
    ]}


def camera_and_settings(tracks):
    return [
        {"op": "put_clip", "clip": {"id": "move", "duration": 1, "tracks": tracks, "camera_keys": [
            {"time": 0, "eye": [0.4, 0.3, 4.0], "target": [0.3, 0, 0], "fov_degrees": 45},
            {"time": 1, "eye": [0.8, 0.5, 3.5], "target": [0.5, 0.1, 0], "fov_degrees": 55},
        ]}},
        {"op": "set_camera", "camera": {"eye": [0.4, 0.3, 4], "target": [0.3, 0, 0],
                                           "up": [0, 1, 0], "fov_degrees": 45}},
        {"op": "set_settings", "settings": {"width": 160, "height": 100, "spatial_aa": 2, "film": FILM}},
    ]


def author_composed(client, open_rim=False):
    operations = []
    for identifier, radius, position in [("body", 0.6, [0, 0, 0]), ("satellite", 0.25, [1 if open_rim else 1.2, 0, 0]),
                                         ("cutter", 0.3, [0.35 if open_rim else 0, 0, 0])]:
        entity = {"id": identifier, "shape": {"type": "sphere", "radius": radius},
                  "position": position, "material": MATERIALS[identifier]}
        if identifier == "cutter":
            entity["combine"] = {"type": "subtract"}
        operations.append({"op": "create", "object": entity})
    operations += camera_and_settings([track("body", [0.2, 0, 0]), track("satellite", [0.35, 0, 0]),
                                       track("cutter", [0.2, 0, 0])])
    client.apply(operations)


def author_sheet(client):
    client.apply([{"op": "create", "object": {
        "id": "sheet", "shape": {"type": "surface", "vertices": SHEET_POINTS,
                                      "triangles": SHEET_TRIANGLES, "thickness_m": 0.12},
        "material": MATERIALS["sheet"],
    }}] + camera_and_settings([track("sheet", [0.12, 0.06, 0])]))


def export_request(path, ids, bounds_min, bounds_max):
    return {"path": path, "object_ids": ids, "bounds_min": bounds_min, "bounds_max": bounds_max,
            "resolution": [32, 32, 32], "max_surface_error_m": 0.075, "max_field_residual": 0.025,
            "camera_near_clip_m": 0.0001,
            "clip": "move", "start_seconds": 0, "end_seconds": 1, "frames_per_second": FPS}


def native_samples(client, points, seconds, object_id=None):
    result = []
    for offset in range(0, len(points), 4096):
        command = {"op": "sample", "points": points[offset:offset + 4096], "animation": animation(seconds)}
        if object_id is not None:
            command["id"] = object_id
        reply = client.call(command)
        result.extend(reply["samples"])
    assert len(result) == len(points)
    return result


def validate_materials(stage, expected_ids, usd):
    UsdShade = usd["UsdShade"]
    seen = set()
    material_by_path = {}
    for prim in stage.Traverse():
        if not prim.IsA(UsdShade.Material):
            continue
        material = UsdShade.Material(prim)
        identity = prim.GetCustomDataByKey("originalMaterialId")
        assert identity in expected_ids, (identity, expected_ids)
        seen.add(identity)
        material_by_path[str(prim.GetPath())] = identity
        wanted = MATERIALS[identity]
        shader, _, _ = material.ComputeSurfaceSource()
        assert shader and shader.GetIdAttr().Get() == "UsdPreviewSurface"
        assert shader.GetInput("useSpecularWorkflow").Get() == 0
        close(shader.GetInput("diffuseColor").Get(), wanted["albedo"])
        close(shader.GetInput("roughness").Get(), wanted["roughness"])
        close(shader.GetInput("metallic").Get(), wanted["metallic"])
        close(shader.GetInput("emissiveColor").Get(), wanted.get("emissive", [0, 0, 0]))
        ior = shader.GetInput("ior").Get()
        close(((ior - 1) / (ior + 1)) ** 2, 0.08 * wanted["specular"])
    assert seen == set(expected_ids), (seen, expected_ids)
    return material_by_path


def validate_camera(stage, client, usd):
    UsdGeom, Gf = usd["UsdGeom"], usd["Gf"]
    cameras = [UsdGeom.Camera(prim) for prim in stage.Traverse() if prim.IsA(UsdGeom.Camera)]
    assert len(cameras) == 1
    camera = cameras[0]
    assert camera.GetFocalLengthAttr().GetTimeSamples() == [t * FPS for t in SAMPLE_TIMES]
    for seconds in SAMPLE_TIMES:
        pose = client.call({"op": "pose", "animation": animation(seconds)})["camera"]
        time_code = seconds * FPS
        matrix = camera.ComputeLocalToWorldTransform(time_code)
        close(matrix.ExtractTranslation(), pose["eye"])
        close(matrix.TransformDir(Gf.Vec3d(0, 0, -1)).GetNormalized(), pose["forward"])
        close(matrix.TransformDir(Gf.Vec3d(0, 1, 0)).GetNormalized(), pose["up"])
        close(matrix.GetDeterminant(), 1)
        gf_camera = camera.GetCamera(time_code)
        close(gf_camera.GetFieldOfView(Gf.Camera.FOVVertical), pose["fov_degrees"])
        close(gf_camera.aspectRatio, 1.6)
        close(gf_camera.verticalAperture * Gf.Camera.APERTURE_UNIT, 0.024)
        focal_m = gf_camera.focalLength * Gf.Camera.FOCAL_LENGTH_UNIT
        close(focal_m, 0.012 / math.tan(math.radians(pose["fov_degrees"]) / 2))
        close(focal_m / (2 * gf_camera.fStop), FILM["aperture_radius_m"], 1e-7)
        close(gf_camera.focusDistance, FILM["focus_distance_m"])
        close(gf_camera.clippingRange.GetMin(), 0.0001, 1e-8)
        projection = gf_camera.frustum.ComputeProjectionMatrix()
        assert all(math.isfinite(projection[row][column]) for row in range(4) for column in range(4))
        assert abs(projection.GetDeterminant()) > 0, "consumer camera has a singular perspective projection"
        close(camera.GetShutterOpenAttr().Get(time_code), FILM["shutter_open_seconds"] * FPS)
        close(camera.GetShutterCloseAttr().Get(time_code), FILM["shutter_close_seconds"] * FPS)


def connected_components(point_count, triangles):
    parent = list(range(point_count))

    def find(index):
        while parent[index] != index:
            parent[index] = parent[parent[index]]
            index = parent[index]
        return index

    used = set()
    for triangle in triangles:
        used.update(triangle)
        a, b, c = map(find, triangle)
        parent[b] = a
        parent[c] = a
    assert used == set(range(point_count)), "mesh contains unused positions"
    return len({find(index) for index in range(point_count)})


def validate_delivery(path, client, request, analytic, usd, object_id=None, expected_components=3):
    Usd, UsdGeom, UsdShade, UsdValidation = (usd[name] for name in ["Usd", "UsdGeom", "UsdShade", "UsdValidation"])
    stage = Usd.Stage.Open(str(path))
    assert stage, path
    assert stage.GetDefaultPrim().GetPath() == "/World"
    assert UsdGeom.GetStageMetersPerUnit(stage) == 1
    assert UsdGeom.GetStageUpAxis(stage) == "Y"
    assert stage.GetFramesPerSecond() == FPS
    assert stage.GetTimeCodesPerSecond() == FPS
    assert stage.GetStartTimeCode() == 0 and stage.GetEndTimeCode() == 2
    assert stage.GetRootLayer().customLayerData["representation"] == "evaluatedPolygonMeshCache"
    assert stage.GetRootLayer().customLayerData["colorEncoding"] == "scene-linear Rec.709/D65 RGB"
    registry = UsdValidation.ValidationRegistry()
    metadata = [m for m in registry.GetAllValidatorMetadata()
                if set(m.GetKeywords()) & {"UsdCoreValidators", "UsdGeomValidators", "UsdShadeValidators"}]
    assert metadata
    validators = [registry.GetOrLoadValidatorByName(m.name) for m in metadata]
    errors = UsdValidation.ValidationContext(validators).Validate(stage)
    assert not errors, [str(error) for error in errors]
    material_by_path = validate_materials(stage, request["object_ids"], usd)
    meshes = [UsdGeom.Mesh(prim) for prim in stage.Traverse() if prim.IsA(UsdGeom.Mesh)]
    assert len(meshes) == 1, "delivery must contain the selected composed field"
    mesh = meshes[0]
    assert set(mesh.GetPrim().GetCustomDataByKey("originalEntityIds")) == set(request["object_ids"])
    assert mesh.GetSubdivisionSchemeAttr().Get() == "none"
    assert mesh.GetOrientationAttr().Get() == "rightHanded"
    assert mesh.GetPointsAttr().GetTimeSamples() == [seconds * FPS for seconds in SAMPLE_TIMES]
    subsets = UsdShade.MaterialBindingAPI(mesh).GetMaterialBindSubsets()
    assert UsdGeom.Subset.GetFamilyType(mesh, "materialBind") == "partition"
    valid, reason = UsdGeom.Subset.ValidateFamily(mesh, "face", "materialBind")
    assert valid, reason
    frames = []
    previous = None
    topology_changes = False
    for seconds in SAMPLE_TIMES:
        time_code = seconds * FPS
        points = [list(point) for point in mesh.GetPointsAttr().Get(time_code)]
        counts = list(mesh.GetFaceVertexCountsAttr().Get(time_code))
        indices = list(mesh.GetFaceVertexIndicesAttr().Get(time_code))
        assert points and counts and all(count == 3 for count in counts)
        valid, reason = UsdGeom.Mesh.ValidateTopology(indices, counts, len(points))
        assert valid, reason
        triangles = [indices[offset:offset + 3] for offset in range(0, len(indices), 3)]
        component_count = connected_components(len(points), triangles)
        assert component_count == expected_components, (component_count, expected_components)
        assert all(math.isfinite(value) for point in points for value in point)
        if previous is not None:
            topology_changes |= len(previous[0]) != len(points) or previous[1] != indices
        previous = points, indices
        expected_extent = [[min(point[axis] for point in points) for axis in range(3)],
                           [max(point[axis] for point in points) for axis in range(3)]]
        close(mesh.GetExtentAttr().Get(time_code), expected_extent)
        assert all(request["bounds_min"][axis] < expected_extent[0][axis]
                   and expected_extent[1][axis] < request["bounds_max"][axis] for axis in range(3))
        face_materials = {}
        for subset in subsets:
            material, relationship = UsdShade.MaterialBindingAPI(subset).ComputeBoundMaterial()
            assert material and relationship
            source = material_by_path[str(material.GetPath())]
            for face in subset.GetIndicesAttr().Get(time_code):
                assert face not in face_materials
                face_materials[face] = source
        assert set(face_materials) == set(range(len(triangles)))
        normals = mesh.GetNormalsAttr().Get(time_code)
        assert len(normals) == len(triangles)
        assert mesh.GetNormalsInterpolation() == "uniform"
        centers, midpoints, offset_points, edges = [], [], [], {}
        for triangle, normal in zip(triangles, normals):
            a, b, c = [points[index] for index in triangle]
            expected_normal = normalize(cross(sub(b, a), sub(c, a)))
            close(normal, expected_normal)
            # Match the native f32 centroid calculation before querying ownership;
            # a double-precision recomputation could pick the other side of a tie.
            center = [f32(f32(f32(x + y) + z) * f32(1 / 3)) for x, y, z in zip(a, b, c)]
            centers.append(center)
            offset_points.extend([add(center, scale(normal, 0.001)), add(center, scale(normal, -0.001))])
            midpoints.extend([scale(add(a, b), 0.5), scale(add(b, c), 0.5), scale(add(c, a), 0.5)])
            for edge in [(triangle[0], triangle[1]), (triangle[1], triangle[2]), (triangle[2], triangle[0])]:
                key = tuple(sorted(edge))
                edges[key] = edges.get(key, 0) + 1
        assert all(count == 2 for count in edges.values()), "closed field mesh has boundary/nonmanifold edges"
        tested = points + centers + midpoints
        samples = native_samples(client, tested, seconds, object_id)
        native_residual = max(abs(sample["value"]) for sample in samples)
        analytic_residual = max(abs(analytic(point, seconds)) for point in tested)
        for point, sample in zip(tested, samples):
            close(sample["value"], analytic(point, seconds), 1e-5)
        assert native_residual <= request["max_field_residual"] + 1e-5, native_residual
        for index, sample in enumerate(samples[len(points):len(points) + len(centers)]):
            assert face_materials[index] == sample["material_owner_id"], (index, face_materials[index], sample)
        oriented = native_samples(client, offset_points, seconds, object_id)
        backwards = sum(oriented[index]["value"] + 1e-5 < oriented[index + 1]["value"]
                        for index in range(0, len(oriented), 2))
        assert backwards == 0, backwards
        frames.append({"seconds": seconds, "time_code": time_code, "vertices": len(points),
                       "triangles": len(triangles), "extent": expected_extent,
                       "native_field_max_abs_residual": native_residual,
                       "analytic_field_max_abs_residual": analytic_residual,
                       "independently_checked_surface_samples": len(tested),
                       "backwards_faces": backwards,
                       "connected_components": component_count,
                       "points_sha256": hashlib.sha256(json.dumps(points).encode()).hexdigest()})
    assert frames[0]["points_sha256"] != frames[-1]["points_sha256"], "export remained in rest pose"
    assert frames[-1]["extent"][1][0] - frames[0]["extent"][1][0] > 0.09, "animated translation was lost"
    if topology_changes:
        assert mesh.GetFaceVertexIndicesAttr().GetTimeSamples() == [seconds * FPS for seconds in SAMPLE_TIMES]
        assert mesh.GetFaceVertexCountsAttr().GetTimeSamples() == [seconds * FPS for seconds in SAMPLE_TIMES]
    validate_camera(stage, client, usd)
    return {"path": str(path), "sha256": sha256(path), "schema_validators": len(validators),
            "schema_validator_names": sorted(m.name for m in metadata),
            "frames": frames, "topology_changes": topology_changes,
            "field_residual_units": "authored scalar; distinct from mesh-convergence distance in meters"}


def artifact_inventory(root):
    return {str(path.relative_to(root)): sha256(path) for path in root.rglob("*")
            if path.is_file() and (path.suffix in {".usda", ".tmp"} or path.name == "project.json")}


def reject_export(client, request, report):
    before = artifact_inventory(client.root)
    revision = client.revision
    response = client.call({"op": "export_usd", "request": request}, ok=False)
    assert client.revision == revision
    assert artifact_inventory(client.root) == before, "failed export changed delivery/project files"
    report.append({"request": request, "error": response["error"], "revision_unchanged": True,
                   "project_and_output_inventory_unchanged": True})


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable", "--binary", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--reader-path", type=Path, default=Path("/tmp/mm3e-usd-reader"))
    args = parser.parse_args()
    if args.reader_path.is_dir():
        sys.path.insert(0, str(args.reader_path))
    try:
        from pxr import Usd, UsdGeom, UsdShade, UsdValidation, Gf
    except ImportError as error:
        parser.error(f"Independent OpenUSD (usd-core) reader with UsdValidation required: {error}")
    usd = {"Usd": Usd, "UsdGeom": UsdGeom, "UsdShade": UsdShade, "UsdValidation": UsdValidation, "Gf": Gf}
    executable = args.executable.resolve(strict=True)
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    started = time.monotonic()
    report = {"status": "running", "passed": False, "binary": str(executable),
              "harness_sha256": sha256(Path(__file__)),
              "binary_sha256": sha256(executable), "reader": "OpenUSD " + ".".join(map(str, Usd.GetVersion())),
              "cases": [], "rejections": [], "scope": "evaluated composed-field mesh-cache interchange",
              "required_open_rim_challenge": {"status": "not_run", "required": True},
              "limits": ["no editable SDF, skeleton, skin or simulation interchange",
                         "no image parity claim for another renderer",
                         "sampled convergence diagnostics are not a certified Hausdorff bound",
                         "lights and environment are outside this geometry/material/camera delivery"]}
    report_path = root / "acceptance.json"
    write_json(report_path, report)
    client = None
    try:
        with (root / "transcript.jsonl").open("w") as transcript:
            for case, author, ids, bounds_min, bounds_max, analytic in [
                ("composed", author_composed, ["body", "satellite", "cutter"], [-1, -0.85, -0.85], [2.1, 0.85, 0.85], composed_field),
                ("thick-surface", author_sheet, ["sheet"], [-0.7, -0.6, -0.3], [0.9, 0.7, 0.3], sheet_field),
            ]:
                case_root = root / case
                case_root.mkdir()
                client = Client(executable, case_root, transcript, f"{case}-author")
                author(client)
                before = client.call({"op": "get_document"})
                project_hash = sha256(case_root / "project.json")
                revision = client.revision
                request = export_request("animated.usda", ids, bounds_min, bounds_max)
                delivery = client.call({"op": "export_usd", "request": request})
                assert client.revision == revision and sha256(case_root / "project.json") == project_hash
                case_report = {"case": case, "export_response": delivery,
                               "independent_reader": validate_delivery(case_root / "animated.usda", client, request, analytic, usd,
                                                                        expected_components=3 if case == "composed" else 1)}
                report["cases"].append(case_report)
                write_json(report_path, report)
                if case == "thick-surface":
                    for frame in case_report["independent_reader"]["frames"]:
                        close(frame["extent"][0][2], -0.06, 0.002)
                        close(frame["extent"][1][2], 0.06, 0.002)
                        assert frame["vertices"] > len(SHEET_POINTS), "export substituted the unthickened midsheet"
                if case == "composed":
                    selected = dict(request, path="body-only.usda", object_ids=["body"])
                    client.call({"op": "export_usd", "request": selected})
                    case_report["object_selection"] = validate_delivery(
                        case_root / "body-only.usda", client, selected,
                        lambda p, t: sphere_field(p, [0.2 * t, 0, 0], 0.6), usd, "body", expected_components=1)
                    for overrides in [
                        {"path": "unknown.usda", "object_ids": ["missing"]},
                        {"path": "duplicate.usda", "object_ids": ["body", "body"]},
                        {"path": "subtract-seed.usda", "object_ids": ["cutter"]},
                        {"path": "clipped.usda", "bounds_min": [-0.1, -0.1, -0.1], "bounds_max": [0.1, 0.1, 0.1]},
                        {"path": "resolution.usda", "resolution": [7, 8, 8]},
                        {"path": "accuracy.usda", "resolution": [8, 8, 8], "max_surface_error_m": 0.000001, "max_field_residual": 0.000001},
                        {"path": "clip.usda", "clip": "missing"},
                        {"path": "range.usda", "end_seconds": -1},
                        {"path": "near-clip.usda", "camera_near_clip_m": 0},
                        {"path": "wrong-extension.usd"},
                        {"path": "animated.usda"},
                    ]:
                        reject_export(client, dict(request, **overrides), report["rejections"])
                    for field, value in [("checker", True), ("reflectivity", 0.2)]:
                        client.apply([{"op": "update", "id": "body", "patch": {"material": dict(MATERIALS["body"], **{field: value})}}])
                        reject_export(client, dict(request, path=f"unsupported-{field}.usda"), report["rejections"])
                        client.apply([{"op": "update", "id": "body", "patch": {"material": MATERIALS["body"]}}])
                    # Restore expected state after the rejection fixtures, with new revisions retained.
                    assert client.call({"op": "get_document"}) == before
                saved_document = client.call({"op": "get_document"})
                saved_revision = client.revision
                client.close()
                client = Client(executable, case_root, transcript, f"{case}-cold-reopen")
                assert client.revision == saved_revision
                assert client.call({"op": "get_document"}) == saved_document
                reopened = dict(request, path="reopened.usda")
                client.call({"op": "export_usd", "request": reopened})
                assert (case_root / "animated.usda").read_bytes() == (case_root / "reopened.usda").read_bytes()
                case_report["cold_reload"] = {"document_equal": True, "revision": saved_revision,
                                               "delivery_bytes_identical": True, "sha256": sha256(case_root / "reopened.usda")}
                client.close()
                client = None
                write_json(report_path, report)
            # Keep the original sharp open-rim CSG case mandatory and visible.
            # A successful simpler fixture never turns this required gate green.
            challenge_root = root / "open-rim-challenge"
            challenge_root.mkdir()
            client = Client(executable, challenge_root, transcript, "open-rim-author")
            author_composed(client, open_rim=True)
            challenge = export_request("open-rim.usda", ["body", "satellite", "cutter"], [-1, -0.85, -0.85], [1.9, 0.85, 0.85])
            challenge["resolution"] = [64, 64, 64]
            report["required_open_rim_challenge"] = {"status": "running", "request": challenge,
                                                     "expected_connected_components": 2}
            write_json(report_path, report)
            inventory = artifact_inventory(challenge_root)
            response = client.call({"op": "export_usd", "request": challenge}, ok=None)
            if response["ok"]:
                report["required_open_rim_challenge"].update({
                    "status": "passed", "export_response": response["result"],
                    "independent_reader": validate_delivery(challenge_root / "open-rim.usda", client, challenge,
                        lambda p, t: composed_field(p, t, 0.35, 1.0), usd, expected_components=2),
                })
                report.update({"status": "passed", "passed": True})
            else:
                assert artifact_inventory(challenge_root) == inventory, "rejected challenge changed files"
                report["required_open_rim_challenge"].update({"status": "failed", "error": response["error"],
                                                             "output_absent": True})
                report.update({"status": "partial", "passed": False,
                               "unresolved": "Required sharp open-rim CSG export did not meet the original quality gates"})
            client.close()
            client = None
    except Exception:
        report.update({"status": "failed", "passed": False, "failure": traceback.format_exc()})
        challenge_status=report.get("required_open_rim_challenge",{})
        if challenge_status.get("status")=="running":
            challenge_status.update({"status":"failed","failure":report["failure"]})
        raise
    finally:
        if client is not None:
            client.abort()
        report["elapsed_seconds"] = time.monotonic() - started
        write_json(report_path, report)
        print(json.dumps({"status": report["status"], "report": str(report_path), "elapsed_seconds": report["elapsed_seconds"]}))
    if not report["passed"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
