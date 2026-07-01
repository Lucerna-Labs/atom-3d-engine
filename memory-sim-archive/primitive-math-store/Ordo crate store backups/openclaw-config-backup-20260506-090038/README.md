# OpenClaw + Claude config backup — 2026-05-06

Snapshot of all OpenClaw and Claude configuration / state / secrets
on the user's machine, captured **before** investigating a possible
post-update OpenClaw breakage.

Source machine: Windows 11, user `jgali`
Snapshot time:   2026-05-06 09:00 UTC-7 (local)
Backup root:     `G:\Ordo crate store backups\openclaw-config-backup-20260506-090038\`

---

## Layout

```
openclaw-config-backup-20260506-090038/
├── README.md                               (this file)
├── openclaw/                               (357 MB / ~6.5K files)
│   └── (full content of C:\Users\jgali\.openclaw\
│        — see "OpenClaw layer" below)
├── claude-code/                            (244 MB)
│   └── (full content of C:\Users\jgali\.claude\)
├── user-home/                              (509 MB)
│   ├── .claude.json
│   ├── .claude.json.backup
│   ├── .claude-server-commander/
│   ├── .claude-server-commander-logs/
│   ├── .openclaw-dev/
│   ├── browseros-mcp/
│   └── mcp-hub/
├── claude-desktop/                         (713 MB)
│   └── (selective from C:\Users\jgali\AppData\Roaming\Claude\
│        — see "Claude desktop layer" below)
└── openclaw_BROKEN_PARTIAL_DELETEME/       (artefact — see Notes)
```

Total:  ~1.8 GB of actual config + state + secrets.

---

## OpenClaw layer (`openclaw/`)

Source: `C:\Users\jgali\.openclaw\`

### Included

- **Top-level config**:
  - `openclaw.json` (live config)
  - `openclaw.json.bak`, `openclaw.json.bak.1` … `.bak.4` (rolling backups)
  - `openclaw.json.clobbered.*` (auto-saved snapshots before a clobber)
  - `openclaw.json.pre-*.bak` (named-checkpoint backups: pre-canonicalize,
    pre-local-fix, pre-systemprompt-fix, pre-ollama-cloud)
  - `exec-approvals.json`
  - `update-check.json`
- **Subdirectories** (every dir under `.openclaw/`):
  - `agents/`, `browser/`, `canvas/`, `classifier/`, `completions/`
  - `credentials/` ← **secrets included** (per user request)
  - `cron/`, `delivery-queue/`, `devices/`, `identity/`, `logs/`, `media/`
  - `memory/` ← OpenClaw's working / pinned memory
  - `skills/` ← github, hit-song-lyrics, lyric-persona-styles,
    lyric-style-generator, openclaw-secrets-operator, poppy-king-horror
  - `subagents/`, `telegram/`
  - `workspace-dev/`, `workspace-lite-lm/`, `workspace-local-lm/`
- **`workspace/` — the OpenClaw "brain"**:
  - All identity / memory `.md` files: MEMORY.md (22 KB), SOUL.md, IDENTITY.md, USER.md, AGENTS.md, TOOLS.md, HEARTBEAT.md, BCC-DEPLOYMENT-STATUS.md
  - Modelfile, Modelfile.Nemotron, Modelfile.Qwen3.5-35B
  - `Tracey Buisness plans/` directory
  - `.auth/`, `.clawhub/`, `.git/`, `.openclaw/`, `.quantum/` subdirs
  - **`rag/zed-deconstruction/`** — full content:
    - `crates/` (149 cap-* deconstructions — **complete**)
    - `zed-source/crates/` (all 232 Zed crates — **complete**)
    - DECONSTRUCTION-MAP.md, ZED-CRATE-INVENTORY.md
- **All grapesjs-app source** (templates, configs) but NOT its `node_modules`

### Deliberately excluded

These cache / build directories were **not** copied — they're easily
regenerated (`npm install`, `cargo build`) and add gigabytes for no
restore value:

- `node_modules/`, `.pnpm/`, `.yarn/`, `.turbo/`, `.svelte-kit/`
- `target/` (Rust build output)
- `dist/`, `build/`, `.next/`
- `__pycache__/`, `.pytest_cache/`, `.ruff_cache/`
- `*.pyc`

256 such directories were detected and skipped during copy, then 254
of any leftovers from a failed first attempt were also removed.

### Notes

- **`openclaw_BROKEN_PARTIAL_DELETEME/`** at the backup root is a
  remnant from a first copy attempt that hit Windows MAX_PATH limits
  on deeply-nested `node_modules/.pnpm/` paths in `grapesjs-app/`. It
  was renamed and partly cleaned, but a couple of leftover dirs
  remain because Windows refuses to delete those long paths through
  any standard shell. **Safe to ignore — the full clean copy is in
  `openclaw/`.** When convenient, you can wipe this folder by going
  to its enclosing dir in File Explorer and using "Move to Recycle
  Bin," which works around the long-path issue.
- 2 leftover long-path dirs (also under `grapesjs-app/node_modules/.pnpm/`)
  could not be removed from `openclaw/` either. They are harmless —
  same data as the BROKEN folder. Restore should ignore them.

---

## Claude Code layer (`claude-code/`)

Source: `C:\Users\jgali\.claude\`

Full directory copy. Notable contents:

- `.credentials.json` ← Anthropic API token / account secrets
- `settings.json`
- `CLAUDE.md` (global user instructions)
- `mcp-needs-auth-cache.json`
- `stats-cache.json`
- `backups/`, `plugins/`, `projects/` (98 MB of project sessions/transcripts),
  `session-env/`, `sessions/`, `shell-snapshots/`, `todos/`, `telemetry/`,
  `debug/`, `ide/`, `plans/`

Nothing excluded from this layer.

---

## User-home layer (`user-home/`)

Source: `C:\Users\jgali\` (selected entries)

- `.claude.json`, `.claude.json.backup` (Claude CLI top-level config)
- `.claude-server-commander/` + `.claude-server-commander-logs/`
- `.openclaw-dev/` (dev-mode OpenClaw config)
- `mcp-hub/` (29 KB)
- `browseros-mcp/` (34 MB — bundled MCP server for browser automation)

Not included (not asked, but flagging in case):
- `openclaw-backup-archives/` (13 MB — local backup archives)
- `openclaw-backup-staging/` (69 MB — backup staging area)

---

## Claude desktop layer (`claude-desktop/`)

Source: `C:\Users\jgali\AppData\Roaming\Claude\` (selective)

### Included

All config files at root: `bridge-state.json`, `buddy-tokens.json`,
`claude_desktop_config.json`, `config.json`, `cowork-enabled-cli-ops.json`,
`extensions-blocklist.json`, `extensions-installations.json`,
`git-worktrees.json`, `Local State`, `Preferences`, `window-state.json`.

All app-state directories: `Claude Extensions/`, `Claude Extensions Settings/`,
`Local Storage/`, `Session Storage/`, `IndexedDB/`, `WebStorage/`,
`Network/`, `Partitions/`, `Shared Dictionary/`, `SharedStorage*`,
`VideoDecodeStats`, `ant-did/`, `blob_storage/`, `Conversions*`,
`DIPS*`, `fcache/`, `lockfile`, `pending-uploads/`, `sentry/`,
`shared_proto_db/`, `ChromeNativeHost/`.

Application binaries / VM bundles: `claude-code/`, `claude-code-vm/`,
`claude-code-sessions/`, `local-agent-mode-sessions/`, `logs/`.

### Deliberately excluded

Pure-cache directories (~14 GB of regenerable files):

- `vm_bundles/` (13 GB — Claude desktop VM disk images)
- `Cache/` (213 MB)
- `Code Cache/` (273 MB)
- `GPUCache/`, `DawnGraphiteCache/`, `DawnWebGPUCache/`
- `Crashpad/`

### Notes

- A handful of `LOCK` / `Cookies-journal` files in `IndexedDB/` /
  `Local Storage/` / `Session Storage/` could not be copied because
  Claude desktop was running and held them open. These are the
  **leveldb lock indicators**, not data — they're recreated
  automatically on next launch and their absence does not affect
  restore.
- One broken symlink under `local-agent-mode-sessions/.../debug/latest`
  also failed; it's a stale pointer and irrelevant.

---

## Restore guidance

To restore on the same machine (or a fresh one):

1. Stop Claude desktop / Claude Code / OpenClaw if running.
2. Copy each subdirectory back to its source path:
   - `openclaw/` → `C:\Users\jgali\.openclaw\` (or equivalent on a new user)
   - `claude-code/` → `C:\Users\jgali\.claude\`
   - `user-home/` → contents go directly into `C:\Users\jgali\`
   - `claude-desktop/` → `C:\Users\jgali\AppData\Roaming\Claude\`
3. The excluded cache dirs will recreate themselves on first launch.
4. Run `npm install` / `cargo build` in any project subdir that needs
   `node_modules` / `target`.
5. **Do NOT copy back** the `openclaw_BROKEN_PARTIAL_DELETEME/` folder.

## Verification snapshot at backup time

(Sizes measured with PowerShell `Get-ChildItem | Measure-Object Length`,
which reports actual file bytes. Earlier cygwin `du -sh` numbers were
inflated by NAS cluster-block padding and aren't reliable.)

```
openclaw/         511.6 MB     36,675 files
claude-code/      107.7 MB      1,202 files
user-home/         31.8 MB      3,970 files
claude-desktop/   615.6 MB        868 files
──────────────────────────────────────────────
Total           ~1.28 GB     45,045 files
```

Brain-file integrity check (source size = backup size):

```
MEMORY.md     22,032 bytes  ✓
SOUL.md        1,999 bytes  ✓
IDENTITY.md      586 bytes  ✓
USER.md        2,469 bytes  ✓
AGENTS.md     10,256 bytes  ✓
TOOLS.md       4,235 bytes  ✓
HEARTBEAT.md     193 bytes  ✓
```

Rag-folder content count (source = backup):

```
.openclaw/workspace/rag/zed-deconstruction/crates/             149 / 149  ✓
.openclaw/workspace/rag/zed-deconstruction/zed-source/crates/  232 / 232  ✓
```
