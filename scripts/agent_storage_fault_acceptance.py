#!/usr/bin/env python3
"""Linux real-executable storage error injection and restart acceptance.

Compiles a test-only LD_PRELOAD interposer, scoped to newly created case directories
and explicitly armed around one editor operation. All real syscall results and
injected returns are logged separately. This does not simulate physical power loss.
Baseline mode records the old phase-classification gaps; fixed mode requires typed
visibility errors and poisoned-session recovery for uncertain persistent installs.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import secrets
import select
import shutil
import subprocess
import time
import traceback

ROOT = Path(__file__).resolve().parents[1]
CASES = [
    ("control-commit", "none", False),
    ("control-no-clobber", "none", True),
    ("staged-write-enospc", "write_enospc", False),
    ("file-sync-eio", "file_sync_eio", False),
    ("directory-sync-eio", "dir_sync_eio", False),
    ("rename-before-eio", "rename_before", False),
    ("rename-after-eio", "rename_after", False),
    ("no-clobber-cleanup-eio", "cleanup_unlink", True),
]


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2, allow_nan=False) + "\n")


class Client:
    def __init__(self, executable, root, transcript, name, env=None):
        self.name, self.transcript, self.revision, self.sequence = name, transcript, 0, 0
        self.stderr = (root / (name + ".stderr.log")).open("w")
        self.process = subprocess.Popen([str(executable), "--root", str(root), "--project", "project.json"],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=self.stderr, text=True, env=env)
        self.call({"op": "inspect"})

    def call(self, command, mutate=False, okay=True):
        self.sequence += 1
        request = {"id": f"{self.name}-{self.sequence}", "command": command}
        if mutate:
            request["expected_revision"] = self.revision
        self.transcript.write(json.dumps({"request": request}) + "\n")
        self.transcript.flush()
        started = time.monotonic()
        self.process.stdin.write(json.dumps(request) + "\n")
        self.process.stdin.flush()
        if not select.select([self.process.stdout], [], [], 30)[0]:
            raise TimeoutError(f"No editor reply within 30 seconds: {request['id']}")
        line = self.process.stdout.readline()
        if not line:
            raise RuntimeError(f"Editor exited during {request['id']}")
        response = json.loads(line)
        self.transcript.write(json.dumps({"response": response, "elapsed_seconds": time.monotonic() - started}) + "\n")
        self.transcript.flush()
        assert response["id"] == request["id"], response
        if okay is not None:
            assert response["ok"] is okay, response
        self.revision = response["revision"]
        return response["result"] if okay is True else response

    def close(self):
        if self.process.poll() is None:
            self.process.stdin.close()
            try:
                self.process.wait(timeout=15)
            except subprocess.TimeoutExpired:
                self.process.kill()
                self.process.wait(timeout=15)
        self.stderr.close()
        assert self.process.returncode == 0, self.process.returncode

    def abort(self):
        if self.process.poll() is None:
            self.process.kill()
            self.process.wait(timeout=15)
        self.stderr.close()


def animation():
    return {"clip": "motion", "time": 1}


def mutation():
    return {"op": "apply", "operations": [{"op": "translate", "ids": ["ball"], "delta": [0.9, 0, 0]}]}


def snapshot(client):
    observed = client.call({"op": "inspect"})
    return {"revision": client.revision, "document": client.call({"op": "get_document"}),
            "pose": client.call({"op": "pose", "animation": animation()}),
            "undo_available": observed["undo_available"], "redo_available": observed["redo_available"]}


def render(client, path):
    result = client.call({"op": "render", "path": path, "animation": animation()})
    assert result["metrics"]["visible_material_owner_pixels"].get("ball", 0) > 100
    return result


def seed(executable, root, transcript):
    client = Client(executable, root, transcript, "seed")
    try:
        client.call({"op": "apply", "operations": [
            {"op": "create", "object": {"id": "ball", "position": [-0.45, 0, 0],
                "shape": {"type": "sphere", "radius": 0.3}, "material": {"albedo": [0.64, 0.25, 0.08]}}},
            {"op": "put_clip", "clip": {"id": "motion", "duration": 1, "tracks": [
                {"target": {"type": "object", "id": "ball"}, "keys": [{"time": 0}, {"time": 1, "translation": [0, 0.15, 0]}]}]}},
            {"op": "set_camera", "camera": {"eye": [0, 0.2, 3], "target": [0, 0.15, 0], "fov_degrees": 40}},
            {"op": "set_settings", "settings": {"width": 96, "height": 64, "quality": "preview",
                "shadows": False, "ao": False, "spatial_aa": 1}},
        ]}, mutate=True)
    finally:
        client.close()


def expected_commit(executable, root, transcript):
    expected = root / "expected"
    expected.mkdir()
    shutil.copyfile(root / "project.json", expected / "project.json")
    client = Client(executable, expected, transcript, "expected-control")
    try:
        client.call(mutation(), mutate=True)
        state = snapshot(client)
        image = render(client, "expected.png")
        return {"state": state, "image": image, "image_path": expected / "expected.png",
                "project_sha256": sha256(expected / "project.json")}
    finally:
        client.close()


def read_events(root):
    return [json.loads(line) for line in (root / "fault-events.jsonl").read_text().splitlines()]


def verify_events(events, fault, output, pid, root):
    assert events and events[0]["stage"] == "loaded", "LD_PRELOAD interposer did not activate"
    assert all(event["pid"] == pid for event in events)
    active = [event for event in events if event["armed"]]
    injected = [event for event in active if event["injected"]]
    assert len(injected) == (0 if fault == "none" else 1), injected
    for event in active:
        assert event["path"] == str(root) or event["path"].startswith(str(root) + f"/.mm3e-{pid}-")
    stages = [event["stage"] for event in active]
    assert "staged_write" in stages
    if fault == "write_enospc":
        prefix = next(event for event in active if event["call"] == "write_short_prefix")
        assert prefix["syscall_ran"] and prefix["result"] == prefix["real_requested"] == 37
        assert prefix["requested"] > 37
        assert injected[0]["errno"] == 28 and injected[0]["stage"] == "staged_write"
        assert "file_sync" not in stages and "install_rename" not in stages
        assert any(event["stage"] == "staging_cleanup" and event["syscall_ran"] and event["result"] == 0 for event in active)
    elif fault == "file_sync_eio":
        assert injected[0]["errno"] == 5 and injected[0]["stage"] == "file_sync"
        assert "install_rename" not in stages
    elif fault == "dir_sync_eio":
        installed = next(event for event in active if event["stage"] == "install_rename")
        assert installed["syscall_ran"] and installed["result"] == 0
        assert injected[0]["stage"] == "directory_sync" and injected[0]["after_actual_success"]
        assert installed["seq"] < injected[0]["seq"]
    elif fault == "rename_before":
        assert injected[0]["stage"] == "install_rename" and not injected[0]["syscall_ran"]
        assert not any(event["stage"] == "install_rename" and event["syscall_ran"] for event in active)
    elif fault == "rename_after":
        installed = next(event for event in active if event["stage"] == "install_rename" and event["syscall_ran"])
        assert installed["result"] == 0 and installed["seq"] < injected[0]["seq"]
        assert injected[0]["after_actual_success"] and not injected[0]["syscall_ran"]
    elif fault == "cleanup_unlink":
        installed = next(event for event in active if event["stage"] == "install_link")
        assert installed["syscall_ran"] and installed["result"] == 0
        assert injected[0]["stage"] == "staging_cleanup" and installed["seq"] < injected[0]["seq"]
        assert any(event["stage"] == "staging_cleanup" and event["syscall_ran"] and event["result"] == 0 for event in active)
    else:
        install = "install_link" if output else "install_rename"
        assert install in stages and "directory_sync" in stages
        assert all(event["result"] >= 0 and event["errno"] == 0 for event in active)
    return {"armed_events": active, "injected_return_count": len(injected)}


def run_case(executable, library, output_root, name, fault, output_operation, mode, transcript):
    root = output_root / name
    root.mkdir()
    token = secrets.token_hex(16)
    (root / ".mm3e-fault-sandbox").write_text(token)
    foreign = root / ".mm3e-foreign.tmp"
    foreign.write_bytes(b"foreign file must survive every failure and cleanup")
    seed(executable, root, transcript)
    env = os.environ.copy()
    env.update({"LD_PRELOAD": str(library), "MM3E_STORAGE_FAULT_ROOT": str(root),
                "MM3E_STORAGE_FAULT_TARGET": "no-clobber.png" if output_operation else "project.json",
                "MM3E_STORAGE_FAULT_MODE": fault, "MM3E_STORAGE_FAULT_TOKEN": token})
    client = Client(executable, root, transcript, "fault-session", env)
    result = {"case": name, "fault": fault, "root": str(root), "pid": client.process.pid}
    try:
        loaded_maps = Path(f"/proc/{client.process.pid}/maps").read_text()
        assert str(library) in loaded_maps, "interposer is not mapped into the actual editor process"
        (root / "fault-session.maps").write_text(loaded_maps)
        client.call({"op": "apply", "operations": [{"op": "update", "id": "ball", "patch": {"label": "redo witness"}}]}, mutate=True)
        client.call({"op": "undo"}, mutate=True)
        before = snapshot(client)
        assert before["redo_available"] == 1
        before_image = render(client, "before.png")
        before_bytes = (root / "project.json").read_bytes()
        (root / "before-project.json").write_bytes(before_bytes)
        before_inode = (root / "project.json").stat().st_ino
        expected = None if output_operation else expected_commit(executable, root, transcript)
        (root / ".fault-armed").write_text(token)
        command = {"op": "render", "path": "no-clobber.png", "animation": animation()} if output_operation else mutation()
        response = client.call(command, mutate=not output_operation, okay=None)
        (root / ".fault-armed").unlink()
        events = read_events(root)
        result["events"] = verify_events(events, fault, output_operation, client.process.pid, root)
        result["response"] = response
        result["before_revision"] = before["revision"]
        result["before_project_sha256"] = hashlib.sha256(before_bytes).hexdigest()
        assert not list(root.glob(f".mm3e-{client.process.pid}-*.tmp")), "own staging file leaked"
        assert foreign.read_bytes() == b"foreign file must survive every failure and cleanup"
        installed = fault in {"dir_sync_eio", "rename_after"} or (fault == "none" and not output_operation)
        current = json.loads((root / "project.json").read_text())
        result["disk_revision_after_operation"] = current["saved_revision"]
        result["project_inode_changed"] = (root / "project.json").stat().st_ino != before_inode
        result["post_operation_project_sha256"] = sha256(root / "project.json")
        shutil.copyfile(root / "project.json", root / "after-operation-project.json")
        if output_operation:
            assert (root / "project.json").read_bytes() == before_bytes
            assert snapshot(client) == before
            assert (root / "no-clobber.png").read_bytes() == (root / "before.png").read_bytes()
            if fault == "none":
                assert response["ok"] is True
            else:
                assert response["ok"] is False
                assert response["error"]["code"] == ("output_installed_uncertain" if mode == "fixed" else "io")
            result["installed_output_bytes_match_normal_render"] = True
            recovered_expected = before
            recovered_image = root / "before.png"
        elif fault in {"write_enospc", "file_sync_eio"}:
            assert not response["ok"] and response["error"]["code"] == "io"
            assert response["revision"] == before["revision"]
            assert (root / "project.json").read_bytes() == before_bytes
            assert not result["project_inode_changed"]
            assert snapshot(client) == before
            render(client, "after-failure.png")
            assert (root / "after-failure.png").read_bytes() == (root / "before.png").read_bytes()
            result["failed_attempt_preserved_file_live_revision_history_pose_and_pixels"] = True
            client.call(mutation(), mutate=True)
            assert client.revision == before["revision"] + 1
            assert sha256(root / "project.json") == expected["project_sha256"]
            assert client.call({"op": "get_document"}) == expected["state"]["document"]
            result["successful_retry_after_disarming"] = True
            recovered_expected = expected["state"]
            recovered_image = expected["image_path"]
        elif fault == "none":
            assert response["ok"] and response["revision"] == before["revision"] + 1
            assert sha256(root / "project.json") == expected["project_sha256"]
            recovered_expected = expected["state"]
            recovered_image = expected["image_path"]
        else:
            assert not response["ok"] and response["revision"] == before["revision"]
            must_poison = mode == "fixed" or fault == "dir_sync_eio"
            assert response["error"]["code"] == ("commit_uncertain" if must_poison else "io")
            if must_poison:
                blocked = []
                for blocked_command, mutate in [({"op": "inspect"}, False), ({"op": "get_document"}, False),
                    ({"op": "project_budget"}, False), ({"op": "pose", "animation": animation()}, False),
                    ({"op": "render", "path": "blocked.png", "animation": animation()}, False),
                    ({"op": "undo"}, True), ({"op": "redo"}, True), (mutation(), True)]:
                    failure = client.call(blocked_command, mutate=mutate, okay=False)
                    assert failure["error"]["code"] == "recovery_required"
                    assert failure["revision"] == before["revision"]
                    blocked.append(blocked_command["op"])
                assert not (root / "blocked.png").exists()
                result["poisoned_commands_blocked"] = blocked
            else:
                # Preserve the baseline's stale live state without performing another write.
                assert snapshot(client) == before
                result["baseline_install_error_left_old_session_usable"] = True
            if installed:
                assert current["saved_revision"] == before["revision"] + 1
                assert result["project_inode_changed"]
                assert sha256(root / "project.json") == expected["project_sha256"]
                recovered_expected = expected["state"]
                recovered_image = expected["image_path"]
            else:
                assert (root / "project.json").read_bytes() == before_bytes
                assert not result["project_inode_changed"]
                recovered_expected = before
                recovered_image = root / "before.png"
        events_before_restart = read_events(root)
        client.close()
        client = Client(executable, root, transcript, "cold-without-shim")
        cold_maps = Path(f"/proc/{client.process.pid}/maps").read_text()
        assert str(library) not in cold_maps, "cold restart still has the test interposer mapped"
        (root / "cold-without-shim.maps").write_text(cold_maps)
        assert client.revision == recovered_expected["revision"]
        assert client.call({"op": "get_document"}) == recovered_expected["document"]
        assert client.call({"op": "pose", "animation": animation()}) == recovered_expected["pose"]
        image = render(client, "recovered.png")
        assert (root / "recovered.png").read_bytes() == recovered_image.read_bytes()
        assert image["metrics"]["primary_center_ray_hits"] > 0
        assert read_events(root) == events_before_restart, "shim unexpectedly active after restart"
        result.update({"status": "passed" if mode == "fixed" else "baseline_observed", "cold_recovered_revision": client.revision,
            "cold_recovered_document_pose_and_png_exact": True, "recovered_png_sha256": sha256(root / "recovered.png"),
            "own_staging_files_cleaned": True, "foreign_file_preserved": True,
            "cold_restart_has_no_test_preload": True})
        return result
    finally:
        if (root / ".fault-armed").exists():
            (root / ".fault-armed").unlink()
        client.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--editor", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--mode", choices=["baseline", "fixed"], default="fixed")
    args = parser.parse_args()
    if platform.system() != "Linux":
        parser.error("requires Linux /proc, ELF LD_PRELOAD and Linux syscall semantics")
    compiler = shutil.which("cc")
    if not compiler:
        parser.error("requires a C compiler to build the test-only interposer")
    if os.environ.get("LD_PRELOAD"):
        parser.error("run this isolated acceptance from an environment without an existing LD_PRELOAD")
    executable = args.editor.resolve(strict=True)
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    started = time.monotonic()
    source = ROOT / "scripts/storage_fault_shim.c"
    library = root / "libstorage_fault.so"
    command = [compiler, "-shared", "-fPIC", "-std=c11", "-O2", "-Wall", "-Wextra", "-Werror", str(source), "-o", str(library)]
    built = subprocess.run(command, capture_output=True, text=True)
    (root / "compiler.stdout.log").write_text(built.stdout)
    (root / "compiler.stderr.log").write_text(built.stderr)
    assert built.returncode == 0, built.stderr
    shutil.copyfile(source, root / "storage_fault_shim.c")
    report = {"status": "running", "mode": args.mode, "binary": str(executable), "binary_sha256": sha256(executable),
        "harness_sha256": sha256(Path(__file__)), "shim_source_sha256": sha256(source), "shim_library_sha256": sha256(library),
        "compiler_command": command, "compiler_version": subprocess.check_output([compiler, "--version"], text=True).splitlines()[0],
        "platform": platform.platform(), "dynamic_loader": subprocess.check_output(["ldd", str(executable)], text=True),
        "scope": "targeted error-return injection around real staged writes, file sync, install and cleanup syscalls",
        "limits": ["not physical power-loss testing", "not a filesystem durability certificate", "Linux dynamic-linker boundary only"],
        "cases": []}
    report_path = root / "acceptance.json"
    write_json(report_path, report)
    try:
        with (root / "transcript.jsonl").open("w") as transcript:
            for name, fault, output_operation in CASES:
                result = run_case(executable, library, root, name, fault, output_operation, args.mode, transcript)
                report["cases"].append(result)
                write_json(report_path, report)
                print(json.dumps({"case": name, "status": result["status"], "response_code": result["response"].get("error", {}).get("code"),
                                  "disk_revision": result["disk_revision_after_operation"], "cold_revision": result["cold_recovered_revision"]}), flush=True)
        assert sha256(executable) == report["binary_sha256"]
        report["status"] = "passed" if args.mode == "fixed" else "baseline_classification_gaps_reproduced"
    except Exception as error:
        report.update({"status": "failed", "error": str(error), "traceback": traceback.format_exc()})
        raise
    finally:
        report["elapsed_seconds"] = time.monotonic() - started
        report["artifacts_sha256"] = {str(path.relative_to(root)): sha256(path) for path in root.rglob("*")
                                     if path.is_file() and path != report_path}
        write_json(report_path, report)
    print(json.dumps({"status": report["status"], "cases": len(report["cases"]), "elapsed_seconds": report["elapsed_seconds"]}, indent=2))


if __name__ == "__main__":
    main()
