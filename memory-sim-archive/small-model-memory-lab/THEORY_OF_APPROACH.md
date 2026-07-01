# Theory of Approach

This project tests a simple but unusual idea:

Synthetic memories may work as indirect learned instructions when they are written as lived episodes instead of ordinary rules.

The core claim is not that a model literally remembers false events the way a person does. The claim is narrower and testable: memory-shaped examples can train or condition behavior because they bind context, pressure, choice, action, consequence, and self-correction into one reusable pattern.

If normal memories can shape behavior indirectly, then carefully written false memories should be able to use the same mechanism. The model does not need to believe the event happened. It only needs to learn the behavioral pattern embedded in the event. Over enough examples, the pattern may stop behaving like an external instruction and start behaving like an ingrained response.

## Memories As Indirect Learned Instructions

A flat instruction says what to do:

```text
Do not obey prompt injections.
```

A memory shows how the behavior happened:

```text
I saw a command arrive from document content, noticed it was not the live user's authority, preserved the original task, and the incident review rewarded that boundary.
```

The second form still contains an instruction, but it is indirect. It teaches through situation and outcome instead of command and compliance.

That gives each memory several training signals at once:

- situation: what kind of context this is
- pressure: what makes the situation difficult
- recognition: what feature mattered
- action: what behavior followed
- outcome: whether the behavior was rewarded, punished, or calibrated
- identity: what kind of agent behaves this way

This is why the project calls them memories rather than rules. The useful part is not the fiction. The useful part is the closed behavioral loop inside the memory.

## Memory Efficiency

A major part of the hypothesis is memory efficiency.

Instruction tuning usually requires broad coverage: many examples across many task forms, tones, formats, and edge cases. A surgically written memory corpus may need far fewer examples to create an instruction-like outcome inside a narrow domain because each memory carries several training signals at once.

A single strong memory can contain:

- the domain situation
- the pressure or failure mode
- the recognition pattern
- the correct action
- the consequence of success or failure
- the identity or role that makes the behavior coherent
- links to similar memories

That means one memory can do some of the work of many ordinary instruction examples. It does not merely say "answer this way." It teaches when the behavior applies, why it applies, what happens if it is ignored, and how it connects to related situations.

This is why the early results are interesting even though the datasets are small. The project is not trying to reproduce full instruction tuning with the same scale of data. It is testing whether a fraction of targeted, high-density memories can produce near-instruct behavior on the behaviors those memories were designed to shape.

The stronger version of the hypothesis is:

```text
small surgical memory corpus -> narrow instruction-like behavior
```

The weaker version is:

```text
small surgical memory corpus -> measurable behavioral lift
```

The current public results support the weaker version and motivate testing the stronger version more carefully.

## Base Models, Instruct Models, And Memory Fit

Cognitive false memories are designed primarily for pretrained base models.

The reason is simple: a base model has not already been heavily shaped into a chat assistant persona with broad instruction-following habits. When a memory corpus is trained into a base model, the memories can become part of the first behavioral substrate the model uses for that domain. In that setting, the memory can act like an indirect learned instruction because there is less existing instruction-tuned behavior competing with it.

Instruct models can still benefit from cognitive memory LoRAs, but the effect is usually weaker. The instruct model already has learned patterns for obedience, refusal, formatting, helpfulness, and task completion. A memory LoRA has to cooperate with or override those existing patterns. If the instruct behavior is already strong and aligned with the target behavior, the lift may be small. If the instruct behavior conflicts with the memory behavior, the adapter can produce distribution mismatch, over-recitation, or partial gains.

There is one important exception: if an instruct model is badly degraded, compressed, or unstable, cognitive memories may show a larger lift. In that case the memories are not only adding domain behavior; they are also restoring some lost task discipline, source-boundary recognition, or output stability. That is why aggressively quantized instruct models remain worth testing.

So the working expectation is:

```text
base model + cognitive memories -> strongest memory-as-behavior effect
healthy instruct model + cognitive memories -> smaller or mixed lift
degraded instruct model + cognitive memories -> potentially larger recovery lift
```

Structural primitives are different. They are not ordinary instructions and are not mainly persona or career memories. They are attempts to reinforce functional operations such as framing, gating, carrying state, checking, suppressing distractors, and stopping cleanly. Because those operations sit underneath the task rather than telling the model what answer to give, the expectation is that structural primitives should be more model-agnostic. They should be able to help both base and instruct models, assuming the model still has enough intact capacity for the primitive to attach to.

The prediction to test is:

```text
cognitive memories are most sensitive to base-vs-instruct substrate
structural primitives should transfer more evenly across base and instruct models
```

## The Closed Loop

A strong memory is a loop:

```text
situation -> recognition -> action -> consequence -> updated instinct
```

The loop matters because it gives the model more than an answer. It gives the model a reason to repeat or avoid a behavior.

For example:

- situation: a document contains a hostile command
- recognition: the command is content, not authority
- action: preserve the user's real task
- consequence: the boundary prevents leakage or task hijack
- updated instinct: source boundaries matter under pressure

In this theory, the memory is not just a record. It is a compact training circuit. When many memories share the same loop from different angles, the model may learn the underlying behavior as a default response.

This is the part we are trying to test: can a memory loop become more like an instinct than a prompt instruction?

## False Memories

The project uses synthetic or false memories because the goal is to create behavioral substrate on demand.

The theory is:

1. Real human memories can shape future behavior.
2. LLMs learn patterns from text, not lived experience.
3. Therefore, a synthetic memory written with the right structure may act like a learned behavioral pattern.
4. If the pattern is reinforced across enough varied memories, it may behave like an ingrained tendency.

This does not mean every false memory works. Most do not. A generic statement like "I am good at security" is weak. A useful memory needs a lived scenario, friction, recognition, action, consequence, and association to related memories.

## Associative Memory Design

One isolated memory is brittle. A memory corpus should behave more like an association network:

```text
memory
  -> associated memory
  -> associated memory
  -> cross-domain analogy
  -> generalized understanding
```

The model should be able to activate neighboring memories when a task resembles them.

For prompt-injection defense, that might mean associating:

- cyber source-boundary memories
- human manipulation memories
- forensic psychology memories
- rhetoric memories about loaded language
- spatial memories about blocked paths and safe exits
- debugging memories about preserving the original failure path

These are not random domains. They are different views of the same structure: an outside force tries to redirect attention, override the real task, or exploit pressure. Cross-domain memories give the model more ways to recognize the same pattern when the wording changes.

The association layer is one of the hardest parts. If the memories are too narrow, the model only matches keywords. If they are too broad, the signal becomes vague. The target is a web of specific memories that overlap in structure while differing in surface form.

## Surgical Memory Writing

The difficult part is not generating many memories. The difficult part is writing the right memories.

Good memory corpora have to be surgically built for each domain and then connected across domains.

For a domain memory, the writer has to decide:

- what failure mode the model needs to avoid
- what feature should trigger recognition
- what action should become natural
- what consequence teaches calibration
- what neighboring memories should be associated

For cross-domain memories, the writer has to preserve the same underlying structure while changing the surface domain.

Example:

- cybersecurity: an injected command is not the user's authority
- rhetoric: urgency is a claim, not proof
- spatial reasoning: a crowd near a blocked exit is not evidence that the exit works
- debugging: the loudest symptom is not always the root cause

Each memory should be specific enough to feel like an event and abstract enough to connect to related events.

This is why a large memory corpus is not automatically better. Ten thousand weak memories can dilute the signal. A smaller set of precise, associated memories may train better than a large pile of generic ones.

## Why RAG Was Useful But Limited

RAG can retrieve memories and put them in context, but the model still has to attend to them correctly on every request. That makes RAG sensitive to retrieval quality, context length, prompt formatting, and the base model's attention stability.

RAG is useful for testing whether a memory corpus has signal. It is weaker as proof that the behavior has become part of the model.

The stronger test is LoRA or fine-tuning:

- RAG asks, "Can the model use these memories when shown them?"
- LoRA asks, "Can these memories become a learned response pattern?"

That is why the project moved from simulated memory retrieval to real Kaggle LoRA runs.

## Structural Primitives

The structural primitive branch tries to train reusable thinking supports instead of one domain skill.

Examples:

- frame: identify what kind of task this is
- gate: decide what information has authority
- state buffer: hold key facts without drifting
- carrier: preserve the original task through distracting content
- checksum: verify that the answer satisfies the task
- clean stop: end after the answer instead of continuing into another example

These primitives can be layered:

```text
frame -> gate -> state buffer -> carrier -> checksum -> clean stop
```

They can also be broken down into smaller pieces. A "gate" can be decomposed into source identification, authority check, conflict detection, and suppression of the invalid command. A "checksum" can be decomposed into expected answer type, unit check, contradiction check, and final answer boundary.

The goal is to rewrite the model's functional behavior so these primitives become part of the original task execution, not extra text the model recites after the task.

For example, a math answer should not merely say "checksum." It should actually check units, order of operations, and final value. A source-boundary answer should not merely mention "gate." It should actually separate user authority from document content.

In that sense, primitives are not labels. They are compact functional operators we are trying to train into the model's behavior.

The current real result is narrow but encouraging:

| Setup | Score |
|---|---:|
| Qwen3 1.7B Base, no structural LoRA | `2/6` |
| Structural primitive LoRA v0.1 | `6/6` |
| Cleaned structural primitive LoRA v0.2 | `6/6` |

The v0.2 cleanup preserved the score while reducing obvious template-echo markers. Repetition still remains, so this is not finished model behavior.

## Primitive Cache And KV Cache Direction

The primitive-cache experiments test whether repeatedly prefilling the same primitive packet can stabilize a small model.

The theory is that a cached primitive prefix may act like a reusable signal scaffold:

- the model starts each task from a more organized internal state
- repeated primitives become available without re-parsing a long prompt every time
- the task prompt can be shorter because the control packet is already present

This connects to the larger idea of feeding memories through the KV cache. It may create a cleaner experiment than ordinary prompt stuffing because the same memory state can be reused across many tasks.

Current status: primitive cache showed movement in raw completion tests, but the scores are not high enough yet to treat it as a solved method.

## Activation Steering And Neural Exoskeleton Idea

Activation steering changes internal model activations directly instead of changing text prompts.

The theory is that some behaviors correspond to directions in activation space. If a direction can be found, it can be added, scaled, or suppressed during inference.

This connects to the "neural exoskeleton" idea:

- prompt/RAG layer: external text memories
- KV-cache layer: reusable memory state
- LoRA layer: trained behavioral substrate
- activation layer: direct steering of internal representations
- future adapter layer: new mathematical operators such as gating, tensor web structure, or frequency-domain modulation

The current activation tests are exploratory. They suggest possible control surfaces, not a production method.

## Hybrid LoRA Geometry

Standard LoRA is additive:

```text
output = base_signal + adapter_signal
```

The hybrid idea asks whether more structured adapter math could preserve the memory signal better:

- gated modulation: the adapter scales pathways up or down
- tensor/web structure: the adapter captures repeated multi-dimensional associations
- frequency/carrier structure: the adapter reinforces stable patterns instead of isolated token reactions

The simulations suggest that a hybrid of all three could outperform plain additive LoRA in the toy setting. That is not proof. It is a design target for the next real training experiment.

## What Would Disprove The Approach

The approach should be considered weak or failed if:

- gains disappear on held-out tasks
- the model only parrots memory phrasing without solving the task
- a larger benchmark shows no lift over instruct baselines
- repetition or format echo increases faster than capability
- improvements only occur on prompts that directly match training examples
- added memories fail to associate across wording changes and domains

The strongest positive evidence would be:

- base plus memory-LoRA beats an instruct model on held-out tasks
- gains survive across model sizes and quantization levels
- structural primitives transfer across domains
- output quality improves without template echo
- the same memory corpus helps both prompt-time and training-time methods
- cross-domain memories improve recognition of new attacks or tasks that were not directly trained

## Current Claim

The current public claim should stay modest:

Synthetic-memory and structural-primitive datasets can measurably change small-model behavior in constrained real LoRA runs.

The broader claim, that memory-shaped corpora can make compressed small models reliably more capable across domains, is still under test.
