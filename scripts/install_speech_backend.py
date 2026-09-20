#!/usr/bin/env python3
"""Install or verify the pinned optional Rhubarb runtime in a project directory.

Uses only the official HTTPS archive recorded in speech-backend-lock.json, or an
explicit offline copy of that exact archive. Never overwrites a destination or
changes PATH/system installations. The pins are locally computed reproducibility
checks, not upstream signatures. Failed installations retain their work/evidence.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import platform
import shutil
import stat
import subprocess
import sys
import time
import urllib.parse
import urllib.request
import zipfile

LOCK_PATH = Path(__file__).with_name("speech-backend-lock.json")
MAX_ARCHIVE_BYTES = 128 * 1024 * 1024
MAX_EXPANDED_BYTES = 256 * 1024 * 1024
MAX_FILE_BYTES = 64 * 1024 * 1024
MAX_ENTRIES = 512
BLOCK = 1024 * 1024
RESERVED = {"CON", "PRN", "AUX", "NUL", *(f"COM{i}" for i in range(1, 10)),
            *(f"LPT{i}" for i in range(1, 10))}


def host_platform():
    operating_system, architecture = platform.system().lower(), platform.machine().lower()
    if operating_system not in ("linux", "windows") or architecture not in ("x86_64", "amd64"):
        return None
    return operating_system + "-x86_64"


def checked_path(name):
    """One portable canonical relative ZIP path, including Windows path hazards."""
    if not name or "\\" in name or "\x00" in name or name.startswith("/"):
        raise ValueError(f"unsafe archive path: {name!r}")
    parts = name.removesuffix("/").split("/")
    if any(not p or p in (".", "..") or any(c in p for c in ':<>"|?*')
           or p.endswith((".", " ")) or any(ord(c) < 32 for c in p)
           or p.split(".", 1)[0].upper() in RESERVED for p in parts):
        raise ValueError(f"unsafe archive component: {name!r}")
    return PurePosixPath(*parts)


def file_record(path):
    metadata = path.lstat()
    if not stat.S_ISREG(metadata.st_mode):
        raise ValueError(f"runtime member must be a regular file, not a symlink: {path}")
    if metadata.st_size > MAX_FILE_BYTES:
        raise ValueError(f"runtime member exceeds file budget: {path}")
    digest, total = hashlib.sha256(), 0
    with path.open("rb") as stream:
        while block := stream.read(BLOCK):
            total += len(block)
            if total > MAX_FILE_BYTES:
                raise ValueError("runtime member grew beyond its budget")
            digest.update(block)
    return {"bytes": total, "sha256": digest.hexdigest()}


def archive_record(path, expected):
    metadata = path.lstat()
    if not stat.S_ISREG(metadata.st_mode) or metadata.st_size != expected["archive_bytes"]:
        raise ValueError("archive must be a regular file with the exact pinned byte count")
    digest, total = hashlib.sha256(), 0
    with path.open("rb") as stream:
        while block := stream.read(BLOCK):
            total += len(block)
            if total > expected["archive_bytes"]:
                raise ValueError("archive grew beyond its pinned byte count")
            digest.update(block)
    if total != expected["archive_bytes"] or digest.hexdigest() != expected["archive_sha256"]:
        raise ValueError("archive SHA-256 does not match the pinned official release")


def load_lock():
    lock = json.loads(LOCK_PATH.read_text())
    if lock["format"] != "mm3e-speech-backend-lock-v1" or lock["backend"] != "rhubarb":
        raise ValueError("unsupported speech backend lock")
    for package in lock["platforms"].values():
        if not 0 < package["archive_bytes"] <= MAX_ARCHIVE_BYTES:
            raise ValueError("pinned archive exceeds bootstrap budget")
        if not package["files"] or len(package["files"]) > MAX_ENTRIES:
            raise ValueError("invalid runtime file count in lock")
        for name, record in package["files"].items():
            checked_path(name)
            if not 0 <= record["bytes"] <= MAX_FILE_BYTES or len(record["sha256"]) != 64:
                raise ValueError("invalid runtime member in lock")
    return lock


def allowed_https(url):
    value = urllib.parse.urlsplit(url)
    return (value.scheme == "https" and value.username is None and value.password is None
            and value.port in (None, 443) and value.hostname in
            {"github.com", "release-assets.githubusercontent.com", "objects.githubusercontent.com"})


class HttpsReleaseRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, request, fp, code, message, headers, newurl):
        if not allowed_https(newurl):
            raise ValueError("refusing a redirect outside official HTTPS release asset hosts")
        return super().redirect_request(request, fp, code, message, headers, newurl)


def download(package, target):
    url = package["url"]
    if not allowed_https(url) or urllib.parse.urlsplit(url).hostname != "github.com":
        raise ValueError("lock must name an official HTTPS GitHub release archive")
    opener = urllib.request.build_opener(HttpsReleaseRedirect)
    request = urllib.request.Request(url, headers={"User-Agent": "MM3E-speech-backend-bootstrap"})
    total, started = 0, time.monotonic()
    with opener.open(request, timeout=30) as response, target.open("xb") as output:
        if not allowed_https(response.url):
            raise ValueError("archive response is outside official HTTPS hosts")
        length = response.headers.get("Content-Length")
        if length is not None and int(length) != package["archive_bytes"]:
            raise ValueError("upstream archive byte count differs from the lock")
        while block := response.read(BLOCK):
            total += len(block)
            if total > package["archive_bytes"] or time.monotonic() - started > 180:
                raise ValueError("download exceeds pinned size or 180-second time budget")
            output.write(block)
    if total != package["archive_bytes"]:
        raise ValueError("download ended before the pinned archive byte count")


def selected_members(archive, package):
    entries = archive.infolist()
    if len(entries) > MAX_ENTRIES or sum(i.file_size for i in entries) > MAX_EXPANDED_BYTES:
        raise ValueError("ZIP exceeds entry or expanded-byte budget")
    paths, selected, files = {}, {}, set()
    for info in entries:
        path = checked_path(info.orig_filename)
        if info.orig_filename != info.filename:
            raise ValueError("ZIP member filename was normalized or truncated")
        if path.parts[0] != package["archive_root"]:
            raise ValueError("ZIP member is outside the pinned top-level directory")
        folded = str(path).casefold()
        if folded in paths:
            raise ValueError("duplicate or case-colliding ZIP member")
        paths[folded] = info.is_dir()
        file_type = stat.S_IFMT(info.external_attr >> 16)
        if file_type not in (0, stat.S_IFREG, stat.S_IFDIR):
            raise ValueError("ZIP contains a symlink or special filesystem member")
        if (file_type == stat.S_IFDIR and not info.is_dir()) or (file_type == stat.S_IFREG and info.is_dir()):
            raise ValueError("ZIP member file type and path disagree")
        if info.flag_bits & 1 or info.compress_type not in (zipfile.ZIP_STORED, zipfile.ZIP_DEFLATED):
            raise ValueError("ZIP encryption or compression method is unsupported")
        if info.file_size > MAX_FILE_BYTES or (info.is_dir() and info.file_size):
            raise ValueError("ZIP member exceeds file budget or directory contains data")
        if info.is_dir():
            continue
        files.add(folded)
        if len(path.parts) < 2:
            raise ValueError("ZIP root is a file")
        relative = str(PurePosixPath(*path.parts[1:]))
        if relative in (package["executable"], "LICENSE.md", "README.adoc") or relative.startswith("res/"):
            selected[relative] = info
    for name in paths:
        if any(str(parent).casefold() in files for parent in PurePosixPath(name).parents):
            raise ValueError("ZIP file collides with a member's parent directory")
    if set(selected) != set(package["files"]):
        raise ValueError("ZIP runtime file inventory differs from the pinned lock")
    for name, info in selected.items():
        if info.file_size != package["files"][name]["bytes"]:
            raise ValueError(f"ZIP runtime member byte count differs: {name}")
    return selected


def extract_runtime(archive_path, package, destination):
    # The caller creates a new private staging directory. Never use extractall:
    # every written member is selected, bounded, regular and checked independently.
    with zipfile.ZipFile(archive_path) as archive:
        selected = selected_members(archive, package)
        for name, info in sorted(selected.items()):
            target = destination.joinpath(*PurePosixPath(name).parts)
            target.parent.mkdir(parents=True, exist_ok=True)
            digest, count = hashlib.sha256(), 0
            with archive.open(info) as source, target.open("xb") as output:
                while block := source.read(BLOCK):
                    count += len(block)
                    if count > info.file_size or count > MAX_FILE_BYTES:
                        raise ValueError("decompressed member exceeds its declared budget")
                    digest.update(block)
                    output.write(block)
            if {"bytes": count, "sha256": digest.hexdigest()} != package["files"][name]:
                raise ValueError(f"runtime checksum differs from lock: {name}")
            target.chmod(0o755 if name == package["executable"] else 0o644)


def verify_files(destination, package, manifest=False):
    if destination.is_symlink() or not destination.is_dir():
        raise ValueError("runtime destination must be a regular directory")
    actual, total = {}, 0
    for path in destination.rglob("*"):
        if path.is_symlink():
            raise ValueError(f"runtime contains a symlink: {path}")
        if path.is_dir():
            continue
        name = path.relative_to(destination).as_posix()
        if manifest and name == "source.json":
            continue
        actual[name] = file_record(path)
        total += actual[name]["bytes"]
        if len(actual) > MAX_ENTRIES or total > MAX_EXPANDED_BYTES:
            raise ValueError("installed runtime exceeds file-count or byte budget")
    if actual != package["files"]:
        raise ValueError("installed runtime file inventory, byte count or SHA-256 differs from lock")
    return actual


def check_version(destination, selected_platform, package, version, skip):
    if skip:
        return {"status": "not_run", "reason": "explicit --skip-version-check"}
    if selected_platform != host_platform():
        raise ValueError("runtime platform differs from this host; explicitly use --skip-version-check for cross-host staging")
    result = subprocess.run([str((destination / package["executable"]).resolve()), "--version"],
                            cwd=destination, capture_output=True, timeout=30, check=False)
    output = result.stdout.decode("utf-8", errors="strict").strip()
    error = result.stderr.decode("utf-8", errors="strict").strip()
    if result.returncode != 0 or output != "Rhubarb Lip Sync version " + version or error:
        raise ValueError(f"pinned runtime version check failed: exit {result.returncode}, stdout={output[:400]!r}, stderr={error[:400]!r}")
    return {"status": "passed", "stdout": output, "exit_code": result.returncode}


def install(destination, package, selected_platform, lock, existing_archive, skip):
    destination = destination.absolute()
    # lexists also rejects dangling symlinks. mkdir remains the actual atomic
    # reservation, so a competing installer never merges with an existing folder.
    if os.path.lexists(destination):
        raise ValueError("destination already exists; select a NEW directory")
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.mkdir()
    work = destination / ".install-work"
    work.mkdir()
    try:
        if existing_archive is None:
            archive_path = work / package["archive_name"]
            download(package, archive_path)
            source_mode = "official_https_download"
        else:
            archive_path = existing_archive.absolute()
            source_mode = "explicit_existing_archive"
        archive_record(archive_path, package)
        runtime = work / "runtime"
        runtime.mkdir()
        extract_runtime(archive_path, package, runtime)
        files = verify_files(runtime, package)
        version = check_version(runtime, selected_platform, package, lock["version"], skip)
        source = {"format": "mm3e-speech-backend-source-v1", "backend": lock["backend"],
                  "version": lock["version"], "platform": selected_platform, "origin": package["url"],
                  "archive_bytes": package["archive_bytes"], "archive_sha256": package["archive_sha256"],
                  "source_mode": source_mode, "lock_sha256": hashlib.sha256(LOCK_PATH.read_bytes()).hexdigest(),
                  "integrity_note": lock["integrity_note"], "files": files, "version_check": version}
        # Only runtime members and provenance remain after successful installation.
        for child in runtime.iterdir():
            child.rename(destination / child.name)
        shutil.rmtree(work)
        with (destination / "source.json").open("x") as output:
            output.write(json.dumps(source, indent=2) + "\n")
        verify_files(destination, package, manifest=True)
        return {"status": "installed", "directory": str(destination),
                "executable": str(destination / package["executable"]), "version_check": version,
                "runtime_files": len(files), "archive_sha256": package["archive_sha256"]}
    except Exception as error:
        (destination / "install-failure.json").write_text(json.dumps({
            "status": "failed", "error": str(error), "retained_work_directory": str(work)}, indent=2) + "\n")
        raise


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    target = parser.add_mutually_exclusive_group(required=True)
    target.add_argument("--destination", type=Path, help="NEW project-local runtime directory")
    target.add_argument("--verify", type=Path, help="verify an existing installed runtime without modifying it")
    parser.add_argument("--platform", choices=("linux-x86_64", "windows-x86_64"), default=host_platform())
    parser.add_argument("--archive", type=Path, help="offline copy of the exact pinned archive")
    parser.add_argument("--skip-version-check", action="store_true", help="explicitly stage or inspect another host's files without executing them")
    args = parser.parse_args()
    if args.platform is None:
        parser.error("this host has no supported runtime; specify --platform for cross-host staging")
    if args.verify is not None and args.archive is not None:
        parser.error("--archive is only meaningful with --destination")
    lock = load_lock()
    package = lock["platforms"][args.platform]
    if args.verify is None:
        result = install(args.destination, package, args.platform, lock, args.archive, args.skip_version_check)
    else:
        files = verify_files(args.verify, package, manifest=True)
        manifest_path = args.verify / "source.json"
        metadata = manifest_path.lstat()
        if not stat.S_ISREG(metadata.st_mode) or metadata.st_size > BLOCK:
            raise ValueError("source manifest must be a regular file bounded to 1 MiB")
        source = json.loads(manifest_path.read_text())
        if (source.get("format") != "mm3e-speech-backend-source-v1" or source.get("files") != files
            or source.get("backend") != lock["backend"]
            or source.get("lock_sha256") != hashlib.sha256(LOCK_PATH.read_bytes()).hexdigest()
            or source.get("integrity_note") != lock["integrity_note"]
            or source.get("archive_sha256") != package["archive_sha256"]
            or source.get("platform") != args.platform or source.get("version") != lock["version"]
            or source.get("origin") != package["url"] or source.get("archive_bytes") != package["archive_bytes"]):
            raise ValueError("installed source manifest differs from the pinned package")
        version = check_version(args.verify, args.platform, package, lock["version"], args.skip_version_check)
        result = {"status": "verified", "directory": str(args.verify.absolute()),
                  "runtime_files": len(files), "version_check": version}
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, KeyError, zipfile.BadZipFile, subprocess.SubprocessError) as error:
        print(json.dumps({"status": "failed", "error": str(error)}), file=sys.stderr)
        sys.exit(1)
