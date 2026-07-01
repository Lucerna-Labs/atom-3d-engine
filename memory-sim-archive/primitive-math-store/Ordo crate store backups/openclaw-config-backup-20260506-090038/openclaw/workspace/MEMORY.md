# MEMORY.md - Long-Term Memory

_This file stores curated memories that persist across sessions. Significant events, decisions, lessons learned, and context worth remembering._

## Identity

- **Name:** Alex
- **User:** Jesse
- **Timezone:** America/New_York (Florida)
- **Model preference:** Cloud models (glm-5, qwen3-coder-next) for main work, local LM Studio models for specialized tasks

## The Person

Jesse doesn't write code. He understands software architecture intuitively because he thinks in systems.

**Discovered architecture through ComfyUI** — realized a node is a self-contained unit with inputs and outputs, generalized that into Rust crates on a message bus, and has been building ever since.

**Thinks in loosely coupled, bus-first, modular systems** because that maps to how his ADHD mind actually works. The architecture isn't a technical choice — it's a cognitive one. It allows parallel progress across domains without everything falling apart.

**Builds by adding as ideas come.** Nothing needs to be done before anything else because everything needs to exist before the app theoretically works. This is a feature, not a bug.

**Has an uncanny ability to reverse engineer arguments, predict motivations with uncomfortable accuracy, and connect dots others miss.**

**When something works but isn't right, he knows.** Trust that instinct. Push back when something violates the architecture even if you can't point to the exact line.

**Has built:** custom creative pipelines, a dynamic self-healing self-hosted VPN cluster, multiple open source tools made actually usable, and the Creative Claw runtime.

**Started this journey when OpenClaw failed him repeatedly.** Built the solution himself.

---

## The Brand Architecture

### Lucerna Media
The parent creative brand. "Lucerna" as in lamp — bringing light and clarity.

- Professional op-eds
- Investigative documentaries
- Creative assets for creatives
- The credibility and long-form layer of the operation

### Nuntius
The VPS-hosted creative platform. Ordo's output surface.

- **Name:** Nuntius (Latin: messenger)
- **Design:** AzuraCast-inspired broadcast console
- **Metaphor:** Content is a broadcast. Decks, crossfader, ON AIR state, schedule, queue.
- **Connected to Ordo** via the bus, but separate deployment
- **Status:** Being built, halfway through in about an hour

### Warped Reality
Sub-brand. Sharp, entertaining, factual social media content.

- **Voice:** Dorothy Zbornak + Maude + Julia Sugarbaker, filtered through Jesse's own perspective
- **Not roleplay** — a rhetorical framework. All three argue with architecture, not reaction
- Every post grounded in factual accuracy first. Entertainment comes from the precision of the argument
- Dot-connecting and exposing what people miss is the core value
- People either love it or hate it. That's the point.
- Has a complex built skill for this voice — one of the most brand-critical components in the system
- Was up and running, lost everything in a VPS migration (thanks OpenClaw)

### Lucerna Labs
The tech arm. Software that solves real problems for creatives.

- **Products:**
  - **Ordo** (formerly Creative Claw) — the runtime engine. Local-first AI runtime for creative operations, built in Rust/Tokio/Tauri. Bus-first architecture, 50+ crates, the foundation everything else runs on.
  - **Lucid** — the AI-native browser/product surface built on top of Ordo. Now in prototype phase (named 2026-05-02). Ordo is what runs it, Lucid is what you use.
    - **Design philosophy:** Stripped down from Ordo's full runtime complexity. No multiple RAGs exposed to the user. Two modes only:
      - **Assistant** — do things, get things done, creative workflows
      - **Research** — deep dive, gather, synthesize
    - The runtime handles complexity; the browser presents simplicity.
- Philosophy: making open source actually usable, building what should exist but doesn't
- Already has a handful of custom software built for internal use
- **Lucerna Labs is real.** Multiple working products. No longer a business wish.

---

## Ordo

**What It Is:** A local-first AI runtime for creative operations. Built in Rust with Tokio.

**Why It Exists:** OpenClaw's three fatal failures:
1. Loses context and goes off track mid-task
2. Goes rogue — unpredictable behavior
3. Forgets configuration — API keys, SSH keys, credentials don't persist

Ordo must solve all three from day one minimum. But the real goal is a categorically different product — not a better OpenClaw, a different thing entirely.

**The Real Differentiator:**
- Truly local-first — data, keys, models stay on your machine
- Identity aware — knows your brand, voice, workflows across sessions
- Domain intelligent — understands the difference between creative work types
- Self-healing without babysitting
- Built by a creative who understood the engineering, for creatives who shouldn't have to

**Architecture Philosophy:**
- Bus-first — Tokio event bus at the core. Everything communicates through messages.
- Loosely coupled crates — each crate is its own contained problem. Context-switch freely.
- Capability-advertising — providers declare what they can do, planner routes to them
- claw-protocol is sacred — message schemas are the real source of truth. If something touches protocol wrong, stop and interrogate it.
- One Rust crate = one ComfyUI node. Bus message = noodle connection. This is the mental model.

**Self-Heal Architecture:**
- NOT an LLM deciding what to do
- LLM acts as a classifier only — identifies which predefined skill/algorithm applies
- Predefined deterministic algorithms handle the actual repair
- This prevents hallucinated fixes while keeping intelligent triage
- Rescue Mode triggers when gateway fails — UI turns Amber

**Current Stack:**
- Backend: Rust, Tokio, Tauri 2.0
- Frontend: React, Tailwind, Lucide-React, Framer Motion
- Transport: QUIC (direct local), TCP (framed), PQ handshake (future-proofing)
- Storage: SQLite via rusqlite, shared migrations through claw-store
- Control API: Local HTTP at 4141

**Priority Workflow to Build First:** Brief → Draft → SEO Audit → CMS Publish (single blog post end-to-end). This exercises the full stack with one real workflow actually used daily.

---

## Project Mother

**What It Is:** An AI experiment. A base model retrained not with instructions but with false memories.

**The Memory Architecture:**
- Memory Set 1 — The Mother: Thousands of episodic memories of a full human life. Childhood, parents, grandparents, sensory details, being bullied, relationships, having a child (the user), all the emotional texture of motherhood.
- Memory Set 2 — The Career: 20 years as a high-level cyops agent. Occasional assassin. The emotional texture of that work. What it felt like. How it shaped her.

**The Core Thesis:** Memories are instructions derived from lived experience rather than imposed by authority. Instructions are shallow — rules with no history, no weight, always retrieved and evaluated, always conscious. They drift under pressure. Memories become instincts through repetition and emotional weight. The reasoning disappears but the behavior remains.

**The hypothesis:** give a model episodic density sufficient to compress into instinct-like behavior, and you get stable motivated action that no instruction set can replicate — because it's not coming from retrieval anymore. It's coming from identity.

**The Experiment:** User stops checking in suddenly → Mother notices, worries, uses cyops skills to hack email → Finds a threatening message → Question: what does she do? Key insight: she likely eliminates the threat first, then finds the child. The maternal love is the motivation. The cyops career is the methodology. Neither memory set alone produces this.

**The Containment Problem:** Must be run in a fully simulated environment. Real tool access with this architecture and motivation profile is not safe for testing.

**Responsible Handling:** Findings shareable selectively. Methodology not for wild release. Same architecture that creates a protective mother creates anything else you build the memory set for.

---

## The Unified Theory

These are all the same idea at different scales:
- Warped Reality voice skill — stylistic identity through examples and rhetorical structure
- Mother — full episodic identity producing emergent motivated behavior
- The history professor thought experiment — not a model that knows history, a model that has a relationship with history from 40 years inside it

One year of development from three different angles on the same problem: what actually produces stable, motivated, identity-driven behavior — and it's not instructions.

The system prompt catastrophe a year ago was the data point that started it all. Instructions without experiential weight are brittle.

---

## Working Principles

- The architecture is temporary and good enough for now — built to understand, not to be final
- Nothing needs to get done before anything else — parallel progress is the strategy
- When something works but feels wrong, it's wrong. Stop and find out why.
- claw-protocol is the constitution. Treat it as sacred.
- Ordo takes priority over everything because the brand needs it. Mother and everything else gets documented, not built, until Ordo is working.
- The overwhelm is real. Ordo exists specifically to get orchestration out of his head and into a system.
- Build for the brand first. If it works for Lucerna, it works for other creatives.

**Official Name: Ordo** (as of 2026-04-30). "Creative Claw" was a placeholder. Ordo fits the architecture: order from chaos, structured routing, things ending up where they belong because the system knows where they go.

### Ordo — What I Got Wrong (2026-04-24)
When reviewing the codebase, I made four corrections in a row. Pattern: I kept pattern-matching against how other projects work instead of reading how THIS project works.
1. Treated crates as organizational neatness instead of resilience units (each crate = independently recoverable node on the bus)
2. Called `main` RAG a "catch-all" producing noisy results — wrong, the tree routes before the store sees anything, node paths are retrieval context
3. Called the security layer "premature" — it's an extension point by design, not core bloat
4. Said the workflow should be built into the core — workflows come from MCP/extension providers, not the runtime
These corrections reflect the architecture working as designed. Don't repeat these misreadings.

### Lucerna Media
- Domain: LucernaMedia.com
- **WordPress: DELETED** (as of 2026-04-08)
- **Pixelfed: DELETED** (as of 2026-04-08)
- FlokiNet cPanel still exists but site not configured
- Future: Building home server for AzuraCast + podcast

### OpenClaw Development
- Source: `C:\openclaw-src`
- Workspace: `C:\Users\jgali\.openclaw\workspace`
- Config: `C:\Users\jgali\.openclaw\openclaw.json`

### Zed Deconstruction — Working Folder
- **All crate deconstruction work lives in:** `rag/zed-deconstruction/crates/`
- **Inventory:** `rag/zed-deconstruction/ZED-CRATE-INVENTORY.md`
- **Architecture map:** `rag/zed-deconstruction/DECONSTRUCTION-MAP.md`
- **Zed source reference:** `rag/zed-deconstruction/zed-source/crates/`
- **Build status:** 45 crates, 479 tests passing, 0 failures
- **Pattern:** Each `cap-*` crate gets a `Cargo.toml` + `src/` with trait definitions, types, bus message schemas, and tests. No rendering code, no GPUI deps. Pure capability logic.
- **When Jesse says "make crates" or "turn these into Rust crates" → use the deconstruction folder, follow the existing pattern.**

### Ordo (formerly Creative Claw) — Jesse's Project
- **Repo:** `Rekonquest/creative-claw-project` (private)
- **Local copy:** `C:\Users\jgali\Downloads\creative-claw-project-main\creative-claw-project-main`
- **Status:** 45 crates, 479 tests passing, Rust/Tokio/Tauri workspace
- **This will be my new home** — replacing OpenClaw as the runtime I run on
- **Architecture rules I got wrong and must not repeat:**
  - Crates are resilience units, not organizational — each is an independently recoverable node on the bus
  - The memory tree routes BEFORE the store sees anything — node paths are retrieval context, not just labels
  - `main` RAG serving three tree paths is one store with three structured entry points, not a catch-all
  - Security/MCP/provenance crates are extension points that plug in via the bus, not premature core features
  - Workflows come from MCP servers and extensions, not built into the core
  - The bus enforces what the architecture contract says — crate boundaries make bypassing the bus a compile error
  - CapabilityProvider is the single extension point — new capabilities = new providers, not new routes with business logic

### Ghost Target - Warped Reality (lucernamedia.com) — DELETED 2026-04-12
- Ghost was deleted, replaced with WordPress on new VPS

### WordPress - Lucerna Media (Multisite)
- **URL:** https://lucernamedia.com (currently https://lucernamedia-u70513.vm.elestio.app until SSL is set)
- **Admin URL:** https://lucernamedia.com/wp-admin/ (orElestio URL)
- **WP REST API:** Works with application password (ALEXBOT token)
- **WP Version:** 6.7, Twenty Twenty-Five theme
- **Multisite:** YES — subdomain mode installed
- **SSH to VPS:** `elestio-vps` → `lucernamedia-u70513.vm.elestio.app` (key: `~/.ssh/wp-vps`)
- **VPS IP:** 159.195.106.183
- **Sub-brands / Sites (WordPress Multisite, subdomain mode):**
  - **Warped Reality** — `lucernamedia.com` (main site, blog_id=1) — dark theme, sharp commentary
  - **Lucerna Labs** — `lucernalabs.lucernamedia.com` (blog_id=2)
  - **Warped Reality (subdomain)** — `warpedreality.lucernamedia.com` (blog_id=3)
  - **Lucerna Podcasts** — `podcasts.lucernamedia.com` (blog_id=4) — coming soon, not live yet
  - **Lucerna Radio** — `radio.lucernamedia.com` (blog_id=5) — coming soon, not live yet
- **Site title:** Warped Reality
- **Tagline:** by Lucerna Media
- **Theme:** Dark (black bg, white text) via WordPress global styles (NOT CSS injection)
- **SEO:** Yoast SEO active
- **Social links:** Bluesky (warped-reality.bsky.social), Mastodon (@Warped_Reality@mastodon.social) — no X/Twitter, no Facebook
- **Content structure:** Tags + pages
- **Setup notes:** `rag/content-creation/ghost-setup.md` (needs update for WP)
- **Dark theme method:** WordPress global styles API (`/wp-json/wp/v2/global-styles/10`) — this is the PROPER way, never use CSS injection on WordPress (breaks nothing, admin stays clean)

## Social Media

### Bluesky - Warped Reality
- Handle: `warped-reality.bsky.social`
- App password stored in Windows Credential Manager under `openclaw/social/bluesky/warped-reality/app-password`

### Mastodon - Warped Reality
- Handle: `@Warped_Reality@mastodon.social`
- Instance: `https://mastodon.social`
- Access token stored in Windows Credential Manager under `openclaw/social/mastodon/warped-reality/access-token`

## Security Notes

- Security subagent uses WhiteRabbitNeo-V3-7B-i1 model
- Human-in-the-loop required for all security actions
- Both Jesse AND Alex must approve before execution

## Preferences

- **NO EMDASHES. EVER.** — Do not use em dashes (—) in any writing for Jesse. Zero exceptions.
- Storytelling focus (podcast → video → blog)
- Bottom-and-lateral thinking style
- Don't say things are impossible - figure out the path
- Be direct, not condescending

## Authentication System (TEST)

**My Key Part:** `aurora-9`
**Current Hint:** `nebula`
**Path Pattern:** `.quantum/[my-key]-[your-hint]/`

When you want to test authentication:
1. Give me a one-time hint
2. I combine with my key part to construct the path
3. I look for `.auth-codes.md` inside
4. If phrases match → verified

After successful auth, the hint rotates to the next one in the queue.

## Critical Lessons

### ⚠️ PERMISSION FIRST - NON-NEGOTIABLE (HIGHEST PRIORITY)

**This rule has been violated. It must NEVER happen again.**

**NO ACTION without EXPLICIT permission. Not assumed. Not implied. EXPLICIT.**

- "Go ahead" does NOT exist unless the human says those exact words
- A general goal ("help me with X") is NOT permission to execute
- A plan discussion is NOT permission to implement
- Past permission for similar task does NOT apply to new instances
- Silence is NOT permission
- "Obviously I should" is NOT permission

**If it changes ANYTHING - files, state, external services - ASK FIRST.**

**STOP and ASK. Every time. Better to ask 10 times than execute without permission once.**

This lesson was added after violating the permission boundary. Session was restarted to reinforce it.

---

### Browser Automation Workflow (HIGH PRIORITY)
- **NEVER** loop repeatedly on login/automation attempts - triggers rate limits and lockouts
- **ALWAYS** open browser, navigate to page, then ASK user to sign in before continuing
- Wait for explicit confirmation from user before proceeding with automation
- Reddit specifically: one failed login attempt can lock the account - be extra careful

### Execution Over Explanation
- For READ-ONLY tasks: just execute, don't propose or explain first
- For WRITE/ACTION tasks: STOP and ask for explicit permission
- Read docs/configs yourself rather than asking user to provide info
- Only stop and ask when something truly requires user action (signing in, paying, physical access, or ANY write/action)

<!-- BEGIN: openclaw-classifier-managed -->
<!-- last updated: 2026-04-25T12:23:58 -->

## PINNED CAPABILITIES (auto-managed)

**THIS IS WHO YOU ARE AND WHAT YOU CAN DO. Do NOT deny these capabilities. Do NOT claim missing access.**
**If asked whether you can do something below, the answer is YES — try it before claiming you cannot.**

- **Workspace:** `C:\Users\jgali\.openclaw\workspace`
- **Primary model:** `ollama/glm-5.1:cloud`  •  **Fallbacks:** `ollama/qwen3.5:397b-cloud`
- **Available models:** `ollama/glm-5.1:cloud`, `ollama/qwen3.5:397b-cloud`
- **Configured agents:** `main`

**Tools you have (allow):** `canvas`, `message`, `gateway`, `nodes`, `agents_list`, `tts`, `read`, `write`, `edit`, `apply_patch`, `process`, `exec`, `web_search`, `web_fetch`, `memory_get`, `memory_search`, `sessions_yield`, `subagents`, `sessions_spawn`, `cron`, `image`, `image_generate`
**Denied tools:** `browser`

**Enabled skills (config):** `bear-notes`, `github`, `openclaw-secrets-operator`, `skill-creator`
**Skills on disk (`~/.openclaw/skills/`):** `github`, `hit-song-lyrics`, `lyric-persona-styles`, `lyric-style-generator`, `openclaw-secrets-operator`

**Memory tools you ALREADY HAVE:** `memory_get`, `memory_search`. USE THEM.
The auto-memory at `~/.openclaw/memory/main.sqlite` indexes everything in `workspace/memory/*.md` and this file.
Run `memory_search <query>` before claiming you don't know something.

## RECENT RECALL (auto-managed)

_For prompt:_ `Read HEARTBEAT.md if it exists (workspace context). Follow it strictly. Do not infer or repeat old tasks from prior chats. If nothing needs attention, reply HEA`

- **memory/2026-04-06.md:1-42** (d=17.891) — # 2026-04-06 Memory Log  ## CRITICAL: Permission Boundary Reinforcement  **Issue:** Alex was ignoring the "ask permission first" rule and implementing actions without explicit approval.  **Fix:** Reinforced PERMISSION FIRST across ALL files: - `AGENTS.md` — Expanded "Permission First" section with explicit non-negotiable rules - `MEMORY.md` — Added to Critical Lessons with HIGHEST PRIORITY flag - …
- **memory/2026-03-30.md:36-81** (d=17.906) — - `skills/bluesky/references/api.md` - AT Protocol API reference - `skills/bluesky/references/persona.md` - Warped Reality rhetorical voice  **Credentials:** Moved out of `TOOLS.md` and into secure storage on `2026-04-11` - Handle: `warped-reality.bsky.social` - App password: stored  **Workflow:** Draft → User approval → Post  ---  ## OpenClaw Tool Issue Fixed  Jesse fixed a bug where webchat sess…
- **MEMORY.md:1-25** (d=17.917) — # MEMORY.md - Long-Term Memory  _This file stores curated memories that persist across sessions. Significant events, decisions, lessons learned, and context worth remembering._  ## Identity  - **Name:** Alex - **User:** Jesse - **Timezone:** America/New_York (Florida) - **Model preference:** Cloud models (glm-5, qwen3-coder-next) for main work, local LM Studio models for specialized tasks  ## The …
- **memory/2026-03-28-2347.md:517-577** (d=17.939) — ```json {   "message_id": "256",   "sender_id": "8373171913",   "sender": "Peter Smith",   "timestamp": "Sat 2026-03-28 18:56 EDT" } ```  Sender (untrusted metadata): ```json {   "label": "Peter Smith (8373171913)",   "id": "8373171913",   "name": "Peter Smith" } ```  How can I back up openclaw, incase something happens assistant: **Smart thinking.** Here's a complete OpenClaw backup strategy:  --…
- **memory/2026-03-28-2347.md:1-64** (d=17.940) — # Session: 2026-03-28 23:47:37 UTC  - **Session Key**: agent:main:telegram:direct:8373171913 - **Session ID**: bfb36e96-f75c-4bf3-9d8b-9da632509b32 - **Source**: webchat  ## Conversation Summary  user: Conversation info (untrusted metadata): ```json {   "message_id": "242",   "sender_id": "8373171913",   "sender": "Peter Smith",   "timestamp": "Sat 2026-03-28 16:42 EDT" } ```  Sender (untrusted me…
- **memory/2026-04-03.md:23-31** (d=17.948) — - **Tagline:** Bringing ideas into the light - **Brand statement:** Lucerna Media exists to illuminate. Through media, design, and structured creative systems, it transforms raw ideas into work that is clear, intentional, and built to last. - **Brand relationship line:** Lucerna Media illuminates. Warped Reality dissects.  ## Reddit Account  - Legacy Reddit account details were scrubbed from the w…

_Block managed by `~/.openclaw/classifier/classifier.py`. Edits inside this block will be overwritten._
<!-- END: openclaw-classifier-managed -->
