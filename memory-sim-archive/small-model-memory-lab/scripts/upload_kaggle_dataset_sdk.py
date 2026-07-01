#!/usr/bin/env python
"""Upload the packaged lab export to Kaggle using the bearer-token SDK path.

The classic `kaggle` CLI expects username/key credentials. Newer Kaggle tokens
can work through `KAGGLE_API_TOKEN`, so this script uses `kagglesdk` directly.
It uploads top-level folders as zip files and keeps datasets private unless
`--public` is explicitly passed.
"""

from __future__ import annotations

import argparse
import json
import mimetypes
import os
import shutil
import tempfile
from dataclasses import dataclass
from pathlib import Path

import requests
from requests import HTTPError
from kagglesdk import KaggleClient, KaggleEnv
from kagglesdk.blobs.types.blob_api_service import (
    ApiBlobType,
    ApiStartBlobUploadRequest,
)
from kagglesdk.datasets.types.dataset_api_service import (
    ApiDatasetColumn,
    ApiCreateDatasetRequest,
    ApiCreateDatasetVersionRequest,
    ApiCreateDatasetVersionRequestBody,
    ApiDatasetNewFile,
)


DEFAULT_EXPORT_ROOT = (
    Path(__file__).resolve().parents[1]
    / "kaggle_export"
    / "small-model-memory-lab"
)


@dataclass(frozen=True)
class Payload:
    path: Path
    kaggle_name: str


def load_token() -> str:
    token = os.environ.get("KAGGLE_API_TOKEN")
    if token:
        return token

    cred_path = Path.home() / ".kaggle" / "kaggle.json"
    if not cred_path.exists():
        raise FileNotFoundError(
            "No KAGGLE_API_TOKEN env var and no ~/.kaggle/kaggle.json file found."
        )
    creds = json.loads(cred_path.read_text(encoding="utf-8"))
    token = creds.get("key")
    if not token:
        raise ValueError("~/.kaggle/kaggle.json does not contain a key field.")
    return token


def build_payloads(export_root: Path, tmp_dir: Path, *, zip_folders: bool) -> list[Payload]:
    payloads: list[Payload] = []
    for name in [
        "README.md",
        "THEORY_OF_APPROACH.md",
        "TODO.md",
        "KAGGLE_PROJECT_OVERVIEW.md",
        "CYBER_MEMORY_LORA_RESULTS.csv",
        "STRUCTURAL_PRIMITIVE_LORA_RESULTS.csv",
        "RUST_CYBER_DEFENDER_GENERATED.tsv",
        "CYBER_MEMORY_LORA_RESULTS.json",
        "STRUCTURAL_PRIMITIVE_LORA_RESULTS.json",
        "dataset-metadata.json",
        "EXPORT_MANIFEST.json",
    ]:
        path = export_root / name
        if path.exists():
            payloads.append(Payload(path=path, kaggle_name=name))

    for name in ["assets", "benchmarks", "corpus", "data", "docs", "loras", "runs", "scripts", "kaggle_notebooks"]:
        source = export_root / name
        if source.exists():
            if zip_folders:
                archive_base = tmp_dir / name
                archive = Path(shutil.make_archive(str(archive_base), "zip", root_dir=source))
                payloads.append(Payload(path=archive, kaggle_name=archive.name))
            else:
                for path in sorted(source.rglob("*")):
                    if path.is_file():
                        if "__pycache__" in path.parts or path.suffix == ".pyc":
                            continue
                        rel = path.relative_to(export_root).as_posix()
                        payloads.append(Payload(path=path, kaggle_name=rel))

    return payloads


def load_resource_metadata(export_root: Path) -> dict[str, dict]:
    metadata_path = export_root / "dataset-metadata.json"
    if not metadata_path.exists():
        return {}
    metadata = json.loads(metadata_path.read_text(encoding="utf-8"))
    resources = metadata.get("resources") or []
    return {
        str(resource.get("path", "")).replace("\\", "/"): resource
        for resource in resources
        if resource.get("path")
    }


def normalize_kaggle_type(original_type: str) -> str:
    original = (original_type or "string").lower()
    if original in {"numeric", "number", "decimal", "integer", "year"}:
        return "numeric"
    if original == "boolean":
        return "boolean"
    if original == "datetime":
        return "datetime"
    return original


def build_columns(resource: dict) -> list[ApiDatasetColumn]:
    fields = ((resource.get("schema") or {}).get("fields")) or []
    columns: list[ApiDatasetColumn] = []
    for order, field in enumerate(fields):
        column = ApiDatasetColumn()
        column.order = order
        column.name = field["name"]
        description = field.get("description") or field.get("title") or ""
        column.description = description
        original_type = field.get("type") or "string"
        column.original_type = original_type
        column.type = normalize_kaggle_type(original_type)
        columns.append(column)
    return columns


def upload_file(
    client: KaggleClient,
    payload: Payload,
    resource_metadata: dict[str, dict],
) -> ApiDatasetNewFile:
    request = ApiStartBlobUploadRequest()
    request.type = ApiBlobType.DATASET
    request.name = payload.kaggle_name
    request.content_type = (
        mimetypes.guess_type(payload.kaggle_name)[0] or "application/octet-stream"
    )
    request.content_length = payload.path.stat().st_size
    request.last_modified_epoch_seconds = int(payload.path.stat().st_mtime)

    response = client.blobs.blob_api_client.start_blob_upload(request)
    with payload.path.open("rb") as file_handle:
        put = requests.put(response.create_url, data=file_handle, timeout=120)
    if put.status_code not in (200, 201):
        raise RuntimeError(
            f"Upload failed for {payload.kaggle_name}: HTTP {put.status_code}"
        )

    dataset_file = ApiDatasetNewFile()
    dataset_file.token = response.token
    resource = resource_metadata.get(payload.kaggle_name)
    if resource:
        dataset_file.description = resource.get("description") or ""
        dataset_file.columns = build_columns(resource)
    return dataset_file


def create_or_version_dataset(
    client: KaggleClient,
    *,
    owner: str,
    slug: str,
    title: str,
    subtitle: str,
    description: str,
    license_name: str,
    is_private: bool,
    version_notes: str,
    files: list[ApiDatasetNewFile],
):
    create = ApiCreateDatasetRequest()
    create.owner_slug = owner
    create.slug = slug
    create.title = title
    create.license_name = license_name
    create.is_private = is_private
    create.files = files
    create.subtitle = subtitle
    create.description = description

    try:
        response = client.datasets.dataset_api_client.create_dataset(create)
        message = f"{response.status} {response.error}".lower()
        if (
            "already" not in message
            and "in use" not in message
            and "exist" not in message
            and "path must be unique" not in message
        ):
            return "created", response
    except Exception as exc:
        message = str(exc).splitlines()[0].lower()
        if (
            "already" not in message
            and "exist" not in message
            and "409" not in message
            and "path must be unique" not in message
        ):
            raise

    body = ApiCreateDatasetVersionRequestBody()
    body.version_notes = version_notes
    body.delete_old_versions = False
    body.files = files
    body.subtitle = subtitle
    body.description = description

    version = ApiCreateDatasetVersionRequest()
    version.owner_slug = owner
    version.dataset_slug = slug
    version.body = body
    try:
        return "versioned", client.datasets.dataset_api_client.create_dataset_version(version)
    except HTTPError as exc:
        body_text = exc.response.text if exc.response is not None else ""
        raise RuntimeError(f"Dataset version request failed: {body_text}") from exc


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--owner", default="jessealicea")
    parser.add_argument("--slug", default="small-model-memory-lab")
    parser.add_argument("--title", default="Small Model Memory and Structural Primitive Lab")
    parser.add_argument(
        "--subtitle",
        default="Memory LoRA adapters and structural primitive tests",
    )
    parser.add_argument(
        "--description",
        default=(
            "Small Model Memory Lab publishes real LoRA runs, public PEFT adapter "
            "weights, synthetic-memory corpora, structural primitive datasets, "
            "primitive-cache tests, activation-steering runs, and theory notes. "
            "Latest visible result: structural primitive LoRA v0.2 on "
            "Qwen/Qwen3-1.7B-Base improves the smoke eval from 2/6 to 6/6. "
            "Adapters are included under loras/ for independent testing."
        ),
    )
    parser.add_argument("--license", default="CC0-1.0")
    parser.add_argument("--version-notes", default="Refresh small-model memory lab export.")
    parser.add_argument("--export-root", type=Path, default=DEFAULT_EXPORT_ROOT)
    parser.add_argument("--public", action="store_true")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument(
        "--zip-folders",
        action="store_true",
        help=(
            "Upload top-level folders as zip archives. Faster, but file-level "
            "metadata for files inside those folders cannot be attached."
        ),
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    export_root = args.export_root.resolve()
    if not export_root.exists():
        raise FileNotFoundError(f"Export root not found: {export_root}")

    with tempfile.TemporaryDirectory(prefix="kaggle-small-model-memory-") as tmp:
        payloads = build_payloads(export_root, Path(tmp), zip_folders=args.zip_folders)
        resource_metadata = load_resource_metadata(export_root)
        total_size = sum(payload.path.stat().st_size for payload in payloads)
        print(f"Payloads: {len(payloads)} files, {total_size} bytes")
        for payload in payloads:
            meta = " +metadata" if payload.kaggle_name in resource_metadata else ""
            print(
                f"- {payload.kaggle_name}: {payload.path.stat().st_size} bytes{meta}"
            )

        if args.dry_run:
            return 0

        os.environ["KAGGLE_API_TOKEN"] = load_token()
        with KaggleClient(env=KaggleEnv.PROD) as client:
            uploaded = []
            for payload in payloads:
                uploaded.append(upload_file(client, payload, resource_metadata))
                print(f"Uploaded: {payload.kaggle_name}")

            action, response = create_or_version_dataset(
                client,
                owner=args.owner,
                slug=args.slug,
                title=args.title,
                subtitle=args.subtitle,
                description=args.description,
                license_name=args.license,
                is_private=not args.public,
                version_notes=args.version_notes,
                files=uploaded,
            )

    print(f"Dataset {action}: {response.status} {response.ref} {response.url}")
    if response.error:
        print(f"Error: {response.error}")
    if response.invalid_tags:
        print(f"Invalid tags: {response.invalid_tags}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
