#!/usr/bin/env python3
"""Real-process native cloth authoring, dynamics, rendering and persistence acceptance.

Preserves requests, responses, timings, failed assertions and actual PNG artifacts.
This is discrete vertex-contact cloth, not self-collision or a film-readiness certificate.
"""
import argparse
import copy
import hashlib
import json
import select
import subprocess
import time
import traceback
from pathlib import Path

from agent_animation_acceptance import Client, write_json


class DurableClient(Client):
    def __init__(self, executable, root, transcript, session):
        self.stderr = (root / f"{session}-stderr.log").open("w+")
        self.process = subprocess.Popen(
            [str(executable), "--root", str(root), "--project", "cloth-project.json"],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=self.stderr, text=True,
        )
        self.revision = 0
        self.sequence = 0
        self.transcript = transcript
        self.session = session
        self.call({"op": "inspect"})


def panel(identifier, collision=True):
    return {
        "id": identifier, "label": "Draped cloth panel", "origin": [0, 1, 0],
        "axis_u": [1, 0, 0], "axis_v": [0, 0, 1], "segments": [6, 6],
        "width_m": 0.8, "height_m": 0.8, "thickness_m": 0.008, "vertex_mass_kg": 0.02,
        "pins": [
            {"vertex": 0, "target_object": "anchor", "point": [-0.4, 0, 0]},
            {"vertex": 6, "target_object": "anchor", "point": [0.4, 0, 0]},
        ],
        "collision_object_ids": ["floor"] if collision else [],
        "settings": {"iterations": 24, "bend_compliance": 0.05, "collision_thickness": 0.005},
        "material": {"albedo": [0.75, 0.055, 0.12], "roughness": 0.7},
    }


def clip(duration=1, motion=0.15):
    return {"id": "move", "duration": duration, "tracks": [
        {"target": {"type": "object", "id": "anchor"}, "keys": [
            {"time": 0}, {"time": duration, "translation": [motion, 0.1, 0]},
        ]},
    ]}


def sample(value):
    return {"clip": "move", "time": value}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--executable", "--exe", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    executable = args.executable.resolve(strict=True)
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    started = time.monotonic()
    result = {"status": "running", "root": str(root), "binary_sha256": hashlib.sha256(executable.read_bytes()).hexdigest(), "not_proven": [
        "self-collision", "continuous collision detection", "triangle-interior collision",
        "friction", "rigid-body reaction", "measured fabric behavior", "film production readiness",
    ]}
    client = None
    try:
        with (root / "transcript.jsonl").open("w") as transcript:
            client = DurableClient(executable, root, transcript, "author")
            client.apply([
                {"op": "create", "object": {"id": "floor", "shape": {"type": "plane", "normal": [0, 1, 0], "offset": -0.85},
                    "material": {"albedo": [0.20, 0.23, 0.28], "roughness": 0.8}}},
                {"op": "create", "object": {"id": "anchor", "shape": {"type": "box", "half_extents": [0.45, 0.025, 0.025]},
                    "position": [0, 1, -0.4], "material": {"albedo": [0.3, 0.4, 0.5]}}},
                {"op": "put_clip", "clip": clip()},
                {"op": "set_camera", "camera": {"eye": [1.5, 1.9, 2.0], "target": [0.05, 0.78, 0], "up": [0, 1, 0], "fov_degrees": 40}},
                {"op": "set_settings", "settings": {"width": 128, "height": 112, "quality": "balanced", "shadows": True, "ao": True}},
                {"op": "create_cloth_panel", "request": panel("cape")},
                {"op": "create_cloth_panel", "request": panel("no-contact-control", False)},
            ])
            before = client.call({"op": "get_document"})
            baked = client.call({"op": "bake_cloth", "request": {"id": "cape", "clip": "move"}}, mutate=True)
            client.call({"op": "bake_cloth", "request": {"id": "no-contact-control", "clip": "move"}}, mutate=True)
            after = client.call({"op": "get_document"})
            assert before["objects"] == after["objects"], "baking modified rest objects"
            original = client.call({"op": "cloth_state", "id": "cape"})
            posed = client.call({"op": "cloth_state", "id": "cape", "animation": sample(1)})
            control = client.call({"op": "cloth_state", "id": "no-contact-control", "animation": sample(1)})
            assert min(p[1] for p in posed["vertices"]) >= 0.855 - 0.002
            assert min(p[1] for p in control["vertices"]) < 0.80, "gravity/contact negative control failed"
            assert posed["cache"]["diagnostics"]["contact_projections"] > 0
            assert control["cache"]["diagnostics"]["contact_projections"] == 0
            assert any(abs(a[1] - b[1]) > 0.1 for a, b in zip(original["vertices"], posed["vertices"]))
            client.apply([{"op": "remove_cloth", "id": "no-contact-control"}])
            settled = client.call({"op": "get_document"})
            state_before = client.call({"op": "cloth_state", "id": "cape", "animation": sample(1)})
            # Invalid duration must not mutate the cache, geometry, revision or durable project.
            revision = client.revision
            bad = client.call({"op": "bake_cloth", "request": {"id": "cape", "clip": "move", "duration": 0.501}}, mutate=True, expect_ok=False)
            assert client.revision == revision
            assert settled == client.call({"op": "get_document"})
            renders = {}
            for name, value in [("rest", None), ("middle", sample(0.5)), ("draped", sample(1))]:
                command = {"op": "render", "path": f"{name}.png"}
                if value is not None:
                    command["animation"] = value
                renders[name] = client.call(command)
            assert hashlib.sha256((root / "rest.png").read_bytes()).digest() != hashlib.sha256((root / "draped.png").read_bytes()).digest()
            # Isolate visible cloth deformation from the independently moving anchor.
            # The control evaluates the identical clip, camera, objects and materials,
            # but retains the explicitly authored rest triangle sheet without a cache.
            frozen_document = copy.deepcopy(settled)
            frozen_document["cloths"] = []
            write_json(root / "frozen-control.json", {"format": "mm3e-agent-project-v1",
                "saved_revision": 0, "document": frozen_document})
            frozen_client = Client(executable, root, transcript, "frozen-control")
            try:
                frozen_client.call({"op": "load", "path": "frozen-control.json"}, mutate=True)
                frozen_render = frozen_client.call({"op": "render", "path": "frozen-same-anchor.png", "animation": sample(1)})
                assert frozen_render["rgba_fnv1a64"] != renders["draped"]["rgba_fnv1a64"], "identical anchor/camera with frozen cloth must render differently"
                # Resolve actual visible pixels to the cloth material in both poses.
                visible_cloth_checks = []
                for y in range(4, 112, 8):
                    for x in range(4, 128, 8):
                        live_hit = client.call({"op": "pick", "x": x, "y": y, "animation": sample(1)})
                        if live_hit.get("material_owner_id") != "cape":
                            continue
                        frozen_hit = frozen_client.call({"op": "pick", "x": x, "y": y, "animation": sample(1)})
                        if frozen_hit.get("material_owner_id") != "cape" or abs(live_hit["ray_t"] - frozen_hit["ray_t"]) > 0.01:
                            visible_cloth_checks.append({"pixel": [x,y], "live": live_hit, "frozen": frozen_hit})
                assert len(visible_cloth_checks) >= 3, "actual visible cloth geometry did not move relative to the fixed-pose control"
                write_json(root / "cloth-visible-deformation.json", visible_cloth_checks)
                frozen_client.close()
            finally:
                if frozen_client.process.poll() is None:
                    frozen_client.abort()
            client.close()
            client = DurableClient(executable, root, transcript, "reopen")
            assert client.call({"op": "get_document"}) == settled
            assert client.call({"op": "cloth_state", "id": "cape", "animation": sample(1)}) == state_before
            client.call({"op": "render", "path": "reopened.png", "animation": sample(1)})
            assert (root / "reopened.png").read_bytes() == (root / "draped.png").read_bytes()
            # A changed clip must invalidate playback until a real new bake succeeds.
            client.apply([{"op": "put_clip", "clip": clip(motion=0.22)}])
            stale = client.call({"op": "cloth_state", "id": "cape", "animation": sample(0.5)}, expect_ok=False)
            assert "stale" in stale["error"]["message"]
            client.call({"op": "bake_cloth", "request": {"id": "cape", "clip": "move"}}, mutate=True)
            # Start a sufficiently long physical bake, verify its process is still live
            # without an acknowledgement, kill it, and inspect persisted state anew.
            client.apply([{"op": "put_clip", "clip": clip(duration=10, motion=0.22)}])
            interrupted_before = client.call({"op": "get_document"})
            interrupted_revision = client.revision
            command = {"id": "interrupt-active-bake", "expected_revision": interrupted_revision,
                       "command": {"op": "bake_cloth", "request": {"id": "cape", "clip": "move"}}}
            transcript.write(json.dumps({"session": client.session, "request": command}) + "\n")
            transcript.flush()
            client.process.stdin.write(json.dumps(command) + "\n")
            client.process.stdin.flush()
            time.sleep(0.05)
            assert client.process.poll() is None, "bake process was not live"
            assert not select.select([client.process.stdout], [], [], 0)[0], "bake completed before interruption fixture"
            client.abort()
            transcript.write(json.dumps({"session": "reopen", "interruption": "SIGKILL before bake acknowledgement", "returncode": client.process.returncode}) + "\n")
            transcript.flush()
            client = DurableClient(executable, root, transcript, "after-interruption")
            assert client.revision == interrupted_revision
            assert client.call({"op": "get_document"}) == interrupted_before
            # Restore the original clip and rebake; interrupted state remains in transcript.
            client.apply([{"op": "put_clip", "clip": clip()}])
            client.call({"op": "bake_cloth", "request": {"id": "cape", "clip": "move"}}, mutate=True)
            final = client.call({"op": "cloth_state", "id": "cape", "animation": sample(1)})
            assert final["vertices"] == posed["vertices"]
            client.close()
            client = None
            result.update({"status": "passed", "bake": baked, "cache_frames": posed["cache"]["frames"],
                "vertex_count": len(posed["vertices"]), "cloth_diagnostics": posed["cache"]["diagnostics"],
                "negative_control_min_y": min(p[1] for p in control["vertices"]),
                "cloth_min_y": min(p[1] for p in posed["vertices"]), "rest_sources_unchanged": True,
                "bad_bake_atomic": True, "bad_bake_error": bad["error"], "stale_cache_rejected": True,
                "fresh_process_mesh_identical": True, "fresh_process_png_identical": True,
                "same_anchor_frozen_cloth_image_control": True, "visible_cloth_changed_probe_count": len(visible_cloth_checks),
                "interrupted_bake_durable_state_unchanged": True, "rebake_replays_identical_vertices": True,
                "renders": renders, "elapsed_seconds": time.monotonic() - started})
    except Exception as error:
        result.update({"status": "failed", "error": str(error), "traceback": traceback.format_exc(), "elapsed_seconds": time.monotonic() - started})
        raise
    finally:
        if client is not None:
            client.abort()
        write_json(root / "acceptance.json", result)
    print(json.dumps({key: result[key] for key in ("status", "root", "vertex_count", "cache_frames", "elapsed_seconds")}, indent=2))


if __name__ == "__main__":
    main()
