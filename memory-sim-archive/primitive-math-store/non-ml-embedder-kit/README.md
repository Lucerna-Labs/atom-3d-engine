# Non-ML Embedder Kit + Orchestrator

This folder contains a `non-ml-embedder-primitives` crate (mechanism-only)
and a `non-ml-embedder-orchestrator` crate (policy + composition), mirroring the
`driver-primitives` / `redox-virtio-drivers` split in
`D:\primitves math\Example kernel driver build`.

- `non-ml-embedder-primitives`
  - tokenization
  - term frequency
  - document frequency
  - vocab selection
  - idf weighting
  - sparse-to-dense TF-IDF
  - dimension projection
  - cosine similarity

- `non-ml-embedder-orchestrator`
  - fit pipeline over a corpus
  - embedding policy defaults (min_df, vocab cap, output dimension, normalization)
  - `embed`, `embed_many`, `search` orchestration

## Build / Run

From this folder:

```bash
cargo run -p non-ml-embedder-orchestrator
```

## Why non-ML

The embedder is deterministic and does not use learned parameters:
- token frequencies and document frequencies are only corpus-derived statistics,
- no transformer/ML model is invoked,
- similarity is plain cosine math.
