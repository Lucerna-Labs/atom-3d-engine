//! Key context and context predicate system.
//!
//! This module provides the context matching system that determines which
//! keybindings are active based on the current UI state. A context stack
//! represents the element hierarchy (e.g., `[Workspace, Pane, Editor]`),
//! and context predicates declare which contexts a binding applies to
//! (e.g., `"editor && mode == full"`).

use std::fmt;

/// A context entry: either a bare identifier or a key-value pair.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ContextEntry {
    /// The key (or identifier if no value)
    pub key: String,
    /// The value, if this is a key-value pair
    pub value: Option<String>,
}

/// A set of context entries representing the state of one level in the
/// element hierarchy. For example: `Editor mode=full`.
#[derive(Clone, Default, Eq, PartialEq, Hash)]
pub struct KeyContext(pub Vec<ContextEntry>);

impl KeyContext {
    /// Create a context with default OS identification.
    pub fn new_with_defaults() -> Self {
        let mut ctx = Self::default();
        #[cfg(target_os = "macos")]
        ctx.set("os", "macos");
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        ctx.set("os", "linux");
        #[cfg(target_os = "windows")]
        ctx.set("os", "windows");
        #[cfg(not(any(
            target_os = "macos",
            target_os = "linux",
            target_os = "freebsd",
            target_os = "windows"
        )))]
        ctx.set("os", "unknown");
        ctx
    }

    /// Parse a key context from a string.
    ///
    /// Format: identifiers and key=value pairs separated by whitespace.
    /// Example: `Editor mode=full` or `StatusBar` or `pane x=y`
    pub fn parse(source: &str) -> Result<Self, ContextParseError> {
        let mut context = Self::default();
        let source = skip_whitespace(source);
        Self::parse_expr(source, &mut context)?;
        Ok(context)
    }

    fn parse_expr(mut source: &str, context: &mut Self) -> Result<(), ContextParseError> {
        if source.is_empty() {
            return Ok(());
        }

        let key: String = source
            .chars()
            .take_while(|c| is_identifier_char(*c))
            .collect();
        if key.is_empty() {
            return Err(ContextParseError::UnexpectedChar(
                source.chars().next().unwrap(),
            ));
        }
        source = skip_whitespace(&source[key.len()..]);

        if let Some(suffix) = source.strip_prefix('=') {
            source = skip_whitespace(suffix);
            let value: String = source
                .chars()
                .take_while(|c| is_identifier_char(*c))
                .collect();
            if value.is_empty() {
                return Err(ContextParseError::MissingValue(key));
            }
            source = skip_whitespace(&source[value.len()..]);
            context.set(key, value);
        } else {
            context.add(key);
        }

        Self::parse_expr(source, context)
    }

    /// Returns the primary (first bare identifier) entry.
    pub fn primary(&self) -> Option<&ContextEntry> {
        self.0.iter().find(|e| e.value.is_none())
    }

    /// Returns all non-primary entries.
    pub fn secondary(&self) -> impl Iterator<Item = &ContextEntry> {
        let primary = self.primary();
        self.0.iter().filter(move |&e| Some(e) != primary)
    }

    /// Check if this context is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Extend this context with entries from another (skipping duplicates).
    pub fn extend(&mut self, other: &Self) {
        for entry in &other.0 {
            if !self.contains(&entry.key) {
                self.0.push(entry.clone());
            }
        }
    }

    /// Add a bare identifier to this context.
    pub fn add(&mut self, identifier: impl Into<String>) {
        let key = identifier.into();
        if !self.contains(&key) {
            self.0.push(ContextEntry { key, value: None })
        }
    }

    /// Set a key-value pair in this context.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let key = key.into();
        if !self.contains(&key) {
            self.0.push(ContextEntry {
                key,
                value: Some(value.into()),
            })
        }
    }

    /// Check if this context contains a given key or identifier.
    pub fn contains(&self, key: &str) -> bool {
        self.0.iter().any(|e| e.key == key)
    }

    /// Get the value for a given key.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.iter().find(|e| e.key == key)?.value.as_ref()
    }
}

impl fmt::Debug for KeyContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut entries = self.0.iter().peekable();
        while let Some(entry) = entries.next() {
            if let Some(ref value) = entry.value {
                write!(f, "{}={}", entry.key, value)?;
            } else {
                write!(f, "{}", entry.key)?;
            }
            if entries.peek().is_some() {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for KeyContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A predicate language for matching keybindings against context stacks.
///
/// Supports:
/// - Bare identifiers: `Editor`
/// - Key-value equality: `mode == full`
/// - Key-value inequality: `mode != mini`
/// - Logical AND: `Editor && mode == full`
/// - Logical OR: `Editor || Terminal`
/// - Negation: `!Editor`
/// - Descendant (parent > child): `Workspace > Editor`
/// - Parenthesized groups: `(a || b) && c`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum KeyBindingContextPredicate {
    /// Match a context containing this identifier.
    Identifier(String),
    /// Match a context with key == value.
    Equal(String, String),
    /// Match a context with key != value (or key absent).
    NotEqual(String, String),
    /// Match a parent context containing the first predicate
    /// and a child context containing the second.
    Descendant(
        Box<KeyBindingContextPredicate>,
        Box<KeyBindingContextPredicate>,
    ),
    /// Negate a predicate.
    Not(Box<KeyBindingContextPredicate>),
    /// Both predicates must match.
    And(
        Box<KeyBindingContextPredicate>,
        Box<KeyBindingContextPredicate>,
    ),
    /// Either predicate must match.
    Or(
        Box<KeyBindingContextPredicate>,
        Box<KeyBindingContextPredicate>,
    ),
}

impl fmt::Display for KeyBindingContextPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(name) => write!(f, "{name}"),
            Self::Equal(left, right) => write!(f, "{left} == {right}"),
            Self::NotEqual(left, right) => write!(f, "{left} != {right}"),
            Self::Descendant(parent, child) => write!(f, "{parent} > {child}"),
            Self::Not(pred) => match pred.as_ref() {
                Self::Identifier(name) => write!(f, "!{name}"),
                _ => write!(f, "!({pred})"),
            },
            Self::And(..) => self.fmt_joined(f, " && ", true),
            Self::Or(..) => self.fmt_joined(f, " || ", false),
        }
    }
}

impl KeyBindingContextPredicate {
    /// Parse a context predicate from a string.
    ///
    /// # Examples
    ///
    /// - `"Editor"` → matches context with identifier `Editor`
    /// - `"Editor && mode == full"` → matches Editor context where mode is full
    /// - `"Workspace > Editor"` → matches when Workspace is a parent of Editor
    /// - `"!Terminal"` → matches when Terminal is NOT in the context
    pub fn parse(source: &str) -> Result<Self, ContextParseError> {
        let source = skip_whitespace(source);
        let (predicate, rest) = Self::parse_expr(source, 0)?;
        if let Some(next) = rest.chars().next() {
            return Err(ContextParseError::UnexpectedChar(next));
        }
        Ok(predicate)
    }

    /// Find the deepest depth at which this predicate matches the context stack.
    pub fn depth_of(&self, contexts: &[KeyContext]) -> Option<usize> {
        for depth in (0..=contexts.len()).rev() {
            let context_slice = &contexts[0..depth];
            if self.eval_inner(context_slice, contexts) {
                return Some(depth);
            }
        }
        None
    }

    /// Evaluate this predicate against a full context stack.
    pub fn eval(&self, contexts: &[KeyContext]) -> bool {
        self.eval_inner(contexts, contexts)
    }

    pub(crate) fn eval_inner(&self, contexts: &[KeyContext], all_contexts: &[KeyContext]) -> bool {
        let Some(context) = contexts.last() else {
            return false;
        };
        match self {
            Self::Identifier(name) => context.contains(name),
            Self::Equal(left, right) => context
                .get(left)
                .map(|value| value == right)
                .unwrap_or(false),
            Self::NotEqual(left, right) => context
                .get(left)
                .map(|value| value != right)
                .unwrap_or(true),
            Self::Not(pred) => {
                for i in 0..all_contexts.len() {
                    if pred.eval_inner(&all_contexts[..=i], all_contexts) {
                        return false;
                    }
                }
                true
            }
            Self::Descendant(parent, child) => {
                for i in 0..contexts.len() - 1 {
                    if parent.eval_inner(&contexts[..=i], all_contexts) {
                        if !child.eval_inner(&contexts[i + 1..], &contexts[i + 1..]) {
                            return false;
                        }
                        return true;
                    }
                }
                false
            }
            Self::And(left, right) => {
                left.eval_inner(contexts, all_contexts) && right.eval_inner(contexts, all_contexts)
            }
            Self::Or(left, right) => {
                left.eval_inner(contexts, all_contexts) || right.eval_inner(contexts, all_contexts)
            }
        }
    }

    /// Whether this predicate matches all contexts that `other` matches.
    pub fn is_superset(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }

        if let Self::Or(left, right) = self {
            return left.is_superset(other) || right.is_superset(other);
        }

        match other {
            Self::Descendant(_, child) => self.is_superset(child),
            Self::And(left, right) => self.is_superset(left) || self.is_superset(right),
            _ => false,
        }
    }

    // --- Parser internals ---

    fn parse_expr(
        mut source: &str,
        min_precedence: u32,
    ) -> Result<(Self, &str), ContextParseError> {
        let (mut predicate, rest) = Self::parse_primary(source)?;
        source = rest;

        'parse: loop {
            for (operator, precedence) in [
                (">", PRECEDENCE_CHILD),
                ("&&", PRECEDENCE_AND),
                ("||", PRECEDENCE_OR),
                ("==", PRECEDENCE_EQ),
                ("!=", PRECEDENCE_EQ),
            ] {
                if source.starts_with(operator) && precedence >= min_precedence {
                    source = skip_whitespace(&source[operator.len()..]);
                    let (right, rest) = Self::parse_expr(source, precedence + 1)?;

                    predicate = match operator {
                        ">" => Self::new_child(predicate, right)?,
                        "&&" => Self::new_and(predicate, right)?,
                        "||" => Self::new_or(predicate, right)?,
                        "==" => Self::new_eq(predicate, right)?,
                        "!=" => Self::new_neq(predicate, right)?,
                        _ => unreachable!(),
                    };

                    source = rest;
                    continue 'parse;
                }
            }
            break;
        }

        Ok((predicate, source))
    }

    fn parse_primary(source: &str) -> Result<(Self, &str), ContextParseError> {
        let next = match source.chars().next() {
            Some(c) => c,
            None => return Err(ContextParseError::UnexpectedEnd),
        };

        match next {
            '(' => {
                let source = skip_whitespace(&source[1..]);
                let (predicate, rest) = Self::parse_expr(source, 0)?;
                let stripped = rest
                    .strip_prefix(')')
                    .ok_or(ContextParseError::MissingCloseParen)?;
                let source = skip_whitespace(stripped);
                Ok((predicate, source))
            }
            '!' => {
                let source = skip_whitespace(&source[1..]);
                let (predicate, source) = Self::parse_expr(source, PRECEDENCE_NOT)?;
                Ok((Self::Not(Box::new(predicate)), source))
            }
            _ if is_identifier_char(next) => {
                let len = source
                    .find(|c: char| !is_identifier_char(c))
                    .unwrap_or(source.len());
                let (identifier, rest) = source.split_at(len);
                let source = skip_whitespace(rest);
                Ok((Self::Identifier(identifier.to_string()), source))
            }
            _ => Err(ContextParseError::UnexpectedChar(next)),
        }
    }

    fn new_or(self, other: Self) -> Result<Self, ContextParseError> {
        Ok(Self::Or(Box::new(self), Box::new(other)))
    }
    fn new_and(self, other: Self) -> Result<Self, ContextParseError> {
        Ok(Self::And(Box::new(self), Box::new(other)))
    }
    fn new_child(self, other: Self) -> Result<Self, ContextParseError> {
        Ok(Self::Descendant(Box::new(self), Box::new(other)))
    }
    fn new_eq(self, other: Self) -> Result<Self, ContextParseError> {
        match (self, other) {
            (Self::Identifier(left), Self::Identifier(right)) => Ok(Self::Equal(left, right)),
            _ => Err(ContextParseError::InvalidEqOperands),
        }
    }
    fn new_neq(self, other: Self) -> Result<Self, ContextParseError> {
        match (self, other) {
            (Self::Identifier(left), Self::Identifier(right)) => Ok(Self::NotEqual(left, right)),
            _ => Err(ContextParseError::InvalidNeqOperands),
        }
    }

    fn fmt_joined(&self, f: &mut fmt::Formatter<'_>, separator: &str, is_and: bool) -> fmt::Result {
        let mut first = true;
        self.fmt_joined_inner(f, separator, is_and, &mut first)
    }

    fn fmt_joined_inner(
        &self,
        f: &mut fmt::Formatter<'_>,
        separator: &str,
        is_and: bool,
        first: &mut bool,
    ) -> fmt::Result {
        match (is_and, self) {
            (true, Self::And(left, right)) | (false, Self::Or(left, right)) => {
                left.fmt_joined_inner(f, separator, is_and, first)?;
                right.fmt_joined_inner(f, separator, is_and, first)
            }
            (_, node) => {
                if !*first {
                    f.write_str(separator)?;
                }
                *first = false;
                // Wrap OR inside AND and vice versa
                let needs_parens = if is_and {
                    matches!(node, Self::Or(..))
                } else {
                    matches!(node, Self::And(..))
                };
                if needs_parens {
                    write!(f, "({node})")
                } else {
                    write!(f, "{node}")
                }
            }
        }
    }
}

// Operator precedence
const PRECEDENCE_CHILD: u32 = 1;
const PRECEDENCE_OR: u32 = 2;
const PRECEDENCE_AND: u32 = 3;
const PRECEDENCE_EQ: u32 = 4;
const PRECEDENCE_NOT: u32 = 5;

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-'
}

fn skip_whitespace(source: &str) -> &str {
    let len = source
        .find(|c: char| !c.is_whitespace())
        .unwrap_or(source.len());
    &source[len..]
}

/// Errors from parsing key contexts or predicates.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ContextParseError {
    #[error("unexpected end of input")]
    UnexpectedEnd,
    #[error("unexpected character: {0}")]
    UnexpectedChar(char),
    #[error("missing value for key: {0}")]
    MissingValue(String),
    #[error("missing closing parenthesis")]
    MissingCloseParen,
    #[error("operands of == must be identifiers")]
    InvalidEqOperands,
    #[error("operands of != must be identifiers")]
    InvalidNeqOperands,
}

#[cfg(test)]
mod tests {
    use super::*;
    use KeyBindingContextPredicate::*;

    #[test]
    fn test_parse_context() {
        let mut expected = KeyContext::default();
        expected.add("baz");
        expected.set("foo", "bar");
        assert_eq!(KeyContext::parse("baz foo=bar").unwrap(), expected);
        assert_eq!(KeyContext::parse("baz foo = bar").unwrap(), expected);
    }

    #[test]
    fn test_parse_identifiers() {
        assert_eq!(
            KeyBindingContextPredicate::parse("abc12").unwrap(),
            Identifier("abc12".into())
        );
    }

    #[test]
    fn test_parse_negations() {
        assert_eq!(
            KeyBindingContextPredicate::parse("!abc").unwrap(),
            Not(Box::new(Identifier("abc".into())))
        );
    }

    #[test]
    fn test_parse_equality_operators() {
        assert_eq!(
            KeyBindingContextPredicate::parse("a == b").unwrap(),
            Equal("a".into(), "b".into())
        );
        assert_eq!(
            KeyBindingContextPredicate::parse("c!=d").unwrap(),
            NotEqual("c".into(), "d".into())
        );
    }

    #[test]
    fn test_parse_boolean_operators() {
        assert_eq!(
            KeyBindingContextPredicate::parse("a || b").unwrap(),
            Or(
                Box::new(Identifier("a".into())),
                Box::new(Identifier("b".into()))
            )
        );
        assert_eq!(
            KeyBindingContextPredicate::parse("a && b || c&&d").unwrap(),
            Or(
                Box::new(And(
                    Box::new(Identifier("a".into())),
                    Box::new(Identifier("b".into()))
                )),
                Box::new(And(
                    Box::new(Identifier("c".into())),
                    Box::new(Identifier("d".into()))
                ))
            )
        );
    }

    #[test]
    fn test_child_operator() {
        let predicate = KeyBindingContextPredicate::parse("parent > child").unwrap();
        let parent = KeyContext::parse("parent").unwrap();
        let child = KeyContext::parse("child").unwrap();

        assert!(predicate.eval(&[parent.clone(), child.clone()]));
        assert!(!predicate.eval(&[child.clone()]));
        assert!(!predicate.eval(&[parent]));
    }

    #[test]
    fn test_not_operator() {
        let not_editor = KeyBindingContextPredicate::parse("!editor").unwrap();
        let editor = KeyContext::parse("editor").unwrap();
        let workspace = KeyContext::parse("workspace").unwrap();

        assert!(not_editor.eval(&[workspace]));
        assert!(!not_editor.eval(&[editor.clone()]));
        assert!(!not_editor.eval(&[editor, KeyContext::parse("workspace").unwrap()]));
    }

    #[test]
    fn test_is_superset() {
        let a = KeyBindingContextPredicate::parse("editor").unwrap();
        let b = KeyBindingContextPredicate::parse("editor && vim_mode").unwrap();
        assert!(a.is_superset(&b));
        assert!(!b.is_superset(&a));
    }

    #[test]
    fn test_display_roundtrip() {
        let cases = [
            "a",
            "a == b",
            "a != b",
            "a > b",
            "!a",
            "a && b",
            "a || b",
            "a && (b || c)",
        ];
        for case in cases {
            let parsed = KeyBindingContextPredicate::parse(case).unwrap();
            let displayed = parsed.to_string();
            let re_parsed = KeyBindingContextPredicate::parse(&displayed).unwrap();
            assert_eq!(parsed, re_parsed, "roundtrip failed for: {case}");
        }
    }
}
