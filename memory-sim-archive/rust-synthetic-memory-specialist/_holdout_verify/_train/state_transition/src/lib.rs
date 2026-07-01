//! state_transition: each node carries a lifecycle state. A node first seen is
//! `open`. Events drive a fixed forward chain: the `start` event moves a node
//! from `open` to `busy`, and the `finish` event moves it from `busy` to
//! `done`. Events that do not match the current state are ignored, leaving the
//! state unchanged, so the machine is deterministic regardless of event order.
//! The final state of a node can be queried at any time.

/// The reported lifecycle state of a single node: its `node` name and the
/// current `state` string (one of `open`, `busy`, `done`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeState {
    pub node: String,
    pub state: String,
}

struct Entry {
    node: String,
    state: String,
}

/// The state machine fabric. It remembers, in insertion order, every node it
/// has seen along with that node's current lifecycle state, and applies
/// transition events deterministically.
pub struct StateMachine {
    entries: Vec<Entry>,
}

impl StateMachine {
    /// Create an empty machine with no known nodes.
    pub fn new() -> Self {
        StateMachine {
            entries: Vec::new(),
        }
    }

    /// Apply a transition `event` to `node`. A node not seen before is created
    /// in the `open` state first. The `start` event moves `open` to `busy`;
    /// the `finish` event moves `busy` to `done`. Any other event, or an event
    /// that does not match the current state, leaves the state unchanged.
    pub fn event(&mut self, node: &str, event: &str) {
        let idx = self.ensure(node);
        let current = self.entries[idx].state.clone();
        let next = match (current.as_str(), event) {
            ("open", "start") => "busy",
            ("busy", "finish") => "done",
            _ => return,
        };
        self.entries[idx].state = next.to_string();
    }

    /// Find `node`, inserting it in the `open` state if unknown, and return its
    /// index in insertion order.
    fn ensure(&mut self, node: &str) -> usize {
        if let Some(i) = self.entries.iter().position(|e| e.node == node) {
            return i;
        }
        self.entries.push(Entry {
            node: node.to_string(),
            state: "open".to_string(),
        });
        self.entries.len() - 1
    }

    /// Report the current state of `node`. A node never seen is reported as
    /// `open`.
    pub fn state(&self, node: &str) -> NodeState {
        match self.entries.iter().find(|e| e.node == node) {
            Some(entry) => NodeState {
                node: entry.node.clone(),
                state: entry.state.clone(),
            },
            None => NodeState {
                node: node.to_string(),
                state: "open".to_string(),
            },
        }
    }
}
