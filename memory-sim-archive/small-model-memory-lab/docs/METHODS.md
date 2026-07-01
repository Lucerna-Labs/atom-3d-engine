# Methods And Reproduction Notes

This file describes how to reproduce the main experimental paths.

## Important Runtime Separation

Do not mix scores across runtimes without labeling them.

The lab currently uses three different inference families:

1. LM Studio chat API

Used for:

- Qwen3.5 2B Q4_K_M memory benchmarks
- Qwen3.5 2B Q8_0 comparisons

Traits:

- chat-template behavior
- better shaped outputs than raw completion
- no direct residual hooks
- no public direct KV-cache injection interface found

2. llama.cpp raw `/completion`

Used for:

- primitive prefix/KV cache tests

Traits:

- raw base completion
- lower absolute scores
- exposes prefix cache behavior
- can reuse cached prompt prefixes
- does not provide arbitrary residual-stream hooks

3. HuggingFace / PyTorch

Used for:

- residual-stream activation steering

Traits:

- can register layer hooks
- can add steering vectors to hidden states
- needs HF-format models, not GGUF

## Benchmark Runner: Prompt/RAG Memory

Script:

```powershell
.\scripts\run_tiny_memory_benchmark.ps1
```

Key options:

- `-Model`: model id used by LM Studio or Ollama
- `-UseLmStudioApi`: use LM Studio OpenAI-compatible chat API
- `-DisableThink`: request thinking disabled where supported
- `-BenchmarkFile`: benchmark JSON file
- `-CorpusFile`: restrict retrieval to one corpus file
- `-MemoryCueMode`: `Title`, `Body`, or `Hybrid`
- `-TopK`: number of retrieved memories
- `-NoAnswerShapeCue`: remove family-specific output scaffold
- `-CompactCues`: convert known memory ids into compact cues
- `-StrongMemoryActivation`: stronger instruction-like memory activation, which hurt Q4 in tests

### Capability Q4 Run

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\run_tiny_memory_benchmark.ps1 `
  -Model 'qwen3.5-2b-base@q4_k_m' `
  -UseLmStudioApi `
  -DisableThink `
  -NoAnswerShapeCue `
  -BenchmarkFile 'capability_tasks.json' `
  -LmStudioMaxTokens 4096
```

Canonical result:

- run: `runs\tiny-memory-20260604-102754-893`
- baseline: `26/38`
- memory: `31/38`

### Capability Q8 Run

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\run_tiny_memory_benchmark.ps1 `
  -Model 'qwen3.5-2b-base@q8_0' `
  -UseLmStudioApi `
  -DisableThink `
  -NoAnswerShapeCue `
  -BenchmarkFile 'capability_tasks.json' `
  -LmStudioMaxTokens 4096
```

Canonical result:

- run: `runs\tiny-memory-20260604-102842-929`
- baseline: `23/38`
- memory: `28/38`

### Computation Q4: Old Corpus Hurt

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\run_tiny_memory_benchmark.ps1 `
  -Model 'qwen3.5-2b-base@q4_k_m' `
  -UseLmStudioApi `
  -DisableThink `
  -NoAnswerShapeCue `
  -BenchmarkFile 'computation_damage_tasks.json' `
  -LmStudioMaxTokens 4096
```

Canonical result:

- run: `runs\tiny-memory-20260604-103441-668`
- baseline: `44/67`
- memory: `39/67`

### Computation Q8: Old Corpus Hurt Less

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\run_tiny_memory_benchmark.ps1 `
  -Model 'qwen3.5-2b-base@q8_0' `
  -UseLmStudioApi `
  -DisableThink `
  -NoAnswerShapeCue `
  -BenchmarkFile 'computation_damage_tasks.json' `
  -LmStudioMaxTokens 4096
```

Canonical result:

- run: `runs\tiny-memory-20260604-103517-417`
- baseline: `48/67`
- memory: `46/67`

### Computation Q4: Best Memory Recipe

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\run_tiny_memory_benchmark.ps1 `
  -Model 'qwen3.5-2b-base@q4_k_m' `
  -UseLmStudioApi `
  -DisableThink `
  -BenchmarkFile 'computation_damage_tasks.json' `
  -CorpusFile 'computation_primitives_v0_1.json' `
  -MemoryCueMode Body `
  -TopK 3 `
  -LmStudioMaxTokens 4096
```

Canonical results:

- run: `runs\tiny-memory-20260604-104217-527`, baseline `50/67`, memory `58/67`
- repeat: `runs\tiny-memory-20260604-104425-388`, baseline `51/67`, memory `58/67`

### Computation Q8: Compute Body Comparison

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\run_tiny_memory_benchmark.ps1 `
  -Model 'qwen3.5-2b-base@q8_0' `
  -UseLmStudioApi `
  -DisableThink `
  -BenchmarkFile 'computation_damage_tasks.json' `
  -CorpusFile 'computation_primitives_v0_1.json' `
  -MemoryCueMode Body `
  -TopK 3 `
  -LmStudioMaxTokens 4096
```

Canonical result:

- run: `runs\tiny-memory-20260604-104243-433`
- baseline: `55/67`
- memory: `54/67`

## Primitive Cache Benchmark

Script:

```powershell
python .\scripts\run_primitive_cache_benchmark.py
```

Purpose:

- test primitive prefix cache through llama.cpp raw `/completion`

Prerequisite:

- `llama-server` running on `http://127.0.0.1:18081`
- Qwen3.5 2B Q4_K_M GGUF loaded

Clean run command:

```powershell
python .\scripts\run_primitive_cache_benchmark.py `
  --server-url 'http://127.0.0.1:18081' `
  --benchmark-file 'computation_damage_tasks.json' `
  --n-predict 220 `
  --baseline-slot-id 0 `
  --primitive-slot-id 1 `
  --routed-slot-id 1
```

Canonical result:

- run: `runs\primitive-cache-20260604-113609`
- baseline raw completion: `29/67`
- universal primitive cache: `36/67`
- routed primitive cache: `34/67`

Meaning of `cache_n`:

- llama.cpp reused that many prompt-prefix tokens from cache
- universal primitive cache averaged `244` cached tokens in the clean run

Important limitation:

- this is prefix-cache reuse, not arbitrary hidden KV tensor injection

## Activation Steering Probe

Script:

```powershell
python .\scripts\activation_steering_probe.py
```

Purpose:

- compute one residual-stream steering vector
- apply it to one prompt
- save vector/report

Smoke test:

```powershell
python .\scripts\activation_steering_probe.py `
  --model sshleifer/tiny-gpt2 `
  --dtype float32 `
  --device-map cpu `
  --strength 1.0 `
  --max-new-tokens 40 `
  --out .\runs\activation-steering-smoke
```

Meaning:

- proves the hook mechanism works
- not a capability test

## Activation Steering Benchmark

Script:

```powershell
python .\scripts\run_activation_steering_benchmark.py
```

Purpose:

- compute contrastive careful-arithmetic vectors
- sweep layer/strength
- score against the computation benchmark

Wide sweep:

```powershell
python .\scripts\run_activation_steering_benchmark.py `
  --model Qwen/Qwen3-1.7B-Base `
  --dtype bfloat16 `
  --device-map auto `
  --layers 7,14,21 `
  --strengths 0.5,1.0,2.0 `
  --max-new-tokens 180
```

Canonical result:

- run: `runs\activation-steering-20260604-115247`
- baseline: `38/67`
- best: layer `21`, strength `0.5`, `43/67`

Late-layer sweep:

```powershell
python .\scripts\run_activation_steering_benchmark.py `
  --model Qwen/Qwen3-1.7B-Base `
  --dtype bfloat16 `
  --device-map auto `
  --layers 20,21,22 `
  --strengths 0.25,0.5,0.75 `
  --max-new-tokens 180
```

Canonical result:

- run: `runs\activation-steering-20260604-120833`
- baseline: `38/67`
- best: layer `21`, strength `0.5`, `43/67`
- tied best: layer `22`, strength `0.75`, `43/67`

## Scoring

Benchmarks are criteria based.

Each task has regex patterns for required artifacts:

- specific setup terms
- intermediate values
- final answer
- units
- comparison or uncertainty artifacts where relevant

This means some answers can be logically close but lose points if they omit visible intermediate values. That is intentional because the experiment is about repairing small-model reliability, not only final-answer guessing.

## Known Caveats

- Scores are not directly comparable across runtimes.
- LM Studio chat mode can shape outputs more helpfully than raw llama.cpp completion.
- Raw base models may continue lists, prompts, or examples.
- Negative memories can leak bad answer tokens.
- Prompt cache is not the same as direct tensor/KV injection.
- Residual steering requires HF/PyTorch model access.
- Current datasets are small; repeat runs and held-out tasks are needed before publication claims.
