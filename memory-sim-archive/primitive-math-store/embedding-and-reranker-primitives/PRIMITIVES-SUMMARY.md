# Embedding & Reranker Primitives - Summary

## What's Included

This folder contains **primitive implementations** (interfaces + scaffolding) for state-of-the-art embedding and reranker models. The actual model weights need to be downloaded separately.

## Folder Structure

```
F:\embedding-and-reranker-primitives\
├── README.md                          # Overview and quickstart guide
├── DOWNLOAD.md                        # Model download instructions
├── PRIMITIVES-SUMMARY.md              # This file
├── download-models.py                 # Python download script
├── download-models.bat                # Windows batch download script
│
├── embedding-primitives/              # Rust trait + implementations
│   ├── trait.rs                       # Core EmbeddingEngine trait
│   ├── bge_m3.rs                      # BGE-M3 implementation
│   ├── mxbai_large.rs                 # MXBAI Embed Large v1
│   └── mod.rs                         # Module exports
│
├── reranker-primitives/               # Rust trait + implementations
│   ├── trait.rs                       # Core RerankerEngine trait
│   ├── bge_reranker_v2_m3.rs          # BGE Reranker v2-M3
│   ├── bge_reranker_lightweight.rs    # Gemma2 Lightweight
│   └── mod.rs                         # Module exports
│
├── models/                            # Model configurations
│   ├── bge-m3.json                    # BGE-M3 metadata
│   └── weights/                       # Downloaded model weights go here
│       ├── BAAI_bge-m3\
│       ├── mixedbread-ai_mxbai-embed-large-v1\
│       ├── BAAI_bge-reranker-v2-m3\
│       └── BAAI_bge-reranker-v2.5-gemma2-lightweight\
│
└── examples/                          # Usage examples (TBD)
```

## Models Supported

### Embedding Models

| Model | Dimensions | Languages | Max Length | Size | License |
|-------|-----------|-----------|------------|------|---------|
| **BGE-M3** | 1024 | 100+ | 8192 | 2.2 GB | MIT |
| **mxbai-embed-large-v1** | 1024 | EN | 512 | 1.3 GB | Apache 2.0 |

**Key Features:**
- **BGE-M3**: Only model supporting dense + sparse + ColBERT retrieval
- **mxbai-embed-large-v1**: Matryoshka embeddings, binary quantization, MTEB 64.68

### Reranker Models

| Model | Type | Languages | Max Length | Size | License |
|-------|------|-----------|------------|------|---------|
| **BGE-Reranker-v2-M3** | Cross-encoder | 100+ | 8192 | 1.2 GB | MIT |
| **BGE-Reranker-v2.5-Gemma2-Lightweight** | Cross-encoder | Multilingual | 8192 | 18 GB | MIT |

**Key Features:**
- **v2-M3**: Lightweight, fast inference, multilingual
- **v2.5-Gemma2**: Token compression, layerwise selection, resource efficient

## Next Steps

### 1. Download Model Weights

Choose one method:

```bash
# Method A: Python script (requires huggingface_hub)
pip install huggingface_hub
python download-models.py

# Method B: Windows batch script (uses curl)
download-models.bat

# Method C: Manual download
# See DOWNLOAD.md for direct links
```

### 2. Install Dependencies (for Python usage)

```bash
pip install huggingface_hub transformers sentence-transformers FlagEmbedding onnxruntime
```

### 3. Use the Primitives

#### Python Example
```python
from FlagEmbedding import FlagAutoModel, FlagReranker

# Load embedding model
embedder = FlagAutoModel.from_finetuned(
    'F:\\embedding-and-reranker-primitives\\models\\weights\\BAAI_bge-m3',
    use_fp16=True
)

# Generate embeddings
query_emb = embedder.encode("Represent this sentence for searching relevant passages: What is AI?")
doc_emb = embedder.encode("Artificial intelligence is...")

# Load reranker
reranker = FlagReranker(
    'F:\\embedding-and-reranker-primitives\\models\\weights\\BAAI_bge-reranker-v2-m3',
    use_fp16=True
)

# Rerank documents
pairs = [
    ["What is AI?", "AI stands for Artificial Intelligence"],
    ["What is AI?", "The weather is nice today"]
]
scores = reranker.compute_score(pairs)
print(scores)  # [0.89, 0.12]
```

#### Rust Example (Ordo Integration)
```rust
use embedding_primitives::{BgeM3Engine, EmbeddingEngine, EmbeddingConfig};
use reranker_primitives::{BgeRerankerV2M3, RerankerEngine, RerankerConfig};

// Create embedding engine
let embedder = BgeM3Engine::new("BAAI/bge-m3")?;
let config = EmbeddingConfig::default();

// Generate embeddings
let query = "What is machine learning?";
let embedding = embedder.encode_with_type(
    query,
    EmbeddingInputType::Query,
    &config
)?;

// Create reranker
let reranker = BgeRerankerV2M3::default_model()?;
let rerank_config = RerankerConfig::default();

// Rank documents
let docs = vec![
    "Machine learning is a subset of AI...",
    "The sky is blue today"
];
let ranked = reranker.rank("What is ML?", &docs, &rerank_config)?;

for result in ranked {
    println!("Score: {:.4} - {}", result.score, result.document);
}
```

## Architecture

### Trait Design

Both embedding and reranker primitives follow a consistent trait-based design:

```
EmbeddingEngine (trait)
├── encode_one()           # Single text → embedding
├── encode_batch()         # Multiple texts → embeddings
├── encode_with_type()     # Query vs Document differentiation
├── encode_sparse()        # Lexical retrieval (BGE-M3 only)
└── encode_multi_vector()  # ColBERT-style (BGE-M3 only)

RerankerEngine (trait)
├── score_pair()           # Query + Doc → relevance score
├── score_batch()          # Multiple pairs → scores
├── rank()                 # Query + Docs → ranked results
└── rerank_with_metadata() # Rerank with associated metadata
```

### Implementation Status

| Component | Status | Notes |
|-----------|--------|-------|
| **Traits** | ✅ Complete | Full API surface defined |
| **BGE-M3** | 🟡 Scaffold | Ready for ONNX/Python bridge |
| **mxbai-large** | 🟡 Scaffold | Ready for ONNX/Python bridge |
| **BGE-Reranker-v2-M3** | 🟡 Scaffold | Ready for ONNX/Python bridge |
| **Gemma2-Lightweight** | 🟡 Scaffold | Ready for ONNX/Python bridge |
| **Model weights** | ⏳ Pending | User must download |

🟡 **Scaffold** = Trait implemented, placeholder inference. Needs actual model loading code.

## Integration with Ordo

These primitives are designed for Ordo's `ordo-inference` crate system:

1. Add to `Cargo.toml`:
```toml
[dependencies]
embedding-primitives = { path = "F:/embedding-and-reranker-primitives" }
reranker-primitives = { path = "F:/embedding-and-reranker-primitives" }
```

2. Wire into Ordo's bus:
```rust
// In ordo-runtime component registration
register_embedding_service(BgeM3Engine::default_model()?);
register_reranker_service(BgeRerankerV2M3::default_model()?);
```

3. Use in RAG pipelines:
```
Retrieve (dense) → Rerank (cross-encoder) → Generate (LLM)
```

## Performance Benchmarks

### MTEB Leaderboard (as of 2024)

| Model | Avg (56 datasets) | Retrieval | Reranking |
|-------|------------------|-----------|-----------|
| mxbai-embed-large-v1 | **64.68** | 54.39 | 60.11 |
| bge-large-en-v1.5 | 64.23 | 54.29 | 60.03 |
| OpenAI text-embedding-3-large | 64.58 | 55.44 | 59.16 |

### Inference Speed (approximate, batch=32, FP16)

| Model | CPU (ops/sec) | GPU RTX 4090 (ops/sec) |
|-------|--------------|------------------------|
| BGE-M3 | ~50 | ~800 |
| mxbai-large | ~60 | ~900 |
| BGE-Reranker-v2-M3 | ~100 | ~1200 |
| Gemma2-Lightweight | ~20 | ~400 |

## Resources

- **FlagEmbedding GitHub:** https://github.com/FlagOpen/FlagEmbedding
- **BGE Documentation:** https://www.bge-model.com
- **MTEB Leaderboard:** https://huggingface.co/spaces/mteb/leaderboard
- **Mixedbread AI Blog:** https://mixedbread.ai/blog/mxbai-embed-large-v1
- **Hugging Face Collection:** https://huggingface.co/collections/BAAI/bge-66797a74476eb1f085c7446d

## License Summary

- **BGE Models:** MIT License (commercial use OK)
- **mxbai-embed-large-v1:** Apache 2.0 (commercial use OK)
- **All primitives in this folder:** Inherit from upstream model licenses

## Contributing

To add new primitives:

1. Add model config to `models/<model-id>.json`
2. Implement trait in `embedding-primitives/` or `reranker-primitives/`
3. Update `mod.rs` exports
4. Add to this SUMMARY.md
5. Update DOWNLOAD.md with download instructions
6. Add example to `examples/`

---

**Last Updated:** 2026-06-21  
**Status:** Primitives ready, awaiting model weight downloads
