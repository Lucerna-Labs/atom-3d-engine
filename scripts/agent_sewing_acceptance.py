#!/usr/bin/env python3
"""Actual JSONL sewn-panel dynamics, visible PNG delivery and cold-project acceptance.

Uses only Python's standard library. The output directory must be new; all
requests/replies, project files, rendered frames, failures and numeric evidence
are retained. This fixture verifies rectangular sewn panels, not arbitrary CAD
patterns or a measured fabric model.
"""
import argparse
import copy
import hashlib
import json
import math
from pathlib import Path
import struct
import sys
import time
import traceback
import zlib

from agent_usd_acceptance import Client, write_json, sha256

ROOT = Path(__file__).resolve().parents[1]


def garment(sewn=True):
    return {"id": "garment", "label": "Two-piece hanging garment", "group": "wardrobe",
            "panels": [
                {"id": "left", "origin": [-0.2, 1, 0], "axis_u": [1, 0, 0], "axis_v": [0, 1, 0],
                 "segments": [2, 3], "width_m": 0.4, "height_m": 0.6,
                 "pins": [{"vertex": 9, "target_object": "anchor", "point": [-0.4, 0, 0]},
                          {"vertex": 11, "target_object": "anchor", "point": [0, 0, 0]}]},
                {"id": "right", "origin": [0.2, 1, 0], "axis_u": [1, 0, 0], "axis_v": [0, 1, 0],
                 "segments": [2, 3], "width_m": 0.4, "height_m": 0.6}],
            "seams": [{"panel_a": "left", "chain_a": [2, 5, 8, 11], "panel_b": "right",
                       "chain_b": [0, 3, 6, 9], "rest_length_m": 0, "compliance": 0}] if sewn else [],
            "thickness_m": 0.002, "vertex_mass_kg": 0.02,
            "settings": {"iterations": 48, "substeps": 4, "bend_compliance": 0.02, "max_seam_error_m": 0.001},
            "material": {"albedo": [0.6, 0.1, 0.08], "roughness": 0.75}}


def sources():
    return [
        {"op": "create", "object": {"id": "anchor", "shape": {"type": "sphere", "radius": 0.03}, "position": [0, 1.3, 0]}},
        {"op": "set_joints", "joints": [{"id": "shoulder", "pivot": [0, 1.3, 0], "objects": []},
                                         {"id": "attachment", "parent": "shoulder", "pivot": [0, 1.3, 0], "objects": ["anchor"]}]},
        {"op": "put_clip", "clip": {"id": "hang", "duration": 0.5, "tracks": [
            {"target": {"type": "joint", "id": "shoulder"}, "keys": [{"time": 0}, {"time": 0.5, "rotation_degrees": [0, 0, 10]}]},
            {"target": {"type": "joint", "id": "attachment"}, "keys": [{"time": 0}, {"time": 0.5, "translation": [0.02, 0.02, 0]}]}]}},
        {"op": "set_camera", "camera": {"eye": [0.9, 1.35, 2.8], "target": [0, 0.7, 0], "up": [0, 1, 0], "fov_degrees": 42}},
        {"op": "set_settings", "settings": {"width": 160, "height": 160, "quality": "full", "spatial_aa": 2, "shadows": False, "ao": False}},
    ]


def sample(seconds):
    return {"clip": "hang", "time": seconds}


def seam_gap(value):
    return max(math.dist(value["vertices"][2 + row * 3], value["vertices"][12 + row * 3]) for row in range(4))


def png_pixels(path):
    """Independent CRC-checked RGBA8 decoder, supporting all five PNG scanline filters."""
    data = path.read_bytes()
    assert data[:8] == b"\x89PNG\r\n\x1a\n"
    offset, compressed, dimensions = 8, bytearray(), None
    while offset < len(data):
        size = struct.unpack_from(">I", data, offset)[0]
        kind = data[offset + 4:offset + 8]
        chunk = data[offset + 8:offset + 8 + size]
        checksum = struct.unpack_from(">I", data, offset + 8 + size)[0]
        assert zlib.crc32(kind + chunk) & 0xffffffff == checksum
        offset += size + 12
        if kind == b"IHDR":
            w, h, depth, color, compression, filtering, interlace = struct.unpack(">IIBBBBB", chunk)
            assert (depth, color, compression, filtering, interlace) == (8, 6, 0, 0, 0)
            dimensions = (w, h)
        elif kind == b"IDAT":
            compressed.extend(chunk)
        elif kind == b"IEND":
            break
    assert offset == len(data) and dimensions
    width, height = dimensions
    raw, stride = zlib.decompress(compressed), width * 4
    assert len(raw) == height * (stride + 1)
    rows, previous = [], bytearray(stride)
    for row in range(height):
        offset = row * (stride + 1)
        filter_kind = raw[offset]
        assert 0 <= filter_kind <= 4
        values = bytearray(raw[offset + 1:offset + stride + 1])
        for column in range(stride):
            a = values[column - 4] if column >= 4 else 0
            b = previous[column]
            c = previous[column - 4] if column >= 4 else 0
            p = a + b - c
            pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
            paeth = a if pa <= pb and pa <= pc else b if pb <= pc else c
            predictor = (0, a, b, (a + b) // 2, paeth)[filter_kind]
            values[column] = (values[column] + predictor) & 255
        rows.append(values)
        previous = values
    pixels = b"".join(rows)
    red = sum(pixels[i] > 25 and pixels[i] > 1.4 * pixels[i + 1] and pixels[i] > 1.4 * pixels[i + 2]
              for i in range(0, len(pixels), 4))
    return pixels, {"width": width, "height": height, "red_garment_pixels": red, "sha256": sha256(path)}


def pixel_difference(a, b):
    assert len(a) == len(b)
    return sum(a[i:i + 4] != b[i:i + 4] for i in range(0, len(a), 4))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--executable", "--binary", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    executable = args.executable.resolve(strict=True)
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    report = {"status": "running", "passed": False, "binary": str(executable), "binary_sha256": sha256(executable),
              "harness_sha256": sha256(Path(__file__)), "cases": [], "rejections": []}
    report_path = root / "acceptance.json"
    write_json(report_path, report)
    started, client, decoded = time.monotonic(), None, {}
    try:
        with (root / "transcript.jsonl").open("w") as transcript:
            for name, sewn in [("sewn", True), ("no-seam-control", False)]:
                case_root = root / name
                case_root.mkdir()
                client = Client(executable, case_root, transcript, name + "-author")
                recipe = garment(sewn)
                write_json(case_root / "requested-garment.json", recipe)
                client.apply(sources() + [{"op": "create_sewn_cloth", "request": recipe}])
                authored = client.call({"op": "get_document"})
                baked = client.call({"op": "bake_cloth", "request": {"id": "garment", "clip": "hang"}}, mutation=True)
                assert client.call({"op": "get_document"})["objects"] == authored["objects"]
                case = {"case": name, "bake": baked, "frames": []}
                report["cases"].append(case)
                client.call({"op": "render", "path": "rest.png"})
                rest_pixels, rest_metrics = png_pixels(case_root / "rest.png")
                assert rest_metrics["red_garment_pixels"] > 200
                case["rest_image"] = rest_metrics
                states = {}
                for seconds, filename in [(0, "frame-000.png"), (0.25, "frame-015.png"), (0.5, "frame-030.png")]:
                    state = client.call({"op": "cloth_state", "id": "garment", "animation": sample(seconds)})
                    states[seconds] = state
                    assert state["sample_within_bake_tolerances"]
                    pose = client.call({"op": "pose", "animation": sample(seconds)})
                    anchor = next(o for o in pose["objects"] if o["id"] == "anchor")
                    for index, local in [(9, [-0.4, 0, 0]), (11, [0, 0, 0])]:
                        wanted = [anchor["position"][axis] + anchor["scale"] * sum(anchor["basis"][j][axis] * local[j] for j in range(3)) for axis in range(3)]
                        assert math.dist(state["vertices"][index], wanted) < 1e-6
                    gap = seam_gap(state)
                    if sewn:
                        assert gap <= 0.001
                    probe = client.call({"op": "sample", "id": "garment", "animation": sample(seconds), "points": state["vertices"]})
                    assert all(point["value"] < 0 for point in probe["samples"]), "evaluated cloth did not reach native geometry"
                    client.call({"op": "render", "path": filename, "animation": sample(seconds)})
                    pixels, metrics = png_pixels(case_root / filename)
                    assert metrics["red_garment_pixels"] > 100
                    decoded[(name, seconds)] = pixels
                    case["frames"].append({"seconds": seconds, "seam_pair_gap_m": gap,
                                           "actual_max_seam_length_error_m": state["sample_max_seam_length_error_m"],
                                           "image": metrics, "changed_pixels_from_rest": pixel_difference(pixels, rest_pixels)})
                assert case["frames"][-1]["changed_pixels_from_rest"] > 200
                if sewn:
                    assert baked["diagnostics"]["seam_projections"] > 0
                    full = client.call({"op": "get_document"})
                    assert full["cloths"][0]["sewing"]["panels"] == authored["cloths"][0]["sewing"]["panels"]
                    assert all(seam_gap(frame) <= 0.001 for frame in full["cloths"][0]["cache"]["frames"])
                else:
                    assert seam_gap(states[0.5]) > 0.3
                saved = client.call({"op": "get_document"})
                saved_revision = client.revision
                client.close()
                client = Client(executable, case_root, transcript, name + "-cold-reload")
                assert client.revision == saved_revision and client.call({"op": "get_document"}) == saved
                assert client.call({"op": "cloth_state", "id": "garment", "animation": sample(0.5)}) == states[0.5]
                client.call({"op": "render", "path": "reopened.png", "animation": sample(0.5)})
                assert (case_root / "reopened.png").read_bytes() == (case_root / "frame-030.png").read_bytes()
                case["cold_reload"] = {"document_and_state_equal": True, "image_bytes_equal": True, "revision": saved_revision}
                if sewn:
                    bad = copy.deepcopy(recipe)
                    bad["seams"][0]["chain_a"] = [1, 4, 7, 10]
                    before_hash = sha256(case_root / "project.json")
                    rejected = client.call({"op": "apply", "operations": [{"op": "update_sewn_cloth", "request": bad}]}, mutation=True, ok=False)
                    assert client.revision == saved_revision and sha256(case_root / "project.json") == before_hash
                    assert client.call({"op": "get_document"}) == saved
                    report["rejections"].append({"error": rejected["error"], "project_document_cache_and_revision_unchanged": True})
                    edited = copy.deepcopy(recipe)
                    edited["seams"][0]["compliance"] = 1e-7
                    client.apply([{"op": "update_sewn_cloth", "request": edited}])
                    assert client.call({"op": "get_document"})["cloths"][0]["cache"] is None
                    rejected = client.call({"op": "cloth_state", "id": "garment", "animation": sample(0.5)}, ok=False)
                    assert "bake_cloth" in rejected["error"]["message"]
                    client.call({"op": "undo"}, mutation=True)
                    assert client.call({"op": "get_document"}) == saved
                    client.call({"op": "redo"}, mutation=True)
                    assert client.call({"op": "get_document"})["cloths"][0]["cache"] is None
                    rebaked = client.call({"op": "bake_cloth", "request": {"id": "garment", "clip": "hang"}}, mutation=True)
                    edited_state = client.call({"op": "cloth_state", "id": "garment", "animation": sample(0.5)})
                    assert rebaked["source_fnv1a64"] != baked["source_fnv1a64"]
                    assert 1e-7 < seam_gap(edited_state) <= 0.001
                    case["requested_edit"] = {"compliance": 1e-7, "new_seam_gap_m": seam_gap(edited_state),
                                               "cache_invalidated_before_rebake": True, "undo_restored_cache": True,
                                               "source_fingerprint_changed": True, "rebake": rebaked}
                client.close()
                client = None
                write_json(report_path, report)
        difference = pixel_difference(decoded[("sewn", 0.5)], decoded[("no-seam-control", 0.5)])
        assert difference > 500
        report.update({"status": "passed", "passed": True, "sewn_vs_no_seam_changed_pixels": difference})
    except Exception:
        report.update({"status": "failed", "passed": False, "failure": traceback.format_exc()})
        raise
    finally:
        if client is not None:
            client.abort()
        report["elapsed_seconds"] = time.monotonic() - started
        write_json(report_path, report)
        print(json.dumps({"status": report["status"], "report": str(report_path), "elapsed_seconds": report["elapsed_seconds"]}))


if __name__ == "__main__":
    main()
