#!/usr/bin/env python3
"""Actual editor texture attachment on a morphing face and simulated sewn cloth.

Uses the existing continuous-face and garment fixtures. UVs, source image bytes,
linear alpha filtering, geometry ownership, physical-cache preservation, renders
and cold reload are checked. This is not an artistic or film-delivery certification.
"""
import argparse
import copy
import hashlib
import json
import math
from pathlib import Path
import shutil
import struct
import time
import traceback
import zlib

from agent_animation_acceptance import Client, write_json
from agent_deformed_face_acceptance import scene_recipe
from agent_pattern_acceptance import recipe, source_operations, prepare
from agent_sewing_acceptance import png_pixels, pixel_difference

ROOT = Path(__file__).resolve().parents[1]
SIZE = 64


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def pattern():
    pixels = []
    for y in range(SIZE):
        for x in range(SIZE):
            rgb = [235, 205, 100] if (x//8 + y//8) % 2 else [35, 85, 165]
            if 8 <= x < 24 and 8 <= y < 24:
                rgb = [235, 35, 25]
            if 40 <= x < 56 and 40 <= y < 56:
                rgb = [30, 230, 60]
            alpha = 0 if x < 2 else 128 if x < 4 else 255
            pixels.append([*rgb, alpha])
    raw = b"".join(b"\x00" + bytes(sum(pixels[y*SIZE:(y+1)*SIZE], [])) for y in range(SIZE))
    def chunk(name, payload):
        return struct.pack(">I", len(payload)) + name + payload + struct.pack(">I", zlib.crc32(name + payload))
    encoded = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", SIZE, SIZE, 8, 6, 0, 0, 0)) + chunk(b"IDAT", zlib.compress(raw)) + chunk(b"IEND", b"")
    return encoded, pixels


def linear(pixel):
    a = pixel[3]/255
    def decode(v):
        v /= 255
        return v/12.92 if v <= .04045 else ((v+.055)/1.055)**2.4
    return [decode(c)*a for c in pixel[:3]] + [a]


def texture_sample(pixels, uv, repeat_u=False):
    u = uv[0] % 1 if repeat_u else min(1, max(0, uv[0]))
    v = min(1, max(0, uv[1]))
    x, y = u*SIZE-.5, (1-v)*SIZE-.5
    ix, iy, fx, fy = math.floor(x), math.floor(y), x % 1, y % 1
    def pixel(x, y):
        x = x % SIZE if repeat_u else max(0, min(SIZE-1, x))
        y = max(0, min(SIZE-1, y))
        return linear(pixels[y*SIZE+x])
    a, b, c, d = pixel(ix, iy), pixel(ix+1, iy), pixel(ix, iy+1), pixel(ix+1, iy+1)
    return [((a[i]*(1-fx)+b[i]*fx)*(1-fy)+(c[i]*(1-fx)+d[i]*fx)*fy) for i in range(4)]


def sub(a, b):
    return [x-y for x, y in zip(a, b)]


def dot(a, b):
    return sum(x*y for x, y in zip(a, b))


def barycentric(point, vertices):
    a, b, c = vertices
    ab, ac, ap = sub(b, a), sub(c, a), sub(point, a)
    aa, bb, cc, d, e = dot(ab, ab), dot(ab, ac), dot(ac, ac), dot(ap, ab), dot(ap, ac)
    determinant = aa*cc-bb*bb
    assert determinant > 0
    v, w = (cc*d-bb*e)/determinant, (aa*e-bb*d)/determinant
    return [1-v-w, v, w]


def uv_recipe(obj):
    vertices, triangles = obj["shape"]["vertices"], obj["shape"]["triangles"]
    periodic = obj["id"] in ("face/lips", "face/mouth-skin")
    if periodic:
        columns, rows = 32, len(vertices)//32
        values = [[(i % columns)/columns, (i//columns)/(rows-1)] for i in range(len(vertices))]
        indices, copies = [], {}
        for triangle in triangles:
            closing = max(values[i][0] for i in triangle)-min(values[i][0] for i in triangle) > .5
            row = []
            for i in triangle:
                if closing and values[i][0] == 0:
                    if i not in copies:
                        copies[i] = len(values)
                        values.append([1, values[i][1]])
                    row.append(copies[i])
                else:
                    row.append(i)
            indices.append(row)
        assert len(values) > len(vertices), "UV seam must add attributes without splitting geometry"
    elif obj["id"].endswith("-lid"):
        columns, rows = 17, 9
        values = [[(i % columns)/(columns-1), (i//columns)/(rows-1)] for i in range(len(vertices))]
        indices = triangles
    else:
        lo = [min(p[a] for p in vertices) for a in (0, 1)]
        hi = [max(p[a] for p in vertices) for a in (0, 1)]
        values = [[(p[a]-lo[a])/(hi[a]-lo[a]) for a in (0, 1)] for p in vertices]
        indices = triangles
    return {"id": obj["id"]+"/uv", "object": obj["id"], "values": values, "corner_indices": indices}, periodic


def attach(client, objects):
    definitions, operations = {}, []
    for obj in objects:
        if obj["shape"]["type"] != "surface":
            continue
        uv, periodic = uv_recipe(obj)
        definitions[obj["id"]] = (uv, periodic)
        operations += [{"op": "put_uvs", "request": uv}, {"op": "bind_texture", "binding": {
            "object": obj["id"], "uv_set": uv["id"], "texture": "print",
            "sampler": {"u": "repeat" if periodic else "clamp", "v": "clamp", "filter": "trilinear"},
        }}]
    client.apply(operations)
    return definitions


def probes(client, document, definitions, pixels, animation, cloth=False):
    checks, max_uv, max_color = {}, 0.0, 0.0
    for identifier, (uv, periodic) in definitions.items():
        obj = next(o for o in document["objects"] if o["id"] == identifier)
        state = client.call({"op": "cloth_state" if cloth else "deformer_state", "id": identifier, "animation": animation})
        vertices, triangles = state["vertices"], obj["shape"]["triangles"]
        ids = list(range(0, len(triangles), max(1, len(triangles)//40)))
        points = [[sum(vertices[triangles[i][corner]][axis]*weight for corner, weight in enumerate([.23, .31, .46])) for axis in range(3)] for i in ids]
        observed = client.call({"op": "material_state", "request": {"points": points, "animation": animation}})["samples"]
        matched = 0
        for triangle, sample in zip(ids, observed):
            if sample["object"] != identifier:  # A different authored surface can cover this point.
                continue
            assert sample["triangle"] == triangle, (identifier, triangle, sample)
            weights = barycentric(sample["point"], [vertices[i] for i in triangles[triangle]])
            expected = [sum(uv["values"][i][axis]*w for i, w in zip(uv["corner_indices"][triangle], weights)) for axis in range(2)]
            max_uv = max(max_uv, max(abs(a-b) for a, b in zip(expected, sample["uv"])))
            rgba = texture_sample(pixels, sample["uv"], periodic)
            max_color = max(max_color, max(abs(a-b) for a, b in zip(rgba, sample["premultiplied_linear_rgba"])))
            expected_color = [obj["material"]["albedo"][i]*(rgba[i]+1-rgba[3]) for i in range(3)]
            max_color = max(max_color, max(abs(a-b) for a, b in zip(expected_color, sample["linear_albedo"])))
            matched += 1
        assert matched >= 10, (identifier, matched)
        checks[identifier] = matched
    assert max_uv < 2e-5, max_uv
    assert max_color < 2e-6, max_color
    return {"matched_visible_owner_probes": checks, "maximum_uv_error": max_uv, "maximum_linear_rgba_error": max_color}


def run_case(executable, root, transcript, kind, encoded, pixels):
    root.mkdir()
    source = root / "source.png"
    source.write_bytes(encoded)
    client = Client(executable, root, transcript, kind)
    try:
        if kind == "face":
            objects, joints, deformers, clip = scene_recipe()
            operations = [{"op": "create", "object": o} for o in objects] + [{"op": "set_joints", "joints": joints}]
            operations += [{"op": "bind_surface", "request": {"deformer": d}} for d in deformers]
            operations += [{"op": "put_clip", "clip": clip}, {"op": "set_settings", "settings": {"width": 192, "height": 192, "quality": "full", "spatial_aa": 2, "shadows": False, "ao": False}},
                           {"op": "set_camera", "camera": {"eye": [0, 1.42, 1.9], "target": [0, 1.42, .04], "fov_degrees": 29}}]
            client.apply(operations)
            times, clip_id = [0, .4, .8], "expression"
        else:
            client.apply(source_operations())
            requested = recipe()
            prepare(client, requested, True)
            client.apply([{"op": "create_pattern_cloth", "request": requested}])
            client.call({"op": "bake_cloth", "request": {"id": "garment", "clip": "pose"}}, True)
            times, clip_id = [0, .15, .3], "pose"
        before = client.call({"op": "get_document"})
        before_cache = copy.deepcopy(before.get("cloths", []))
        base = client.call({"op": "render", "path": "untextured.png", "animation": {"clip": clip_id, "time": 0}})
        revision = client.revision
        dry = client.call({"op": "import_texture", "request": {"id": "print", "path": "source.png", "color_space": "srgb"}, "dry_run": True}, True)
        assert dry["committed"] is False and client.revision == revision
        assert client.call({"op": "get_document"}) == before
        imported = client.call({"op": "import_texture", "request": {"id": "print", "path": "source.png", "color_space": "srgb"}}, True)
        assert imported["source_sha256"] == hashlib.sha256(encoded).hexdigest()
        definitions = attach(client, before["objects"])
        authored = client.call({"op": "get_document"})
        for key in before:
            assert before[key] == authored[key], f"texture authoring changed pre-existing {key}"
        image = client.call({"op": "texture_state", "request": {"id": "print", "uv": [[.123, .87]], "lod": 100}})
        expected_mean = [sum(linear(p)[i] for p in pixels)/len(pixels) for i in range(4)]
        assert max(abs(a-b) for a, b in zip(expected_mean, image["samples"][0]["premultiplied_linear_rgba"])) < 2e-6
        frames, observations = {}, {}
        for i, seconds in enumerate(times):
            animation = {"clip": clip_id, "time": seconds}
            observations[str(seconds)] = probes(client, authored, definitions, pixels, animation, kind == "cloth")
            frames[str(seconds)] = client.call({"op": "render", "path": f"textured-{i}.png", "animation": animation})
        assert base["metrics"]["visible_material_owner_pixels"] == frames["0"]["metrics"]["visible_material_owner_pixels"]
        change = pixel_difference(png_pixels(root / "untextured.png")[0], png_pixels(root / "textured-0.png")[0])
        assert change > 250, change
        assert pixel_difference(png_pixels(root / "textured-0.png")[0], png_pixels(root / "textured-1.png")[0]) > 250
        if kind == "face":
            assert (root / "textured-0.png").read_bytes() == (root / "textured-2.png").read_bytes()
        else:
            assert client.call({"op": "cloth_state", "id": "garment"})["cache"]["fresh"] is True
            assert client.call({"op": "get_document"})["cloths"] == before_cache
            client.call({"op": "bake_cloth", "request": {"id": "garment", "clip": "pose"}}, True)
            assert client.call({"op": "get_document"})["cloths"] == before_cache
        retained = client.call({"op": "get_document"})
        identifier = next(iter(definitions))
        shape = copy.deepcopy(next(o for o in retained["objects"] if o["id"] == identifier)["shape"])
        shape["triangles"] = list(reversed(shape["triangles"]))
        errors = []
        for operations in [[{"op": "remove_texture", "id": "print"}], [{"op": "remove_uvs", "id": definitions[identifier][0]["id"]}],
                           [{"op": "update", "id": identifier, "patch": {"shape": shape}}]]:
            revision = client.revision
            errors.append(client.apply(operations, expect_ok=False))
            assert client.revision == revision and client.call({"op": "get_document"}) == retained
        rejected = client.call({"op": "export_usd", "request": {"path": "unsupported.usda", "object_ids": [identifier],
            "bounds_min": [-1, 0, -1], "bounds_max": [1, 2, 1], "resolution": [16, 16, 16], "max_surface_error_m": .05, "max_field_residual": .02}}, expect_ok=False)
        assert "textur" in rejected["error"]["message"] and not (root / "unsupported.usda").exists()
        uv = copy.deepcopy(definitions[identifier][0]);uv["values"] = [[u+.17, v] for u, v in uv["values"]]
        client.apply([{"op": "put_uvs", "request": uv}])
        modified = client.call({"op": "get_document"})
        expected = copy.deepcopy(retained)
        expected["uv_sets"] = modified["uv_sets"]
        assert modified == expected
        if kind == "cloth":
            assert client.call({"op": "cloth_state", "id": "garment"})["cache"]["fresh"] is True
        client.call({"op": "undo"}, True)
        assert client.call({"op": "get_document"}) == retained
        client.call({"op": "save", "path": "textured.json"})
        source.unlink()
        client.close();client = Client(executable, root, transcript, kind+"-cold")
        client.call({"op": "load", "path": "textured.json"}, True)
        assert client.call({"op": "get_document"}) == retained
        client.call({"op": "export_texture", "request": {"id": "print", "path": "original-recovered.png"}})
        assert (root / "original-recovered.png").read_bytes() == encoded
        for i, seconds in enumerate(times):
            client.call({"op": "render", "path": f"reopened-{i}.png", "animation": {"clip": clip_id, "time": seconds}})
            assert (root / f"reopened-{i}.png").read_bytes() == (root / f"textured-{i}.png").read_bytes()
        return {"changed_pixels_from_texture": change, "observations": observations, "frames": frames,
                "source_geometry_and_motion_preserved": True, "source_png_and_all_frames_reload_exactly": True,
                "invalid_edits": errors, "textured_usd_explicitly_rejected_without_output": True,
                "physical_cache_preserved_by_texture_and_uv_edits": kind == "cloth"}
    finally:
        client.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--editor", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    root = args.output.resolve();root.mkdir(parents=True, exist_ok=False)
    executable = root / ("mm3e-editor.exe" if args.editor.suffix == ".exe" else "mm3e-editor")
    shutil.copyfile(args.editor.resolve(strict=True), executable);executable.chmod(0o755)
    encoded, pixels = pattern()
    report = {"status": "running", "binary_sha256": sha(executable), "harness_sha256": sha(Path(__file__)), "cases": {},
              "scope": "Attached albedo textures on actual continuous facial morphs and sewn cloth; no artistic approval, normal/displacement maps, anisotropic filtering or textured USD transfer claim."}
    started = time.monotonic()
    try:
        with (root / "transcript.jsonl").open("w") as transcript:
            for kind in ["face", "cloth"]:
                report["cases"][kind] = run_case(executable, root / kind, transcript, kind, encoded, pixels)
                write_json(root / "acceptance.json", report)
        assert sha(executable) == report["binary_sha256"]
        report["status"] = "passed"
    except Exception as error:
        report.update({"status": "failed", "error": str(error), "traceback": traceback.format_exc()})
        raise
    finally:
        report["elapsed_seconds"] = time.monotonic()-started
        write_json(root / "acceptance.json", report)
    print(json.dumps({"status": report["status"], "elapsed_seconds": report["elapsed_seconds"]}, indent=2))


if __name__ == "__main__":
    main()
