# Model Weights Download Guide

This document provides instructions for downloading the actual model weights for the embedding and reranker primitives.

## Quick Download (Recommended)

### Option 1: Using huggingface-cli (Python)

```bash
# Install huggingface_hub
pip install huggingface_hub

# Download all models
python download-models.py --output-dir "F:\embedding-and-reranker-primitives\models\weights"

# Or download specific models
python download-models.py -m BAAI/bge-m3
python download-models.py -m mixedbread-ai/mxbai-embed-large-v1
python download-models.py -m BAAI/bge-reranker-v2-m3
```

### Option 2: Manual Download from Hugging Face

Visit these URLs and click "Download" on each file:

#### BGE-M3 (Embedding)
- **URL:** https://huggingface.co/BAAI/bge-m3
- **Size:** ~2.2 GB
- **Files needed:**
  - `config.json`
  - `tokenizer.json`
  - `tokenizer_config.json`
  - `special_tokens_map.json`
  - `vocab.txt`
  - `model.safetensors`
- **Save to:** `F:\embedding-and-reranker-primitives\models\weights\BAAI_bge-m3\`

#### mxbai-embed-large-v1 (Embedding)
- **URL:** https://huggingface.co/mixedbread-ai/mxbai-embed-large-v1
- **Size:** ~1.3 GB
- **Files needed:** Same as above
- **Save to:** `F:\embedding-and-reranker-primitives\models\weights\mixedbread-ai_mxbai-embed-large-v1\`

#### BGE-Reranker-v2-M3 (Reranker)
- **URL:** https://huggingface.co/BAAI/bge-reranker-v2-m3
- **Size:** ~1.2 GB
- **Files needed:** Same as above
- **Save to:** `F:\embedding-and-reranker-primitives\models\weights\BAAI_bge-reranker-v2-m3\`

#### BGE-Reranker-v2.5-Gemma2-Lightweight (Reranker) ⚠️ LARGE
- **URL:** https://huggingface.co/BAAI/bge-reranker-v2.5-gemma2-lightweight
- **Size:** ~18 GB (sharded)
- **Files needed:** Same as above + `model.safetensors.index.json`
- **Save to:** `F:\embedding-and-reranker-primitives\models\weights\BAAI_bge-reranker-v2.5-gemma2-lightweight\`

### Option 3: Git LFS Clone

```bash
# Install Git LFS first
git lfs install

# Clone individual models
git lfs clone https://huggingface.co/BAAI/bge-m3 "F:\embedding-and-reranker-primitives\models\weights\BAAI_bge-m3"
git lfs clone https://huggingface.co/mixedbread-ai/mxbai-embed-large-v1 "F:\embedding-and-reranker-primitives\models\weights\mixedbread-ai_mxbai-embed-large-v1"
```

## ONNX Versions (Smaller, Faster Inference)

For production use, ONNX versions are recommended:

### BGE-M3 ONNX
- **URL:** https://huggingface.co/yuniko-software/bge-m3-onnx
- **Size:** ~1.1 GB
- **Advantages:** Faster inference, no Python dependencies

### mxbai-embed-large-v1 ONNX
- **URL:** https://huggingface.co/onnx-community/bge-m3-ONNX (similar structure)
- Use transformers.js version or convert manually

## Directory Structure After Download

```
F:\embedding-and-reranker-primitives\models\weights\
├── BAAI_bge-m3\
│   ├── config.json
│   ├── tokenizer.json
│   ├── tokenizer_config.json
│   ├── special_tokens_map.json
│   ├── vocab.txt
│   └── model.safetensors
├── mixedbread-ai_mxbai-embed-large-v1\
│   └── [same files]
├── BAAI_bge-reranker-v2-m3\
│   └── [same files]
└── BAAI_bge-reranker-v2.5-gemma2-lightweight\
    └── [same files + shards]
```

## Verification

After downloading, verify the models:

```bash
python download-models.py --list
```

Or check manually:
```powershell
Get-ChildItem "F:\embedding-and-reranker-primitives\models\weights" -Recurse -File | 
    Measure-Object -Property Length -Sum | 
    Select-Object @{Name="Size";Expression={[math]::Round($_.Sum / 1GB, 2)}}
```

Expected total size: ~22-25 GB for all 4 models.

## Usage After Download

Once downloaded, the models can be used with:

### Python (FlagEmbedding)
```python
from FlagEmbedding import FlagAutoModel, FlagReranker

# Embedding
embedder = FlagAutoModel.from_finetuned(
    'F:\\embedding-and-reranker-primitives\\models\\weights\\BAAI_bge-m3',
    use_fp16=True
)

# Reranker
reranker = FlagReranker(
    'F:\\embedding-and-reranker-primitives\\models\\weights\\BAAI_bge-reranker-v2-m3',
    use_fp16=True
)
```

### Rust (ORT/ONNX Runtime)
See `examples/` directory for Rust integration examples using ONNX models.

## Troubleshooting

### Download fails with authentication error
- Create a Hugging Face account at https://huggingface.co/join
- Generate a token at https://huggingface.co/settings/tokens
- Login: `huggingface-cli login`
- Or set env var: `HF_TOKEN=your_token_here`

### Out of disk space
- Start with just BGE-M3 (2.2 GB) - most versatile
- Skip Gemma2-Lightweight (18 GB) unless needed
- Use ONNX versions (smaller)

### Slow download speed
- Hugging Face has CDN mirrors
- Try: `HF_ENDPOINT=https://hf-mirror.com` (China)
- Or use Git LFS clone (resumable)

## Model Cards & Documentation

- **BGE Models:** https://www.bge-model.com
- **FlagEmbedding GitHub:** https://github.com/FlagOpen/FlagEmbedding
- **MXBAI Blog:** https://mixedbread.ai/blog/mxbai-embed-large-v1
- **MTEB Leaderboard:** https://huggingface.co/spaces/mteb/leaderboard
