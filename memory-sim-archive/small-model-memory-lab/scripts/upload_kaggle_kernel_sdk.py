#!/usr/bin/env python
"""Push a Kaggle kernel using the bearer-token SDK path."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path

from kagglesdk import KaggleClient, KaggleEnv
from kagglesdk.kernels.types.kernels_api_service import ApiSaveKernelRequest


DEFAULT_KERNEL_DIR = (
    Path(__file__).resolve().parents[1] / "kaggle_notebooks" / "starter"
)


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


def as_bool(value, default: bool = False) -> bool:
    if value is None:
        return default
    if isinstance(value, bool):
        return value
    return str(value).strip().lower() in {"1", "true", "yes", "y"}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--kernel-dir", type=Path, default=DEFAULT_KERNEL_DIR)
    parser.add_argument("--timeout", type=int, default=None)
    parser.add_argument("--dry-run", action="store_true")
    return parser.parse_args()


def build_request(kernel_dir: Path, timeout: int | None) -> ApiSaveKernelRequest:
    metadata_path = kernel_dir / "kernel-metadata.json"
    metadata = json.loads(metadata_path.read_text(encoding="utf-8"))

    code_path = kernel_dir / metadata["code_file"]
    code_text = code_path.read_text(encoding="utf-8")

    request = ApiSaveKernelRequest()
    request.slug = metadata["id"]
    request.new_title = metadata["title"]
    request.text = code_text
    request.language = metadata["language"]
    request.kernel_type = metadata["kernel_type"]
    request.dataset_data_sources = metadata.get("dataset_sources", [])
    request.kernel_data_sources = metadata.get("kernel_sources", [])
    request.competition_data_sources = metadata.get("competition_sources", [])
    request.model_data_sources = metadata.get("model_sources", [])
    request.category_ids = metadata.get("keywords", [])
    request.is_private = as_bool(metadata.get("is_private"), default=True)
    request.enable_gpu = as_bool(metadata.get("enable_gpu"), default=False)
    request.enable_tpu = as_bool(metadata.get("enable_tpu"), default=False)
    request.enable_internet = as_bool(metadata.get("enable_internet"), default=True)
    if timeout:
        request.session_timeout_seconds = timeout
    return request


def main() -> int:
    args = parse_args()
    kernel_dir = args.kernel_dir.resolve()
    request = build_request(kernel_dir, args.timeout)

    print(f"Kernel: {request.slug}")
    print(f"Title: {request.new_title}")
    print(f"Dataset sources: {request.dataset_data_sources}")
    print(f"Private: {request.is_private}")

    if args.dry_run:
        return 0

    os.environ["KAGGLE_API_TOKEN"] = load_token()
    with KaggleClient(env=KaggleEnv.PROD) as client:
        response = client.kernels.kernels_api_client.save_kernel(request)

    print(f"Kernel pushed: {response.ref} {response.url}")
    if response.version_number:
        print(f"Version: {response.version_number}")
    if response.error:
        print(f"Error: {response.error}")
    if response.invalid_dataset_sources:
        print(f"Invalid dataset sources: {response.invalid_dataset_sources}")
    if response.invalid_model_sources:
        print(f"Invalid model sources: {response.invalid_model_sources}")
    return 0 if not response.error else 1


if __name__ == "__main__":
    raise SystemExit(main())
