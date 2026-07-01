//! priority_merge: merge two integer-priority candidate lists into one ordered
//! list. Candidates are drained highest priority first. Ties are broken
//! deterministically: a candidate from the first list outranks one from the
//! second list at the same priority, and within a single list insertion order
//! is preserved. The merge emits "top:<name>" naming the winning candidate, or
//! "empty" when both lists are empty.

/// Result of merging the two candidate lists: the merged order (highest
/// priority first, ties by first list then insertion order) and any signals.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MergeResult {
    pub order: Vec<String>,
    pub signals: Vec<String>,
}

/// A single candidate tagged with its integer priority.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Candidate {
    name: String,
    priority: i64,
}

/// Runtime holding two ordered candidate lists to be merged on demand.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PriorityMerge {
    first: Vec<Candidate>,
    second: Vec<Candidate>,
}

impl PriorityMerge {
    pub fn new() -> PriorityMerge {
        PriorityMerge {
            first: Vec::new(),
            second: Vec::new(),
        }
    }

    /// Append a candidate to the first list, preserving insertion order.
    pub fn first(&mut self, name: &str, priority: i64) {
        self.first.push(Candidate {
            name: name.to_string(),
            priority,
        });
    }

    /// Append a candidate to the second list, preserving insertion order.
    pub fn second(&mut self, name: &str, priority: i64) {
        self.second.push(Candidate {
            name: name.to_string(),
            priority,
        });
    }

    /// Merge both lists: highest priority first; at equal priority a first-list
    /// candidate precedes a second-list candidate, and within one list the
    /// original insertion order is kept. Emits "top:<name>" for the winner, or
    /// "empty" when there are no candidates.
    pub fn merge(&self) -> MergeResult {
        // list rank: 0 for first, 1 for second. Build a stable index over the
        // concatenation so equal-priority ties resolve to (list, position).
        let mut entries: Vec<(i64, usize, usize, &Candidate)> = Vec::new();
        for (pos, c) in self.first.iter().enumerate() {
            entries.push((c.priority, 0, pos, c));
        }
        for (pos, c) in self.second.iter().enumerate() {
            entries.push((c.priority, 1, pos, c));
        }
        // Highest priority first; then first list before second; then insertion
        // order within the list.
        entries.sort_by(|a, b| {
            b.0.cmp(&a.0)
                .then_with(|| a.1.cmp(&b.1))
                .then_with(|| a.2.cmp(&b.2))
        });

        let order: Vec<String> = entries.iter().map(|e| e.3.name.clone()).collect();
        let mut signals: Vec<String> = Vec::new();
        match entries.first() {
            Some(e) => signals.push(format!("top:{}", e.3.name)),
            None => signals.push("empty".to_string()),
        }
        MergeResult { order, signals }
    }
}

impl Default for PriorityMerge {
    fn default() -> PriorityMerge {
        PriorityMerge::new()
    }
}
