#!/usr/bin/env python3
"""Real local speech-to-editable-mouth acceptance, with preserved recognition evidence.

The declared procedural pose profile is an authored test mapping. Varied cues,
field samples and image changes do not establish recognition/acting accuracy or
forced transcript alignment. Native curves and source PCM must remain usable
after the optional recognizer and external analysis files are disconnected.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import shutil
import struct
import time
import traceback
import wave

from agent_animation_acceptance import Client, write_json
from agent_dialogue_acceptance import FIXTURE, PROVENANCE, pcm16, source_operations, verify_wave
from agent_sewing_acceptance import png_pixels, pixel_difference

ROOT = Path(__file__).resolve().parents[1]
POSES = {"A": (0, 0, 0), "B": (.18, 0, .35), "C": (.65, 0, .2),
         "D": (1, 0, .1), "E": (.45, 1, -.3), "F": (.18, .5, -.2), "X": (0, 0, 0)}
CHANNELS = ("jaw_open", "lip_round", "lip_wide")
POINTS = [[x, y, z] for x in (-.25, 0, .25) for y in (-.55, -.467, -.38) for z in (.55, .66, .72)]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def clip_sample(clip, seconds):
    return {"clip": clip, "time": seconds}


def face_state(client, clip, seconds):
    return client.call({"op": "face_state", "animation": clip_sample(clip, seconds)})["faces"][0]


def pose(client, clip, seconds):
    return client.call({"op": "pose", "animation": clip_sample(clip, seconds)})


def field(client, clip, seconds):
    result = client.call({"op": "sample", "id": "actor/head", "points": POINTS,
                          "animation": clip_sample(clip, seconds)})
    return [v["value"] for v in result["samples"]]


def profile():
    return {"poses": [{"shape": shape, "values": [{"type": "face", "face": "face", "channel": channel,
                       "value": value} for channel, value in zip(CHANNELS, values)]} for shape, values in POSES.items()]}


def write_wave(path, rate, channels, pcm):
    with wave.open(str(path), "wb") as output:
        output.setnchannels(channels)
        output.setsampwidth(2)
        output.setframerate(rate)
        output.writeframes(pcm)


def analyze(client, audio, directory, **options):
    before = client.call({"op": "get_document"})
    revision = client.revision
    result = client.call({"op": "analyze_speech", "request": {
        "audio": audio, "directory": directory, "recognizer": "english", "timeout_seconds": 120, **options}})
    assert client.revision == revision and client.call({"op": "get_document"}) == before
    return result


def generate(client, request, dry_run=False, expect_ok=True):
    return client.call({"op": "generate_lip_sync", "request": request, "dry_run": dry_run}, True, expect_ok)


def setup(client):
    operations = source_operations()
    original = next(o["clip"] for o in operations if o["op"] == "put_clip" and o["clip"]["id"] == "dialogue")
    operations = [o for o in operations if o["op"] != "put_clip"]
    blink = {"id": "blink", "duration": 3, "face_tracks": [t for t in original["face_tracks"]
              if t["channel"].startswith("blink")]}
    body = {"id": "body", "duration": 3, "camera_keys": original["camera_keys"],
            "tracks": [{"target": {"type": "object", "id": "actor/head"}, "keys": [
                {"time": 0}, {"time": 1.5, "rotation_degrees": [0, 8, 0]}, {"time": 3}]}]}
    for shape, values in POSES.items():
        operations.append({"op": "put_clip", "clip": {"id": "calibration-" + shape, "duration": 3,
            "face_tracks": [{"face": "face", "channel": channel, "keys": [{"time": 0, "value": value}]}
                            for channel, value in zip(CHANNELS, values)]}})
    operations += [{"op": "put_clip", "clip": c} for c in (body, blink)]
    operations.append({"op": "put_clip", "clip": {"id": "body-blink", "duration": 3,
        "layers": [{"id": "body", "clip": "body"}, {"id": "blink", "clip": "blink"}]}})
    client.apply(operations)


def verify_cues(client, cues):
    references = {shape: field(client, "calibration-" + shape, 0) for shape in POSES}
    assert references["D"] != references["A"], "profile must change real head cavity geometry"
    checked = []
    for cue in cues:
        seconds, shape = cue["start_cs"] / 100, cue["shape"]
        state = face_state(client, "mouth", seconds)
        for channel, expected in zip(CHANNELS, POSES[shape]):
            assert abs(state["controls"][channel] - expected) < 1e-6, (cue, state)
        actual = field(client, "mouth", seconds)
        assert max(abs(a-b) for a, b in zip(actual, references[shape])) < 2e-6, cue
        checked.append({"time": seconds, "shape": shape, "controls": state["controls"],
                        "cavity_open": state["mouth"]["cavity_open"], "field_sha256":
                        hashlib.sha256(json.dumps(actual).encode()).hexdigest()})
    assert len({v["field_sha256"] for v in checked}) > 1, "speech mapping produced no changing mouth geometry"
    return checked


def run(executable, backend, directory, transcript):
    source = pcm16(FIXTURE)
    provenance = json.loads(PROVENANCE.read_text())
    original_hash = sha(FIXTURE)
    assert original_hash == provenance["sha256"] and source["channels"] == 1
    evidence = directory / "evidence"
    evidence.mkdir()
    shutil.copyfile(FIXTURE, evidence / "original-voice.wav")
    shutil.copyfile(PROVENANCE, evidence / "original-voice-provenance.json")
    shutil.copyfile(FIXTURE, directory / "voice.wav")
    write_wave(directory / "silence.wav", source["rate"], 1, bytes(len(source["pcm"])))
    stereo = b"".join(struct.pack("<hh", 1234 if i % 2 else -2345, value)
                      for i, value in enumerate(source["values"]))
    write_wave(directory / "stereo.wav", source["rate"], 2, stereo)
    client = Client(executable, directory, transcript, "speech-author", extra_args=["--speech-backend", backend])
    try:
        configured = client.call({"op": "speech_backend_state"})
        assert configured["configured"] and configured["backend"]["version"] == "1.14.0"
        setup(client)
        for name in ("voice", "silence", "stereo"):
            client.call({"op": "import_audio", "request": {"id": name, "path": name + ".wav"}}, True)
        before_audio = copy.deepcopy(client.call({"op": "get_document"})["audio"])
        analysis = analyze(client, "voice", "spoken", dialogue_hint=provenance["text"])
        assert analysis["source"]["source_sha256"] == original_hash
        assert analysis["analysis_sha256"] == sha(directory / "spoken/analysis.json")
        spoken_pcm = pcm16(directory / "spoken/input.wav")
        assert spoken_pcm["pcm"] == source["pcm"] and spoken_pcm["rate"] == source["rate"]
        silence = analyze(client, "silence", "silent", dialogue_hint=provenance["text"])
        assert all(c["shape"] == "X" for c in silence["cues"]), silence
        assert silence["source"]["range"]["frame_count"] == source["frames"]
        assert pcm16(directory / "silent/input.wav")["pcm"] == bytes(len(source["pcm"]))
        assert analysis["cues"] != silence["cues"]
        rejected = client.call({"op": "analyze_speech", "request": {
            "audio": "stereo", "directory": "missing-channel"}}, expect_ok=False)
        assert "channel" in rejected["error"]["message"] and not (directory / "missing-channel").exists()
        selected_range = {"start_sample": 317, "frame_count": 32000}
        selected = analyze(client, "stereo", "selected", range=selected_range, channel=1, recognizer="phonetic")
        prepared = pcm16(directory / "selected/input.wav")
        assert prepared["channels"] == 1 and prepared["rate"] == source["rate"] and prepared["frames"] == 32000
        assert prepared["pcm"] == source["pcm"][317*2:(317+32000)*2]
        assert selected["source"]["channel"] == 1 and selected["source"]["range"] == selected_range
        for name, result in (("spoken", analysis), ("silent", silence), ("selected", selected)):
            report = json.loads((directory / name / "analysis.json").read_text())
            raw = json.loads((directory / name / "raw.json").read_text())
            assert report["raw_output"] == raw and report["cues"] == result["cues"]
            assert result["raw_file_sha256"] == sha(directory / name / "raw.json")
            assert report["input_wav_sha256"] == sha(directory / name / "input.wav")
            assert report["backend"] == configured["backend"]
            assert (directory / name / "status.jsonl").stat().st_size > 0
            assert [{"start_cs": round(c["start"] * 100), "end_cs": round(c["end"] * 100),
                     "shape": c["value"]} for c in raw["mouthCues"]] == result["cues"]
        assert client.call({"op": "get_document"})["audio"] == before_audio
        assert sha(FIXTURE) == original_hash and sha(directory / "voice.wav") == original_hash
        print("[speech] Real spoken/silence analyses and selected stereo PCM match original source evidence", flush=True)

        request = {"analysis_path": "spoken/analysis.json", "clip": "mouth", "profile": profile(),
                   "transition_seconds": .04}
        before = client.call({"op": "get_document"})
        revision = client.revision
        preview = generate(client, request, dry_run=True)
        assert preview["committed"] is False and client.revision == revision
        assert client.call({"op": "get_document"}) == before
        generated = generate(client, request)
        assert generated["committed"] and generated["edited"] is False
        after = client.call({"op": "get_document"})
        client.call({"op": "undo"}, True)
        assert client.call({"op": "get_document"}) == before
        client.call({"op": "redo"}, True)
        assert client.call({"op": "get_document"}) == after
        generate(client, request, expect_ok=False)
        assert client.call({"op": "get_document"}) == after
        replaced = generate(client, dict(request, replace=True))
        assert replaced["edited"] is False and client.call({"op": "get_document"}) == after
        checked_cues = verify_cues(client, analysis["cues"])
        first_cue = checked_cues[0]
        different_cue = next(c for c in checked_cues if c["field_sha256"] != first_cue["field_sha256"])
        for name, cue in (("mouth-reference", first_cue), ("mouth-changed", different_cue)):
            client.call({"op": "render", "path": name + ".png", "animation": clip_sample("mouth", cue["time"])})
        mouth_pixels = pixel_difference(png_pixels(directory / "mouth-reference.png")[0],
                                        png_pixels(directory / "mouth-changed.png")[0])
        assert mouth_pixels > 0, "generated mouth curves did not change pixels without body/blink motion"
        client.apply([{"op": "put_clip", "clip": {"id": "performance", "duration": 3,
            "layers": [{"id": "body", "clip": "body"}, {"id": "blink", "clip": "blink"},
                       {"id": "mouth", "clip": "mouth"}]}}])
        for seconds in (0, .75, 1, 1.5, 2.5):
            base, complete = pose(client, "body-blink", seconds), pose(client, "performance", seconds)
            assert base["objects"] == complete["objects"] and base["camera"] == complete["camera"]
            a, b = face_state(client, "body-blink", seconds), face_state(client, "performance", seconds)
            for channel in ("blink_left", "blink_right"):
                assert a["controls"][channel] == b["controls"][channel]

        # An explicit ordinary source-curve edit must retain its recognition evidence
        # and become visibly marked as edited; independent body/blink clips survive.
        document = client.call({"op": "get_document"})
        mouth = next(c for c in document["clips"] if c["id"] == "mouth")
        track = copy.deepcopy(next(t for t in mouth["face_tracks"] if t["channel"] == "jaw_open"))
        for key in track["keys"]:
            key["value"] *= .8
        client.apply([{"op": "edit_curve", "request": {"clip": "mouth", "edit": {"op": "upsert_face", "track": track}}}])
        edited = client.call({"op": "lip_sync_state", "clip": "mouth"})
        assert edited["edited"] and edited["provenance"] == generated["provenance"]
        edited_document = client.call({"op": "get_document"})
        for identifier in ("body", "blink"):
            assert next(c for c in edited_document["clips"] if c["id"] == identifier) == next(c for c in document["clips"] if c["id"] == identifier)
        client.call({"op": "undo"}, True)
        assert client.call({"op": "lip_sync_state", "clip": "mouth"})["edited"] is False
        client.call({"op": "redo"}, True)
        assert client.call({"op": "lip_sync_state", "clip": "mouth"})["edited"]
        # Restore the explicit generated profile before delivery, retaining the edit
        # and history evidence in the transcript rather than claiming it was never made.
        generate(client, dict(request, replace=True))
        shot = {"id": "spoken-shot", "clip": "performance", "rate": {"numerator": 10, "denominator": 1},
                "start_frame": 1001, "frame_count": 29, "clip_frame_zero": 1001,
                "audio": {"asset": "voice", "start_sample": 0}}
        client.apply([{"op": "put_shot", "shot": shot}])
        client.call({"op": "render_shot", "request": {"shot": shot["id"], "directory": "shot", "format": "png"}})
        audio = verify_wave(directory / "shot/audio.wav", source, 0, 46400)
        manifest = json.loads((directory / "shot/manifest.json").read_text())
        assert len(manifest["frames"]) == 29
        for index in (0, 10, 20, 28):
            client.call({"op": "render", "path": f"direct-{index}.png", "animation": clip_sample("performance", index / 10)})
            assert (directory / f"direct-{index}.png").read_bytes() == (directory / f"shot/frame_{index:04}.png").read_bytes()
        changed_pixels = pixel_difference(png_pixels(directory / "shot/frame_0000.png")[0],
                                         png_pixels(directory / "shot/frame_0010.png")[0])
        assert changed_pixels > 0
        final_document = client.call({"op": "get_document"})
        final_state = client.call({"op": "lip_sync_state", "clip": "mouth"})
        final_pose = pose(client, "performance", 1)
        final_field = field(client, "mouth", 1)
        client.call({"op": "save", "path": "speech-project.json"})
        for name in ("voice", "silence", "stereo"):
            (directory / (name + ".wav")).rename(evidence / (name + "-removed-input.wav"))
        for name in ("spoken", "silent", "selected"):
            (directory / name).rename(evidence / (name + "-removed-analysis"))
        client.close()
        # The new process has neither an analysis backend option nor the original
        # input/report paths. Retained native curves must still render normally.
        client = Client(executable, directory, transcript, "speech-cold-without-backend")
        assert client.call({"op": "speech_backend_state"})["configured"] is False
        client.call({"op": "load", "path": "speech-project.json"}, True)
        assert client.call({"op": "get_document"}) == final_document
        assert client.call({"op": "lip_sync_state", "clip": "mouth"}) == final_state
        assert pose(client, "performance", 1) == final_pose and field(client, "mouth", 1) == final_field
        client.call({"op": "render", "path": "cold.png", "animation": clip_sample("performance", 1)})
        assert (directory / "cold.png").read_bytes() == (directory / "direct-10.png").read_bytes()
        client.call({"op": "export_audio", "request": {"id": "voice", "path": "cold-audio.wav",
                     "range": {"start_sample": 0, "frame_count": source["frames"]}}})
        verify_wave(directory / "cold-audio.wav", source, 0, source["frames"])
        unavailable = client.call({"op": "analyze_speech", "request": {"audio": "voice", "directory": "unavailable"}}, expect_ok=False)
        assert unavailable["error"]["code"] == "speech_backend_unavailable" and not (directory / "unavailable").exists()
        print("[speech] Generated curves, independent layers, history, shot PCM and cold rendering passed", flush=True)
        return {"backend": configured["backend"], "original_source_sha256": original_hash,
                "spoken_cues": analysis["cues"], "silence_cues": silence["cues"],
                "selected_stereo_range": selected_range, "selected_channel": 1,
                "cue_boundary_checks": checked_cues, "source_audio_unchanged": True,
                "dry_run_undo_redo_explicit_replace": True, "edited_provenance_observed": True,
                "independent_body_blink_preserved": True, "shot_frames": 29, "shot_audio": audio,
                "mouth_only_changed_pixels": mouth_pixels, "rendered_changed_pixels": changed_pixels,
                "cold_without_backend_or_external_reports": True}
    finally:
        client.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--editor", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--backend", type=Path, default=ROOT / ".dependencies/rhubarb-installer-https-verified/rhubarb")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    directory = args.output.resolve()
    directory.mkdir(parents=True, exist_ok=False)
    executable = directory / ("mm3e-editor.exe" if args.editor.suffix == ".exe" else "mm3e-editor")
    shutil.copyfile(args.editor.resolve(strict=True), executable)
    executable.chmod(0o755)
    backend = args.backend.resolve(strict=True)
    report = {"status": "running", "editor_sha256": sha(executable), "backend_binary_sha256": sha(backend),
              "harness_sha256": sha(Path(__file__)), "profile": profile(),
              "scope": "Real local recognition and editable procedural rig mapping with source PCM preservation; not forced alignment, measured recognition accuracy, acting approval or anatomical calibration."}
    started = time.monotonic()
    try:
        with (directory / "transcript.jsonl").open("w") as transcript:
            report["checks"] = run(executable, backend, directory, transcript)
        assert sha(executable) == report["editor_sha256"] and sha(backend) == report["backend_binary_sha256"]
        report["status"] = "passed"
    except Exception as error:
        report.update(status="failed", error=str(error), traceback=traceback.format_exc())
        raise
    finally:
        report["elapsed_seconds"] = time.monotonic() - started
        report["artifact_sha256"] = {path.relative_to(directory).as_posix(): sha(path)
                                     for path in sorted(directory.rglob("*"))
                                     if path.is_file() and path.name != "acceptance.json"}
        write_json(directory / "acceptance.json", report)
    print(json.dumps({"status": report["status"], "elapsed_seconds": report["elapsed_seconds"]}, indent=2))


if __name__ == "__main__":
    main()
