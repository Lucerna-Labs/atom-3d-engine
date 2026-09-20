#!/usr/bin/env python3
"""Real-process native-project admission acceptance at the actual 64 MiB boundary.

Builds valid indexed surfaces whose compact input fits under 64 MiB while their
canonical pretty Saved representation crosses that limit. Baseline mode preserves
the old accepted-but-unsaveable failure; fixed mode requires atomic rejection of
Load, Apply and dry-run candidates, then verifies successful save/reload/history.
All output directories must be new. No project implementation is imported or mocked.
"""
import argparse
import copy
import hashlib
import json
import os
from pathlib import Path
import select
import subprocess
import time
import traceback

ROOT = Path(__file__).resolve().parents[1]
LIMIT = 64 * 1024 * 1024
REQUEST_LIMIT = 4 * 1024 * 1024


def sha256(path):
    value = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            value.update(block)
    return value.hexdigest()


def document_digest(path):
    """Hash canonical document bytes, excluding only the changing Saved revision."""
    with path.open("rb") as source:
        first = source.read(4096)
        marker = b'\n  "document": '
        index = first.index(marker) + len(marker)
        digest = hashlib.sha256(first[index:])
        for block in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2, allow_nan=False) + "\n")


class Client:
    def __init__(self, executable, root, transcript, name, project=None):
        self.name, self.transcript, self.revision, self.sequence = name, transcript, 0, 0
        self.stderr = (root / (name + ".stderr.log")).open("w")
        command = [str(executable), "--root", str(root)]
        if project:
            command += ["--project", project]
        self.process = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                        stderr=self.stderr, text=True)
        self.last_request_bytes = 0
        self.call({"op": "inspect"})

    def call(self, command, mutate=False, okay=True):
        self.sequence += 1
        request = {"id": f"{self.name}-{self.sequence}", "command": command}
        if mutate:
            request["expected_revision"] = self.revision
        encoded = json.dumps(request, separators=(",", ":"), allow_nan=False) + "\n"
        self.last_request_bytes = len(encoded.encode())
        assert self.last_request_bytes <= REQUEST_LIMIT, self.last_request_bytes
        self.transcript.write(json.dumps({"request": request, "wire_bytes": self.last_request_bytes},
                                        separators=(",", ":")) + "\n")
        self.transcript.flush()
        started = time.monotonic()
        self.process.stdin.write(encoded)
        self.process.stdin.flush()
        if not select.select([self.process.stdout], [], [], 120)[0]:
            raise TimeoutError(f"No reply within 120 seconds: {request['id']}")
        line = self.process.stdout.readline()
        if not line:
            raise RuntimeError(f"Editor exited during {request['id']}")
        response = json.loads(line)
        self.transcript.write(json.dumps({"response": response, "elapsed_seconds": time.monotonic() - started},
                                        separators=(",", ":")) + "\n")
        self.transcript.flush()
        assert response["id"] == request["id"], response
        assert response["ok"] is okay, response
        self.revision = response["revision"]
        return response["result"] if okay else response

    def close(self):
        self.process.stdin.close()
        self.process.wait(timeout=30)
        self.stderr.close()
        assert self.process.returncode == 0, self.process.returncode

    def abort(self):
        if self.process.poll() is None:
            self.process.kill()
            self.process.wait(timeout=30)
        self.stderr.close()


def grid(side):
    points = [[float(x), float(y), 0.0] for y in range(side) for x in range(side)]
    triangles = []
    for y in range(side - 1):
        for x in range(side - 1):
            a = y * side + x
            b, c, d = a + 1, a + side, a + side + 1
            triangles.extend([[a, b, c], [b, d, c]])
    return {"type": "surface", "vertices": points, "triangles": triangles, "thickness_m": 0.01}


def entity(template, index, shape):
    result = copy.deepcopy(template)
    result.update({"id": f"surface-{index:03}", "shape": shape, "scale": 0.01})
    return result


def embedded_pretty_size(item):
    # Objects occur at indentation six in the canonical Saved.document.objects array.
    encoded = json.dumps(item, indent=2, allow_nan=False)
    return len(encoded.encode()) + 6 * (encoded.count("\n") + 1)


def fixtures(executable, root, transcript, reuse=None):
    directory = root / "fixtures"
    directory.mkdir()
    if reuse is not None:
        for name in ["valid-compact.json", "oversized-compact.json", "metadata.json"]:
            source = reuse / name
            try:
                os.link(source, directory / name)
            except OSError:
                import shutil
                shutil.copyfile(source, directory / name)
        metadata = json.loads((directory / "metadata.json").read_text())
        assert sha256(directory / "valid-compact.json") == metadata["valid_compact_sha256"]
        assert sha256(directory / "oversized-compact.json") == metadata["oversized_compact_sha256"]
        return metadata, grid(256)
    bootstrap = Client(executable, root, transcript, "template")
    try:
        bootstrap.call({"op": "apply", "operations": [
            {"op": "create", "object": {"id": "seed", "shape": {"type": "sphere", "radius": 0.1}}},
            {"op": "set_settings", "settings": {"width": 32, "height": 32, "quality": "preview",
                                                  "shadows": False, "ao": False, "spatial_aa": 1}},
            {"op": "set_camera", "camera": {"eye": [1.2, 1.2, 3], "target": [1.2, 1.2, 0], "fov_degrees": 45}}
        ]}, mutate=True)
        bootstrap.call({"op": "save", "path": "template.json"})
    finally:
        bootstrap.close()
    saved = json.loads((root / "template.json").read_text())
    saved["saved_revision"] = 1
    template = saved["document"]["objects"][0]
    saved["document"]["objects"] = []
    base = len(json.dumps(saved, indent=2, allow_nan=False).encode())
    full = grid(256)
    full_entities = [entity(template, index, full) for index in range(4)]
    three_cost = sum(embedded_pretty_size(item) for item in full_entities[:3])
    target = LIMIT - 128 * 1024
    low, high, selected, selected_size = 2, 256, None, None
    while low <= high:
        side = (low + high) // 2
        candidate = entity(template, 3, grid(side))
        predicted = base + three_cost + embedded_pretty_size(candidate) + 2 * 3 + 6
        if predicted <= target:
            selected, selected_size = side, predicted
            low = side + 1
        else:
            high = side - 1
    assert selected is not None and selected < 256
    good_entities = full_entities[:3] + [entity(template, 3, grid(selected))]
    metadata = {"limit_bytes": LIMIT, "full_grid_side": 256, "valid_final_grid_side": selected,
        "vertices_per_full_surface": 65536, "triangles_per_full_surface": 130050,
        "valid_predicted_pretty_bytes_at_revision_1": selected_size,
        "oversized_predicted_pretty_bytes_at_revision_1": base + sum(map(embedded_pretty_size, full_entities)) + 12,
        "fixture_semantics": "Four valid independent planar indexed surfaces; no geometry caps or numeric ranges bypassed"}
    for name, objects in [("valid-compact.json", good_entities), ("oversized-compact.json", full_entities)]:
        saved["document"]["objects"] = objects
        with (directory / name).open("w") as stream:
            json.dump(saved, stream, separators=(",", ":"), allow_nan=False)
        assert (directory / name).stat().st_size < LIMIT
        prefix = "valid" if name.startswith("valid-") else "oversized"
        metadata[prefix + "_compact_bytes"] = (directory / name).stat().st_size
        metadata[prefix + "_compact_sha256"] = sha256(directory / name)
    assert metadata["oversized_predicted_pretty_bytes_at_revision_1"] > LIMIT
    write_json(directory / "metadata.json", metadata)
    return metadata, full


def oversized_edit(shape, dry=False):
    return {"op": "apply", "operations": [{"op": "update", "id": "surface-003", "patch": {"shape": shape}}],
            "dry_run": dry}


def small_edit(label, dry=False):
    return {"op": "apply", "operations": [{"op": "update", "id": "surface-000", "patch": {"label": label}}],
            "dry_run": dry}


def budget(client):
    result = client.call({"op": "project_budget"})
    assert result["limit_bytes"] == LIMIT
    assert 0 <= result["revision_reserve_bytes"] <= 19
    assert result["encoded_bytes"] + result["revision_reserve_bytes"] + result["remaining_bytes"] == LIMIT
    return result


def snapshot(client):
    observed = client.call({"op": "inspect"})
    return {"revision": client.revision, "inspect": observed, "budget": budget(client)}


def save(client, root, name):
    result = client.call({"op": "save", "path": name})
    path = root / name
    assert result["bytes"] == path.stat().st_size <= LIMIT
    return {"path": str(path), "bytes": path.stat().st_size, "sha256": sha256(path), "document_sha256": document_digest(path)}


def baseline(executable, root, transcript, metadata, shape, report):
    client = Client(executable, root, transcript, "baseline")
    try:
        client.call({"op": "load", "path": "fixtures/valid-compact.json"}, mutate=True)
        last_valid = save(client, root, "last-valid.json")
        assert LIMIT - 1024 * 1024 < last_valid["bytes"] < LIMIT
        assert last_valid["bytes"] == metadata["valid_predicted_pretty_bytes_at_revision_1"]
        revision = client.revision
        accepted = client.call({"op": "load", "path": "fixtures/oversized-compact.json"}, mutate=True)
        assert client.revision == revision + 1 and accepted["object_count"] == 4
        client.call({"op": "validate"})
        failed = client.call({"op": "save", "path": "last-valid.json", "overwrite": True}, okay=False)
        assert "64MiB" in failed["error"]["message"] or "64 MiB" in failed["error"]["message"]
        assert sha256(root / "last-valid.json") == last_valid["sha256"]
        client.call({"op": "undo"}, mutate=True)
        accepted_edit = client.call(oversized_edit(shape), mutate=True)
        wire_bytes = client.last_request_bytes
        assert accepted_edit["committed"] is True and wire_bytes < REQUEST_LIMIT
        failed_edit = client.call({"op": "save", "path": "edit-must-not-exist.json"}, okay=False)
        assert not (root / "edit-must-not-exist.json").exists()
        report.update({"status": "baseline_bug_reproduced", "fixed": False, "last_valid_save": last_valid,
            "oversized_compact_load_was_accepted": accepted, "save_after_accepted_load_failed": failed["error"],
            "last_valid_file_preserved": True, "oversized_edit_wire_bytes": wire_bytes,
            "oversized_edit_was_accepted": True, "save_after_accepted_edit_failed": failed_edit["error"]})
    finally:
        client.close()


def fixed(executable, root, transcript, metadata, shape, report):
    client = Client(executable, root, transcript, "transient")
    try:
        client.call({"op": "load", "path": "fixtures/valid-compact.json"}, mutate=True)
        client.call(small_edit("redo witness"), mutate=True)
        client.call({"op": "undo"}, mutate=True)
        before = snapshot(client)
        assert before["inspect"]["redo_available"] == 1
        assert 0 < before["budget"]["remaining_bytes"] < 1024 * 1024
        last_valid = save(client, root, "last-valid.json")
        assert last_valid["bytes"] == before["budget"]["encoded_bytes"]
        assert last_valid["bytes"] == metadata["valid_predicted_pretty_bytes_at_revision_1"]
        rejections = []
        for command in [
            {"op": "load", "path": "fixtures/oversized-compact.json"},
            oversized_edit(shape, True), oversized_edit(shape),
        ]:
            failed = client.call(command, mutate=True, okay=False)
            assert failed["error"]["code"] == "project_size_limit", failed
            rejections.append({"op": command["op"], "dry_run": command.get("dry_run", False),
                               "wire_bytes": client.last_request_bytes, "error": failed["error"]})
            assert snapshot(client) == before
            assert sha256(root / "last-valid.json") == last_valid["sha256"]
        preview = client.call(small_edit("valid dry-run" , True), mutate=True)
        assert preview["committed"] is False
        assert snapshot(client) == before
        after_rejection = save(client, root, "after-rejections.json")
        assert after_rejection["sha256"] == last_valid["sha256"]
        client.call({"op": "redo"}, mutate=True)
        edited = save(client, root, "redone.json")
        assert edited["document_sha256"] != last_valid["document_sha256"]
        client.call({"op": "undo"}, mutate=True)
        restored = save(client, root, "undone.json")
        assert restored["document_sha256"] == last_valid["document_sha256"]
        client.call(small_edit("accepted and durable"), mutate=True)
        accepted = save(client, root, "accepted.json")
        accepted_budget = budget(client)
        assert accepted_budget["encoded_bytes"] == accepted["bytes"]
        first_render = client.call({"op": "render", "path": "accepted.png"})
        client.close()
        client = Client(executable, root, transcript, "transient-reopen")
        client.call({"op": "load", "path": "accepted.json"}, mutate=True)
        reloaded = save(client, root, "reloaded.json")
        assert reloaded["document_sha256"] == accepted["document_sha256"]
        second_render = client.call({"op": "render", "path": "reloaded.png"})
        assert first_render["rgba_fnv1a64"] == second_render["rgba_fnv1a64"]
        assert (root / "accepted.png").read_bytes() == (root / "reloaded.png").read_bytes()
        client.close()
        client = Client(executable, root, transcript, "persistent", "durable.json")
        client.call({"op": "load", "path": "accepted.json"}, mutate=True)
        durable = snapshot(client)
        durable_hash = sha256(root / "durable.json")
        assert document_digest(root / "durable.json") == accepted["document_sha256"]
        for command in [{"op": "load", "path": "fixtures/oversized-compact.json"}, oversized_edit(shape, True), oversized_edit(shape)]:
            failed = client.call(command, mutate=True, okay=False)
            assert failed["error"]["code"] == "project_size_limit"
            assert snapshot(client) == durable
            assert sha256(root / "durable.json") == durable_hash
        client.close()
        client = Client(executable, root, transcript, "persistent-reopen", "durable.json")
        assert client.revision == durable["revision"]
        assert budget(client) == durable["budget"]
        assert sha256(root / "durable.json") == durable_hash
        persistent_render = client.call({"op": "render", "path": "durable-reopened.png"})
        assert persistent_render["rgba_fnv1a64"] == first_render["rgba_fnv1a64"]
        assert (root / "durable-reopened.png").read_bytes() == (root / "accepted.png").read_bytes()
        report.update({"status": "passed", "fixed": True, "last_valid_save": last_valid,
            "project_budget_near_limit": before["budget"], "transient_rejections": rejections,
            "transient_failed_candidates_preserve_revision_history_and_complete_saved_document": True,
            "admissible_dry_run_preserves_state": True, "undo_redo_preserve_complete_document": True,
            "accepted_save": accepted, "accepted_budget": accepted_budget,
            "transient_save_reload_document_and_png_exact": True,
            "persistent_rejections_preserve_durable_bytes_revision_and_history": True,
            "persistent_cold_reload_document_budget_and_png_exact": True})
    finally:
        client.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--editor", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--mode", choices=["baseline", "fixed"], default="fixed")
    parser.add_argument("--fixtures", type=Path, help="Reuse immutable fixtures from a prior run (linked/copied inside this editor root)")
    args = parser.parse_args()
    executable = args.editor.resolve(strict=True)
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    started = time.monotonic()
    report = {"status": "running", "mode": args.mode, "binary": str(executable), "binary_sha256": sha256(executable),
              "harness_sha256": sha256(Path(__file__)), "root": str(root), "limit_bytes": LIMIT}
    report_path = root / "acceptance.json"
    write_json(report_path, report)
    try:
        with (root / "transcript.jsonl").open("w") as transcript:
            metadata, shape = fixtures(executable, root, transcript, args.fixtures.resolve() if args.fixtures else None)
            report["fixture"] = metadata
            write_json(report_path, report)
            print(json.dumps({"fixture": metadata}), flush=True)
            if args.mode == "baseline":
                baseline(executable, root, transcript, metadata, shape, report)
            else:
                fixed(executable, root, transcript, metadata, shape, report)
            assert sha256(executable) == report["binary_sha256"]
    except Exception as error:
        report.update({"status": "failed", "error": str(error), "traceback": traceback.format_exc()})
        raise
    finally:
        report["elapsed_seconds"] = time.monotonic() - started
        write_json(report_path, report)
    print(json.dumps({key: report[key] for key in ["status", "mode", "elapsed_seconds"]}, indent=2))


if __name__ == "__main__":
    main()
