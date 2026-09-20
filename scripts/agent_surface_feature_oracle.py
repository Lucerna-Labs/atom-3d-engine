#!/usr/bin/env python3
"""Independent rounded-triangle oracle plus unchanged delivery challenge probes.

The calibration meshes are generated only to test this verifier. Real export
results come from the separately identified editor process. A prism, midsurface
or coarse bevel is never substituted for an exported native thickened field.
"""
import argparse
import collections
import hashlib
import json
import math
from pathlib import Path
import shutil
import struct
import sys
import time
import traceback
import zlib

from agent_animation_acceptance import Client, write_json

ROOT = Path(__file__).resolve().parents[1]
FINAL_CHALLENGE = ROOT / "artifacts/textured-usd-acceptance-20260907-final"
WORK_CAP = 200_000_000
LOCAL_RADIUS_FRACTION = .02
VOLUME_RELATIVE_TOLERANCE = .001
UV_TOLERANCE_TEXELS = .25


def add(a, b):
    return [x+y for x, y in zip(a, b)]


def sub(a, b):
    return [x-y for x, y in zip(a, b)]


def scale(a, b):
    return [x*b for x in a]


def dot(a, b):
    return sum(x*y for x, y in zip(a, b))


def cross(a, b):
    return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]


def length(a):
    return math.sqrt(dot(a, a))


def unit(a):
    return scale(a, 1/length(a))


def f32(x):
    return struct.unpack("<f", struct.pack("<f", x))[0]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def closest(point, triangle):
    """Double-precision Euclidean triangle distance; no engine scalar is reused."""
    a, b, c = triangle
    ab, ac, ap = sub(b, a), sub(c, a), sub(point, a)
    aa, bb, cc, d, e = dot(ab, ab), dot(ab, ac), dot(ac, ac), dot(ap, ab), dot(ap, ac)
    determinant = aa*cc-bb*bb
    assert determinant > 0
    v, w = (cc*d-bb*e)/determinant, (aa*e-bb*d)/determinant
    candidates = [[1-v-w, v, w]] if v >= 0 and w >= 0 and v+w <= 1 else []
    for i, j in ((0, 1), (1, 2), (2, 0)):
        edge = sub(triangle[j], triangle[i])
        t = max(0, min(1, dot(sub(point, triangle[i]), edge)/dot(edge, edge)))
        weights = [0, 0, 0]
        weights[i], weights[j] = 1-t, t
        candidates.append(weights)
    def point_for(weights):
        return [sum(weights[i]*triangle[i][axis] for i in range(3)) for axis in range(3)]
    weights = min(candidates, key=lambda weights: length(sub(point, point_for(weights))))
    q = point_for(weights)
    return length(sub(point, q)), weights, q


def native_scalar(point, triangle, radius):
    return closest(point, triangle)[0]-radius


def measurements(triangle, radius):
    area = length(cross(sub(triangle[1], triangle[0]), sub(triangle[2], triangle[0])))/2
    perimeter = sum(length(sub(triangle[(i+1) % 3], triangle[i])) for i in range(3))
    # Integrate A + P*sqrt(r^2-z^2) + pi*(r^2-z^2) from -r to r.
    volume = 2*area*radius + math.pi*perimeter*radius**2/2 + 4*math.pi*radius**3/3
    return {"source_area_m2": area, "source_perimeter_m": perimeter, "rounded_volume_m3": volume,
            "rounded_area_m2": 2*area+math.pi*perimeter*radius+4*math.pi*radius**2,
            "prism_volume_m3": 2*area*radius}


def mesh_metrics(points, triangles):
    edges, links = collections.defaultdict(lambda: [0, 0]), collections.defaultdict(lambda: collections.defaultdict(set))
    assert points and triangles and all(math.isfinite(v) for p in points for v in p)
    volume_terms, areas = [], []
    for a, b, c in triangles:
        assert len({a, b, c}) == 3 and min(a, b, c) >= 0 and max(a, b, c) < len(points)
        n = cross(sub(points[b], points[a]), sub(points[c], points[a]))
        assert length(n) > 0, "degenerate delivered triangle"
        areas.append(length(n)/2)
        volume_terms.append(dot(points[a], cross(points[b], points[c]))/6)
        for i, j in ((a, b), (b, c), (c, a)):
            record = edges[min(i, j), max(i, j)]
            record[0] += 1
            record[1] += 1 if i < j else -1
        for vertex, i, j in ((a, b, c), (b, c, a), (c, a, b)):
            links[vertex][i].add(j)
            links[vertex][j].add(i)
    assert all(count == 2 and balance == 0 for count, balance in edges.values()), "boundary/nonmanifold/orientation edge failure"
    assert len(links) == len(points), "unused vertices"
    for neighbors in links.values():
        assert all(len(link) == 2 for link in neighbors.values()), "vertex link is not a cycle"
        seen, pending = set(), [next(iter(neighbors))]
        while pending:
            vertex = pending.pop()
            if vertex not in seen:
                seen.add(vertex)
                pending.extend(neighbors[vertex]-seen)
        assert seen == set(neighbors), "disconnected vertex link"
    seen, pending = set(), [0]
    while pending:
        vertex = pending.pop()
        if vertex not in seen:
            seen.add(vertex)
            pending.extend(set(links[vertex])-seen)
    assert len(seen) == len(points), "rounded convex triangle must have one component"
    assert len(points)-len(edges)+len(triangles) == 2, "rounded convex triangle must retain genus zero"
    volume = math.fsum(volume_terms)
    assert volume > 0, "globally reversed or zero-volume surface"
    return {"vertices": len(points), "triangles": len(triangles), "closed_oriented_edges": len(edges),
            "signed_volume_m3": volume, "surface_area_m2": math.fsum(areas), "connected_vertex_links": len(links)}


def witnesses(triangle, radius):
    normal = unit(cross(sub(triangle[1], triangle[0]), sub(triangle[2], triangle[0])))
    outward = [unit(cross(sub(triangle[(i+1) % 3], triangle[i]), normal)) for i in range(3)]
    center = scale(add(add(*triangle[:2]), triangle[2]), 1/3)
    result = [{"feature": "face", "origin": center, "direction": scale(normal, sign), "expected_distance": radius} for sign in (-1, 1)]
    for edge in range(3):
        origin = scale(add(triangle[edge], triangle[(edge+1) % 3]), .5)
        for degrees in (-75, -45, -15, 0, 15, 45, 75):
            theta = math.radians(degrees)
            direction = add(scale(outward[edge], math.cos(theta)), scale(normal, math.sin(theta)))
            result.append({"feature": "edge-cylinder", "origin": origin, "direction": direction,
                           "edge": edge, "elevation_degrees": degrees, "expected_distance": radius})
    for vertex in range(3):
        previous, following = outward[(vertex-1) % 3], outward[vertex]
        angle = math.atan2(dot(normal, cross(previous, following)), dot(previous, following)) % (2*math.pi)
        for fraction in (.2, .5, .8):
            angle_direction = add(scale(previous, math.cos(angle*fraction)),
                                  scale(cross(normal, previous), math.sin(angle*fraction)))
            for degrees in (-45, 0, 45):
                theta = math.radians(degrees)
                direction = add(scale(angle_direction, math.cos(theta)), scale(normal, math.sin(theta)))
                result.append({"feature": "spherical-corner", "origin": triangle[vertex], "direction": direction,
                               "vertex": vertex, "normal_fan_fraction": fraction,
                               "elevation_degrees": degrees, "expected_distance": radius})
    for witness in result:
        point = add(witness["origin"], scale(witness["direction"], radius))
        assert abs(native_scalar(point, triangle, radius)) < radius*1e-8
    return result


def ray_hit(origin, direction, points, triangles, maximum):
    nearest = None
    for face, (a, b, c) in enumerate(triangles):
        e1, e2 = sub(points[b], points[a]), sub(points[c], points[a])
        h = cross(direction, e2)
        determinant = dot(e1, h)
        if abs(determinant) < 1e-20:
            continue
        tvec = sub(origin, points[a])
        u = dot(tvec, h)/determinant
        q = cross(tvec, e1)
        v = dot(direction, q)/determinant
        distance = dot(e2, q)/determinant
        if u >= -1e-8 and v >= -1e-8 and u+v <= 1+1e-8 and 0 <= distance <= maximum:
            if nearest is None or distance < nearest[0]:
                nearest = distance, face, [1-u-v, u, v]
    return nearest


def validate_mesh(points, triangles, source, radius, corner_uvs=None, texture_size=512):
    result = mesh_metrics(points, triangles)
    analytic = measurements(source, radius)
    volume_error = abs(result["signed_volume_m3"]-analytic["rounded_volume_m3"])/analytic["rounded_volume_m3"]
    assert volume_error <= VOLUME_RELATIVE_TOLERANCE, f"rounded volume relative error {volume_error}"
    maximum_residual, maximum_uv = 0, 0
    for face, triangle in enumerate(triangles):
        for weights in ([1, 0, 0], [0, 1, 0], [0, 0, 1], [.5, .5, 0], [0, .5, .5], [.5, 0, .5], [1/3]*3):
            point = [sum(weights[i]*points[triangle[i]][axis] for i in range(3)) for axis in range(3)]
            distance, bary, _ = closest(point, source)
            maximum_residual = max(maximum_residual, abs(distance-radius))
            if corner_uvs is not None:
                actual = [sum(weights[i]*corner_uvs[face][i][axis] for i in range(3)) for axis in range(2)]
                maximum_uv = max(maximum_uv, math.dist(actual, [bary[1], bary[2]])*texture_size)
    assert maximum_residual <= radius*LOCAL_RADIUS_FRACTION, f"rounded local scalar error {maximum_residual}m"
    assert maximum_uv <= UV_TOLERANCE_TEXELS, f"closest-feature UV error {maximum_uv} texels"
    checks = []
    for witness in witnesses(source, radius):
        hit = ray_hit(witness["origin"], witness["direction"], points, triangles, radius*3)
        assert hit is not None, f"missing {witness['feature']} boundary: {witness}"
        error = abs(hit[0]-radius)
        assert error <= radius*LOCAL_RADIUS_FRACTION, f"{witness['feature']} radial error {error}m: {witness}"
        checks.append({**witness, "actual_distance": hit[0], "error_m": error})
    result.update(analytic=analytic, volume_relative_error=volume_error, maximum_scalar_residual_m=maximum_residual,
                  maximum_uv_error_texels=maximum_uv, witnesses=checks,
                  local_tolerance_m=radius*LOCAL_RADIUS_FRACTION)
    return result


def rounded_reference(source, radius, elevation_steps=32, max_azimuth_step=math.pi/32):
    """Calibration-only exact face/cylinder/sphere vertices; never used as delivery."""
    normal = unit(cross(sub(source[1], source[0]), sub(source[2], source[0])))
    outward = [unit(cross(sub(source[(i+1) % 3], source[i]), normal)) for i in range(3)]
    points, faces, ids = [], [], {}
    def vertex(center, azimuth, elevation):
        if elevation == 0:
            p = add(center, scale(normal, -radius))
        elif elevation == elevation_steps:
            p = add(center, scale(normal, radius))
        else:
            theta = -math.pi/2+math.pi*elevation/elevation_steps
            p = add(center, scale(add(scale(azimuth, math.cos(theta)), scale(normal, math.sin(theta))), radius))
        key = tuple(round(v, 13) for v in p)
        if key not in ids:
            ids[key] = len(points)
            points.append(list(map(f32, p)))
        return ids[key]
    def face(a, b, c):
        if len({a, b, c}) == 3:
            faces.append([a, b, c])
    bottom = [vertex(p, normal, 0) for p in source]
    top = [vertex(p, normal, elevation_steps) for p in source]
    face(*top)
    face(bottom[0], bottom[2], bottom[1])
    for edge in range(3):
        a = [vertex(source[edge], outward[edge], j) for j in range(elevation_steps+1)]
        b = [vertex(source[(edge+1) % 3], outward[edge], j) for j in range(elevation_steps+1)]
        for j in range(elevation_steps):
            face(a[j], b[j], b[j+1])
            face(a[j], b[j+1], a[j+1])
    for corner in range(3):
        previous, following = outward[(corner-1) % 3], outward[corner]
        angle = math.atan2(dot(normal, cross(previous, following)), dot(previous, following)) % (2*math.pi)
        steps = math.ceil(angle/max_azimuth_step)
        grid = []
        for i in range(steps+1):
            direction = (previous if i == 0 else following if i == steps else
                         add(scale(previous, math.cos(angle*i/steps)), scale(cross(normal, previous), math.sin(angle*i/steps))))
            grid.append([vertex(source[corner], direction, j) for j in range(elevation_steps+1)])
        for i in range(steps):
            for j in range(elevation_steps):
                face(grid[i][j], grid[i+1][j], grid[i+1][j+1])
                face(grid[i][j], grid[i+1][j+1], grid[i][j+1])
    uvs = [[[closest(points[index], source)[1][axis] for axis in (1, 2)] for index in face] for face in faces]
    return points, faces, uvs


def prism(source, radius):
    normal = unit(cross(sub(source[1], source[0]), sub(source[2], source[0])))
    points = [add(p, scale(normal, sign*radius)) for sign in (-1, 1) for p in source]
    faces = [[3, 4, 5], [0, 2, 1]]
    for i in range(3):
        j = (i+1) % 3
        faces += [[i, j, j+3], [i, j+3, i+3]]
    return points, faces


def self_test(root):
    source = [[-.3, -.25, 0], [.3, -.25, 0], [-.3, .35, 0]]
    radius = .001
    points, faces, uvs = rounded_reference(source, radius)
    positive = validate_mesh(points, faces, source, radius, uvs)
    negatives = {}
    coarse = rounded_reference(source, radius, elevation_steps=2, max_azimuth_step=math.pi/2)
    for name, (vertices, triangles) in {"midsurface": (source, [[0, 1, 2]]), "prism": prism(source, radius),
                                       "coarse-bevel": coarse[:2]}.items():
        loose_residual = 0
        for triangle in triangles:
            for weights in ([1, 0, 0], [0, 1, 0], [0, 0, 1], [.5, .5, 0], [0, .5, .5], [.5, 0, .5], [1/3]*3):
                point = [sum(weights[i]*vertices[triangle[i]][axis] for i in range(3)) for axis in range(3)]
                loose_residual = max(loose_residual, abs(native_scalar(point, source, radius)))
        assert loose_residual < .003, (name, loose_residual)
        curved_failures, missing_rays, curved_errors = 0, 0, []
        for witness in witnesses(source, radius):
            if witness["feature"] == "face":
                continue
            hit = ray_hit(witness["origin"], witness["direction"], vertices, triangles, 3*radius)
            if hit is None:
                missing_rays += 1
                curved_failures += 1
            else:
                error = abs(hit[0]-radius)
                curved_errors.append(error)
                curved_failures += error > radius*LOCAL_RADIUS_FRACTION
        assert curved_failures > 0, name+" surrogate passed the rounded-edge/corner rays"
        try:
            validate_mesh(vertices, triangles, source, radius)
        except AssertionError as error:
            negatives[name] = {"rejected": True, "reason": str(error), "passes_old_loose_scalar_gate": True,
                               "old_gate_maximum_scalar_residual_m": loose_residual,
                               "failed_edge_corner_witnesses": curved_failures, "missing_edge_corner_hits": missing_rays,
                               "maximum_edge_corner_error_m": max(curved_errors, default=None)}
        else:
            raise AssertionError(name+" surrogate passed focused oracle")
    # Dropping closest-feature clamping extrapolates UVs onto edge/corner caps.
    a, b, c = source
    ab, ac = sub(b, a), sub(c, a)
    def plane_uv(point):
        ap = sub(point, a)
        aa, bb, cc = dot(ab, ab), dot(ab, ac), dot(ac, ac)
        d, e = dot(ap, ab), dot(ap, ac)
        return [(cc*d-bb*e)/(aa*cc-bb*bb), (aa*e-bb*d)/(aa*cc-bb*bb)]
    bad_uvs = [[plane_uv(points[index]) for index in triangle] for triangle in faces]
    try:
        validate_mesh(points, faces, source, radius, bad_uvs)
    except AssertionError as error:
        assert "closest-feature UV error" in str(error)
        negatives["unclamped-feature-uv"] = {"rejected": True, "reason": str(error), "geometry_unchanged": True}
    else:
        raise AssertionError("unclamped UV feature surrogate passed")
    report = {"status": "passed", "calibration_only": True, "positive": positive, "negative_controls": negatives}
    write_json(root / "oracle-self-test.json", report)
    return report


def read_usd(path, reader_path):
    sys.path.insert(0, str(reader_path))
    from pxr import Usd, UsdGeom
    stage = Usd.Stage.Open(str(path))
    meshes = [UsdGeom.Mesh(p) for p in stage.Traverse() if p.IsA(UsdGeom.Mesh)]
    assert len(meshes) == 1
    mesh = meshes[0]
    points = [list(p) for p in mesh.GetPointsAttr().Get(0)]
    indices = list(mesh.GetFaceVertexIndicesAttr().Get(0))
    assert all(c == 3 for c in mesh.GetFaceVertexCountsAttr().Get(0))
    st = UsdGeom.PrimvarsAPI(mesh).GetPrimvar("st")
    coordinates = [list(uv) for uv in st.ComputeFlattened(0)] if st else None
    if coordinates is not None:
        assert st.GetInterpolation() == "faceVarying" and len(coordinates) == len(indices)
        coordinates = [coordinates[i:i+3] for i in range(0, len(coordinates), 3)]
    return points, [indices[i:i+3] for i in range(0, len(indices), 3)], coordinates


def calibration_png():
    size = 512
    rows = []
    for y in range(size):
        row = bytearray([0])
        for x in range(size):
            row.extend([x % 256, y % 256, 255 if (x//16+y//16) % 2 else 0, 255])
        rows.append(bytes(row))
    def chunk(name, data):
        return struct.pack(">I", len(data))+name+data+struct.pack(">I", zlib.crc32(name+data))
    return b"\x89PNG\r\n\x1a\n"+chunk(b"IHDR", struct.pack(">IIBBBBB", size, size, 8, 6, 0, 0, 0))+chunk(b"IDAT", zlib.compress(b"".join(rows)))+chunk(b"IEND", b"")


def focused_export(binary, root, transcript, reader_path, tilted=False, with_uv=False):
    root.mkdir()
    source = [[-.3, -.25, .014], [.3, -.25, .014], [-.3, .35, .014]]
    if tilted:
        angle = math.radians(31)
        source = [[x, y*math.cos(angle)-z*math.sin(angle), y*math.sin(angle)+z*math.cos(angle)] for x, y, z in source]
    client = Client(binary, root, transcript, "tilted" if tilted else "flat-alias")
    try:
        client.apply([{"op": "create", "object": {"id": "triangle", "shape": {"type": "surface", "vertices": source,
                       "triangles": [[0, 1, 2]], "thickness_m": .002}}}])
        if with_uv:
            (root / "calibration.png").write_bytes(calibration_png())
            client.call({"op": "import_texture", "request": {"id": "calibration", "path": "calibration.png", "color_space": "linear"}}, True)
            client.apply([
                {"op": "put_uvs", "request": {"id": "source-uv", "object": "triangle", "values": [[0, 0], [1, 0], [0, 1]], "corner_indices": [[0, 1, 2]]}},
                {"op": "bind_texture", "binding": {"object": "triangle", "uv_set": "source-uv", "texture": "calibration",
                                                   "sampler": {"u": "clamp", "v": "clamp", "filter": "bilinear"}}}])
        document = client.call({"op": "get_document"})
        source = document["objects"][0]["shape"]["vertices"]
        radius = document["objects"][0]["shape"]["thickness_m"]/2
        request = {"path": "rounded.usda", "object_ids": ["triangle"],
                   "bounds_min": [-.4, -.4, -.03] if not tilted else [-.4, -.4, -.3],
                   "bounds_max": [.4, .45, .05] if not tilted else [.4, .45, .3],
                   "resolution": [32, 32, 8] if not tilted else [32, 32, 32],
                   "max_surface_error_m": radius*.04, "max_field_residual": radius*LOCAL_RADIUS_FRACTION}
        if with_uv:
            request["texture_delivery"] = {"filtering": "reader_defined", "max_uv_error_texels": UV_TOLERANCE_TEXELS, "max_refinement_passes": 8}
        write_json(root / "request.json", request)
        write_json(root / "source.json", document)
        center = scale(add(add(source[0], source[1]), source[2]), 1/3)
        boundary = [add(w["origin"], scale(w["direction"], radius)) for w in witnesses(source, radius)]
        native = client.call({"op": "sample", "id": "triangle", "points": [center]+boundary})["samples"]
        assert native[0]["value"] < -.99*radius
        field_errors = [abs(v["value"]-native_scalar(p, source, radius)) for p, v in zip([center]+boundary, native)]
        assert max(field_errors) < radius*.0002
        minimum_lattice = math.inf
        for z in range(request["resolution"][2]+1):
            for y in range(request["resolution"][1]+1):
                for x in range(request["resolution"][0]+1):
                    point = [request["bounds_min"][a]+(request["bounds_max"][a]-request["bounds_min"][a])*i/request["resolution"][a]
                             for a, i in enumerate((x, y, z))]
                    minimum_lattice = min(minimum_lattice, native_scalar(point, source, radius))
        if not tilted:
            assert minimum_lattice > 0, "flat-alias fixture must hide both sides from every lattice corner"
        write_json(root / "source-field-witnesses.json", {"interior_sample": native[0]["value"],
            "boundary_witness_count": len(boundary), "maximum_native_vs_oracle_error_m": max(field_errors),
            "minimum_lattice_oracle_value_m": minimum_lattice, "all_lattice_nodes_outside": minimum_lattice > 0})
        response = client.call({"op": "export_usd", "request": request})
        assert response["delivery"]["charged_work_total"] <= WORK_CAP
        points, triangles, coordinates = read_usd(root / "rounded.usda", reader_path)
        if with_uv:
            assert coordinates is not None
        result = validate_mesh(points, triangles, source, radius, coordinates)
        return {"status": "passed", "request": request, "export": response, "independent_oracle": result}
    finally:
        client.close()


def challenge_probe(binary, root, transcript, challenge_root, name):
    root.mkdir()
    original = challenge_root / name
    shutil.copyfile(original / "input.json", root / "input.json")
    shutil.copyfile(original / "request.json", root / "request.json")
    assert sha(root / "input.json") == sha(original / "input.json")
    assert sha(root / "request.json") == sha(original / "request.json")
    request = json.loads((root / "request.json").read_text())
    assert request["max_surface_error_m"] == .015 and request["max_field_residual"] == .003
    assert request["texture_delivery"]["max_uv_error_texels"] == .25
    assert request["resolution"] == ([128, 128, 64] if name == "face" else [96, 96, 128])
    expected_times = [0, .4, .8] if name == "face" else [0, .15, .3]
    actual_times = [request["start_seconds"]+i/request["frames_per_second"] for i in range(3)]
    assert all(abs(a-b) < 1e-12 for a, b in zip(actual_times, expected_times))
    (root / "bundle").mkdir()
    client = Client(binary, root, transcript, "unchanged-"+name)
    try:
        client.call({"op": "load", "path": "input.json"}, True)
        revision = client.revision
        result = client.call({"op": "export_usd", "request": request})
        assert client.revision == revision and result["delivery"]["charged_work_total"] <= WORK_CAP
        return {"status": "exported_pending_primary_verifier", "export": result,
                "input_sha256": sha(root / "input.json"), "request_sha256": sha(root / "request.json"),
                "times": expected_times, "primary_verifier_still_required": True}
    finally:
        client.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=FINAL_CHALLENGE / "mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--reader-path", type=Path, default=ROOT / ".dependencies/usd-reader")
    parser.add_argument("--challenge-root", type=Path, default=FINAL_CHALLENGE)
    parser.add_argument("--self-test-only", action="store_true")
    parser.add_argument("--focused-only", action="store_true", help="run real local-radius tests; full unchanged challenge remains a separate required acceptance")
    args = parser.parse_args()
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    sources = [Path(__file__), Path(__file__).with_name("agent_animation_acceptance.py")]
    snapshot = root / "harness"
    snapshot.mkdir()
    harness_hashes = {}
    for source in sources:
        shutil.copyfile(source, snapshot/source.name)
        harness_hashes[source.name] = sha(source)
    report = {"status": "running", "harness_sha256": sha(Path(__file__)), "harness_snapshot_sha256": harness_hashes, "cases": {},
              "gates": {"unchanged_work_cap": WORK_CAP, "local_radial_scalar_error_fraction": LOCAL_RADIUS_FRACTION,
                        "volume_relative_error": VOLUME_RELATIVE_TOLERANCE, "feature_uv_error_texels": UV_TOLERANCE_TEXELS}}
    started = time.monotonic()
    try:
        report["oracle_self_test"] = self_test(root)
        if not args.self_test_only:
            binary = root / ("mm3e-editor.exe" if args.binary.suffix == ".exe" else "mm3e-editor")
            shutil.copyfile(args.binary.resolve(strict=True), binary)
            binary.chmod(0o755)
            report["binary_sha256"] = sha(binary)
            with (root / "transcript.jsonl").open("w") as transcript:
                runs = [("flat-alias", lambda path: focused_export(binary, path, transcript, args.reader_path)),
                        ("tilted", lambda path: focused_export(binary, path, transcript, args.reader_path, True)),
                        ("feature-uv", lambda path: focused_export(binary, path, transcript, args.reader_path, with_uv=True))]
                if not args.focused_only:
                    runs.extend([("face", lambda path: challenge_probe(binary, path, transcript, args.challenge_root, "face")),
                                 ("cloth", lambda path: challenge_probe(binary, path, transcript, args.challenge_root, "cloth"))])
                for name, run in runs:
                    try:
                        report["cases"][name] = run(root/name)
                    except Exception as error:
                        report["cases"][name] = {"status": "failed", "error": str(error), "traceback": traceback.format_exc()}
                    write_json(root / "acceptance.json", report)
                    print(name, report["cases"][name]["status"], flush=True)
            assert sha(binary) == report["binary_sha256"]
        assert all(sha(source) == harness_hashes[source.name] and sha(snapshot/source.name) == harness_hashes[source.name] for source in sources)
        report["status"] = "passed" if not report["cases"] or all(c["status"] == "passed" for c in report["cases"].values()) else "failed"
    except Exception as error:
        report.update(status="failed", error=str(error), traceback=traceback.format_exc())
        raise
    finally:
        report["elapsed_seconds"] = time.monotonic()-started
        report["artifact_sha256"] = {p.relative_to(root).as_posix(): sha(p) for p in sorted(root.rglob("*"))
                                     if p.is_file() and p.name != "acceptance.json"}
        write_json(root / "acceptance.json", report)
    if report["status"] != "passed":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
