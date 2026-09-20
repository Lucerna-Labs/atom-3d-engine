#!/usr/bin/env python3
"""Actual textured USD bundles, read independently with OpenUSD and OpenEXR.

Checks composed thick geometry, chart attachment, exact original images, opaque
linear color conversion, connected shader values, relocation and cold export.
The explicit reader-defined filtering contract does not imply Hydra/image/BRDF
parity: usd-core supplies an independent schema reader, not a rendering delegate.
"""
import argparse
import base64
import collections
import copy
import hashlib
import json
import math
from pathlib import Path, PurePosixPath
import shutil
import subprocess
import sys
import time
import traceback

from agent_animation_acceptance import Client, write_json
from agent_sewing_acceptance import png_pixels

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "artifacts/material-maps-normal-filter-20260907-opt-in"
WORK_CAP = 200_000_000


def validate_work(delivery):
    """Check exclusive top-level ledgers, without double-counting frame details."""
    assert isinstance(delivery, dict), "delivery work report must be an object"
    required = ("estimated_field_work", "boolean_arrangement_work", "charged_work_total")
    assert all(name in delivery for name in required), "delivery work report is missing a required ledger or total"
    names = ["estimated_field_work", "boolean_arrangement_work"]
    if "source_feature_work" in delivery:
        names.append("source_feature_work")
    for name in names + ["charged_work_total"]:
        assert type(delivery[name]) is int and delivery[name] >= 0, f"delivery {name} must be a nonnegative integer"
    total = delivery["charged_work_total"]
    assert total <= WORK_CAP, f"delivery charged work {total} exceeds unchanged cap {WORK_CAP}"
    ledgers = {name: delivery[name] for name in names}
    assert sum(ledgers.values()) == total, f"delivery exclusive ledgers {ledgers} do not sum to charged total {total}"
    return {"status": "passed", "cap": WORK_CAP, "exclusive_ledgers": ledgers, "charged_work_total": total,
            "source_feature_ledger_present": "source_feature_work" in delivery}


def compare_requests(request, frozen):
    """Exact JSON values and fields; this harness needs no path exception."""
    assert isinstance(request, dict) and isinstance(frozen, dict), "challenge requests must be objects"
    fields = sorted(set(request) | set(frozen))
    # Canonical JSON also distinguishes booleans from numbers and retains numeric
    # values exactly; no tolerance, default insertion, or field removal is used.
    encode = lambda value: json.dumps(value, sort_keys=True, separators=(",", ":"), allow_nan=False)
    changed = [name for name in fields if name not in request or name not in frozen
               or encode(request[name]) != encode(frozen[name])]
    assert not changed, f"generated challenge request changed frozen fields: {changed}"
    return {"status": "passed", "compared_fields": fields, "comparison": "exact JSON values including output path",
            "permitted_output_destination_differences": []}


def validate_challenge(root, challenge_root, kind, request):
    original = challenge_root / kind
    generated_path, frozen_path = root / "request.json", original / "request.json"
    frozen = json.loads(frozen_path.read_text())
    report = {"status": "running", "case": kind, "frozen_root": str(original.resolve()),
              "generated_request_sha256": sha(generated_path), "frozen_request_sha256": sha(frozen_path),
              "generated_input_sha256": sha(root / "input.json"), "frozen_input_sha256": sha(original / "input.json"),
              "compared_fields": sorted(set(request) | set(frozen)), "permitted_output_destination_differences": []}
    try:
        assert report["generated_input_sha256"] == report["frozen_input_sha256"], "challenge source input differs from frozen original"
        report.update(compare_requests(request, frozen))
        report["request_bytes_exact"] = report["generated_request_sha256"] == report["frozen_request_sha256"]
        return report
    except Exception as error:
        report.update(status="failed", error=str(error))
        raise
    finally:
        write_json(root / "challenge-identity.json", report)


def contract_self_test():
    """Independent negative controls for the acceptance contract, not engine output."""
    positive = {"estimated_field_work": WORK_CAP-5, "boolean_arrangement_work": 2,
                "source_feature_work": 3, "charged_work_total": WORK_CAP}
    passed = {"at_cap": validate_work(positive),
              "legacy_two_ledgers": validate_work({"estimated_field_work": 4, "boolean_arrangement_work": 0,
                                                   "charged_work_total": 4})}
    negatives = {}
    def rejected(name, action, expected):
        try:
            action()
        except AssertionError as error:
            assert expected in str(error), (name, str(error))
            negatives[name] = {"rejected": True, "reason": str(error)}
        else:
            raise AssertionError(f"acceptance negative control {name} was accepted")
    for name, changes, expected in [
        ("over_budget", {"estimated_field_work": WORK_CAP-4, "charged_work_total": WORK_CAP+1}, "exceeds unchanged cap"),
        ("omitted_source_cost", {"charged_work_total": WORK_CAP-3}, "do not sum"),
        ("double_counted_total", {"charged_work_total": WORK_CAP-1}, "do not sum"),
        ("negative_ledger", {"source_feature_work": -3}, "nonnegative integer"),
        ("fractional_total", {"charged_work_total": float(WORK_CAP)}, "nonnegative integer"),
        ("boolean_ledger", {"boolean_arrangement_work": True}, "nonnegative integer"),
    ]:
        rejected(name, lambda changes=changes: validate_work(positive | changes), expected)
    rejected("missing_ledger", lambda: validate_work({"charged_work_total": 0}), "missing a required")
    request = {"path": "bundle/animated.usda", "object_ids": ["face/lips"], "bounds_min": [-1., 0., -1.],
               "bounds_max": [1., 2., 1.], "resolution": [128, 128, 64], "max_surface_error_m": .015,
               "max_field_residual": .003, "clip": "expression", "start_seconds": 0, "end_seconds": .8,
               "frames_per_second": 2.5, "texture_delivery": {"filtering": "reader_defined",
                   "max_uv_error_texels": .25, "max_refinement_passes": 8}}
    passed["identical_request"] = compare_requests(copy.deepcopy(request), request)
    changes = {"path": "other/animated.usda", "object_ids": ["face"], "bounds_min": [-1.0000000000000002, 0., -1.],
               "bounds_max": [1., 2., 1.0000000000000002], "resolution": [128, 128, 32], "max_surface_error_m": .016,
               "max_field_residual": .004, "clip": "other", "start_seconds": .1, "end_seconds": .9,
               "frames_per_second": 3., "texture_delivery": request["texture_delivery"] | {"max_uv_error_texels": .5}}
    for field, value in changes.items():
        rejected("changed_"+field, lambda field=field, value=value: compare_requests(request | {field: value}, request),
                 "changed frozen fields")
    rejected("extra_geometry_field", lambda: compare_requests(request | {"camera_near_clip_m": .2}, request),
             "changed frozen fields")
    rejected("missing_geometry_field", lambda: compare_requests({k: v for k, v in request.items() if k != "resolution"}, request),
             "changed frozen fields")
    return {"status": "passed", "scope": "acceptance contract calibration only; no engine export or intersection certification",
            "positive_controls": passed, "negative_controls": negatives}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def sub(a, b):
    return [x-y for x, y in zip(a, b)]


def dot(a, b):
    return sum(x*y for x, y in zip(a, b))


def closest(point, triangle):
    """Independent double-precision plane/edge closest-point coordinates."""
    a, b, c = triangle
    ab, ac, ap = sub(b, a), sub(c, a), sub(point, a)
    aa, bb, cc = dot(ab, ab), dot(ab, ac), dot(ac, ac)
    determinant = aa*cc-bb*bb
    assert determinant > 0
    d, e = dot(ap, ab), dot(ap, ac)
    v, w = (cc*d-bb*e)/determinant, (aa*e-bb*d)/determinant
    candidates = []
    if v >= 0 and w >= 0 and v+w <= 1:
        candidates.append([1-v-w, v, w])
    for first, second in ((0, 1), (1, 2), (2, 0)):
        edge = sub(triangle[second], triangle[first])
        amount = max(0, min(1, dot(sub(point, triangle[first]), edge)/dot(edge, edge)))
        weights = [0, 0, 0]
        weights[first], weights[second] = 1-amount, amount
        candidates.append(weights)
    values = []
    for weights in candidates:
        q = [sum(weights[i]*triangle[i][axis] for i in range(3)) for axis in range(3)]
        values.append((dot(sub(point, q), sub(point, q)), weights))
    return min(values, key=lambda x: x[0])


def nearest_uv(point, source, uv):
    candidates = []
    for triangle, corners in zip(source["triangles"], uv["corner_indices"]):
        distance, weights = closest(point, [source["vertices"][i] for i in triangle])
        coordinate = [sum(weights[i]*uv["values"][corners[i]][axis] for i in range(3)) for axis in range(2)]
        candidates.append((distance, coordinate))
    nearest = min(d for d, _ in candidates)
    return [st for d, st in candidates if d <= nearest + 1e-13]


def shell_witnesses(points, triangles, source, np):
    """Normal rays must hit both delivered shell sides; midsurface control must fail."""
    def prepared(vertices, faces):
        a = np.asarray(vertices, dtype=np.float64)[np.asarray(faces, dtype=np.int64)]
        return a[:, 0], a[:, 1]-a[:, 0], a[:, 2]-a[:, 0]
    delivered, midsurface = prepared(points, triangles), prepared(source["vertices"], source["triangles"])
    half = source["thickness_m"]/2
    def intersections(geometry, center, normal):
        a, e1, e2 = geometry
        origin = np.asarray(center)-3*half*normal
        h = np.cross(normal, e2)
        determinant = np.sum(e1*h, axis=1)
        valid = np.abs(determinant) > 1e-15
        inverse = np.zeros_like(determinant)
        inverse[valid] = 1/determinant[valid]
        tvec = origin-a
        u = np.sum(tvec*h, axis=1)*inverse
        q = np.cross(tvec, e1)
        v = np.sum(normal*q, axis=1)*inverse
        t = np.sum(e2*q, axis=1)*inverse
        valid &= (u >= -1e-9) & (v >= -1e-9) & (u+v <= 1+1e-9) & (t >= 0) & (t <= 6*half)
        return t[valid]
    checked = []
    for triangle in source["triangles"][::max(1, len(source["triangles"])//30)]:
        original = [source["vertices"][i] for i in triangle]
        normal = np.cross(sub(original[1], original[0]), sub(original[2], original[0]))
        normal /= np.linalg.norm(normal)
        center = np.mean(np.asarray(original), axis=0)
        isolated = True
        for sign in (-1, 1):
            point = center+sign*half*normal
            distance = min(closest(point, [source["vertices"][i] for i in face])[0] for face in source["triangles"])
            isolated &= abs(math.sqrt(distance)-half) <= half*.02
        if not isolated:
            continue
        hits = intersections(delivered, center, normal)
        control = intersections(midsurface, center, normal)
        expected = (2*half, 4*half)
        error = [min((abs(float(t)-want) for t in hits), default=math.inf) for want in expected]
        control_error = [min((abs(float(t)-want) for t in control), default=math.inf) for want in expected]
        assert all(e <= half*.4 for e in error), {"center": center.tolist(), "shell_hits": hits.tolist(), "expected": expected}
        assert any(e > half*.4 for e in control_error), "midsurface negative control incorrectly satisfies shell thickness"
        checked.append({"source_center": center.tolist(), "normal": normal.tolist(), "half_thickness_m": half,
                        "front_back_error_m": error, "midsurface_rejected": True})
        if len(checked) == 4:
            break
    assert len(checked) >= 2, "no independent two-sided thickness witnesses"
    return checked


def fold(value, wrap):
    if wrap == "clamp":
        return max(0, min(1, value))
    if wrap == "repeat":
        return value % 1
    assert wrap == "mirror", wrap
    return 1-abs(value % 2-1)


def common_periods(binding):
    """Translations must preserve every bound map, including sampler overrides."""
    samplers = []
    if binding.get("texture"):
        samplers.append(binding["sampler"])
    for role in ("roughness", "metallic", "emissive", "normal"):
        if binding.get(role):
            samplers.append(binding[role].get("sampler") or binding["sampler"])
    assert samplers
    periods = []
    for axis in ("u", "v"):
        modes = {s[axis] for s in samplers}
        assert modes <= {"repeat", "mirror", "clamp"}
        periods.append(None if "clamp" in modes else 2.0 if "mirror" in modes else 1.0)
    return periods


def uv_errors(actual, reference, periods, size):
    raw = [abs(a-b) for a, b in zip(actual, reference)]
    preserved = [d if period is None else abs((d+period/2) % period-period/2)
                 for d, period in zip(raw, periods)]
    return (math.hypot(*(d*n for d, n in zip(raw, size))),
            math.hypot(*(d*n for d, n in zip(preserved, size))))


def checked_uv(actual, candidates, periods, size, tolerance):
    reference = min(candidates, key=lambda uv: uv_errors(actual, uv, periods, size)[1])
    raw, preserved = uv_errors(actual, reference, periods, size)
    assert preserved <= tolerance+.003, (f"USD UV attachment error {preserved} texels; "
                                         f"raw {raw}; common preserving periods {periods}")
    return reference, raw, preserved


def index(value, size, wrap):
    if wrap == "clamp":
        return max(0, min(size-1, value))
    if wrap == "repeat":
        return value % size
    value %= 2*size
    return value if value < size else 2*size-1-value


def bilinear(pixels, uv, wraps):
    height, width = pixels.shape[:2]
    x, y = fold(uv[0], wraps[0])*width-.5, (1-fold(uv[1], wraps[1]))*height-.5
    ix, iy, fx, fy = math.floor(x), math.floor(y), x % 1, y % 1
    def p(a, b):
        return pixels[index(b, height, wraps[1]), index(a, width, wraps[0])]
    return (p(ix, iy)*(1-fx)+p(ix+1, iy)*fx)*(1-fy)+(p(ix, iy+1)*(1-fx)+p(ix+1, iy+1)*fx)*fy


def read_modules(reader_path, exr_path):
    sys.path[:0] = [str(reader_path), str(exr_path)]
    from pxr import Usd, UsdGeom, UsdShade, UsdValidation, Sdf, Ar
    import OpenEXR
    import numpy as np
    return Usd, UsdGeom, UsdShade, UsdValidation, Sdf, Ar, OpenEXR, np


def inspect_bundle(path, expected, reader_path, exr_path):
    Usd, UsdGeom, UsdShade, UsdValidation, Sdf, Ar, OpenEXR, np = read_modules(reader_path, exr_path)
    stage = Usd.Stage.Open(str(path))
    assert stage
    registry = UsdValidation.ValidationRegistry()
    metadata = [m for m in registry.GetAllValidatorMetadata() if set(m.GetKeywords()) &
                {"UsdCoreValidators", "UsdGeomValidators", "UsdShadeValidators"}]
    assert metadata
    errors = UsdValidation.ValidationContext([registry.GetOrLoadValidatorByName(m.name) for m in metadata]).Validate(stage)
    assert not errors, [str(e) for e in errors]
    asset_directory = path.parent / expected["asset_directory"]
    manifest = json.loads((asset_directory / "manifest.json").read_text())
    assert manifest["format"] == "mm3e-usd-assets-v1" and manifest["layer"] == path.name
    assert manifest["layer_sha256"] == sha(path)
    records = {r["file"]: r for r in manifest["assets"]}
    assert len(records) == len(manifest["assets"])
    assert set(records) == {p.name for p in asset_directory.iterdir() if p.name != "manifest.json"}
    for name, record in records.items():
        assert name == PurePosixPath(name).name and (asset_directory/name).is_file()
        assert (asset_directory/name).stat().st_size == record["bytes"] and sha(asset_directory/name) == record["sha256"]
    document, identifier = expected["document"], expected["object"]
    binding = next(b for b in document["texture_bindings"] if b["object"] == identifier)
    periods = common_periods(binding)
    obj = next(o for o in document["objects"] if o["id"] == identifier)
    material = obj["material"]
    uv = next(u for u in document["uv_sets"] if u["id"] == binding["uv_set"])
    source_pixels, original_hashes = {}, {}
    source_ids = {binding[k]["texture"] for k in ("roughness", "metallic", "emissive", "normal") if binding.get(k)}
    if binding.get("texture"):
        source_ids.add(binding["texture"])
    for texture in document["textures"]:
        if texture["id"] not in source_ids:
            continue
        original = base64.b64decode(texture["data"], validate=True)
        digest = hashlib.sha256(original).hexdigest()
        original_hashes[texture["id"]] = digest
        retained = asset_directory / ("source-"+digest+".png")
        assert retained.read_bytes() == original
        raw, info = png_pixels(retained)
        pixels = np.frombuffer(raw, dtype=np.uint8).reshape(info["height"], info["width"], 4).astype(np.float64)/255
        if texture["color_space"] == "srgb":
            pixels[..., :3] = np.where(pixels[..., :3] <= .04045, pixels[..., :3]/12.92,
                                       ((pixels[..., :3]+.055)/1.055)**2.4)
        source_pixels[texture["id"]] = pixels
    materials = [UsdShade.Material(p) for p in stage.Traverse() if p.IsA(UsdShade.Material)]
    assert len(materials) == 1
    shader, _, _ = materials[0].ComputeSurfaceSource()
    assert shader.GetIdAttr().Get() == "UsdPreviewSurface"
    assert shader.GetInput("opacity").Get() == 1 and not shader.GetInput("opacity").HasConnectedSource()
    nodes, converted_error, resolved_assets, texture_metadata = {}, 0, [], {}
    roles = {"diffuseColor": (binding.get("texture"), material["albedo"]),
             "emissiveColor": ((binding.get("emissive") or {}).get("texture"), material["emissive"]),
             "roughness": ((binding.get("roughness") or {}).get("texture"), material["roughness"]),
             "metallic": ((binding.get("metallic") or {}).get("texture"), material["metallic"])}
    for name, (source_id, factor) in roles.items():
        input_value = shader.GetInput(name)
        if not input_value.HasConnectedSource():
            if source_id and name == "roughness":
                component = "rgba".index(binding["roughness"]["channel"])
                assert float(np.max(source_pixels[source_id][..., component])*factor) <= .04+1e-7
                assert abs(input_value.Get()-.04) < 1e-7
            else:
                assert not source_id, (name, source_id)
            continue
        node_api, output_name, _ = input_value.GetConnectedSource()
        node = UsdShade.Shader(node_api.GetPrim())
        assert node.GetIdAttr().Get() == "UsdUVTexture" and source_id
        st_api, st_output, _ = node.GetInput("st").GetConnectedSource()
        st_reader = UsdShade.Shader(st_api.GetPrim())
        assert st_output == "result" and st_reader.GetIdAttr().Get() == "UsdPrimvarReader_float2"
        assert st_reader.GetInput("varname").Get() == "st"
        assert node.GetInput("sourceColorSpace").Get() == "raw"
        asset = node.GetInput("file").Get()
        relative = PurePosixPath(asset.path)
        assert not relative.is_absolute() and ".." not in relative.parts and "\\" not in asset.path
        assert relative.parts[0] == expected["asset_directory"]
        resolved = Path(str(Ar.GetResolver().Resolve(Sdf.ComputeAssetPathRelativeToLayer(stage.GetRootLayer(), asset.path)))).resolve(strict=True)
        assert resolved.is_relative_to(path.parent.resolve()) and resolved.parent == asset_directory.resolve()
        resolved_assets.append(asset.path)
        scales, bias = np.array(node.GetInput("scale").Get()), np.array(node.GetInput("bias").Get())
        assert np.max(np.abs(bias)) == 0
        map_name = {"emissiveColor": "emissive"}.get(name, name)
        sampler = (binding.get(map_name) or {}).get("sampler") or binding["sampler"]
        wraps = (str(node.GetInput("wrapS").Get()), str(node.GetInput("wrapT").Get()))
        assert wraps == (sampler["u"], sampler["v"])
        source = source_pixels[source_id]
        if name in ("diffuseColor", "emissiveColor"):
            assert output_name == "rgb" and resolved.suffix == ".exr"
            with OpenEXR.File(str(resolved)) as image:
                pixels = image.channels()["RGB"].pixels.copy()
                header = dict(image.header())
            assert header["transferFunction"] == "linear"
            assert header["colorSpace"] == "Linear Rec.709 (sRGB primaries), D65"
            assert header["mm3e:alphaSemantics"] == "RGB material modulation; source alpha incorporated into values; no opacity channel"
            assert "material modulation texture" in header["comments"] and "background" not in header["comments"]
            texture_metadata[resolved.name] = {key: header[key] for key in
                ("comments", "colorSpace", "transferFunction", "mm3e:alphaSemantics")}
            assert pixels.dtype == np.float32 and np.isfinite(pixels).all()
            original32 = source.astype(np.float32)
            expected_pixels = original32[..., :3]*original32[..., 3:4]
            if name == "diffuseColor":
                expected_pixels = np.clip(expected_pixels+np.float32(1)-original32[..., 3:4], 0, 1)
            error = float(np.max(np.abs(pixels.astype(np.float64)-expected_pixels)))
            converted_error = max(converted_error, error)
            assert error <= 4e-7, (name, error)
            assert np.max(np.abs(scales[:3]-factor)) < 1e-6
            expected_values = expected_pixels.astype(np.float64)
        else:
            assert resolved.suffix == ".png" and sha(resolved) == original_hashes[source_id]
            assert output_name == binding[name]["channel"]
            pixels, expected_values = source, source
            assert np.max(np.abs(scales-factor)) < 1e-6
        nodes[name] = {"pixels": pixels, "expected": expected_values, "wraps": wraps,
                       "scale": scales, "output": str(output_name)}
    normal_input = shader.GetInput("normal")
    assert not normal_input or not normal_input.HasConnectedSource(), "active normal transfer is outside this delivery profile"
    meshes = [UsdGeom.Mesh(p) for p in stage.Traverse() if p.IsA(UsdGeom.Mesh)]
    assert len(meshes) == 1
    mesh = meshes[0]
    assert mesh.GetPrim().GetCustomDataByKey("representation") == "composedSdfBake"
    assert mesh.GetSubdivisionSchemeAttr().Get() == "none"
    st = UsdGeom.PrimvarsAPI(mesh).GetPrimvar("st")
    assert st and st.GetInterpolation() == "faceVarying"
    assert len(UsdShade.MaterialBindingAPI(mesh).GetMaterialBindSubsets()) == 1
    source_count = len(expected["frames"][0]["vertices"])
    frames, appearance_error, attachment_error, attachment_bound, uv_error, raw_uv_error = [], 0, 0, 0, 0, 0
    for source in expected["frames"]:
        time_code = source["seconds"]*expected["request"]["frames_per_second"]
        points = [list(p) for p in mesh.GetPointsAttr().Get(time_code)]
        counts = list(mesh.GetFaceVertexCountsAttr().Get(time_code))
        indices = list(mesh.GetFaceVertexIndicesAttr().Get(time_code))
        assert all(c == 3 for c in counts) and len(indices) == 3*len(counts)
        triangles = [indices[i:i+3] for i in range(0, len(indices), 3)]
        subsets = UsdShade.MaterialBindingAPI(mesh).GetMaterialBindSubsets()
        bound, _ = UsdShade.MaterialBindingAPI(subsets[0].GetPrim()).ComputeBoundMaterial()
        assert bound.GetPath() == materials[0].GetPath()
        assert sorted(subsets[0].GetIndicesAttr().Get(time_code)) == list(range(len(triangles)))
        edges = collections.defaultdict(lambda: [0, 0])
        for triangle in triangles:
            assert len(set(triangle)) == 3
            for a, b in zip(triangle, triangle[1:]+triangle[:1]):
                count = edges[min(a, b), max(a, b)]
                count[0] += 1
                count[1] += 1 if a < b else -1
        assert all(count == 2 and orientation == 0 for count, orientation in edges.values()), "textured shell has boundary/nonmanifold/inconsistently oriented edges"
        coordinates = [list(v) for v in st.ComputeFlattened(time_code)]
        assert len(coordinates) == len(indices) and len(points) > source_count
        assert all(math.isfinite(v) for p in points+coordinates for v in p)
        seams = collections.defaultdict(list)
        for vertex, coordinate in zip(indices, coordinates):
            seams[tuple(round(v, 6) for v in points[vertex])].append(coordinate)
        seam_groups = sum(max(u[0] for u in group)-min(u[0] for u in group) > .49 for group in seams.values())
        if expected["require_seam"]:
            assert seam_groups > 0, "delivered mesh lost the explicit UV chart discontinuity"
        raw_triangle_span = [max(max(u[axis] for u in coordinates[i:i+3])-min(u[axis] for u in coordinates[i:i+3])
                                 for i in range(0, len(coordinates), 3)) for axis in range(2)]
        thickness_witnesses = shell_witnesses(points, triangles, source, np)
        seam_positions = {p for p, group in seams.items() if max(u[0] for u in group)-min(u[0] for u in group) > .49}
        seam_faces = [i for i, triangle in enumerate(triangles)
                      if any(tuple(round(v, 6) for v in points[index]) in seam_positions for index in triangle)]
        face_probes = set(range(0, len(triangles), max(1, len(triangles)//64)))
        face_probes.update(seam_faces[::max(1, len(seam_faces)//48)])
        probes, seam_sides = [], {"front": 0, "back": 0}
        probe_locations = [(face, weights) for face in sorted(face_probes)
                           for weights in ([.5, .5, 0], [1/3, 1/3, 1/3], [0, .5, .5], [.5, 0, .5])]
        for face, weights in probe_locations:
            point = [sum(points[index][a]*weight for index, weight in zip(triangles[face], weights)) for a in range(3)]
            coordinate = [sum(coordinates[face*3+i][a]*weights[i] for i in range(3)) for a in range(2)]
            source_coordinates = nearest_uv(point, source, uv)
            nearest, raw, preserved = checked_uv(coordinate, source_coordinates, periods, expected["texture_size"],
                                                 expected["request"]["texture_delivery"]["max_uv_error_texels"])
            uv_error, raw_uv_error = max(uv_error, preserved), max(raw_uv_error, raw)
            unambiguous = all(math.dist(st, nearest) < 1e-5 for st in source_coordinates)
            values = {}
            for name, node in nodes.items():
                actual = bilinear(node["pixels"], coordinate, node["wraps"])
                wanted = bilinear(node["expected"], coordinate, node["wraps"])
                if node["output"] == "rgb":
                    actual, wanted = actual[:3]*node["scale"][:3], wanted[:3]*node["scale"][:3]
                    values[name] = [float(x) for x in actual]
                else:
                    component = "rgba".index(node["output"])
                    actual, wanted = actual[component]*node["scale"][component], wanted[component]*node["scale"][component]
                    values[name] = float(actual)
                appearance_error = max(appearance_error, float(np.max(np.abs(actual-wanted))))
                if unambiguous:
                    original = bilinear(node["expected"], nearest, node["wraps"])
                    image = node["expected"]
                    if node["output"] == "rgb":
                        original = original[:3]*node["scale"][:3]
                        image = image[..., :3]*node["scale"][:3]
                    else:
                        component = "rgba".index(node["output"])
                        original = original[component]*node["scale"][component]
                        image = image[..., component]*node["scale"][component]
                    delta = [abs(coordinate[i]-nearest[i]) for i in range(2)]
                    for axis, wrap in enumerate(node["wraps"]):
                        if wrap == "repeat":
                            delta[axis] = abs((delta[axis]+.5) % 1-.5)
                    gradient_u = float(np.max(np.abs(np.diff(image, axis=1))))
                    gradient_v = float(np.max(np.abs(np.diff(image, axis=0))))
                    if node["wraps"][0] == "repeat":
                        gradient_u = max(gradient_u, float(np.max(np.abs(image[:, 0]-image[:, -1]))))
                    if node["wraps"][1] == "repeat":
                        gradient_v = max(gradient_v, float(np.max(np.abs(image[0]-image[-1]))))
                    bound = gradient_u*delta[0]*image.shape[1]+gradient_v*delta[1]*image.shape[0]+2e-6
                    attached = float(np.max(np.abs(actual-original)))
                    assert attached <= bound, (attached, bound, coordinate, nearest)
                    attachment_error = max(attachment_error, attached)
                    attachment_bound = max(attachment_bound, bound)
            if face in seam_faces and unambiguous:
                matches = []
                for original in source["triangles"]:
                    triangle = [source["vertices"][i] for i in original]
                    distance, weights = closest(point, triangle)
                    normal = np.cross(sub(triangle[1], triangle[0]), sub(triangle[2], triangle[0]))
                    normal /= np.linalg.norm(normal)
                    center = np.asarray(weights) @ np.asarray(triangle)
                    matches.append((distance, float(np.dot(np.asarray(point)-center, normal))))
                signed = min(matches, key=lambda item: item[0])[1]
                if abs(signed) > source["thickness_m"]*.125:
                    seam_sides["front" if signed > 0 else "back"] += 1
            probes.append({"point": point, "uv": coordinate, "nearest_source_uv": nearest,
                           "source_uv_unambiguous": unambiguous, "raw_uv_error_texels": raw,
                           "preserving_uv_error_texels": preserved, "barycentric_weights": weights, "values": values})
        if expected["require_seam"]:
            assert all(seam_sides.values()), {"seam_sides": seam_sides, "candidate_faces": len(seam_faces)}
        frames.append({"seconds": source["seconds"], "vertices": len(points), "triangles": len(triangles),
                       "closed_oriented_edges": len(edges), "seam_position_groups": seam_groups,
                       "maximum_raw_triangle_uv_span": raw_triangle_span,
                       "unambiguous_seam_side_probes": seam_sides, "thickness_witnesses": thickness_witnesses, "probes": probes})
    assert uv_error <= expected["request"]["texture_delivery"]["max_uv_error_texels"]+.003, uv_error
    assert appearance_error <= 2e-6, appearance_error
    return {"reader": ".".join(map(str, Usd.GetVersion())), "validators": len(metadata),
            "original_image_sha256": original_hashes, "resolved_relative_assets": sorted(set(resolved_assets)),
            "texture_metadata": texture_metadata,
            "maximum_converted_pixel_error": converted_error, "maximum_uv_error_texels": uv_error,
            "maximum_raw_uv_error_texels": raw_uv_error, "common_preserving_uv_periods": periods,
            "maximum_sampled_shader_signal_error": appearance_error, "frames": frames,
            "maximum_attachment_signal_error": attachment_error, "maximum_attachment_signal_bound": attachment_bound,
            "appearance_scope": "Independent bilinear shader signals compared with source images at independently closest source UV; the attachment bound derives from measured image gradients and UV error. Native/consumer LOD and renderer BRDF parity are not asserted."}


def reader(path, expected_path, result_path, args, half_phase_negative=False):
    command = [sys.executable, str(args.inspector_script), "--inspect-bundle", str(path),
        "--expected", str(expected_path), "--reader-result", str(result_path),
        "--reader-path", str(args.reader_path.resolve()), "--exr-reader-path", str(args.exr_reader_path.resolve())]
    if half_phase_negative:
        command.append("--half-phase-negative")
    process = subprocess.run(command, capture_output=True, text=True, timeout=180)
    result_path.with_suffix(".stdout.log").write_text(process.stdout)
    result_path.with_suffix(".stderr.log").write_text(process.stderr)
    if half_phase_negative:
        assert process.returncode != 0 and "USD UV attachment error" in process.stderr, process.stderr
        metadata = json.loads(next(path.parent.glob("mm3e-assets-*/manifest.json")).read_text())["negative_control"]
        return {"rejected": True, "reason": "USD UV attachment error", "corruption": metadata,
                "geometry_and_assets_unchanged": True}
    assert process.returncode == 0, process.stderr
    return json.loads(result_path.read_text())


def shift_phase_negative(path, expected, args):
    """Keep periodic endpoint samples but corrupt their interior interpolation."""
    Usd, UsdGeom, _, _, _, _, _, _ = read_modules(args.reader_path, args.exr_reader_path)
    stage = Usd.Stage.Open(str(path))
    assert stage
    def geometry_signature():
        geometry = []
        for prim in stage.Traverse():
            if prim.IsA(UsdGeom.Mesh):
                mesh = UsdGeom.Mesh(prim)
                fields = {}
                for name in ("points", "faceVertexCounts", "faceVertexIndices", "normals", "extent"):
                    attribute = prim.GetAttribute(name)
                    def native(value):
                        return [list(v) if hasattr(v, "__len__") else v for v in value] if value is not None else None
                    fields[name] = {"default": native(attribute.Get()), "samples":
                                    [(t, native(attribute.Get(t))) for t in attribute.GetTimeSamples()]}
                geometry.append((str(mesh.GetPath()), fields))
        return hashlib.sha256(json.dumps(geometry, sort_keys=True).encode()).hexdigest()
    geometry_before = geometry_signature()
    binding = next(b for b in expected["document"]["texture_bindings"] if b["object"] == expected["object"])
    periods = common_periods(binding)
    axis = next((i for i, period in enumerate(periods) if period == 1), None)
    if axis is None:
        axis = next((i for i, period in enumerate(periods) if period is not None), None)
    period = periods[axis] if axis is not None else None
    def shifted(values):
        result = []
        for i, value in enumerate(values):
            coordinate = list(map(float, value))
            if period is None:
                coordinate[0] += .5
            elif i % 3 == 0:
                coordinate[axis] += period
            result.append(coordinate)
        return result
    changed = 0
    for prim in stage.Traverse():
        if not prim.IsA(UsdGeom.Mesh):
            continue
        st = UsdGeom.PrimvarsAPI(prim).GetPrimvar("st")
        assert st and not st.IsIndexed()
        values = st.Get()
        if values is not None:
            st.Set(shifted(values))
        for time_code in st.GetAttr().GetTimeSamples():
            values = st.Get(time_code)
            st.Set(shifted(values), time_code)
        changed += 1
    assert changed > 0
    stage.GetRootLayer().Save()
    stage.GetRootLayer().Reload()
    assert geometry_signature() == geometry_before
    manifest_path = path.parent / expected["asset_directory"] / "manifest.json"
    manifest = json.loads(manifest_path.read_text())
    manifest["layer_sha256"] = sha(path)
    manifest["negative_control"] = {"axis": "uv"[axis] if axis is not None else "u",
        "corner_period_shift": period, "edge_midpoint_phase_shift": period/2 if period is not None else .5,
        "endpoint_samples_preserved_by_all_maps": period is not None,
        "operation": "First corner of each face shifted by a common period; edge midpoint blends are wrong" if period is not None
                     else "No preserving period; all U coordinates shifted by 0.5",
        "geometry_signature_sha256": geometry_before}
    write_json(manifest_path, manifest)


def export_request(path, identifier, frames, resolution, clip):
    points = [p for f in frames for p in f["vertices"]]
    margin = max(.005, max(f["thickness_m"] for f in frames)*.75)
    minimum = [min(p[a] for p in points)-margin for a in range(3)]
    maximum = [max(p[a] for p in points)+margin for a in range(3)]
    end = frames[-1]["seconds"]
    return {"path": path, "object_ids": [identifier], "bounds_min": minimum, "bounds_max": maximum,
            "resolution": resolution, "max_surface_error_m": .015, "max_field_residual": .003,
            "clip": clip, "start_seconds": 0, "end_seconds": end,
            "frames_per_second": 2/end if end else 24,
            "texture_delivery": {"filtering": "reader_defined", "max_uv_error_texels": .25, "max_refinement_passes": 8}}


def reject(client, root, request):
    before, revision = client.call({"op": "get_document"}), client.revision
    inventory = {p.relative_to(root).as_posix() for p in root.rglob("*")}
    files = {p.relative_to(root).as_posix(): sha(p) for p in root.rglob("*") if p.is_file()}
    response = client.call({"op": "export_usd", "request": request}, expect_ok=False)
    assert client.revision == revision and client.call({"op": "get_document"}) == before
    assert {p.relative_to(root).as_posix() for p in root.rglob("*")} == inventory
    assert {p.relative_to(root).as_posix(): sha(p) for p in root.rglob("*") if p.is_file()} == files
    return response["error"]


def positive(client, root, identifier, clip, seconds, args, require_seam=False, resolution=(64, 64, 64), challenge_kind=None):
    document = client.call({"op": "get_document"})
    frames = []
    obj = next(o for o in document["objects"] if o["id"] == identifier)
    for t in seconds:
        asset = next((d for d in document.get("deformers", []) if d["object"] == identifier), None)
        if asset is not None:
            state = client.call({"op": "deformer_state", "id": asset["id"], "animation": {"clip": clip, "time": t}})
        elif any(c["id"] == identifier for c in document["cloths"]):
            state = client.call({"op": "cloth_state", "id": identifier, "animation": {"clip": clip, "time": t}})
        else:
            state = obj["shape"]
        frames.append({"seconds": t, "vertices": state["vertices"], "triangles": obj["shape"]["triangles"],
                       "thickness_m": state["thickness_m"]})
    request = export_request("bundle/animated.usda", identifier, frames, list(resolution), clip)
    write_json(root / "request.json", request)
    challenge_identity = None
    if args.challenge_root is not None and challenge_kind is not None:
        challenge_identity = validate_challenge(root, args.challenge_root, challenge_kind, request)
    (root / "bundle").mkdir()
    no_policy = copy.deepcopy(request)
    no_policy.pop("texture_delivery")
    missing_policy = reject(client, root, no_policy)
    assert missing_policy["code"] == "invalid" and "requires explicit texture_delivery" in missing_policy["message"]
    revision = client.revision
    result = client.call({"op": "export_usd", "request": request})
    work_validation = validate_work(result.get("delivery"))
    assert client.revision == revision and client.call({"op": "get_document"}) == document
    asset_directory = "mm3e-assets-"+hashlib.sha256(request["path"].encode()).hexdigest()
    assert Path(result["asset_directory"]).name == asset_directory
    expected = {"document": document, "object": identifier, "frames": frames, "request": request,
                "asset_directory": asset_directory, "require_seam": require_seam, "texture_size": [64, 64]}
    write_json(root / "expected.json", expected)
    first = reader(root / request["path"], root / "expected.json", root / "reader-original.json", args)
    field_error = 0
    for frame in first["frames"]:
        samples = client.call({"op": "sample", "id": identifier, "points": [p["point"] for p in frame["probes"]],
                               "animation": {"clip": clip, "time": frame["seconds"]}})["samples"]
        field_error = max(field_error, max(abs(s["value"]) for s in samples))
    assert field_error <= request["max_field_residual"]+1e-6, field_error
    shutil.copytree(root / "bundle", root / "relocated bundle")
    (root / "bundle").rename(root / "original-bundle-removed")
    relocated = reader(root / "relocated bundle/animated.usda", root / "expected.json", root / "reader-relocated.json", args)
    assert first == relocated
    shutil.copytree(root / "relocated bundle", root / "half-phase-negative")
    phase_control = reader(root / "half-phase-negative/animated.usda", root / "expected.json",
                           root / "reader-half-phase-negative.json", args, half_phase_negative=True)
    client.call({"op": "save", "path": "export-source.json"})
    original_layer = sha(root / "original-bundle-removed/animated.usda")
    original_assets = {p.name: sha(p) for p in (root / "original-bundle-removed" / asset_directory).iterdir() if p.name != "manifest.json"}
    return {"request": request, "export_response": result, "work_validation": work_validation,
            "challenge_identity": challenge_identity, "independent_reader": first,
            "maximum_native_field_residual": field_error, "missing_policy_rejection": missing_policy,
            "relocated_bundle_exact": True, "half_phase_negative_control": phase_control,
            "original_layer_sha256": original_layer,
            "original_asset_sha256": original_assets}


def run_fixture(binary, root, transcript, kind, args):
    root.mkdir()
    source = args.fixture_root / kind / "textured.json"
    shutil.copyfile(source, root / "input.json")
    client = Client(binary, root, transcript, kind)
    try:
        client.call({"op": "load", "path": "input.json"}, True)
        identifier, clip, times, resolution = (("face/lips", "expression", [0, .4, .8], (128, 128, 64)) if kind == "face"
                                               else ("garment", "pose", [0, .15, .3], (96, 96, 128)))
        result = positive(client, root, identifier, clip, times, args, kind == "face", resolution, kind)
        client.close()
        client = Client(binary, root, transcript, kind+"-cold")
        client.call({"op": "load", "path": "export-source.json"}, True)
        (root / "bundle").mkdir()
        cold_result = client.call({"op": "export_usd", "request": result["request"]})
        result["cold_work_validation"] = validate_work(cold_result.get("delivery"))
        assert sha(root / result["request"]["path"]) == result["original_layer_sha256"]
        directory = Path(result["export_response"]["asset_directory"]).name
        assert {p.name: sha(p) for p in (root / "bundle" / directory).iterdir() if p.name != "manifest.json"} == result["original_asset_sha256"]
        result["cold_export_bytes_exact"] = True
        return result
    finally:
        client.close()


def run_material(binary, root, transcript, args, floor=False):
    root.mkdir()
    client = Client(binary, root, transcript, "material-floor" if floor else "material")
    try:
        source = json.loads((args.fixture_root / "face/material-project.json").read_text())["document"]
        for texture in source["textures"]:
            (root / (texture["id"]+".png")).write_bytes(base64.b64decode(texture["data"], validate=True))
            client.call({"op": "import_texture", "request": {"id": texture["id"], "path": texture["id"]+".png",
                         "color_space": texture["color_space"]}}, True)
        material = {"albedo": [.4, .6, .8], "roughness": .04 if floor else 1, "metallic": .7, "emissive": [4, 1, .25]}
        binding = {"object": "tile", "uv_set": "charts", "texture": "print",
                   "sampler": {"u": "mirror", "v": "repeat", "filter": "trilinear"},
                   "roughness": {"texture": "packed", "channel": "r"}, "metallic": {"texture": "packed", "channel": "a"},
                   "emissive": {"texture": "glow"}, "normal": {"texture": "detail", "strength": 0}}
        client.apply([
            {"op": "create", "object": {"id": "tile", "material": material, "shape": {"type": "surface",
                "vertices": [[-.5, -.5, 0], [.5, -.5, 0], [.5, .5, 0], [-.5, .5, 0]], "triangles": [[0, 1, 2], [0, 2, 3]], "thickness_m": .02}}},
            {"op": "put_uvs", "request": {"id": "charts", "object": "tile", "values": [[-.2, -.1], [.3, -.1], [.3, 1.1], [.8, -.1], [1.3, 1.1], [.8, 1.1]], "corner_indices": [[0, 1, 2], [3, 4, 5]]}},
            {"op": "bind_texture", "binding": binding}, {"op": "put_clip", "clip": {"id": "still", "duration": 1}}])
        result = positive(client, root, "tile", "still", [0], args, True, (96, 96, 24))
        material["roughness"] = .1
        client.apply([{"op": "update", "id": "tile", "patch": {"material": material}}])
        invalid = dict(result["request"], path="mixed-floor.usda")
        error = reject(client, root, invalid)
        assert error["code"] == "invalid" and "roughness straddles the native 0.04 floor" in error["message"], error
        result["mixed_floor_rejection"] = error
        return result
    finally:
        client.close()


def run_normal_rejection(binary, root, transcript, args):
    root.mkdir()
    shutil.copyfile(args.fixture_root / "face/material-project.json", root / "input.json")
    client = Client(binary, root, transcript, "normal-rejection")
    try:
        client.call({"op": "load", "path": "input.json"}, True)
        request = {"path": "active-normal.usda", "object_ids": ["face/lips"], "bounds_min": [-1, 0, -1], "bounds_max": [1, 2, 1],
                   "resolution": [16, 16, 16], "max_surface_error_m": .1, "max_field_residual": .1,
                   "texture_delivery": {"filtering": "reader_defined", "max_uv_error_texels": .25, "max_refinement_passes": 8}}
        error = reject(client, root, request)
        assert error["code"] == "invalid" and "active normal map" in error["message"] and "tangent-frame transfer" in error["message"], error
        return {"active_normal_rejection": error, "no_project_or_output_change": True}
    finally:
        client.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--fixture-root", type=Path, default=FIXTURES)
    parser.add_argument("--challenge-root", type=Path,
                        help="require face/cloth inputs and every generated request field to match this frozen acceptance directory")
    parser.add_argument("--reader-path", type=Path, default=ROOT / ".dependencies/usd-reader")
    parser.add_argument("--exr-reader-path", type=Path, default=ROOT / ".dependencies/exr-reader")
    parser.add_argument("--self-test-only", action="store_true", help="run acceptance-contract positive and negative controls without an editor")
    parser.add_argument("--inspect-bundle", type=Path, help=argparse.SUPPRESS)
    parser.add_argument("--expected", type=Path, help=argparse.SUPPRESS)
    parser.add_argument("--reader-result", type=Path, help=argparse.SUPPRESS)
    parser.add_argument("--half-phase-negative", action="store_true", help=argparse.SUPPRESS)
    args = parser.parse_args()
    if args.inspect_bundle:
        path = args.inspect_bundle.resolve(strict=True)
        expected = json.loads(args.expected.read_text())
        if args.half_phase_negative:
            shift_phase_negative(path, expected, args)
        result = inspect_bundle(path, expected, args.reader_path, args.exr_reader_path)
        write_json(args.reader_result, result)
        return
    if not args.output:
        parser.error("--output is required")
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    if args.self_test_only:
        report = {"status": "running", "harness_sha256": sha(Path(__file__))}
        try:
            report["contract_self_test"] = contract_self_test()
            report["status"] = "passed"
        except Exception as error:
            report.update(status="failed", error=str(error), traceback=traceback.format_exc())
            raise
        finally:
            write_json(root / "acceptance.json", report)
        return
    binary = root / ("mm3e-editor.exe" if args.binary.suffix == ".exe" else "mm3e-editor")
    shutil.copyfile(args.binary.resolve(strict=True), binary)
    binary.chmod(0o755)
    snapshot = root / "harness"
    snapshot.mkdir()
    helper_names = [Path(__file__).name, "agent_animation_acceptance.py", "agent_sewing_acceptance.py", "agent_usd_acceptance.py"]
    snapshots = {}
    for name in helper_names:
        source = Path(__file__).parent / name
        shutil.copyfile(source, snapshot / name)
        snapshots[name] = sha(snapshot / name)
        assert snapshots[name] == sha(source)
    args.inspector_script = snapshot / Path(__file__).name
    report = {"status": "running", "binary_sha256": sha(binary), "harness_sha256": sha(Path(__file__)), "cases": {},
              "harness_snapshot_sha256": snapshots,
              "challenge_root": str(args.challenge_root.resolve()) if args.challenge_root is not None else None,
              "scope": "Independent OpenUSD schema, UV/geometry attachment, image/EXR values, PreviewSurface signal connections, work accounting and bundle relocation; no independent all-pairs intersection, consumer renderer/filter/BRDF or anatomical-quality parity claim."}
    started = time.monotonic()
    try:
        report["contract_self_test"] = contract_self_test()
        with (root / "transcript.jsonl").open("w") as transcript:
            for name, runner in [("face", lambda p: run_fixture(binary, p, transcript, "face", args)),
                                 ("cloth", lambda p: run_fixture(binary, p, transcript, "cloth", args)),
                                 ("material", lambda p: run_material(binary, p, transcript, args)),
                                 ("constant-floor", lambda p: run_material(binary, p, transcript, args, True)),
                                 ("active-normal", lambda p: run_normal_rejection(binary, p, transcript, args))]:
                try:
                    report["cases"][name] = {"status": "passed", "result": runner(root/name)}
                except Exception as error:
                    report["cases"][name] = {"status": "failed", "error": str(error), "traceback": traceback.format_exc()}
                print("[textured USD]", name, report["cases"][name]["status"], flush=True)
                write_json(root / "acceptance.json", report)
        assert sha(binary) == report["binary_sha256"]
        assert all(sha(snapshot / name) == digest for name, digest in snapshots.items())
        failed = [name for name, case in report["cases"].items() if case["status"] != "passed"]
        if failed:
            raise AssertionError("Required cases failed: "+", ".join(failed))
        report["status"] = "passed"
    except Exception as error:
        report.update(status="failed", error=str(error))
        raise
    finally:
        report["elapsed_seconds"] = time.monotonic()-started
        report["artifact_sha256"] = {p.relative_to(root).as_posix(): sha(p) for p in sorted(root.rglob("*")) if p.is_file() and p.name != "acceptance.json"}
        write_json(root / "acceptance.json", report)


if __name__ == "__main__":
    main()
