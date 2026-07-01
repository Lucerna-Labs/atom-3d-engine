//! hop_budget: a flow carries a hop budget and walks edges (first outgoing edge
//! in insertion order, one hop per edge) until the budget is spent or no edge
//! remains. If it stops because the budget ran out while more edges existed, it
//! emits the signal "budget_exhausted".

/// Result of walking a flow: the path of visited nodes and any emitted signals.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HopTrace {
    pub path: Vec<String>,
    pub signals: Vec<String>,
}

/// A directed edge from one node to another, stored in insertion order.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Edge {
    from: String,
    to: String,
}

/// Runtime that holds the directed edges and walks budgeted flows over them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HopRuntime {
    edges: Vec<Edge>,
}

impl HopRuntime {
    pub fn new() -> HopRuntime {
        HopRuntime { edges: Vec::new() }
    }

    /// Record a directed edge from `from` to `to`, preserving insertion order.
    pub fn connect(&mut self, from: &str, to: &str) {
        self.edges.push(Edge {
            from: from.to_string(),
            to: to.to_string(),
        });
    }

    /// Find the first outgoing edge (in insertion order) for `node`.
    fn first_outgoing(&self, node: &str) -> Option<&Edge> {
        self.edges.iter().find(|e| e.from == node)
    }

    /// Walk from `start`, taking the first outgoing edge per node, one hop per
    /// edge, until the budget is spent or no edge remains. Emits
    /// "budget_exhausted" if it stops on budget while an edge still existed.
    pub fn flow(&self, start: &str, budget: usize) -> HopTrace {
        let mut path: Vec<String> = vec![start.to_string()];
        let mut signals: Vec<String> = Vec::new();
        let mut current = start.to_string();
        let mut remaining = budget;

        loop {
            match self.first_outgoing(&current) {
                Some(edge) => {
                    if remaining == 0 {
                        // An edge existed but the budget is spent.
                        signals.push("budget_exhausted".to_string());
                        break;
                    }
                    let next = edge.to.clone();
                    path.push(next.clone());
                    current = next;
                    remaining -= 1;
                }
                None => break,
            }
        }

        HopTrace { path, signals }
    }
}

impl Default for HopRuntime {
    fn default() -> HopRuntime {
        HopRuntime::new()
    }
}
