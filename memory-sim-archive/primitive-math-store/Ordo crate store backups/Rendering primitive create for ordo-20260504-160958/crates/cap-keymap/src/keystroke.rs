//! Keystroke representation — platform-independent, renderer-neutral.

use std::fmt;

/// Keyboard modifier keys.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Modifiers {
    /// Control key held
    pub control: bool,
    /// Alt/Option key held
    pub alt: bool,
    /// Shift key held
    pub shift: bool,
    /// Super/Command/Windows key held
    pub super_key: bool,
}

impl Modifiers {
    /// No modifiers active.
    pub const NONE: Self = Self {
        control: false,
        alt: false,
        shift: false,
        super_key: false,
    };

    /// Parse modifiers from a keystroke string prefix.
    /// Supports: `ctrl-`, `alt-`, `shift-`, `cmd-`, `super-`, `win-`
    pub fn parse_prefixes(input: &str) -> (Self, &str) {
        let mut mods = Self::NONE;
        let mut remaining = input;

        loop {
            let lower = remaining.to_lowercase();
            if let Some(_rest) = lower.strip_prefix("ctrl-") {
                mods.control = true;
                remaining = &remaining[5..];
            } else if let Some(_rest) = lower.strip_prefix("alt-") {
                mods.alt = true;
                remaining = &remaining[4..];
            } else if let Some(_rest) = lower.strip_prefix("shift-") {
                mods.shift = true;
                remaining = &remaining[6..];
            } else if let Some(_rest) = lower.strip_prefix("cmd-") {
                mods.super_key = true;
                remaining = &remaining[4..];
            } else if let Some(_rest) = lower.strip_prefix("super-") {
                mods.super_key = true;
                remaining = &remaining[6..];
            } else if let Some(_rest) = lower.strip_prefix("win-") {
                mods.super_key = true;
                remaining = &remaining[4..];
            } else {
                break;
            }
        }

        (mods, remaining)
    }
}

impl fmt::Display for Modifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        if self.control {
            parts.push("ctrl");
        }
        if self.alt {
            parts.push("alt");
        }
        if self.shift {
            parts.push("shift");
        }
        if self.super_key {
            parts.push("cmd");
        }
        write!(f, "{}", parts.join("-"))
    }
}

/// A single keystroke — a key plus its modifier state.
///
/// This is the input type that the keymap matches against.
/// It does not depend on any platform or windowing library.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Keystroke {
    /// Modifier keys held when this keystroke was produced
    pub modifiers: Modifiers,
    /// The key character (e.g., "a", "1", "space", "enter", "left")
    pub key: String,
}

impl Keystroke {
    /// Create a new keystroke with the given modifiers and key.
    pub fn new(modifiers: Modifiers, key: impl Into<String>) -> Self {
        Self {
            modifiers,
            key: key.into(),
        }
    }

    /// Parse a keystroke from a string like `ctrl-alt-shift-a` or `space` or `cmd-s`.
    ///
    /// Format: `[modifier-]*key` where modifiers are `ctrl`, `alt`, `shift`, `cmd`/`super`/`win`.
    pub fn parse(input: &str) -> Result<Self, KeystrokeError> {
        let (modifiers, key) = Modifiers::parse_prefixes(input);

        if key.is_empty() {
            return Err(KeystrokeError::MissingKey(input.to_string()));
        }

        Ok(Self {
            modifiers,
            key: key.to_string(),
        })
    }

    /// Convert back to a parseable string representation.
    pub fn unparse(&self) -> String {
        let mut parts = Vec::new();

        if self.modifiers.control {
            parts.push("ctrl");
        }
        if self.modifiers.alt {
            parts.push("alt");
        }
        if self.modifiers.shift {
            parts.push("shift");
        }
        if self.modifiers.super_key {
            parts.push("cmd");
        }
        parts.push(&self.key);

        parts.join("-")
    }

    /// Check if this keystroke should match a keybinding keystroke.
    /// For matching, the typed keystroke's modifiers must be a superset
    /// and the key must match (case-insensitive for letters).
    pub fn should_match(&self, binding: &KeybindingKeystroke) -> bool {
        // Modifiers must match exactly
        if self.modifiers != binding.keystroke.modifiers {
            return false;
        }

        // Key match: case-insensitive for single letters, exact otherwise
        if self.key.len() == 1 && binding.keystroke.key.len() == 1 {
            self.key.eq_ignore_ascii_case(&binding.keystroke.key)
        } else {
            self.key == binding.keystroke.key
        }
    }
}

impl fmt::Display for Keystroke {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.unparse())
    }
}

/// A keystroke as it appears in a keybinding definition.
///
/// This wraps a [`Keystroke`] and may later carry platform-specific display
/// information. For now, it's a clean separation between "what the user typed"
/// and "what the keybinding declares."
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct KeybindingKeystroke {
    /// The keystroke this binding entry refers to
    pub keystroke: Keystroke,
}

impl KeybindingKeystroke {
    /// Create a keybinding keystroke from a parsed keystroke.
    pub fn new(keystroke: Keystroke) -> Self {
        Self { keystroke }
    }

    /// Access the inner keystroke.
    pub fn inner(&self) -> &Keystroke {
        &self.keystroke
    }
}

/// Trait for types that can be compared against a keybinding keystroke.
/// Allows both `Keystroke` and `KeybindingKeystroke` to be used in matching.
pub trait AsKeystroke {
    fn as_keystroke(&self) -> &Keystroke;
}

impl AsKeystroke for Keystroke {
    fn as_keystroke(&self) -> &Keystroke {
        self
    }
}

impl AsKeystroke for KeybindingKeystroke {
    fn as_keystroke(&self) -> &Keystroke {
        &self.keystroke
    }
}

/// Errors that can occur when parsing keystrokes.
#[derive(Debug, Clone, thiserror::Error)]
pub enum KeystrokeError {
    #[error("missing key in keystroke: {0}")]
    MissingKey(String),
    #[error("invalid keystroke format: {0}")]
    InvalidFormat(String),
}
