# Extracted Primitives - Complete Inventory

Following the Linux primitives extraction pattern.

## What Was Extracted

Structural and operational primitives that define embedding/reranker engines:

### 1. Architecture Primitives ✅

**Location:** `output/*/architecture_*.md|json`

**Extracted from model configs:**
- Layer configurations (24 layers, 1024 hidden, 16 heads)
- Attention mechanisms (self-attention, absolute positions)
- Activation functions (GeLU)
- Normalization layers (LayerNorm, eps=1e-05/1e-12)
- Pooling strategies (CLS, mean, max)

**Models:**
- ✅ BGE-M3 (XLMRobertaModel)
- ✅ mxbai-embed-large-v1 (BertModel)
- ✅ BGE-Reranker-v2-M3 (XLMRobertaForSequenceClassification)

### 2. Tensor Primitives ✅

**Location:** `architecture_data.json` → `tensor_shapes` section

**Extracted tensor shapes:**
- Word embeddings: [vocab_size × hidden_dim]
- Position embeddings: [max_positions × hidden_dim]
- Q/K/V projections: [hidden_dim × hidden_dim]
- Attention output: [hidden_dim × hidden_dim]
- FFN intermediate: [hidden_dim × intermediate_dim]
- FFN output: [intermediate_dim × hidden_dim]
- LayerNorm parameters: [hidden_dim]

**Parameter counts:**
- BGE-M3: 567M total (264M embeddings + 303M encoder)
- mxbai-large: 335M total (32M embeddings + 303M encoder)
- BGE-Reranker: ~567M (same as BGE-M3 + classification head)

### 3. Operator Primitives ✅

**Location:** `00-ARCHITECTURE-PRIMITIVES-SUMMARY.md` → "Operator Inventory"

**Linear algebra:**
- Matrix multiplication (dense projections)
- Scaled dot-product attention
- Layer normalization
- Softmax

**Element-wise:**
- GeLU activation
- Dropout (p=0.1)
- Residual addition
- Scaling (1/√d_k)

**Reduction:**
- Mean pooling
- Max pooling
- Attention weighted sum

### 4. Tokenizer Primitives ⏳

**Status:** Vocabulary files need to be fetched separately

**What we have:**
- Vocab sizes (250,002 for XLM-R, 30,522 for BERT)
- Special token IDs (BOS=0, PAD=1, EOS=2)
- Tokenizer type (SentencePiece vs WordPiece)

**What's missing:**
- Actual vocabulary lists
- Merge rules (for BPE)
- Normalization configs

### 5. Inference Graph Primitives ✅

**Location:** `architecture_analysis.md` → "Inference Graph"

**Forward pass structure:**
```
Input IDs → Embeddings → Transformer[0..23] → Pooler → Output
```

**Per-layer operations:**
1. LayerNorm → QKV projection → Attention → Output projection → Add
2. LayerNorm → FFN expansion → GeLU → FFN contraction → Add

## File Structure

```
F:\embedding-and-reranker-primitives\
├── 00-ARCHITECTURE-PRIMITIVES-SUMMARY.md    ✅ Complete
├── PRIMITIVES-EXTRACTED.md                   ✅ This file
│
├── output\
│   ├── bge-m3\
│   │   ├── architecture_analysis.md          ✅ Complete
│   │   └── architecture_data.json            ✅ Complete
│   ├── mxbai-large\
│   │   ├── architecture_analysis.md          ⏳ TODO
│   │   └── architecture_data.json            ✅ Complete
│   └── rerankers\
│       └── bge-reranker-v2-m3\
│           ├── architecture_analysis.md      ⏳ TODO
│           └── architecture_data.json        ⏳ TODO
│
└── models\
    └── bge-m3.json                           ✅ Model metadata
```

## Primitive Categories (By Analogy to Linux)

### Linux Primitives → Embedding/Reranker Equivalents

| Linux Primitive | Embedding/Reranker Equivalent |
|-----------------|-------------------------------|
| **ioctl numbers** | Operator types (attention, layernorm, etc.) |
| **struct definitions** | Tensor shapes and layouts |
| **syscall APIs** | Forward pass interface |
| **header constants** | Hyperparameters (layers, heads, dims) |
| **device drivers** | Model implementations (BertModel, XLMRobertaModel) |
| **file system ops** | Tokenizer operations |
| **network protocols** | Attention mechanisms |

### What Makes These "Primitives"

Following your Linux pattern, these are primitives because they are:

1. **Atomic operations** - Can't be decomposed further (matrix multiply, softmax, etc.)
2. **Composable** - Combine to build complete inference graphs
3. **Standardized** - Same patterns across different models
4. **Documented** - Clear specs with shapes, types, constraints
5. **Reusable** - Same primitives work across BGE-M3, mxbai, etc.

## Next Steps (To Complete Extraction)

### High Priority

1. **Tokenizer primitives**
   - Fetch `tokenizer.json` from each model
   - Extract vocab, merge rules, special tokens
   - Document tokenization pipeline

2. **Operator details**
   - Extract exact operator signatures
   - Document input/output shapes
   - Specify precision requirements

3. **Reranker specifics**
   - Classification head architecture
   - Pairwise scoring mechanism
   - Cross-attention pattern (if any)

### Medium Priority

4. **Multi-functionality primitives** (BGE-M3 only)
   - Sparse retrieval head
   - ColBERT multi-vector head
   - Combination logic

5. **Quantization primitives** (mxbai-large only)
   - int8 quantization spec
   - Binary quantization spec
   - Matryoshka truncation logic

### Low Priority

6. **Performance characteristics**
   - Benchmark data (ops/sec, latency)
   - Memory bandwidth requirements
   - Optimal batch sizes

## Usage

These primitives can be used to:

1. **Implement from scratch** - Build compatible models using these specs
2. **Verify implementations** - Check your code matches extracted shapes
3. **Optimize inference** - Know exact ops to optimize
4. **Port to new frameworks** - Translate primitives to JAX, Rust, etc.
5. **Design derivatives** - Modify primitives for custom architectures

## Comparison to Linux Primitives

**Similarities:**
- Both extract structural/operational specs from source
- Both organized by category (architecture, tensor, operator)
- Both provide JSON data + markdown analysis
- Both serve as reference for implementation

**Differences:**
- Linux primitives = system interfaces (ioctls, syscalls)
- Embedding primitives = mathematical operations (matmul, attention)
- Linux primitives are discrete/countable
- Embedding primitives are continuous/parametric

---

**Extraction Status:** Partially complete (~60%)  
**Models Covered:** 3/3 (architecture only)  
**Next:** Tokenizer and operator details

**Date:** 2026-06-21  
**Pattern:** Following `F:\OPENCLAW-PROJECTS\linux-primitives-extracted`
