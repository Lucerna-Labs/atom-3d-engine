#!/usr/bin/env python3
"""Independent real-process acceptance for resumable native render jobs.

Retains the exact executable, frozen job files, dispatch/response transcript, encoded
frame hashes, failure/corruption evidence and a final report in a new output directory.
Tests ordinary rendering against resumed output, cancellation during an active step,
an actual killed worker, source isolation, and fractional-shot PCM sample windows.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import queue
import shutil
import struct
import time
import traceback
import wave

from agent_animation_acceptance import frame_path, png_chunks, write_json
from agent_durable_acceptance import Client as ProcessClient
from agent_sewing_acceptance import png_pixels, pixel_difference

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "mm3e-editor/tests/fixtures/dialogue/hello-reference.wav"


def sha256(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


class Client(ProcessClient):
    """Queue-backed responses permit an in-flight step and a second control process."""
    def __init__(self, binary, root, name, transcript):
        super().__init__(binary, root, name, transcript)
        self.sequence = 0
        self.pending = None

    def dispatch(self, command, revision=None):
        assert self.pending is None, "one pending request per process"
        self.sequence += 1
        request = {"id": f"{self.name}-{self.sequence}", "command": command}
        if revision is not None:
            request["expected_revision"] = revision
        self.log("request", request)
        self.process.stdin.write(json.dumps(request) + "\n")
        self.process.stdin.flush()
        self.pending = (request, time.monotonic())

    def finish(self, expect_ok=True, timeout=120):
        request, started = self.pending
        line = self.responses.get(timeout=timeout)
        if line is None:
            raise RuntimeError(f"{self.name} exited during {request['id']}; inspect stderr")
        response = json.loads(line)
        self.log("response", {**response, "elapsed_seconds": time.monotonic() - started})
        self.pending = None
        assert response["id"] == request["id"], response
        if expect_ok is not None:
            assert response["ok"] is expect_ok, response
        self.revision = response["revision"]
        return response.get("result") if expect_ok is True else response

    def call(self, command, mutate=False, expect_ok=True, revision=None):
        self.dispatch(command, self.revision if mutate else revision)
        return self.finish(expect_ok)

    def apply(self, operations):
        return self.call({"op": "apply", "operations": operations}, mutate=True)


def source_operations(size=96, aa=1):
    return [
        {"op": "create", "object": {"id": "ball", "position": [-.4, 0, 0],
            "shape": {"type": "sphere", "radius": .3},
            "material": {"albedo": [.8, .08, .04], "roughness": .6}}},
        {"op": "put_clip", "clip": {"id": "motion", "duration": 2,
            "tracks": [{"target": {"type": "object", "id": "ball"}, "keys": [
                {"time": 0}, {"time": 2, "translation": [.8, 0, 0]}]}]}},
        {"op": "set_camera", "camera": {"eye": [0, 0, 3], "target": [0, 0, 0], "fov_degrees": 40}},
        {"op": "set_settings", "settings": {"width": size, "height": size, "quality": "preview",
            "spatial_aa": aa, "shadows": False, "ao": False}},
    ]


def sequence_request(directory, large=False):
    return {"type": "sequence", "directory": directory, "clip": "motion", "start": 0,
            "end": 1.625 if large else .75, "fps": 24 if large else 12, "format": "png", "pass": "beauty"}


def job_state(client, directory, verify=False):
    return client.call({"op": "render_job_state", "directory": directory, "verify_outputs": verify})


def prefix(root, directory, count):
    result = []
    for index in range(count):
        path = root / directory / f"frame_{index:04}.png"
        stat = path.stat()
        result.append({"index": index, "sha256": sha256(path), "bytes": stat.st_size,
                       "mtime_ns": stat.st_mtime_ns, "inode": stat.st_ino})
    return result


def same_prefix(root, directory, before):
    assert prefix(root, directory, len(before)) == before, "committed frames were rewritten or changed"


def step(client, directory, maximum):
    before = job_state(client, directory)
    revision = client.revision
    result = client.call({"op": "step_render_job", "directory": directory, "max_frames": maximum})
    assert client.revision == revision
    assert before["completed_frames"] <= result["completed_frames"] <= before["completed_frames"] + maximum
    assert result["total_frames"] == before["total_frames"]
    assert result["snapshot_sha256"] == before["snapshot_sha256"]
    assert result["binary_sha256"] == before["binary_sha256"]
    assert result["plan_sha256"] == before["plan_sha256"]
    return result


def drive(client, directory):
    for _ in range(100):
        before = job_state(client, directory)
        if before["status"] == "complete":
            return before
        assert before["status"] == "pending", before
        after = step(client, directory, 32)
        assert after["completed_frames"] > before["completed_frames"], "pending job made no progress"
    raise AssertionError("bounded job failed to complete")


def completed(root, client, directory, reference=None, expected_size=96):
    state = job_state(client, directory, True)
    assert state["status"] == "complete" and state["pending_frame"] is None
    assert state["completed_frames"] == state["total_frames"]
    manifest_path = frame_path(root, state["manifest_path"])
    assert manifest_path == root / directory / "manifest.json"
    manifest = json.loads(manifest_path.read_text())
    assert len(manifest["frames"]) == state["total_frames"]
    for index, frame in enumerate(manifest["frames"]):
        path = root / directory / f"frame_{index:04}.png"
        chunks = png_chunks(path)
        width, height = struct.unpack_from(">II", next(data for kind, data in chunks if kind == b"IHDR"))
        assert (width, height) == (expected_size, expected_size)
        if reference is not None:
            old = reference["frames"][index]
            assert path.read_bytes() == frame_path(root, old["path"]).read_bytes()
            for field in ("time", "scheduled_time", "rgba_fnv1a64", "linear_rgb_fnv1a64", "view"):
                assert field in frame and frame[field] == old[field], (field, frame, old)
            if "shot_index" in old:
                for field in ("shot_index", "shot_frame", "audio_window"):
                    assert frame[field] == old[field], (field, frame, old)
    before = prefix(root, directory, state["total_frames"])
    result = client.call({"op": "step_render_job", "directory": directory, "max_frames": 1}, expect_ok=None)
    assert not result["ok"] or result["result"]["status"] == "complete"
    assert job_state(client, directory, True)["status"] == "complete"
    same_prefix(root, directory, before)
    return manifest, {"state": state, "manifest_sha256": sha256(manifest_path), "frames": before}


def await_in_flight_progress(client, root, directory, minimum=1):
    """Observe durable state after a commit while that step still awaits its reply."""
    deadline = time.monotonic() + 30
    while time.monotonic() < deadline:
        assert client.process.poll() is None, "worker exited before a crash/cancel window"
        if not client.responses.empty():
            response = client.finish(expect_ok=None)
            raise AssertionError(f"worker replied before the intended in-flight checkpoint: {response}")
        try:
            state = json.loads((root / directory / "state.json").read_text())
            count = len(state["frames"])
            if minimum <= count < 32:
                client.log("observed-in-flight-checkpoint", {"directory": directory, "completed_frames": count,
                            "pending": state.get("pending"), "response_pending": True})
                return count
        except (FileNotFoundError, json.JSONDecodeError):
            pass
        time.sleep(.005)
    raise AssertionError("no bounded in-flight checkpoint observed")


def pcm(path):
    with wave.open(str(path), "rb") as stream:
        assert stream.getcomptype() == "NONE"
        metadata = (stream.getnchannels(), stream.getframerate(), stream.getsampwidth(), stream.getnframes())
        samples = stream.readframes(stream.getnframes())
    return metadata, samples


def assert_pcm_slice(path, reference, start, count):
    metadata, values = pcm(path)
    source_metadata, source = reference
    assert metadata == (*source_metadata[:3], count)
    width = source_metadata[0] * source_metadata[2]
    assert values == source[start * width:(start + count) * width]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--editor", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    source_binary = args.editor.resolve(strict=True)
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    binary = root / "tested-mm3e-editor"
    shutil.copy2(source_binary, binary)
    assert sha256(binary) == sha256(source_binary)
    transcript = (root / "transcript.jsonl").open("w")
    clients = []
    report = {"status": "running", "binary": str(binary), "binary_source": str(source_binary),
              "binary_sha256": sha256(binary), "script_sha256": sha256(Path(__file__).resolve()), "cases": {}}
    started = time.monotonic()

    def client(name, executable=binary):
        created = Client(executable, root, name, transcript)
        clients.append(created)
        return created

    def create(creator, request):
        before = creator.call({"op": "get_document"})
        revision = creator.revision
        state = creator.call({"op": "create_render_job", "request": request})
        assert state["status"] == "pending" and state["completed_frames"] == 0
        assert state["pending_frame"] is None and state["manifest_path"] is None
        assert state["source_revision"] == revision and state["binary_sha256"] == report["binary_sha256"]
        assert creator.revision == revision and creator.call({"op": "get_document"}) == before
        directory = root / request["directory"]
        assert (directory / "job.json").is_file() and (directory / "snapshot.json").is_file()
        assert not (directory / "manifest.json").exists()
        return state

    try:
        write_json(root / "acceptance.json", report)
        author = client("author")
        author.apply(source_operations())
        original = author.call({"op": "get_document"})
        ordinary = author.call({"op": "render_sequence", "request": {
            k: v for k, v in sequence_request("sequence-reference").items() if k != "type"}})
        reference = json.loads(frame_path(root, ordinary["manifest_path"]).read_text())
        initial = create(author, sequence_request("sequence-job"))
        assert initial["total_frames"] == 10
        for maximum in (0, 33):
            rejected = author.call({"op": "step_render_job", "directory": "sequence-job", "max_frames": maximum}, expect_ok=False)
            assert job_state(author, "sequence-job")["completed_frames"] == 0
            assert not (root / "sequence-job/frame_0000.png").exists()
            report["cases"][f"invalid-step-{maximum}"] = rejected["error"]
        assert step(author, "sequence-job", 1)["completed_frames"] == 1
        retained = prefix(root, "sequence-job", 1)
        author.apply([
            {"op": "update", "id": "ball", "patch": {"shape": {"type": "sphere", "radius": .15},
                "material": {"albedo": [.05, .8, .1], "roughness": .2}}},
            {"op": "set_camera", "camera": {"eye": [1, 0, 3], "target": [0, 0, 0], "fov_degrees": 55}},
        ])
        assert author.call({"op": "get_document"}) != original
        assert step(author, "sequence-job", 2)["completed_frames"] == 3
        cancelled = author.call({"op": "cancel_render_job", "directory": "sequence-job"})
        assert cancelled["status"] == "cancelled" and cancelled["completed_frames"] == 3
        stopped = author.call({"op": "step_render_job", "directory": "sequence-job", "max_frames": 1}, expect_ok=None)
        assert not stopped["ok"] or stopped["result"]["status"] == "cancelled"
        assert job_state(author, "sequence-job")["completed_frames"] == 3
        assert author.call({"op": "resume_render_job", "directory": "sequence-job"})["status"] == "pending"
        author.close()
        fresh = client("restart-empty-document")
        assert fresh.call({"op": "get_document"})["objects"] == []
        assert job_state(fresh, "sequence-job", True)["completed_frames"] == 3
        assert step(fresh, "sequence-job", 2)["completed_frames"] == 5
        drive(fresh, "sequence-job")
        _, report["cases"]["bounded_snapshot_restart"] = completed(root, fresh, "sequence-job", reference)
        same_prefix(root, "sequence-job", retained)
        assert fresh.call({"op": "get_document"})["objects"] == [] and fresh.revision == 0
        assert pixel_difference(png_pixels(root / "sequence-job/frame_0000.png")[0],
                                png_pixels(root / "sequence-job/frame_0009.png")[0]) > 0
        print("[render jobs] Bounded steps, live-edit isolation, cancel/resume and empty-process restart match reference", flush=True)

        # The fractional shot is resumed with no live source/audio document present.
        fresh.apply(source_operations())
        shutil.copy2(FIXTURE, root / "reference.wav")
        source_pcm = pcm(root / "reference.wav")
        fresh.call({"op": "import_audio", "request": {"id": "voice", "path": "reference.wav"}}, mutate=True)
        shot = {"id": "phase", "clip": "motion", "rate": {"numerator": 24000, "denominator": 1001},
                "start_frame": 1001, "frame_count": 12, "clip_frame_zero": 1001,
                "audio": {"asset": "voice", "start_sample": 317}}
        fresh.apply([{"op": "put_shot", "shot": shot}])
        selection = {"start_frame": 1002, "frame_count": 5}
        rendered = fresh.call({"op": "render_shot", "request": {"shot": "phase", "directory": "shot-reference",
                               "selection": selection, "format": "png"}})
        shot_reference = json.loads(frame_path(root, rendered["manifest_path"]).read_text())
        create(fresh, {"type": "shot", "directory": "shot-job", "shot": "phase", "selection": selection, "format": "png"})
        assert step(fresh, "shot-job", 1)["completed_frames"] == 1
        first_shot = prefix(root, "shot-job", 1)
        (root / "reference.wav").unlink()
        fresh.close()
        worker = client("shot-restart")
        assert worker.call({"op": "get_document"}).get("audio", []) == []
        drive(worker, "shot-job")
        manifest, report["cases"]["fractional_shot_phase"] = completed(root, worker, "shot-job", shot_reference)
        same_prefix(root, "shot-job", first_shot)
        assert (manifest["audio"]["start_sample"], manifest["audio"]["end_sample_exclusive"],
                manifest["audio"]["sample_frames"]) == (984, 4321, 3337)
        assert_pcm_slice(root / "shot-job/audio.wav", source_pcm, 984, 3337)
        assert (root / "shot-job/audio.wav").read_bytes() == (root / "shot-reference/audio.wav").read_bytes()
        assert 3337 != 5 * 16000 * 1001 // 24000, "fixture must detect a partial/resume phase reset"
        print("[render jobs] Fractional shot retains whole-shot audio phase and exact embedded PCM after restart", flush=True)

        # Larger snapshots keep a step in flight long enough to exercise independent
        # state and cancellation owners and a real kill between durable frame commits.
        worker.apply(source_operations(512, 2))
        controller = client("controller")
        create(worker, sequence_request("live-cancel", True))
        worker.dispatch({"op": "step_render_job", "directory": "live-cancel", "max_frames": 32})
        seen = await_in_flight_progress(worker, root, "live-cancel")
        locked = controller.call({"op": "step_render_job", "directory": "live-cancel", "max_frames": 1}, expect_ok=False)
        assert "lock" in json.dumps(locked["error"]).lower(), locked
        cancel_ack = controller.call({"op": "cancel_render_job", "directory": "live-cancel"})
        assert cancel_ack["status"] == "cancelled"
        stopped = worker.finish()
        assert stopped["status"] == "cancelled" and seen <= stopped["completed_frames"] < 32
        assert stopped["completed_frames"] <= cancel_ack["completed_frames"] + 1
        before_resume = prefix(root, "live-cancel", stopped["completed_frames"])
        assert controller.call({"op": "resume_render_job", "directory": "live-cancel"})["status"] == "pending"
        drive(controller, "live-cancel")
        _, evidence = completed(root, controller, "live-cancel", expected_size=512)
        same_prefix(root, "live-cancel", before_resume)
        report["cases"]["live_cancel_and_single_worker"] = {"observed_completed": seen, "cancel_ack": cancel_ack,
            "stopped": stopped, "competing_step_error": locked["error"], "completion": evidence}
        print("[render jobs] Concurrent step excluded; separate process cancels active step at frame boundary", flush=True)

        create(worker, sequence_request("killed-worker", True))
        worker.dispatch({"op": "step_render_job", "directory": "killed-worker", "max_frames": 32})
        seen = await_in_flight_progress(worker, root, "killed-worker")
        before_kill = prefix(root, "killed-worker", seen)
        worker.kill()
        after_kill = json.loads((root / "killed-worker/state.json").read_text())
        persisted = len(after_kill["frames"])
        assert seen <= persisted < 32
        resurrected = client("after-real-kill")
        assert resurrected.call({"op": "get_document"})["objects"] == []
        resumed = resurrected.call({"op": "resume_render_job", "directory": "killed-worker"})
        assert resumed["status"] == "pending" and resumed["completed_frames"] >= persisted
        drive(resurrected, "killed-worker")
        _, completion = completed(root, resurrected, "killed-worker", expected_size=512)
        same_prefix(root, "killed-worker", before_kill)
        report["cases"]["actual_worker_kill"] = {"exit_code": worker.process.returncode, "observed_completed": seen,
            "persisted_completed_after_kill": persisted, "pending_after_kill": after_kill.get("pending"),
            "resumed": resumed, "completion": completion}
        print("[render jobs] Real killed worker recovered; committed prefix hashes, mtimes and inodes retained", flush=True)

        resurrected.apply(source_operations())
        unowned = root / "unowned-job"
        unowned.mkdir()
        (unowned / "preserve.txt").write_bytes(b"preexisting output must remain untouched")
        rejected = resurrected.call({"op": "create_render_job", "request": sequence_request("unowned-job")}, expect_ok=False)
        assert sorted(p.name for p in unowned.iterdir()) == ["preserve.txt"]
        assert (unowned / "preserve.txt").read_bytes() == b"preexisting output must remain untouched"
        report["cases"]["unowned_directory_rejection"] = rejected["error"]

        create(resurrected, sequence_request("frame-collision"))
        collision = root / "frame-collision/frame_0000.png"
        collision.write_bytes(b"foreign frame collision evidence")
        rejected = resurrected.call({"op": "step_render_job", "directory": "frame-collision", "max_frames": 1}, expect_ok=False)
        assert collision.read_bytes() == b"foreign frame collision evidence"
        assert job_state(resurrected, "frame-collision")["completed_frames"] == 0
        assert not (root / "frame-collision/manifest.json").exists()
        report["cases"]["frame_collision"] = rejected["error"]

        create(resurrected, sequence_request("frame-corruption"))
        assert step(resurrected, "frame-corruption", 1)["completed_frames"] == 1
        corrupted = root / "frame-corruption/frame_0000.png"
        shutil.copy2(corrupted, root / "original-before-corruption.png")
        damaged = bytearray(corrupted.read_bytes())
        damaged[-8] ^= 1
        corrupted.write_bytes(damaged)
        detected = resurrected.call({"op": "render_job_state", "directory": "frame-corruption", "verify_outputs": True}, expect_ok=False)
        rejected = resurrected.call({"op": "step_render_job", "directory": "frame-corruption", "max_frames": 1}, expect_ok=False)
        assert corrupted.read_bytes() == damaged and not (root / "frame-corruption/frame_0001.png").exists()
        report["cases"]["completed_frame_corruption"] = {"inspection": detected["error"], "step": rejected["error"]}

        create(resurrected, sequence_request("snapshot-corruption"))
        snapshot = root / "snapshot-corruption/snapshot.json"
        shutil.copy2(snapshot, root / "original-before-corruption.json")
        snapshot.write_bytes(snapshot.read_bytes() + b"\ncorruption evidence\n")
        rejected = resurrected.call({"op": "step_render_job", "directory": "snapshot-corruption", "max_frames": 1}, expect_ok=False)
        assert snapshot.read_bytes().endswith(b"corruption evidence\n")
        assert not (root / "snapshot-corruption/frame_0000.png").exists()
        report["cases"]["snapshot_corruption"] = rejected["error"]

        # Appending inert bytes preserves this ELF program but changes its identity;
        # a second process must not append images to a job frozen for another build.
        create(resurrected, sequence_request("binary-mismatch"))
        changed_binary = root / "different-identity-mm3e-editor"
        shutil.copy2(binary, changed_binary)
        with changed_binary.open("ab") as stream:
            stream.write(b"\nrender-job binary identity negative control\n")
        assert sha256(changed_binary) != report["binary_sha256"]
        other_build = client("different-binary", changed_binary)
        rejected = other_build.call({"op": "step_render_job", "directory": "binary-mismatch", "max_frames": 1}, expect_ok=False)
        assert not (root / "binary-mismatch/frame_0000.png").exists()
        report["cases"]["binary_identity_mismatch"] = rejected["error"]
        assert job_state(resurrected, "sequence-job", True)["status"] == "complete"
        report["status"] = "passed"
        print("[render jobs] Collisions, corruption and mismatched executable rejected without overwriting evidence", flush=True)
    except Exception as error:
        report.update({"status": "failed", "error": str(error), "traceback": traceback.format_exc()})
        raise
    finally:
        for running in clients:
            if running.process.poll() is None and running.pending is not None:
                running.kill()
            else:
                running.close()
        transcript.close()
        report["elapsed_seconds"] = time.monotonic() - started
        report["binary_sha256_after"] = sha256(binary)
        write_json(root / "acceptance.json", report)
    print(json.dumps({"status": report["status"], "elapsed_seconds": report["elapsed_seconds"],
                      "report": str(root / "acceptance.json")}, indent=2))


if __name__ == "__main__":
    main()
