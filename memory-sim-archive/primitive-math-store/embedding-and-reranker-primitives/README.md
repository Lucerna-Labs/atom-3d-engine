# Embedding & Reranker Primitives

A collection of primitive implementations for text embedding and reranking models, designed for integration into the Ordo runtime and other Rust-based AI systems.

## Overview

This folder contains:
- **Embedding primitives**: Dense vector representations for semantic search
- **Reranker primitives**: Cross-encoder models for re-ranking retrieved documents

## Quick Reference

### Top Embedding Models

| Model | Dimensions | Languages | Max Length | License |
|-------|-----------|-----------|------------|---------|
| [BGE-M3](https://huggingface.co/BAAI/bge-m3) | 1024 | 100+ | 8192 | MIT |
| [mxbai-embed-large-v1](https://huggingface.co/mixedbread-ai/mxbai-embed-large-v1) | 1024 | English | 512 | Apache 2.0 |
| [bge-multilingual-gemma2](https://huggingface.co/BAAI/bge-multilingual-gemma2) | 3584 | Multilingual | 8192 | MIT |

### Top Reranker Models

| Model | Type | Languages | Max Length | License |
|-------|------|-----------|------------|---------|
| [bge-reranker-v2-m3](https://huggingface.co/BAAI/bge-reranker-v2-m3) | Cross-encoder | 100+ | 8192 | MIT |
| [bge-reranker-v2.5-gemma2-lightweight](https://huggingface.co/BAAI/bge-reranker-v2.5-gemma2-lightweight) | Cross-encoder | Multilingual | 8192 | MIT |
| [bge-reranker-large](https://huggingface.co/BAAI/bge-reranker-large) | Cross-encoder | EN/ZH | 512 | MIT |

## Architecture

```
embedding-and-reranker-primitives/
├── README.md                    # This file
├── embedding-primitives/        # Embedding model primitives
│   ├── trait.rs                 # Core EmbeddingEngine trait
│   ├── bge_m3.rs                # BGE-M3 implementation
│   ├── mxbai_large.rs           # MXBAI Large implementation
│   └── mod.rs                   # Module exports
├── reranker-primitives/         # Reranker model primitives
│   ├── trait.rs                 # Core RerankerEngine trait
│   ├── bge_reranker_v2_m3.rs    # BGE Reranker v2-M3
│   ├── bge_reranker_lightweight.rs  # Lightweight Gemma2 version
│   └── mod.rs                   # Module exports
├── models/                      # Model configs and metadata
│   ├── bge-m3.json
│   ├── mxbai-embed-large-v1.json
│   ├── bge-reranker-v2-m3.json
│   └── bge-reranker-v2.5-gemma2-lightweight.json
└── examples/                    # Usage examples
    ├── embedding_example.rs
    └── reranker_example.rs
```

## Installation (Python Reference)

```bash
# FlagEmbedding (BGE models)
pip install -U FlagEmbedding

# Sentence Transformers (mxbai-embed)
pip install -U sentence-transformers

# For fine-tuning support
pip install -U FlagEmbedding[finetune]
```

## Python Quickstart

### Embedding (BGE-M3)

```python
from FlagEmbedding import FlagAutoModel

model = FlagAutoModel.from_finetuned('BAAI/bge-m3', use_fp16=True)

sentences = ["I love NLP", "I love machine learning"]
embeddings = model.encode(sentences)

# Compute similarity
similarity = embeddings[0] @ embeddings[1].T
```

### Embedding (mxbai-embed-large-v1)

```python
from sentence_transformers import SentenceTransformer

model = SentenceTransformer("mixedbread-ai/mxbai-embed-large-v1", truncate_dim=512)

query = "Represent this sentence for searching relevant passages: A man is eating bread"
docs = ["A man is eating food.", "A man is riding a horse."]

query_emb = model.encode(query, prompt_name="query")
docs_emb = model.encode(docs)
```

### Reranking (BGE-Reranker-v2-M3)

```python
from FlagEmbedding import FlagReranker

reranker = FlagReranker('BAAI/bge-reranker-v2-m3', use_fp16=True)

pairs = [
    ['What is the capital of France?', 'Paris is the capital of France.'],
    ['What is the capital of France?', 'London is the capital of England.']
]

scores = reranker.compute_score(pairs)
print(scores)  # [0.89, 0.12]
```

## Rust Integration (Ordo)

These primitives are designed to integrate with Ordo's `ordo-inference` crate system:

```rust
use ordo_inference::EmbeddingEngine;
use embedding_primitives::BgeM3Engine;

let engine = BgeM3Engine::new("BAAI/bge-m3")?;
let embeddings = engine.encode(&["hello world", "rust is great"])?;
```

## Key Features

### BGE-M3
- **Multi-Functionality**: Dense + Sparse + Multi-vector (ColBERT) retrieval
- **Multi-Linguality**: 100+ languages
- **Multi-Granularity**: Up to 8192 tokens
- **First embedding model** supporting all three retrieval methods

### mxbai-embed-large-v1
- **MTEB Score**: 64.68 (SOTA for BERT-large size)
- **Matryoshka embeddings**: Configurable dimensions (1024 → 512 → 256)
- **Binary quantization**: int8 and ubinary support
- **Outperforms** OpenAI text-embedding-3-large on MTEB

### BGE-Reranker-v2-M3
- **Lightweight cross-encoder**: Fast inference
- **Multilingual**: Strong performance across languages
- **Direct similarity output**: No embedding generation needed

### BGE-Reranker-v2.5-Gemma2-Lightweight
- **Token compression**: Reduces context length
- **Layerwise operations**: Selectable output layers
- **Resource efficient**: Significant savings with maintained performance

## Benchmarks

### MTEB Leaderboard (Top Models)

| Model | Avg (56 datasets) | Retrieval | Reranking |
|-------|------------------|-----------|-----------|
| mxbai-embed-large-v1 | 64.68 | 54.39 | 60.11 |
| bge-large-en-v1.5 | 64.23 | 54.29 | 60.03 |
| OpenAI text-embedding-3-large | 64.58 | 55.44 | 59.16 |

## Resources

- [FlagEmbedding GitHub](https://github.com/FlagOpen/FlagEmbedding)
- [BGE Documentation](https://www.bge-model.com)
- [MTEB Leaderboard](https://huggingface.co/spaces/mteb/leaderboard)
- [Mixedbread AI Blog](https://mixedbread.ai/blog/mxbai-embed-large-v1)

## License

- BGE models: MIT License
- mxbai-embed-large-v1: Apache 2.0 License

## Contributing

When adding new primitives:
1. Add model config to `models/`
2. Implement trait in corresponding primitive file
3. Update this README with model specs
4. Add example usage to `examples/`
