#!/usr/bin/env python3
"""Exercise durable commits, writer exclusion and crash recovery through the real CLI."""

import argparse
import hashlib
import json
from pathlib import Path
import queue
import subprocess
import struct
import threading
import time


ROOT = Path(__file__).resolve().parents[1]


class Client:
    def __init__(self, binary, root, name, transcript, project=None):
        self.name, self.transcript = name, transcript
        self.revision = 0
        self.stderr = (root / f"{name}.stderr.txt").open("w")
        command = [str(binary), "--root", str(root)]
        if project is not None:
            command.extend(["--project", project])
        self.process = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=self.stderr, text=True, bufsize=1)
        self.responses = queue.Queue()
        self.reader = threading.Thread(target=self._read, daemon=True)
        self.reader.start()

    def _read(self):
        for line in self.process.stdout:
            self.responses.put(line)
        self.responses.put(None)

    def log(self, event, value):
        self.transcript.write(json.dumps({"process": self.name, "pid": self.process.pid, "event": event, "value": value}) + "\n")
        self.transcript.flush()

    def send(self, identifier, command, revision=None, okay=True):
        request = {"id": identifier, "command": command}
        if revision is not None:
            request["expected_revision"] = revision
        self.log("request", request)
        self.process.stdin.write(json.dumps(request) + "\n")
        self.process.stdin.flush()
        line = self.responses.get(timeout=30)
        if line is None:
            raise RuntimeError(f"{self.name} exited before responding; inspect its stderr file")
        response = json.loads(line)
        self.log("response", response)
        assert response["id"] == identifier, response
        assert response["ok"] is okay, response
        self.revision = response["revision"]
        return response

    def kill(self):
        if self.process.poll() is None:
            self.process.kill()
        self.process.wait(timeout=10)
        self.reader.join(timeout=10)
        self.process.stdin.close()
        self.process.stdout.close()
        self.stderr.close()
        self.log("killed", {"returncode": self.process.returncode})

    def close(self):
        if self.process.poll() is None:
            self.process.stdin.close()
            try:
                self.process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                self.process.kill()
                self.process.wait(timeout=10)
        self.reader.join(timeout=10)
        self.process.stdout.close()
        self.stderr.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    output = args.output.resolve()
    output.mkdir(parents=False, exist_ok=False)
    started = time.perf_counter()
    clients = []
    project_file = output / "durable.json"
    transcript = (output / "transcript.jsonl").open("w")

    def client(name, project=None):
        value = Client(binary, output, name, transcript, project)
        clients.append(value)
        return value

    def document(value):
        return authoring_value(value.send("document", {"op": "get_document"})["result"])

    def authoring_value(value):
        # Native documents serialize f32 directly, while JSON Value observations can print
        # their exact f64 promotion. Compare exact authored f32 bits, not decimal formatting.
        if isinstance(value, float):
            return struct.unpack("<f", struct.pack("<f", value))[0]
        if isinstance(value, dict):
            return {key: authoring_value(item) for key, item in value.items()}
        if isinstance(value, list):
            return [authoring_value(item) for item in value]
        return value

    def assert_disk(value, expected):
        saved = json.loads(project_file.read_text())
        assert saved["saved_revision"] == value.revision
        assert authoring_value(saved["document"]) == expected

    try:
        primary = client("primary", "durable.json")
        initial = primary.send("initial", {"op": "inspect"})
        assert initial["revision"] == 0 and project_file.is_file()
        primary.send("author", {"op": "apply", "operations": [
            {"op": "set_settings", "settings": {"width": 48, "height": 48, "quality": "preview", "shadows": False, "ao": False}},
            {"op": "create", "object": {"id": "ball", "shape": {"type": "sphere", "radius": 0.2}, "position": [0, 0.7, 0]}},
        ]}, 0)
        baseline = document(primary)
        assert_disk(primary, baseline)
        baseline_bytes = project_file.read_bytes()
        primary.send("dry-run", {"op": "apply", "dry_run": True, "operations": [{"op": "translate", "ids": ["ball"], "delta": [1, 0, 0]}]}, 1)
        primary.send("invalid", {"op": "apply", "operations": [
            {"op": "translate", "ids": ["ball"], "delta": [2, 0, 0]},
            {"op": "update", "id": "ball", "patch": {"shape": {"type": "sphere", "radius": -1}}},
        ]}, 1, okay=False)
        assert primary.revision == 1 and document(primary) == baseline
        assert project_file.read_bytes() == baseline_bytes

        competing = subprocess.run([str(binary), "--root", str(output), "--project", "durable.json"], input="", text=True, capture_output=True, timeout=10)
        (output / "competing-startup.stderr.txt").write_text(competing.stderr)
        primary.log("competing-startup", {"returncode": competing.returncode, "stdout": competing.stdout, "stderr": competing.stderr})
        assert competing.returncode != 0 and "already open" in competing.stderr
        transient = client("transient")
        conflict = transient.send("locked-save", {"op": "save", "path": "durable.json", "overwrite": True}, okay=False)
        assert conflict["error"]["code"] == "project_locked"
        assert project_file.read_bytes() == baseline_bytes
        transient.close()

        primary.send("undo", {"op": "undo"}, 1)
        assert json.loads(project_file.read_text())["document"]["objects"] == []
        primary.send("redo", {"op": "redo"}, 2)
        primary.send("move", {"op": "apply", "operations": [{"op": "translate", "ids": ["ball"], "delta": [0.5, 0, 0]}]}, 3)
        committed = document(primary)
        assert primary.revision == 4
        assert_disk(primary, committed)
        field = primary.send("field-before", {"op": "sample", "id": "ball", "points": [[0.5, 0.7, 0]]})["result"]["samples"][0]["value"]
        assert field < 0
        before = primary.send("before", {"op": "render", "path": "before.png"})["result"]["rgba_fnv1a64"]
        primary.kill()

        reopened = client("reopened", "durable.json")
        inspected = reopened.send("reopened-inspect", {"op": "inspect"})
        assert inspected["revision"] == 4
        assert inspected["result"]["undo_available"] == 0 and inspected["result"]["redo_available"] == 0
        assert document(reopened) == committed
        restored_field = reopened.send("field-after", {"op": "sample", "id": "ball", "points": [[0.5, 0.7, 0]]})["result"]["samples"][0]["value"]
        assert restored_field == field
        after = reopened.send("after", {"op": "render", "path": "after.png"})["result"]["rgba_fnv1a64"]
        assert before == after
        current_bytes = project_file.read_bytes()
        stale = reopened.send("stale", {"op": "apply", "operations": [{"op": "translate", "ids": ["ball"], "delta": [1, 0, 0]}]}, 0, okay=False)
        assert stale["error"]["code"] == "revision_conflict" and reopened.revision == 4
        assert project_file.read_bytes() == current_bytes
        reopened.send("same-store-save", {"op": "save", "path": "durable.json", "overwrite": True})
        reopened.close()

        # A new transient process has made no storage writes: its first staging sequence is 0.
        collision = client("temp-collision")
        old_target = output / "collision-target.json"
        old_target.write_bytes(b"previous destination remains intact")
        remnant = output / f".mm3e-{collision.process.pid}-0.tmp"
        remnant.write_bytes(b"pre-existing crash evidence must survive")
        failed = collision.send("collision-save", {"op": "save", "path": old_target.name, "overwrite": True}, okay=False)
        assert failed["error"]["code"] == "io"
        assert remnant.read_bytes() == b"pre-existing crash evidence must survive"
        assert old_target.read_bytes() == b"previous destination remains intact"
        collision.send("save-after-collision", {"op": "save", "path": "after-collision.json"})
        collision.close()

        # Reopening an existing persistent project performs no write before its first mutation.
        failed_commit = client("failed-commit", "durable.json")
        failed_commit.send("ready", {"op": "inspect"})
        preserved = project_file.read_bytes()
        commit_remnant = output / f".mm3e-{failed_commit.process.pid}-0.tmp"
        commit_remnant.write_bytes(b"persistent pre-existing staging evidence")
        response = failed_commit.send("failed-mutation", {"op": "apply", "operations": [{"op": "translate", "ids": ["ball"], "delta": [9, 0, 0]}]}, 4, okay=False)
        assert response["error"]["code"] == "io" and failed_commit.revision == 4
        assert project_file.read_bytes() == preserved and document(failed_commit) == committed
        assert commit_remnant.read_bytes() == b"persistent pre-existing staging evidence"
        failed_commit.send("successful-mutation", {"op": "apply", "operations": [{"op": "translate", "ids": ["ball"], "delta": [0.1, 0, 0]}]}, 4)
        final_document = document(failed_commit)
        assert_disk(failed_commit, final_document)
        assert failed_commit.revision == 5
        failed_commit.kill()
        final = client("final-reopen", "durable.json")
        final.send("ready", {"op": "inspect"})
        assert final.revision == 5 and document(final) == final_document
        final.close()
        summary = {
            "passed": True, "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
            "restored_revision": 5, "image_rgba_fnv1a64": before,
            "checks": ["mutations durable before success response", "dry run and invalid transaction preserve file and live document", "concurrent persistent startup rejected", "transient Save cannot bypass held lock", "undo/redo persist with increasing revisions", "forced process termination restores revision and geometry", "restored scene reproduces image pixels", "stale revision rejected after restart", "same-store save reuses held lock", "pre-existing temporary file and old destination preserved on save failure", "failed persistent write preserves live state and disk", "following successful mutation survives a second forced termination"],
            "elapsed_seconds": time.perf_counter() - started,
            "not_verified": ["power-loss durability on every filesystem", "network filesystem lock guarantees", "persistent undo history", "post-rename directory-fsync failure injection", "hostile concurrent filesystem mutation"],
        }
        (output / "acceptance.json").write_text(json.dumps(summary, indent=2) + "\n")
        print(json.dumps(summary, indent=2))
    except Exception as error:
        (output / "failure.json").write_text(json.dumps({"passed": False, "error": repr(error), "elapsed_seconds": time.perf_counter() - started}, indent=2) + "\n")
        raise
    finally:
        for value in clients:
            value.close()
        transcript.close()


if __name__ == "__main__":
    main()
