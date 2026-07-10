# PMRE / Fable5 Session Applicability Notes

Date: 2026-07-02

Source document:

```text
C:\Users\jgali\Downloads\This is a math primitive rendering engine, - Google Docs.pdf
```

Extracted read artifact:

```text
C:\Projects\3D Primitve math engine\tmp\pdfs\fable5_renderer_session_extracted.txt
```

Related project inspected:

```text
C:\Projects\primitive-math-rendering-engine
```

## Short Read

The Fable5 session is highly relevant, but it applies mainly as a **2D UI/UX primitive renderer** and Ordo UI surface, not as a replacement for MM3E's 3D SDF marcher.

The useful transfer is:

- PMRE can provide the Ordo/provider-tab UI renderer layer.
- PMRE's text, layout, HTML/CSS reduction, hit-testing, and framebuffer code can become the 2D overlay/HUD/UI companion to MM3E.
- MM3E should keep its 3D analytic field renderer separate.
- The mechanism/policy split is strongly aligned with the MM3E kit/orchestrator split.
- The session exposed a doctrine problem: the README claims zero dependencies, but the current `pmre-orchestrator` depends on `wgpu` and `pollster` for GPU bloom. That needs quarantining or documenting before treating PMRE as zero-dependency Ordo core.

## What Was Verified Locally

Commands run in `C:\Projects\primitive-math-rendering-engine`:

```powershell
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo tree
```

Observed:

- `cargo test --workspace` passed.
- Test count: 13 kit tests plus 1 orchestrator test passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- Working tree was clean before inspection.
- Render screenshots exist and were visually checked: `docs/screenshot.png`, `docs/html.png`, and `docs/todo.png`.
- `cargo tree` showed `pmre-kit` has no external dependencies, but `pmre-orchestrator` depends on `wgpu` and `pollster`.

## What PMRE Built

PMRE is a 2D renderer built from primitive math:

- SDF rectangles, rounded rectangles, circles, and lines.
- Scanline path rasterization for arbitrary polygon/path fills and strokes.
- Smoothstep analytic anti-aliasing.
- Gradients.
- Alpha-over compositing.
- Framebuffer and BMP output.
- TrueType parsing and glyph rasterization without a font crate.
- Rich inline text flow.
- Reduced HTML/CSS parser.
- Reduced flex/box layout.
- Hit-testing, clipping, scroll regions, buttons, toggles, text input, and a live scrollbar.
- Raw Win32/GDI live window path.
- DPI-aware painting.
- Post effects and bloom variants.

This is not the same engine shape as MM3E. PMRE is a 2D UI renderer; MM3E is a 3D analytic field renderer. The bridge is UI composition, not core geometry.

## What Applies To MM3E

### 1. Shared Primitive Doctrine

PMRE maps UI rendering onto the same eight root atoms:

- scan: pixels, layout nodes, glyph contours, parser tokens
- hash: stable ids, cache keys, color/name lookup, glyph cache
- fold: layout measurement, text wrap, compositing, bloom passes
- project: affine transforms, local-to-screen mapping, glyph/path flattening
- scale: DPI, font size, opacity, gradients, margins
- compare: SDF distance, hit testing, clipping, z/order tests
- combine: alpha-over, spans into paragraphs, layout boxes, post effects
- order: painter order, focus/hover precedence, render passes

This confirms that the root-atom doctrine works below 3D rendering too. It covers UI, text, and HTML reduction.

### 2. Framebuffer And Overlay Layer

PMRE's `Framebuffer`, `Surface`, `BandView`, alpha-over compositing, clipping, and BMP output are directly useful for a 2D overlay on top of MM3E output.

Likely transfer:

```text
MM3E 3D framebuffer -> PMRE UI overlay -> final image/window
```

This is cleaner than forcing MM3E itself to become a UI engine.

### 3. Text Rendering

The TrueType work is probably the biggest transfer win.

Useful pieces:

- system font lookup
- cmap format 4/12 parsing
- simple and composite glyf outlines
- real metrics and baselines
- accumulation-buffer glyph rasterization
- per-size glyph cache
- bitmap fallback

MM3E currently needs strong debug overlays, labels, AOV annotations, scene inspector UI, and provider/router surfaces. PMRE text solves that better than ad hoc bitmap labels.

### 4. HTML/CSS Snapshot Rendering

The Ordo provider tab screenshot was an HTML snapshot. PMRE's reduced HTML/CSS renderer could render that tab as an engine-native UI without Electron/Tauri/browser dependence.

Useful PMRE support:

- inline element coalescing into rich text
- margins and percentage sizing
- box shadows
- text alignment
- opacity
- font weight
- text decoration
- rgba/hsl/hex/named colors
- list and hr support
- comment and script/style skipping

This strongly applies to the Ordo provider tab.

### 5. Layout And Hit Testing

PMRE's reduced layout solver applies to:

- Ordo provider cards
- model lists
- key wizard steps
- test console panes
- engine debug controls
- MM3E scene inspector
- renderer tuning panels

The session specifically fixed available-width propagation, nested wrapping, flex rows, scroll clamping, stuck hover/drag state, and scrollbar thumb behavior. Those are exactly the kinds of UI bugs that would make Ordo feel unstable.

### 6. Validation Habits

The session's best process result is the adversarial review and hard validation loop:

- tests
- clippy
- visual render checks
- live window launch
- specific bug list
- before/after screenshots

This applies directly to our sim-output archive and should be repeated for model-generated renderer work.

## What Does Not Transfer Directly

### 1. PMRE Does Not Replace MM3E

PMRE is 2D CPU UI rendering. It does not replace:

- MM3E's 3D SDF ray marcher
- MM3E's material and lighting system
- MM3E's GPU scene-to-WGSL path
- MM3E's 3D domain operators
- MM3E's camera/ray pipeline

Use PMRE beside MM3E, not over it.

### 2. Do Not Merge GPU Bloom Into Core Doctrine Blindly

The PMRE README/session repeatedly claims zero dependencies. Local inspection found:

```text
pmre-orchestrator -> wgpu
pmre-orchestrator -> pollster
```

That means the kit is zero-dependency, but the full workspace is not. This matters because Ordo's UI core and MM3E's core both care about dependency boundaries.

Recommended fix:

- Keep `pmre-kit` zero-dependency.
- Move GPU bloom into a separate optional crate, such as `pmre-gpu` or `pmre-wgpu`.
- Or feature-gate GPU bloom so default workspace policy stays honest.
- Update README/docs to distinguish core, orchestrator, examples, and optional GPU acceleration.

This mirrors MM3E's `mm3e-gpu` quarantine pattern.

### 3. HTML/CSS Support Is Reduced, Not Browser-Complete

PMRE can render a useful subset of HTML/CSS, but it should not be treated as a browser.

Good for:

- Ordo-native dashboards
- provider tabs
- docs cards
- settings panels
- inspector overlays

Not yet enough for:

- arbitrary web pages
- complex CSS layout
- full DOM events
- accessibility tree
- web security model

## Specific Cross-Project Uses

### Ordo Provider Tab

PMRE should be considered a renderer for the static provider-tab HTML snapshot.

Best path:

1. Feed provider-tab HTML/CSS into PMRE's reduced parser.
2. Render a static image first.
3. Add hit regions for provider cards, tabs, search, and key wizard.
4. Emit Ordo bus messages from UI events.
5. Let the spiderweb bus route provider/model state changes.

This keeps the UI native and bus-aligned.

### MM3E Renderer

Use PMRE as a 2D overlay/control plane:

- render stats
- model/provider selector
- frame timing
- AOV labels
- scene inspector
- primitive toggles
- raymarch parameter sliders
- GPU adapter selector

Do not fold PMRE layout into MM3E's core marcher.

### Spiderweb Bus

PMRE can be the visible bus dashboard:

- L0 transport status
- L1 message lanes
- L2 flow graph nodes
- L3 orchestration decisions
- vibrations: provider failure, route demotion, backpressure, repair overhead

The provider-tab UI should not only show configured providers. It should show routing health.

### Inference Engine / Model Ecoskeleton

PMRE's provider UI pairs naturally with the model incident log idea:

- model selected
- repair history
- trust tier
- context window
- cost/rate limit
- local/cloud lane
- allowed task classes
- regression count

This turns "which model fixed/broke Ordo" into a first-class routing primitive.

## Important Defect Lessons From The Session

The session's adversarial review found defects worth turning into tests elsewhere:

- malformed font allocation bomb
- stack overflow on deeply nested HTML
- O(n^2) parser paths
- stuck drag/hover state when pointer leaves the window
- scrollbar thumb jump on grab
- scroll offsets not re-clamped after content shrinks
- mixed-size text baseline misalignment
- text overlap from bad available-width propagation

These are not just PMRE bugs. They are a checklist for any Ordo UI, provider tab, or engine overlay.

## Recommended Next Steps

1. Treat PMRE as the 2D UI renderer for Ordo and MM3E overlays.
2. Do not merge PMRE into MM3E core.
3. Port or bridge only through stable surfaces: framebuffer, events, layout tree, and bus messages.
4. Split or feature-gate PMRE GPU bloom so dependency claims are honest.
5. Convert the Ordo provider-tab HTML snapshot into a PMRE render test.
6. Add a provider-tab interaction harness: select provider, search models, open key wizard, run test console.
7. Add model incident tracking to the provider UI.
8. Archive PMRE validation outputs just like the sim outputs before publication.

## Bottom Line

Yes, this applies. The strongest application is not "PMRE replaces MM3E." It is:

```text
MM3E renders the 3D world.
PMRE renders the 2D UI/control surface.
Ordo/spiderweb bus carries the events, provider state, model trust, and routing vibrations between them.
```

The Fable5 session produced real usable work, and local tests/clippy confirm the project is currently healthy. The one serious caution is the dependency boundary: PMRE's core kit is zero-dependency, but the full workspace is not because GPU bloom added `wgpu` and `pollster` to the orchestrator.

