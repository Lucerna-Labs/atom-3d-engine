//! # cap-keymap
//!
//! A renderer-neutral keymap system with context-aware binding resolution,
//! multi-key sequences, and precedence-based dispatch.
//!
//! This crate extracts the keymap architecture from Zed's GPUI, removing all
//! rendering and UI framework dependencies. It provides the core logic for:
//!
//! - **Binding registration** — declare keybindings with optional context predicates
//! - **Context resolution** — match bindings against a stack of active contexts
//! - **Multi-key sequences** — support for chords like `ctrl-x ctrl-s`
//! - **Precedence ordering** — deeper contexts override shallower ones; later
//!   bindings override earlier ones; user bindings override defaults
//! - **Disable/unbind** — support for `NoAction` and targeted `Unbind` to
//!   disable specific bindings in specific contexts
//!
//! ## Architecture
//!
//! The keymap is the **contract** between user input and action dispatch. It does
//! not know about windows, views, or renderers. It knows about keystrokes, actions,
//! and contexts. Whatever system consumes this crate is responsible for wiring
//! resolved actions to their handlers.
//!
//! ```text
//! User input → Keymap::bindings_for_input() → Vec<KeyBinding>
//!                                                 ↓
//!                                           Consumer dispatches actions
//! ```

mod binding;
mod context;
mod keymap;
mod keystroke;

pub use binding::*;
pub use context::*;
pub use keymap::*;
pub use keystroke::*;
