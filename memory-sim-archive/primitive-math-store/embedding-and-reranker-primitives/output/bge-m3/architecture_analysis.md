# BGE-M3 Architecture Analysis

## Overview

**Model:** BAAI/bge-m3  
**Architecture:** XLMRobertaModel (XLM-RoBERTa Large based)  
**Type:** Bi-encoder with multi-functionality support  
**Parameters:** ~567M  

## Core Architecture Primitives

### Layer Stack (24 layers)

```
┌─────────────────────────────────────┐
│ 24 × Transformer Blocks             │
│ ┌─────────────────────────────────┐ │
│ │ Multi-Head Self-Attention       │ │
│ │ - 16 heads, 64 dim each         │ │
│ │ - Q/K/V projections: 1024→1024  │ │
│ │ - Output projection: 1024→1024  │ │
│ │ - LayerNorm + residual          │ │
│ ├─────────────────────────────────┤ │
│ │ Feed-Forward Network (MLP)      │ │
│ │ - Hidden: 1024 → 4096           │ │
│ │ - Activation: GeLU              │ │
│ │ - Output: 4096 → 1024           │ │
│ │ - LayerNorm + residual          │ │
│ └─────────────────────────────────┘ │
└─────────────────────────────────────┘
```

### Attention Mechanism

**Type:** Multi-head self-attention with absolute positional embeddings

**Configuration:**
- **Heads:** 16 attention heads
- **Head dimension:** 64 (1024 / 16)
- **QKV projection:** Separate weight matrices for Query, Key, Value
- **Output projection:** Linear layer (1024×1024)
- **Dropout:** 0.1 on attention probabilities
- **Position embeddings:** Absolute (learned, 8194 positions)

**Attention primitive:**
```
Attention(Q, K, V) = softmax(QK^T / √d_k)V
MultiHead(Q, K, V) = Concat(head_1, ..., head_16)W^O
where head_i = Attention(QW_i^Q, KW_i^K, VW_i^V)
```

### Feed-Forward Network

**Type:** Standard MLP with GeLU activation

**Structure:**
```
FFN(x) = W_2 · GeLU(W_1 · x + b_1) + b_2
where:
  W_1 ∈ ℝ^(1024×4096)  [intermediate_weight]
  W_2 ∈ ℝ^(4096×1024)  [output_weight]
  b_1 ∈ ℝ^4096
  b_2 ∈ ℝ^1024
```

**Expansion ratio:** 4× (1024 → 4096 → 1024)

### Normalization

**Type:** LayerNorm (pre-norm or post-norm depending on implementation)

**Configuration:**
- **Epsilon:** 1e-05
- **Elementwise affine:** true (learnable scale and shift)
- **Applied:** After each sub-layer (attention + FFN)

### Embedding Layer

**Word Embeddings:**
- **Vocabulary size:** 250,002 tokens
- **Embedding dimension:** 1024
- **Padding index:** 1
- **Weight shape:** [250002 × 1024] = 261M parameters

**Position Embeddings:**
- **Max positions:** 8,194
- **Dimension:** 1024
- **Type:** Absolute (learned)
- **Weight shape:** [8194 × 1024] = 8.4M parameters

**Token Type Embeddings:** None (type_vocab_size = 1)

## Multi-Functionality Architecture

BGE-M3 uniquely supports three retrieval methods from a single model:

### 1. Dense Retrieval (Primary)
- **Output:** Single 1024-d vector per text
- **Pooling:** CLS token, mean, or max
- **Use case:** Semantic similarity search

### 2. Sparse Retrieval (Lexical)
- **Output:** Sparse vector over vocabulary (250,002 dims)
- **Method:** Token-level weighting (like SPLADE)
- **Use case:** Keyword-aware retrieval

### 3. Multi-Vector / ColBERT
- **Output:** Token-level vectors [seq_len × 1024]
- **Method:** Late interaction (maxsim)
- **Use case:** Fine-grained matching

## Tensor Primitive Map

| Component | Shape | Parameters | Purpose |
|-----------|-------|------------|---------|
| `embeddings.word_embeddings.weight` | [250002, 1024] | 261,123,072 | Token → dense vector |
| `embeddings.position_embeddings.weight` | [8194, 1024] | 8,390,656 | Position encoding |
| `encoder.layer.*.attention.self.query.weight` | [1024, 1024] | 1,048,576 | Q projection |
| `encoder.layer.*.attention.self.key.weight` | [1024, 1024] | 1,048,576 | K projection |
| `encoder.layer.*.attention.self.value.weight` | [1024, 1024] | 1,048,576 | V projection |
| `encoder.layer.*.attention.output.dense.weight` | [1024, 1024] | 1,048,576 | Attention output |
| `encoder.layer.*.intermediate.dense.weight` | [1024, 4096] | 4,194,304 | FFN expansion |
| `encoder.layer.*.output.dense.weight` | [4096, 1024] | 4,194,304 | FFN projection |
| `encoder.layer.*.LayerNorm.weight` | [1024] | 1,024 | Normalization scale |
| `encoder.layer.*.LayerNorm.bias` | [1024] | 1,024 | Normalization shift |

**Total per layer:** ~12.7M parameters  
**24 layers:** ~305M parameters  
**Embeddings:** ~262M parameters  
**Grand total:** ~567M parameters

## Inference Graph

```
Input IDs [batch, seq_len]
    ↓
[Word Embeddings] → [Position Embeddings]
    ↓ (+)
[Hidden State: batch × seq_len × 1024]
    ↓
[Transformer Block 0]
    ↓
[Transformer Block 1]
    ↓
...
    ↓
[Transformer Block 23]
    ↓
[Pooler] → CLS / Mean / Max
    ↓
[Output Embedding: batch × 1024]
    ↓ (optional L2 normalize)
[Normalized Embedding: batch × 1024]
```

## Special Tokens

| Token | ID | Usage |
|-------|----|-------|
| BOS | 0 | Beginning of sequence |
| PAD | 1 | Padding |
| EOS | 2 | End of sequence |

## Computational Complexity

**Per-token operations (single layer):**
- QKV projection: 3 × 1024² = 3.1M FLOPs
- Attention: O(seq_len² × heads × head_dim)
- FFN: 2 × 1024 × 4096 = 8.4M FLOPs
- **Total per layer:** ~12M FLOPs/token
- **Total (24 layers):** ~288M FLOPs/token

**Memory bandwidth (FP32):**
- Model size: 567M × 4 bytes = 2.27 GB
- KV cache: seq_len × batch × 2 × 1024 × 4 bytes

## Precision Support

- **Training:** float32
- **Inference:** float32 (can use float16 for 2× speedup)
- **Quantization:** Not natively supported (requires ONNX conversion)

## Key Design Decisions

1. **XLM-RoBERTa base:** Chosen for multilingual capability (100+ languages)
2. **Large model size:** 24 layers, 1024 hidden for high capacity
3. **Absolute positions:** Simpler than RoPE, works well for retrieval
4. **GeLU activation:** Smooth non-linearity, better than ReLU for transformers
5. **LayerNorm epsilon:** 1e-05 (standard for BERT-style models)
6. **No token types:** Simplified for single-sequence encoding

## References

- Paper: https://arxiv.org/abs/2402.03216
- Hugging Face: https://huggingface.co/BAAI/bge-m3
- FlagEmbedding: https://github.com/FlagOpen/FlagEmbedding
