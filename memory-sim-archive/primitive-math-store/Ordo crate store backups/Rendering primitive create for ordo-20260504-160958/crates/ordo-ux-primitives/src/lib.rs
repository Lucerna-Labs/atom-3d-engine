//! Renderer-neutral UX primitives for Ordo.
//!
//! This crate defines Ordo's shared UX primitive language. It does not render
//! anything, create windows, or bind to a graphics backend.
//!
//! Renderer backends translate these primitives into actual draw calls. A Vello
//! implementation should live in a separate backend crate later, such as
//! `ordo-ux-vello`.

pub mod bounds;
pub mod hit;
pub mod image;
pub mod layer;
pub mod paint;
pub mod primitive;
pub mod shape;
pub mod text;
pub mod theme;
pub mod transform;

pub use bounds::Bounds;
pub use hit::{CursorHint, HitRegion};
pub use image::{ImageFit, ImageRef};
pub use layer::Layer;
pub use paint::{Fill, FillRule, LineCap, LineJoin, Paint, Stroke};
pub use primitive::Primitive;
pub use shape::{Clip, Shape};
pub use text::{FontStyle, TextAlign, TextRun};
pub use theme::ThemeTokens;
pub use transform::Transform;
