#!/usr/bin/env python3
"""Verify real JSONL film output with the independent OpenEXR reference decoder.

Requires Python OpenEXR >=3.3 and numpy. Output must be a new directory; all requests,
responses, images and failure evidence remain there. This is output acceptance, not
a certification of character quality, cloth simulation, motion blur or color management.
"""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import time


ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    try:
        import OpenEXR
        import numpy as np
    except ImportError as error:
        parser.error(f"independent decoder required: install OpenEXR and numpy ({error})")
    binary = args.binary.resolve(strict=True)
    output = args.output.resolve()
    output.mkdir(parents=False, exist_ok=False)
    started = time.perf_counter()

    def run(requests, name):
        wire = "".join(json.dumps(item) + "\n" for item in requests)
        (output / f"{name}.requests.jsonl").write_text(wire)
        result = subprocess.run([str(binary), "--root", str(output)], input=wire, text=True, capture_output=True, timeout=300)
        (output / f"{name}.responses.jsonl").write_text(result.stdout)
        (output / f"{name}.stderr.txt").write_text(result.stderr)
        result.check_returncode()
        responses = [json.loads(line) for line in result.stdout.splitlines()]
        assert len(responses) == len(requests), "missing JSONL response"
        for request, response in zip(requests, responses):
            assert response["id"] == request["id"]
            expected = request["id"] not in {"reject-display-pass", "reject-display-sequence"}
            assert response["ok"] is expected, response
        return responses

    def request(identifier, command, revision=None):
        item = {"id": identifier, "command": command}
        if revision is not None:
            item["expected_revision"] = revision
        return item

    def settings(width, height, exposure=1):
        return {"op": "set_settings", "settings": {"width": width, "height": height, "quality": "preview", "shadows": False, "ao": False, "exposure": exposure, "sky_ambient": [0, 0, 0]}}

    def decode(name, width, height):
        with OpenEXR.File(str(output / name), separate_channels=True) as image:
            assert len(image.parts) == 1, "unexpected multipart output"
            header = image.header()
            channels = image.channels()
            assert set(channels) == {"R", "G", "B"}, channels.keys()
            for channel, value in zip(("R", "G", "B"), (4.0, 8.0, 16.0)):
                pixels = channels[channel].pixels
                assert pixels.shape == (height, width), pixels.shape
                assert pixels.dtype == np.float32, pixels.dtype
                assert np.isfinite(pixels).all(), "nonfinite image data"
                assert np.all(pixels == value), (channel, float(pixels.min()), float(pixels.max()))
            assert header["transferFunction"] == "linear", header
            assert header["colorSpace"] == "Linear Rec.709 (sRGB primaries), D65", header
            assert np.allclose(np.asarray(header["chromaticities"]).reshape(-1), [0.64, 0.33, 0.30, 0.60, 0.15, 0.06, 0.3127, 0.3290])
            assert np.array_equal(header["dataWindow"][0], [0, 0])
            assert np.array_equal(header["dataWindow"][1], [width - 1, height - 1])
            return {"width": width, "height": height, "channels": ["R", "G", "B"], "sample_type": "float32", "constant_rgb": [4, 8, 16], "transfer_function": header["transferFunction"], "color_space": header["colorSpace"]}

    try:
        requests = [
            request("author", {"op": "apply", "operations": [
                settings(64, 32),
                {"op": "set_camera", "camera": {"eye": [0, 0, 2], "target": [0, 0, 0], "up": [0, 1, 0], "fov_degrees": 40}},
                {"op": "set_lights", "lights": []},
                {"op": "create", "object": {"id": "emission-plate", "shape": {"type": "plane", "normal": [0, 0, 1], "offset": 0}, "material": {"albedo": [0, 0, 0], "emissive": [4, 8, 16], "specular": 0}}},
                {"op": "put_clip", "clip": {"id": "hold", "duration": 12}},
            ]}, 0),
            request("linear", {"op": "render", "path": "linear.exr"}),
            request("save", {"op": "save", "path": "film.mm3e-agent.json"}),
            request("exposure", {"op": "apply", "operations": [settings(64, 32, 32)]}, 1),
            request("exposure-linear", {"op": "render", "path": "exposure-32.exr"}),
            request("reject-display-pass", {"op": "render", "path": "normal.exr", "pass": "normal"}),
            request("hd-settings", {"op": "apply", "operations": [settings(1920, 1080)]}, 2),
            request("hd-linear", {"op": "render", "path": "hd.exr"}),
            request("long-settings", {"op": "apply", "operations": [settings(16, 8)]}, 3),
            request("long-export", {"op": "render_sequence", "request": {"directory": "twelve-seconds", "clip": "hold", "start": 0, "end": 12, "fps": 24}}),
            request("linear-sequence", {"op": "render_sequence", "request": {"directory": "linear-frames", "clip": "hold", "start": 0, "end": 1, "fps": 2, "format": "exr"}}),
            request("reject-display-sequence", {"op": "render_sequence", "request": {"directory": "invalid-exr-frames", "clip": "hold", "start": 0, "end": 1, "fps": 2, "format": "exr", "pass": "normal"}}),
        ]
        first = run(requests, "author")
        assert not (output / "normal.exr").exists(), "rejected EXR display pass created output"
        assert not (output / "invalid-exr-frames").exists(), "rejected EXR sequence reserved an output directory"
        second = run([
            request("load", {"op": "load", "path": "film.mm3e-agent.json"}, 0),
            request("reopen-linear", {"op": "render", "path": "reopened.exr"}),
        ], "reopen")
        decoded = {name: decode(name, width, height) for name, width, height in [
            ("linear.exr", 64, 32), ("exposure-32.exr", 64, 32), ("hd.exr", 1920, 1080), ("reopened.exr", 64, 32),
        ]}
        manifest = json.loads((output / "twelve-seconds/manifest.json").read_text())
        assert manifest["frame_count"] == 289 == len(manifest["frames"])
        assert manifest["frames"][0]["time"] == 0 and manifest["frames"][-1]["time"] == 12
        assert len(list((output / "twelve-seconds").glob("frame_*.png"))) == 289
        linear_manifest = json.loads((output / "linear-frames/manifest.json").read_text())
        assert linear_manifest["image_format"] == "exr"
        assert linear_manifest["frame_count"] == 3 == len(linear_manifest["frames"])
        assert [frame["time"] for frame in linear_manifest["frames"]] == [0, 0.5, 1]
        for index, frame in enumerate(linear_manifest["frames"]):
            name = f"linear-frames/frame_{index:04}.exr"
            decoded[name] = decode(name, 16, 8)
            assert frame["rgba_fnv1a64"] is None
            assert isinstance(frame["linear_rgb_fnv1a64"], str)
            assert frame["color_encoding"] == "scene_linear_rec709_f32_rgb"
        expected_hash = 0xCBF29CE484222325
        for byte in np.tile(np.array([4, 8, 16], dtype="<f4"), 16 * 8).tobytes():
            expected_hash = ((expected_hash ^ byte) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
        assert all(frame["linear_rgb_fnv1a64"] == f"{expected_hash:016x}" for frame in linear_manifest["frames"])
        assert first[-1]["revision"] == 4 and second[-1]["revision"] == 1
        summary = {
            "passed": True, "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
            "independent_decoder": f"OpenEXR Python {OpenEXR.__version__}",
            "decoded_images": decoded, "long_sequence_frames": 289, "linear_sequence_frames": 3,
            "checks": ["actual linear f32 HDR values preserved", "named RGB channels and dimensions", "declared chromaticities and linear transfer", "display exposure does not modify scene-linear EXR", "display AOV still and sequence rejected without output", "HD image delivered", "fresh process reproduces saved image values", "12-second 24-fps inclusive PNG export", "EXR sequence frames independently decoded with linear manifest fields"],
            "elapsed_seconds": time.perf_counter() - started,
            "not_verified": ["alpha/compositing mattes", "OCIO color transforms", "motion blur", "film character quality", "cloth draping", "complex scene throughput"],
        }
        (output / "acceptance.json").write_text(json.dumps(summary, indent=2) + "\n")
        print(json.dumps(summary, indent=2))
    except Exception as error:
        (output / "failure.json").write_text(json.dumps({"passed": False, "error": repr(error), "elapsed_seconds": time.perf_counter() - started}, indent=2) + "\n")
        raise


if __name__ == "__main__":
    main()
