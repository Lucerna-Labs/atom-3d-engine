# Embedding & Reranker Primitive Extraction Plan

## What We're Extracting

Following the Linux primitives pattern, we extract the **structural/operational primitives** that make up embedding/reranker engines:

### Primitive Categories

1. **Architecture Primitives**
   - Layer configurations (num_layers, hidden_dim, num_heads)
   - Attention mechanisms (self-attention, cross-attention configs)
   - Activation functions (GELU, SwiGLU, ReLU)
   - Normalization layers (LayerNorm, RMSNorm)
   - Pooling strategies (CLS, mean, max)

2. **Tensor Primitives**
   - Weight tensor shapes (Q, K, V, O matrices)
   - Embedding table dimensions
   - Positional encoding schemes
   - Output projection shapes

3. **Operator Primitives**
   - Matrix multiplication patterns
   - Softmax variants
   - Dropout configurations
   - Quantization parameters

4. **Tokenizer Primitives**
   - Vocabulary files
   - Special token mappings
   - Tokenization rules (BPE, WordPiece, Unigram)
   - Normalization pipelines

5. **Inference Graph Primitives**
   - Forward pass structure
   - Skip connection patterns
   - Multi-head attention layout
   - Feed-forward network structure

## Extraction Sources

### BGE-M3
- Source: https://huggingface.co/BAAI/bge-m3
- Files to extract:
  - `config.json` → architecture primitives
  - `tokenizer.json` → tokenizer primitives  
  - `model.safetensors.index.json` → tensor name/shape map
  - Code: https://github.com/FlagOpen/FlagEmbedding

### mxbai-embed-large-v1
- Source: https://huggingface.co/mixedbread-ai/mxbai-embed-large-v1
- Same extraction pattern

### BGE-Reranker Models
- Cross-encoder specific primitives
- Pairwise scoring architecture
- Classification head configs

## Output Structure

```
F:\embedding-and-reranker-primitives\
├── output\
│   ├── bge-m3\
│   │   ├── architecture_analysis.md
│   │   ├── architecture_data.json
│   │   ├── tensor_analysis.md
│   │   ├── tensor_data.json
│   │   ├── operator_analysis.md
│   │   ├── operator_data.json
│   │   └── tokenizer_analysis.md
│   │   └── tokenizer_data.json
│   ├── mxbai-large\
│   │   └── [same pattern]
│   └── rerankers\
│       ├── bge-reranker-v2-m3\
│       └── bge-reranker-gemma2\
├── 00-ARCHITECTURE-SUMMARY.md
├── 01-TENSOR-PRIMITIVES-SUMMARY.md
├── 02-OPERATOR-PRIMITIVES-SUMMARY.md
└── 03-TOKENIZER-PRIMITIVES-SUMMARY.md
```

## Extraction Method

Since we can't install Python packages easily, I'll:
1. Fetch config files via curl/web_fetch
2. Parse JSON manually 
3. Extract structural info into analysis docs
4. Generate JSON data files with shapes/dims/operators
5. Create summary documents following the Linux primitives pattern

This gives you the **blueprint** of how these models work without the actual weights.
