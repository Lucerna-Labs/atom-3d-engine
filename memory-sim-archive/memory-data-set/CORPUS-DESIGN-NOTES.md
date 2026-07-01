# Memory Corpus Design Notes

## Core Principle

A memory is not a story. A memory is a **steering signal** with emotional weight.

A woman who heard on the news that a neighborhood is dangerous does not walk around with the full news report in her head. She has an *impression*: "this is a dangerous part of town, be careful." That impression, with enough weight, fires before she walks down the street. The full incident — the victim, the crime, the date, the anchor's voice — is gone. What remains is the gist with weight.

We are building that. The gist, not the report.

---

## Why This Matters

A model trained on full scenes with rich detail learns to *narrate* the scenes. A model trained on compressed memories learns to *act* on the signal.

Full scenes are literature. Compressed memories are instinct. We are not writing literature. We are installing instinct.

A child who burns their hand on the stove does not replay the incident before every kitchen task. They have a flinch. The flinch is the memory. The flinch fires before the hand reaches.

For the cyber security persona, the equivalent of the flinch is:
- Reading a low-priority alert, the cadence looks familiar, the flinch fires, the analyst reads the PCAP
- A vendor's analytics platform is in the environment, the flinch fires, the analyst treats it as a possible mask
- A packet capture is on the second monitor, the flinch fires, the analyst stays in the seat

Each of these is a compressed memory with weight. The set of them is the instinct.

---

## The Compressed Format

Each memory is **20-50 words**. Some anchors can stretch to 60-80 if the scene is dense. Reinforcements stay short.

The structure is:
- A *category* of moment (when, where, what kind of situation)
- A *signal* (what tipped the analyst off, or what was the trigger)
- A *response* (what the analyst did)
- A *weight* (the cost, the lesson, the scar)

What is **not** in the compressed memory:
- Specific timestamps (no "2:14am", no "Tuesday in October")
- Specific counts (no "42 days", no "11 minutes", no "41,000 customers")
- Forensic detail (no specific TLD names, no specific beacon patterns beyond the category)
- Sensory texture beyond the category (no "the blue-white of three monitors", no "the hum of HVAC")

What **is** in the compressed memory:
- Categories of time: "a long night shift", "that Thursday", "by morning", "weeks of dwell time"
- Categories of count: "tens of thousands", "most of the night", "a few minutes"
- Categories of place: "the SOC", "the parking lot", "the post-mortem room"
- The signal: "the cadence was off", "the workstation was supposed to be off", "the user was on PTO"
- The response: "I pulled the PCAP", "I paged Marcus", "I stayed in the seat"
- The cost: "the dwell time was months", "tens of thousands of customers", "the scar"
- The lesson: "the flinch is the muscle", "auto-dismiss is not a verdict", "the muscle passes down"

---

## The Set, Not the Single Memory

A single compressed memory is a *condensation nucleus*. It carries one facet of a behavior. By itself it is too thin to install robust instinct.

The **set** of compressed memories, each carrying a different facet, is what installs the behavior. The set has:
- An anchor or two (the load-bearing compressed scene, with the most weight)
- 5-8 reinforcement memories (each one a different facet, a different trigger, a different scene)
- Optional: a "carrying the scar" memory or two (the slow burn, the duration, the way the lesson lives in the body)

The grandmother example: you don't have one dense memory of "my grandmother loved me." You have a *set* of small memories — the hand-holding in public, the standing between you and the bullies, the crying when you broke your arm, the food she made, the way she watched the street. The set is the love. Each small memory is a condensation nucleus for the same load-bearing truth.

For the cyber corpus, the set for the flinch truth includes:
- The anchor: "I dismissed once. The dwell time was months. The muscle catches the next one."
- The first reflex: "I pulled a PCAP once. The alert was nothing. The nothing was the lesson."
- The retuned rule: "The auto-dismiss rule was retuned. The scar moved from the analyst to the system."
- The post-it: "I wrote a post-it. A junior asked. The muscle passes down."
- (To be added) The carrying the scar: the morning after, the slow burn, the duration
- (To be added) The high-pressure night: a 36-hour incident, the muscle under pressure
- (To be added) The vendor pattern: the analytics platform as a mask, the supply chain
- (To be added) The escalation: paging Marcus, the cost of escalation, the second ring

The set is the flinch. No single memory is the flinch. The model trained on the set develops the flinch.

---

## Why Categories of Time and Count

Specific numbers in a model are brittle. A model trained on "42 days of dwell time" will recognize 42 days, but it will not transfer the instinct to 30 days or 90 days. A model trained on "weeks of dwell time" or "months of dwell time" recognizes the *category* and transfers the instinct across the range.

Specific timestamps in a memory are also brittle. A model trained on "2:14am" learns that 2:14am is a thing, not that late-night shifts are dangerous. A model trained on "a long night shift" learns the *category of time* and the *category of risk* that comes with it.

This is also how people remember. We don't walk around with "the 2:14am alert" in our head. We have "that night shift where I almost dismissed." The exact minute is gone. The category of the night is what remains.

---

## The Source Materials Folder

When a memory is compressed for the corpus, the full scene (if it was ever written) goes in `source-materials/`. The source material is for *human reference* — to remember what the corpus is encoding, to debug when a sim result is unexpected, to write new compressed memories from.

The source material does **not** go into training. Only the compressed memories do.

This is critical: the model never sees the full scene. The model only ever sees the compressed form. The full scene is for us, not for the model.

---

## Future Corpus Design

When building new memory sets (for new domains, new behaviors, new personas), the same architecture applies:
- Compressed, 20-50 word memories
- Sets of 5-8 reinforcements per load-bearing behavior
- 1-2 anchors per behavior
- Echo chains between memories
- Source materials kept separately

The format is not a stylistic choice. The format is the architecture. Compressed memories install behavior. Full scenes install narration. We want behavior.

---

## Iteration Rule

When the sim shows a gap in installed behavior:
- Do not expand the existing memories into full scenes. The compressed form is correct.
- Add more *condensation nuclei* — more small memories carrying the same truth through different facets.
- The set gets denser. The individual memories do not get longer.

The set is the flinch. The set gets denser. The flinch fires harder.
