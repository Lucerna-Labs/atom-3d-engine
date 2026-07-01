#!/usr/bin/env python3
"""
Download Embedding and Reranker Model Primitives

This script downloads model weights from Hugging Face for:
- BGE-M3 (embedding)
- mxbai-embed-large-v1 (embedding)
- BGE-Reranker-v2-M3 (reranker)
- BGE-Reranker-v2.5-Gemma2-Lightweight (reranker)

Usage:
    python download-models.py [--output-dir OUTPUT_DIR] [--model MODEL_ID]

Examples:
    python download-models.py --output-dir "F:\embedding-and-reranker-primitives\models\weights"
    python download-models.py --model BAAI/bge-m3
"""

import os
import sys
import argparse
from pathlib import Path

# Try to import huggingface_hub, provide fallback instructions
try:
    from huggingface_hub import snapshot_download, hf_hub_download
    HAS_HF_HUB = True
except ImportError:
    HAS_HF_HUB = False
    print("⚠️  huggingface_hub not installed.")
    print("   Install with: pip install huggingface_hub")
    print("   Or download manually from the URLs below.\n")

# Model configurations
MODELS = {
    "BAAI/bge-m3": {
        "type": "embedding",
        "description": "Multi-functionality embedding (dense+sparse+ColBERT)",
        "files": [
            "config.json",
            "tokenizer.json", 
            "tokenizer_config.json",
            "special_tokens_map.json",
            "vocab.txt",
            "model.safetensors",
        ],
        "size_estimate": "~2.2 GB",
        "alternative_onnx": "yuniko-software/bge-m3-onnx"
    },
    "mixedbread-ai/mxbai-embed-large-v1": {
        "type": "embedding",
        "description": "SOTA MTEB 64.68, Matryoshka support",
        "files": [
            "config.json",
            "tokenizer.json",
            "tokenizer_config.json", 
            "special_tokens_map.json",
            "vocab.txt",
            "model.safetensors",
        ],
        "size_estimate": "~1.3 GB",
    },
    "BAAI/bge-reranker-v2-m3": {
        "type": "reranker",
        "description": "Lightweight multilingual cross-encoder",
        "files": [
            "config.json",
            "tokenizer.json",
            "tokenizer_config.json",
            "special_tokens_map.json",
            "vocab.txt",
            "model.safetensors",
        ],
        "size_estimate": "~1.2 GB",
    },
    "BAAI/bge-reranker-v2.5-gemma2-lightweight": {
        "type": "reranker",
        "description": "Gemma2-based with token compression",
        "files": [
            "config.json",
            "tokenizer.json",
            "tokenizer_config.json",
            "special_tokens_map.json",
            "model.safetensors.index.json",
        ],
        "size_estimate": "~18 GB (sharded)",
        "note": "Large model - requires significant disk space"
    }
}


def download_model(model_id: str, output_dir: Path, allow_patterns: list = None):
    """Download a model from Hugging Face."""
    print(f"\n{'='*60}")
    print(f"Downloading: {model_id}")
    print(f"{'='*60}")
    
    if model_id not in MODELS:
        print(f"⚠️  Unknown model: {model_id}")
        print("Available models:")
        for mid in MODELS.keys():
            print(f"  - {mid}")
        return False
    
    model_info = MODELS[model_id]
    print(f"Type: {model_info['type']}")
    print(f"Description: {model_info['description']}")
    print(f"Size: {model_info['size_estimate']}")
    if "note" in model_info:
        print(f"Note: {model_info['note']}")
    
    # Create model-specific directory
    model_dir = output_dir / model_id.replace("/", "_")
    model_dir.mkdir(parents=True, exist_ok=True)
    
    if not HAS_HF_HUB:
        print("\n❌ huggingface_hub not available.")
        print(f"\nManual download instructions:")
        print(f"1. Visit: https://huggingface.co/{model_id}")
        print(f"2. Download files: {', '.join(model_info['files'])}")
        print(f"3. Place in: {model_dir}")
        
        if "alternative_onnx" in model_info:
            print(f"\nAlternative ONNX version: https://huggingface.co/{model_info['alternative_onnx']}")
        
        return False
    
    try:
        # Download the model
        print(f"\n📥 Downloading to: {model_dir}")
        
        downloaded_path = snapshot_download(
            repo_id=model_id,
            local_dir=str(model_dir),
            allow_patterns=allow_patterns or model_info["files"],
            ignore_patterns=["*.msgpack", "*.h5"],  # Skip non-essential formats
        )
        
        print(f"✅ Successfully downloaded to: {downloaded_path}")
        return True
        
    except Exception as e:
        print(f"\n❌ Error downloading {model_id}: {e}")
        print(f"\nManual download: https://huggingface.co/{model_id}/tree/main")
        return False


def download_all_models(output_dir: Path):
    """Download all configured models."""
    print(f"\n{'='*60}")
    print(f"Embedding & Reranker Primitives Download")
    print(f"Output directory: {output_dir}")
    print(f"{'='*60}")
    
    results = {}
    for model_id in MODELS.keys():
        success = download_model(model_id, output_dir)
        results[model_id] = success
    
    # Summary
    print(f"\n{'='*60}")
    print("Download Summary")
    print(f"{'='*60}")
    
    for model_id, success in results.items():
        status = "✅" if success else "❌"
        print(f"{status} {model_id}: {'Success' if success else 'Failed/Skipped'}")
    
    successful = sum(1 for s in results.values() if s)
    total = len(results)
    print(f"\nTotal: {successful}/{total} models downloaded")
    
    return results


def main():
    parser = argparse.ArgumentParser(
        description="Download embedding and reranker model primitives from Hugging Face"
    )
    parser.add_argument(
        "--output-dir", "-o",
        type=str,
        default="F:\\embedding-and-reranker-primitives\\models\\weights",
        help="Output directory for model weights"
    )
    parser.add_argument(
        "--model", "-m",
        type=str,
        action="append",
        help="Specific model(s) to download (can be used multiple times)"
    )
    parser.add_argument(
        "--list", "-l",
        action="store_true",
        help="List available models and exit"
    )
    
    args = parser.parse_args()
    
    output_dir = Path(args.output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)
    
    if args.list:
        print("\nAvailable Models:")
        print("="*60)
        for model_id, info in MODELS.items():
            print(f"\n{model_id}")
            print(f"  Type: {info['type']}")
            print(f"  Description: {info['description']}")
            print(f"  Size: {info['size_estimate']}")
            if "note" in info:
                print(f"  Note: {info['note']}")
        return 0
    
    if args.model:
        # Download specific models
        results = {}
        for model_id in args.model:
            success = download_model(model_id, output_dir)
            results[model_id] = success
        
        successful = sum(1 for s in results.values() if s)
        return 0 if successful == len(results) else 1
    else:
        # Download all models
        results = download_all_models(output_dir)
        successful = sum(1 for s in results.values() if s)
        return 0 if successful == len(results) else 1


if __name__ == "__main__":
    sys.exit(main())
