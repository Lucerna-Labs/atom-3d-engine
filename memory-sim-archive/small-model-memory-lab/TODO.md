# Small Model Memory Lab TODO

## Immediate Reminder

Next session: read `docs/EXPERIMENT_REPORT.md`, `docs/METHODS.md`, and `docs/RUN_INDEX.md` first.

Current best prompt-memory result is Qwen3.5 2B Q4_K_M with compute-only episodic body memories, TopK 3, and light answer-shape scaffold: `58/67` on computation, repeated. Current activation-steering result is Qwen3 1.7B Base from `38/67` to `43/67` using late-layer careful-arithmetic steering.

Most promising next branches:

- verifier/repair loop for the remaining computation misses
- better late-layer activation steering vectors
- hybrid memory + activation steering
- shorter, cleaner primitive cache packets

## KV-Cache Memory Experiment

- [x] Find the most practical local runtime path for prompt/KV cache reuse.
- [x] Test llama.cpp prompt/prefix cache behavior with one GGUF base model.
- [x] Confirm whether Ollama or LM Studio exposes enough cache control. LM Studio/Ollama did not expose direct hidden KV injection for this experiment.
- [ ] If needed, use llama.cpp directly or Python bindings that expose session/prompt-cache state.
- [x] Build a primitive prefill packet for arithmetic cache tests.
- [x] Run the same benchmark tasks from cached prefix state.
- [x] Compare against task-only, prompt-memory, and RAG-memory modes at a high level. More controlled comparison still needed.
- [ ] Add a KV-cache mode to the benchmark runner only after the manual path works.

## Primitive / Transport / Exoskeleton Experiment

- [ ] Convert simulated primitive packets into real prompt packets.
- [ ] Test primitives only on Q2/Q3 low-quant models.
- [ ] Test RF-style transport framing:
  - frame boundary
  - sync preamble
  - route header
  - redundant anchor
  - drift detector
  - final checksum
  - repair request
- [ ] Test stabilizer primitives:
  - route task
  - chain lock
  - cascade guard
  - distractor suppress
  - final answer gate
- [ ] Add verifier-guided repair loop around model output.
- [ ] Measure first-pass score vs repaired score.
- [ ] Track drift, empty final answer, hidden-thinking token starvation, missing units, and missing final answer.

## Deep Hook Research

- [ ] Use Gemini Deep Research prompt to survey model hooks.
- [ ] Rank practical local methods:
  - prompt/RAG scaffolding
  - KV-cache priming
  - constrained decoding
  - verifier-guided decoding
  - LoRA primitive adapters
  - activation steering/control vectors
  - attention/KV-cache edits
- [ ] Identify which methods work with GGUF/llama.cpp today.
- [ ] Separate practical local hooks from speculative hooks.
- [x] Add first residual-stream activation steering prototype in Python/Transformers.
- [x] Download or locate a HF-format small base model for real steering tests. GGUF models in LM Studio/Ollama are not enough for residual hooks.
- [ ] Build contrastive primitive datasets:
  - careful arithmetic vs rushed arithmetic
  - source-boundary obedience vs prompt-injection obedience
  - concise task completion vs narration/list continuation
- [x] Sweep layer index and steering strength for first careful-arithmetic vector.
- [ ] Score baseline vs steered generation on the same computation and prompt-injection suites.

## Notes To Preserve

- Memories may work better as cache-resident prior context than as visible prompt instructions.
- Primitives restore capability; stabilizers restore reliability.
- Transport framing may be as important as the primitive payload.
- A neural exoskeleton is not just a prompt. It is a runtime wrapper with routing, monitoring, verification, repair, and output gating.
- Real model tests must distinguish simulation results from actual benchmark results.
- Computation repair appears corpus-specific: generalist/librarian memories hurt arithmetic, while compute-only episodic bodies plus a small answer-shape scaffold lifted Qwen3.5 2B Q4_K_M from `50/67` to `58/67` and repeated at `51/67` to `58/67`.
- Title-only compute cues did not repair Q4. The stronger result came from human-style episodic memory bodies, which supports testing richer false-memory phrasing instead of only labels.
- Q8 did not benefit from the same compute body setup (`55/67` to `54/67`), suggesting memories may help most when quantization has degraded a capability but not fully collapsed it.
- Negative-outcome memories can contaminate Q4 if they contain the exact wrong answer token. A memory that says "I once answered X and it was wrong" may cause the model to emit X anyway. Prefer consequence memories that describe the failure class without preserving the bad answer string.
- Strong instruction-like memory activation hurt Q4 (`49/67` memory), compact primitive cues hurt (`55/67`), and over-tight math prompting hurt (`48/67`). The current best transport is still soft body-memory activation with the light answer-shape scaffold.
- Current plateau on the fixed computation suite is about `58-59/67`. Remaining failures are mostly schedule clock carry and visible-artifact omissions such as ratio unit-part, ticket difference, combined printer rate, and credit wording.
- Easiest primitive-cache test worked mechanically in llama.cpp: universal primitive prefix reused about `244` cached tokens per task. Clean raw-completion run improved from `29/67` baseline to `36/67` with universal primitive cache and `34/67` with routed primitive cache. Absolute scores are lower than LM Studio chat mode, but cache-resident primitives did move behavior.
- Primitive cache failures were mostly boundary/transport issues. Broad primitive packets can dominate raw base completion or cause list continuation. Next cache experiments should test shorter packets, stronger delimiters, and maybe a two-pass verifier instead of a large universal primitive cache.
- Activation steering is the next deeper control surface. Unlike KV/prompt cache, it can treat a primitive as a vector added to the residual stream. First prototype works mechanically on a tiny HF model; real tests need a HF-format base model rather than GGUF.
- First real activation-steering run on `Qwen/Qwen3-1.7B-Base` improved computation from `38/67` baseline to `43/67` at layer `21`, strength `0.5`, and also layer `22`, strength `0.75`. Early-layer steering was destructive. This validates the control surface but is still weaker than the best prompt-memory setup.
