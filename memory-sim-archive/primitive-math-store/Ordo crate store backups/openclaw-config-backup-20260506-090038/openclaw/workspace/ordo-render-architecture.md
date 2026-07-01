# Ordo Render Architecture

_Date: 2026-05-04_
_Status: Confirmed architecture decisions_

## Core Decisions

### UXI Stack: Vello + Dioxus

- **Vello** — 2D vector rendering specialist (GPU compute, 9-stage pipeline)
- **Dioxus** — Component model, compiled to WASM, rendered by embedded Servo
- **No WebView2. No Microsoft. No Chromium. No second binary.**

### Embedded Servo

- Ordo embeds Servo as a render surface (not a browser feature — a UI layer)
- The user never sees "Servo." They see Ordo's interface
- Content preview (blog posts, etc.) renders through the same Servo surface
- One binary. No Lucid dependency. No external browser required.

### Peer-Based Rendering Architecture

Rendering is not a monolithic dependency. It is a coordinated subsystem of specialized rendering peers.

```
App emits UI intent
  ↓
Scene builder creates neutral render tree
  ↓
Render coordinator analyzes scene
  ↓
Pass graph assigns work
  ↓
Rendering peers execute specialized passes
  ↓
Composite pass merges output
  ↓
Surface manager presents final frame
  ↓
Health system monitors failures
```

### The RenderPass Trait

```rust
trait RenderPass {
    fn name(&self) -> &'static str;
    fn prepare(&mut self, context: &mut RenderContext);
    fn render(&mut self, frame: &mut FrameTarget);
    fn fallback(&mut self, reason: RenderFailure);
}
```

Every rendering peer implements this trait. The coordinator runs them in order. If a pass fails, fallback keeps the app usable.

### Capability Model

Rendering peers receive capabilities, not ownership.

```rust
struct RenderCapabilities {
    can_write_color_target: bool,
    can_read_shared_textures: bool,
    can_allocate_buffers: bool,
    can_compile_shaders: bool,
    can_present_frame: bool,
}
```

Vello doesn't own the GPU. Dioxus doesn't own the GPU. The coordinator owns the GPU context. Peers borrow capabilities.

### Resource Ownership

```
SurfaceManager → owns the window/swapchain
GpuContext      → owns device and queue
ResourceRegistry → owns shared textures/buffers
RenderCoordinator → grants scoped access
Rendering peers → receive capabilities, not ownership
```

### Failure Handling

If Vello pass fails:
1. Coordinator disables advanced vector decorations
2. BasicPanelPass draws simpler rectangles
3. App remains usable
4. Health system logs the failure

Same resilience model as the bus: a component can fail without taking the whole system down.

---

## Ordo vs Lucid Scope

### Ordo (the brain)

Minimal coordinator. Two peers.

```
RenderBus
├── RenderCoordinator (light)
├── SurfaceManager
├── GpuContext
├── PassGraph
│   ├── VelloVectorPass
│   └── TextPass
└── FallbackManager (simple rects)
```

Ordo's UXI is a control panel: dashboards, status, configuration, content preview. No custom GPU pipelines. No video surfaces. No shader effects.

### Lucid (the browser)

Full coordinator. Multiple peers.

```
RenderBus
├── SceneBuilder
├── RenderCoordinator (full)
├── SurfaceManager
├── GpuContext
├── ResourceRegistry
├── PassGraph
│   ├── BackgroundPass
│   ├── VelloVectorPass
│   ├── BladeCustomPass
│   ├── TextPass
│   ├── MediaPass
│   └── OverlayPass
├── FallbackManager
└── RenderHealthMonitor
```

Lucid loads WebGPU pages, video elements, WebGL canvases. That's when Blade, media backends, and the full coordinator earn their place.

---

## Anti-Patterns

These violate the architecture. Do not do them.

- ❌ **WebView2 / Microsoft WebView** — closed-source dependency, platform lock-in
- ❌ **Two executables** — UXI is a bus node, not a separate program
- ❌ **Chromium embedding** — same problem as WebView2, different vendor
- ❌ **Single renderer ownership** — no peer should own the GPU context
- ❌ **Bypassing the scene tree** — app emits intent, not render commands
- ❌ **CSS injection for theming** — use the rendering system, not hacks on top of it

---

## The Big Principle

> Do not choose between rendering backends too early.
> Build the layer that makes choosing less permanent.

The coordinator, the scene tree, and the RenderPass trait make it cheap to add, remove, or replace any rendering peer. That's the architectural guarantee.

---

## Sequencing

1. **Now:** Coordinator interfaces + Vello + Text peer for Ordo's UXI
2. **Next:** Dioxus components compiled to WASM, rendered by embedded Servo
3. **Later:** Full coordinator for Lucid (Blade, media, overlay passes)
4. **Future:** Neural rendering peers as GPU hardware evolves

Build the interfaces now. Implement only the peers you need. The architecture is designed to make adding peers cheap — that only works if you actually start cheap and add later.

---

## Source Research

- Deep Research: Rust UI Architecture (2026-05-04)
- Peer-Based Rendering Architecture thesis (Jesse, 2026-05-04)
- Architecture conversation with Alex (2026-05-04, 00:00–10:07 EDT)