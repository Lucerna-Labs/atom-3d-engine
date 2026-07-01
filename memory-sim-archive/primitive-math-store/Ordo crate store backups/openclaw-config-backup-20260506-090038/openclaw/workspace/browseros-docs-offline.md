# BrowserOS Documentation (Offline Copy)

> Scraped from https://docs.browseros.com/ on April 30, 2026
> Complete offline reference document

---

# Table of Contents

1. [Getting Started](#getting-started)
2. [Changelog](#changelog)
3. [Comparisons](#comparisons)
   - [Chrome DevTools MCP](#chrome-devtools-mcp)
   - [Claude Cowork](#claude-cowork)
   - [OpenClaw](#openclaw)
4. [Contributing](#contributing)
5. [Features](#features)
   - [Ad Blocking](#ad-blocking)
   - [Bring Your Own LLM](#bring-your-own-llm)
   - [ChatGPT Pro / Plus OAuth](#chatgpt-pro--plus-oauth)
   - [Connect Apps (MCP)](#connect-apps-mcp)
   - [Cowork](#cowork)
   - [GitHub Copilot OAuth](#github-copilot-oauth)
   - [LLM Chat & Hub](#llm-chat--hub)
   - [Local Models](#local-models)
   - [Memory](#memory)
   - [Qwen Code OAuth](#qwen-code-oauth)
   - [Scheduled Tasks](#scheduled-tasks)
   - [Skills](#skills)
   - [Smart Nudges](#smart-nudges)
   - [SOUL.md](#soulmd)
   - [Sync to Cloud](#sync-to-cloud)
   - [Use with Claude Code / MCP Clients](#use-with-claude-code--mcp-clients)
   - [Vertical Tabs](#vertical-tabs)
   - [Workflows](#workflows)
6. [Integrations](#integrations)
   - [n8n](#n8n-integration)
7. [Troubleshooting](#troubleshooting)
   - [Connection Issues (Agent Crashed)](#has-agent-crashed)
8. [Updates](#updates)
   - [macOS](#update-on-macos)
   - [Windows](#update-on-windows)
   - [Linux](#update-on-linux)

---

# Getting Started

Set up BrowserOS in 2 minutes.

## Step 1: Import from Chrome

Bring your bookmarks, history, and passwords from Chrome.

1. Go to `chrome://settings/importData`
2. Select **Google Chrome** and click **Import**
3. Choose **Always allow** when prompted

This imports everything in one click - bookmarks, passwords, history, and extensions.

## Step 2: Set up your AI

BrowserOS includes a default AI model with limited daily usage. For the best experience, add your own.

Quick option: Get a free Gemini API key from aistudio.google.com - 20 requests per minute at no cost.

See [Bring Your Own LLM](#bring-your-own-llm) for full setup details.

## Step 3: Try it out

Open any webpage and click the **Assistant** button in the toolbar.

- **Chat Mode:** Ask questions about the page
- **Agent Mode:** Describe a task and watch it execute

For Agent Mode, use Claude Opus 4.5 or Sonnet 4.5. Local models work great for Chat but aren't powerful enough for agents yet.

## You're all set!

Block ads with uBlock Origin - Chrome dropped support for the full uBlock Origin extension, but BrowserOS brought it back. Install it from the Chrome Web Store.

---

# Changelog

## v0.42.0 (March 9, 2026)

- **SOUL.md** - Your assistant now has a soul. Tell it how you like to communicate, set boundaries, shape its personality - and it adapts on its own over time.
- **Vertical tabs** - BrowserOS now ships with vertical tabs by default. More screen space, better tab management. Prefer horizontal? Switch back in settings.
- **Long-term memory** - Your assistant remembers you across every conversation. All stored locally on your machine.
- **Chromium 146** - Updated to the latest Chromium release

## v0.41.0 (March 4, 2026)

- **New agent (v3)** - Completely redone from scratch. 2x faster, 2-3x better performance
- **Tools major upgrade** - ~20 new tools (54 total) including file upload, save as PDF, background windows. Connection with third-party coding agents (Claude Code, Codex, etc.) is much better
- **General fixes** - Better agent installation, bug fixes, smoother experience
- **Linux Debian packaging** - Fixed remaining Debian packaging issues

## v0.40.1 (February 16, 2026)

- **Chromium 145** - Upgraded to latest Chromium
- **Login session import improvements** - More reliable imports
- **Stability & reliability** - General improvements

## v0.39.0 (February 3, 2026)

- **Sync** - Save browser configuration, agent history, and scheduled tasks across machines
- **App Connector redesign** - Easier MCP app connecting UI
- **MCP port stability** - More reliable connections across restarts
- **Keyboard shortcuts** - Updated to avoid European keyboard conflicts

## v0.38.0 (January 28, 2026)

- **MCP port fix on Windows & Linux** - Port stays consistent across restarts
- **Settings fix** - Fixed chrome.browser.settings not working correctly
- **Improved agent** - More reliable and performant

## v0.37.0 (January 21, 2026)

- **Workflows** - Build reliable, repeatable browser automations with a visual graph builder
- **Cowork** - Combine browser automation with local file operations in one task

## v0.36.3 (January 15, 2026)

- **Agent history** - Agent conversations saved automatically. View and resume from Assistant panel

## v0.36.2 (January 10, 2026)

- **MCP server disconnect fix** - Fixed port handling causing connections to drop

## v0.36.0 (January 8, 2026)

- **Agent personalization** - Add your own prompts to personalize the agent
- **Toolbar customization** - Hide the Hub chat and labels
- **MCP server port stability** - Port stays consistent through browser restarts
- **Fixed agent install/update issues**

## v0.35.0 (December 25, 2025)

- **Agent stability fixes** - Much more reliable agent loop
- **Gemini 3 support** - Through OpenRouter and Google adapters
- **Better error surfacing** - Clearer error messages

## v0.34.0 (December 20, 2025)

- **Third-party MCP server support** - Connect external MCP servers like Google Calendar, Notion, Gmail, and more
- **Gemini 3 support** - Pro and Flash models
- **Windows icon fix**
- **Agent & UI improvements**

## v0.33.0 (December 18, 2025)

- **OpenAI-compatible provider support** - Connect any OpenAI-compatible API endpoint
- **Multi-window & multi-profile agent support**
- **MCP server reliability** - Fixed connection drops
- **Agent reliability improvements**

## v0.32.0 (December 12, 2025)

Complete revamp:
- **New Agent** - Completely rebuilt
- **Agent Per Tab** - Run multiple agents simultaneously
- **Polished UI** - Fresh, cleaner interface
- **Manifest V2 Support** - Ad blockers like uBlock Origin work again
- **Native Split View** - Work on two things side by side
- **Chromium 142** - Updated to Chromium 142.0.7444.49
- **Azure & AWS Bedrock** - Native support

Breaking changes: Old agent stops working, LLM provider settings won't carry over.

## v0.30.0 (November 14, 2025)

- Better text extraction, MCP server stability, UI cleanup, third-party MCP fixes

---

# Comparisons

## Chrome DevTools MCP

> How BrowserOS MCP compares to Chrome DevTools MCP for browser automation

Chrome DevTools MCP is an MCP server that gives AI agents limited access to Chrome through the Chrome DevTools Protocol (CDP). It is useful for taking snapshots and performing basic interactions, but it has significant limitations compared to BrowserOS's built-in MCP server.

### At a Glance

| | **BrowserOS MCP** | **Chrome DevTools MCP** |
|---|---|---|
| **Setup** | Copy URL from settings | Install Node.js, configure Chrome debug profile, run server |
| **Browser tools** | 53 (tabs, clicks, forms, navigation, screenshots, bookmarks, history, tab groups, windows) | ~10 (snapshots, click, type, navigate) |
| **App integrations** | 40+ built-in (Gmail, Slack, GitHub, Notion, etc.) | None |
| **Authentication** | OAuth for external services | None - can only access pages you manually navigate to |
| **Works with your session** | Yes - uses your browser with all your cookies and logins | No - opens a separate debug Chrome instance |
| **MCP transport** | Native HTTP (streamable) | SSE (requires mcp-remote for HTTP clients) |
| **Cost** | Free (open source) | Free (open source) |

### Where BrowserOS MCP Wins

**53 browser automation tools vs ~10.** BrowserOS exposes navigation, tab management, clicks, form filling, screenshots, bookmarks, history search, tab groups, window management, file downloads, PDF saves, and more. Chrome DevTools MCP offers basic snapshot + click + type.

**40+ app integrations built in.** Gmail, Slack, GitHub, Notion, Google Calendar, Jira, Linear, and 30+ more - all through the same MCP connection with OAuth. Chrome DevTools MCP has no integrations.

**Works with your logged-in session.** BrowserOS MCP operates on the browser you are already using, with all your cookies, logins, and open tabs. Chrome DevTools MCP opens a separate debug Chrome instance with a clean profile - no cookies, no logins, no access to your existing tabs.

**Zero configuration.** Copy the MCP URL from BrowserOS settings and paste it into Claude Code. Done. Chrome DevTools MCP requires installing Node.js, launching Chrome with --remote-debugging-port, starting the MCP server, and configuring the connection.

**Native HTTP transport.** BrowserOS MCP uses HTTP streamable transport, which works directly with Claude Code, Gemini CLI, and Codex. Chrome DevTools MCP uses SSE, which requires an mcp-remote wrapper for HTTP-based clients like Claude Code.

### Where Chrome DevTools MCP Wins

**Works with any Chrome-based browser.** If you don't want to switch to BrowserOS, Chrome DevTools MCP is a way to get basic browser automation in your existing browser.

**Lighter weight.** Just a Node.js server. No custom browser build required.

### Detailed Tool Comparison

| Category | BrowserOS MCP | Chrome DevTools MCP |
|---|---|---|
| Navigation & Tabs | 8 tools (navigate, new tab, close, move, hidden tabs) | 1 tool (navigate) |
| Content & Observation | 8 tools (snapshot, DOM, screenshots, links, JS eval) | 2 tools (snapshot, screenshot) |
| Interaction & Input | 14 tools (click, fill, select, drag, scroll, upload, dialogs) | 3 tools (click, type, hover) |
| File & Export | 3 tools (PDF, screenshot save, download) | 0 tools |
| Window Management | 5 tools (list, create, close, activate, hidden) | 0 tools |
| Tab Groups | 5 tools (list, group, update, ungroup, close) | 0 tools |
| Bookmarks | 6 tools (list, create, remove, update, move, search) | 0 tools |
| History | 4 tools (search, recent, delete URL, delete range) | 0 tools |
| External Apps | 40+ integrations | 0 integrations |

### Setup Comparison

**BrowserOS MCP:**
1. Open BrowserOS
2. Go to Settings > BrowserOS as MCP
3. Copy the URL
4. Run: `claude mcp add --transport http browseros http://127.0.0.1:9239/mcp`

**Chrome DevTools MCP:**
1. Install Node.js 18+
2. Clone the repo and install dependencies
3. Launch Chrome with `--remote-debugging-port=9222`
4. Start the MCP server
5. Configure mcp-remote wrapper for Claude Code
6. Run: `claude mcp add browseros -- npx mcp-remote http://localhost:13042/sse`

### When to Use Which

Choose BrowserOS MCP if you want full browser control, app integrations, and zero setup. Choose Chrome DevTools MCP if you need basic automation in your existing Chrome and don't want to install a new browser.

---

## Claude Cowork

> How BrowserOS compares to Claude Cowork (Anthropic's desktop app with computer use)

Claude Cowork is Anthropic's desktop application that lets Claude control your computer through screen observation and input simulation. It is a powerful tool for tasks that require desktop-level automation.

BrowserOS takes a different approach. Instead of giving an AI agent control of your entire computer, it gives the agent control of your browser through structured tools - and only your browser.

### At a Glance

| | **BrowserOS** | **Claude Cowork** |
|---|---|---|
| **What it controls** | Your browser only | Your entire computer |
| **Control method** | Structured tools (click by element, fill by selector) | Screen observation + mouse/keyboard simulation |
| **Accuracy** | High (element-level targeting) | Moderate (pixel-level targeting, can miss) |
| **Speed** | Fast (direct API calls) | Slow (must observe screen between actions) |
| **Setup** | Download and open | Install Anthropic desktop app |
| **App integrations** | 40+ built-in | None (uses desktop apps directly) |
| **Cost** | Free + your LLM API keys | Requires Claude Pro/Max subscription |
| **Open source** | Yes (AGPL-3.0) | No (proprietary) |

### Where BrowserOS Wins

**Structured tools beat screen observation.** BrowserOS clicks elements by their accessibility ID, fills forms by selector, and reads page content as structured data. Claude Cowork looks at pixels and simulates mouse movements. Element-level targeting is faster, more reliable, and doesn't break when the UI shifts by a pixel.

**40+ app integrations.** BrowserOS connects directly to Gmail, Slack, GitHub, Notion, and 30+ more through OAuth. Claude Cowork uses desktop apps by clicking around them.

**Free and open source.** BrowserOS is AGPL-3.0. Use your own API keys. Claude Cowork requires a Claude Pro or Max subscription.

**Much faster.** BrowserOS's structured tools execute instantly. Claude Cowork must take a screenshot, analyze it, decide what to do, simulate input, then take another screenshot. Each step takes seconds.

### Where Claude Cowork Wins

**Controls your entire computer.** BrowserOS is browser-only. Claude Cowork can open desktop apps, manage files in Finder/Explorer, use terminal applications, and do anything a human can do at the computer.

**No web-only limitation.** If your workflow involves desktop apps (Excel, VS Code, Figma desktop), Claude Cowork can interact with them directly.

### When to Use Which

Choose BrowserOS for web-based tasks. Choose Claude Cowork for desktop automation. Or use both - BrowserOS for web, Claude Cowork for everything else.

---

## OpenClaw

> How BrowserOS compares to OpenClaw for everyday AI assistance

OpenClaw is an open-source personal AI assistant that runs on your machine and connects through messaging apps like WhatsApp, Telegram, Slack, and Discord. It is a powerful tool for technical users who want a self-hosted, always-on AI agent.

BrowserOS takes a different approach. Instead of running a background server that you message through chat apps, BrowserOS puts the AI assistant directly inside your browser, where most of your work already happens. No terminal setup, no daemon management, no Node.js required.

### At a Glance

| | **BrowserOS** | **OpenClaw** |
|---|---|---|
| **What it is** | AI-powered browser with built-in assistant | Self-hosted AI agent you message through chat apps |
| **Setup** | Download and open | Install via npm, run onboarding wizard, configure daemon |
| **Technical skill needed** | None | Comfortable with terminal and Node.js |
| **Interface** | Built into your browser | WhatsApp, Telegram, Slack, Discord, iMessage, and 15+ more |
| **Browser automation** | 53 tools (clicks, forms, navigation, screenshots, tabs, bookmarks, history) | Chrome via CDP (snapshots and actions) |
| **App integrations** | 40+ built-in (OAuth or API key) | Skills-based (community-built, self-installable) |
| **Memory** | Two-tier: permanent core facts + 30-day daily notes | Persistent memory across conversations |
| **Personality** | SOUL.md (inspired by OpenClaw's original concept) | SOUL.md (originated the concept) |
| **LLM support** | 11+ providers including local models | Multiple providers with failover routing |
| **Runs on** | macOS, Windows, Linux | macOS, Windows, Linux (+ iOS/Android companion apps) |
| **Authentication** | OAuth or API key depending on the service | API keys, OAuth, pairing codes per channel |
| **Open source** | Yes (AGPL-3.0) | Yes (MIT) |

### Where BrowserOS Shines

**No technical setup required.** OpenClaw requires Node.js 22+, npm installation, a terminal-based onboarding wizard, daemon configuration, and channel pairing. BrowserOS is a browser. Download it, open it, start talking.

**Browser automation built in.** BrowserOS gives the assistant full control of your browser with 53 tools. OpenClaw has browser automation through a dedicated Chrome instance with CDP, but it runs as a separate process rather than being integrated into the browser you are already using.

**40+ app integrations built in.** BrowserOS connects to Gmail, Google Calendar, Slack, Notion, GitHub, Linear, Jira, Figma, Salesforce, Stripe, and 30+ more out of the box. OpenClaw uses a skills system where integrations are community-built plugins.

**Works where you already are.** Most of your work happens in a browser. BrowserOS puts the assistant right there. No context-switching.

### Where OpenClaw Shines

**Messaging app access.** OpenClaw connects to 20+ messaging platforms. Message your assistant from your phone or any chat app.

**Always-on background agent.** OpenClaw runs as a daemon, processing tasks even when you are not actively chatting. It supports cron jobs, webhooks, and Gmail Pub/Sub for automated triggers.

**Mobile companion apps.** iOS and Android apps with camera access, voice input, screen recording, and device-level actions.

**Agent-to-agent communication.** Multiple specialized agents can discover each other and collaborate.

**Self-modifying skills.** OpenClaw agents can write and install their own skills during a conversation.

### Feature Comparison

**App Integrations:**

| Service | BrowserOS | OpenClaw |
|---|---|---|
| Gmail | Built-in (OAuth) | Skill + API setup |
| Google Calendar | Built-in (OAuth) | Skill + API setup |
| Slack | Built-in (OAuth) | Built-in channel |
| Discord | Built-in (OAuth) | Built-in channel |
| Notion | Built-in (OAuth) | Skill |
| GitHub | Built-in (OAuth) | Skill |
| Linear | Built-in (OAuth or API key) | Skill |
| Jira | Built-in (OAuth) | Skill |
| Figma | Built-in (OAuth) | Skill |
| Salesforce | Built-in (OAuth) | Skill |
| Stripe | Built-in (API key) | Skill |
| WhatsApp | Built-in (OAuth) | Built-in channel |
| Shopify | Built-in (OAuth or API key) | Community skill |
| Total integrations | 40+ built-in | 50+ via skills |

**Memory and Personality:**

| Feature | BrowserOS | OpenClaw |
|---|---|---|
| Persistent memory | Core facts (permanent) + daily notes (30 days) | Persistent across sessions |
| Memory location | Local files on your machine | Local files on your machine |
| Personality system | SOUL.md (inspired by OpenClaw) | SOUL.md (originated the concept) |
| Memory search | Fuzzy search across all memories | Context-based recall |

**Setup and Maintenance:**

| | BrowserOS | OpenClaw |
|---|---|---|
| Installation | Download browser | npm install -g openclaw, run onboarding wizard |
| Runtime | Open the browser | Daemon process (launchd/systemd) |
| Updates | Auto-update | openclaw update --channel stable |
| Troubleshooting | Built-in | openclaw doctor CLI tool |
| Node.js required | No | Yes (v22+) |
| Terminal required | No | Yes |

### Who Should Use What

Choose BrowserOS if you want an AI assistant without any technical setup, do most of your work in a browser, need browser automation, and want 40+ app integrations with one click.

Choose OpenClaw if you want to message your AI from WhatsApp, Telegram, or Signal, need an always-on agent that runs 24/7 as a background service, are comfortable with terminal-based setup, want mobile companion apps, or need agents that can write their own extensions.

### Using Both Together

BrowserOS and OpenClaw are not mutually exclusive. Some users run OpenClaw as their always-on mobile assistant (accessible through WhatsApp or Telegram) while using BrowserOS as their desktop browser for work that involves web apps, browser automation, and visual tasks. The two tools complement each other rather than compete directly.

---

# Contributing

BrowserOS has two main parts you can contribute to:

- **Agent** - The AI features, UI, and browser automation (TypeScript/React)
- **Browser** - The custom Chromium build (C++/Python)

Most contributors work on the Agent since it's much easier to set up.

## Quick Links

- GitHub Repository: https://github.com/BrowserOS-ai/BrowserOS
- Discord Community: https://discord.gg/YKwjt5vuKr
- Report Issues: https://github.com/BrowserOS-ai/BrowserOS/issues

## Path 1: Agent Development

The Agent is a monorepo with 3 components:

| Component | Path | What it does |
|---|---|---|
| **Agent UI** | `apps/agent` | Chrome extension - chat interface, settings, side panel |
| **Server** | `apps/server` | Bun server - agent loop, MCP tools, API endpoints |
| **Controller** | `apps/controller-ext` | Chrome extension - bridges chrome.* APIs to the server |

### Setup

```bash
git clone https://github.com/YOUR-USERNAME/BrowserOS.git
cd BrowserOS/packages/browseros-agent
bun install
cp apps/server/.env.example apps/server/.env.development
cp apps/agent/.env.example apps/agent/.env.development
```

### Running Locally

```bash
# Terminal 1: Start the server
bun run start:server

# Terminal 2: Start the agent extension (dev mode)
bun run start:agent
```

Then load the extension in BrowserOS:
1. Go to chrome://extensions/
2. Enable Developer mode
3. Click Load unpacked and select the apps/agent/dist/ folder

### Commands

| Command | Description |
|---|---|
| `bun run start:server` | Start the server |
| `bun run start:agent` | Start agent extension (dev mode) |
| `bun run build:server` | Build server for production |
| `bun run build:agent` | Build agent extension |
| `bun run build:ext` | Build controller extension |
| `bun run test` | Run tests |
| `bun run lint` | Check with Biome |
| `bun run typecheck` | TypeScript check |

## Path 2: Browser Development

Only go down this path if you're working on Chromium-level features.

**Requirements:** ~100GB disk space, 16GB+ RAM, 3+ hours for first build.

### Build Instructions

1. Clone Chromium source following the official guide
2. Install UV and dependencies:
```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
cd packages/browseros
uv sync
```
3. Build debug version:
```bash
uv run browseros build \
  --chromium-src <your-chromium-src-path> \
  --setup \
  --prep \
  --build \
  --build-type debug
```
4. Run BrowserOS (platform-specific binary paths)

## Making Your First PR

1. Fork the repository
2. Clone your fork locally
3. Create a branch: `git checkout -b feature/your-feature`
4. Make changes and test
5. Commit: `git commit -m "feat: add your feature"`
6. Push: `git push origin feature/your-feature`
7. Open a PR with a clear description

Sign the CLA on your first PR by commenting: `I have read the CLA Document and I hereby sign the CLA`

## Code Standards

**TypeScript:** Use strict typing, avoid `any`. Use Zod schemas instead of TypeScript interfaces. Use path aliases (@/lib) not relative paths. PascalCase for classes, camelCase for functions.

**React:** Tailwind CSS only. Hooks at top level only. Test with Vitest.

**General:** Keep functions under 20 lines. Write tests for new features. Handle errors gracefully.

By contributing, you agree that your contributions will be licensed under AGPL-3.0.

---

# Features

## Ad Blocking

BrowserOS supports full ad blocking through uBlock Origin, the most powerful open-source ad blocker available - the full extension, not the watered-down "Lite" version.

Chrome killed support for uBlock Origin by phasing out Manifest V2 extensions. BrowserOS re-enabled full Manifest V2 support, so you can install and run the original uBlock Origin at full power.

Install from the Chrome Web Store. Works out of the box.

### BrowserOS vs Chrome Ad Blocking

Test results from adblock.turtlecute.org (133 ad-related requests):

- **BrowserOS** blocked 91 (68%)
- **Chrome** blocked 9 (7%)

That's roughly 10x more protection with zero configuration.

---

## Bring Your Own LLM

Connect your own AI models to BrowserOS. Includes a default AI model with strict rate limits. For best experience, bring your own API keys or run models locally.

### Use Your Existing Subscription

Already paying for ChatGPT Pro, GitHub Copilot, or Qwen Code? Connect your existing account with a single sign-in - no API keys, no extra cost.

### Which Model Should I Use?

| Mode | What works | Recommendation |
|---|---|---|
| **Chat Mode** | Any model, including local | Ollama or Gemini Flash |
| **Agent Mode** | Cloud models only | Claude Opus 4.5, GPT-5, or Kimi K2.5 (open source) |

Local LLMs aren't powerful for most agentic tasks yet. They're great for Chat. But agent tasks need strong reasoning.

### Kimi K2.5 - In Partnership with Moonshot AI

Kimi K2.5 is the recommended model in BrowserOS. Open source, multimodal, 128K context window. For a limited time, BrowserOS users get extended usage limits.

**Bring your own Kimi API key:**
1. Go to platform.moonshot.ai and create an account
2. Navigate to API keys section
3. Create new API key and copy it
4. In BrowserOS Settings, click USE on the Moonshot AI card
5. Enter your API key (encrypted and stored locally)
6. Pre-configured to kimi-k2.5 with 128K context

### Cloud Providers

**Gemini (Free)** - Fast and free. 20 requests/minute. Get API key from aistudio.google.com. Set Model ID to gemini-2.5-flash, context window 1,000,000.

**NVIDIA (Free)** - 80+ models including GLM 5.1, MiniMax M2.7, GPT-OSS-120B, Qwen 3.5. Free OpenAI-compatible endpoint. Base URL: https://integrate.api.nvidia.com/v1

**Claude (Best for Agents)** - Claude Opus 4.5 gives best Agent Mode results. Get key from console.anthropic.com. Model: claude-opus-4-5-20251101, context 200K.

**OpenAI** - GPT-5. Get key from platform.openai.com. Model: gpt-5, context 200K.

**OpenRouter** - Access 500+ models through one API. Get key from openrouter.ai/keys.

**Azure OpenAI** - Enterprise compliance and data residency. Use your Azure endpoint.

**AWS Bedrock** - Access Claude, Llama through AWS with IAM-based auth.

**OpenAI Compatible** - Connect any provider that implements OpenAI-compatible API format (Together AI, Fireworks, Groq, Perplexity, etc.).

### Switching Between Models

Use the model switcher in the Assistant panel to change providers anytime. Use local models for sensitive work data. Switch to Claude for agent tasks.

---

## ChatGPT Pro / Plus OAuth

Connect your ChatGPT Pro or Plus subscription to BrowserOS. Access GPT-5 Codex, GPT-5.4, and the full lineup with up to 400K context. No API keys needed.

### Setup

1. Open BrowserOS Settings (chrome://browseros/settings)
2. Click USE on the ChatGPT Plus/Pro card
3. Sign in with your OpenAI account
4. Authorize and select a model

### Available Models

| Model | Context Window |
|---|---|
| gpt-5.4 | 400K |
| gpt-5.3-codex | 400K |
| gpt-5.2-codex | 400K |
| gpt-5.2 | 200K |
| gpt-5.1-codex | 400K |
| gpt-5.1-codex-max | 400K |
| gpt-5.1-codex-mini | 400K |
| gpt-5.1 | 200K |

ChatGPT Pro includes reasoning effort and reasoning summary settings.

To disconnect: Settings > ChatGPT Plus/Pro provider > Disconnect. OAuth tokens immediately deleted.

---

## Connect Apps (MCP)

Connect 40+ apps to BrowserOS so the assistant can work with your email, calendar, projects, and more. Uses the Model Context Protocol (MCP). No API keys to manage. Just sign in once.

### Smart App Connection

When you ask the assistant to do something that needs an app you haven't connected, it shows an interactive card. Connect with one click, or choose to skip it.

### 40+ Supported Apps

**Email:** Gmail, Outlook Mail, Resend
**Calendar & Scheduling:** Google Calendar, Outlook Calendar, Cal.com
**Messaging:** Slack, Discord, WhatsApp, Microsoft Teams
**Development:** GitHub, GitLab, Vercel, Postman, Cloudflare, Supabase
**Project Management:** Linear, Jira, Asana, Monday, ClickUp
**Documents & Productivity:** Notion, Google Docs, Google Sheets, Google Drive, Google Forms, Confluence, Airtable
**File Storage:** Dropbox, OneDrive, Box
**Design:** Figma, Canva
**CRM & Marketing:** Salesforce, HubSpot
**E-commerce & Payments:** Shopify, Stripe
**Analytics:** PostHog, Mixpanel
**Support:** Zendesk, Intercom
**Search & AI:** Brave Search, Exa, Mem0
**Social & Content:** LinkedIn, YouTube, WordPress

### Cross-App Workflows

- "Find action items in my latest emails and add them to my Notion tasks"
- "Check my calendar for tomorrow, then draft an email to John summarizing what we're meeting about"
- "Test the checkout flow on our staging site, file a Linear issue if anything is broken, and post a summary to #engineering on Slack"

### Add a Custom MCP Server

1. Go to Settings > Connected Apps
2. Click Add custom app
3. Enter your server URL (e.g., http://localhost:8000/sse) and give it a name

For OAuth-protected remote servers, use mcp-remote + supergateway:
```bash
npx -y supergateway --stdio "npx -y mcp-remote https://mcp.atlassian.com/v1/sse" --port 8000
```

### Privacy

All apps use OAuth sign-in. BrowserOS never sees or stores your passwords. Apps only accessed when you ask. Credentials stay local.

---

## Cowork

Give the agent controlled access to local files and commands alongside browser automation. Combines browser automation with local file operations in one session.

### Why Cowork?

Without Cowork, the agent can only interact with browser tabs. With Cowork enabled, it gains full access to a folder on your machine through 7 filesystem tools:

- **Read & write files** - Read documents, write reports, markdown, HTML
- **Edit files** - Surgical string replacement
- **Run commands** - Execute shell commands within sandboxed folder
- **Search content** - Regex or literal pattern search
- **Find files** - Glob pattern matching
- **Browse directories** - List contents with sizes

### Setting Up Cowork

1. Click the Cowork dropdown next to the prompt input
2. Select a recent folder or choose a different folder
3. Grant access when prompted

Agent is sandboxed to your selected folder. Cannot access files outside it. Available in Agent Mode only.

### Filesystem Tools

**filesystem_read** - Read file content with line numbers. Supports pagination. Capped at 2000 lines or 50KB.

**filesystem_write** - Create or overwrite files. Auto-creates parent directories.

**filesystem_edit** - Targeted edit by exact string match. Falls back to fuzzy match. Preserves line endings and BOM.

**filesystem_bash** - Execute shell commands. Run in sh/bash on Unix, cmd on Windows. Default timeout 120 seconds.

**filesystem_find** - Find files by glob pattern. Skips node_modules, .git, dist, etc. Max 1000 results.

**filesystem_grep** - Search file contents with regex or literal matching. Skips binary files and files over 2MB.

**filesystem_ls** - List directory contents. Directories first with trailing /. Shows human-readable sizes. Max 500 entries.

### Security

Sandboxed access (only your selected folder). Revoke anytime (select No folder). Local only.

---

## GitHub Copilot OAuth

Connect your GitHub Copilot subscription to BrowserOS and access 19+ models through a single GitHub sign-in. No API keys needed.

Free tier includes GPT-5 Mini, Claude Haiku 4.5, GPT-4o, GPT-4.1. Copilot Pro ($10/month) unlocks Claude Sonnet 4.6, Claude Opus 4.6, Gemini 3 Pro, GPT-5.4, and more.

### Setup

1. Open BrowserOS Settings
2. Click USE on the GitHub Copilot card
3. Copy device code, click link to open GitHub authorization
4. Paste device code and authorize

### Available Models

**Free Tier:**

| Model | Context Window |
|---|---|
| gpt-5-mini | 128K |
| claude-haiku-4.5 | 128K |
| gpt-4o | 64K |
| gpt-4.1 | 64K |

**Copilot Pro / Pro+:**

| Model | Context Window |
|---|---|
| claude-sonnet-4.6 | 200K |
| claude-opus-4.6 | 200K |
| gemini-2.5-pro | 1M |
| gemini-3-pro-preview | 1M |
| gpt-5.4 | 400K |
| gpt-5.3-codex | 400K |
| gpt-5.2-codex | 400K |
| grok-code-fast-1 | 128K |

---

## LLM Chat & Hub

Access ChatGPT, Claude, and Gemini from any webpage with one click.

### LLM Chat

Click the Chat button in the toolbar (or press Option+K) to open an AI chat panel on any webpage.

Features:
- Switch providers instantly (Option+L to cycle)
- Copy page context
- Screenshot and attach
- Works everywhere - panel stays open as you browse

### LLM Hub (Clash of GPTs)

Click the Hub button (or Cmd+Shift+U) to open a multi-pane comparison window. Query multiple LLMs simultaneously. Choose 1, 2, or 3 panes. Compare responses across Claude, Gemini, and ChatGPT.

### Customization

Control which buttons appear in toolbar from Settings > Customization: Show Chat Button, Show Hub Button, Show Button Labels.

---

## Local Models

Run AI models locally with Ollama or LM Studio for free, private, offline use.

### Context Length Warning

Ollama defaults to 4,096 tokens of context - this is too low for BrowserOS. Below 15K tokens, context overflows and the agent gets stuck in a loop. Set at least 15,000-20,000 tokens.

```bash
OLLAMA_CONTEXT_LENGTH=20000 ollama serve
```

### Ollama Setup

1. Download from ollama.com
2. Pull a model: `ollama pull qwen/qwen3-4b`
3. Start with higher context: `OLLAMA_CONTEXT_LENGTH=20000 ollama serve`
4. Configure in BrowserOS Settings > USE on Ollama card > Model ID: qwen/qwen3-4b, Context Window: 20000

### LM Studio Setup

1. Download from lmstudio.ai
2. Open LM Studio > Developer tab > load a model (runs at http://localhost:1234/v1/)
3. Configure in BrowserOS Settings > USE on OpenAI Compatible card > Base URL: http://localhost:1234/v1/

### Recommended Models

**Lightweight (under 5 GB, 8 GB RAM):**

| Model | Params | Size |
|---|---|---|
| qwen/qwen3-4b | 4B | 2.28 GB |
| mistralai/ministral-3-3b | 3B | 2.99 GB |
| deepseek-r1-distill-qwen-7b | 7B | 4.68 GB |
| deepseek-r1-distill-llama-8b | 8B | 4.92 GB |

**Mid-range (10-15 GB, 16+ GB RAM):**

| Model | Params | Size |
|---|---|---|
| openai/gpt-oss-20b | 20B | 12.11 GB |
| mistralai/magistral-small | 23.6B | 13.28 GB |
| mistralai/devstral-small-2-2512 | 24B | 14.12 GB |

**Heavy (60+ GB, 64+ GB RAM):**

| Model | Params | Size |
|---|---|---|
| openai/gpt-oss-120b | 120B | 63.39 GB |

Start with qwen/qwen3-4b if unsure.

---

## Memory

Your assistant remembers what matters across every conversation.

### How Memory Works

Memory is automatic. As you chat, the assistant saves important facts and observations to local files. Before responding in future conversations, it searches these files to recall relevant context.

### Two Types of Memory

**Core Memory** - Permanent facts about you (name, job, projects, tools, people). Stored in CORE.md. Persists forever.

**Daily Memory** - Session notes, observations, recent events. Each day gets its own file (e.g., 2026-03-07.md). Auto-expires after 30 days. Promoted to core memory if it keeps coming up.

### Where Memory Lives

| File | Path | Purpose |
|---|---|---|
| Core memory | ~/.browseros/memory/CORE.md | Permanent facts |
| Daily notes | ~/.browseros/memory/2026-03-07.md | Session notes, auto-expire after 30 days |

### Memory vs SOUL.md

Memory = facts about you and the world. SOUL.md = how the assistant acts (personality, tone, boundaries). Separate by design.

### Privacy

Never leaves your machine. Even with Sync to Cloud enabled, memory stays local. You control what is remembered. Plain text files. 30-day auto-cleanup for daily notes.

---

## Qwen Code OAuth

Connect your Qwen Code account to BrowserOS. Access Alibaba's coding models with up to a 1 million token context window. No API keys needed.

### Available Models

| Model | Context Window |
|---|---|
| coder-model | 1M |
| qwen3-coder-plus | 1M |
| qwen3-coder-flash | 1M |
| qwen3.5-plus | 1M |

1M token context is ideal for long documents, entire documentation sites, or working across many browser tabs.

---

## Scheduled Tasks

Run the BrowserOS agent automatically on a schedule. Write a prompt once, set a schedule, let the agent handle it on autopilot.

### Creating a Scheduled Task

From a conversation: After the agent completes a task, it will suggest scheduling it. Or ask directly: "Schedule this to run every morning."

From settings: Click Scheduled Tasks in sidebar > New Task > fill in Name, Prompt, Schedule type, Enable.

### Schedule Types

- **Daily** - Once a day at a specific time
- **Hourly** - Every N hours (1-24)
- **Minutes** - Every N minutes (1-60)

### Example Use Cases

- Morning briefing: check Google Calendar + summarize events
- LinkedIn automation: accept connection requests daily
- Price monitoring: check Amazon every hour
- Competitor tracking: check competitor websites every morning
- Social media digest: top posts from HN and r/programming every evening
- Cross-app workflow: Calendar > Slack > Notion

### How It Works

1. Browser alarm triggers on schedule (runs when you open BrowserOS if laptop was closed)
2. Hidden browser window opens (never interrupts your work)
3. Agent executes your prompt with full browser automation and connected apps
4. Results saved to New Tab page and task run history

BrowserOS needs to be open. 10-minute timeout per task.

### Cloud Sync

Task configurations sync across devices. Run results stay on the device where the task ran.

---

## Skills

Teach your BrowserOS agent new abilities with reusable, custom instructions. Each skill is a set of instructions in plain Markdown that the agent loads when it recognizes a matching task.

BrowserOS implements the open Agent Skills specification (https://agentskills.io/specification), so skills are portable across any AI agent that supports the standard.

### How Skills Work

1. You create a skill with name, description, and instructions
2. Agent sees the skill catalog at conversation start
3. When your request matches a skill's description, agent loads full instructions

### Creating a Skill

1. Click Skills in sidebar
2. Click New Skill
3. Fill in Name, Description (trigger-like), Content (Markdown instructions)
4. Click Create (enabled by default)

Write descriptions like triggers: say both what the skill does and when to use it.

### Example Skills

**Morning Status Report** - Description: "When the user wants to read status updates from work." Instructions: Check Notion, Linear, Slack. Summarize by source.

**PDF Processing** - Description: "Extract text and tables from PDFs, fill forms, merge PDFs." Instructions: Download/open, extract with page content tool, preserve tables.

**Code Review Checklist** - Description: "When the user asks to review code or PRs." Instructions: Check security, performance, error handling, naming, tests. Format as Critical/Warning/Suggestion.

### Skill File Format

Each skill is a SKILL.md file with YAML frontmatter:

```markdown
---
name: morning-status-report
description: When the user wants to read status updates from work
metadata:
  display-name: Morning Status Report
  enabled: "true"
---
Always look for updates in 3 sources:
1. Notion - Check the team updates page
2. Linear - Look at assigned issues updated in the last 24 hours
3. Slack - Check #team-updates and #engineering channels
```

Frontmatter fields: name (required), description (required), license, compatibility, metadata, allowed-tools.

Supporting directories: scripts/, references/, assets/.

### Where Skills Live

| OS | Path |
|---|---|
| macOS | ~/.browseros/skills/ |
| Windows | %USERPROFILE%\.browseros\skills\ |
| Linux | ~/.browseros/skills/ |

### Tips

Be specific in descriptions. Keep instructions focused (one thing well). Include examples. Use supporting files for detailed references (saves context space).

---

## Smart Nudges

Context-aware suggestions that appear as interactive cards during a conversation.

### App Connection Nudge

When you ask the agent to do something involving an unconnected app, it shows a connection card. Connect or skip. If you decline, agent remembers and won't ask again. Use browser automation as fallback.

### Schedule Suggestion Nudge

After the agent completes a schedulable task, it shows a scheduling card. "Run this automatically? 'Morning News Briefing' - daily at 09:00." Schedule it or dismiss.

You can also ask directly: "Schedule this to run every morning."

### When Nudges Appear

| Nudge | When | How often |
|---|---|---|
| App Connection | Before agent starts, when app not connected | Once per app per conversation |
| Schedule Suggestion | After agent finishes a schedulable task | Once per conversation |

Nudges do NOT appear in scheduled tasks (background) or Chat mode (read-only).

---

## SOUL.md

Give your AI assistant a personality that grows with you. SOUL.md is a plain text file that defines who the assistant is: how it talks, what it prioritizes, how it behaves.

The SOUL.md concept was pioneered by OpenClaw and inspired by soul.md. BrowserOS builds on this with a file the assistant can read and rewrite on its own.

### How It Works

1. First conversation: assistant starts with default template
2. As you chat: assistant learns your style and rewrites SOUL.md
3. Every future conversation: assistant reads updated SOUL.md before responding

You don't need to write or edit SOUL.md yourself. The assistant handles it. But you can always view it or ask for changes.

### Shaping Your Assistant

- "Be more casual and direct. Skip the formalities." (Set the tone)
- "Never post to Slack or send emails without confirming with me first." (Add a boundary)
- "Be more opinionated. If you think my approach is wrong, say so." (Change personality)
- "Reset your personality to the default." (Start fresh)

### Where SOUL.md Lives

| OS | Path |
|---|---|
| macOS | ~/.browseros/SOUL.md |
| Windows | %APPDATA%/.browseros/SOUL.md |
| Linux | ~/.browseros/SOUL.md |

Plain Markdown, limited to 150 lines.

### SOUL vs Memory

SOUL.md = how the assistant behaves (personality, tone, rules).
Memory = what the assistant knows about you (name, projects, events).

Separate by design. Change personality without losing knowledge, and vice versa.

---

## Sync to Cloud

Sign in to sync your conversations, settings, and automations across all your devices.

### What Gets Synced

- **Conversations** - Full chat history, real time. Locally keeps 50 most recent. Cloud: no limit.
- **AI model settings** - Provider configs (model name, type, base URL, temperature, context). **API keys never synced.**
- **Scheduled tasks** - Configurations sync both ways. Conflicts resolved by timestamps. Run results stay local.
- **Profile** - Name, picture, role, company.

### What Stays Local

- API keys and secrets for LLM providers
- Memory (core facts and daily notes)
- SOUL.md (assistant personality)
- Theme (light/dark mode)
- Workspace folder selection
- Connected MCP servers
- Workflows
- Scheduled task results

### How Sync Works

Local-first. Everything saved on your device first, then synced to cloud in background. Works fully offline. Sync resumes automatically when connectivity restored.

### Security

API keys never leave your device. Session-based authentication (magic links or Google OAuth). No passwords stored. All synced data scoped to your account. Sync failures are silent (local data unaffected).

---

## Use with Claude Code / MCP Clients

Control your browser and 40+ apps from Claude Code, OpenClaw, Gemini CLI, or any MCP client. BrowserOS comes with a built-in MCP server.

### Getting Started

1. Open BrowserOS Settings > chrome://browseros/mcp
2. Copy the Server URL (e.g., http://127.0.0.1:9239/mcp)
3. Connect your MCP client

### Connecting Clients

**Claude Code:**
```bash
claude mcp add --transport http browseros http://127.0.0.1:9239/mcp --scope user
```

**Gemini CLI:**
```bash
gemini mcp add local-server http://127.0.0.1:9239/mcp --transport http --scope user
```

**Codex:**
```bash
codex mcp add browseros http://127.0.0.1:9239/mcp --transport http
```

**OpenClaw:** Add to mcpServers in openclaw.json:
```json
{
  "mcpServers": {
    "browseros": {
      "url": "http://127.0.0.1:9239/mcp"
    }
  }
}
```

**Claude Desktop:** Add to claude_desktop_config.json:
```json
{
  "mcpServers": {
    "browserOS": {
      "command": "npx",
      "args": ["mcp-remote", "http://127.0.0.1:9239/mcp"]
    }
  }
}
```

### 53 Browser Automation Tools

**Navigation & Tabs (8 tools):** get_active_page, list_pages, navigate_page, new_page, new_hidden_page, show_page, move_page, close_page

**Content & Observation (8 tools):** take_snapshot, take_enhanced_snapshot, get_page_content, get_page_links, get_dom, search_dom, take_screenshot, evaluate_script

**Interaction & Input (14 tools):** click, click_at, hover, focus, fill, clear, check, uncheck, select_option, press_key, drag, scroll, upload_file, handle_dialog

**File & Export (3 tools):** save_pdf, save_screenshot, download_file

**Window Management (5 tools):** list_windows, create_window, create_hidden_window, close_window, activate_window

**Tab Groups (5 tools):** list_tab_groups, group_tabs, update_tab_group, ungroup_tabs, close_tab_group

**Bookmarks (6 tools):** get_bookmarks, create_bookmark, remove_bookmark, update_bookmark, move_bookmark, search_bookmarks

**History (4 tools):** search_history, get_recent_history, delete_history_url, delete_history_range

### 40+ External App Integrations

All accessible through the same MCP connection. OAuth-based. See [Connect Apps](#connect-apps-mcp) for full list.

---

## Vertical Tabs

Move your tabs to the side for a cleaner, more organized browsing experience. BrowserOS ships with vertical tabs by default.

### Why Vertical Tabs?

Modern screens are wide, not tall. Horizontal tab bar wastes vertical space. Vertical tabs give each tab its own full-width row.

- Read every tab title at a glance
- Handle many tabs (30, 50, 100+) without strip becoming unusable
- Reclaim vertical space on widescreen monitors
- Combine with tab groups for organization

### Enabling

1. Go to chrome://browseros/settings
2. Select Customization
3. Toggle Use Vertical Tabs on/off

When enabled: tabs relocate to a collapsible side panel on the left. Click to switch, right-click for context menu, drag to reorder. Can collapse to show only favicons.

---

## Workflows

Build reliable, repeatable browser automations with a visual graph builder.

### When to Use Workflows

- Reliability matters (needs to work the same way every time)
- Steps are complex (multiple pages, loops, conditionals, parallel actions)
- You'll repeat it (run daily, weekly, or on-demand)

For quick one-off tasks, the regular agent works well. For serious automation, build a workflow.

### Creating Your First Workflow

1. Open Workflows page from sidebar
2. Click + New Workflow
3. Describe what you want in the chat panel
4. Workflow agent generates a visual graph
5. Refine by chatting further
6. Click Test Workflow to verify
7. Click Save Changes

### Example Use Cases

- Data entry automation: read from Google Sheet, submit to web form
- LinkedIn outreach: visit profiles, check criteria, send connection requests
- Price monitoring: check multiple e-commerce sites, compile spreadsheet
- Bulk unsubscribes: find subscription emails in Gmail, click unsubscribe

---

# Integrations

## n8n Integration

Connect BrowserOS to n8n to build automated workflows. Your n8n AI agents can control the browser.

### What You Can Build

- Lead enrichment: Pull LinkedIn URLs from spreadsheet and scrape each profile
- Price monitoring: Check product prices hourly, send Slack alert on drops
- Form automation: Auto-fill job applications from CRM data
- Data extraction: Scrape authenticated pages requiring login

### Setup

1. Install MCP community node in n8n: Settings > Community Nodes > install `n8n-nodes-mcp`
2. Create a workflow with Chat Trigger > AI Agent > Chat Model > MCP Client Tool
3. Configure AI Agent: Source for Prompt = Connected Chat Trigger Node
4. Configure MCP Client: Endpoint = your BrowserOS MCP URL, Server Transport = HTTP Streamable
5. Test: "open google.com in BrowserOS"

### Troubleshooting

Make sure BrowserOS is running, MCP server is enabled, and port number matches.

---

# Troubleshooting

## Has Agent Crashed?

If you see "Failed to Fetch" or "Unable to connect to BrowserOS agent", your agent has been killed.

### Windows: Did you allow the BrowserOS Agent?

When BrowserOS starts for the first time on Windows, a popup asks to allow BrowserOS Agent network access. Click Allow. If you clicked Cancel, go to Windows Security > Allow an app through your network settings and enable BrowserOS Agent.

### Fix It

1. Go to Settings > MCP Server and click Restart. Wait a couple minutes.
2. If restart doesn't work:
   - **Windows:** Open Task Manager (Ctrl+Shift+Esc), search "browseros agent", End task, reopen BrowserOS
   - **macOS:** Right-click BrowserOS in Dock > Quit, then reopen

### Still Not Working?

**Check if another app is using the port** (default 9100):

macOS/Linux:
```bash
lsof -i :9100
kill -9 PID
```

Windows:
```powershell
netstat -ano | findstr :9100
taskkill /PID PID /F
```

Then restart BrowserOS.

**Check firewall/antivirus:** Make sure firewall allows connections to 127.0.0.1:9100. Windows users: try temporarily disabling antivirus.

### Get Help

- Discord: https://discord.gg/YKwjt5vuKr
- Slack: https://dub.sh/browserOS-slack

---

# Updates

## Update on macOS

1. Check version: Settings > About BrowserOS
2. Auto-updates: restart browser to apply pending updates
3. Manual: download from browseros.com or GitHub Releases

## Update on Windows

1. Check for updates: Click BrowserOS Feedback extension (bug icon) > Check for Updates
2. Download Windows installer from GitHub Releases
3. Run installer (temporary folder appears - normal). Wait 30-40 seconds. BrowserOS appears on desktop.
4. Allow BrowserOS Agent network access when prompted. If you click Cancel, agent won't connect - see troubleshooting.

Settings and data preserved after update.

## Update on Linux

1. Check for updates: Click BrowserOS Feedback extension (bug icon) > Check for Updates
2. Download Linux build from GitHub Releases
3. Extract and run. Settings and data preserved.

---

*End of BrowserOS documentation offline copy. Source: https://docs.browseros.com/ | Scraped: April 30, 2026*