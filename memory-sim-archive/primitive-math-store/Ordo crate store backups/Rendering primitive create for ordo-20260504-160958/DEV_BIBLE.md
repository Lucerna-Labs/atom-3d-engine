# Ordo UX Crate Store Dev Bible

This workspace is a crate store for Ordo UX ideas.

Each crate should have one clear job, a small public surface, and a clean boundary. Some crates are production foundations. Some are experiments. The point is to let ideas grow without turning every idea into the core.

## North Star

Ordo UX should be built from small renderer-neutral building blocks first, then renderer, windowing, app, and tooling layers later.

The primitive language describes what the UI is. Renderer backends decide how to draw it. Dev tools may help shape it visually, but they must be easy to disable before shipping.

## Workspace Rules

- Keep crates under `crates/`.
- Keep high-risk architecture sketches under `frontier-lab/` until promoted.
- Prefer focused crate names: `ordo-ux-primitives`, `ordo-ux-dnd-experiment`, `ordo-ux-vello`.
- A crate should do one thing well.
- Avoid adding app frameworks to foundational crates.
- Keep production contracts separate from experiments.
- Use feature flags for optional behavior.
- Default features should be conservative and ship-safe.
- Run `cargo fmt`, `cargo check`, and `cargo test` before considering a crate healthy.

## Current Crates

### `cap-geometry`

Purpose: clean-room geometry capability extracted from the Zed deconstruction work.

This crate provides renderer-neutral spatial types such as pixels, points, sizes, bounds, edges, corners, axes, and length units. It is useful reference material for Ordo's own geometry vocabulary and may later influence or merge with Ordo primitive/layout foundations.

Allowed here:

- pure geometry types
- DPI-aware pixel units
- bounds math
- box-model helpers
- layout length units

Not allowed here:

- rendering
- windowing
- app framework state
- backend-specific coordinate handles

### `cap-keymap`

Purpose: clean-room keymap capability extracted from the Zed deconstruction work.

This crate owns context-aware key binding resolution, multi-key sequences, and action identifiers. It should remain pure dispatch logic: it maps normalized key inputs and active contexts to actions without owning windows, views, renderers, or platform event loops.

Allowed here:

- key stroke representations
- key binding registration
- context predicates
- binding precedence
- disabled/unbound action logic

Not allowed here:

- OS keyboard APIs
- Winit events
- focus tree ownership
- action handler execution
- renderer or window dependencies

### `cap-primitives`

Purpose: clean-room primitive capability extracted from the Zed deconstruction work.

This crate is a renderer-neutral primitive vocabulary built on `cap-geometry`. It can coexist with `ordo-ux-primitives` while we compare designs and decide which ideas belong in the Ordo production vocabulary.

Allowed here:

- shapes
- fills
- strokes
- text intent
- image references
- clips
- layers
- transforms

Not allowed here:

- renderer backends
- GPU resources
- platform windows
- text shaping engines
- product-stable Ordo API promises without a promotion pass

### `ordo-ux-primitives`

Purpose: Ordo's shared renderer-neutral UX primitive language.

This crate defines data structures that future renderers can consume. It must not render, create windows, own GPU state, or depend on backend-specific event systems.

Allowed here:

- primitive data types
- geometry and bounds
- paint descriptions
- text/image references
- layers, clips, transforms
- hit regions
- theme tokens
- lightweight helpers for building primitive trees

Not allowed here:

- Vello
- WGPU
- Winit
- Dioxus
- Tauri
- Electron
- renderer command buffers
- window creation
- OS event loops

Future renderer crates, such as `ordo-ux-vello`, should translate these primitives into real draw calls.

### `ordo-ux-dnd-experiment`

Purpose: dev-mode internal drag and drop experiments for visually editing UX layouts.

This crate is for the surgical visual customization idea: moving UI pieces around during development and emitting primitive overlays that can be drawn by any renderer later.

It should remain clearly dev-oriented. Shipping apps should either avoid this crate, avoid enabling dev-only features, or pass `DesignModeConfig::disabled()`.

Allowed here:

- drag/drop state machines
- draggable element metadata
- drop zone logic
- snap-grid experiments
- overlay primitives
- design-mode interaction events

Not allowed here yet:

- real windows
- OS file drag/drop
- renderer backends
- persistence formats that pretend to be stable product contracts

### `ordo-ux-pulse-experiment`

Purpose: pulsing indicators, attention lights, and primitive-based button experiments.

This crate explores small UX components that produce primitive trees from explicit state and time inputs. It should not own animation loops, clocks, renderers, or windows.

Allowed here:

- pulse timing models
- indicator light primitives
- button state primitives
- semantic tones
- focus, hover, pressed, and disabled visuals
- renderer-neutral component output

Not allowed here yet:

- renderer animation loops
- timers or task schedulers
- window events
- GPU effects
- product-stable component APIs without a promotion pass

### `ordo-ux-branding-experiment`

Purpose: branding, logo marks, wordmarks, lockups, and brand theme experiments.

This crate explores brand assets as primitive trees. It should describe marks and lockups in renderer-neutral data, not generate bitmap logos or depend on a renderer.

Allowed here:

- brand palettes
- logo mark builders
- wordmark primitives
- lockup layout rules
- clearspace and protected bounds
- brand-to-theme token mapping

Not allowed here yet:

- bitmap/logo image generation
- SVG import/export as the core contract
- renderer-specific text shaping
- product-stable brand guidelines without a promotion pass

### `ordo-ux-color-experiment`

Purpose: exhaustive, customizable, adjustable, living color-system experiments.

This crate explores modern color palettes that can feel alive through explicit time-sampled motion. It should define semantic roles, ramps, presets, and adjustment knobs, not own animation loops or renderer effects.

Allowed here:

- semantic color roles
- exhaustive token sets
- modern palette presets
- color ramps
- global palette adjustments
- living color motion sampled from explicit time
- theme-token conversion

Not allowed here yet:

- renderer-owned glow effects
- animation loops or timers
- OS/theme integration as a hard dependency
- product-stable brand color contracts without a promotion pass

### `ordo-ux-overlays-experiment`

Purpose: renderer-neutral tooltip, popover, modal, panel, command palette, and HUD overlay experiments.

This crate explores overlay placement, surface chrome, scrims, arrows, and stack ordering. It should emit primitive trees and keep focus trapping, event routing, and rendering outside the crate.

Allowed here:

- overlay kinds
- viewport-aware placement
- overlay chrome
- modal scrims
- tooltip/popover arrows
- overlay stack ordering
- primitive output for overlay surfaces

Not allowed here yet:

- focus trapping
- platform window management
- renderer-specific shadows or blur passes
- event loops
- product-stable overlay APIs without a promotion pass

### `ordo-ux-typography-experiment`

Purpose: exhaustive, customizable, renderer-neutral typography-system experiments.

This crate explores semantic type roles, font stacks, type scales, OpenType feature intent, variable-font axes, and text primitive builders. It should not shape glyphs, load fonts, render text, or bind to platform text systems.

Allowed here:

- semantic typography roles
- customizable type styles
- font stacks
- type scale presets
- density/weight/spacing adjustments
- OpenType feature intent
- variable-font axis metadata
- text primitive builders

Not allowed here yet:

- font loading
- glyph shaping
- line breaking engines
- renderer-specific text caches
- platform font discovery as a hard dependency
- product-stable typography contracts without a promotion pass

### `ordo-ux-layouts-experiment`

Purpose: renderer-neutral UXI layout composition experiments for app shells, dashboards, editors, side carts, bars, rails, panels, and floating regions.

This crate explores developer-friendly layout recipes that compute concrete bounds for named regions. It should not mount UI components, render, own windows, or become a full app framework.

Allowed here:

- named layout slots
- top/bottom bars
- left/right rails
- side carts and panels
- docked vs floating regions
- responsive breakpoints
- content headers and footers
- resolved bounds
- debug primitive output

Not allowed here yet:

- component mounting
- renderer layout passes
- window management
- flex/grid engines as hard dependencies
- product-stable layout APIs without a promotion pass

### `ordo-ux-menus-experiment`

Purpose: renderer-neutral dropdown menu, submenu, and menu navigation experiments.

This crate explores menu trees, item states, submenu opening, dropdown primitive output, and keyboard-like navigation helpers. It should not own platform focus, global shortcuts, input listeners, or action execution.

Allowed here:

- dropdown menu trees
- submenu children
- command, checkbox, radio, separator, and submenu items
- disabled and highlighted item state
- shortcut display metadata
- open/highlight navigation state
- primitive output for open menus

Not allowed here yet:

- platform event listeners
- focus ownership
- action execution
- OS menu integration
- product-stable menu APIs without a promotion pass

### `ordo-ux-textures-experiment`

Purpose: renderer-neutral texture and material recipe experiments for film grain, glass, scanlines, sheen, paper, metal, glow, and vignette treatments.

This crate explores material intent and fallback primitive approximations. It should describe texture layers and effect hints without owning shaders, images, render targets, GPU passes, or renderer-specific effect code.

Allowed here:

- material recipes
- texture layers
- noise settings
- glass/backdrop blur intent
- directional band settings
- effect hints
- deterministic fallback primitives
- material token systems

Not allowed here yet:

- shader code
- GPU render passes
- generated bitmap textures
- image asset pipelines
- renderer-specific blur or blend implementations
- product-stable material APIs without a promotion pass

### `ordo-ux-curves-experiment`

Purpose: renderer-neutral animation curve experiments.

Allowed here: easing curves, cubic Bezier sampling, spring approximations, stepped curves, interpolation helpers.

Not allowed here: frame scheduling, clocks, renderer animation passes, or component state ownership.

### `ordo-ux-motion-experiment`

Purpose: renderer-neutral motion timeline experiments.

Allowed here: explicit time snapshots, scalar tracks, motion tokens, named motion values.

Not allowed here: animation loops, timers, renderer callbacks, or mutation of UI state.

### `ordo-ux-transitions-experiment`

Purpose: renderer-neutral visual transition intent experiments.

Allowed here: transition properties, channels, fade/slide recipes, sampled transition values.

Not allowed here: renderer commands, component mounting, or lifecycle ownership.

### `ordo-ux-microinteractions-experiment`

Purpose: renderer-neutral micro-interaction feedback experiments.

Allowed here: hover, press, focus, drag, disabled feedback values such as scale, elevation, opacity, and focus intensity.

Not allowed here: input event listening, rendering, or action execution.

### `ordo-ux-input-experiment`

Purpose: renderer-neutral input handling experiments.

Allowed here: normalized input events, hit testing against `HitRegion`, focus traversal, and simple gesture recognition.

Not allowed here: windows, platform event loops, device ownership, IME integration, or OS-specific input APIs.

## Production vs Experiment

Production crates define stable contracts. Experiment crates test ideas.

Experiment crates should make their status obvious in the crate name or docs. They can move faster, but they should still compile, test, and keep their dependencies honest.

Before promoting an experiment into production:

- rename or extract it into a non-experiment crate
- reduce the public API
- document the contract
- add tests for edge cases
- check that default behavior is safe for shipping

## Frontier Lab

`frontier-lab/` is for architecture work that is more difficult than a normal experiment.

Use it for ideas that cross multiple crates, need research, or may change the shape of Ordo UX itself. Frontier crates should start outside the root Cargo workspace. Promote them into `crates/` only when their boundary becomes clear.

The crate-by-crate frontier roadmap lives in `frontier-lab/CRATE_LAYOUT.md`.

## Renderer Boundary

Renderer-neutral crates may describe:

- shapes
- colors and brushes
- bounds
- transforms
- images by reference
- text runs by intent
- layers and clips
- hit-test metadata

Renderer-neutral crates may not own:

- GPU devices
- swapchains
- windows
- platform event loops
- renderer-specific scenes
- backend-specific resource handles

Renderer crates may translate primitives into backend-specific calls. For example, a future `ordo-ux-vello` crate may depend on Vello and know how to turn `Primitive` values into Vello scene operations.

## Dev-Mode Visual Editing

The drag/drop design-mode idea is allowed, but it must be ship-safe.

Design-mode tools should:

- default to disabled
- expose explicit runtime switches
- use feature flags when behavior grows heavier
- keep interaction state separate from production layout state
- emit primitive overlays instead of rendering directly
- avoid mutating product layouts unless the caller explicitly applies the change

Design-mode tools should not silently ship as active behavior.

## Dependency Policy

Foundational crates should use lightweight dependencies only when they clarify the model.

Good examples:

- `kurbo` for geometry
- `peniko` for color and brush-like concepts
- optional `serde` behind a feature flag

Avoid in foundational crates:

- app frameworks
- renderer backends
- windowing crates
- heavy async stacks unless the crate truly owns async behavior

## Naming Patterns

Use names that make the layer obvious:

- `ordo-ux-primitives`: shared vocabulary
- `ordo-ux-layout`: layout contracts or algorithms
- `ordo-ux-hit`: hit testing, if it grows beyond primitives
- `ordo-ux-vello`: Vello renderer backend
- `ordo-ux-winit`: Winit shell or event adapter
- `ordo-ux-*-experiment`: ideas not yet promoted

## Test Expectations

Every crate should have at least basic tests for its public contract.

For primitive/data crates, test:

- constructors
- defaults
- bounds inference
- feature-gated behavior
- conversion helpers

For interaction crates, test:

- disabled behavior
- lifecycle events
- edge cases
- cancellation
- locked or inactive states
- output primitives

## Release Checklist

Before a crate is considered ready:

- `cargo fmt`
- `cargo check`
- `cargo test`
- check dependency tree with `cargo tree -p <crate>`
- verify docs say what layer the crate belongs to
- verify forbidden dependencies are not present
- verify dev-only behavior is disabled by default

## Guiding Taste

Prefer small, boring contracts with enough expressive power to grow.

The core should feel calm. The experiments can be playful. The boundary between them should be bright.
