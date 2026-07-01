# TOOLS.md - Local Notes

This file intentionally excludes raw secrets.

Use it for stable metadata, paths, hostnames, aliases, and workflow notes. Store passwords, tokens, API keys, and OAuth secrets in Windows Credential Manager or the tool's native secure store.

## Secret Storage Policy

- Never store raw credentials in `TOOLS.md`, `MEMORY.md`, scripts, JSON exports, or scratch files.
- Prefer Windows Credential Manager aliases for app passwords, bearer tokens, and API secrets.
- Prefer manifest files plus aliases for integrations that need repeatable startup wiring.
- If a token must be exported for a one-off task, delete the export when the task is done.

## Cloudflare - Lucerna Media

- Domain: lucernamedia.com
- Zone ID: 17c934d5d77e944d99f257ffd9fa9139
- Account: jgalicea@gmail.com
- API token alias: `openclaw/cloudflare/alexbot/api-token`
- **Working:** DNS:Edit (read + write confirmed), Zone:Read
- **NOT working:** SSL settings writes, zone settings writes, certificate writes (403 despite permissions showing them)
- **For DNS changes:** DELETE old record then CREATE new one (PUT/UPDATE returns 403, but DELETE+POST works)
- Proxy must stay OFF — VPS nginx handles SSL directly

## Ghost CMS - Warped Reality (lucernamedia.com)

- Status: Active (live)
- URL: https://lucernamedia.com
- Admin URL: https://lucernamedia.com/ghost/
- SSH alias: `elestio-vps`
- Access source: `C:\Users\jgali\.openclaw\skills\openclaw-secrets-operator\SKILL.md`
- DB access: Direct MySQL on VPS (Admin API key is read-only for posts)
- Setup notes: `rag/content-creation/ghost-setup.md`
- Guardrail:
- Keep SSH private key files, passphrases, and `IdentityFile` paths out of OpenClaw docs, manifests, and config.

## Bluesky - Warped Reality

- Handle: `warped-reality.bsky.social`
- Profile: `https://bsky.app/profile/warped-reality.bsky.social`
- API Base: `https://bsky.social/xrpc/`
- Credential alias:
- `openclaw/social/bluesky/warped-reality/app-password`
- Session guidance:
- Keep `accessJwt`, `refreshJwt`, and `did` in runtime memory only.
- Do not write session tokens back into the workspace.

## Mastodon - Warped Reality

- Handle: `@Warped_Reality@mastodon.social`
- Profile: `https://mastodon.social/@Warped_Reality`
- Instance: `https://mastodon.social`
- Credential aliases:
- `openclaw/social/mastodon/warped-reality/password`
- `openclaw/social/mastodon/warped-reality/access-token`
- API notes:
- Use `Authorization: Bearer <token>` for API calls.
- Keep bearer tokens out of docs and scripts.

## Google OAuth - AlexBot

- Project: `alexbot-491817`
- Client ID: `883344862728-ngqmf4pkgtva1b91huj737bn7fra1nc6.apps.googleusercontent.com`
- Redirect URI: `http://localhost`
- Client secret alias:
- `openclaw/google/alexbot/client-secret`
- Note:
- Raw OAuth export files were removed from the workspace on `2026-04-11`.
- Use `gog`'s stored credentials or a secure re-auth flow if fresh exports are needed.

## Blogger API - Warped Reality

- Blog ID: `93415309474644637`
- Blog URL: `https://warpedreality1.blogspot.com/`
- Blog Name: `Warped Reality`
- API Base: `https://www.googleapis.com/blogger/v3/blogs/93415309474644637`
- OAuth Scope: `https://www.googleapis.com/auth/blogger`
- SEO Setup Guide: `references/blogger-seo-setup.md`
- Workflow note:
- Direct Blogger API calls should use `gog`'s stored credentials or a secure export path, not workspace token dumps.

## Reddit / Devvit

- Status: Legacy moderation automation was retired from the workspace on `2026-04-11`.
- Devvit app: `alex-mod-assista`
- Devvit workspace: `C:\Users\jgali\.openclaw\workspace\devvit\alex-mod-tool\alex-mod-assista`
- External auth token path: `C:\Users\jgali\.devvit\token`
- Guardrail:
- Do not store Reddit usernames, emails, or passwords in workspace files.

## LibreOffice

- Folder: `C:\Users\jgali\LibreOffice Documents`
- Binary: `C:\Program Files\LibreOffice\program\soffice.exe`
- Formats: ODT (Writer), ODS (Calc), ODP (Impress)
- Conversions: ODT <-> DOCX <-> PDF, ODS <-> XLSX, ODP <-> PPTX

## Retired Platforms

- Pixelfed: Deleted on `2026-04-08`
- FlokiNet: Old access details and tokens removed on `2026-04-11`
- WordPress: Old workspace wiring removed; Ghost is now the canonical CMS target
