//! checkpoint_replay: a small append-only event log with checkpoint/replay.
//!
//! Events are recorded with monotonic integer ids starting at 1. A checkpoint
//! captures the current latest id. Replay returns only the events recorded
//! STRICTLY AFTER the last checkpoint.

/// A single recorded event, surfaced by [`ReplayLog::replay`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayEvent {
    pub index: usize,
    pub kind: String,
    pub node: String,
}

/// An append-only event log supporting checkpoint and replay.
pub struct ReplayLog {
    events: Vec<ReplayEvent>,
    checkpoint: usize,
}

impl ReplayLog {
    /// Create an empty log with no events and no checkpoint.
    pub fn new() -> ReplayLog {
        ReplayLog {
            events: Vec::new(),
            checkpoint: 0,
        }
    }

    /// Record a new event. Ids are monotonic, starting at 1.
    pub fn record(&mut self, kind: &str, node: &str) {
        let index = self.events.len() + 1;
        self.events.push(ReplayEvent {
            index,
            kind: kind.to_string(),
            node: node.to_string(),
        });
    }

    /// Capture the current latest id as the checkpoint. Returns the latest id
    /// so far (0 if no events have been recorded).
    pub fn checkpoint(&mut self) -> usize {
        self.checkpoint = self.events.len();
        self.checkpoint
    }

    /// Return events recorded strictly after the last checkpoint
    /// (those with index > the last checkpoint), in insertion order.
    pub fn replay(&self) -> Vec<ReplayEvent> {
        let mut out = Vec::new();
        for event in &self.events {
            if event.index > self.checkpoint {
                out.push(event.clone());
            }
        }
        out
    }
}

impl Default for ReplayLog {
    fn default() -> ReplayLog {
        ReplayLog::new()
    }
}
