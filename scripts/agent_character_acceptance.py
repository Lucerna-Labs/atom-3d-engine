#!/usr/bin/env python3
"""Exercise the actual JSONL executable, including a measurement-driven character edit.

This is a deterministic tool-client acceptance run, not an LLM or a character-quality benchmark.
It preserves every request/response and the intermediate measurements in a fresh output directory.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import time


class Client:
    def __init__(self, executable, root, transcript):
        self.process = subprocess.Popen(
            [str(executable), "--root", str(root)],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
        )
        self.revision = 0
        self.sequence = 0
        self.transcript = transcript

    def call(self, command, mutate=False, expect_ok=True):
        self.sequence += 1
        request = {"id": f"acceptance-{self.sequence}", "command": command}
        if mutate:
            request["expected_revision"] = self.revision
        self.process.stdin.write(json.dumps(request) + "\n")
        self.process.stdin.flush()
        line = self.process.stdout.readline()
        if not line:
            raise RuntimeError("editor exited: " + self.process.stderr.read())
        response = json.loads(line)
        self.transcript.write(json.dumps({"request": request, "response": response}) + "\n")
        self.transcript.flush()
        assert response["id"] == request["id"]
        assert response["ok"] == expect_ok, response
        self.revision = response["revision"]
        return response.get("result") if expect_ok else response

    def apply(self, operations, dry_run=False, expect_ok=True):
        return self.call({"op": "apply", "operations": operations, "dry_run": dry_run}, True, expect_ok)

    def close(self):
        self.process.stdin.close()
        self.process.wait(timeout=30)
        assert self.process.returncode == 0, self.process.stderr.read()


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
    result = {"status": "running", "kind": "real JSONL process acceptance", "backend": "MM3E CPU", "root": str(root)}
    result_path = root / "acceptance.json"
    result_path.write_text(json.dumps(result, indent=2) + "\n")
    client = None
    try:
        with (root / "transcript.jsonl").open("w") as transcript:
            client = Client(executable, root, transcript)
            capabilities = client.call({"op": "describe"})
            (root / "capabilities.json").write_text(json.dumps(capabilities, indent=2) + "\n")
            client.apply([
                {"op": "create_humanoid", "id": "hero", "height": 1.8, "build": 1.0, "head_scale": 1.15},
                {"op": "create", "object": {"id": "pedestal", "shape": {"type": "cylinder", "half_height": 0.025, "radius": 0.60},
                    "position": [0, -0.025, 0], "material": {"albedo": [0.07, 0.11, 0.14], "roughness": 0.8}}},
                {"op": "set_settings", "settings": {"width": 384, "height": 512, "quality": "full", "exposure": 1.0}},
                {"op": "set_camera", "camera": {"eye": [0, 1.05, 4.3], "target": [0, 0.91, 0], "fov_degrees": 30}},
            ])
            document = client.call({"op": "get_document"})
            assert len(document["objects"]) == 20
            chest = client.call({"op": "inspect", "id": "hero/chest"})
            original_radii = chest["shape"]["radii"]
            before = client.call({"op": "render", "path": "before-front.png"})
            assert before["metrics"]["primary_center_ray_hits"] > 0

            # Widen the chest until the authored surface reaches a target landmark. Other
            # chest radii and every other entity remain fixed. The only feedback is the
            # engine's field measurement, rather than a substitute geometry implementation.
            target = [0.285, chest["position"][1], chest["position"][2]]
            lo, hi = original_radii[0], 0.36
            trajectory = []
            for iteration in range(14):
                radius = (lo + hi) * 0.5
                client.apply([{"op": "update", "id": "hero/chest", "patch": {
                    "shape": {"type": "ellipsoid", "radii": [radius, *original_radii[1:]]}}}])
                sample = client.call({"op": "sample", "id": "hero/chest", "points": [target]})["samples"][0]
                residual = sample["value"]
                trajectory.append({"iteration": iteration, "radius": radius, "authored_field_residual": residual})
                assert sample["material_owner_id"] == "hero/chest", sample
                if abs(residual) <= 0.0002:
                    break
                if residual > 0:
                    lo = radius
                else:
                    hi = radius
            assert abs(trajectory[-1]["authored_field_residual"]) <= 0.0002, trajectory
            after_doc = client.call({"op": "get_document"})
            for old, new in zip(document["objects"], after_doc["objects"]):
                assert old["id"] == new["id"]
                if old["id"] != "hero/chest":
                    assert old == new
                else:
                    assert old["shape"]["radii"][1:] == new["shape"]["radii"][1:]

            # Partial changes must not survive a failed batch; dry runs must not commit.
            revision = client.revision
            client.apply([
                {"op": "translate", "ids": ["hero/left_hand"], "delta": [0.1, 0, 0]},
                {"op": "update", "id": "hero/right_hand", "patch": {"scale": -1}},
            ], expect_ok=False)
            assert client.revision == revision
            assert client.call({"op": "get_document"}) == after_doc
            client.apply([{"op": "delete", "id": "pedestal"}], dry_run=True)
            assert client.revision == revision
            assert client.call({"op": "get_document"}) == after_doc
            client.call({"op": "undo"}, True)
            client.call({"op": "redo"}, True)
            assert client.call({"op": "get_document"}) == after_doc
            after = client.call({"op": "render", "path": "after-front.png"})
            assert before["rgba_fnv1a64"] != after["rgba_fnv1a64"]

            views = {
                "three-quarter": {"eye": [2.7, 1.8, 4.1], "target": [0, 0.92, 0], "fov_degrees": 27},
                "side": {"eye": [4.3, 1.05, 0], "target": [0, 0.91, 0], "fov_degrees": 30},
                "back": {"eye": [0, 1.05, -4.3], "target": [0, 0.91, 0], "fov_degrees": 30},
            }
            renders = {"before-front": before, "after-front": after}
            for name, view in views.items():
                renders[name] = client.call({"op": "render", "path": f"{name}.png", "view": view})
            for name in ("normal", "depth"):
                renders[name] = client.call({"op": "render", "path": f"{name}.png", "pass": name})
            client.call({"op": "save", "path": "hero.mm3e-agent.json"})
            final_revision = client.revision
            client.close()
            client = Client(executable, root, transcript)
            client.call({"op": "load", "path": "hero.mm3e-agent.json"}, True)
            assert client.call({"op": "get_document"}) == after_doc
            reloaded = client.call({"op": "render", "path": "reloaded-front.png"})
            assert after["rgba_fnv1a64"] == reloaded["rgba_fnv1a64"]
            assert (root / "after-front.png").read_bytes() == (root / "reloaded-front.png").read_bytes()
            client.close()
            client = None
            result.update({"status": "passed", "entity_count": 20, "character_parts": 19,
                "surface_fit": {"target": target, "tolerance": 0.0002, "trajectory": trajectory},
                "atomic_rejection": True, "dry_run_unchanged": True, "undo_redo_restored": True,
                "fresh_process_reload_pixel_identical": True, "final_saved_revision": final_revision,
                "renders": renders, "final_png_sha256": hashlib.sha256((root / "after-front.png").read_bytes()).hexdigest(),
                "not_evaluated": ["LLM planning", "anatomical quality", "rigging", "topology", "GPU rendering"],
                "elapsed_seconds": time.monotonic() - started})
    except Exception as error:
        result.update({"status": "failed", "error": str(error), "elapsed_seconds": time.monotonic() - started})
        raise
    finally:
        if client is not None and client.process.poll() is None:
            client.process.kill()
            client.process.wait()
        result_path.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({key: result[key] for key in ("status", "root", "character_parts", "elapsed_seconds")}, indent=2))


if __name__ == "__main__":
    main()
