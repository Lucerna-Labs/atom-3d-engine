//! state_diff: capture a "before" set and an "after" set of node names, then
//! report which nodes were added (present in after but not before) and which
//! were removed (present in before but not after). Both lists are sorted.

/// The result of diffing the before and after node sets. `added` holds nodes
/// that appear in the after set but not the before set; `removed` holds nodes
/// that appear in the before set but not the after set. Both are sorted and
/// deduplicated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

/// The diff fabric. It remembers the most recently declared before set and
/// after set as owned node names, and computes the StateDiff on demand.
#[derive(Clone, Debug, Default)]
pub struct DiffFabric {
    before: Vec<String>,
    after: Vec<String>,
}

impl DiffFabric {
    /// Create an empty fabric with no before or after nodes.
    pub fn new() -> Self {
        DiffFabric {
            before: Vec::new(),
            after: Vec::new(),
        }
    }

    /// Declare the before set, replacing any previously declared before nodes.
    pub fn before(&mut self, nodes: &[&str]) {
        self.before = nodes.iter().map(|&n| n.to_string()).collect();
    }

    /// Declare the after set, replacing any previously declared after nodes.
    pub fn after(&mut self, nodes: &[&str]) {
        self.after = nodes.iter().map(|&n| n.to_string()).collect();
    }

    /// Compute the diff: nodes in after but not before are added, nodes in
    /// before but not after are removed. Both lists are sorted and deduped.
    pub fn diff(&self) -> StateDiff {
        let mut added: Vec<String> = Vec::new();
        for node in &self.after {
            if !self.before.iter().any(|b| b == node) && !added.iter().any(|a| a == node) {
                added.push(node.clone());
            }
        }
        let mut removed: Vec<String> = Vec::new();
        for node in &self.before {
            if !self.after.iter().any(|a| a == node) && !removed.iter().any(|r| r == node) {
                removed.push(node.clone());
            }
        }
        added.sort();
        removed.sort();
        StateDiff { added, removed }
    }
}
