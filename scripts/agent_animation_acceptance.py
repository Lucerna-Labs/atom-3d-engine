#!/usr/bin/env python3
"""Verify persistent character animation through the actual editor JSONL process.

Creates a fresh artifact directory, preserves request/response and failure evidence, checks
articulated geometry rather than metadata alone, and exports playback from rendered PNGs.
This deterministic client does not benchmark autonomous LLM planning or anatomical quality.
"""
import argparse
import copy
import hashlib
import json
import math
from pathlib import Path
import struct
import subprocess
import time
import traceback
import zlib


class Client:
    def __init__(self, executable, root, transcript, session, extra_args=()):
        self.stderr = (root / f"{session}-stderr.log").open("w+")
        self.process = subprocess.Popen(
            [str(executable), "--root", str(root), *map(str, extra_args)], stdin=subprocess.PIPE,
            stdout=subprocess.PIPE, stderr=self.stderr, text=True,
        )
        self.revision = 0
        self.sequence = 0
        self.transcript = transcript
        self.session = session

    def call(self, command, mutate=False, expect_ok=True):
        self.sequence += 1
        request = {"id": f"{self.session}-{self.sequence}", "command": command}
        if mutate:
            request["expected_revision"] = self.revision
        # Record dispatch separately so a crash or interrupted render retains its request.
        self.transcript.write(json.dumps({"session": self.session, "request": request}) + "\n")
        self.transcript.flush()
        self.process.stdin.write(json.dumps(request) + "\n")
        self.process.stdin.flush()
        started = time.monotonic()
        line = self.process.stdout.readline()
        if not line:
            raise RuntimeError(f"editor exited during {request['id']}; see {self.session}-stderr.log")
        response = json.loads(line)
        self.transcript.write(json.dumps({"session": self.session, "response": response,
                                          "elapsed_seconds": time.monotonic() - started}) + "\n")
        self.transcript.flush()
        assert response["id"] == request["id"], response
        assert response["ok"] == expect_ok, response
        self.revision = response["revision"]
        return response.get("result") if expect_ok else response

    def apply(self, operations, dry_run=False, expect_ok=True):
        return self.call({"op": "apply", "operations": operations, "dry_run": dry_run}, True, expect_ok)

    def close(self):
        if self.process.poll() is None:
            self.process.stdin.close()
            self.process.wait(timeout=30)
        self.stderr.close()
        assert self.process.returncode == 0, f"editor exited {self.process.returncode}"

    def abort(self):
        if self.process.poll() is None:
            self.process.kill()
            self.process.wait(timeout=30)
        self.stderr.close()


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n")


def distance(a, b):
    return math.sqrt(sum((x - y) ** 2 for x, y in zip(a, b)))


def by_id(items):
    return {item["id"]: item for item in items}


def frame_path(root, value):
    path = Path(value)
    if not path.is_absolute():
        path = root / path
    path = path.resolve(strict=True)
    assert path.is_relative_to(root), f"frame escapes artifact root: {path}"
    return path


def png_chunks(path):
    data = path.read_bytes()
    assert data[:8] == b"\x89PNG\r\n\x1a\n", f"not a PNG: {path}"
    offset = 8
    result = []
    while offset < len(data):
        length = struct.unpack_from(">I", data, offset)[0]
        kind = data[offset + 4:offset + 8]
        payload = data[offset + 8:offset + 8 + length]
        crc = struct.unpack_from(">I", data, offset + 8 + length)[0]
        assert zlib.crc32(kind + payload) & 0xffffffff == crc, f"PNG CRC mismatch: {path}"
        result.append((kind, payload))
        offset += length + 12
        if kind == b"IEND":
            break
    assert offset == len(data), f"trailing or truncated PNG data: {path}"
    return result


def chunk(kind, payload):
    return struct.pack(">I", len(payload)) + kind + payload + struct.pack(">I", zlib.crc32(kind + payload) & 0xffffffff)


def make_playback(paths, output, fps):
    """Losslessly package actual RGBA PNG frame streams into a looping animated PNG.

    No resampling, synthesized frames, image library or external encoder is involved.
    The inclusive duplicate final endpoint is omitted for an evenly timed loop.
    """
    sequence = 0
    data = bytearray(b"\x89PNG\r\n\x1a\n")
    expected_header = None
    for index, path in enumerate(paths):
        parts = png_chunks(path)
        header = next(payload for kind, payload in parts if kind == b"IHDR")
        width, height, depth, color, compression, filtering, interlace = struct.unpack(">IIBBBBB", header)
        assert (depth, color, compression, filtering, interlace) == (8, 6, 0, 0, 0)
        if index == 0:
            expected_header = header
            data.extend(chunk(b"IHDR", header))
            data.extend(chunk(b"acTL", struct.pack(">II", len(paths), 0)))
        else:
            assert header == expected_header, "all playback frame dimensions must match"
        data.extend(chunk(b"fcTL", struct.pack(">IIIIIHHBB", sequence, width, height, 0, 0, 1, fps, 0, 0)))
        sequence += 1
        for kind, payload in parts:
            if kind != b"IDAT":
                continue
            if index == 0:
                data.extend(chunk(b"IDAT", payload))
            else:
                data.extend(chunk(b"fdAT", struct.pack(">I", sequence) + payload))
                sequence += 1
    data.extend(chunk(b"IEND", b""))
    output.write_bytes(data)
    animation = dict(png_chunks(output))
    assert struct.unpack(">II", animation[b"acTL"]) == (len(paths), 0)
    return {"path": str(output), "format": "animated_png", "frame_count": len(paths),
            "fps": fps, "duration_seconds": len(paths) / fps,
            "sha256": hashlib.sha256(data).hexdigest(), "source": "lossless engine-rendered PNG frame streams"}


def wave_clip():
    def joint(name, keys):
        return {"target": {"type": "joint", "id": f"hero/{name}"}, "easing": "smooth_step", "keys": keys}

    def rotate(time_value, z):
        return {"time": time_value, "rotation_degrees": [0, 0, z]}

    return {"id": "wave", "duration": 2.0, "tracks": [
        joint("root", [{"time": 0}, {"time": 1, "translation": [0.10, 0, 0]}, {"time": 2}]),
        joint("left_shoulder", [rotate(0, 0), rotate(0.45, 95), rotate(1.5, 95), rotate(2, 0)]),
        joint("left_elbow", [rotate(0, 0), rotate(0.45, 40), rotate(0.8, 75), rotate(1.15, 25),
                             rotate(1.5, 60), rotate(2, 0)]),
        joint("left_wrist", [rotate(0, 0), rotate(0.45, -20), rotate(0.8, 30), rotate(1.15, -25),
                             rotate(1.5, 25), rotate(2, 0)]),
        joint("head", [{"time": 0}, {"time": 1, "rotation_degrees": [0, 12, 0]}, {"time": 2}]),
    ]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    workspace = Path(__file__).resolve().parents[1]
    parser.add_argument("--editor", type=Path, default=workspace / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    executable = args.editor.resolve(strict=True)
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    started = time.monotonic()
    result = {"status": "running", "kind": "real JSONL process animation acceptance", "backend": "MM3E CPU",
              "root": str(root), "executable": str(executable)}
    result_path = root / "acceptance.json"
    write_json(result_path, result)
    client = None
    try:
        with (root / "transcript.jsonl").open("w") as transcript:
            client = Client(executable, root, transcript, "author")
            write_json(root / "capabilities.json", client.call({"op": "describe"}))
            clip = wave_clip()
            client.apply([
                {"op": "create_humanoid", "id": "hero", "height": 1.8, "build": 1.0, "head_scale": 1.1},
                {"op": "create", "object": {"id": "pedestal", "shape": {
                    "type": "cylinder", "half_height": 0.025, "radius": 0.65}, "position": [0, -0.025, 0],
                    "material": {"albedo": [0.07, 0.11, 0.14], "roughness": 0.8}}},
                {"op": "rig_humanoid", "id": "hero"},
                {"op": "put_clip", "clip": clip},
                {"op": "set_settings", "settings": {"width": 192, "height": 256, "quality": "preview",
                    "exposure": 1.0, "shadows": False, "ao": False}},
                {"op": "set_camera", "camera": {"eye": [2.0, 1.7, 4.8], "target": [0.05, 1.0, 0],
                    "fov_degrees": 31}},
            ])
            authored = client.call({"op": "get_document"})
            assert len(authored["objects"]) == 20
            assert len(authored["joints"]) == 16
            assert len(authored["clips"]) == 1
            client.call({"op": "validate"})
            revision = client.revision

            invalid = copy.deepcopy(clip)
            invalid["tracks"][0]["target"]["id"] = "missing-joint"
            client.apply([{"op": "translate", "ids": ["pedestal"], "delta": [1, 0, 0]},
                          {"op": "put_clip", "clip": invalid}], expect_ok=False)
            assert client.revision == revision
            assert client.call({"op": "get_document"}) == authored
            variant = copy.deepcopy(clip)
            variant["id"] = "wave-variant"
            client.apply([{"op": "put_clip", "clip": variant}], dry_run=True)
            assert client.revision == revision
            assert client.call({"op": "get_document"}) == authored
            client.apply([{"op": "put_clip", "clip": variant}])
            client.call({"op": "undo"}, True)
            assert client.call({"op": "get_document"}) == authored
            # Keep redo pending throughout all scrubbing/rendering observations.
            observation_revision = client.revision
            history_before = client.call({"op": "inspect"})

            def animation(t, playback="clamp"):
                return {"clip": "wave", "time": t, "playback": playback}

            def pose(t, playback="clamp"):
                return client.call({"op": "pose", "animation": animation(t, playback)})

            poses = {str(t): pose(t) for t in (0, 0.5, 1, 1.5, 2)}
            rest_objects, rest_joints = by_id(poses["0"]["objects"]), by_id(poses["0"]["joints"])
            moved_objects, moved_joints = by_id(poses["1"]["objects"]), by_id(poses["1"]["joints"])
            wrist_travel = distance(rest_joints["hero/left_wrist"]["world_pivot"],
                                    moved_joints["hero/left_wrist"]["world_pivot"])
            elbow_travel = distance(rest_joints["hero/left_elbow"]["world_pivot"],
                                    moved_joints["hero/left_elbow"]["world_pivot"])
            hand_travel = distance(rest_objects["hero/left_hand"]["position"], moved_objects["hero/left_hand"]["position"])
            assert wrist_travel > 0.3, wrist_travel
            assert elbow_travel > 0.2, elbow_travel
            assert hand_travel > 0.3, hand_travel
            assert abs(distance(rest_joints["hero/root"]["world_pivot"], moved_joints["hero/root"]["world_pivot"]) - 0.10) < 1e-5
            assert rest_objects["pedestal"] == moved_objects["pedestal"]
            assert rest_objects["hero/left_hand"]["basis"] != moved_objects["hero/left_hand"]["basis"]
            assert poses["0"]["objects"] == poses["2"]["objects"]
            assert pose(1) == poses["1"], "sampling order changed the evaluated pose"
            assert pose(3)["objects"] == poses["2"]["objects"], "clamp endpoint differs"
            assert pose(3, "loop")["objects"] == poses["1"]["objects"], "loop does not repeat"
            assert pose(2, "loop")["objects"] == poses["0"]["objects"], "exact loop endpoint differs"
            write_json(root / "poses.json", poses)

            # The evaluated hand center must be inside the real animated field, and outside
            # its rest field. This detects a pose readback that was never wired into geometry.
            point = moved_objects["hero/left_hand"]["position"]
            sample_animated = client.call({"op": "sample", "id": "hero/left_hand", "points": [point],
                                          "animation": animation(1)})
            sample_rest = client.call({"op": "sample", "id": "hero/left_hand", "points": [point]})
            assert sample_animated["samples"][0]["value"] < -0.005, sample_animated
            assert sample_rest["samples"][0]["value"] > 0.10, sample_rest
            picked = client.call({"op": "pick", "x": 96, "y": 128, "animation": animation(1)})
            assert picked["status"] == "hit", picked

            renders = {}
            for name, t in (("rest", 0), ("wave", 1), ("return", 2)):
                renders[name] = client.call({"op": "render", "path": f"{name}.png", "animation": animation(t)})
                assert renders[name]["metrics"]["primary_center_ray_hits"] > 0
            assert renders["rest"]["rgba_fnv1a64"] != renders["wave"]["rgba_fnv1a64"]
            assert renders["rest"]["rgba_fnv1a64"] == renders["return"]["rgba_fnv1a64"]
            sequence = client.call({"op": "render_sequence", "request": {
                "directory": "frames", "clip": "wave", "start": 0, "end": 2, "fps": 12}})
            manifest_path = frame_path(root, sequence["manifest_path"])
            manifest = json.loads(manifest_path.read_text())
            frames = manifest["frames"]
            assert len(frames) == 25, "inclusive interval must produce 25 frames"
            for i, frame in enumerate(frames):
                assert abs(frame["time"] - i / 12) < 1e-5, frame
            frame_paths = [frame_path(root, frame["path"]) for frame in frames]
            fingerprints = {frame["rgba_fnv1a64"] for frame in frames}
            assert len(fingerprints) >= 15, "expected many distinct rendered motion frames"
            assert frames[0]["rgba_fnv1a64"] == renders["rest"]["rgba_fnv1a64"]
            assert frames[12]["rgba_fnv1a64"] == renders["wave"]["rgba_fnv1a64"]
            assert frames[24]["rgba_fnv1a64"] == renders["return"]["rgba_fnv1a64"]
            assert frame_paths[12].read_bytes() == (root / "wave.png").read_bytes()
            for path in frame_paths:
                png_chunks(path)
            # Existing sequence output must not be silently overwritten on a second request.
            manifest_bytes = manifest_path.read_bytes()
            client.call({"op": "render_sequence", "request": {
                "directory": "frames", "clip": "wave", "start": 0, "end": 2, "fps": 12}}, expect_ok=False)
            assert manifest_path.read_bytes() == manifest_bytes
            assert client.revision == observation_revision
            assert client.call({"op": "get_document"}) == authored
            history_after = client.call({"op": "inspect"})
            for field in ("undo_available", "redo_available"):
                assert history_after[field] == history_before[field]
            client.call({"op": "redo"}, True)
            assert len(client.call({"op": "get_document"})["clips"]) == 2
            client.call({"op": "undo"}, True)
            assert client.call({"op": "get_document"}) == authored
            client.call({"op": "save", "path": "hero-animation.mm3e-agent.json"})
            saved_revision = client.revision
            client.close()
            client = None

            client = Client(executable, root, transcript, "reopen")
            client.call({"op": "load", "path": "hero-animation.mm3e-agent.json"}, True)
            assert client.call({"op": "get_document"}) == authored
            assert client.call({"op": "pose", "animation": animation(1)}) == poses["1"]
            reopened = client.call({"op": "render", "path": "reopened-wave.png", "animation": animation(1)})
            assert reopened["rgba_fnv1a64"] == renders["wave"]["rgba_fnv1a64"]
            assert (root / "reopened-wave.png").read_bytes() == (root / "wave.png").read_bytes()
            assert client.revision == 1
            client.close()
            client = None

            playback = make_playback(frame_paths[:-1], root / "wave-playback.png", 12)
            (root / "playback.html").write_text("""<!doctype html>
<meta charset="utf-8"><title>MM3E character animation acceptance</title>
<style>body{background:#111a23;color:#d6e2ea;font:16px system-ui;margin:2rem}
img{width:384px;height:512px;max-width:90vw;object-fit:contain;border:1px solid #365}</style>
<h1>Character wave</h1><p>24 actual engine-rendered frames at 12 fps, looping.</p>
<img src="wave-playback.png" alt="Animated rigged character waving">
<p><a href="acceptance.json">Acceptance evidence</a> · <a href="frames/manifest.json">Frame manifest</a></p>
""")
            result.update({"status": "passed", "character_parts": 19, "entity_count": 20, "joint_count": 16,
                "clip_count": 1, "duration_seconds": 2, "fps": 12, "frame_count": len(frames),
                "distinct_frame_fingerprints": len(fingerprints), "manifest_path": str(manifest_path),
                "motion": {"left_wrist_travel_m": wrist_travel, "left_elbow_travel_m": elbow_travel,
                           "left_hand_travel_m": hand_travel, "unbound_pedestal_unchanged": True,
                           "animated_hand_sample": sample_animated, "rest_hand_sample": sample_rest},
                "atomic_rejection": True, "dry_run_unchanged": True, "scrub_and_render_document_unchanged": True,
                "scrub_and_render_preserved_redo": True, "clamp_and_loop_verified": True,
                "sequence_collision_preserved_output": True, "fresh_process_reload_pose_identical": True,
                "fresh_process_reload_pixel_identical": True, "saved_revision": saved_revision,
                "renders": renders, "playback": playback,
                "not_evaluated": ["autonomous LLM planning", "anatomical quality", "skinning", "inverse kinematics",
                                  "physics", "collision avoidance", "GPU rendering", "real-time performance"],
                "elapsed_seconds": time.monotonic() - started})
    except Exception as error:
        result.update({"status": "failed", "error": str(error), "traceback": traceback.format_exc(),
                       "elapsed_seconds": time.monotonic() - started})
        raise
    finally:
        if client is not None:
            client.abort()
        write_json(result_path, result)
    print(json.dumps({key: result[key] for key in ("status", "root", "joint_count", "frame_count", "elapsed_seconds")}, indent=2))


if __name__ == "__main__":
    main()
