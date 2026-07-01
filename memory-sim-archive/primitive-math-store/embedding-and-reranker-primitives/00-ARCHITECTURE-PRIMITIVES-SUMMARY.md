# Architecture Primitives Summary

Extracted from embedding and reranker model configurations.

## Models Analyzed

| Model | Architecture | Layers | Hidden | Heads | Params | Max Length |
|-------|-------------|--------|--------|-------|--------|------------|
| **BGE-M3** | XLMRobertaModel | 24 | 1024 | 16 | 567M | 8192 |
| **mxbai-embed-large-v1** | BertModel | 24 | 1024 | 16 | 335M | 512 |
| **BGE-Reranker-v2-M3** | XLMRobertaForSequenceClassification | 24 | 1024 | 16 | ~567M | 8192 |

## Common Architecture Primitives

### Transformer Block (All Models)

```
TransformerBlock:
  Input: x ∈ ℝ^(batch × seq_len × hidden)
  
  # Multi-Head Self-Attention
  q = LayerNorm(x)
  Q = q · W_Q  [W_Q ∈ ℝ^(hidden × hidden)]
  K = q · W_K  [W_K ∈ ℝ^(hidden × hidden)]
  V = q · W_V  [W_V ∈ ℝ^(hidden × hidden)]
  
  Attention = softmax(QK^T / √head_dim)V
  MultiHead = Concat(head_1..head_h) · W_O
  
  x = x + Dropout(MultiHead)
  
  # Feed-Forward Network
  ff = LayerNorm(x)
  ff = ff · W_1  [W_1 ∈ ℝ^(hidden × intermediate)]
  ff = GeLU(ff)
  ff = ff · W_2  [W_2 ∈ ℝ^(intermediate × hidden)]
  
  Output = x + Dropout(ff)
```

### Standard Dimensions

**Large-scale embedding models (all three):**
- Hidden size: 1024
- Intermediate size: 4096 (4× expansion)
- Num heads: 16
- Head dim: 64 (1024 / 16)
- Num layers: 24

**This matches BERT Large architecture exactly.**

### Activation Function

**All models use:** GeLU (Gaussian Error Linear Unit)

```
GeLU(x) = x · Φ(x)
where Φ is the standard Gaussian CDF

Approximation (common):
GeLU(x) ≈ 0.5x(1 + tanh[√(2/π)(x + 0.044715x³)])
```

### Normalization

**Type:** LayerNorm (Ba et al., 2016)

```
LayerNorm(x) = γ · (x - μ) / √(σ² + ε) + β

where:
  μ = mean(x)
  σ² = variance(x)
  γ = weight (learnable)
  β = bias (learnable)
  ε = epsilon (stability constant)
```

**Epsilon values:**
- BGE-M3: 1e-05
- mxbai-large: 1e-12
- BGE-Reranker: 1e-05

### Position Embeddings

**All models:** Absolute position embeddings (learned)

```
PositionEmbeddings:
  Type: Lookup table
  Shape: [max_positions × hidden]
  BGE-M3: [8194 × 1024]
  mxbai-large: [512 × 1024]
```

**Not RoPE or ALiBi** - these are standard learned embeddings like original BERT.

## Key Differences

### Vocabulary Size

| Model | Vocab Size | Type | Languages |
|-------|-----------|------|-----------|
| BGE-M3 | 250,002 | SentencePiece (XLM-R) | 100+ |
| mxbai-large | 30,522 | WordPiece (BERT) | English |
| BGE-Reranker | 250,002 | SentencePiece (XLM-R) | 100+ |

### Maximum Sequence Length

| Model | Max Positions | Use Case |
|-------|--------------|----------|
| BGE-M3 | 8,194 | Long documents |
| mxbai-large | 512 | Standard sentences/paragraphs |
| BGE-Reranker | 8,194 | Long query-document pairs |

### Special Capabilities

#### BGE-M3 Unique Primitives

**Multi-functionality heads:**
1. **Dense head:** CLS pooling → 1024-d vector
2. **Sparse head:** Token weights → 250,002-d sparse vector
3. **ColBERT head:** Token vectors → [seq_len × 1024] matrix

#### mxbai-large Unique Primitives

**Matryoshka embeddings:**
- Native dimension: 1024
- Truncatable to: 768, 512, 256, 128, 64
- Method: Simple truncation (no retraining needed)

**Quantization support:**
- int8 quantization (post-encoding)
- Binary quantization (ubinary, 1-bit per dim)

#### BGE-Reranker Unique Primitives

**Classification head:**
```
ClassificationHead:
  Input: CLS token representation
  Dense: 1024 → 1
  Activation: None (logit output)
  Output: Relevance score (scalar)
```

## Tensor Primitive Inventory

### Shared Across All Models

| Primitive | Shape Range | Count | Total Params |
|-----------|-------------|-------|--------------|
| Q projection | [1024, 1024] | 24 | 25,165,824 |
| K projection | [1024, 1024] | 24 | 25,165,824 |
| V projection | [1024, 1024] | 24 | 25,165,824 |
| Attn output | [1024, 1024] | 24 | 25,165,824 |
| FFN intermediate | [1024, 4096] | 24 | 100,663,296 |
| FFN output | [4096, 1024] | 24 | 100,663,296 |
| LayerNorm γ | [1024] | 48 | 49,152 |
| LayerNorm β | [1024] | 48 | 49,152 |

**Encoder total:** ~302M parameters

### Model-Specific Embeddings

| Model | Word Emb | Pos Emb | Type Emb | Total Emb |
|-------|----------|---------|----------|-----------|
| BGE-M3 | 250,002×1024 | 8,194×1024 | None | 264M |
| mxbai-large | 30,522×1024 | 512×1024 | 2×1024 | 32M |
| BGE-Reranker | 250,002×1024 | 8,194×1024 | None | 264M |

## Inference Operations

### Forward Pass (per token, single layer)

**Operations:**
1. LayerNorm: O(hidden)
2. QKV projection: 3 × hidden² FLOPs
3. Attention: O(seq_len² × heads × head_dim)
4. Output projection: hidden² FLOPs
5. LayerNorm: O(hidden)
6. FFN expansion: hidden × intermediate FLOPs
7. FFN contraction: intermediate × hidden FLOPs
8. Output projection: hidden² FLOPs

**Total per layer:** ~12M FLOPs/token  
**Total (24 layers):** ~288M FLOPs/token

### Memory Requirements (FP32)

| Model | Model Size | KV Cache* | Peak Memory** |
|-------|-----------|-----------|---------------|
| BGE-M3 | 2.27 GB | 64 MB | ~3 GB |
| mxbai-large | 1.34 GB | 4 MB | ~2 GB |
| BGE-Reranker | 2.27 GB | 64 MB | ~3 GB |

*KV cache for batch=1, seq_len=512  
**Including activations and temporary buffers

## Design Patterns

### Encoder-Only Architecture

All three models use **encoder-only** (BERT-style) architecture:
- No decoder
- Bidirectional attention
- Fixed-length output representations

### Pre-Norm vs Post-Norm

Based on config analysis:
- **Likely post-norm** (original BERT pattern)
- LayerNorm applied after sub-layer + residual

### Pooling Strategies

**Supported patterns:**
1. **CLS pooling:** Use [CLS] token representation
2. **Mean pooling:** Average all token embeddings
3. **Max pooling:** Max over all token embeddings

**Default:** CLS pooling for all models

## Operator Inventory

### Linear Algebra Operators

- **Matrix multiply:** Dense projections (Q, K, V, FFN)
- **Scaled dot-product attention:** Core attention mechanism
- **Layer normalization:** Per-token normalization
- **Softmax:** Attention weight normalization

### Element-wise Operators

- **GeLU activation:** Non-linearity in FFN
- **Dropout:** Regularization (0.1 probability)
- **Add:** Residual connections
- **Divide:** Attention scaling (1/√d_k)

### Reduction Operators

- **Mean:** Mean pooling
- **Max:** Max pooling
- **Sum:** Attention weighted sum

## Precision and Quantization

### Training Precision

- **BGE-M3:** float32
- **mxbai-large:** float16 (mixed precision)
- **BGE-Reranker:** float32

### Inference Precision

All support:
- **float32:** Full precision
- **float16:** 2× speedup on GPU, slight accuracy loss

### Quantization (mxbai-large only)

Native support for:
- **int8:** 8-bit integer quantization (4× compression)
- **ubinary:** 1-bit binary quantization (32× compression)

## References

- BGE-M3: https://huggingface.co/BAAI/bge-m3
- mxbai-embed-large-v1: https://huggingface.co/mixedbread-ai/mxbai-embed-large-v1
- BGE-Reranker-v2-M3: https://huggingface.co/BAAI/bge-reranker-v2-m3
- Attention Is All You Need: https://arxiv.org/abs/1706.03762
- BERT: https://arxiv.org/abs/1810.04805

---

**Extraction Date:** 2026-06-21  
**Primitive Type:** Architecture definitions and tensor shapes  
**Status:** Complete for 3 models
