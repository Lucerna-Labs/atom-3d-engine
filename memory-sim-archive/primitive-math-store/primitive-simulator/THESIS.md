# Type-Validity Is Not Discovery — A Working Thesis on Simulating Primitives

*Companion to the kit's `../THESIS.md` (the Painted Fence position) and `../HOW-THE-KIT-WORKS.md`. Where those argue that the span of composition is larger than it looks, this argues a narrower, sharper point about how to **search** that span without fooling yourself. Position formed by building two simulators against this store and watching one of them lie.*

---

## Abstract

A primitive store invites a search: enumerate compositions, score them, keep the best. The obvious score is **type-correctness** — does the output of each primitive feed the input of the next, end to end? This thesis argues that type-correctness is the wrong objective for *discovery*, and that mistaking it for discovery is the simulation-shaped version of the doctrine's fourth default — claiming a win without naming what it cost. A composition search scored on type-validity will reliably reconstruct known-good structure (it rebuilds a transformer block from primitives on the first try) and just as reliably **discover nothing new**, because its optimum is fixed by the type graph, not by the world. Real discovery requires a second engine that **executes** candidate compositions and charges them a **conserved currency**. The thesis names the failure mode, gives the evidence, and states the propose-and-validate architecture that fixes it.

---

## Thesis statement

> A simulation discovers something only when a candidate composition is **executed** and the benefit it claims is paid for in a **conserved currency** (compute, field-evaluations, bits, error, latency). Type-validity is a necessary precondition, not a finding. A search scored on type-validity alone produces *architectural theater*: type-correct pipelines that look like discoveries and survive no adversarial execution. Pair a generator that proposes (recipe inference, or a binding-primitive finder) with a validator that executes and charges a currency, and the search becomes honest; omit the validator and every plateau reads as a result.

---

## The default the simulator replaces

Confronted with a primitive store and asked to "find something," the default move is:

1. **Score compositions by how well they type-check.** It is cheap, total, and gives a clean number that goes up. The number going up *feels* like progress.
2. **Report the highest-scoring composition as a discovery.** It is type-correct, has full coverage, looks non-trivial, and no one ran it.

Both steps are the doctrine's Interruption 4 in disguise: a win — "typed=189.10, 100% coverage" — claimed without naming the conserved quantity it charged. The answer is that it charged *nothing*. Nothing was spent because nothing was executed. The score measures a property of the **type graph**, and the type graph does not move when you learn something true about the world.

---

## Evidence

**The recipe-inference run.** Pointed at this store, a type-validity search over 7-primitive recipes found its best score within the first one million recipes and held it, unchanged, through **two hundred and thirteen million** more. The plateau is the tell: an exhaustive-ish search settling on a fixed type-optimum is *confirming a property of the graph*, not generating a stream of findings. Lengthening recipes to 8 raised the ceiling once (inserting a Johnson–Lindenstrauss random projection improved the type-flow) and then plateaued again — a higher number, still charging no currency. Impressive as completeness validation; empty as discovery.

**The same store, executed.** A second engine took cross-domain *generators* — over-relaxation from signal processing, rate-distortion stepping from information theory, a secant root-find from numerical optimization — and ran each *inside* the engine's marcher, measuring field-evaluations at held image correctness. Two outcomes mattered:

- **The secant transfer paid.** Regula-falsi root-finding cut near-surface field-evaluations ~1.17×, and the optimizer *adopted it* in the joint best (lifting it from 2.337× to 2.375× over baseline). It charges a real currency; it is a discovery, small and honest.
- **The empty-space seed did not.** Type-plausible, intuitively appealing, it returned neutral — 0.995×. The executor refused it. That refusal is the load-bearing event: the type-validity search would have ranked it as fine, because it composes; only execution exposed it as painted.

The contrast is the whole thesis. The first engine cannot tell the secant win from the empty-space dud, because both type-check. The second engine separates them, because one spends a currency and the other doesn't.

---

## Binding primitives: where novelty lives

Utility (does it pay?) is half of discovery; **novelty** (is it new?) is the other. The novelty a primitive store can offer is structural: a **binding primitive** — glue that joins primitives, or whole domains, that normally would not compose. The store names these implicitly in its cross-domain wiring; a search makes them explicit by scoring each primitive on **bridge rarity**: how few other primitives connect the domain-pairs it spans, weighted by how load-bearing it is (does it name a real wall, or is it free glue?). The rarest bridges across the most distant domains are the candidates for genuine cross-domain transfer — and the best of them are also *useful*: the top binding primitive against this store, `BVH-traverse`, glues graphics to computational geometry to queueing-theory cost models, and is precisely the spatial-acceleration structure the renderer it was tested against is missing. Novelty surfaced by the bridge score; utility still owed to the executor.

---

## What this thesis does not claim

It does not claim type-validity is useless — it is a necessary filter, and reconstructing a transformer block from primitives is a real validation of the kit's completeness. It does not claim execution is cheap — the executor is the expensive half, which is exactly why the default avoids it. It does not claim the binding score is utility — it is a novelty heuristic that *nominates* candidates for the executor to judge. And it does not claim the discoveries will be large; the honest ones so far are modest (a 1.17× transfer), but they charge a currency, and a small win that names its cost outranks a large number that names none.

---

## Conclusion

Build the search with two engines, not one. Let a proposer — recipe inference over the type graph, or a binding-primitive finder over the domain graph — generate type-valid, novel candidates cheaply and in bulk. Then make every candidate face an **executor** that charges it a conserved currency, and keep only the ones that pay. The proposer guards completeness and novelty; the executor guards utility and honesty. A composition that survives both is a discovery. A composition that only survives the proposer is what the type-validity search keeps mistaking for one.

The simulator in this folder is that pair, built. Its rule is the doctrine's fourth interruption made mechanical: *name the currency, or it isn't a win.*

---

*Position based on building `xdsim` against this store. Honest about limits; corrigible to evidence. Source doctrine: The Painted Fence — Jesse.*
