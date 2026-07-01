# GrapesJS Standalone App

## What You Have

This is a complete GrapesJS setup ready for building a standalone web builder application.

## Quick Start

### Option 1: Open the Example Directly
Simply open `examples/standalone.html` in your browser. That's it — no server needed for basic testing.

### Option 2: Run the Dev Server
```bash
cd C:\Users\jgali\.openclaw\workspace\grapesjs-app
pnpm run start
```
Then open http://localhost:8080

## Project Structure

```
grapesjs-app/
├── dist/                    # Built GrapesJS library
│   ├── grapes.min.js       # Main JS bundle
│   └── css/grapes.min.css  # Styles
├── packages/
│   ├── core/               # GrapesJS core
│   ├── cli/                # Build tools
│   └── preset-webpage/     # Webpage preset plugin
├── examples/
│   └── standalone.html     # Ready-to-use example
└── STANDALONE-README.md    # This file
```

## The Example App

`examples/standalone.html` includes:
- Full GrapesJS editor initialized
- Basic blocks (sections, images, text)
- Export buttons (HTML/CSS to console)
- Local storage autosave
- Clean header with actions

## Next Steps

To customize this for your needs:

1. **Modify the example** - Edit `examples/standalone.html` to change the UI, add plugins, or customize blocks

2. **Add plugins** - Install GrapesJS plugins via pnpm:
   ```bash
   pnpm add grapesjs-plugin-export
   ```

3. **Create your own app** - Copy `examples/standalone.html` and build from there

4. **Check the docs** - https://grapesjs.com/docs/

## Built Version

GrapesJS v0.22.14 (latest stable)

---

🗡️ Ready to build something.
