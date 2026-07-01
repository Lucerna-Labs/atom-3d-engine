# Quick Start Guide

Get up and running with embedding and reranker primitives in 5 minutes.

## Step 1: Download Model Weights (Choose One)

### Option A: Python Script (Recommended)
```bash
pip install huggingface_hub
cd F:\embedding-and-reranker-primitives
python download-models.py
```

### Option B: Windows Batch Script
```cmd
cd F:\embedding-and-reranker-primitives
download-models.bat
```

### Option C: Download Just BGE-M3 (Fastest)
```bash
# Minimal setup - just the most versatile model
python download-models.py -m BAAI/bge-m3
```

**Total download size:** ~22 GB for all models, ~2.2 GB for BGE-M3 only

## Step 2: Install Python Dependencies

```bash
pip install huggingface_hub transformers sentence-transformers FlagEmbedding onnxruntime
```

## Step 3: Test It Works

### Python Test
```python
from FlagEmbedding import FlagAutoModel, FlagReranker

# Test embedding
print("Testing BGE-M3 embedding...")
model = FlagAutoModel.from_finetuned(
    r'F:\embedding-and-reranker-primitives\models\weights\BAAI_bge-m3',
    use_fp16=True
)
emb = model.encode(["Hello world", "Test embedding"])
print(f"✓ Embedding shape: {emb.shape}")

# Test reranker
print("\nTesting BGE-Reranker...")
reranker = FlagReranker(
    r'F:\embedding-and-reranker-primitives\models\weights\BAAI_bge-reranker-v2-m3',
    use_fp16=True
)
score = reranker.compute_score(['What is AI?', 'Artificial intelligence is...'])
print(f"✓ Relevance score: {score:.4f}")

print("\n✅ All tests passed!")
```

### Rust Test (if integrating with Ordo)
```bash
cd F:\Ordo-portable-personal-20260607-012453
cargo test --package embedding-primitives
```

## Step 4: Use in Your Project

### Simple RAG Pipeline Example

```python
from FlagEmbedding import FlagAutoModel, FlagReranker

# Initialize models
embedder = FlagAutoModel.from_finetuned(
    r'F:\embedding-and-reranker-primitives\models\weights\BAAI_bge-m3'
)
reranker = FlagReranker(
    r'F:\embedding-and-reranker-primitives\models\weights\BAAI_bge-reranker-v2-m3'
)

# Your documents
documents = [
    "Machine learning is a subset of artificial intelligence.",
    "The weather today is sunny with a high of 75°F.",
    "Python is a popular programming language for data science.",
    "Deep learning uses neural networks with many layers."
]

# User query
query = "What is machine learning?"

# Step 1: Generate embeddings
query_emb = embedder.encode(f"Represent this sentence for searching relevant passages: {query}")
doc_embs = embedder.encode(documents)

# Step 2: Find top-k by similarity
import numpy as np
similarities = query_emb @ doc_embs.T
top_k_indices = np.argsort(similarities)[::-1][:2]  # Top 2

# Step 3: Rerank for better accuracy
top_k_docs = [documents[i] for i in top_k_indices]
pairs = [[query, doc] for doc in top_k_docs]
rerank_scores = reranker.compute_score(pairs)

# Step 4: Show results
print("Reranked Results:")
for i, (doc, score) in enumerate(zip(top_k_docs, rerank_scores)):
    print(f"{i+1}. Score: {score:.4f}")
    print(f"   {doc}\n")
```

**Expected Output:**
```
Reranked Results:
1. Score: 0.8923
   Machine learning is a subset of artificial intelligence.

2. Score: 0.3421
   Deep learning uses neural networks with many layers.
```

## Common Tasks

### Generate Embeddings for Many Documents

```python
import numpy as np
from FlagEmbedding import FlagAutoModel

model = FlagAutoModel.from_finetuned(
    r'F:\embedding-and-reranker-primitives\models\weights\BAAI_bge-m3',
    use_fp16=True
)

# Batch encode (efficient)
documents = ["Doc 1", "Doc 2", ..., "Doc 1000"]
embeddings = model.encode(documents, batch_size=64)

# Save to numpy
np.save('document_embeddings.npy', embeddings)

# Later: load and use
loaded = np.load('document_embeddings.npy')
```

### Matryoshka Embeddings (MXBAI Only)

```python
from sentence_transformers import SentenceTransformer

# Load with dimension truncation
model = SentenceTransformer(
    'mixedbread-ai/mxbai-embed-large-v1',
    truncate_dim=256  # Reduce from 1024 to 256
)

emb = model.encode("Hello world")
print(f"Embedding dim: {len(emb)}")  # 256 instead of 1024
```

### Binary Quantization (Save 32x Memory)

```python
from sentence_transformers import SentenceTransformer
from sentence_transformers.quantization import quantize_embeddings

model = SentenceTransformer('mixedbread-ai/mxbai-embed-large-v1')
emb = model.encode("Hello world")

# Quantize to binary
binary_emb = quantize_embeddings(emb, precision='ubinary')
print(f"Original size: {emb.nbytes} bytes")
print(f"Binary size: {binary_emb.nbytes} bytes")  # 32x smaller!
```

### Sparse Embeddings (BGE-M3 Only)

```python
from FlagEmbedding import FlagAutoModel

model = FlagAutoModel.from_finetuned('BAAI/bge-m3')

# Get sparse embedding (lexical retrieval)
sparse = model.encode("Hello world", return_sparse=True)
# Returns dict: {token_id: weight, ...}
```

## Troubleshooting

### "Model not found" error
Make sure weights are downloaded:
```powershell
Test-Path "F:\embedding-and-reranker-primitives\models\weights\BAAI_bge-m3\model.safetensors"
```

### Out of memory
Use FP16 and smaller batch sizes:
```python
model = FlagAutoModel.from_finetuned('...', use_fp16=True)
embeddings = model.encode(docs, batch_size=16)  # Reduce from default 32
```

### Slow inference
- Use GPU if available: `device='cuda'`
- Try ONNX version for CPU: `yuniko-software/bge-m3-onnx`
- Enable FP16: `use_fp16=True`

## Next Steps

1. **Read the full documentation:**
   - `README.md` - Complete overview
   - `PRIMITIVES-SUMMARY.md` - Architecture details
   - `DOWNLOAD.md` - Download troubleshooting

2. **Integrate with Ordo:**
   - See `embedding-primitives/trait.rs` for Rust API
   - Wire into `ordo-inference` crate system

3. **Explore advanced features:**
   - Token compression (Gemma2 reranker)
   - Layerwise selection (early exit)
   - Multi-vector ColBERT retrieval (BGE-M3)

---

**Need help?** Check `DOWNLOAD.md` for detailed download instructions or visit:
- https://github.com/FlagOpen/FlagEmbedding
- https://www.bge-model.com
