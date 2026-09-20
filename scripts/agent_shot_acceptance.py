#!/usr/bin/env python3
"""Real JSONL film-shot acceptance with independent OpenEXR and PNG decoding.

Requires OpenEXR and numpy. The output directory must be new. This checks actual temporal
radiance integration, geometric coverage, utility AOVs, persistence, lens output and output
preflight; it does not certify cloth appearance, character quality, hardware GPU rendering,
transmission, volumetrics, production sampling convergence or complete film readiness.
"""

import argparse
import hashlib
import json
from pathlib import Path
import select
import struct
import subprocess
import sys
import time
import traceback
import zlib


ROOT = Path(__file__).resolve().parents[1]


class Client:
    def __init__(self, binary, root, transcript, session):
        self.stderr = (root / f"{session}.stderr.txt").open("w")
        self.process = subprocess.Popen([str(binary), "--root", str(root)], stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE, stderr=self.stderr, text=True)
        self.revision = 0
        self.count = 0
        self.transcript = transcript
        self.session = session

    def call(self, command, mutation=False, ok=True):
        self.count += 1
        request = {"id": f"{self.session}-{self.count}", "command": command}
        if mutation:
            request["expected_revision"] = self.revision
        self.transcript.write(json.dumps({"request": request}) + "\n")
        self.transcript.flush()
        self.process.stdin.write(json.dumps(request) + "\n")
        self.process.stdin.flush()
        if not select.select([self.process.stdout], [], [], 300)[0]:
            raise TimeoutError(f"editor did not answer {request['id']} within 300 seconds")
        response = json.loads(self.process.stdout.readline())
        self.transcript.write(json.dumps({"response": response}) + "\n")
        self.transcript.flush()
        assert response["id"] == request["id"], response
        assert response["ok"] == ok, response
        self.revision = response["revision"]
        return response["result"] if ok else response

    def apply(self, operations, ok=True):
        return self.call({"op": "apply", "operations": operations}, mutation=True, ok=ok)

    def close(self):
        self.process.stdin.close()
        self.process.wait(timeout=30)
        self.stderr.close()
        assert self.process.returncode == 0

    def abort(self):
        if self.process.poll() is None:
            self.process.kill()
            self.process.wait(timeout=30)
        self.stderr.close()


def decode_png(path, np):
    """Decode and CRC-check the actual emitted noninterlaced RGBA8 PNG, all five filters."""
    data = path.read_bytes()
    assert data[:8] == b"\x89PNG\r\n\x1a\n"
    cursor, compressed, dimensions = 8, bytearray(), None
    while cursor < len(data):
        size = struct.unpack_from(">I", data, cursor)[0]
        kind = data[cursor + 4:cursor + 8]
        payload = data[cursor + 8:cursor + 8 + size]
        checksum = struct.unpack_from(">I", data, cursor + 8 + size)[0]
        assert zlib.crc32(kind + payload) & 0xffffffff == checksum
        cursor += size + 12
        if kind == b"IHDR":
            width, height, depth, color, compression, filtering, interlace = struct.unpack(">IIBBBBB", payload)
            assert (depth, color, compression, filtering, interlace) == (8, 6, 0, 0, 0)
            dimensions = width, height
        elif kind == b"IDAT":
            compressed.extend(payload)
        elif kind == b"IEND":
            break
    assert cursor == len(data) and dimensions
    width, height = dimensions
    stride = width * 4
    raw = zlib.decompress(compressed)
    assert len(raw) == height * (stride + 1)
    rows, previous = [], bytearray(stride)
    for row in range(height):
        offset = row * (stride + 1)
        filter_kind = raw[offset]
        assert 0 <= filter_kind <= 4
        decoded = bytearray(raw[offset + 1:offset + stride + 1])
        for column in range(stride):
            left = decoded[column - 4] if column >= 4 else 0
            up = previous[column]
            upper_left = previous[column - 4] if column >= 4 else 0
            if filter_kind == 0:
                predictor = 0
            elif filter_kind == 1:
                predictor = left
            elif filter_kind == 2:
                predictor = up
            elif filter_kind == 3:
                predictor = (left + up) // 2
            else:
                p = left + up - upper_left
                distances = [abs(p - left), abs(p - up), abs(p - upper_left)]
                predictor = (left, up, upper_left)[distances.index(min(distances))]
            decoded[column] = (decoded[column] + predictor) & 255
        rows.append(decoded)
        previous = decoded
    return np.frombuffer(b"".join(rows), dtype=np.uint8).reshape(height, width, 4).copy()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=ROOT / "target/release/mm3e-editor")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--decoder-path", type=Path, default=Path("/tmp/mm3e-exr-decoder"))
    args = parser.parse_args()
    if args.decoder_path.is_dir():
        sys.path.insert(0, str(args.decoder_path))
    try:
        import OpenEXR
        import numpy as np
    except ImportError as error:
        parser.error(f"OpenEXR and numpy independent decoder required: {error}")
    binary = args.binary.resolve(strict=True)
    root = args.output.resolve()
    root.mkdir(parents=False, exist_ok=False)
    started = time.monotonic()
    result = {"passed": False, "status": "running", "binary": str(binary),
              "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
              "independent_decoder": f"OpenEXR Python {OpenEXR.__version__}", "output": str(root)}
    result_path = root / "acceptance.json"
    result_path.write_text(json.dumps(result, indent=2) + "\n")
    client = None

    def settings(film=None, aa=4, width=65, height=49):
        return {"op": "set_settings", "settings": {"width": width, "height": height, "quality": "full",
                "spatial_aa": aa, "shadows": False, "ao": False, "sky_ambient": [0, 0, 0],
                "fog_density": 0, "exposure": 1, "film": film or {}}}

    def film(**overrides):
        value = {"transparent_background": True, "exr_data_channels": True, "aperture_radius_m": 0,
                 "focus_distance_m": 3, "shutter_open_seconds": 0, "shutter_close_seconds": 0,
                 "shutter_samples": 1}
        value.update(overrides)
        return value

    def render(path, clip="travel", moment=0.5, ok=True, **options):
        return client.call({"op": "render", "path": path, "animation": {"clip": clip, "time": moment},
                            **options}, ok=ok)

    def decode_exr(path, data_channels=True, legacy=False):
        with OpenEXR.File(str(root / path), separate_channels=True) as image:
            assert len(image.parts) == 1
            # OpenEXR owns this dictionary; retain an independent copy before File.__exit__.
            header = dict(image.header())
            channels = {name: channel.pixels.copy() for name, channel in image.channels().items()}
            expected = {"R", "G", "B"} if legacy else {"R", "G", "B", "A"}
            if data_channels and not legacy:
                expected |= {"Z", "N.X", "N.Y", "N.Z", "material.ID"}
            assert set(channels) == expected, (path, set(channels), expected)
            for name, pixels in channels.items():
                assert pixels.shape == (49, 65), (path, name, pixels.shape)
                assert pixels.dtype == (np.uint32 if name == "material.ID" else np.float32)
            assert header["transferFunction"] == "linear"
            assert header["colorSpace"] == "Linear Rec.709 (sRGB primaries), D65"
            if not legacy:
                assert "premultiplied" in header["mm3e:alphaSemantics"]
                assert np.isfinite(channels["A"]).all()
                assert ((channels["A"] >= 0) & (channels["A"] <= 1)).all()
                for name in ["R", "G", "B"]:
                    assert np.isfinite(channels[name]).all()
                    assert np.all(channels[name][channels["A"] == 0] == 0)
                if data_channels:
                    assert channels["material.ID"].dtype == np.uint32
                    miss = channels["material.ID"] == np.uint32(0xffffffff)
                    assert np.isposinf(channels["Z"][miss]).all()
                    assert np.isfinite(channels["Z"][~miss]).all()
                    assert np.all(channels["Z"][~miss] >= 0)
                    normals = np.stack([channels[name] for name in ["N.X", "N.Y", "N.Z"]], axis=-1)
                    assert np.all(normals[miss] == 0)
                    assert np.allclose(np.linalg.norm(normals[~miss], axis=-1), 1, atol=0.001)
                    assert "pinhole" in header["mm3e:aovSampling"]
            return channels, header

    def check_png(path, channels):
        pixels = decode_png(root / path, np)
        alpha = pixels[:, :, 3]
        expected_alpha = np.floor(channels["A"] * 255 + 0.5).astype(np.uint8)
        assert np.array_equal(alpha, expected_alpha)
        assert np.all(pixels[:, :, :3][alpha == 0] == 0), "encoded alpha-zero PNG pixels must have RGB zero"
        covered = alpha > 0
        # Constant emissive surface gives the same straight display color at partial and full
        # coverage. Premultiplied values mistakenly written as PNG would darken edge pixels.
        expected = np.asarray([4.0, 1.0, 0.25], dtype=np.float32)
        expected = np.clip((expected * (2.51 * expected + 0.03)) / (expected * (2.43 * expected + 0.59) + 0.14), 0, 1)
        expected = np.floor(np.power(expected, 1 / 2.2) * 255 + 0.5).astype(np.uint8)
        assert np.max(np.abs(pixels[:, :, :3][covered].astype(int) - expected.astype(int))) <= 1
        assert ((alpha > 0) & (alpha < 255)).any()
        return {"covered_pixels": int(covered.sum()), "partial_pixels": int(((alpha > 0) & (alpha < 255)).sum())}

    def unchanged(before, revision):
        assert client.call({"op": "get_document"}) == before
        assert client.revision == revision

    try:
        with (root / "transcript.jsonl").open("w") as transcript:
            client = Client(binary, root, transcript, "author")
            client.apply([
                settings(film()),
                {"op": "set_camera", "camera": {"eye": [0, 0, 3], "target": [0, 0, 0], "fov_degrees": 45}},
                {"op": "set_lights", "lights": []},
                {"op": "create", "object": {"id": "emissive-orb", "shape": {"type": "sphere", "radius": 0.25},
                    "material": {"albedo": [0, 0, 0], "emissive": [4, 1, 0.25], "specular": 0}}},
                {"op": "put_clip", "clip": {"id": "travel", "duration": 1, "tracks": [
                    {"target": {"type": "object", "id": "emissive-orb"}, "keys": [
                        {"time": 0, "translation": [-0.7, 0, 0]}, {"time": 1, "translation": [0.7, 0, 0]}]}]}},
            ])
            authored, revision = client.call({"op": "get_document"}), client.revision
            render("instant.exr")
            render("instant.png")
            render("reject-alpha.bmp", ok=False)
            render("reject-normal.png", ok=False, **{"pass": "normal"})
            for name in ["reject-alpha.bmp", "reject-normal.png"]:
                assert not (root / name).exists()
            unchanged(authored, revision)
            instantaneous, instant_header = decode_exr("instant.exr")
            result["instant_png"] = check_png("instant.png", instantaneous)
            assert abs(float(instantaneous["Z"][24, 32]) - 2.75) < 0.002
            assert int(instantaneous["material.ID"][24, 32]) == 1
            assert json.loads(instant_header["mm3e:materialMap"]) == {"1": "emissive-orb"}

            motion_settings = film(shutter_open_seconds=-0.4, shutter_close_seconds=0.4, shutter_samples=8)
            client.apply([settings(motion_settings)])
            motion_document, revision = client.call({"op": "get_document"}), client.revision
            observation = render("motion.exr")
            render("motion.png")
            client.call({"op": "save", "path": "motion.mm3e-agent.json"})
            unchanged(motion_document, revision)
            motion, metadata = decode_exr("motion.exr")
            result["motion_png"] = check_png("motion.png", motion)
            sample_times = observation["film"]["temporal_sample_times"]
            assert np.allclose(sample_times, np.arange(0.15, 0.86, 0.1), rtol=0, atol=0.000001)
            shutter = json.loads(metadata["mm3e:shutter"])
            assert shutter["sample_times_seconds"] == sample_times and shutter["data_aov_time"] == 0.5
            assert metadata["mm3e:documentFingerprint"].startswith("fnv1a64:")
            assert "camera-forward" in metadata["mm3e:depthSemantics"]
            assert json.loads(metadata["mm3e:camera"])["focus_distance_m"] == 3
            assert abs(json.loads(metadata["mm3e:camera"])["fov_y_degrees"] - 45) < 0.0001
            for name in ["Z", "N.X", "N.Y", "N.Z", "material.ID"]:
                assert np.array_equal(motion[name], instantaneous[name]), name
            static_columns = np.where(instantaneous["A"].max(axis=0) > 0)[0]
            motion_columns = np.where(motion["A"].max(axis=0) > 0)[0]
            assert np.ptp(motion_columns) > np.ptp(static_columns) + 5
            client.apply([settings(film())])
            summed = {name: np.zeros_like(motion[name]) for name in ["R", "G", "B", "A"]}
            for index, sample_time in enumerate(sample_times):
                path = f"sample-{index:02}.exr"
                render(path, moment=sample_time)
                samples, _ = decode_exr(path)
                for name in summed:
                    summed[name] += samples[name]
            for name, values in summed.items():
                assert np.array_equal(values * np.float32(1 / len(sample_times)), motion[name]), name
            for name, emission in [("R", 4), ("G", 1), ("B", 0.25)]:
                assert np.allclose(motion[name], motion["A"] * emission, atol=0.000001, rtol=0)

            client.apply([settings(film(), aa=8)])
            render("pinhole-aa8.exr")
            pinhole, _ = decode_exr("pinhole-aa8.exr")
            client.apply([settings(film(aperture_radius_m=0.3, focus_distance_m=8), aa=8)])
            render("dof.exr")
            render("dof.png")
            dof, dof_header = decode_exr("dof.exr")
            result["dof_png"] = check_png("dof.png", dof)
            assert not np.array_equal(dof["A"], pinhole["A"])
            assert ((dof["A"] > 0) & (dof["A"] < 1)).sum() > ((pinhole["A"] > 0) & (pinhole["A"] < 1)).sum()
            for name in ["Z", "N.X", "N.Y", "N.Z", "material.ID"]:
                assert np.array_equal(dof[name], pinhole[name]), name
            assert abs(json.loads(dof_header["mm3e:camera"])["lens_radius_m"] - 0.3) < 0.000001

            # A real sparse exposure exercises coverage that survives in f32 EXR but rounds
            # down to alpha zero in PNG. RGB must also become zero in that encoded pixel.
            client.apply([settings(film(shutter_open_seconds=-0.4, shutter_close_seconds=0.4,
                                        shutter_samples=64), aa=8),
                          {"op": "update", "id": "emissive-orb", "patch": {"shape": {"type": "sphere", "radius": 0.008}}}])
            render("sparse-motion.exr")
            render("sparse-motion.png")
            sparse, _ = decode_exr("sparse-motion.exr")
            sparse_png = decode_png(root / "sparse-motion.png", np)
            quantized_zero = (sparse["A"] > 0) & (sparse_png[:, :, 3] == 0)
            retained_rgb = (sparse_png[:, :, 3] == 0) & np.any(sparse_png[:, :, :3] != 0, axis=2)
            result["low_alpha_png"] = {"positive_float_coverage_quantized_to_zero": int(quantized_zero.sum()),
                                       "alpha_zero_pixels_retaining_rgb": int(retained_rgb.sum())}
            assert quantized_zero.any(), "fixture must exercise quantized coverage loss"
            assert not retained_rgb.any(), "PNG alpha quantization must also clear RGB"
            client.apply([{"op": "update", "id": "emissive-orb", "patch": {"shape": {"type": "sphere", "radius": 0.25}}}])

            client.apply([settings(film(exr_data_channels=False))])
            render("rgba-only.exr")
            rgba_only, rgba_header = decode_exr("rgba-only.exr", data_channels=False)
            assert "Z" not in rgba_only and "mm3e:depthSemantics" not in rgba_header
            client.apply([settings(film())])
            render("inside-origin.exr", view={"eye": [0, 0, 0.1], "target": [0, 0, -1], "fov_degrees": 45})
            inside, _ = decode_exr("inside-origin.exr")
            assert np.all(inside["Z"] == 0), "origin-inside immediate hits must retain camera-forward depth zero"
            assert np.all(inside["A"] == 1)

            client.apply([settings({})])
            render("legacy.exr")
            legacy, _ = decode_exr("legacy.exr", legacy=True)
            assert all(np.isfinite(legacy[name]).all() for name in legacy)

            client.apply([settings(film(shutter_open_seconds=0, shutter_close_seconds=1, shutter_samples=2)),
                {"op": "put_clip", "clip": {"id": "invalid-camera", "duration": 1, "camera_keys": [
                    {"time": 0, "eye": [0, 0, 1], "target": [0, 0, 0], "fov_degrees": 45},
                    {"time": 1, "eye": [0, 0, -1], "target": [0, 0, 0], "fov_degrees": 45}]}}])
            bad_document, revision = client.call({"op": "get_document"}), client.revision
            client.call({"op": "pose", "animation": {"clip": "invalid-camera", "time": 0.25}})
            bad = render("invalid-subframe.exr", clip="invalid-camera", moment=0.25, ok=False)
            sequence_bad = client.call({"op": "render_sequence", "request": {"directory": "invalid-subframes",
                "clip": "invalid-camera", "start": 0.25, "end": 0.25, "fps": 1, "format": "exr"}}, ok=False)
            assert not (root / "invalid-subframe.exr").exists()
            assert not (root / "invalid-subframes").exists()
            unchanged(bad_document, revision)
            result["subframe_rejection"] = [bad["error"], sequence_bad["error"]]
            client.apply([settings(film(shutter_open_seconds=-1, shutter_close_seconds=1, shutter_samples=64),
                                   aa=8, width=4096, height=4096)])
            render("over-budget.exr", ok=False)
            client.call({"op": "render_sequence", "request": {"directory": "over-budget-sequence", "clip": "travel",
                "start": 0, "end": 0, "fps": 1, "format": "exr"}}, ok=False)
            assert not (root / "over-budget.exr").exists() and not (root / "over-budget-sequence").exists()
            client.close()
            client = Client(binary, root, transcript, "cold-reopen")
            client.call({"op": "load", "path": "motion.mm3e-agent.json"}, mutation=True)
            assert client.call({"op": "get_document"}) == motion_document
            render("reopened.exr")
            render("reopened.png")
            reopened, reopened_header = decode_exr("reopened.exr")
            assert all(np.array_equal(reopened[name], motion[name]) for name in motion)
            assert reopened_header["mm3e:documentFingerprint"] == metadata["mm3e:documentFingerprint"]
            assert (root / "reopened.png").read_bytes() == (root / "motion.png").read_bytes()
            unchanged(motion_document, client.revision)
            client.close()
            client = None
        result.update({"passed": True, "status": "passed", "sample_times_seconds": sample_times,
                       "instant_width_pixels": int(np.ptp(static_columns) + 1),
                       "motion_width_pixels": int(np.ptp(motion_columns) + 1),
                       "checks": ["real JSONL structured authoring", "true premultiplied HDR with partial coverage",
                           "independent instantaneous samples reproduce every temporal RGB/A value exactly",
                           "center-time pinhole Z/N/material IDs unaffected by shutter and lens integration",
                           "PNG straight alpha with zero RGB for encoded alpha zero", "known-sphere raw depth",
                           "shutter/lens/reference/document/material metadata independently decoded",
                           "actual defocus differs at matched spatial sampling", "legacy RGB EXR compatibility",
                           "explicit RGBA-only EXR omits disabled data channels", "inside-origin depth zero survives EXR delivery",
                           "observations preserve authored state and revision", "cold reopen reproduces all EXR samples and PNG bytes",
                           "BMP alpha, non-beauty film, bad intermediate shutter pose and over-budget work reject without output"],
                       "not_verified": ["complete production readiness", "film character or clothing quality",
                           "production sampling convergence", "transmission or volumetric mattes", "hardware GPU output",
                           "OCIO transforms", "autonomous agent performance"],
                       "elapsed_seconds": time.monotonic() - started})
        result_path.write_text(json.dumps(result, indent=2) + "\n")
        print(json.dumps(result, indent=2))
    except Exception as error:
        result.update({"status": "failed", "error": repr(error), "traceback": traceback.format_exc(),
                       "elapsed_seconds": time.monotonic() - started})
        result_path.write_text(json.dumps(result, indent=2) + "\n")
        if client:
            client.abort()
        raise


if __name__ == "__main__":
    main()
