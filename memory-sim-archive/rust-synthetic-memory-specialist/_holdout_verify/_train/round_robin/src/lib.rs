//! round_robin: a fixed pool of candidate nodes is registered in insertion
//! order, then a batch of N requests is assigned to nodes by cycling through
//! the pool in order (request i goes to candidate i % pool_len). If the pool
//! is empty when a batch is requested, no assignments are produced and the
//! signal "empty_pool" is emitted.

/// Result of assigning a batch of requests: the node chosen per request (in
/// request order) and any emitted signals.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rotation {
    pub assignments: Vec<String>,
    pub signals: Vec<String>,
}

/// Runtime that holds the ordered candidate pool and assigns request batches
/// round-robin across it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoundRobin {
    candidates: Vec<String>,
}

impl RoundRobin {
    pub fn new() -> RoundRobin {
        RoundRobin {
            candidates: Vec::new(),
        }
    }

    /// Register a candidate node, preserving insertion order. Duplicates are
    /// kept as separate slots in the cycle.
    pub fn add(&mut self, node: &str) {
        self.candidates.push(node.to_string());
    }

    /// Assign `count` requests to candidates by cycling the pool in insertion
    /// order: request i (0-based) goes to candidate i % pool_len. With an empty
    /// pool, produces no assignments and emits the signal "empty_pool".
    pub fn assign(&self, count: usize) -> Rotation {
        let mut assignments: Vec<String> = Vec::new();
        let mut signals: Vec<String> = Vec::new();

        if self.candidates.is_empty() {
            signals.push("empty_pool".to_string());
            return Rotation {
                assignments,
                signals,
            };
        }

        let pool_len = self.candidates.len();
        for i in 0..count {
            let node = self.candidates[i % pool_len].clone();
            assignments.push(node);
        }

        Rotation {
            assignments,
            signals,
        }
    }
}

impl Default for RoundRobin {
    fn default() -> RoundRobin {
        RoundRobin::new()
    }
}
