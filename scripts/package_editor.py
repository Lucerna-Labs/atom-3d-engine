#!/usr/bin/env python3
"""Package and launch-check the actual editor for the current host platform.

The archive includes schemas, a runnable example, documentation and Cargo dependency
notices. It does not cross-compile or claim compatibility with untested operating systems.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import subprocess
import tempfile
import wave
import zipfile


ROOT = Path(__file__).resolve().parents[1]


def request(identifier, command, revision=None):
    result = {"id": identifier, "command": command}
    if revision is not None:
        result["expected_revision"] = revision
    return result


EXAMPLE = [
    request("author", {"op": "apply", "operations": [
        {"op": "set_settings", "settings": {"width": 64, "height": 48, "quality": "preview", "shadows": False, "ao": False}},
        {"op": "create", "object": {"id": "ball", "shape": {"type": "sphere", "radius": 0.4}, "position": [0, 0.9, 0]}},
        {"op": "put_clip", "clip": {"id": "move", "duration": 1, "tracks": [{"target": {"type": "object", "id": "ball"}, "keys": [{"time": 0}, {"time": 1, "translation": [0.5, 0, 0]}]}]}},
    ]}, 0),
    request("pose", {"op": "pose", "animation": {"clip": "move", "time": 0.5}}),
    request("image", {"op": "render", "path": "quickstart.png", "animation": {"clip": "move", "time": 0.5}}),
    request("save", {"op": "save", "path": "quickstart.mm3e-agent.json"}),
]


def run(binary, root, requests, extra_args=()):
    completed = subprocess.run(
        [str(binary), "--root", str(root), *map(str, extra_args)],
        input="".join(json.dumps(item) + "\n" for item in requests),
        text=True, capture_output=True, check=True, timeout=120,
    )
    responses = [json.loads(line) for line in completed.stdout.splitlines()]
    if len(responses) != len(requests) or any(not item.get("ok") for item in responses):
        raise RuntimeError(f"packaged editor acceptance failed: {responses}; stderr={completed.stderr}")
    return responses


def verify_reference_audio(binary, root, source):
    """Exercise the packaged bytes, real partial-shot exporter and embedded reload."""
    path = root / "reference.wav"
    path.write_bytes(source)
    responses = run(binary, root, [
        request("load-animation", {"op": "load", "path": "quickstart.mm3e-agent.json"}, 0),
        request("import-reference", {"op": "import_audio", "request": {"id": "voice", "path": "reference.wav"}}, 1),
        request("author-shot", {"op": "apply", "operations": [{"op": "put_shot", "shot": {
            "id": "line", "clip": "move", "rate": {"numerator": 24000, "denominator": 1001},
            "start_frame": 1001, "clip_frame_zero": 1001, "frame_count": 12,
            "audio": {"asset": "voice", "start_sample": 317},
        }}]}, 2),
        request("partial-shot", {"op": "render_shot", "request": {
            "shot": "line", "directory": "audio-check", "selection": {"start_frame": 1002, "frame_count": 2},
        }}),
        request("save-reference", {"op": "save", "path": "audio.mm3e-agent.json"}),
    ])
    if responses[1]["result"]["source_sha256"] != hashlib.sha256(source).hexdigest():
        raise RuntimeError("packaged reference audio fingerprint differs")
    manifest = json.loads((root / "audio-check/manifest.json").read_text())
    # Absolute whole-shot indices 1..3 preserve fractional sample-window phase.
    start, end = 317 + 16000 * 1001 // 24000, 317 + 3 * 16000 * 1001 // 24000
    with wave.open(str(path), "rb") as original:
        original.setpos(start)
        expected = original.readframes(end - start)
    with wave.open(str(root / "audio-check/audio.wav"), "rb") as delivered:
        if delivered.getnframes() != end - start or delivered.readframes(end - start) != expected:
            raise RuntimeError("packaged shot did not preserve exact source audio samples")
    if manifest["frame_count"] != 2 or manifest["audio"]["start_sample"] != start or manifest["audio"]["end_sample_exclusive"] != end:
        raise RuntimeError("packaged shot manifest has incorrect frame/audio bounds")
    path.unlink()  # The project must retain its original reference without this copy.
    reopened = run(binary, root, [
        request("reload-reference", {"op": "load", "path": "audio.mm3e-agent.json"}, 0),
        request("inspect-reference", {"op": "shot_state", "id": "line"}),
        request("native-budget", {"op": "project_budget"}),
    ])
    if reopened[1]["result"]["audio_asset"]["source_sha256"] != hashlib.sha256(source).hexdigest():
        raise RuntimeError("packaged project lost its embedded reference audio")
    budget = reopened[2]["result"]
    if budget["encoded_bytes"] != (root / "audio.mm3e-agent.json").stat().st_size or budget["encoded_bytes"] + budget["revision_reserve_bytes"] + budget["remaining_bytes"] != budget["limit_bytes"]:
        raise RuntimeError("packaged native-project budget differs from its saved encoding")


def dependency_notices():
    metadata = json.loads(subprocess.run(
        ["cargo", "metadata", "--locked", "--format-version", "1"],
        cwd=ROOT, capture_output=True, text=True, check=True,
    ).stdout)
    packages = {package["id"]: package for package in metadata["packages"]}
    nodes = {node["id"]: node for node in metadata["resolve"]["nodes"]}
    editor = next(package for package in metadata["packages"] if package["name"] == "mm3e-editor")
    pending, included = [editor["id"]], set()
    while pending:
        package_id = pending.pop()
        if package_id in included:
            continue
        included.add(package_id)
        for dependency in nodes[package_id]["deps"]:
            if any(kind["kind"] != "dev" for kind in dependency["dep_kinds"]):
                pending.append(dependency["pkg"])
    files, index = {}, []
    for package_id in sorted(included):
        package = packages[package_id]
        if not package["source"]:
            continue  # workspace source is covered by its root license
        directory = Path(package["manifest_path"]).parent
        license_paths = set()
        for pattern in ("LICENSE*", "LICENCE*", "COPYING*", "COPYRIGHT*", "NOTICE*"):
            license_paths.update(path for path in directory.glob(pattern) if path.is_file())
        if package.get("license_file"):
            license_paths.add(directory / package["license_file"])
        upstream_notices = ROOT / "scripts/package-notices" / f"{package['name']}-{package['version']}"
        if upstream_notices.is_dir():
            license_paths.update(path for path in upstream_notices.iterdir() if path.is_file())
        if not license_paths:
            raise RuntimeError(f"no bundled license file for {package['name']} {package['version']}")
        entry = {"name": package["name"], "version": package["version"], "license": package["license"], "repository": package["repository"], "files": []}
        for path in sorted(license_paths):
            name = f"third-party/{package['name']}-{package['version']}/{path.name}"
            files[name] = path.read_bytes()
            entry["files"].append(name)
        index.append(entry)
    files["third-party/index.json"] = (json.dumps(index, indent=2) + "\n").encode()
    return files


def verify_texture(binary, root, source):
    path = root / "texture-reference.png"
    path.write_bytes(source)
    responses = run(binary, root, [
        request("load-base", {"op": "load", "path": "quickstart.mm3e-agent.json"}, 0),
        request("import-texture", {"op": "import_texture", "request": {"id": "print", "path": path.name, "color_space": "srgb"}}, 1),
        request("texture-surface", {"op": "apply", "operations": [
            {"op": "create", "object": {"id": "printed-panel", "shape": {"type": "surface", "vertices": [[-.5, 0, 0], [.5, 0, 0], [.5, 1, 0], [-.5, 1, 0]], "triangles": [[0, 1, 2], [0, 2, 3]], "thickness_m": .01}, "material": {"albedo": [1, 1, 1]}}},
            {"op": "project_uvs", "request": {"id": "uv", "object": "printed-panel", "origin": [-.5, 0, 0], "axis_u": [1, 0, 0], "axis_v": [0, 1, 0], "meters_per_uv": [1, 1]}},
            {"op": "bind_texture", "binding": {"object": "printed-panel", "uv_set": "uv", "texture": "print"}},
            {"op": "set_camera", "camera": {"eye": [0, .5, 2], "target": [0, .5, 0], "fov_degrees": 40}},
        ]}, 2),
        request("texture-sample", {"op": "material_state", "request": {"points": [[-.25, .25, .005]]}}),
        request("texture-render", {"op": "render", "path": "texture-preview.png"}),
        request("texture-save", {"op": "save", "path": "texture.mm3e-agent.json"}),
    ])
    sample = responses[3]["result"]["samples"][0]
    if sample["object"] != "printed-panel" or any(abs(a-b) > 1e-6 for a, b in zip(sample["uv"], [.25, .25])) or sample["linear_albedo"] == [1, 1, 1]:
        raise RuntimeError("packaged texture did not reach native surface appearance")
    path.unlink()
    run(binary, root, [
        request("texture-reload", {"op": "load", "path": "texture.mm3e-agent.json"}, 0),
        request("texture-source", {"op": "export_texture", "request": {"id": "print", "path": "texture-restored.png"}}),
        request("texture-cold-render", {"op": "render", "path": "texture-cold.png"}),
    ])
    if (root / "texture-restored.png").read_bytes() != source or (root / "texture-cold.png").read_bytes() != (root / "texture-preview.png").read_bytes():
        raise RuntimeError("packaged texture/project/render cold reload differs")


def verify_material_maps(binary, root, sources):
    commands=[request("load-textured-base",{"op":"load","path":"texture.mm3e-agent.json"},0)]
    for index,(name,data) in enumerate(sources.items()):
        (root/f"material-{name}.png").write_bytes(data)
        commands.append(request("import-"+name,{"op":"import_texture","request":{"id":name,"path":f"material-{name}.png","color_space":"linear"}},index+1))
    commands.extend([
        request("material-maps",{"op":"apply","operations":[
            {"op":"update","id":"printed-panel","patch":{"material":{"albedo":[0.5,0.25,0.12],"roughness":1,"metallic":1,"emissive":[3,1,.25]}}},
            {"op":"bind_texture","binding":{"object":"printed-panel","uv_set":"uv","roughness":{"texture":"packed","channel":"r"},"metallic":{"texture":"packed","channel":"g"},"emissive":{"texture":"glow"},"normal":{"texture":"detail","variance_filter":True}}}
        ]},4),
        request("mapped-sample",{"op":"material_state","request":{"points":[[-.4,.23,.005]]}}),
        request("minified-sample",{"op":"material_state","request":{"points":[[-.4,.23,.005]],"footprint_m":1}}),
        request("mapped-render",{"op":"render","path":"material-preview.png"}),
        request("mapped-save",{"op":"save","path":"material.mm3e-agent.json"}),
    ])
    responses=run(binary,root,commands)
    sample=responses[5]["result"]["samples"][0]
    if sample["metallic"]<.8 or sum((a-b)**2 for a,b in zip(sample["geometric_normal"],sample["shading_normal"]))<.0025:
        raise RuntimeError("packaged metallic/normal maps did not affect the resolved surface")
    minified=responses[6]["result"]["samples"][0]
    if minified["normal_variance"]<=0 or minified["roughness"]<=minified["unfiltered_roughness"]:
        raise RuntimeError("packaged minified normal variance did not broaden material roughness")
    for name in sources:(root/f"material-{name}.png").unlink()
    cold=run(binary,root,[request("mapped-reload",{"op":"load","path":"material.mm3e-agent.json"},0),request("mapped-cold-render",{"op":"render","path":"material-cold.png"}),request("minified-cold",{"op":"material_state","request":{"points":[[-.4,.23,.005]],"footprint_m":1}})])
    if cold[2]["result"]["samples"][0]!=minified:
        raise RuntimeError("packaged normal variance changed after embedded-source reload")
    if (root/"material-preview.png").read_bytes()!=(root/"material-cold.png").read_bytes():
        raise RuntimeError("packaged material maps changed after embedded-source reload")



def verify_render_jobs(binary, root):
    first=run(binary,root,[
        request("job-load",{"op":"load","path":"quickstart.mm3e-agent.json"},0),
        request("job-create",{"op":"create_render_job","request":{"type":"sequence","directory":"render-job","clip":"move","start":0,"end":1,"fps":2}}),
        request("job-step",{"op":"step_render_job","directory":"render-job"}),
        request("job-cancel",{"op":"cancel_render_job","directory":"render-job"}),
        request("job-cancelled-step",{"op":"step_render_job","directory":"render-job"}),
    ])
    if first[2]["result"]["completed_frames"]!=1 or first[4]["result"]["status"]!="cancelled":
        raise RuntimeError("packaged job step/cancellation failed")
    prefix=(root/"render-job/frame_0000.png").read_bytes()
    # Fresh process has an empty live document; the job must use its frozen snapshot.
    reopened=run(binary,root,[
        request("job-resume",{"op":"resume_render_job","directory":"render-job"}),
        request("job-finish",{"op":"step_render_job","directory":"render-job","max_frames":32}),
        request("job-verify",{"op":"render_job_state","directory":"render-job","verify_outputs":True}),
        request("job-reference-load",{"op":"load","path":"quickstart.mm3e-agent.json"},0),
        request("job-reference",{"op":"render_sequence","request":{"directory":"job-reference","clip":"move","start":0,"end":1,"fps":2}}),
    ])
    if reopened[1]["result"]["status"]!="complete" or reopened[1]["result"]["completed_frames"]!=3 or not reopened[2]["result"]["outputs_verified"]:
        raise RuntimeError("packaged render job did not finish and verify after restart")
    if (root/"render-job/frame_0000.png").read_bytes()!=prefix:
        raise RuntimeError("packaged render job replaced a completed frame")
    for index in range(3):
        name=f"frame_{index:04}.png"
        if (root/"render-job"/name).read_bytes()!=(root/"job-reference"/name).read_bytes():
            raise RuntimeError("packaged resumed frame differs from synchronous reference")



def verify_animation_layers(binary, root):
    result=run(binary,root,[
        request("layers-load",{"op":"load","path":"quickstart.mm3e-agent.json"},0),
        request("layers-author",{"op":"apply","operations":[
            {"op":"put_clip","clip":{"id":"bob","duration":1,"tracks":[{"target":{"type":"object","id":"ball"},"keys":[{"time":0,"translation":[0,.2,0]}]}]}},
            {"op":"put_clip","clip":{"id":"layered","duration":1,"layers":[{"id":"body","clip":"move"},{"id":"accent","clip":"bob","mode":"additive","weight":.5}]}}
        ]},1),
        request("layers-pose",{"op":"pose","animation":{"clip":"layered","time":.5}}),
        request("layers-render",{"op":"render","path":"layers-before.png","animation":{"clip":"layered","time":.5}}),
        request("layers-save",{"op":"save","path":"layers.mm3e-agent.json"}),
    ])
    pose=result[2]["result"];ball=next(o for o in pose["objects"] if o["id"]=="ball")
    if any(abs(a-b)>2e-6 for a,b in zip(ball["position"],[.25,1,0])) or len(pose["layer_samples"])!=2:
        raise RuntimeError("packaged body/additive layers did not compose actual object position")
    cold=run(binary,root,[
        request("layers-reload",{"op":"load","path":"layers.mm3e-agent.json"},0),
        request("layers-cold-render",{"op":"render","path":"layers-after.png","animation":{"clip":"layered","time":.5}}),
        request("layers-edit",{"op":"apply","operations":[{"op":"edit_layer","request":{"clip":"layered","action":{"op":"upsert","layer":{"id":"accent","clip":"bob","mode":"additive","weight":0}}}}]},1),
        request("layers-edited-pose",{"op":"pose","animation":{"clip":"layered","time":.5}}),
    ])
    if (root/"layers-before.png").read_bytes()!=(root/"layers-after.png").read_bytes():
        raise RuntimeError("packaged layered image changed after cold reload")
    ball=next(o for o in cold[3]["result"]["objects"] if o["id"]=="ball")
    if any(abs(a-b)>2e-6 for a,b in zip(ball["position"],[.25,.9,0])):
        raise RuntimeError("packaged sparse layer edit did not preserve underlying motion")



def verify_speech(binary, root, source):
    (root/"speech-reference.wav").write_bytes(source)
    values={"A":(0,0,0),"B":(.12,0,.35),"C":(.5,0,.2),"D":(1,0,.2),"E":(.3,.5,-.1),"F":(.1,1,-.3),"X":(0,0,0)}
    profile={"poses":[{"shape":shape,"values":[{"type":"face","face":"face","channel":channel,"value":value} for channel,value in zip(("jaw_open","lip_round","lip_wide"),pose)]} for shape,pose in values.items()]}
    prepared=run(binary,root,[
        request("speech-backend",{"op":"speech_backend_state"}),
        request("speech-load",{"op":"load","path":"quickstart.mm3e-agent.json"},0),
        request("speech-import",{"op":"import_audio","request":{"id":"speech-voice","path":"speech-reference.wav"}},1),
        request("speech-face",{"op":"apply","operations":[
            {"op":"create","object":{"id":"actor/head","shape":{"type":"ellipsoid","radii":[.8,1,.8]}}},
            {"op":"create","object":{"id":"actor/left_eye","position":[.28,.2,.78],"shape":{"type":"sphere","radius":.15}}},
            {"op":"create","object":{"id":"actor/right_eye","position":[-.28,.2,.78],"shape":{"type":"sphere","radius":.15}}},
            {"op":"create_face","request":{"id":"face","character":"actor"}},
            {"op":"set_camera","camera":{"eye":[0,0,4],"target":[0,0,0],"fov_degrees":38}}
        ]},2),
        request("speech-analyze",{"op":"analyze_speech","request":{"audio":"speech-voice","directory":"speech-analysis","dialogue_hint":"Hello. We make animated characters."}}),
        request("speech-prepared",{"op":"save","path":"speech-prepared.json"}),
    ])
    if not prepared[0]["result"]["configured"]:raise RuntimeError("packaged speech runtime was not discovered")
    analysis=prepared[4]["result"]
    if len(analysis["cues"])<3 or not any(c["shape"]!="X" for c in analysis["cues"]):raise RuntimeError("packaged recognizer returned no varied speech cues")
    probe=next(c["start_cs"]/100 for c in analysis["cues"] if c["shape"]!="X")
    generated=run(binary,root,[
        request("speech-reload",{"op":"load","path":"speech-prepared.json"},0),
        request("speech-generate",{"op":"generate_lip_sync","request":{"analysis_path":"speech-analysis/analysis.json","analysis_sha256":analysis["analysis_sha256"],"clip":"spoken","profile":profile}},1),
        request("speech-render",{"op":"render","path":"speech-before.png","animation":{"clip":"spoken","time":probe}}),
        request("speech-save",{"op":"save","path":"speech-project.json"}),
    ])
    if generated[1]["result"]["edited"]:raise RuntimeError("fresh packaged lip curves report edited provenance")
    (root/"speech-reference.wav").unlink()
    cold=run(binary,root,[request("speech-no-backend",{"op":"speech_backend_state"}),request("speech-cold-load",{"op":"load","path":"speech-project.json"},0),request("speech-cold-render",{"op":"render","path":"speech-after.png","animation":{"clip":"spoken","time":probe}}),request("speech-source-export",{"op":"export_audio","request":{"id":"speech-voice","path":"speech-recovered.wav","range":analysis["source"]["range"]}})],extra_args=("--no-speech-backend",))
    if cold[0]["result"]["configured"] or (root/"speech-before.png").read_bytes()!=(root/"speech-after.png").read_bytes():raise RuntimeError("packaged baked speech changed without backend")
    with wave.open(str(root/"speech-recovered.wav"),"rb") as recovered:
        import io
        with wave.open(io.BytesIO(source),"rb") as original:
            if recovered.readframes(recovered.getnframes())!=original.readframes(original.getnframes()):raise RuntimeError("packaged speech analysis changed original PCM")



def verify_speech_example(binary, root, files):
    guide=root/"speech-guide";guide.mkdir();(guide/"examples").mkdir()
    (guide/"examples/dialogue-reference.wav").write_bytes(files["examples/dialogue-reference.wav"])
    requests=[json.loads(line) for line in files["examples/speech-quickstart.jsonl"].decode().splitlines()]
    output=run(binary,guide,requests)
    if not (guide/"speech-project.json").is_file() or output[5]["result"]["frame_count"]!=29:
        raise RuntimeError("packaged speech quickstart did not produce its declared shot/project")
    manifest=json.loads((guide/"speech-demo/manifest.json").read_text())
    if manifest["audio"]["sample_frames"]!=46400:
        raise RuntimeError("packaged speech quickstart lost its exact selected audio range")



def verify_joint_limits(binary, root):
    import math
    responses=run(binary,root,[
        request("limits-load",{"op":"load","path":"quickstart.mm3e-agent.json"},0),
        request("limits-author",{"op":"apply","operations":[
            {"op":"set_joints","joints":[{"id":"hinge","pivot":[0,0,0],"objects":["ball"]}]},
            {"op":"set_joint_limit","id":"hinge","limit":{"type":"hinge","axis":[0,0,1],"min_degrees":-30,"max_degrees":30,"mode":"project"}},
            {"op":"put_clip","clip":{"id":"limited","duration":1,"tracks":[{"target":{"type":"joint","id":"hinge"},"keys":[{"time":0,"rotation_degrees":[0,0,60]}]}]}}
        ]},1),
        request("limits-observe",{"op":"joint_limit_state","animation":{"clip":"limited","time":0}}),
        request("limits-pose",{"op":"pose","animation":{"clip":"limited","time":0}}),
        request("limits-render",{"op":"render","path":"limits-before.png","animation":{"clip":"limited","time":0}}),
        request("limits-save",{"op":"save","path":"limits-project.json"}),
    ])
    if not responses[2]["result"]["joints"][0]["would_project"] or not responses[3]["result"]["joint_limits"][0]["applied"]:
        raise RuntimeError("packaged hinge limit did not report applied geometry projection")
    position=next(o for o in responses[3]["result"]["objects"] if o["id"]=="ball")["position"]
    if max(abs(a-b) for a,b in zip(position,[-.45,.9*math.cos(math.pi/6),0]))>2e-6:
        raise RuntimeError("packaged hinge geometry does not match explicit 30 degree reference")
    reopened=run(binary,root,[
        request("limits-reload",{"op":"load","path":"limits-project.json"},0),
        request("limits-cold-render",{"op":"render","path":"limits-after.png","animation":{"clip":"limited","time":0}}),
        request("limits-clear",{"op":"apply","operations":[{"op":"clear_joint_limit","id":"hinge"}]},1),
        request("limits-restored",{"op":"pose","animation":{"clip":"limited","time":0}}),
    ])
    if (root/"limits-before.png").read_bytes()!=(root/"limits-after.png").read_bytes():
        raise RuntimeError("packaged constrained image changed after cold reload")
    position=next(o for o in reopened[3]["result"]["objects"] if o["id"]=="ball")["position"]
    if max(abs(a-b) for a,b in zip(position,[-.9*math.sin(math.pi/3),.45,0]))>2e-6:
        raise RuntimeError("clearing packaged joint limit did not restore original authored motion")


def verify_usd_textures(binary, root, source):
    response = run(binary, root, [
        request("usd-texture-load", {"op": "load", "path": "texture.mm3e-agent.json"}, 0),
        request("usd-texture-export", {"op": "export_usd", "request": {
            "path": "printed-panel.usda", "object_ids": ["printed-panel"],
            "bounds_min": [-.56, -.06, -.02], "bounds_max": [.56, 1.06, .02],
            "resolution": [32, 32, 16], "max_surface_error_m": .04, "max_field_residual": .01,
            "texture_delivery": {"filtering": "reader_defined", "max_uv_error_texels": .25, "max_refinement_passes": 8},
        }}),
    ])[1]["result"]
    directory = Path(response["asset_directory"])
    manifest = json.loads((directory / "manifest.json").read_text())
    layer = (root / "printed-panel.usda").read_bytes()
    if hashlib.sha256(layer).hexdigest() != manifest["layer_sha256"]:
        raise RuntimeError("packaged USD layer hash differs from asset manifest")
    original = f"source-{hashlib.sha256(source).hexdigest()}.png"
    if (directory / original).read_bytes() != source:
        raise RuntimeError("packaged USD lost original PNG bytes")
    for record in manifest["assets"]:
        data = (directory / record["file"]).read_bytes()
        if len(data) != record["bytes"] or hashlib.sha256(data).hexdigest() != record["sha256"]:
            raise RuntimeError("packaged USD image asset failed manifest verification")
    if b"primvars:st" not in layer or b'"UsdUVTexture"' not in layer:
        raise RuntimeError("packaged USD omitted texture coordinates or material graph")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True, help="new ZIP file; parent must exist")
    parser.add_argument("--speech-backend-dir", type=Path, help="include the pinned Linux Rhubarb runtime and verify real speech analysis")
    parser.add_argument("--platform", default=f"{platform.system().lower()}-{platform.machine().lower()}")
    args = parser.parse_args()
    binary, output = args.binary.resolve(strict=True), args.output.resolve()
    if not re.fullmatch(r"[A-Za-z0-9_.-]+", args.platform):
        parser.error("platform must contain only letters, digits, dot, underscore or hyphen")
    if output.exists() or not output.parent.is_dir():
        parser.error("output must be a new file with an existing parent")
    binary_name = "mm3e-editor.exe" if os.name == "nt" else "mm3e-editor"
    files = {
        f"bin/{binary_name}": binary.read_bytes(),
        "LICENSE": (ROOT / "LICENSE").read_bytes(),
        "examples/speech-quickstart.jsonl": (ROOT / "mm3e-editor/examples/speech-quickstart.jsonl").read_bytes(),
        "examples/lip-sync-profile.json": (ROOT / "mm3e-editor/examples/lip-sync-profile.json").read_bytes(),
        "examples/quickstart.jsonl": ("".join(json.dumps(item) + "\n" for item in EXAMPLE)).encode(),
        "examples/dialogue-reference.wav": (ROOT / "mm3e-editor/tests/fixtures/dialogue/hello-reference.wav").read_bytes(),
        "examples/dialogue-reference-provenance.json": (ROOT / "mm3e-editor/tests/fixtures/dialogue/provenance.json").read_bytes(),
        "examples/uv-checker.png": (ROOT / "mm3e-editor/tests/fixtures/textures/uv-checker.png").read_bytes(),
        "docs/EDITOR_GUIDE.md": (ROOT / "mm3e-editor/README.md").read_bytes(),
    }
    for name in ("install_speech_backend.py","speech-backend-lock.json"):
        files["setup/"+name]=(ROOT/"scripts"/name).read_bytes()
    for name in ("packed","detail","glow"):
        files[f"examples/material-{name}.png"]=(ROOT / f"mm3e-editor/tests/fixtures/material_maps/{name}.png").read_bytes()
    for name in ("AGENT_EDITOR_ANIMATION.md", "PRODUCTION_READINESS.md", "PRODUCTION_PASS_VERIFICATION.md", "CONTACT_AND_USD_VERIFICATION.md", "SEWING_AND_CURVED_DELIVERY_VERIFICATION.md", "DEFORMATION_VERIFICATION.md", "DURABLE_PROJECTS.md", "FACIAL_CONTROLS.md", "GARMENTS.md", "CLOTH_SIMULATION.md", "SEWN_PANELS.md", "DEFORMATION.md", "IK_CONTROLS.md", "PATTERNS.md", "IK_AND_PATTERN_VERIFICATION.md", "DIALOGUE_SHOTS.md", "DIALOGUE_ACCEPTANCE.md", "DIALOGUE_VERIFICATION.md", "PROJECT_BUDGET_VERIFICATION.md", "STORAGE_FAILURE_VERIFICATION.md", "SURFACE_ATTRIBUTES.md", "UV_TEXTURES.md", "TEXTURE_VERIFICATION.md", "MATERIAL_MAPS.md", "MATERIAL_MAP_VERIFICATION.md", "NORMAL_FILTERING.md", "RENDER_JOBS.md", "ANIMATION_LAYERS.md", "SPEECH_LIP_SYNC.md", "JOINT_LIMITS.md", "TEXTURED_USD.md", "THIN_SURFACE_DELIVERY.md", "NATIVE_TRIANGLE_SURFACES.md", "USD_DELIVERY.md", "BOOLEAN_BOUNDARIES.md", "FILM_SHOTS.md"):
        files[f"docs/{name}"] = (ROOT / "docs" / name).read_bytes()
    files.update(dependency_notices())
    speech_files={}
    if args.speech_backend_dir:
        import install_speech_backend as speech_installer
        if args.platform!="linux-x86_64" or speech_installer.host_platform()!="linux-x86_64":
            parser.error("bundled speech process control is currently supported on Linux x86_64 only")
        speech_lock=speech_installer.load_lock();runtime=speech_lock["platforms"][args.platform]
        speech_installer.verify_files(args.speech_backend_dir,runtime,manifest=True)
        speech_installer.check_version(args.speech_backend_dir,args.platform,runtime,speech_lock["version"],False)
        speech_files={"tools/rhubarb/"+name:(args.speech_backend_dir/name).read_bytes() for name in runtime["files"]}
        speech_files["tools/rhubarb/source-lock.json"]=(json.dumps(speech_lock,indent=2)+"\n").encode()
        files.update(speech_files)
        files["third-party/speech-backend.json"]=(json.dumps({"name":"Rhubarb Lip Sync","version":speech_lock["version"],"release":speech_lock["release_url"],"license_notice":"tools/rhubarb/LICENSE.md","integrity_note":speech_lock["integrity_note"]},indent=2)+"\n").encode()

    with tempfile.TemporaryDirectory(prefix="mm3e-package-check-") as scratch:
        check_root = Path(scratch)
        (check_root / "bin").mkdir()
        staged_binary = check_root / "bin" / binary_name
        staged_binary.write_bytes(files[f"bin/{binary_name}"])
        staged_binary.chmod(0o755)
        for name,data in speech_files.items():
            target=check_root/name;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(data)
            if name=="tools/rhubarb/rhubarb":target.chmod(0o755)

        describe = run(staged_binary, check_root, [request("schema", {"op": "describe"})])[0]["result"]
        files["schemas/describe.json"] = (json.dumps(describe, indent=2) + "\n").encode()
        run(staged_binary, check_root, EXAMPLE)
        if not (check_root / "quickstart.png").is_file() or not (check_root / "quickstart.mm3e-agent.json").is_file():
            raise RuntimeError("packaged executable did not create the example image and project")
        reopened = run(staged_binary, check_root, [request("reload", {"op": "load", "path": "quickstart.mm3e-agent.json"}, 0)])
        if reopened[0]["result"]["object_count"] != 1:
            raise RuntimeError("packaged executable did not reopen its saved object")
        verify_render_jobs(staged_binary, check_root)
        verify_animation_layers(staged_binary, check_root)
        verify_joint_limits(staged_binary, check_root)
        if speech_files:
            verify_speech(staged_binary,check_root,files["examples/dialogue-reference.wav"])
            verify_speech_example(staged_binary,check_root,files)
        verify_reference_audio(staged_binary, check_root, files["examples/dialogue-reference.wav"])
        verify_texture(staged_binary, check_root, files["examples/uv-checker.png"])
        verify_usd_textures(staged_binary, check_root, files["examples/uv-checker.png"])
        verify_material_maps(staged_binary,check_root,{name:files[f"examples/material-{name}.png"] for name in ("packed","detail","glow")})
    files["README.txt"] = (
        "MM3E agent editor\n"
        f"Host package: {args.platform}\n"
        "Extract the archive and create an output directory.\n"
        f"Linux/macOS: chmod +x bin/{binary_name}; mkdir output; bin/{binary_name} --root output < examples/quickstart.jsonl\n"
        "PowerShell: New-Item -ItemType Directory output; Get-Content examples/quickstart.jsonl | & ./bin/mm3e-editor.exe --root output\n"
        "The output directory must be new for this example. It writes a PNG and native project.\n"
        "Send one JSON request per line; describe returns the complete current schema.\n"
        f"For durable work, run bin/{binary_name} --root output --project character.mm3e-agent.json.\n"
        "This archive was launch-tested on its packaging host. It is not a cross-platform binary or a film-production certification.\n"
        "Documentation source links refer to the source repository; generated schemas are included in schemas/describe.json.\n"
        "Third-party license texts and their package versions are included under third-party/.\n"
    ).encode()
    manifest = {
        "format": "mm3e-editor-package-v1", "platform": args.platform,
        "binary": f"bin/{binary_name}", "host": platform.platform(),
        "acceptance": ["copied binary JSONL schema", "archive checksum verification", "extracted binary example posed render", "native save", "fresh process load", "packaged reference WAV import and exact partial-shot audio export", "embedded reference cold reload without source WAV", "packaged PNG texture import, planar UVs, material sampling and rendering", "exact texture and image cold reload without source PNG", "material data/normal maps, minified normal variance and exact render reload", "bounded render job, cancellation, frozen-scene restart and exact frame delivery", "layered motion, sparse layer edits and exact cold render reload", "joint-limit projection, restored authored motion and exact cold rendering", "textured composed-surface USD, original PNG preservation and asset manifest hashes"],
        "files": {name: {"bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()} for name, data in sorted(files.items())},
    }
    if speech_files:manifest["acceptance"].append("bundled real speech recognition, mapped editable mouth curves, exact audio and backend-free cold rendering")
    files["manifest.json"] = (json.dumps(manifest, indent=2) + "\n").encode()
    with zipfile.ZipFile(output, mode="x", compression=zipfile.ZIP_DEFLATED) as archive:
        for name, data in sorted(files.items()):
            info = zipfile.ZipInfo(name)
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = (0o100755 if name in (f"bin/{binary_name}","tools/rhubarb/rhubarb") else 0o100644) << 16
            archive.writestr(info, data)
    with tempfile.TemporaryDirectory(prefix="mm3e-extracted-package-check-") as scratch:
        extracted = Path(scratch)
        with zipfile.ZipFile(output) as archive:
            for name, record in manifest["files"].items():
                data = archive.read(name)
                if len(data) != record["bytes"] or hashlib.sha256(data).hexdigest() != record["sha256"]:
                    raise RuntimeError(f"archive checksum failed: {name}")
            archive.extractall(extracted)
        extracted_binary = extracted / "bin" / binary_name
        extracted_binary.chmod(0o755)
        if speech_files:(extracted/"tools/rhubarb/rhubarb").chmod(0o755)
        render_root = extracted / "output"
        render_root.mkdir()
        run(extracted_binary, render_root, EXAMPLE)
        run(extracted_binary, render_root, [request("reload", {"op": "load", "path": "quickstart.mm3e-agent.json"}, 0)])
        verify_render_jobs(extracted_binary, render_root)
        verify_animation_layers(extracted_binary, render_root)
        verify_joint_limits(extracted_binary, render_root)
        if speech_files:
            verify_speech(extracted_binary,render_root,(extracted/"examples/dialogue-reference.wav").read_bytes())
            verify_speech_example(extracted_binary,render_root,files)
        verify_reference_audio(extracted_binary, render_root, (extracted / "examples/dialogue-reference.wav").read_bytes())
        verify_texture(extracted_binary, render_root, (extracted / "examples/uv-checker.png").read_bytes())
        verify_usd_textures(extracted_binary, render_root, (extracted / "examples/uv-checker.png").read_bytes())
        verify_material_maps(extracted_binary,render_root,{name:(extracted/f"examples/material-{name}.png").read_bytes() for name in ("packed","detail","glow")})
    print(json.dumps({"package": str(output), "platform": args.platform, "sha256": hashlib.sha256(output.read_bytes()).hexdigest(), "files": len(files), "acceptance": manifest["acceptance"]}, indent=2))


if __name__ == "__main__":
    main()
