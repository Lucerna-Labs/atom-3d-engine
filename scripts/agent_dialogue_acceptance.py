#!/usr/bin/env python3
"""Actual JSONL reference-audio, rational-shot and manually authored mouth acceptance.

Uses the checked-in original spoken PCM16 fixture and Python's independent wave reader.
All requests, responses, failures, rendered PNGs, WAV slices and native projects remain
in a new output directory. This checks explicit timing, not automatic speech alignment,
phoneme recognition, measured acting quality or anatomical facial deformation.
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
import wave

from agent_animation_acceptance import Client, frame_path, make_playback, write_json
from agent_sewing_acceptance import png_pixels, pixel_difference

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "mm3e-editor/tests/fixtures/dialogue/hello-reference.wav"
PROVENANCE = FIXTURE.with_name("provenance.json")
SOURCE_IN = 317
START_FRAME = 1001
FRAME_COUNT = 69
SIZE = 128
PROBE = [0.0, -0.467, 0.66]
# Explicit authoring choices in the source recording. These are editable timing
# anchors, not inferred word/phoneme boundaries or an automatic alignment result.
CUES = [
    {"id": "rest-start", "sample": 317, "jaw_open": 0.0, "lip_round": 0.0},
    {"id": "open-a", "sample": 8317, "jaw_open": 1.0, "lip_round": 0.0},
    {"id": "closure", "sample": 16317, "jaw_open": 0.0, "lip_round": 0.0},
    {"id": "rounded-b", "sample": 24317, "jaw_open": 0.7, "lip_round": 1.0},
    {"id": "open-c", "sample": 36317, "jaw_open": 0.5, "lip_round": 0.3},
    {"id": "rest-tail", "sample": 44317, "jaw_open": 0.0, "lip_round": 0.0},
    {"id": "rest-end", "sample": 46317, "jaw_open": 0.0, "lip_round": 0.0},
]


def sha256(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def f32(value):
    return struct.unpack("<f", struct.pack("<f", value))[0]


def pcm16(path):
    """Decode independently of the Rust importer and compare actual PCM bytes."""
    with wave.open(str(path), "rb") as source:
        assert source.getcomptype() == "NONE"
        assert source.getsampwidth() == 2
        channels, rate, count = source.getnchannels(), source.getframerate(), source.getnframes()
        pcm = source.readframes(count)
        assert source.readframes(1) == b""
    assert len(pcm) == count * channels * 2
    values = struct.unpack("<" + str(count * channels) + "h", pcm)
    return {"channels": channels, "rate": rate, "frames": count, "pcm": pcm, "values": values}


def verify_wave(path, source, start, count):
    actual = pcm16(path)
    assert (actual["channels"], actual["rate"], actual["frames"]) == (source["channels"], source["rate"], count)
    width = source["channels"] * 2
    assert actual["pcm"] == source["pcm"][start * width:(start + count) * width]
    return {"path": str(path), "sha256": sha256(path), "start_sample": start,
            "sample_frames": count, "pcm_sha256": hashlib.sha256(actual["pcm"]).hexdigest()}


def verify_audio_state(state, source, original_sha, start=0, count=None):
    count = source["frames"] if count is None else count
    assert state["source_sha256"] == original_sha
    assert state["sample_rate"] == source["rate"] and state["sample_frames"] == source["frames"]
    assert state["channels"] == source["channels"] and state["bits_per_sample"] == 16
    assert state["range"] == {"start_sample": start, "frame_count": count}
    for entry in state["samples"]:
        sample = entry["sample"]
        expected = [source["values"][sample * source["channels"] + c] / 32768.0
                    for c in range(source["channels"])]
        assert entry["values"] == expected
    waveform = state["waveform"]
    assert len(waveform) == min(64, count)
    for i, entry in enumerate(waveform):
        a, b = start + i * count // len(waveform), start + (i + 1) * count // len(waveform)
        assert (entry["start_sample"], entry["end_sample_exclusive"]) == (a, b)
        for channel in entry["channels"]:
            values = [source["values"][j * source["channels"] + channel["channel"]] / 32768.0 for j in range(a, b)]
            assert channel["min"] == min(values) and channel["max"] == max(values)
            rms = math.sqrt(sum(value * value for value in values) / len(values))
            assert math.isclose(channel["rms"], rms, rel_tol=2e-13, abs_tol=1e-15)


def recipe(identifier, numerator, denominator):
    return {"id": identifier, "clip": "dialogue", "rate": {"numerator": numerator, "denominator": denominator},
            "start_frame": START_FRAME, "frame_count": FRAME_COUNT, "clip_frame_zero": START_FRAME,
            "audio": {"asset": "voice", "start_sample": SOURCE_IN}}


def boundary(shot, rate, index):
    return shot["audio"]["start_sample"] + index * rate * shot["rate"]["denominator"] // shot["rate"]["numerator"]


def verify_shot_state(state, shot, source, original_sha):
    assert state["shot"] == shot and len(state["frames"]) == shot["frame_count"]
    assert state["end_frame_exclusive"] == shot["start_frame"] + shot["frame_count"]
    assert state["audio_timing"] == {"sample_rate": source["rate"], "sample_frames": source["frames"]}
    assert state["audio_asset"]["source_sha256"] == original_sha
    for i, frame in enumerate(state["frames"]):
        absolute = shot["start_frame"] + i
        seconds = (absolute - shot["clip_frame_zero"]) * shot["rate"]["denominator"] / shot["rate"]["numerator"]
        assert frame["index"] == i and frame["frame"] == absolute
        assert frame["scheduled_time"] == seconds
        assert f32(frame["sampled_time"]) == f32(seconds)
        assert frame["audio"] == {"start_sample": boundary(shot, source["rate"], i),
                                  "end_sample": boundary(shot, source["rate"], i + 1)}
        if i:
            assert state["frames"][i - 1]["audio"]["end_sample"] == frame["audio"]["start_sample"]
    assert state["audio_range"] == {"start_sample": SOURCE_IN, "end_sample": boundary(shot, source["rate"], FRAME_COUNT)}
    numerator = FRAME_COUNT * source["rate"] * shot["rate"]["denominator"]
    assert state["audio_end_fraction_numerator"] == numerator % shot["rate"]["numerator"]
    end_seconds = FRAME_COUNT * shot["rate"]["denominator"] / shot["rate"]["numerator"]
    assert state["end_clip_time_exclusive"] == end_seconds


def source_operations():
    objects = [
        {"id": "actor/head", "shape": {"type": "ellipsoid", "radii": [.8, 1, .8]},
         "material": {"albedo": [.65, .4, .26], "roughness": .7}},
        {"id": "actor/left_eye", "position": [.28, .2, .78], "shape": {"type": "sphere", "radius": .15},
         "material": {"albedo": [.05, .1, .15]}},
        {"id": "actor/right_eye", "position": [-.28, .2, .78], "shape": {"type": "sphere", "radius": .15},
         "material": {"albedo": [.05, .1, .15]}},
        {"id": "marker", "position": [3, 0, 0], "shape": {"type": "sphere", "radius": .05}},
    ]
    blink = [{"time": 0, "value": 0}, {"time": .85, "value": 0}, {"time": 1, "value": 1},
             {"time": 1.15, "value": 0}, {"time": 3, "value": 0}]
    tracks = [{"face": "face", "channel": channel, "easing": "smooth_step", "keys": blink}
              for channel in ("blink_left", "blink_right")]
    tracks += [{"face": "face", "channel": channel, "keys": [{"time": 0, "value": 0}, {"time": 3, "value": 0}]}
               for channel in ("jaw_open", "lip_round", "smile")]
    clip = {"id": "dialogue", "duration": 3, "face_tracks": tracks,
            "tracks": [{"target": {"type": "object", "id": "marker"}, "keys": [
                {"time": 0}, {"time": 3, "translation": [0, .2, 0]}]}],
            "camera_keys": [{"time": 0, "eye": [0, 0, 4], "target": [0, 0, 0], "fov_degrees": 38},
                            {"time": 3, "eye": [.06, 0, 4], "target": [0, 0, 0], "fov_degrees": 38}]}
    return ([{"op": "create", "object": obj} for obj in objects] + [
        {"op": "create_face", "request": {"id": "face", "character": "actor"}},
        {"op": "put_clip", "clip": clip},
        {"op": "put_clip", "clip": {"id": "unrelated", "duration": 1}},
        {"op": "set_camera", "camera": {"eye": [0, 0, 4], "target": [0, 0, 0], "fov_degrees": 38}},
        {"op": "set_settings", "settings": {"width": SIZE, "height": SIZE, "quality": "preview",
                                                "shadows": False, "ao": False}},
    ])


def sample(seconds):
    return {"clip": "dialogue", "time": seconds, "playback": "clamp"}


def author_curve(client, channel, rate):
    track = {"face": "face", "channel": channel, "easing": "linear",
             "keys": [{"time": f32((cue["sample"] - SOURCE_IN) / rate), "value": f32(cue[channel])} for cue in CUES]}
    before = client.call({"op": "get_document"})
    expected = copy.deepcopy(before)
    clip = next(clip for clip in expected["clips"] if clip["id"] == "dialogue")
    index = next(i for i, curve in enumerate(clip["face_tracks"]) if curve["face"] == "face" and curve["channel"] == channel)
    clip["face_tracks"][index] = track
    client.apply([{"op": "edit_curve", "request": {"clip": "dialogue", "edit": {"op": "upsert_face", "track": track}}}])
    assert client.call({"op": "get_document"}) == expected, "sparse curve edit changed unrelated source/animation/audio data"
    return track


def verify_render(client, root, shot, source, original_sha, directory, start_index=0, count=FRAME_COUNT, reference=None):
    selection = {"start_frame": shot["start_frame"] + start_index, "frame_count": count}
    command = {"op": "render_shot", "request": {"shot": shot["id"], "directory": directory, "format": "png"}}
    if start_index or count != shot["frame_count"]:
        command["request"]["selection"] = selection
    before_revision = client.revision
    result = client.call(command)
    assert client.revision == before_revision
    manifest_path = frame_path(root, result["manifest_path"])
    manifest = json.loads(manifest_path.read_text())
    assert manifest["format"] == "mm3e-shot-sequence" and manifest["version"] == 1
    assert manifest["shot"] == shot and manifest["selection"] == selection
    assert len(manifest["frames"]) == count and manifest["frame_count"] == count
    paths = []
    for local, frame in enumerate(manifest["frames"]):
        index = start_index + local
        seconds = index * shot["rate"]["denominator"] / shot["rate"]["numerator"]
        assert frame["index"] == local and frame["shot_index"] == index
        assert frame["shot_frame"] == shot["start_frame"] + index
        assert frame["scheduled_time"] == seconds and f32(frame["time"]) == f32(seconds)
        assert frame["audio_window"] == {"start_sample": boundary(shot, source["rate"], index),
                                         "end_sample": boundary(shot, source["rate"], index + 1)}
        path = frame_path(root, frame["path"])
        decoded = png_pixels(path)
        assert (decoded[1]["width"], decoded[1]["height"]) == (SIZE, SIZE)
        paths.append(path)
        if reference is not None:
            expected = reference["frames"][index]
            assert frame["audio_window"] == expected["audio_window"]
            assert frame["rgba_fnv1a64"] == expected["rgba_fnv1a64"]
            assert path.read_bytes() == frame_path(root, expected["path"]).read_bytes()
    audio = manifest["audio"]
    start = boundary(shot, source["rate"], start_index)
    end = boundary(shot, source["rate"], start_index + count)
    assert audio["asset"] == "voice" and audio["source_sha256"] == original_sha
    assert (audio["start_sample"], audio["end_sample_exclusive"], audio["sample_frames"]) == (start, end, end - start)
    assert (audio["sample_rate"], audio["channels"]) == (source["rate"], source["channels"])
    audio_path = frame_path(root, audio["path"])
    assert audio_path.name == "audio.wav" and audio["sha256"] == sha256(audio_path)
    wave_result = verify_wave(audio_path, source, start, end - start)
    return manifest, paths, {"manifest": str(manifest_path), "manifest_sha256": sha256(manifest_path),
                             "selection": selection, "audio": wave_result,
                             "frame_sha256": [sha256(path) for path in paths]}


def rejected(client, command, mutation=False):
    before = client.call({"op": "get_document"})
    revision = client.revision
    response = client.call(command, mutate=mutation, expect_ok=False)
    assert client.revision == revision
    assert client.call({"op": "get_document"}) == before
    return response["error"]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--editor", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    source_binary = args.editor.resolve(strict=True)
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    executable = root / "tested-mm3e-editor"
    shutil.copy2(source_binary, executable)
    assert sha256(executable) == sha256(source_binary)
    shutil.copy2(FIXTURE, root / "input.wav")
    shutil.copy2(PROVENANCE, root / "source-provenance.json")
    source = pcm16(root / "input.wav")
    original_sha = sha256(root / "input.wav")
    provenance = json.loads(PROVENANCE.read_text())
    assert (source["channels"], source["rate"], source["frames"]) == (1, 16000, 46936)
    assert original_sha == provenance["sha256"]
    write_json(root / "manual-cues.json", {"source_sha256": original_sha, "source_in_sample": SOURCE_IN,
                                           "automatic_alignment": False, "cues": CUES})
    started = time.monotonic()
    report = {"status": "running", "binary": str(executable), "binary_source": str(source_binary),
              "binary_sha256": sha256(executable), "script_sha256": sha256(Path(__file__).resolve()),
              "source": provenance, "manual_cues": CUES, "cases": {},
              "scope": "Original spoken reference audio, explicit sample timing, rational image/audio delivery and manual procedural SDF mouth curves; no automatic speech alignment, acting-quality or anatomical-deformation claim"}
    client = None
    write_json(root / "acceptance.json", report)
    try:
        with (root / "transcript.jsonl").open("w") as transcript:
            client = Client(executable, root, transcript, "author")
            import_request = {"id": "voice", "label": provenance["text"], "path": "input.wav", "replace": False}
            before = client.call({"op": "get_document"})
            revision = client.revision
            dry = client.call({"op": "import_audio", "request": import_request, "dry_run": True}, True)
            assert dry["committed"] is False and client.revision == revision
            assert client.call({"op": "get_document"}) == before
            imported = client.call({"op": "import_audio", "request": import_request, "dry_run": False}, True)
            assert imported["committed"] is True and imported["source_sha256"] == original_sha
            queries = [0, SOURCE_IN, *[cue["sample"] for cue in CUES[1:]], source["frames"] - 1]
            audio_request = {"id": "voice", "bins": 64, "samples": queries}
            audio_state = client.call({"op": "audio_state", "request": audio_request})
            verify_audio_state(audio_state, source, original_sha)
            ranged = {**audio_request, "range": {"start_sample": SOURCE_IN, "frame_count": 8001}}
            verify_audio_state(client.call({"op": "audio_state", "request": ranged}), source, original_sha, SOURCE_IN, 8001)
            exports = []
            for path, start, count in [("whole.wav", 0, source["frames"]), ("inpoint.wav", SOURCE_IN, 8001)]:
                exported = client.call({"op": "export_audio", "request": {"id": "voice", "path": path,
                                       "range": {"start_sample": start, "frame_count": count}, "overwrite": False}})
                proof = verify_wave(frame_path(root, exported["path"]), source, start, count)
                assert exported["sha256"] == proof["sha256"]
                exports.append(proof)
            report["cases"]["independent_pcm_and_waveform"] = {"queries": queries, "bins": 64, "exports": exports}
            print("[dialogue] Imported speech; independent PCM samples, waveform and WAV slices match", flush=True)

            client.apply(source_operations())
            before_render = client.call({"op": "render", "path": "before-mouth.png", "animation": sample(.5)})
            curves = [author_curve(client, channel, source["rate"]) for channel in ("jaw_open", "lip_round")]
            write_json(root / "authored-curves.json", curves)
            shots = [recipe("dialogue-24", 24, 1), recipe("dialogue-23976", 24000, 1001)]
            client.apply([{"op": "put_shot", "shot": shot} for shot in shots])
            authored = client.call({"op": "get_document"})
            states = {}
            for shot in shots:
                state = client.call({"op": "shot_state", "id": shot["id"]})
                verify_shot_state(state, shot, source, original_sha)
                states[shot["id"]] = state
                for index in [0, 1, 12, 24, 36, 68]:
                    pose = client.call({"op": "pose_shot", "shot": shot["id"], "frame": START_FRAME + index})
                    assert pose["shot"] == shot["id"] and pose["shot_frame"] == state["frames"][index]
                    ordinary = client.call({"op": "pose", "animation": sample(state["frames"][index]["sampled_time"])})
                    assert {k: v for k, v in pose.items() if k not in ("shot", "shot_frame")} == ordinary
            assert client.call({"op": "get_document"}) == authored
            report["cases"]["rational_clocks"] = {name: {"range": state["audio_range"], "remainder": state["audio_end_fraction_numerator"],
                                                           "frames": len(state["frames"])} for name, state in states.items()}

            cue_renders = {}
            distances = {}
            for name, seconds in [("rest-start", 0), ("open-a", .5), ("closure", 1), ("rounded-b", 1.5), ("rest-tail", 2.75)]:
                rendered = client.call({"op": "render", "path": f"cue-{name}.png", "animation": sample(seconds)})
                cue_renders[name] = rendered
                distances[name] = client.call({"op": "sample", "id": "actor/head", "points": [PROBE],
                                               "animation": sample(seconds)})["samples"][0]["value"]
                png_pixels(root / f"cue-{name}.png")
            assert distances["rest-start"] < 0 < distances["open-a"]
            assert distances["closure"] < 0 and distances["rest-tail"] < 0
            opening = cue_renders["open-a"]["metrics"]["visible_material_owner_pixels"].get("face/mouth_interior", 0)
            neutral = cue_renders["rest-start"]["metrics"]["visible_material_owner_pixels"].get("face/mouth_interior", 0)
            assert opening > neutral + 5
            for eye in ("actor/left_eye", "actor/right_eye"):
                assert cue_renders["open-a"]["metrics"]["visible_material_owner_pixels"].get(eye, 0) > 0
                assert cue_renders["closure"]["metrics"]["visible_material_owner_pixels"].get(eye, 0) == 0
            difference = pixel_difference(png_pixels(root / "before-mouth.png")[0], png_pixels(root / "cue-open-a.png")[0])
            assert difference > 20 and before_render["rgba_fnv1a64"] != cue_renders["open-a"]["rgba_fnv1a64"]
            report["cases"]["manual_mouth_and_blink"] = {"field_probe": PROBE, "distances": distances,
                                                        "open_interior_pixels": opening, "neutral_interior_pixels": neutral,
                                                        "changed_pixels_after_sparse_curve_edit": difference}
            print("[dialogue] Rational schedules and real mouth/blink geometry verified", flush=True)

            full = {}
            for shot in shots:
                manifest, paths, proof = verify_render(client, root, shot, source, original_sha, shot["id"])
                full[shot["id"]] = manifest
                report["cases"][shot["id"]] = proof
                _, _, partial = verify_render(client, root, shot, source, original_sha, shot["id"] + "-partial", 1, 5, manifest)
                reset_length = 5 * source["rate"] * shot["rate"]["denominator"] // shot["rate"]["numerator"]
                assert partial["audio"]["sample_frames"] == reset_length + 1, "fixture must expose a reset-floor-phase error"
                report["cases"][shot["id"] + "-partial"] = partial
                if shot["id"] == "dialogue-24":
                    for name, index in [("rest-start", 0), ("open-a", 12), ("closure", 24), ("rounded-b", 36), ("rest-tail", 66)]:
                        assert paths[index].read_bytes() == (root / f"cue-{name}.png").read_bytes()
                    make_playback(paths, root / "dialogue-playback.png", 24)
                print(f"[dialogue] {shot['id']}: {len(paths)} frames and full/partial PCM sidecars verified", flush=True)

            # Retime only the jaw curve; all blink/camera/source/audio fields must persist.
            changed_track = copy.deepcopy(curves[0])
            changed_track["keys"][1]["time"] = .625
            expected = copy.deepcopy(authored)
            clip = next(c for c in expected["clips"] if c["id"] == "dialogue")
            index = next(i for i, t in enumerate(clip["face_tracks"]) if t["channel"] == "jaw_open")
            clip["face_tracks"][index] = changed_track
            client.apply([{"op": "edit_curve", "request": {"clip": "dialogue", "edit": {"op": "upsert_face", "track": changed_track}}}])
            assert client.call({"op": "get_document"}) == expected
            retimed = client.call({"op": "face_state", "animation": sample(.5)})
            assert retimed["faces"][0]["controls"]["jaw_open"] < 1
            client.call({"op": "undo"}, True)
            assert client.call({"op": "get_document"}) == authored
            # Existing clip replacement permits a camera-only edit when the client retains all other fields.
            updated_clip = copy.deepcopy(next(c for c in authored["clips"] if c["id"] == "dialogue"))
            updated_clip["camera_keys"][1]["eye"][0] = .125
            expected = copy.deepcopy(authored)
            next(c for c in expected["clips"] if c["id"] == "dialogue")["camera_keys"] = updated_clip["camera_keys"]
            client.apply([{"op": "put_clip", "clip": updated_clip}])
            assert client.call({"op": "get_document"}) == expected
            client.call({"op": "undo"}, True)
            assert client.call({"op": "get_document"}) == authored
            report["cases"]["sparse_retime_and_camera_preservation"] = True

            (root / "invalid.wav").write_bytes((root / "input.wav").read_bytes()[:31])
            errors = []
            errors.append(rejected(client, {"op": "import_audio", "request": import_request}, True))
            errors.append(rejected(client, {"op": "import_audio", "request": {**import_request, "path": "invalid.wav", "replace": True}}, True))
            errors.append(rejected(client, {"op": "apply", "operations": [{"op": "remove_audio", "id": "voice"}]}, True))
            for field, value in [("frame_count", 0), ("frame_count", 100), ("rate", {"numerator": 24, "denominator": 0}),
                                 ("audio", {"asset": "voice", "start_sample": source["frames"]}),
                                 ("audio", {"asset": "missing", "start_sample": 0})]:
                bad = {**shots[0], field: value}
                errors.append(rejected(client, {"op": "apply", "operations": [{"op": "put_shot", "shot": bad}]}, True))
            duplicate = copy.deepcopy(curves[0])
            duplicate["keys"][1]["time"] = duplicate["keys"][0]["time"]
            errors.append(rejected(client, {"op": "apply", "operations": [{"op": "edit_curve", "request": {
                "clip": "dialogue", "edit": {"op": "upsert_face", "track": duplicate}}}]}, True))
            for i, selection in enumerate([{"start_frame": START_FRAME - 1, "frame_count": 1},
                                           {"start_frame": START_FRAME + FRAME_COUNT - 1, "frame_count": 2}]):
                directory = f"invalid-shot-{i}"
                errors.append(rejected(client, {"op": "render_shot", "request": {"shot": shots[0]["id"],
                    "directory": directory, "selection": selection}}))
                assert not (root / directory).exists()
            errors.append(rejected(client, {"op": "export_audio", "request": {"id": "voice", "path": "invalid-export.wav",
                "range": {"start_sample": source["frames"], "frame_count": 1}}}))
            assert not (root / "invalid-export.wav").exists()
            # A bad import cannot authorize a missing/obsolete revision.
            errors.append(rejected(client, {"op": "import_audio", "request": {**import_request, "id": "revision-check"}}))
            report["cases"]["atomic_failures"] = errors

            client.call({"op": "save", "path": "dialogue.json"})
            saved_document = client.call({"op": "get_document"})
            assert saved_document == authored
            (root / "input.wav").unlink()
            assert not (root / "input.wav").exists()
            client.close()
            client = Client(executable, root, transcript, "cold-reload")
            client.call({"op": "load", "path": "dialogue.json"}, True)
            assert client.call({"op": "get_document"}) == saved_document
            reloaded_audio = client.call({"op": "audio_state", "request": audio_request})
            assert reloaded_audio == audio_state
            for shot in shots:
                assert client.call({"op": "shot_state", "id": shot["id"]}) == states[shot["id"]]
            exported = client.call({"op": "export_audio", "request": {"id": "voice", "path": "cold-whole.wav",
                "range": {"start_sample": 0, "frame_count": source["frames"]}}})
            verify_wave(frame_path(root, exported["path"]), source, 0, source["frames"])
            assert (root / "cold-whole.wav").read_bytes() == (root / "whole.wav").read_bytes()
            _, _, cold = verify_render(client, root, shots[1], source, original_sha, "cold-partial", 1, 5, full[shots[1]["id"]])
            assert (root / "cold-partial/audio.wav").read_bytes() == (root / "dialogue-23976-partial/audio.wav").read_bytes()
            report["cases"]["cold_reload_without_source_file"] = cold
            client.close()
            client = None
            report["status"] = "passed"
            report["rendered_shot_frames"] = FRAME_COUNT * 2 + 15
            report["project_sha256"] = sha256(root / "dialogue.json")
            print("[dialogue] Cold reload retained embedded audio, curves, clocks, PNGs and PCM sidecar", flush=True)
    except Exception as error:
        report.update({"status": "failed", "error": str(error), "traceback": traceback.format_exc()})
        raise
    finally:
        if client is not None:
            client.abort()
        report["elapsed_seconds"] = time.monotonic() - started
        report["binary_sha256_after"] = sha256(executable)
        write_json(root / "acceptance.json", report)
    print(json.dumps({"status": report["status"], "elapsed_seconds": report["elapsed_seconds"],
                      "rendered_shot_frames": report["rendered_shot_frames"], "report": str(root / "acceptance.json")}, indent=2))


if __name__ == "__main__":
    main()
