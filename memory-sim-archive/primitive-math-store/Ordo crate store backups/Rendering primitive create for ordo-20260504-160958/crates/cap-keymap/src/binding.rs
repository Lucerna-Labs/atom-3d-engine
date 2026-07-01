//! Key binding definition and matching.

use crate::{AsKeystroke, KeyBindingContextPredicate, KeybindingKeystroke, Keystroke};
use smallvec::SmallVec;
use std::fmt;

/// A unique identifier for metadata associated with a key binding.
/// Intended as an index into a user-defined store (e.g., binding source).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyBindingMetaIndex(pub u32);

/// The action trait — what a key binding dispatches.
///
/// This is the consumer's contract. The keymap does not define specific actions.
/// Whatever system uses `cap-keymap` implements this trait for its own action types.
pub trait Action: Any + Send {
    /// Clone the action into a boxed trait object.
    fn boxed_clone(&self) -> Box<dyn Action>;

    /// Partial equality — used to match bindings to actions.
    fn partial_eq(&self, other: &dyn Action) -> bool;

    /// The action's type name.
    fn name(&self) -> &'static str;

    /// Access to the `Any` type for downcasting.
    fn as_any(&self) -> &dyn Any;
}

use std::any::Any;

/// A sentinel action meaning "this binding is explicitly disabled."
///
/// When a binding maps to `NoAction`, the keymap treats that keystroke as
/// unbound in the matching context. This allows user keymaps to disable
/// default bindings.
#[derive(Debug, Clone)]
pub struct NoAction;

impl Action for NoAction {
    fn boxed_clone(&self) -> Box<dyn Action> {
        Box::new(NoAction)
    }
    fn partial_eq(&self, _other: &dyn Action) -> bool {
        false
    }
    fn name(&self) -> &'static str {
        "NoAction"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A sentinel action that unbinds a specific action by name.
///
/// Unlike `NoAction` (which disables all bindings on a keystroke),
/// `Unbind` targets a specific action. This allows disabling one
/// action on a keystroke while leaving others active.
#[derive(Debug, Clone)]
pub struct Unbind(pub String);

impl Action for Unbind {
    fn boxed_clone(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
    fn partial_eq(&self, _other: &dyn Action) -> bool {
        false
    }
    fn name(&self) -> &'static str {
        "Unbind"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A key binding: a keystroke sequence mapped to an action, with an optional
/// context predicate.
///
/// # Example
///
/// ```ignore
/// KeyBinding::new("ctrl-s", SaveAction {}, Some("editor"))
/// KeyBinding::new("space w w", CloseWindowAction {}, Some("workspace"))
/// ```
pub struct KeyBinding {
    /// The action this binding dispatches
    pub(crate) action: Box<dyn Action>,
    /// The keystroke sequence (supports chords like `ctrl-x ctrl-s`)
    pub(crate) keystrokes: SmallVec<[KeybindingKeystroke; 2]>,
    /// The context predicate — only active when this matches
    pub(crate) context_predicate: Option<KeyBindingContextPredicate>,
    /// Metadata index for source tracking (user vs default vs vim, etc.)
    pub(crate) meta: Option<KeyBindingMetaIndex>,
    /// The original action string from the keymap JSON, if any
    pub(crate) action_input: Option<String>,
}

impl Clone for KeyBinding {
    fn clone(&self) -> Self {
        KeyBinding {
            action: self.action.boxed_clone(),
            keystrokes: self.keystrokes.clone(),
            context_predicate: self.context_predicate.clone(),
            meta: self.meta,
            action_input: self.action_input.clone(),
        }
    }
}

impl KeyBinding {
    /// Create a keybinding from a keystroke string, action, and optional context.
    ///
    /// Panics on invalid keystroke format.
    pub fn new<A: Action>(keystrokes: &str, action: A, context: Option<&str>) -> Self {
        let context_predicate = context
            .map(|c| KeyBindingContextPredicate::parse(c).expect("invalid context predicate"));

        let keystrokes: SmallVec<[KeybindingKeystroke; 2]> = keystrokes
            .split_whitespace()
            .map(|source| {
                let ks = Keystroke::parse(source).expect("invalid keystroke");
                KeybindingKeystroke::new(ks)
            })
            .collect();

        Self {
            keystrokes,
            action: Box::new(action),
            context_predicate,
            meta: None,
            action_input: None,
        }
    }

    /// Load a keybinding with explicit control over all fields.
    pub fn load(
        keystrokes: &str,
        action: Box<dyn Action>,
        context_predicate: Option<KeyBindingContextPredicate>,
        action_input: Option<String>,
    ) -> Result<Self, BindingError> {
        let keystrokes: SmallVec<[KeybindingKeystroke; 2]> = keystrokes
            .split_whitespace()
            .map(|source| Keystroke::parse(source).map(KeybindingKeystroke::new))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            keystrokes,
            action,
            context_predicate,
            meta: None,
            action_input,
        })
    }

    /// Set the metadata index for this binding.
    pub fn with_meta(mut self, meta: KeyBindingMetaIndex) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Set the metadata index (mutable).
    pub fn set_meta(&mut self, meta: KeyBindingMetaIndex) {
        self.meta = Some(meta);
    }

    /// Check if the given keystrokes match this binding.
    ///
    /// Returns `None` if they don't match, `Some(false)` if they match
    /// completely, or `Some(true)` if they partially match (more keystrokes
    /// needed for a chord).
    pub fn match_keystrokes(&self, typed: &[impl AsKeystroke]) -> Option<bool> {
        if self.keystrokes.len() < typed.len() {
            return None;
        }

        for (target, typed) in self.keystrokes.iter().zip(typed.iter()) {
            if !typed.as_keystroke().should_match(target) {
                return None;
            }
        }

        Some(self.keystrokes.len() > typed.len())
    }

    /// Get the keystrokes for this binding.
    pub fn keystrokes(&self) -> &[KeybindingKeystroke] {
        self.keystrokes.as_slice()
    }

    /// Get the action for this binding.
    pub fn action(&self) -> &dyn Action {
        self.action.as_ref()
    }

    /// Get the context predicate.
    pub fn predicate(&self) -> Option<&KeyBindingContextPredicate> {
        self.context_predicate.as_ref()
    }

    /// Get the metadata index.
    pub fn meta(&self) -> Option<KeyBindingMetaIndex> {
        self.meta
    }

    /// Get the action input string.
    pub fn action_input(&self) -> Option<&str> {
        self.action_input.as_deref()
    }

    /// Check if this binding's action is a NoAction sentinel.
    pub fn is_no_action(&self) -> bool {
        self.action.as_any().downcast_ref::<NoAction>().is_some()
    }

    /// Check if this binding's action is an Unbind sentinel.
    pub fn is_unbind(&self) -> bool {
        self.action.as_any().downcast_ref::<Unbind>().is_some()
    }
}

impl fmt::Debug for KeyBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyBinding")
            .field("keystrokes", &self.keystrokes)
            .field("context_predicate", &self.context_predicate)
            .field("action", &self.action.name())
            .finish()
    }
}

/// Errors from loading keybindings.
#[derive(Debug, Clone, thiserror::Error)]
pub enum BindingError {
    #[error("invalid keystroke: {0}")]
    InvalidKeystroke(#[from] crate::KeystrokeError),
}
