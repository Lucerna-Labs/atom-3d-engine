//! # cap-primitives
//!
//! Renderer-neutral UX primitive vocabulary — the shared language between
//! application logic and renderer backends.
//!
//! This crate defines the **what** of a UI scene: shapes, fills, strokes,
//! text runs, image references, clips, layers, and transforms. It does NOT
//! define the **how** — that's the job of renderer backends like
//! `cap-render-vello` or `cap-render-blade`.
//!
//! ## Architecture
//!
//! ```text
//! App logic → cap-primitives (what to draw) → Renderer backend (how to draw)
//! ```
//!
//! The primitive types here are the scene tree's vocabulary. Any renderer
//! backend translates these into actual draw calls. This means:
//!
//! - Swapping renderers doesn't require changing app code
//! - The scene tree is serializable and testable without a GPU
//! - Multiple renderers can coexist for different primitive types
//!
//! ## Principles
//!
//! - **Renderer-neutral** — no `wgpu`, `vello`, `blade`, `winit` dependencies
//! - **Geometry from `cap-geometry`** — all spatial types come from there
//! - **Serializable intent** — every primitive describes what it looks like,
//!   not how to render it
//! - **Composable** — primitives nest through `Layer` and `Clip`

mod clip;
mod fill;
mod image;
mod layer;
mod primitive;
mod shape;
mod stroke;
mod text;
mod transform;

pub use clip::*;
pub use fill::*;
pub use image::*;
pub use layer::*;
pub use primitive::*;
pub use shape::*;
pub use stroke::*;
pub use text::*;
pub use transform::*;
