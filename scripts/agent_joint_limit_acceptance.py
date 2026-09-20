#!/usr/bin/env python3
"""Real-process joint constraints on continuous facial skin, cloth and speech curves.

Explicitly authored capped curves provide independent reference poses. Numerical
differences are retained; configured rotation bounds do not certify anatomical
suitability, collision freedom, global IK optimality or general temporal quality.
"""
import argparse
import base64
import copy
import hashlib
import json
import math
from pathlib import Path
import shutil
import time
import traceback

from agent_animation_acceptance import Client, write_json
from agent_sewing_acceptance import png_pixels

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "artifacts/material-maps-normal-filter-20260907-opt-in"
SPEECH = ROOT / "artifacts/speech-acceptance-20260907-final"
VERTEX_TOLERANCE_M = 5e-6
CLOTH_TOLERANCE_M = 5e-5
FIELD_TOLERANCE_M = 1e-5
PIXEL_MEAN_TOLERANCE = 0.05


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def at(clip, seconds):
    return {"clip": clip, "time": seconds}


def put(clip):
    return {"op": "put_clip", "clip": clip}


def limit(axis, maximum, mode="project"):
    return {"type": "hinge", "axis": axis, "min_degrees": 0, "max_degrees": maximum, "mode": mode}


def set_limit(client, joint, value):
    client.apply([{"op": "set_joint_limit", "id": joint, "limit": value}])


def pose(client, clip, seconds):
    return client.call({"op": "pose", "animation": at(clip, seconds)})


def render(client, path, clip, seconds):
    return client.call({"op": "render", "path": path, "animation": at(clip, seconds)})


def distances(a, b):
    assert len(a) == len(b)
    return max((math.dist(x, y) for x, y in zip(a, b)), default=0)


def image_difference(a, b):
    first, first_meta = png_pixels(a)
    second, second_meta = png_pixels(b)
    assert (first_meta["width"], first_meta["height"]) == (second_meta["width"], second_meta["height"])
    assert len(first) == len(second)
    changes = [abs(x-y) for x, y in zip(first, second)]
    result = {"changed_pixels": sum(first[i:i+4] != second[i:i+4] for i in range(0, len(first), 4)),
              "maximum_channel_difference": max(changes, default=0),
              "mean_absolute_channel_difference": sum(changes) / len(changes)}
    assert result["mean_absolute_channel_difference"] <= PIXEL_MEAN_TOLERANCE, result
    return result


def deformer(client, identifier, clip, seconds):
    return client.call({"op": "deformer_state", "id": identifier, "animation": at(clip, seconds)})


def compare_skin(client, reference_client, identifiers, projected, reference, seconds):
    maximum_vertex = maximum_field = 0
    for identifier in identifiers:
        actual = deformer(client, identifier, projected, seconds)
        expected = deformer(reference_client, identifier, reference, seconds)
        maximum_vertex = max(maximum_vertex, distances(actual["vertices"], expected["vertices"]))
        assert actual["morph_weights"] == expected["morph_weights"]
        points = expected["vertices"][::max(1, len(expected["vertices"]) // 30)]
        values = []
        for sampler, clip in ((client, projected), (reference_client, reference)):
            sampled = sampler.call({"op": "sample", "id": actual["object"], "points": points,
                                   "animation": at(clip, seconds)})
            values.append([p["value"] for p in sampled["samples"]])
        maximum_field = max(maximum_field, max(abs(a-b) for a, b in zip(*values)))
    assert maximum_vertex <= VERTEX_TOLERANCE_M, maximum_vertex
    assert maximum_field <= FIELD_TOLERANCE_M, maximum_field
    return {"maximum_vertex_error_m": maximum_vertex, "maximum_field_difference_m": maximum_field}


def replace_joint_curve(source, joint, axis, maximum, capped=False):
    result = copy.deepcopy(source)
    curve = next(t for t in result["tracks"] if t["target"] == {"type": "joint", "id": joint})
    duration = result["duration"]
    curve["easing"] = "linear"
    curve["keys"] = [{"time": 0}, {"time": duration, "rotation_degrees": [maximum*x for x in axis]}]
    if capped:
        curve["keys"] = [{"time": 0}, {"time": duration / 2, "rotation_degrees": [maximum*x for x in axis]},
                         {"time": duration, "rotation_degrees": [maximum*x for x in axis]}]
    return result


def run_face(executable, root, transcript, face_project, speech_project, speech_analysis):
    root.mkdir()
    shutil.copyfile(face_project, root / "input.json")
    client = Client(executable, root, transcript, "limited-face")
    reference_root = root / "manual-reference"
    reference_root.mkdir()
    shutil.copyfile(face_project, reference_root / "input.json")
    reference_client = Client(executable, reference_root, transcript, "unlimited-manual-face")
    try:
        client.call({"op": "load", "path": "input.json"}, True)
        reference_client.call({"op": "load", "path": "input.json"}, True)
        reference_client.apply([{"op": "clear_joint_limit", "id": "face/jaw-joint"}])
        original = client.call({"op": "get_document"})
        source = next(c for c in original["clips"] if c["id"] == "expression")
        raw = replace_joint_curve(source, "face/jaw-joint", [1, 0, 0], 20)
        raw["id"] = "raw-jaw"
        reference = replace_joint_curve(source, "face/jaw-joint", [1, 0, 0], 10, capped=True)
        reference["id"] = "manual-cap"
        client.apply([put(raw), put(reference)])
        reference_client.apply([put(reference)])
        keys = copy.deepcopy(client.call({"op": "get_document"})["clips"])
        set_limit(client, "face/jaw-joint", limit([1, 0, 0], 10))
        identifiers = [d["id"] for d in original["deformers"]]
        times = [source["duration"] * fraction for fraction in (0, .25, .5, .75, 1)]
        comparisons = []
        for index, seconds in enumerate(times):
            observation = client.call({"op": "joint_limit_state", "animation": at(raw["id"], seconds)})
            report = next(j for j in observation["joints"] if j["joint"] == "face/jaw-joint")
            assert "applied" not in report
            measured = compare_skin(client, reference_client, identifiers, raw["id"], reference["id"], seconds)
            render(client, f"projected-{index}.png", raw["id"], seconds)
            render(reference_client, f"manual-{index}.png", reference["id"], seconds)
            measured.update(time=seconds, diagnostic=report,
                            image_difference=image_difference(root / f"projected-{index}.png", reference_root / f"manual-{index}.png"))
            comparisons.append(measured)
        assert comparisons[-1]["diagnostic"]["would_project"]
        assert client.call({"op": "get_document"})["clips"] == keys
        set_limit(client, "face/jaw-joint", limit([1, 0, 0], 10, "reject"))
        revision = client.revision
        before = client.call({"op": "get_document"})
        preview = client.call({"op": "joint_limit_state", "animation": at(raw["id"], times[-1])})
        assert preview["joints"][0]["would_reject"]
        for command in ({"op": "pose", "animation": at(raw["id"], times[-1])},
                        {"op": "render", "path": "rejected.png", "animation": at(raw["id"], times[-1])}):
            rejected = client.call(command, expect_ok=False)
            assert "face/jaw-joint" in rejected["error"]["message"] and "rotation limit" in rejected["error"]["message"]
            assert client.revision == revision and client.call({"op": "get_document"}) == before
        assert not (root / "rejected.png").exists()
        set_limit(client, "face/jaw-joint", limit([1, 0, 0], 10))
        client.call({"op": "render_sequence", "request": {"directory": "job-reference", "clip": raw["id"],
                     "start": 0, "end": times[-1], "fps": 5}})
        job = client.call({"op": "create_render_job", "request": {"type": "sequence", "directory": "job",
                           "clip": raw["id"], "start": 0, "end": times[-1], "fps": 5}})
        client.call({"op": "step_render_job", "directory": "job", "max_frames": 1})
        set_limit(client, "face/jaw-joint", limit([1, 0, 0], 5))
        render(client, "live-limit-5.png", raw["id"], times[-1])
        assert (root / "live-limit-5.png").read_bytes() != (root / "projected-4.png").read_bytes()
        client.close()
        client = Client(executable, root, transcript, "limited-face-job-restart")
        assert client.call({"op": "get_document"})["joints"] == []
        done = client.call({"op": "step_render_job", "directory": "job", "max_frames": 32})
        assert done["status"] == "complete"
        for index in range(job["total_frames"]):
            assert (root / f"job/frame_{index:04}.png").read_bytes() == (root / f"job-reference/frame_{index:04}.png").read_bytes()
        # Reopen the source and attach a real analyzed speech report to the actual
        # jaw skin palette, rather than using the procedural fixture's unbound marker.
        client.call({"op": "load", "path": "input.json"}, True)
        speech_document = json.loads(speech_project.read_text())["document"]
        report = json.loads(speech_analysis.read_text())
        audio = next(a for a in speech_document["audio"] if a["id"] == report["source"]["audio"])
        encoded = base64.b64decode(audio["data"], validate=True)
        assert hashlib.sha256(encoded).hexdigest() == report["source"]["source_sha256"]
        (root / "speech-source.wav").write_bytes(encoded)
        shutil.copyfile(speech_analysis, root / "speech-analysis.json")
        client.call({"op": "import_audio", "request": {"id": audio["id"], "path": "speech-source.wav"}}, True)
        angles = {"A": 0, "B": 8, "C": 20, "D": 30, "E": 16, "F": 6, "G": 4, "H": 10, "X": 0}
        profile = {"poses": [{"shape": shape, "values": [{"type": "joint", "joint": "face/jaw-joint",
                             "rotation_degrees": [angle, 0, 0]}]} for shape, angle in angles.items()]}
        generated = client.call({"op": "generate_lip_sync", "request": {"analysis_path": "speech-analysis.json",
                                "clip": "speech-jaw", "profile": profile, "transition_seconds": .04}}, True)
        cue = next(c for c in report["cues"] if angles[c["shape"]] > 10)
        seconds = cue["start_cs"] / 100
        unconstrained = deformer(client, "face/lips", "speech-jaw", seconds)
        authored = copy.deepcopy(next(c for c in client.call({"op": "get_document"})["clips"] if c["id"] == "speech-jaw"))
        set_limit(client, "face/jaw-joint", limit([1, 0, 0], 10))
        reference_speech = {"id": "speech-manual-cap", "duration": generated["current_duration_seconds"],
                            "tracks": [{"target": {"type": "joint", "id": "face/jaw-joint"},
                                        "keys": [{"time": 0, "rotation_degrees": [10, 0, 0]}]}]}
        reference_client.apply([put(reference_speech)])
        speech_comparison = compare_skin(client, reference_client, identifiers, "speech-jaw", reference_speech["id"], seconds)
        constrained = deformer(client, "face/lips", "speech-jaw", seconds)
        speech_movement = distances(unconstrained["vertices"], constrained["vertices"])
        assert speech_movement > 1e-4
        retained = client.call({"op": "lip_sync_state", "clip": "speech-jaw"})
        assert retained["edited"] is False and retained["provenance"] == generated["provenance"]
        assert next(c for c in client.call({"op": "get_document"})["clips"] if c["id"] == "speech-jaw") == authored
        render(client, "speech-limited.png", "speech-jaw", seconds)
        document = client.call({"op": "get_document"})
        client.call({"op": "save", "path": "limited-face-project.json"})
        client.close()
        client = Client(executable, root, transcript, "limited-face-cold")
        client.call({"op": "load", "path": "limited-face-project.json"}, True)
        assert client.call({"op": "get_document"}) == document
        assert client.call({"op": "lip_sync_state", "clip": "speech-jaw"}) == retained
        assert deformer(client, "face/lips", "speech-jaw", seconds)["vertices"] == constrained["vertices"]
        render(client, "speech-cold.png", "speech-jaw", seconds)
        assert (root / "speech-cold.png").read_bytes() == (root / "speech-limited.png").read_bytes()
        return {"pose_comparisons": comparisons, "source_keys_preserved": True,
                "reject_read_only_and_no_output": True, "frozen_job_frames_exact": job["total_frames"],
                "speech_profile": profile, "speech_cue": cue, "speech_skin_movement_m": speech_movement,
                "speech_reference_comparison": speech_comparison, "speech_provenance_stays_unedited": True,
                "native_cold_reload_exact": True}
    finally:
        client.close()
        reference_client.close()


def cloth_state(client, seconds=None):
    command = {"op": "cloth_state", "id": "garment"}
    if seconds is not None:
        command["animation"] = at("pose", seconds)
    return client.call(command)


def pin_error(state, value):
    objects = {o["id"]: o for o in value["objects"]}
    maximum = 0
    for pin in state["pins"]:
        obj = objects[pin["target_object"]]
        expected = [obj["position"][a] + sum(obj["basis"][i][a] * pin["point"][i] * obj["scale"] for i in range(3)) for a in range(3)]
        maximum = max(maximum, math.dist(expected, state["vertices"][pin["vertex"]]))
    return maximum


def run_cloth(executable, root, transcript, source_project):
    root.mkdir()
    clients = []
    try:
        for kind in ("projected", "manual"):
            directory = root / kind
            directory.mkdir()
            shutil.copyfile(source_project, directory / "input.json")
            client = Client(executable, directory, transcript, "cloth-" + kind)
            clients.append(client)
            client.call({"op": "load", "path": "input.json"}, True)
            original = client.call({"op": "get_document"})
            source = next(c for c in original["clips"] if c["id"] == "pose")
            motion = replace_joint_curve(source, "shoulder", [0, 0, 1], 12 if kind == "projected" else 6,
                                         capped=kind == "manual")
            client.apply([put(motion)])
            if kind == "projected":
                set_limit(client, "shoulder", limit([0, 0, 1], 6))
            else:
                client.apply([{"op": "clear_joint_limit", "id": "shoulder"}])
            client.call({"op": "bake_cloth", "request": {"id": "garment", "clip": "pose"}}, True)
        projected, manual = clients
        pdoc, rdoc = (c.call({"op": "get_document"}) for c in clients)
        pframes, rframes = (d["cloths"][0]["cache"]["frames"] for d in (pdoc, rdoc))
        assert len(pframes) == len(rframes)
        frame_error = max(distances(p["vertices"], r["vertices"]) for p, r in zip(pframes, rframes))
        assert frame_error <= CLOTH_TOLERANCE_M, frame_error
        times = [0, source["duration"] / 2, source["duration"]]
        comparisons = []
        maximum_pin_error = 0
        for index, seconds in enumerate(times):
            actual, expected = cloth_state(projected, seconds), cloth_state(manual, seconds)
            error = distances(actual["vertices"], expected["vertices"])
            assert error <= CLOTH_TOLERANCE_M, error
            maximum_pin_error = max(maximum_pin_error, pin_error(actual, pose(projected, "pose", seconds)))
            for name, client in zip(("projected", "manual"), clients):
                render(client, f"frame-{index}.png", "pose", seconds)
            comparisons.append({"time": seconds, "maximum_vertex_error_m": error,
                                "image_difference": image_difference(root / f"projected/frame-{index}.png", root / f"manual/frame-{index}.png")})
        assert maximum_pin_error < 2e-6
        cached = copy.deepcopy(pdoc["cloths"])
        projected.apply([{"op": "set_joints", "joints": pdoc["joints"] + [{"id": "unbound", "pivot": [10, 10, 10]}]}])
        set_limit(projected, "unbound", limit([1, 0, 0], 5))
        assert cloth_state(projected)["cache"]["fresh"]
        assert projected.call({"op": "get_document"})["cloths"] == cached
        set_limit(projected, "shoulder", limit([0, 0, 1], 4))
        assert cloth_state(projected)["cache"]["fresh"] is False
        rejected = projected.call({"op": "cloth_state", "id": "garment", "animation": at("pose", times[-1])}, expect_ok=False)
        assert "stale" in rejected["error"]["message"].lower()
        projected.call({"op": "bake_cloth", "request": {"id": "garment", "clip": "pose"}}, True)
        current = cloth_state(projected, times[-1])
        assert current["cache"]["fresh"] and current["sample_within_bake_tolerances"]
        assert current["vertices"] != pframes[-1]["vertices"]
        projected.call({"op": "save", "path": "limited-cloth-project.json"})
        render(projected, "final.png", "pose", times[-1])
        projected.close()
        projected = Client(executable, root / "projected", transcript, "cloth-limits-cold")
        clients[0] = projected
        projected.call({"op": "load", "path": "limited-cloth-project.json"}, True)
        assert cloth_state(projected, times[-1]) == current
        render(projected, "cold.png", "pose", times[-1])
        assert (root / "projected/cold.png").read_bytes() == (root / "projected/final.png").read_bytes()
        return {"compared_cache_frames": len(pframes), "maximum_cached_vertex_error_m": frame_error,
                "maximum_pin_error_m": maximum_pin_error, "pose_comparisons": comparisons,
                "unbound_limit_preserves_cache": True, "relevant_limit_requires_rebake": True,
                "native_cold_reload_exact": True}
    finally:
        for client in clients:
            client.close()


def run_ik(executable, root, transcript):
    root.mkdir()
    client = Client(executable, root, transcript, "limited-ik")
    try:
        bound = {"type": "hinge", "axis": [0, 0, 1], "min_degrees": -90, "max_degrees": 90, "mode": "project"}
        client.apply([
            {"op": "create", "object": {"id": "hand", "position": [2, 0, 0], "shape": {"type": "sphere", "radius": .1}}},
            {"op": "set_joints", "joints": [{"id": "root", "pivot": [0, 0, 0], "rotation_limit": bound},
                {"id": "middle", "parent": "root", "pivot": [1, 0, 0], "rotation_limit": bound},
                {"id": "tip", "parent": "middle", "pivot": [2, 0, 0], "objects": ["hand"]}]},
            put({"id": "reach", "duration": 1})])
        request = {"clip": "reach", "time": .5, "root_joint": "root", "middle_joint": "middle", "tip_joint": "tip",
                   "target": [1.5, .5, 0], "pole": [0, 1, 0], "limit_evaluations": 4096}
        reached = client.call({"op": "solve_ik", "request": request}, True)
        assert reached["requested_target_error_m"] <= 1e-4
        for joint in ("root", "middle"):
            set_limit(client, joint, limit([0, 0, 1], 0))
        request["target"] = [1, 1, 0]
        before, revision = client.call({"op": "get_document"}), client.revision
        client.call({"op": "solve_ik", "request": request}, True, expect_ok=False)
        assert client.revision == revision and client.call({"op": "get_document"}) == before
        approximate = client.call({"op": "solve_ik", "request": dict(request, limit_policy="best_feasible")}, True)
        assert approximate["limit_search"]["approximation_accepted"] and not approximate["limit_search"]["within_tolerance"]
        value = pose(client, "reach", .5)
        tip = next(j for j in value["joints"] if j["id"] == "tip")["world_pivot"]
        assert math.dist(tip, approximate["achieved_tip"]) < 1e-6 and math.dist(tip, [2, 0, 0]) < 1e-6
        assert abs(math.dist(tip, request["target"]) - approximate["requested_target_error_m"]) < 1e-6
        probe = client.call({"op": "sample", "id": "hand", "points": [tip], "animation": at("reach", .5)})
        assert probe["samples"][0]["value"] < -.09
        return {"feasible_target_error_m": reached["requested_target_error_m"], "rejection_atomic": True,
                "best_feasible_actual_target_error_m": approximate["requested_target_error_m"],
                "best_feasible_is_explicit_approximation": True, "search": approximate["limit_search"]}
    finally:
        client.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--editor", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--face-project", type=Path, default=FIXTURES / "face/material-project.json")
    parser.add_argument("--cloth-project", type=Path, default=FIXTURES / "cloth/material-project.json")
    parser.add_argument("--speech-project", type=Path, default=SPEECH / "speech-project.json")
    parser.add_argument("--speech-analysis", type=Path, default=SPEECH / "evidence/spoken-removed-analysis/analysis.json")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    executable = root / ("mm3e-editor.exe" if args.editor.suffix == ".exe" else "mm3e-editor")
    shutil.copyfile(args.editor.resolve(strict=True), executable)
    executable.chmod(0o755)
    report = {"status": "running", "binary_sha256": sha(executable), "harness_sha256": sha(Path(__file__)),
              "tolerances": {"skin_vertices_m": VERTEX_TOLERANCE_M, "cloth_vertices_m": CLOTH_TOLERANCE_M,
                             "field_m": FIELD_TOLERANCE_M, "pixel_mean_absolute_8bit": PIXEL_MEAN_TOLERANCE}, "cases": {},
              "scope": "Configured local joint rotation limits, native skin/cloth evaluation and dependency tracking, actual retained speech curves, bounded IK and frozen delivery; no anatomical calibration, collision freedom or global optimality claim."}
    started = time.monotonic()
    try:
        sources = {name: getattr(args, name).resolve(strict=True) for name in
                   ("face_project", "cloth_project", "speech_project", "speech_analysis")}
        report["source_sha256"] = {name: sha(path) for name, path in sources.items()}
        for name, path in sources.items():
            shutil.copyfile(path, root / (name + "-source-evidence.json"))
        with (root / "transcript.jsonl").open("w") as transcript:
            report["cases"]["face_and_speech"] = run_face(executable, root / "face", transcript,
                sources["face_project"], sources["speech_project"], sources["speech_analysis"])
            print("[joint limits] Continuous jaw references, speech skin and frozen-job restart passed", flush=True)
            write_json(root / "acceptance.json", report)
            report["cases"]["cloth"] = run_cloth(executable, root / "cloth", transcript, sources["cloth_project"])
            print("[joint limits] Cloth manual-cap comparison, pins, freshness and reload passed", flush=True)
            report["cases"]["ik"] = run_ik(executable, root / "ik", transcript)
        assert sha(executable) == report["binary_sha256"]
        report["status"] = "passed"
    except Exception as error:
        report.update(status="failed", error=str(error), traceback=traceback.format_exc())
        raise
    finally:
        report["elapsed_seconds"] = time.monotonic() - started
        report["artifact_sha256"] = {p.relative_to(root).as_posix(): sha(p) for p in sorted(root.rglob("*"))
                                     if p.is_file() and p.name != "acceptance.json"}
        write_json(root / "acceptance.json", report)
    print(json.dumps({"status": report["status"], "elapsed_seconds": report["elapsed_seconds"]}, indent=2))


if __name__ == "__main__":
    main()
