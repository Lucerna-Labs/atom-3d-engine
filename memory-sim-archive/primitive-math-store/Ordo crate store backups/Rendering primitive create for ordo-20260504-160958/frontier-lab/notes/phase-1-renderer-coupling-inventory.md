# Phase 1 Renderer Coupling Inventory

Date: 2026-05-04

## Scope

This audit covers the current workspace:

`C:\Projects\Rendering primitive create for ordo`

This workspace currently contains the modular Ordo UX extraction/experiment crates and the `frontier-lab/` folder. It does not currently contain the old renderer implementation that needs to be removed from the related project.

## Summary

No hard renderer or windowing dependency was found in the current Cargo workspace.

No direct dependencies were found on:

- `vello`
- `wgpu`
- `winit`
- `tauri`
- `dioxus`
- `electron`
- `blade`
- `skia`
- `ash`
- `bevy`
- `egui`
- `iced`
- `slint`
- `xilem`
- `masonry`

The current crates are already mostly renderer-neutral. They describe data, state, layout, style, input, material intent, and primitive output. They do not create windows, own GPU devices, run event loops, or issue draw calls.

## Actual Dependency Findings

Current external dependencies are limited to:

- `kurbo`
- `peniko`
- optional `serde` in `ordo-ux-primitives`

Internal crate dependencies are experimental and local. Examples:

- `ordo-ux-motion-experiment` depends on `ordo-ux-curves-experiment`
- `ordo-ux-transitions-experiment` depends on `ordo-ux-motion-experiment`
- `ordo-ux-microinteractions-experiment` depends on `ordo-ux-motion-experiment`
- `ordo-ux-menus-experiment` depends on `ordo-ux-overlays-experiment`
- most visual/data experiments depend on `ordo-ux-primitives`

No backend crate is currently present.

## Source-Level Backend Mentions

The only direct mentions of backend names in Rust source are intentional documentation boundaries:

- `crates/ordo-ux-primitives/src/lib.rs`
  - says renderer backends translate primitives into draw calls
  - says Vello should live later in a separate crate such as `ordo-ux-vello`

- `crates/ordo-ux-primitives/src/primitive.rs`
  - says `Primitive` intentionally avoids Vello, WGPU, Winit, and windowing concepts

These are not coupling. They are anti-coupling documentation.

## Renderer-Adjacent Concepts Found

Several crates intentionally contain renderer-adjacent vocabulary. These are not current coupling, but they are important future bridge points.

### `ordo-ux-primitives`

Renderer-neutral draw vocabulary:

- `Primitive`
- `Shape`
- `Fill`
- `Stroke`
- `TextRun`
- `ImageRef`
- `Clip`
- `Layer`
- `Transform`
- `Bounds`
- `HitRegion`
- `ThemeTokens`

Notes:

- `Stroke::to_kurbo_stroke()` is a geometry conversion helper, not a renderer backend dependency.
- `ImageRef` uses an app-defined string id, not a backend texture handle.
- `TextRun` carries font intent, not shaped glyphs or font backend handles.

### `ordo-ux-textures-experiment`

Material intent vocabulary:

- `TextureEffectHint::BackdropBlur`
- `TextureEffectHint::ProceduralNoise`
- `TextureEffectHint::DirectionalBands`
- `TextureEffectHint::GradientMask`
- `GlassSpec::blur_radius`

These are backend-neutral effect hints. They will matter when a renderer backend is introduced because the backend will need to decide how, or whether, to implement each hint.

Risk level: medium, future bridge point.

Recommendation: keep these as declarative intent. Do not add shader code, GPU passes, or renderer-specific blend types here.

### `ordo-ux-input-experiment`

Input is normalized into local data types:

- `InputEvent`
- `PointerButton`
- `FocusOrder`
- `GestureState`

No Winit or platform input types are present.

Risk level: low now, important future boundary.

Recommendation: platform/window crates should translate into these input events; this crate should not import platform event types.

### `ordo-ux-overlays-experiment` and `ordo-ux-menus-experiment`

These crates generate primitive layers and hit regions for UI surfaces.

No renderer/window coupling found.

Risk level: low.

Recommendation: keep focus trapping and event routing outside overlays/menus until a dedicated event-router crate exists.

## Current Workspace Status

The current workspace is already a renderer-neutral extraction surface. It has no renderer to remove because the renderer-bearing source has not been added or pointed to yet.

This means Phase 1 has two tracks:

1. Audit this modular workspace for accidental renderer coupling.
2. Audit the related/current renderer project once its path or source is available.

Track 1 is complete.

Track 2 is blocked until the renderer-bearing project/module is provided.

## Coupling Categories For The Real Renderer Project

When auditing the current renderer project, classify findings into these buckets.

### P0: Hard Backend Dependencies

Examples:

- `vello::...` in core UI APIs
- `wgpu::Device`, `wgpu::Queue`, `wgpu::Texture`, `wgpu::Surface`
- `winit::window::Window`
- renderer scene types in public app/component signatures

Action:

- Must be extracted or hidden behind backend crates.

### P1: Renderer Handles In Shared Types

Examples:

- image types that store GPU texture handles
- font types that store renderer font handles
- layout nodes that own renderer layers
- UI nodes that own backend resources

Action:

- Replace with neutral references such as asset ids, resource ids, or primitive/image/font intent.

### P2: Draw Calls Mixed With UI Logic

Examples:

- component code directly calls `draw_*`
- layout code emits backend commands
- state transitions directly mutate renderer scenes

Action:

- Move draw-call translation into backend crates.
- Make UI code emit `Primitive` or scene data.

### P3: Window/Event Loop Leakage

Examples:

- UI logic takes `winit::event::Event`
- components depend on window scale factor directly
- app state owns window handles

Action:

- Normalize into input/viewport/shell contracts.

### P4: Backend-Specific Styling

Examples:

- shader names in core style
- backend blend modes in theme tokens
- renderer-specific blur/material parameters as required types

Action:

- Convert to intent/hints; implement in backend crates.

### P5: Documentation-Only Mentions

Examples:

- docs saying a future Vello backend should live elsewhere
- comments saying a crate must not depend on WGPU

Action:

- No extraction needed.

## Search Patterns Used

Backend/dependency names:

- `vello`
- `wgpu`
- `winit`
- `tauri`
- `dioxus`
- `electron`
- `blade`
- `skia`
- `ash`
- `bevy`
- `egui`
- `iced`
- `slint`
- `xilem`
- `masonry`

Renderer concepts:

- `Renderer`
- `Render`
- `render_pass`
- `draw_call`
- `DrawCall`
- `Device`
- `Queue`
- `Swapchain`
- `SwapChain`
- `Surface`
- `Shader`
- `Pipeline`
- `TextureHandle`
- `Gpu`
- `GPU`

## Next Step

Point this audit at the renderer-bearing project or module.

Useful inputs:

- path to the related project root
- path to the current renderer crate/module
- current renderer name
- whether the renderer is Vello, WGPU, custom, or something else

Once available, Phase 1 should produce a second report:

`phase-1-current-renderer-coupling-inventory.md`

That report should list exact files, lines, types, dependencies, and recommended extraction targets.
