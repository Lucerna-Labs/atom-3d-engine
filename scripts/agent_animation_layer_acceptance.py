#!/usr/bin/env python3
"""Exercise durable animation layers through real editor processes and film fixtures.

Preserves source projects, dispatched requests, responses, frames and failures.
Checks composition, independent edits, physical cache dependencies and restarted
render jobs; it does not certify anatomical, acting or autonomous-agent quality.
"""
import argparse
import copy
import hashlib
import json
import math
from pathlib import Path
import shutil
import time
import traceback

from agent_animation_acceptance import Client, write_json
from agent_sewing_acceptance import png_pixels, pixel_difference

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_FIXTURES = ROOT / "artifacts/material-maps-normal-filter-20260907-opt-in"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def animation(clip, seconds):
    return {"clip": clip, "time": seconds}


def put(clip):
    return {"op": "put_clip", "clip": clip}


def pose(client, clip, seconds):
    value = client.call({"op": "pose", "animation": animation(clip, seconds)})
    return {k: value[k] for k in ("objects", "joints", "camera", "facial_controls", "deformation")}


def deformed(client, identifiers, clip, seconds):
    return {identifier: client.call({"op": "deformer_state", "id": identifier,
                                   "animation": animation(clip, seconds)}) for identifier in identifiers}


def render(client, path, clip, seconds):
    return client.call({"op": "render", "path": path, "animation": animation(clip, seconds)})


def vertices_equal(a, b):
    assert a["vertices"] == b["vertices"], "evaluated vertex positions differ"
    assert a["morph_weights"] == b["morph_weights"], "evaluated morph weights differ"


def copy_source(source, directory):
    directory.mkdir()
    shutil.copyfile(source, directory / "input.json")
    # Retain source evidence outside the editor's root before deleting its import copy.
    shutil.copyfile(source, directory.parent / (directory.name + "-source-evidence.json"))


def split_expression(source):
    body = copy.deepcopy(source)
    body.update(id=source["id"] + "/body", morph_tracks=[], face_tracks=[])
    body.pop("layers", None)
    blink = {"id": source["id"] + "/blink", "duration": source["duration"],
             "morph_tracks": [], "face_tracks": []}
    mouth = {"id": source["id"] + "/mouth", "duration": source["duration"],
             "morph_tracks": [], "face_tracks": []}
    for track in source.get("morph_tracks", []):
        target = blink if "blink" in track["blendshape"] else mouth
        target["morph_tracks"].append(copy.deepcopy(track))
    for track in source.get("face_tracks", []):
        target = blink if track["channel"].startswith("blink") else mouth
        target["face_tracks"].append(copy.deepcopy(track))
    assert blink["morph_tracks"] and mouth["morph_tracks"], "fixture needs independent eyelid and mouth curves"
    root = {"id": source["id"], "duration": source["duration"],
            "layers": [{"id": name, "clip": clip["id"]} for name, clip in
                       (("body", body), ("blink", blink), ("mouth", mouth))]}
    return body, blink, mouth, root


def face_clock_probe(client, body, mouth, root_clip, identifiers):
    track = mouth["morph_tracks"][0]
    key = (track["deformer"], track["blendshape"])
    duration = root_clip["duration"]
    sample_time = duration / 2
    layer = {"id": "weighted-mouth", "clip": mouth["id"], "start": duration / 4,
             "end": duration * 3 / 4, "source_start": duration * 3 / 8,
             "time_scale": 0.5, "weight": 0.5,
             "weight_keys": [{"time": 0, "value": 0}, {"time": duration, "value": 1}],
             "mask": [{"type": "morph", "deformer": key[0], "blendshape": key[1]}]}
    probe = {"id": "layer-clock-probe", "duration": duration,
             "layers": [{"id": "body", "clip": body["id"]}, layer]}
    client.apply([put(probe)])
    observation = client.call({"op": "pose", "animation": animation(probe["id"], sample_time)})
    records = observation["layer_samples"]
    observed = next(v for v in records if v["layer"] == "weighted-mouth")
    assert abs(observed["source_time"] - sample_time) < 1e-6, observed
    assert abs(observed["weight"] - 0.25) < 1e-7 and observed["active"], observed
    document = client.call({"op": "get_document"})
    asset = next(d for d in document["deformers"] if d["id"] == key[0])
    rest = next(m["weight"] for m in asset["blendshapes"] if m["id"] == key[1])
    source = deformed(client, [key[0]], mouth["id"], observed["source_time"])[key[0]]
    source_weight = next(m["weight"] for m in source["morph_weights"] if m["id"] == key[1])
    expected_weight = rest + (source_weight - rest) * 0.25
    actual = deformed(client, identifiers, probe["id"], sample_time)
    actual_weight = next(m["weight"] for m in actual[key[0]]["morph_weights"] if m["id"] == key[1])
    assert abs(actual_weight - expected_weight) < 2e-6
    # A flattened single-channel reference proves that the mask and blended weight
    # affect actual deformation, not only the layer diagnostic response.
    flat = copy.deepcopy(body)
    flat.update(id="layer-clock-flat-reference", morph_tracks=[{
        "deformer": key[0], "blendshape": key[1],
        "keys": [{"time": 0, "weight": actual_weight}]}])
    client.apply([put(flat)])
    expected = deformed(client, identifiers, flat["id"], sample_time)
    for identifier in identifiers:
        vertices_equal(actual[identifier], expected[identifier])
    inactive = deformed(client, identifiers, probe["id"], 0)
    neutral = deformed(client, identifiers, body["id"], 0)
    for identifier in identifiers:
        vertices_equal(inactive[identifier], neutral[identifier])
    return {"source_time": observed["source_time"], "effective_weight": observed["weight"],
            "expected_morph_weight": expected_weight, "observed_morph_weight": actual_weight,
            "masked_geometry_matches_flat_reference": True, "inactive_window_preserves_body": True}


def run_face(executable, directory, transcript):
    client = Client(executable, directory, transcript, "face-layer-author")
    try:
        client.call({"op": "load", "path": "input.json"}, True)
        document = client.call({"op": "get_document"})
        source = next(c for c in document["clips"] if c["id"] == "expression")
        identifiers = [d["id"] for d in document["deformers"]]
        times = [0, source["duration"] / 2, source["duration"]]
        baseline = []
        for index, seconds in enumerate(times):
            baseline.append((pose(client, source["id"], seconds),
                             deformed(client, identifiers, source["id"], seconds)))
            render(client, f"original-{index}.png", source["id"], seconds)
        body, blink, mouth, assembled = split_expression(source)
        client.apply([put(c) for c in (body, blink, mouth, assembled)])
        layered = client.call({"op": "get_document"})
        for key in document:
            if key != "clips":
                assert layered[key] == document[key], f"layer authoring changed {key}"
        vertex_count = 0
        for index, seconds in enumerate(times):
            assert pose(client, assembled["id"], seconds) == baseline[index][0]
            states = deformed(client, identifiers, assembled["id"], seconds)
            for identifier in identifiers:
                vertices_equal(states[identifier], baseline[index][1][identifier])
                vertex_count += len(states[identifier]["vertices"])
            render(client, f"layered-{index}.png", assembled["id"], seconds)
            assert (directory / f"original-{index}.png").read_bytes() == (directory / f"layered-{index}.png").read_bytes()
        print("[animation layers] Split face body/blink/mouth preserves poses, vertices and PNGs exactly", flush=True)

        # Sparse authoring modifies just one source curve, retaining all other channels.
        track = copy.deepcopy(mouth["morph_tracks"][0])
        for key in track["keys"]:
            key["weight"] *= 0.5
        client.apply([{"op": "edit_curve", "request": {"clip": mouth["id"],
                       "edit": {"op": "upsert_morph", "track": track}}}])
        edited = client.call({"op": "get_document"})
        for clip in layered["clips"]:
            if clip["id"] != mouth["id"]:
                assert next(c for c in edited["clips"] if c["id"] == clip["id"]) == clip
        after = deformed(client, identifiers, assembled["id"], times[1])
        before = baseline[1][1]
        changed_deformers = []
        for identifier in identifiers:
            if identifier == track["deformer"]:
                assert after[identifier]["vertices"] != before[identifier]["vertices"]
                changed_deformers.append(identifier)
            else:
                vertices_equal(after[identifier], before[identifier])
        assert pose(client, assembled["id"], times[1])["joints"] == baseline[1][0]["joints"]
        render(client, "edited-mouth.png", assembled["id"], times[1])
        changed_pixels = pixel_difference(png_pixels(directory / "layered-1.png")[0],
                                         png_pixels(directory / "edited-mouth.png")[0])
        assert changed_pixels > 0, "source mouth edit did not reach rendered image"
        clock = face_clock_probe(client, body, mouth, assembled, identifiers)

        # A real shot freezes the edited layered graph, then completes from an empty
        # process after its first frame; synchronous shot output is the reference.
        shot = {"id": "layered-face-shot", "clip": assembled["id"],
                "rate": {"numerator": 5, "denominator": 1}, "start_frame": 1001,
                "frame_count": 3, "clip_frame_zero": 1001}
        client.apply([{"op": "put_shot", "shot": shot}])
        client.call({"op": "render_shot", "request": {"shot": shot["id"],
                     "directory": "shot-reference", "format": "png"}})
        job = client.call({"op": "create_render_job", "request": {"type": "shot", "shot": shot["id"],
                           "directory": "shot-job", "format": "png"}})
        assert job["total_frames"] == 3
        first = client.call({"op": "step_render_job", "directory": "shot-job", "max_frames": 1})
        assert first["completed_frames"] == 1
        first_bytes = (directory / "shot-job/frame_0000.png").read_bytes()
        saved = client.call({"op": "get_document"})
        client.call({"op": "save", "path": "layered-project.json"})
        saved_pose = pose(client, assembled["id"], times[1])
        saved_geometry = deformed(client, identifiers, assembled["id"], times[1])
        render(client, "saved-final.png", assembled["id"], times[1])
        (directory / "input.json").unlink()
        client.close()
        client = Client(executable, directory, transcript, "face-layer-cold")
        assert client.call({"op": "get_document"})["clips"] == []
        done = client.call({"op": "step_render_job", "directory": "shot-job", "max_frames": 2})
        assert done["status"] == "complete" and done["completed_frames"] == 3
        assert (directory / "shot-job/frame_0000.png").read_bytes() == first_bytes
        for index in range(3):
            assert (directory / f"shot-job/frame_{index:04}.png").read_bytes() == (directory / f"shot-reference/frame_{index:04}.png").read_bytes()
        verified_job = client.call({"op": "render_job_state", "directory": "shot-job", "verify_outputs": True})
        assert verified_job["status"] == "complete"
        client.call({"op": "load", "path": "layered-project.json"}, True)
        assert client.call({"op": "get_document"}) == saved
        assert pose(client, assembled["id"], times[1]) == saved_pose
        reloaded_geometry = deformed(client, identifiers, assembled["id"], times[1])
        for identifier in identifiers:
            vertices_equal(reloaded_geometry[identifier], saved_geometry[identifier])
        render(client, "cold-final.png", assembled["id"], times[1])
        assert (directory / "cold-final.png").read_bytes() == (directory / "saved-final.png").read_bytes()
        return {"exact_equivalence_times": times, "compared_deformed_vertices": vertex_count,
                "edited_deformers": changed_deformers, "mouth_edit_changed_pixels": changed_pixels,
                "clock_and_mask": clock, "cold_reload_exact": True,
                "restarted_shot_frames_exact": 3, "job_snapshot_sha256": verified_job["snapshot_sha256"]}
    finally:
        client.close()


def pin_error(state, pose_value):
    objects = {o["id"]: o for o in pose_value["objects"]}
    errors = []
    for pin in state["pins"]:
        if pin["target_object"] is None:
            expected = pin["point"]
        else:
            obj = objects[pin["target_object"]]
            expected = [obj["position"][axis] + sum(obj["basis"][i][axis] * pin["point"][i]
                        * obj["scale"] for i in range(3)) for axis in range(3)]
        errors.append(math.dist(expected, state["vertices"][pin["vertex"]]))
    return max(errors, default=0)


def auxiliary_face_operations():
    objects = [
        {"id": "layer-aux/head", "position": [20, 20, 20], "shape": {"type": "ellipsoid", "radii": [0.8, 1, 0.8]}},
        {"id": "layer-aux/left_eye", "position": [20.28, 20.2, 20.78], "shape": {"type": "sphere", "radius": 0.15}},
        {"id": "layer-aux/right_eye", "position": [19.72, 20.2, 20.78], "shape": {"type": "sphere", "radius": 0.15}}]
    return [{"op": "create", "object": obj} for obj in objects] + [
        {"op": "create_face", "request": {"id": "layer-aux-face", "character": "layer-aux"}}]


def run_cloth(executable, directory, transcript):
    client = Client(executable, directory, transcript, "cloth-layer-author")
    try:
        client.call({"op": "load", "path": "input.json"}, True)
        document = client.call({"op": "get_document"})
        asset = next(c for c in document["cloths"] if c["id"] == "garment")
        cache = asset["cache"]
        clip = next(c for c in document["clips"] if c["id"] == cache["clip"])
        times = [0, cache["duration"] / 2, cache["duration"]]
        baseline = []
        for index, seconds in enumerate(times):
            state = client.call({"op": "cloth_state", "id": asset["id"], "animation": animation(clip["id"], seconds)})
            baseline.append(state)
            render(client, f"original-{index}.png", clip["id"], seconds)
        motion = copy.deepcopy(clip)
        motion["id"] = clip["id"] + "/physical-motion"
        assembled = {"id": clip["id"], "duration": clip["duration"],
                     "layers": [{"id": "physical-motion", "clip": motion["id"]}]}
        client.apply([put(motion), put(assembled)])
        request = {"id": asset["id"], "clip": clip["id"], "duration": cache["duration"]}
        client.call({"op": "bake_cloth", "request": request}, True)
        layered_document = client.call({"op": "get_document"})
        baked = next(c for c in layered_document["cloths"] if c["id"] == asset["id"])["cache"]
        assert baked["frames"] == cache["frames"], "splitting motion changed simulated cloth frames"
        assert baked["diagnostics"] == cache["diagnostics"]
        maximum_pin_error = 0
        for index, seconds in enumerate(times):
            state = client.call({"op": "cloth_state", "id": asset["id"], "animation": animation(clip["id"], seconds)})
            assert state["vertices"] == baseline[index]["vertices"] and state["cache"]["fresh"]
            maximum_pin_error = max(maximum_pin_error, pin_error(state, pose(client, clip["id"], seconds)))
            render(client, f"layered-{index}.png", clip["id"], seconds)
            assert (directory / f"original-{index}.png").read_bytes() == (directory / f"layered-{index}.png").read_bytes()
        assert maximum_pin_error < 2e-6
        print("[animation layers] Layered cloth rebake matches original frames, sampled geometry, pins and PNGs", flush=True)

        # These real channels are deliberately outside the garment's pin/collider
        # dependency graph. Their edits must preserve the existing physical cache.
        camera = document["camera"]
        unrelated = {"id": "cloth-unrelated", "duration": clip["duration"],
                     "camera_keys": [{"time": 0, "eye": camera["eye"], "target": camera["target"], "fov_degrees": camera["fov_degrees"]}],
                     "face_tracks": [{"face": "layer-aux-face", "channel": "jaw_open",
                                      "keys": [{"time": 0, "value": 0}, {"time": clip["duration"], "value": 0.2}]}]}
        client.apply(auxiliary_face_operations() + [put(unrelated), {"op": "edit_layer", "request": {
            "clip": clip["id"], "action": {"op": "upsert", "layer": {"id": "unrelated", "clip": unrelated["id"]}}}}])
        assert client.call({"op": "cloth_state", "id": asset["id"]})["cache"]["fresh"]
        unrelated["camera_keys"][0]["eye"] = [camera["eye"][0] + 0.2, *camera["eye"][1:]]
        unrelated["face_tracks"][0]["keys"][-1]["value"] = 0.7
        client.apply([put(unrelated)])
        assert client.call({"op": "cloth_state", "id": asset["id"]})["cache"]["fresh"]
        after_unrelated = client.call({"op": "get_document"})
        assert next(c for c in after_unrelated["cloths"] if c["id"] == asset["id"])["cache"] == baked
        sample_unrelated = client.call({"op": "cloth_state", "id": asset["id"], "animation": animation(clip["id"], times[-1])})
        assert sample_unrelated["vertices"] == baseline[-1]["vertices"]
        render(client, "before-physical-edit.png", clip["id"], times[-1])

        physical = copy.deepcopy(motion)
        physical["tracks"][0]["keys"][-1]["rotation_degrees"][1] += 1
        client.apply([put(physical)])
        assert client.call({"op": "cloth_state", "id": asset["id"]})["cache"]["fresh"] is False
        rejected = client.call({"op": "cloth_state", "id": asset["id"],
                               "animation": animation(clip["id"], times[-1])}, expect_ok=False)
        assert "stale" in rejected["error"]["message"].lower(), rejected
        client.call({"op": "bake_cloth", "request": request}, True)
        final_state = client.call({"op": "cloth_state", "id": asset["id"], "animation": animation(clip["id"], times[-1])})
        assert final_state["cache"]["fresh"] and final_state["sample_within_bake_tolerances"]
        assert final_state["vertices"] != baseline[-1]["vertices"]
        maximum_pin_error = max(maximum_pin_error, pin_error(final_state, pose(client, clip["id"], times[-1])))
        assert maximum_pin_error < 2e-6
        render(client, "after-physical-edit.png", clip["id"], times[-1])
        physical_pixels = pixel_difference(png_pixels(directory / "before-physical-edit.png")[0],
                                          png_pixels(directory / "after-physical-edit.png")[0])
        assert physical_pixels > 0
        saved = client.call({"op": "get_document"})
        client.call({"op": "save", "path": "layered-project.json"})
        (directory / "input.json").unlink()
        client.close()
        client = Client(executable, directory, transcript, "cloth-layer-cold")
        client.call({"op": "load", "path": "layered-project.json"}, True)
        assert client.call({"op": "get_document"}) == saved
        assert client.call({"op": "cloth_state", "id": asset["id"], "animation": animation(clip["id"], times[-1])}) == final_state
        render(client, "cold-final.png", clip["id"], times[-1])
        assert (directory / "cold-final.png").read_bytes() == (directory / "after-physical-edit.png").read_bytes()
        return {"rebaked_frames_exact": len(cache["frames"]), "sample_times": times,
                "maximum_pin_error_m": maximum_pin_error, "irrelevant_face_camera_edits_preserve_cache": True,
                "physical_source_edit_requires_rebake": True, "physical_edit_changed_pixels": physical_pixels,
                "cold_reload_exact": True}
    finally:
        client.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--editor", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--face-project", type=Path, default=DEFAULT_FIXTURES / "face/material-project.json")
    parser.add_argument("--cloth-project", type=Path, default=DEFAULT_FIXTURES / "cloth/material-project.json")
    args = parser.parse_args()
    directory = args.output.resolve()
    directory.mkdir(parents=True, exist_ok=False)
    executable = directory / ("mm3e-editor.exe" if args.editor.suffix == ".exe" else "mm3e-editor")
    shutil.copyfile(args.editor.resolve(strict=True), executable)
    executable.chmod(0o755)
    report = {"status": "running", "binary_sha256": sha(executable), "harness_sha256": sha(Path(__file__)),
              "source_sha256": {}, "cases": {},
              "scope": "Deterministic real-process layer authoring, deformation, native persistence, rendering, physical cache dependencies and restarted shot delivery; no artistic or autonomous-planning certification."}
    started = time.monotonic()
    try:
        with (directory / "transcript.jsonl").open("w") as transcript:
            for kind, source, runner in (("face", args.face_project, run_face), ("cloth", args.cloth_project, run_cloth)):
                source = source.resolve(strict=True)
                report["source_sha256"][kind] = sha(source)
                copy_source(source, directory / kind)
                report["cases"][kind] = runner(executable, directory / kind, transcript)
                write_json(directory / "acceptance.json", report)
        assert sha(executable) == report["binary_sha256"]
        report["status"] = "passed"
    except Exception as error:
        report.update(status="failed", error=str(error), traceback=traceback.format_exc())
        raise
    finally:
        report["elapsed_seconds"] = time.monotonic() - started
        write_json(directory / "acceptance.json", report)
    print(json.dumps({"status": report["status"], "elapsed_seconds": report["elapsed_seconds"]}, indent=2))


if __name__ == "__main__":
    main()
