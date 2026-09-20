from pathlib import Path
import concurrent.futures, hashlib, json, shutil, subprocess, time
ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "artifacts/production-corrections-20260919-r3"
OUT.mkdir(exist_ok=False)
records = []
def run(name, command):
    started = time.monotonic()
    print("start", name, flush=True)
    with (OUT / (name + ".log")).open("x") as log:
        result = subprocess.run(command, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT)
    record = dict(name=name, command=command, exit_code=result.returncode, seconds=time.monotonic()-started)
    print(json.dumps(record), flush=True)
    return record
def manifest():
    files = {ROOT / name for name in ("Cargo.toml", "Cargo.lock", "rustfmt.toml")}
    for name in ("mm3e-kit", "mm3e-orchestrator", "mm3e-editor"):
        package = ROOT / name
        files.add(package / "Cargo.toml")
        for folder in ("src", "tests", "examples"):
            files.update(p for p in (package / folder).rglob("*") if p.is_file())
        if (package / "build.rs").is_file(): files.add(package / "build.rs")
    files.update(p for p in (ROOT / "scripts").rglob("*") if p.is_file() and "__pycache__" not in p.parts)
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(files)}
source = manifest()
previous = json.loads((ROOT / "artifacts/production-corrections-20260919-r2/source-sha256.json").read_text())
changed = [name for name in sorted(set(source) | set(previous)) if source.get(name) != previous.get(name)]
assert changed == ["mm3e-editor/tests/usd_delivery.rs"], changed
(OUT / "full-suite-evidence.json").write_text(json.dumps(dict(log="../production-corrections-20260919-r2/tests.log", source_manifest="../production-corrections-20260919-r2/source-sha256.json", only_change_since_full_suite=changed, change="equivalent array-chunk iteration in USD test parser; runtime source unchanged"), indent=2)+"\n")
(OUT / "source-sha256.json").write_text(json.dumps(source, indent=2)+"\n")
for name, command in [
    ("format-check", ["cargo", "fmt", "--all", "--", "--check"]),
    ("tests", ["cargo", "test", "--offline", "-p", "mm3e-editor", "--test", "usd_delivery", "--no-fail-fast"]),
    ("clippy", ["cargo", "clippy", "--offline", "-p", "mm3e-kit", "-p", "mm3e-orchestrator", "-p", "mm3e-editor", "--all-targets", "--", "-D", "warnings"]),
    ("release-build", ["cargo", "build", "--offline", "--release", "-p", "mm3e-editor"]),
]:
    record = run(name, command)
    records.append(record)
    if name != "tests" and record["exit_code"]:
        (OUT / "verification.json").write_text(json.dumps(records, indent=2)+"\n")
        raise SystemExit(record["exit_code"])
assert source == manifest(), "source changed during verification"
with (OUT / "toolchain.txt").open("x") as log:
    for command in (["rustc", "-Vv"], ["cargo", "-V"]): subprocess.run(command, stdout=log, stderr=subprocess.STDOUT, check=True)
binary = OUT / "mm3e-editor"
shutil.copy2(ROOT / "target/release/mm3e-editor", binary)
sha = hashlib.sha256(binary.read_bytes()).hexdigest()
(OUT / "binary-sha256.txt").write_text(sha+"  mm3e-editor\n")
print(json.dumps(dict(binary_sha256=sha, source_files=len(source))), flush=True)
cases = []
for name, flag in [("face", "--editor"), ("deformed_face", "--editor"), ("animation_layer", "--editor"), ("sewing", "--executable"), ("speech", "--editor")]:
    output = ROOT / "artifacts" / ("agent-"+name.replace("_", "-")+"-20260919-r3")
    cases.append((name+"-acceptance", ["python3", str(ROOT/"scripts"/("agent_"+name+"_acceptance.py")), flag, str(binary), "--output", str(output)]))
cases.append(("textured-usd-acceptance", ["python3", str(ROOT/"scripts/agent_textured_usd_acceptance.py"), "--binary", str(binary), "--output", str(ROOT/"artifacts/textured-usd-acceptance-20260919-r3"), "--fixture-root", str(ROOT/"artifacts/material-maps-normal-filter-20260907-opt-in"), "--reader-path", str(ROOT/".dependencies/usd-reader"), "--exr-reader-path", str(ROOT/".dependencies/exr-reader"), "--challenge-root", str(ROOT/"artifacts/textured-usd-acceptance-20260907-final")]))
with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
    records.extend(pool.map(lambda item: run(*item), cases))
after = manifest()
(OUT / "source-after-verification-sha256.json").write_text(json.dumps(after, indent=2)+"\n")
assert source == after, "source changed during acceptance"
assert hashlib.sha256(binary.read_bytes()).hexdigest() == sha
(OUT / "verification.json").write_text(json.dumps(dict(binary_sha256=sha, source_files=len(source), source_stable=True, records=records), indent=2)+"\n")
print("Verification complete; preserved failures:", [r["name"] for r in records if r["exit_code"]], flush=True)
