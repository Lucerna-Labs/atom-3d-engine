//! Keymap — the core binding registry and resolution engine.
//!
//! This is the central type. Register bindings, then resolve input against
//! a context stack to get the matching actions.

use crate::{Action, KeyBinding, KeyContext, Keystroke, Unbind};
use smallvec::SmallVec;
use std::any::TypeId;
use std::collections::HashMap;

/// An opaque version identifier. Changes whenever bindings are added or removed.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct KeymapVersion(pub usize);

/// Index of a binding within the keymap.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct BindingIndex(pub usize);

/// A collection of key bindings with context-aware resolution.
///
/// # Precedence Rules
///
/// 1. **Context depth** — deeper contexts (Editor) override shallower ones (Workspace)
/// 2. **Insertion order** — later bindings override earlier ones at the same depth
/// 3. **User > Default** — user keymaps are added after built-in ones, so they win
/// 4. **Disable/unbind** — `NoAction` and `Unbind` entries can suppress specific bindings
///
/// # Multi-Key Sequences
///
/// Bindings like `ctrl-x ctrl-s` are supported. When the first key in a sequence
/// is pressed, the keymap returns `(bindings, true)` where the boolean indicates
/// more input may complete a chord. When the full sequence is typed, the boolean
/// is `false`.
#[derive(Default)]
pub struct Keymap {
    bindings: Vec<KeyBinding>,
    binding_indices_by_action_type: HashMap<TypeId, SmallVec<[usize; 3]>>,
    disabled_binding_indices: Vec<usize>,
    version: KeymapVersion,
}

fn disabled_binding_matches_context(disabled_binding: &KeyBinding, binding: &KeyBinding) -> bool {
    match (
        &disabled_binding.context_predicate,
        &binding.context_predicate,
    ) {
        (None, _) => true,
        (Some(_), None) => false,
        (Some(disabled_pred), Some(pred)) => disabled_pred.is_superset(pred),
    }
}

fn binding_is_unbound(disabled_binding: &KeyBinding, binding: &KeyBinding) -> bool {
    disabled_binding.keystrokes == binding.keystrokes
        && disabled_binding
            .action
            .as_any()
            .downcast_ref::<Unbind>()
            .is_some_and(|unbind| unbind.0 == binding.action.name())
}

impl Keymap {
    /// Create a keymap with the given bindings.
    pub fn new(bindings: Vec<KeyBinding>) -> Self {
        let mut this = Self::default();
        this.add_bindings(bindings);
        this
    }

    /// Get the current version (changes on every add/clear).
    pub fn version(&self) -> KeymapVersion {
        self.version
    }

    /// Add bindings to this keymap.
    pub fn add_bindings<T: IntoIterator<Item = KeyBinding>>(&mut self, bindings: T) {
        for binding in bindings {
            let action_type = binding.action.as_any().type_id();
            if binding.is_no_action() || binding.is_unbind() {
                self.disabled_binding_indices.push(self.bindings.len());
            } else {
                self.binding_indices_by_action_type
                    .entry(action_type)
                    .or_default()
                    .push(self.bindings.len());
            }
            self.bindings.push(binding);
        }
        self.version.0 += 1;
    }

    /// Reset the keymap to empty.
    pub fn clear(&mut self) {
        self.bindings.clear();
        self.binding_indices_by_action_type.clear();
        self.disabled_binding_indices.clear();
        self.version.0 += 1;
    }

    /// Iterate all bindings in insertion order.
    pub fn bindings(&self) -> impl DoubleEndedIterator<Item = &KeyBinding> + ExactSizeIterator {
        self.bindings.iter()
    }

    /// Iterate bindings for a specific action.
    ///
    /// Bindings disabled by `NoAction` or `Unbind` are excluded.
    pub fn bindings_for_action<'a>(
        &'a self,
        action_type_id: TypeId,
        _action_name: &'a str,
        partial_eq: &'a dyn Fn(&dyn Action) -> bool,
    ) -> impl 'a + DoubleEndedIterator<Item = &'a KeyBinding> {
        let binding_indices = self
            .binding_indices_by_action_type
            .get(&action_type_id)
            .map_or(&[] as _, SmallVec::as_slice)
            .iter();

        binding_indices.filter_map(|ix| {
            let binding = &self.bindings[*ix];
            if !partial_eq(binding.action.as_ref()) {
                return None;
            }

            for disabled_ix in &self.disabled_binding_indices {
                if disabled_ix > ix {
                    let disabled_binding = &self.bindings[*disabled_ix];
                    if disabled_binding.keystrokes != binding.keystrokes {
                        continue;
                    }

                    if disabled_binding.is_no_action()
                        && disabled_binding_matches_context(disabled_binding, binding)
                    {
                        return None;
                    }

                    if disabled_binding.is_unbind()
                        && disabled_binding_matches_context(disabled_binding, binding)
                        && binding_is_unbound(disabled_binding, binding)
                    {
                        return None;
                    }
                }
            }

            Some(binding)
        })
    }

    /// All bindings that match the input without checking context.
    pub fn all_bindings_for_input(&self, input: &[Keystroke]) -> Vec<KeyBinding> {
        self.bindings()
            .rev()
            .filter(|binding| {
                binding
                    .match_keystrokes(input)
                    .is_some_and(|pending| !pending)
            })
            .cloned()
            .collect()
    }

    /// Resolve bindings for the given input and context stack.
    ///
    /// Returns `(matched_bindings, pending)`. If `pending` is true, more
    /// keystrokes may complete a multi-key sequence.
    ///
    /// Bindings are returned in precedence order: deepest context first,
    /// then latest insertion first.
    pub fn bindings_for_input(
        &self,
        input: &[Keystroke],
        context_stack: &[KeyContext],
    ) -> (SmallVec<[KeyBinding; 1]>, bool) {
        let mut matched_bindings = SmallVec::<[(usize, BindingIndex, &KeyBinding); 1]>::new();
        let mut pending_bindings = SmallVec::<[(BindingIndex, &KeyBinding); 1]>::new();

        for (ix, binding) in self.bindings().enumerate().rev() {
            let Some(depth) = self.binding_enabled(binding, context_stack) else {
                continue;
            };
            let Some(pending) = binding.match_keystrokes(input) else {
                continue;
            };

            if !pending {
                matched_bindings.push((depth, BindingIndex(ix), binding));
            } else {
                pending_bindings.push((BindingIndex(ix), binding));
            }
        }

        // Sort by depth (deepest first), then by insertion order (latest first)
        matched_bindings.sort_by(|(depth_a, ix_a, _), (depth_b, ix_b, _)| {
            depth_b.cmp(depth_a).then(ix_b.cmp(ix_a))
        });

        let mut bindings: SmallVec<[KeyBinding; 1]> = SmallVec::new();
        let mut first_binding_index = None;
        let mut unbound_bindings: Vec<&KeyBinding> = Vec::new();

        for (_, ix, binding) in matched_bindings {
            if binding.is_no_action() {
                // User NoAction breaks the search; base NoAction allows continuation
                if let Some(meta) = binding.meta {
                    if meta.0 == 0 {
                        break;
                    }
                } else {
                    break;
                }
                continue;
            }

            if binding.is_unbind() {
                unbound_bindings.push(binding);
                continue;
            }

            if unbound_bindings
                .iter()
                .any(|disabled| binding_is_unbound(disabled, binding))
            {
                continue;
            }

            bindings.push(binding.clone());
            first_binding_index.get_or_insert(ix);
        }

        let mut pending = false;
        for (ix, binding) in pending_bindings.into_iter().rev() {
            if let Some(binding_ix) = first_binding_index {
                if binding_ix > ix {
                    continue;
                }
            }
            if binding.is_no_action() || binding.is_unbind() {
                continue;
            }
            pending = true;
        }

        (bindings, pending)
    }

    /// Find bindings that could follow the current input sequence.
    pub fn possible_next_bindings_for_input(
        &self,
        input: &[Keystroke],
        context_stack: &[KeyContext],
    ) -> Vec<KeyBinding> {
        let mut bindings = self
            .bindings()
            .enumerate()
            .rev()
            .filter_map(|(ix, binding)| {
                let depth = self.binding_enabled(binding, context_stack)?;
                let pending = binding.match_keystrokes(input)?;
                match pending {
                    false | true if binding.is_no_action() || binding.is_unbind() => None,
                    true => Some((depth, BindingIndex(ix), binding)),
                    false => None,
                }
            })
            .collect::<Vec<_>>();

        bindings.sort_by(|(depth_a, ix_a, _), (depth_b, ix_b, _)| {
            depth_b.cmp(depth_a).then(ix_b.cmp(ix_a))
        });

        bindings
            .into_iter()
            .map(|(_, _, binding)| binding.clone())
            .collect()
    }

    /// Check if a binding is enabled for the given context stack.
    /// Returns the matching depth, or None if it doesn't match.
    fn binding_enabled(&self, binding: &KeyBinding, contexts: &[KeyContext]) -> Option<usize> {
        if let Some(predicate) = &binding.context_predicate {
            predicate.depth_of(contexts)
        } else {
            Some(contexts.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use std::any::Any;

    // Test action definitions
    #[derive(Debug, Clone)]
    struct ActionAlpha;
    #[derive(Debug, Clone)]
    struct ActionBeta;
    #[derive(Debug, Clone)]
    struct ActionGamma;

    impl Action for ActionAlpha {
        fn boxed_clone(&self) -> Box<dyn Action> {
            Box::new(ActionAlpha)
        }
        fn partial_eq(&self, _other: &dyn Action) -> bool {
            false
        }
        fn name(&self) -> &'static str {
            "ActionAlpha"
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl Action for ActionBeta {
        fn boxed_clone(&self) -> Box<dyn Action> {
            Box::new(ActionBeta)
        }
        fn partial_eq(&self, _other: &dyn Action) -> bool {
            false
        }
        fn name(&self) -> &'static str {
            "ActionBeta"
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl Action for ActionGamma {
        fn boxed_clone(&self) -> Box<dyn Action> {
            Box::new(ActionGamma)
        }
        fn partial_eq(&self, _other: &dyn Action) -> bool {
            false
        }
        fn name(&self) -> &'static str {
            "ActionGamma"
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn test_basic_binding() {
        let keymap = Keymap::new(vec![KeyBinding::new("ctrl-a", ActionAlpha, None)]);

        let (result, pending) =
            keymap.bindings_for_input(&[Keystroke::parse("ctrl-a").unwrap()], &[]);

        assert!(!pending);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].action().name(), "ActionAlpha");
    }

    #[test]
    fn test_context_depth_precedence() {
        let keymap = Keymap::new(vec![
            KeyBinding::new("ctrl-a", ActionBeta, Some("pane")),
            KeyBinding::new("ctrl-a", ActionGamma, Some("editor")),
        ]);

        let (result, _) = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-a").unwrap()],
            &[
                KeyContext::parse("pane").unwrap(),
                KeyContext::parse("editor").unwrap(),
            ],
        );

        assert_eq!(result.len(), 2);
        // Deeper context (editor) should come first
        assert_eq!(result[0].action().name(), "ActionGamma");
        assert_eq!(result[1].action().name(), "ActionBeta");
    }

    #[test]
    fn test_multi_key_sequence() {
        let keymap = Keymap::new(vec![KeyBinding::new(
            "ctrl-x ctrl-s",
            ActionAlpha,
            Some("editor"),
        )]);

        let (result, pending) = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-x").unwrap()],
            &[KeyContext::parse("editor").unwrap()],
        );

        assert!(result.is_empty());
        assert!(pending); // More keys needed

        let (result, pending) = keymap.bindings_for_input(
            &[
                Keystroke::parse("ctrl-x").unwrap(),
                Keystroke::parse("ctrl-s").unwrap(),
            ],
            &[KeyContext::parse("editor").unwrap()],
        );

        assert_eq!(result.len(), 1);
        assert!(!pending); // Complete
    }

    #[test]
    fn test_no_action_disables_binding() {
        let keymap = Keymap::new(vec![
            KeyBinding::new("ctrl-a", ActionAlpha, Some("editor")),
            KeyBinding::new("ctrl-a", NoAction, Some("editor")),
        ]);

        let (result, _) = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-a").unwrap()],
            &[KeyContext::parse("editor").unwrap()],
        );

        assert!(result.is_empty());
    }

    #[test]
    fn test_unbind_targets_specific_action() {
        let keymap = Keymap::new(vec![
            KeyBinding::new("tab", ActionAlpha, Some("Editor")),
            KeyBinding::new("tab", ActionBeta, Some("Editor && showing_completions")),
            KeyBinding::new(
                "tab",
                Unbind("ActionAlpha".into()),
                Some("Editor && edit_prediction"),
            ),
        ]);

        let (result, pending) = keymap.bindings_for_input(
            &[Keystroke::parse("tab").unwrap()],
            &[KeyContext::parse("Editor showing_completions edit_prediction").unwrap()],
        );

        assert!(!pending);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].action().name(), "ActionBeta");
    }

    #[test]
    fn test_global_binding_matches_all_contexts() {
        let keymap = Keymap::new(vec![
            KeyBinding::new("ctrl-a", ActionAlpha, None), // global
        ]);

        // Works with no context
        let (result, _) = keymap.bindings_for_input(&[Keystroke::parse("ctrl-a").unwrap()], &[]);
        assert_eq!(result.len(), 1);

        // Works with any context
        let (result, _) = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-a").unwrap()],
            &[KeyContext::parse("workspace").unwrap()],
        );
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_context_predicate_key_value() {
        let keymap = Keymap::new(vec![KeyBinding::new(
            "ctrl-a",
            ActionAlpha,
            Some("editor && mode == full"),
        )]);

        // Doesn't match — missing mode
        let (result, _) = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-a").unwrap()],
            &[KeyContext::parse("editor").unwrap()],
        );
        assert!(result.is_empty());

        // Matches — mode is full
        let (result, _) = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-a").unwrap()],
            &[KeyContext::parse("editor mode=full").unwrap()],
        );
        assert_eq!(result.len(), 1);

        // Doesn't match — mode is wrong
        let (result, _) = keymap.bindings_for_input(
            &[Keystroke::parse("ctrl-a").unwrap()],
            &[KeyContext::parse("editor mode=mini").unwrap()],
        );
        assert!(result.is_empty());
    }

    #[test]
    fn test_keymap_version_increments() {
        let mut keymap = Keymap::default();
        assert_eq!(keymap.version().0, 0);

        keymap.add_bindings(vec![KeyBinding::new("ctrl-a", ActionAlpha, None)]);
        assert_eq!(keymap.version().0, 1);

        keymap.clear();
        assert_eq!(keymap.version().0, 2);
    }
}
