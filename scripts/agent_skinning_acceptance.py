#!/usr/bin/env python3
"""Verify native skinning and baked USD delivery through a real persistent JSONL process.

Uses independent analytic joint/morph formulas and a brute-force triangle-distance
oracle, then opens the actual three-pose USD output with OpenUSD. Creates a NEW artifact
folder and retains failures, dispatched requests, source projects, images and fingerprints.
This is mechanism and interchange acceptance, not anatomical or film-quality certification.
USD is a baked polygon cache, not editable UsdSkel or blend-shape interchange.
"""
import argparse
import copy
import hashlib
import json
import math
from pathlib import Path
import sys
import time
import traceback

from agent_usd_acceptance import Client, add, sub, scale, dot, cross, length, close, sha256, write_json, connected_components
from agent_sewing_acceptance import png_pixels, pixel_difference

ROOT = Path(__file__).resolve().parents[1]
TIMES = [0.0, 0.5, 1.0]
POSITION = [0.15, 1.2, 0.0]
RADIUS = 0.08
THICKNESS = 0.025
SEGMENTS = 16
PROFILES = [0.0, 0.3, 1.0, 0.3, 0.0]
UPPER_WEIGHTS = [0.0, 0.0, 0.5, 1.0, 1.0]
MATERIAL = {"albedo": [0.2, 0.46, 0.68], "roughness": 0.43, "metallic": 0.0, "specular": 0.3}
CAMERA = {"eye": [0.45, 1.35, 1.5], "target": POSITION, "up": [0, 1, 0], "fov_degrees": 30}


def animation(seconds):
    return {"clip": "twist", "time": seconds}


def fixture():
    vertices, triangles, weights, deltas, parameters = [], [], [], [], []
    for ring, (profile, upper) in enumerate(zip(PROFILES, UPPER_WEIGHTS)):
        y = -0.3 + ring * 0.15
        for segment in range(SEGMENTS):
            angle = math.tau * segment / SEGMENTS
            x, z = RADIUS * math.cos(angle), RADIUS * math.sin(angle)
            vertices.append([x, y, z])
            deltas.append([x * 0.25 * profile, 0, z * 0.25 * profile])
            weights.append([{"joint": joint, "weight": weight}
                            for joint, weight in [(0, 1 - upper), (1, upper)] if weight > 0])
            parameters.append((upper, profile))
            if ring:
                a = (ring - 1) * SEGMENTS + segment
                b = (ring - 1) * SEGMENTS + (segment + 1) % SEGMENTS
                c, d = a + SEGMENTS, b + SEGMENTS
                triangles.extend([[a, c, b], [b, c, d]])
    for ring in [0, 4]:
        center = len(vertices)
        vertices.append([0, -0.3 + ring * 0.15, 0])
        deltas.append([0, 0, 0])
        weights.append([{"joint": 0 if ring == 0 else 1, "weight": 1}])
        parameters.append((0.0 if ring == 0 else 1.0, 0.0))
        for segment in range(SEGMENTS):
            a = ring * SEGMENTS + segment
            b = ring * SEGMENTS + (segment + 1) % SEGMENTS
            triangles.append([center, b, a] if ring == 0 else [center, a, b])
    asset = {"id": "skin", "object": "body", "method": "dual_quaternion",
             "joints": ["lower", "upper"], "weights": weights,
             "blendshapes": [{"id": "bulge", "deltas": deltas, "weight": 0.0}]}
    clip = {"id": "twist", "duration": 1.0, "tracks": [
        {"target": {"type": "joint", "id": joint}, "keys": [
            {"time": 0}, {"time": 1, "rotation_degrees": [0, angle, 0]}]}
        for joint, angle in [("lower", -70), ("upper", 140)]],
        "morph_tracks": [{"deformer": "skin", "blendshape": "bulge",
                          "keys": [{"time": 0, "weight": 0}, {"time": 1, "weight": 1}]}]}
    operations = [
        {"op": "create", "object": {"id": "body", "position": POSITION, "material": MATERIAL,
            "shape": {"type": "surface", "vertices": vertices, "triangles": triangles,
                      "thickness_m": THICKNESS}}},
        {"op": "set_joints", "joints": [
            {"id": "lower", "pivot": [0.15, 1.0, 0]},
            {"id": "upper", "parent": "lower", "pivot": [0.15, 1.3, 0]}]},
        {"op": "bind_surface", "request": {"deformer": asset}},
        {"op": "put_clip", "clip": clip},
        {"op": "set_camera", "camera": CAMERA},
        {"op": "set_settings", "settings": {"width": 180, "height": 260, "quality": "balanced",
            "exposure": 1.0, "shadows": False, "ao": False, "spatial_aa": 1}},
    ]
    return operations, vertices, triangles, parameters


def analytic_positions(vertices, parameters, seconds, method):
    """Independent opposed-axis closed form; no native pose matrices/readbacks used."""
    theta = math.radians(70 * seconds)
    result = []
    for (x, y, z), (upper, profile) in zip(vertices, parameters):
        # Morph is applied before the weighted rotations, in source rest-local axes.
        growth = 1 + 0.25 * profile * seconds
        x, z = x * growth, z * growth
        if method == "dual_quaternion":
            angle = 2 * math.atan2((2 * upper - 1) * math.sin(theta / 2), math.cos(theta / 2))
            c, s = math.cos(angle), math.sin(angle)
        else:
            c, s = math.cos(theta), (2 * upper - 1) * math.sin(theta)
        result.append(add([c * x + s * z, y, -s * x + c * z], POSITION))
    return result


def point_triangle_distance(point, triangle):
    """Closest point from independent barycentric Voronoi-region classification."""
    a, b, c = triangle
    ab, ac, ap = sub(b, a), sub(c, a), sub(point, a)
    d1, d2 = dot(ab, ap), dot(ac, ap)
    if d1 <= 0 and d2 <= 0:
        return length(ap)
    bp = sub(point, b)
    d3, d4 = dot(ab, bp), dot(ac, bp)
    if d3 >= 0 and d4 <= d3:
        return length(bp)
    vc = d1 * d4 - d3 * d2
    if vc <= 0 and d1 >= 0 and d3 <= 0:
        return length(sub(point, add(a, scale(ab, d1 / (d1 - d3)))))
    cp = sub(point, c)
    d5, d6 = dot(ab, cp), dot(ac, cp)
    if d6 >= 0 and d5 <= d6:
        return length(cp)
    vb = d5 * d2 - d1 * d6
    if vb <= 0 and d2 >= 0 and d6 <= 0:
        return length(sub(point, add(a, scale(ac, d2 / (d2 - d6)))))
    va = d3 * d6 - d5 * d4
    if va <= 0 and d4 - d3 >= 0 and d5 - d6 >= 0:
        w = (d4 - d3) / ((d4 - d3) + (d5 - d6))
        return length(sub(point, add(b, scale(sub(c, b), w))))
    denom = 1 / (va + vb + vc)
    return length(sub(point, add(a, add(scale(ab, vb * denom), scale(ac, vc * denom)))))


def triangle_field(points, triangles):
    faces = [[points[index] for index in triangle] for triangle in triangles]
    return lambda point: min(point_triangle_distance(point, face) for face in faces) - THICKNESS * 0.5


def native_samples(client, points, seconds):
    samples = []
    for offset in range(0, len(points), 4096):
        reply = client.call({"op": "sample", "id": "body", "points": points[offset:offset + 4096],
                             "animation": animation(seconds)})
        samples.extend(reply["samples"])
    assert len(samples) == len(points)
    return samples


def even_indices(count, samples):
    assert count >= samples, (count, samples)
    return [index * count // samples for index in range(samples)]


def validate_usd(path, client, request, source, triangles, parameters, usd, sample_times=TIMES):
    Usd, UsdGeom, UsdShade, UsdValidation, Gf = (usd[name] for name in ["Usd", "UsdGeom", "UsdShade", "UsdValidation", "Gf"])
    stage = Usd.Stage.Open(str(path))
    assert stage and stage.GetDefaultPrim().GetPath() == "/World"
    assert UsdGeom.GetStageMetersPerUnit(stage) == 1 and UsdGeom.GetStageUpAxis(stage) == "Y"
    assert stage.GetFramesPerSecond() == 2 and stage.GetTimeCodesPerSecond() == 2
    assert stage.GetStartTimeCode() == sample_times[0] * 2 and stage.GetEndTimeCode() == sample_times[-1] * 2
    assert stage.GetRootLayer().customLayerData["representation"] == "evaluatedPolygonMeshCache"
    assert stage.GetRootLayer().customLayerData["colorEncoding"] == "scene-linear Rec.709/D65 RGB"
    assert not any(prim.GetTypeName() in {"Skeleton", "SkelRoot", "SkelAnimation", "BlendShape"} for prim in stage.Traverse())
    registry = UsdValidation.ValidationRegistry()
    metadata = [m for m in registry.GetAllValidatorMetadata()
                if set(m.GetKeywords()) & {"UsdCoreValidators", "UsdGeomValidators", "UsdShadeValidators"}]
    validators = [registry.GetOrLoadValidatorByName(m.name) for m in metadata]
    assert validators
    errors = UsdValidation.ValidationContext(validators).Validate(stage)
    assert not errors, [str(error) for error in errors]
    meshes = [UsdGeom.Mesh(prim) for prim in stage.Traverse() if prim.IsA(UsdGeom.Mesh)]
    assert len(meshes) == 1
    mesh = meshes[0]
    assert mesh.GetPointsAttr().GetTimeSamples() == [seconds * 2 for seconds in sample_times]
    assert mesh.GetSubdivisionSchemeAttr().Get() == "none"
    assert list(mesh.GetPrim().GetCustomDataByKey("originalEntityIds")) == ["body"]
    materials = [UsdShade.Material(prim) for prim in stage.Traverse() if prim.IsA(UsdShade.Material)]
    assert len(materials) == 1
    material = materials[0]
    assert material.GetPrim().GetCustomDataByKey("originalMaterialId") == "body"
    shader, _, _ = material.ComputeSurfaceSource()
    assert shader.GetIdAttr().Get() == "UsdPreviewSurface"
    close(shader.GetInput("diffuseColor").Get(), MATERIAL["albedo"])
    close(shader.GetInput("roughness").Get(), MATERIAL["roughness"])
    close(shader.GetInput("metallic").Get(), MATERIAL["metallic"])
    ior = shader.GetInput("ior").Get()
    close(((ior - 1) / (ior + 1)) ** 2, 0.08 * MATERIAL["specular"])
    subsets = UsdShade.MaterialBindingAPI(mesh).GetMaterialBindSubsets()
    valid, reason = UsdGeom.Subset.ValidateFamily(mesh, "face", "materialBind")
    assert valid, reason
    cameras = [UsdGeom.Camera(prim) for prim in stage.Traverse() if prim.IsA(UsdGeom.Camera)]
    assert len(cameras) == 1
    camera = cameras[0]
    frames = []
    for seconds in sample_times:
        timecode = seconds * 2
        points = [list(point) for point in mesh.GetPointsAttr().Get(timecode)]
        counts = list(mesh.GetFaceVertexCountsAttr().Get(timecode))
        indices = list(mesh.GetFaceVertexIndicesAttr().Get(timecode))
        assert all(count == 3 for count in counts)
        valid, reason = UsdGeom.Mesh.ValidateTopology(indices, counts, len(points))
        assert valid, reason
        assert all(math.isfinite(value) for point in points for value in point)
        faces = [indices[offset:offset + 3] for offset in range(0, len(indices), 3)]
        components = connected_components(len(points), faces)
        assert components == 2, "closed source shell must retain distinct inner and outer components"
        extent = [[min(point[axis] for point in points) for axis in range(3)],
                  [max(point[axis] for point in points) for axis in range(3)]]
        close(mesh.GetExtentAttr().Get(timecode), extent)
        assert all(request["bounds_min"][axis] < extent[0][axis] < extent[1][axis] < request["bounds_max"][axis]
                   for axis in range(3)), extent
        covered = []
        for subset in subsets:
            bound, relation = UsdShade.MaterialBindingAPI(subset).ComputeBoundMaterial()
            assert bound.GetPath() == material.GetPath() and relation
            covered.extend(subset.GetIndicesAttr().Get(timecode))
        assert sorted(covered) == list(range(len(faces)))
        normals = mesh.GetNormalsAttr().Get(timecode)
        assert len(normals) == len(faces)
        for index in even_indices(len(faces), 128):
            a, b, c = [points[i] for i in faces[index]]
            expected_normal = cross(sub(b, a), sub(c, a))
            close(normals[index], scale(expected_normal, 1 / length(expected_normal)))
        # Every delivered position is queried through the real native deformed field.
        samples = native_samples(client, points, seconds)
        native_residual = max(abs(sample["value"]) for sample in samples)
        assert native_residual <= request["max_field_residual"] + 1e-5, native_residual
        expected_positions = analytic_positions(source, parameters, seconds, "dual_quaternion")
        field = triangle_field(expected_positions, triangles)
        chosen = even_indices(len(points), 256)
        check_points = [points[index] for index in chosen]
        check_samples = [samples[index] for index in chosen]
        centers = [scale(add(add(points[face[0]], points[face[1]]), points[face[2]]), 1 / 3)
                   for face in [faces[index] for index in even_indices(len(faces), 128)]]
        check_points += centers
        check_samples += native_samples(client, centers, seconds)
        independent_errors, independent_residuals = [], []
        for point, sample in zip(check_points, check_samples):
            independent = field(point)
            error = abs(independent - sample["value"])
            assert error < 2e-6, (seconds, point, independent, sample)
            assert sample["material_owner_id"] == "body"
            independent_errors.append(error)
            independent_residuals.append(abs(independent))
        assert max(independent_residuals) <= request["max_field_residual"] + 1e-5
        matrix = camera.ComputeLocalToWorldTransform(timecode)
        close(matrix.ExtractTranslation(), CAMERA["eye"])
        forward = sub(CAMERA["target"], CAMERA["eye"])
        forward = scale(forward, 1 / length(forward))
        right = cross(forward, CAMERA["up"])
        right = scale(right, 1 / length(right))
        close(matrix.TransformDir(Gf.Vec3d(0, 0, -1)).GetNormalized(), forward)
        close(matrix.TransformDir(Gf.Vec3d(0, 1, 0)).GetNormalized(), cross(right, forward))
        close(matrix.GetDeterminant(), 1.0)
        gf_camera = camera.GetCamera(timecode)
        close(gf_camera.GetFieldOfView(Gf.Camera.FOVVertical), CAMERA["fov_degrees"])
        close(gf_camera.aspectRatio, 180 / 260)
        assert gf_camera.clippingRange.GetMin() > 0
        frames.append({"seconds": seconds, "vertices": len(points), "triangles": len(faces), "extent": extent,
            "independent_connected_components": components,
            "native_field_checked_vertices": len(points), "native_field_max_abs_residual": native_residual,
            "independent_vertex_checks": 256, "independent_centroid_checks": 128,
            "independent_oracle_max_error": max(independent_errors),
            "independent_oracle_max_abs_residual": max(independent_residuals),
            "points_sha256": hashlib.sha256(json.dumps(points).encode()).hexdigest()})
    assert len({frame["points_sha256"] for frame in frames}) == len(sample_times)
    return {"path": str(path), "sha256": sha256(path), "schema_validators": [m.name for m in metadata],
            "frames": frames, "representation": "evaluated polygon mesh cache; no UsdSkel"}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--editor", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--reader-path", type=Path, default=Path("/tmp/mm3e-usd-reader"))
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    sys.path.insert(0, str(args.reader_path))
    from pxr import Usd, UsdGeom, UsdShade, UsdValidation, Gf
    usd = {name: value for name, value in locals().items() if name in ["Usd", "UsdGeom", "UsdShade", "UsdValidation", "Gf"]}
    executable = args.editor.resolve(strict=True)
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    started = time.monotonic()
    report = {"status": "running", "passed": False, "binary": str(executable), "binary_sha256": sha256(executable),
        "harness_sha256": sha256(Path(__file__)), "reader": "OpenUSD " + ".".join(map(str, Usd.GetVersion())),
        "helper_sha256": {name: sha256(ROOT / "scripts" / name) for name in
                          ["agent_usd_acceptance.py", "agent_sewing_acceptance.py", "agent_animation_acceptance.py"]},
        "scope": "persistent native deformation, rendered pixels, and baked evaluated USD mesh cache",
        "limits": ["no anatomical or film-production quality certification", "no editable UsdSkel export",
                   "no consumer renderer image parity", "sampled convergence is not a certified Hausdorff bound"]}
    report_path = root / "acceptance.json"
    write_json(report_path, report)
    client = None
    try:
        operations, vertices, triangles, parameters = fixture()
        with (root / "transcript.jsonl").open("w") as transcript:
            client = Client(executable, root, transcript, "author")
            capabilities = client.call({"op": "describe"})
            write_json(root / "capabilities.json", capabilities)
            schema = json.dumps(capabilities["request_schema"])
            for name in ["bind_surface", "update_deformer", "deformer_state", "set_morph_weights", "morph_tracks"]:
                assert name in schema, name
            assert "weighted skinning and continuous tissue deformation" not in capabilities["not_implemented"]
            client.apply(operations)
            authored = client.call({"op": "get_document"})
            write_json(root / "source-document.json", authored)
            assert len(authored["deformers"]) == 1 and len(authored["joints"]) == 2
            assert all(not joint["objects"] for joint in authored["joints"])
            client.call({"op": "validate"})
            states, renders, errors = {}, {}, {}
            project_hash = sha256(root / "project.json")
            revision = client.revision
            for seconds in [0.0, 1.0, 0.5, 0.0, 1.0]:
                state = client.call({"op": "deformer_state", "id": "skin", "animation": animation(seconds)})
                assert len(state["vertices"]) == len(vertices)
                expected = analytic_positions(vertices, parameters, seconds, "dual_quaternion")
                error = max(length(sub(a, b)) for a, b in zip(state["vertices"], expected))
                assert error < 2e-6, (seconds, error)
                assert state["triangles"] == triangles and state["method"] == "dual_quaternion"
                if str(seconds) in states:
                    assert state == states[str(seconds)], "pose depends on sample order"
                states[str(seconds)] = state
                errors[str(seconds)] = error
            write_json(root / "deformer-states.json", states)
            assert client.revision == revision and sha256(root / "project.json") == project_hash
            for name, seconds in [("rest", 0), ("dqs-middle", 0.5), ("dqs-posed", 1), ("rest-return", 0)]:
                renders[name] = client.call({"op": "render", "path": name + ".png", "animation": animation(seconds)})
                assert renders[name]["metrics"]["visible_material_owner_pixels"].get("body", 0) > 500
            assert (root / "rest.png").read_bytes() == (root / "rest-return.png").read_bytes()
            client.apply([{"op": "update_deformer", "request": {"id": "skin", "method": "linear_blend"}}])
            linear = client.call({"op": "deformer_state", "id": "skin", "animation": animation(1)})
            expected = analytic_positions(vertices, parameters, 1, "linear_blend")
            lbs_error = max(length(sub(a, b)) for a, b in zip(linear["vertices"], expected))
            assert lbs_error < 2e-6
            write_json(root / "linear-state.json", linear)
            radial = lambda point: math.hypot(point[0] - POSITION[0], point[2] - POSITION[2])
            dqs_radii = [radial(point) for point in states["1.0"]["vertices"][32:48]]
            lbs_radii = [radial(point) for point in linear["vertices"][32:48]]
            close(dqs_radii, [0.1] * 16, 2e-6)
            close(lbs_radii, [0.1 * math.cos(math.radians(70))] * 16, 2e-6)
            renders["lbs-posed"] = client.call({"op": "render", "path": "lbs-posed.png", "animation": animation(1)})
            dqs_pixels, _ = png_pixels(root / "dqs-posed.png")
            lbs_pixels, _ = png_pixels(root / "lbs-posed.png")
            changed = pixel_difference(dqs_pixels, lbs_pixels)
            assert changed > 200, changed
            client.call({"op": "undo"}, mutation=True)
            assert client.call({"op": "get_document"}) == authored
            assert client.call({"op": "deformer_state", "id": "skin", "animation": animation(1)}) == states["1.0"]
            renders["dqs-undo"] = client.call({"op": "render", "path": "dqs-undo.png", "animation": animation(1)})
            assert (root / "dqs-undo.png").read_bytes() == (root / "dqs-posed.png").read_bytes()
            # Invalid weights must reject atomically, preserving both durable bytes and state.
            before = sha256(root / "project.json")
            revision = client.revision
            broken = copy.deepcopy(authored["deformers"][0]["weights"])
            broken[0][0]["weight"] = 0.4
            rejected = client.call({"op": "apply", "operations": [
                {"op": "update_deformer", "request": {"id": "skin", "weights": broken}}]}, mutation=True, ok=False)
            assert client.revision == revision and sha256(root / "project.json") == before
            report["invalid_weights_rejection"] = rejected["error"]
            client.close()
            client = None
            client = Client(executable, root, transcript, "reopen")
            assert client.call({"op": "get_document"}) == authored
            assert sha256(root / "project.json") == before
            assert client.call({"op": "deformer_state", "id": "skin", "animation": animation(1)}) == states["1.0"]
            renders["dqs-reopened"] = client.call({"op": "render", "path": "dqs-reopened.png", "animation": animation(1)})
            assert (root / "dqs-reopened.png").read_bytes() == (root / "dqs-posed.png").read_bytes()
            report.update({"native_pose_max_errors": errors, "linear_pose_max_error": lbs_error,
                "dqs_middle_radius_min_m": min(dqs_radii), "lbs_middle_radius_max_m": max(lbs_radii),
                "changed_pixels_dqs_vs_lbs": changed, "renders": renders,
                "source_unchanged": True, "return_pose_exact": True, "undo_pixels_exact": True,
                "fresh_process_project_bytes_and_pose_and_pixels_exact": True})
            write_json(report_path, report)
            # An unrelated unbaked panel must not poison selected body queries/delivery.
            client.apply([{"op": "create_cloth_panel", "request": {"id": "unbaked", "origin": [4, 1, 0],
                "axis_u": [1, 0, 0], "axis_v": [0, 1, 0], "segments": [1, 1], "width_m": 0.2,
                "height_m": 0.2, "thickness_m": 0.01, "vertex_mass_kg": 0.02,
                "settings": {"collision_thickness": 0.005}}}])
            selected = native_samples(client, [states["1.0"]["vertices"][32]], 1)
            close(selected[0]["value"], -THICKNESS * 0.5, 2e-6)
            failed_render = client.call({"op": "render", "path": "must-not-exist.png", "animation": animation(1)}, ok=False)
            assert not (root / "must-not-exist.png").exists()
            report["unbaked_unrelated_cloth"] = {"selected_body_sample_succeeded": True,
                "full_render_error": failed_render["error"], "failed_render_created_no_file": True}
            request = {"path": "deformed-body.usda", "object_ids": ["body"],
                "bounds_min": [0.029, 0.872, -0.127], "bounds_max": [0.279, 1.522, 0.123],
                "resolution": [40, 48, 40], "max_surface_error_m": 0.03, "max_field_residual": 0.012,
                "camera_near_clip_m": 0.0001, "clip": "twist", "start_seconds": 0,
                "end_seconds": 1, "frames_per_second": 2}
            write_json(root / "export-request.json", request)
            export_hash = sha256(root / "project.json")
            report["usd_export"] = client.call({"op": "export_usd", "request": request})
            assert report["usd_export"]["delivery"]["charged_work_total"] <= 200_000_000
            report["usd_reader"] = validate_usd(root / request["path"], client, request, vertices, triangles, parameters, usd)
            assert sha256(root / "project.json") == export_hash
            assert client.call({"op": "get_document"})["objects"][0] == authored["objects"][0]
            report["unbaked_unrelated_cloth"]["selected_body_usd_succeeded"] = True
            client.close()
            client = None
        assert sha256(executable) == report["binary_sha256"], "editor binary changed during acceptance"
        report["status"], report["passed"] = "passed", True
        (root / "comparison.html").write_text("""<!doctype html><meta charset="utf-8"><title>Native deformation acceptance</title>
<style>body{background:#13202b;color:#eef3f7;font:16px system-ui;margin:2rem}.frames{display:flex;gap:1rem}figure{margin:0}
img{width:270px;height:390px}a{color:#9cf}</style><h1>Native skinning and blend shapes</h1>
<p>Actual fixed-camera engine frames. A 140-degree relative joint twist and animated radial blend shape.</p>
<div class="frames"><figure><img src="rest.png"><figcaption>Rest</figcaption></figure>
<figure><img src="dqs-posed.png"><figcaption>Dual quaternion: middle radius 0.1 m</figcaption></figure>
<figure><img src="lbs-posed.png"><figcaption>Linear blend: middle radius 0.0342 m</figcaption></figure></div>
<p>Mechanism acceptance; no anatomical quality claim. USD contains a baked evaluated polygon cache, not UsdSkel.</p>
<p><a href="acceptance.json">Evidence</a> · <a href="deformed-body.usda">Three-pose USD</a></p>""")
    except Exception as error:
        report.update({"status": "failed", "error": str(error), "traceback": traceback.format_exc()})
        raise
    finally:
        if client is not None:
            client.abort()
        report["elapsed_seconds"] = time.monotonic() - started
        report["artifacts_sha256"] = {str(path.relative_to(root)): sha256(path) for path in root.rglob("*")
            if path.is_file() and path != report_path}
        write_json(report_path, report)
    print(json.dumps({key: report[key] for key in ["status", "elapsed_seconds", "dqs_middle_radius_min_m",
        "lbs_middle_radius_max_m", "changed_pixels_dqs_vs_lbs"]}, indent=2))


if __name__ == "__main__":
    main()
