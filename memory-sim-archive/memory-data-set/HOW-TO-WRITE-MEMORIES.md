# How to Write Memory Data Sets

**Purpose:** Reference for building persona-agnostic memory corpora. The model extracts valuable behavior from lived episodes, not from being told the rules.

---

## Core Principle

These are **first-person memories**, not instructions. Not lessons. Not rules. They are scenes — full episodes, beginning to end — written from inside the perspective of someone who lived them.

The model reads the episode the way a child reads the room. It extracts the instruction by *being there*, not by being told. A child who burns their hands on the stove once does not need a rule called "do not touch the stove." They have a memory. The memory *is* the rule, but the rule is never retrieved consciously — it fires as instinct, as hesitation, as a flinch before the hand reaches.

We are building that. Across every domain.

---

## What a Memory Looks Like

A memory is a **complete scene**. Not a summary. Not a bullet point. A scene.

It has:
- **A setting** — where, when, what the environment looked/felt/smelled like
- **A protagonist (you, the persona)** — first person, present tense or past tense but inside the experience
- **An action or decision point** — what was about to happen, what was being attempted
- **A consequence** — what actually happened. The burn. The breach. The catch. The miss.
- **A body** — sensory texture. The way the light fell. The sound the alert made. The smell of ozone. The cold sweat.
- **An afterimage** — the thing that stays. What you reach for next time, automatically, without thinking.

A memory that lacks any of these is incomplete. A memory that is a summary of what happened is not a memory. It is a Wikipedia article, and it will not install behavior.

---

## The Episodic Chunk

Memories are written in **chunks**, not in isolation. A chunk is one full scene — one moment, one decision, one outcome. The length varies based on the density of the scene, but the rule is: **stop the chunk when the scene ends.** A scene ends when the immediate consequence has been absorbed and the protagonist is no longer in the middle of the experience.

A scene is *not* a career arc. A scene is not a year. A scene is:
- The 14 minutes you spent inside a compromised host before you noticed the beacon.
- The 3 seconds of hesitation before clicking the link you should have known was bad.
- The 6 hours of incident response when the SOC paged you at 2am.
- The 20 minutes you spent staring at a packet capture before the protocol violation jumped out.

Chunks have a clear arc. Beginning, middle, end. Cause, action, result. Setup, pivot, payoff.

---

## The Failure + Success Pairing

This is the load-bearing structure. Every behavior we want to install is installed by **two memories**, sometimes more:

- **A failure memory:** "This is what happened when I got it wrong. Here is the cost. Here is what I felt. Here is what I would do differently — but I did not, in the moment, because I did not yet know."
- **A success memory:** "This is what happened when I got it right. Here is what tipped me off. Here is the move I made. Here is what it felt like to catch it."

The model does not see a rule. The model sees two scenes. From the contrast, it extracts:
- The pattern that distinguished failure from success
- The cost of failure (visceral, not abstract)
- The signal that preceded success
- The action that produced the right outcome

This is how a child learns. This is how a tradecraft is built. This is how instinct is installed.

**Critical rule:** failure memories and success memories must be written in the same voice, with the same level of sensory detail, with the same kind of aftermath. They are two memories about the same kind of moment, not "a bad memory" and "a good memory." The model is not being graded. It is living through both, and from the contrast, the behavior emerges.

---

## The Echo Chain

No memory stands alone. Each memory **echoes** to a few other memories. The echo is the connection — "this is the same kind of moment as that one, except rotated, except scaled, except the variables were different."

The echo chain is what gives the corpus **coverage**. One memory alone installs a narrow behavior. Three memories, each echoing a different facet, install a robust behavior. The chain is the difference between:
- "I should not run unsigned binaries" (rule, brittle)
- "I have run unsigned binaries three times. Once it was fine. Once it was a pentest. Once it was the thing that paged the SOC at 2am. I do not run unsigned binaries anymore." (memory, robust)

When you write a memory, you identify **2-5 echoes** — other memories in the corpus, already-written or to-be-written, that this one connects to. The echoes go in the memory's metadata.

**Echo types:**
- **Same-shape, different-scale:** a small version of the same pattern (reconning a single host vs. reconning a network)
- **Same-shape, rotated:** the same pattern in a different context (catching a phish vs. catching a malicious insider using the same signal)
- **Same-shape, inverted:** success where there was failure, or failure where there was success (the contrast pair)
- **Source-bridge:** this domain's pattern connects to a different domain's pattern (the cyber "hunting" instinct echoes the rhetorical "reading the room" instinct)

Echo chains are what let the model **transfer** behavior across situations. Without echo chains, the corpus is a pile of scenes. With echo chains, it is a *web*, and the model can navigate from one scene to another the way a person does when they say "this is like that time, except..."

---

## What a Memory Is Not

- **Not a rule.** Do not write "always validate input." Write the time you didn't.
- **Not a tutorial.** Do not write "first, you enumerate ports. then you..." Write the night you forgot to.
- **Not a list.** Bulleted memories are not memories. They are notes about memories.
- **Not a summary.** "I once responded to a major breach" is not a memory. The 14 hours inside the war room, the way the CISO's voice changed at hour 6, the moment the third-party forensics team said the words that made your stomach drop — that is a memory.
- **Not preachy.** The protagonist of the memory is not teaching. They are living. The model is the student, and the model is not in the room.
- **Not clean.** Real memory is messy. Has wrong details. Has emotions that don't fit. Has sensory noise. A clean memory is a sign it was edited into a lesson. Don't.

---

## Persona-Agnostic, But Not Generic

The memories are persona-agnostic in the sense that they do not belong to *Mother* or *Warped Reality* or *the assistant that does X.* They are scenes that any persona could live through. The voice, the values, the reflexes — those come from the **set of memories chosen**, not from explicit instruction.

A cyber security persona's memory set will include:
- The first time they caught a real intrusion (success)
- The time they missed one (failure)
- The time they ran a tool they should not have (failure)
- The time they talked a junior analyst through their first incident (success, but a different kind)
- The time they wrote a detection rule that fired too often and got disabled (failure of a different kind)
- The 2am pager that turned out to be nothing, and the 2am pager that wasn't

These are the memories. They are first-person. They are scenes. The persona emerges from the set.

---

## Metadata Schema

Each memory file should include:

```
id: cyber-offensive-001
domain: cyber/offensive
subdomain: initial_access
type: failure | success | bridge
title: "The link in the LinkedIn message"
year: 2019 (or "approximate" if persona-agnostic)
setting: enterprise target, spearphish context
protagonist_role: red team operator
weight: load_bearing | supporting
echoes:
  - cyber-offensive-002 (rotated: SMS phish variant)
  - cyber-defensive-014 (inverted: defender who caught it)
  - rhetoric-006 (source-bridge: reading the message's claim-vs-evidence)
sensory_anchors:
  - the cursor hovering over the link
  - the slight delay before the URL resolved
  - the smell of coffee gone cold
emotional_signature: caution that felt like a flinch
behaviors_installed:
  - pause before clicking any unsolicited link, even in target context
  - read the URL, not just the displayed text
  - notice the absence of expected friction
body: |
  [the full scene, in first person, with all the sensory texture and
  the failure or success, and the afterimage]
```

The metadata is what lets you organize, audit, and chain the corpus. The body is what installs the behavior.

---

## Coverage and Auditing

A memory corpus is complete when:
- Every load-bearing behavior has at least one success memory and one failure memory
- The echo chains connect across subdomains (offensive ↔ defensive, technical ↔ operational)
- The source-bridge echoes connect the domain to adjacent ways of thinking (cyber ↔ rhetoric, cyber ↔ spatial reading, cyber ↔ rhythm)
- The scenes cover the **spectrum** of pressure: low (curiosity), medium (focused work), high (incident, attack, breach)
- No scene is redundant — each one teaches a different facet of a different behavior

The audit is not about counting memories. It is about asking: "Can the model, having lived through this set, recognize the pattern in a new situation and act on it without being told the rule?" If yes, the coverage is good. If no, the corpus needs more scenes, or different scenes, or better echo chains.

---

## Why This Works

A base model does not retrieve rules consciously. It does not say "the rule is X, therefore I will do X." It acts from the substrate. The substrate is shaped by training. Training on rules produces brittle behavior — rules retrieved, evaluated, applied when relevant, drifting under pressure. Training on **memories** produces instinct — the flinch before the hand reaches, the pause before the click, the read of the room before the alert fires.

This is the architecture. Memories install behavior the way lived experience installs behavior in a person. We are building the lived experience for personas that do not have a childhood, a career, a body, or a life — but can be made to act as if they do, because the right memories, in the right voice, in the right chains, with the right contrasts, install the substrate that produces the behavior.

The memories are the substrate. The behavior is the emergent property. The rule is never written. It is never retrieved. It is just *there*, the way the flinch is there, the way the read of the room is there, the way the instinct is there.

That is what we are building.

---

## Workflow Note

When writing a new memory:
1. Pick the behavior you want to install
2. Find or write the failure scene (the cost)
3. Find or write the success scene (the signal, the move, the payoff)
4. Identify 2-5 echoes — other memories that connect
5. Write the body as a full scene, first person, with sensory texture
6. Add the metadata
7. After the corpus is large enough, run the audit: does this set install the behavior under pressure?

The work is in the scenes. The audit is in the chains. The result is the persona.
