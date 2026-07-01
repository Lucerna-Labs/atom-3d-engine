# Frontier Lab Crate Layout

This is a lateral roadmap for the harder Ordo UX architecture work. These crates are not created yet. They are proposed lanes we can work through in slices.

## Lateral Passes

Do not build this top-to-bottom like a waterfall. Work across crates in passes.

### Pass 1: Contracts

Define the public types each crate would own. Avoid implementation depth.

### Pass 2: Simulations

Add small in-memory tests: state changes, event routing, layout resolution, scene diffs, render graph planning.

### Pass 3: Prototype Bridges

Connect one or two crates together with fake adapters. No real windowing or renderer yet unless the crate explicitly owns that layer.

### Pass 4: Backend Spike

Only after contracts feel good, spike a backend such as Vello in its own crate.

### Pass 5: Promotion

Move mature crates out of `frontier-lab/` into root `crates/` one at a time.

## Proposed Crate Map

### `ordo-ux-kernel-lab`

Owns the core runtime vocabulary for a full Ordo UX system.

Responsibilities:

- app lifecycle phases
- runtime capabilities
- frame phases
- shared IDs
- diagnostics hooks

Does not own:

- renderer backends
- windows
- component frameworks

### `ordo-ux-app-graph-lab`

Owns a persistent graph of UI entities and relationships.

Responsibilities:

- node IDs
- parent/child relationships
- slots
- diffable tree or graph edits
- stable identity for dev tooling

Pairs with:

- `ordo-ux-state-lab`
- `ordo-ux-devtools-lab`
- `ordo-ux-scene-lab`

### `ordo-ux-state-lab`

Owns reactive or signal-like state experiments.

Responsibilities:

- state cells
- dependency tracking
- invalidation
- subscriptions
- transaction boundaries

Hard questions:

- How much reactivity should Ordo own?
- Can this stay framework-neutral?
- How do we debug causality?

### `ordo-ux-style-cascade-lab`

Owns a style cascade over tokens, themes, component state, and dev overrides.

Responsibilities:

- style rules
- token resolution
- state variants
- inheritance
- override layers
- dev-mode style patching

Builds on:

- `ordo-ux-color-experiment`
- `ordo-ux-typography-experiment`
- `ordo-ux-textures-experiment`

### `ordo-ux-constraint-layout-lab`

Owns advanced layout beyond the current slot-based layout experiments.

Responsibilities:

- constraints
- anchors
- intrinsic sizing
- responsive rules
- measured layouts
- layout invalidation

Does not own:

- rendering
- component mounting

### `ordo-ux-scene-lab`

Owns the retained renderer-neutral scene representation.

Responsibilities:

- scene nodes
- primitive ordering
- clipping hierarchy
- transforms
- dirty regions
- scene diffs

Builds on:

- `ordo-ux-primitives`
- `ordo-ux-app-graph-lab`

### `ordo-ux-render-graph-lab`

Owns backend-neutral render planning.

Responsibilities:

- render passes as data
- surfaces/layers as abstract targets
- effect pass intent
- texture/material effect hints
- dependency ordering

Does not own:

- WGPU devices
- Vello scene execution
- swapchains

### `ordo-ux-vello-lab`

A future backend spike for Vello.

Responsibilities:

- translate primitives or scene nodes to Vello
- prototype text/image/material handling
- measure backend assumptions

Rule:

- This is the place Vello may live, not in `ordo-ux-primitives`.

### `ordo-ux-window-shell-lab`

Owns platform window shell experiments.

Responsibilities:

- window lifecycle
- viewport changes
- scale factor
- platform event normalization
- shell-to-input bridge

Possible backend:

- Winit, later, if intentionally chosen.

### `ordo-ux-event-router-lab`

Owns event routing from normalized input into UI graph targets.

Responsibilities:

- capture/bubble phases
- pointer capture
- focus routing
- keyboard routing
- gesture handoff

Builds on:

- `ordo-ux-input-experiment`
- `ordo-ux-app-graph-lab`

### `ordo-ux-accessibility-lab`

Owns accessibility tree contracts.

Responsibilities:

- semantic roles
- labels/descriptions
- state
- focusable order
- actions
- mapping from UI graph to accessibility tree

Does not own:

- platform accessibility APIs yet

### `ordo-ux-devtools-lab`

Owns the visual editing and inspection experience.

Responsibilities:

- selection model
- inspectors
- drag handles
- style overrides
- layout overlays
- primitive inspection
- record/replay

Builds on:

- `ordo-ux-dnd-experiment`
- `ordo-ux-layouts-experiment`
- `ordo-ux-overlays-experiment`
- `ordo-ux-app-graph-lab`

### `ordo-ux-document-lab`

Owns serialization and project documents.

Responsibilities:

- save/load of UI graphs
- style/theme documents
- scene snapshots
- migration metadata
- diff-friendly document formats

Hard question:

- Which data becomes stable enough to persist?

### `ordo-ux-assets-lab`

Owns asset references and asset pipeline contracts.

Responsibilities:

- image handles
- font handles
- material references
- asset manifests
- lazy loading contracts

Does not own:

- GPU uploads
- renderer textures

### `ordo-ux-animation-runtime-lab`

Owns runtime animation orchestration.

Responsibilities:

- animation timelines
- keyed animations
- interruption behavior
- transition coordination
- frame sampling

Builds on:

- `ordo-ux-curves-experiment`
- `ordo-ux-motion-experiment`
- `ordo-ux-transitions-experiment`
- `ordo-ux-microinteractions-experiment`

### `ordo-ux-plugin-lab`

Owns extension/plugin experiments.

Responsibilities:

- plugin capabilities
- sandbox boundaries
- command registration
- inspector extensions
- devtool extensions

Hard question:

- Can plugins customize UX safely without owning core runtime state?

### `ordo-ux-test-harness-lab`

Owns testing infrastructure for the whole system.

Responsibilities:

- primitive snapshots
- scene diffs
- layout assertions
- event replay
- golden render hooks later

Does not own:

- renderer implementation

## Suggested Lateral Work Order

### Slice A: Identity And Graph

- `ordo-ux-kernel-lab`
- `ordo-ux-app-graph-lab`
- `ordo-ux-state-lab`

Goal: represent UI identity and updates without rendering.

### Slice B: Style And Layout

- `ordo-ux-style-cascade-lab`
- `ordo-ux-constraint-layout-lab`
- `ordo-ux-document-lab`

Goal: derive concrete visual/layout intent from tokens, state, and saved data.

### Slice C: Scene And Render Planning

- `ordo-ux-scene-lab`
- `ordo-ux-render-graph-lab`
- `ordo-ux-test-harness-lab`

Goal: produce inspectable scene/render plans without a backend.

### Slice D: Input And Interaction

- `ordo-ux-window-shell-lab`
- `ordo-ux-event-router-lab`
- `ordo-ux-accessibility-lab`

Goal: normalize input and accessibility contracts without committing to app framework.

### Slice E: Devtools And Backend Spike

- `ordo-ux-devtools-lab`
- `ordo-ux-animation-runtime-lab`
- `ordo-ux-vello-lab`

Goal: make the system visible, editable, animated, and eventually drawable.

## Promotion Criteria

A frontier crate can move into root `crates/` when:

- its boundary is stable
- tests explain its contract
- it has no accidental backend dependency
- it does not duplicate an existing crate
- its relationship to the primitive vocabulary is clear
- it is useful even before a full app exists
