#!/usr/bin/env python3
"""Real persistent JSONL IK acceptance with a continuous skinned limb and native pixels.

Preserves all requests, responses, source projects, rendered images and failures in a
NEW output directory. Checks native joint pivots and actual triangle/capsule fields,
not only the solve command's diagnostics. No anatomical or general IK claim is made.
"""
import argparse
import copy
import hashlib
import json
import math
from pathlib import Path
import time
import traceback

from agent_skinning_acceptance import fixture as cylinder_fixture
from agent_usd_acceptance import Client, write_json, sha256, add, sub, scale, length, dot, cross, close
from agent_sewing_acceptance import png_pixels, pixel_difference

ROOT = Path(__file__).resolve().parents[1]
TIME = 0.5
TIP_ANGLES = [25, -20, 35]


def sample():
    return {"clip": "reach", "time": TIME}


def fixture():
    original, points, triangles, _ = cylinder_fixture()
    ring_x = [0.0, 0.3, 0.6, 0.85, 1.1]
    vertices = [[ring_x[i // 16 if i < 80 else (0 if i == 80 else 4)], p[0] * 0.55, p[2] * 0.55]
                for i, p in enumerate(points)]
    asset = copy.deepcopy(original[2]["request"]["deformer"])
    asset.update({"id": "arm-skin", "object": "limb", "joints": ["shoulder", "elbow"]})
    asset["blendshapes"][0]["deltas"] = [[0, d[0] * 0.55, d[2] * 0.55]
                                        for d in asset["blendshapes"][0]["deltas"]]
    def track(joint, start, end, easing="linear"):
        return {"target": {"type": "joint", "id": joint}, "easing": easing,
                "keys": [{"time": 0, **start}, {"time": 1, **end}]}
    base = {"translation": [0.1, 0.05, 0.02], "rotation_degrees": [0, 15, 0]}
    shoulder_a = {"translation": [0.02, -0.01, 0.01], "rotation_degrees": [0, 0, 12]}
    shoulder_b = {"translation": [0.02, -0.01, 0.01], "rotation_degrees": [0, 0, 20]}
    clip = {"id": "reach", "duration": 1, "tracks": [
        track("base", base, base), track("shoulder", shoulder_a, shoulder_b, "smooth_step"),
        track("elbow", {"rotation_degrees": [0, 0, -5]}, {"rotation_degrees": [0, 0, 5]}),
        track("wrist", {}, {"rotation_degrees": [0, 0, 20]}),
        {"target": {"type": "object", "id": "reference"}, "keys": [
            {"time": 0}, {"time": 1, "translation": [0, 0.04, 0]}]}],
        "morph_tracks": [{"deformer": "arm-skin", "blendshape": "bulge",
                          "keys": [{"time": 0, "weight": 0.2}, {"time": 1, "weight": 0.6}]}]}
    return [
        {"op": "create", "object": {"id": "limb", "position": [0, 1, 0],
            "shape": {"type": "surface", "vertices": vertices, "triangles": triangles, "thickness_m": 0.025},
            "material": {"albedo": [0.36, 0.57, 0.25], "roughness": 0.5}}},
        {"op": "create", "object": {"id": "hand", "position": [1.1, 1, 0],
            "shape": {"type": "capsule", "a": [0, 0, 0], "b": [0.13, 0, 0], "radius": 0.025},
            "material": {"albedo": [0.85, 0.43, 0.15], "roughness": 0.4}}},
        {"op": "create", "object": {"id": "reference", "position": [-0.1, 1, 0],
            "shape": {"type": "sphere", "radius": 0.025}, "material": {"albedo": [0.2, 0.35, 0.7]}}},
        {"op": "set_joints", "joints": [
            {"id": "base", "pivot": [0, 0, 0]},
            {"id": "shoulder", "parent": "base", "pivot": [0, 1, 0]},
            {"id": "elbow", "parent": "shoulder", "pivot": [0.6, 1, 0]},
            {"id": "wrist", "parent": "elbow", "pivot": [1.1, 1, 0], "objects": ["hand"]}]},
        {"op": "bind_surface", "request": {"deformer": asset}},
        {"op": "put_clip", "clip": clip},
        {"op": "set_camera", "camera": {"eye": [2.1, 1.65, 3], "target": [0.55, 1.2, 0.1], "fov_degrees": 30}},
        {"op": "set_settings", "settings": {"width": 320, "height": 260, "quality": "balanced",
            "spatial_aa": 2, "shadows": False, "ao": False}},
    ]


def state(client):
    pose = client.call({"op": "pose", "animation": sample()})
    mesh = client.call({"op": "deformer_state", "id": "arm-skin", "animation": sample()})
    return pose, mesh


def joints(pose):
    return {joint["id"]: joint for joint in pose["joints"]}


def render(client, name):
    reply = client.call({"op": "render", "path": name + ".png", "animation": sample()})
    assert reply["metrics"]["visible_material_owner_pixels"].get("limb", 0) > 300
    return reply


def snapshot(client, root):
    inspect = client.call({"op": "inspect"})
    return {"document": client.call({"op": "get_document"}), "project_sha256": sha256(root / "project.json"),
            "revision": client.revision, "undo": inspect["undo_available"], "redo": inspect["redo_available"]}


def unchanged(client, root, before):
    assert snapshot(client, root) == before


def basis_point(basis, point):
    return [sum(basis[column][axis] * point[column] for column in range(3)) for axis in range(3)]


def euler_point(point, angles):
    x, y, z = point
    rx, ry, rz = map(math.radians, angles)
    x, y = math.cos(rz) * x - math.sin(rz) * y, math.sin(rz) * x + math.cos(rz) * y
    x, z = math.cos(ry) * x + math.sin(ry) * z, -math.sin(ry) * x + math.cos(ry) * z
    y, z = math.cos(rx) * y - math.sin(rx) * z, math.sin(rx) * y + math.cos(rx) * z
    return [x, y, z]


def verify_pose(client, authored, request, solve, pose, mesh, initial_root, lengths):
    mapped = joints(pose)
    a, b, c = [mapped[name]["world_pivot"] for name in ["shoulder", "elbow", "wrist"]]
    close(a, initial_root, 2e-6)
    close(c, solve["effective_target"], request["tolerance_m"])
    achieved_lengths = [length(sub(b, a)), length(sub(c, b))]
    close(achieved_lengths, lengths, 2e-6)
    close(c, solve["achieved_tip"], 2e-6)
    close(length(sub(c, solve["effective_target"])), solve["target_error_m"], 2e-6)
    for column in range(3):
        close(mapped["wrist"]["basis"][column], euler_point([int(i == column) for i in range(3)], TIP_ANGLES), 2e-5)
    close(mesh["vertices"][80], a, 2e-6)
    close(mesh["vertices"][81], c, 2e-6)
    source = authored["objects"][0]
    asset = authored["deformers"][0]
    assert mesh["triangles"] == source["shape"]["triangles"]
    assert len(mesh["vertices"]) == 82
    rest_pivots = {joint["id"]: joint["pivot"] for joint in authored["joints"]}
    max_vertex_error = 0.0
    mixed_different = 0
    weight = mesh["morph_weights"][0]["weight"]
    for index, (point, delta, row) in enumerate(zip(source["shape"]["vertices"], asset["blendshapes"][0]["deltas"], asset["weights"])):
        rest_world = add(add(point, scale(delta, weight)), source["position"])
        rigid = []
        for influence in row:
            joint = asset["joints"][influence["joint"]]
            rigid.append(add(mapped[joint]["world_pivot"], basis_point(mapped[joint]["basis"], sub(rest_world, rest_pivots[joint]))))
        if len(row) == 1:
            error = length(sub(mesh["vertices"][index], rigid[0]))
            assert error < 2e-6, (index, error)
            max_vertex_error = max(max_vertex_error, error)
        elif all(length(sub(mesh["vertices"][index], alternate)) > 1e-4 for alternate in rigid):
            mixed_different += 1
    ring = mesh["vertices"][32:48]
    center = [sum(point[axis] for point in ring) / 16 for axis in range(3)]
    close(center, b, 2e-6)
    radii = [length(sub(point, b)) for point in ring]
    close(radii, [0.044 * (1 + 0.25 * weight)] * 16, 2e-6)
    hand_end = add(c, euler_point([0.13, 0, 0], TIP_ANGLES))
    for identifier, points, expected in [("limb", [a, c], -0.0125), ("hand", [c, hand_end], -0.025)]:
        values = client.call({"op": "sample", "id": identifier, "points": points, "animation": sample()})["samples"]
        for value in values:
            close(value["value"], expected, 2e-6)
            assert value["material_owner_id"] == identifier
    return {"native_tip_error_m": length(sub(c, solve["effective_target"])),
            "native_segment_lengths_m": achieved_lengths, "rigid_vertex_max_error_m": max_vertex_error,
            "mixed_vertices_different_from_both_rigid_bones": mixed_different,
            "middle_ring_radius_min_m": min(radii), "middle_ring_radius_max_m": max(radii)}


def preserve_sources(authored, changed):
    for key in ["objects", "joints", "deformers", "camera", "settings"]:
        assert authored[key] == changed[key], key
    old, new = authored["clips"][0], changed["clips"][0]
    assert old["morph_tracks"] == new["morph_tracks"]
    by_target = lambda clip: {json.dumps(track["target"], sort_keys=True): track for track in clip["tracks"]}
    new_tracks = by_target(new)
    for original in old["tracks"]:
        changed_track = new_tracks[json.dumps(original["target"], sort_keys=True)]
        if original["target"]["id"] in {"base", "reference"}:
            assert changed_track == original
        else:
            assert changed_track["easing"] == original["easing"]
            assert [key for key in changed_track["keys"] if key["time"] != TIME] == original["keys"]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--editor", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    executable = args.editor.resolve(strict=True)
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    started = time.monotonic()
    report = {"status": "running", "passed": False, "binary": str(executable), "binary_sha256": sha256(executable),
              "harness_sha256": sha256(Path(__file__)), "native_geometry": "82-vertex continuous DQS limb with dense morph and rigid hand",
              "limits": ["three-joint direct-chain authoring only", "no anatomical limits or self-collision certification",
                         "related cloth is a fully pinned cache-invalidation fixture"]}
    report_path = root / "acceptance.json"
    write_json(report_path, report)
    client = None
    try:
        with (root / "transcript.jsonl").open("w") as transcript:
            client = Client(executable, root, transcript, "author")
            capabilities = client.call({"op": "describe"})
            assert "solve_ik" in json.dumps(capabilities["request_schema"])
            write_json(root / "capabilities.json", capabilities)
            client.apply(fixture())
            authored = client.call({"op": "get_document"})
            write_json(root / "source-document.json", authored)
            assert all("limb" not in joint["objects"] for joint in authored["joints"])
            baseline_pose, baseline_mesh = state(client)
            original_joints = joints(baseline_pose)
            a, b, c = [original_joints[name]["world_pivot"] for name in ["shoulder", "elbow", "wrist"]]
            lengths = [length(sub(b, a)), length(sub(c, b))]
            request = {"clip": "reach", "time": TIME, "root_joint": "shoulder", "middle_joint": "elbow",
                       "tip_joint": "wrist", "target": add(a, [0.55, 0.4, 0.2]), "pole": add(a, [0, 0, 1]),
                       "tip_world_rotation_degrees": TIP_ANGLES, "tolerance_m": 0.0001}
            before_target = client.call({"op": "sample", "id": "hand", "points": [request["target"]], "animation": sample()})
            assert before_target["samples"][0]["value"] > 0.1
            report["renders"] = {"before": render(client, "before")}
            before = snapshot(client, root)
            missing_revision = client.call({"op": "solve_ik", "request": request, "dry_run": True}, ok=False)
            unchanged(client, root, before)
            preview = client.call({"op": "solve_ik", "request": request, "dry_run": True}, mutation=True)
            assert preview["committed"] is False
            unchanged(client, root, before)
            assert state(client) == (baseline_pose, baseline_mesh)
            unreachable = {**request, "target": add(a, [5, 5, 5])}
            rejected = client.call({"op": "solve_ik", "request": unreachable}, mutation=True, ok=False)
            unchanged(client, root, before)
            report["atomic_controls"] = {"missing_revision": missing_revision["error"], "unreachable_reject": rejected["error"],
                "dry_run_kept_project_revision_history_pose_and_mesh": True, "rejection_kept_project_revision_and_history": True}
            write_json(report_path, report)
            solved = client.call({"op": "solve_ik", "request": request}, mutation=True)
            assert solved["committed"] and not solved["clamped"]
            positive_pose, positive_mesh = state(client)
            positive_document = client.call({"op": "get_document"})
            preserve_sources(authored, positive_document)
            positive_checks = verify_pose(client, authored, request, solved, positive_pose, positive_mesh, a, lengths)
            assert positive_checks["mixed_vertices_different_from_both_rigid_bones"] >= 12
            assert max(length(sub(p, q)) for p, q in zip(positive_mesh["vertices"], baseline_mesh["vertices"])) > 0.3
            report["renders"]["reach-positive"] = render(client, "reach-positive")
            opposite = {**request, "pole": add(a, [0, 0, -1])}
            flipped = client.call({"op": "solve_ik", "request": opposite}, mutation=True)
            negative_pose, negative_mesh = state(client)
            negative_document = client.call({"op": "get_document"})
            preserve_sources(authored, negative_document)
            negative_checks = verify_pose(client, authored, opposite, flipped, negative_pose, negative_mesh, a, lengths)
            positive_elbow = joints(positive_pose)["elbow"]["world_pivot"]
            negative_elbow = joints(negative_pose)["elbow"]["world_pivot"]
            assert length(sub(positive_elbow, negative_elbow)) > 0.5
            direction = scale(sub(request["target"], a), 1 / length(sub(request["target"], a)))
            bend = sub([0, 0, 1], scale(direction, direction[2]))
            assert dot(sub(positive_elbow, a), bend) > 0.1 and dot(sub(negative_elbow, a), bend) < -0.1
            report["renders"]["reach-negative"] = render(client, "reach-negative")
            pixels_before, _ = png_pixels(root / "before.png")
            pixels_positive, _ = png_pixels(root / "reach-positive.png")
            pixels_negative, _ = png_pixels(root / "reach-negative.png")
            change = pixel_difference(pixels_positive, pixels_negative)
            assert change > 500 and pixel_difference(pixels_before, pixels_positive) > 500
            client.call({"op": "undo"}, mutation=True)
            assert client.call({"op": "get_document"}) == positive_document
            assert state(client) == (positive_pose, positive_mesh)
            report["renders"]["undo"] = render(client, "undo")
            assert (root / "undo.png").read_bytes() == (root / "reach-positive.png").read_bytes()
            client.call({"op": "redo"}, mutation=True)
            assert client.call({"op": "get_document"}) == negative_document
            assert state(client) == (negative_pose, negative_mesh)
            report["renders"]["redo"] = render(client, "redo")
            assert (root / "redo.png").read_bytes() == (root / "reach-negative.png").read_bytes()
            clamped_request = {**unreachable, "unreachable": "clamp"}
            clamped = client.call({"op": "solve_ik", "request": clamped_request}, mutation=True)
            assert clamped["clamped"] and clamped["requested_target_error_m"] > 5
            clamp_pose, clamp_mesh = state(client)
            clamp_checks = verify_pose(client, authored, clamped_request, clamped, clamp_pose, clamp_mesh, a, lengths)
            close(length(sub(joints(clamp_pose)["wrist"]["world_pivot"], a)), sum(lengths), 2e-6)
            client.call({"op": "undo"}, mutation=True)
            assert client.call({"op": "get_document"}) == negative_document
            source_hash = sha256(root / "project.json")
            client.close()
            client = Client(executable, root, transcript, "reopen")
            assert sha256(root / "project.json") == source_hash
            assert client.call({"op": "get_document"}) == negative_document
            assert state(client) == (negative_pose, negative_mesh)
            report["renders"]["reopened"] = render(client, "reopened")
            assert (root / "reopened.png").read_bytes() == (root / "reach-negative.png").read_bytes()
            report.update({"positive_solve": solved, "negative_solve": flipped, "clamp_solve": clamped,
                "positive_checks": positive_checks, "negative_checks": negative_checks, "clamp_checks": clamp_checks,
                "changed_pixels_for_opposite_poles": change, "cold_reload_project_pose_mesh_and_png_exact": True,
                "source_geometry_weights_morphs_and_unrelated_keys_preserved": True})
            write_json(root / "solved-poses.json", {"before": baseline_pose, "positive": positive_pose, "negative": negative_pose})
            write_json(report_path, report)
            # Establish a related, fully pinned cloth cache, then edit its driving IK.
            start_pose = client.call({"op": "pose", "animation": {"clip": "reach", "time": 0}})
            start_wrist = joints(start_pose)["wrist"]
            panel_origin = add(start_wrist["world_pivot"], basis_point(start_wrist["basis"], [0, 0, 0.12]))
            pins = [{"vertex": i, "target_object": "hand", "point": [x, y, 0.12]}
                    for i, (x, y) in enumerate([(-0.03, -0.04), (0.03, -0.04), (-0.03, 0.04), (0.03, 0.04)])]
            client.apply([{"op": "create_cloth_panel", "request": {"id": "cuff-cloth", "origin": panel_origin,
                "axis_u": start_wrist["basis"][0], "axis_v": start_wrist["basis"][1], "segments": [1, 1], "width_m": 0.06, "height_m": 0.08,
                "thickness_m": 0.002, "vertex_mass_kg": 0.02, "pins": pins,
                "settings": {"fixed_dt": 1 / 30, "substeps": 2, "iterations": 8, "gravity": [0, 0, 0]}}}])
            client.call({"op": "bake_cloth", "request": {"id": "cuff-cloth", "clip": "reach"}}, mutation=True)
            fresh = client.call({"op": "cloth_state", "id": "cuff-cloth", "animation": sample()})
            assert fresh["cache"]["fresh"] is True
            edited = client.call({"op": "solve_ik", "request": request}, mutation=True)
            assert edited["committed"] and edited["stale_cloth_caches"] == ["cuff-cloth"]
            stale = client.call({"op": "cloth_state", "id": "cuff-cloth"})
            assert stale["cache"]["fresh"] is False
            stale_sample = client.call({"op": "cloth_state", "id": "cuff-cloth", "animation": sample()}, ok=False)
            failed_render = client.call({"op": "render", "path": "stale-must-not-exist.png", "animation": sample()}, ok=False)
            assert not (root / "stale-must-not-exist.png").exists()
            client.call({"op": "bake_cloth", "request": {"id": "cuff-cloth", "clip": "reach"}}, mutation=True)
            rebaked = client.call({"op": "cloth_state", "id": "cuff-cloth", "animation": sample()})
            assert rebaked["cache"]["fresh"] and rebaked["sample_within_bake_tolerances"]
            assert rebaked["cache"]["source_fnv1a64"] != fresh["cache"]["source_fnv1a64"]
            report["renders"]["rebaked"] = render(client, "rebaked")
            final_document = client.call({"op": "get_document"})
            final_state = state(client)
            final_hash = sha256(root / "project.json")
            client.close()
            client = Client(executable, root, transcript, "final-reopen")
            assert sha256(root / "project.json") == final_hash
            assert client.call({"op": "get_document"}) == final_document
            assert state(client) == final_state
            report["renders"]["final-reopened"] = render(client, "final-reopened")
            assert (root / "final-reopened.png").read_bytes() == (root / "rebaked.png").read_bytes()
            report["related_cloth_cache"] = {"reported_stale_assets": edited["stale_cloth_caches"],
                "cloth_state_marked_stale": True, "animated_stale_state_error": stale_sample["error"],
                "stale_render_error": failed_render["error"], "stale_render_created_no_file": True,
                "rebake_fresh_and_within_tolerances": True, "final_cold_reload_including_cache_and_png_exact": True}
            client.close()
            client = None
        assert sha256(executable) == report["binary_sha256"]
        report["status"], report["passed"] = "passed", True
        (root / "comparison.html").write_text("""<!doctype html><meta charset="utf-8"><title>Native IK acceptance</title>
<style>body{background:#14202c;color:#edf3f8;font:16px system-ui;margin:2rem}.frames{display:flex;gap:1rem}figure{margin:0}a{color:#9cf}</style>
<h1>Target-based native IK</h1><p>Actual fixed-camera engine renders of one continuous skinned limb. Both pole choices reach the same wrist target.</p>
<div class="frames"><figure><img src="before.png"><figcaption>Before solve</figcaption></figure>
<figure><img src="reach-positive.png"><figcaption>Positive pole</figcaption></figure>
<figure><img src="reach-negative.png"><figcaption>Opposite pole, same target</figcaption></figure></div>
<p>No anatomical joint-limit or self-collision claim. Related cloth-cache invalidation and explicit rebaking are checked.</p>
<p><a href="acceptance.json">Verification</a> · <a href="project.json">Durable native project</a></p>""")
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
    print(json.dumps({key: report[key] for key in ["status", "elapsed_seconds", "changed_pixels_for_opposite_poles"]}, indent=2))


if __name__ == "__main__":
    main()
