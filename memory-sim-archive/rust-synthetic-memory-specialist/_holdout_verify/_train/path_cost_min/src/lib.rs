//! path_cost_min: directed edges carry an integer cost. Given a source node and
//! a list of candidate target nodes, report the candidate reachable by the
//! cheapest direct (first-edge) cost from the source. The "first-edge cost" of a
//! candidate is the cost of the first edge `source -> candidate` recorded in
//! insertion order. Candidates with no such direct edge are skipped and reported
//! as `miss:<candidate>` signals, in the order they appear in the query. Among
//! the candidates that do have a direct edge, the one with the minimum cost wins;
//! ties are broken by the candidate's position in the query list (earlier wins).
//! If no candidate has a direct edge from the source, the target is "none", the
//! cost is 0, and the single signal "unreachable" is emitted.

/// Result of a cheapest-target query: the chosen candidate (or "none"), the
/// first-edge cost paid to reach it (0 when none), and any emitted signals.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pick {
    pub target: String,
    pub cost: i32,
    pub signals: Vec<String>,
}

/// A directed, integer-cost edge, stored in insertion order.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Edge {
    from: String,
    to: String,
    cost: i32,
}

/// Runtime that holds the cost-bearing edges and answers cheapest-target queries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CostGraph {
    edges: Vec<Edge>,
}

impl CostGraph {
    pub fn new() -> CostGraph {
        CostGraph { edges: Vec::new() }
    }

    /// Record a directed edge from `from` to `to` carrying `cost`, preserving
    /// insertion order.
    pub fn connect(&mut self, from: &str, to: &str, cost: i32) {
        self.edges.push(Edge {
            from: from.to_string(),
            to: to.to_string(),
            cost,
        });
    }

    /// The first-edge cost from `source` to `target`: the cost of the first edge
    /// `source -> target` in insertion order, or None if no such edge exists.
    fn first_edge_cost(&self, source: &str, target: &str) -> Option<i32> {
        self.edges
            .iter()
            .find(|e| e.from == source && e.to == target)
            .map(|e| e.cost)
    }

    /// Among `candidates`, pick the one with the minimum first-edge cost from
    /// `source`. Candidates with no direct edge become "miss:<candidate>" signals
    /// in query order. Ties on cost are broken by query order (earlier wins). If
    /// no candidate is reachable, the target is "none", cost 0, signal
    /// "unreachable".
    pub fn cheapest(&self, source: &str, candidates: &[&str]) -> Pick {
        let mut signals: Vec<String> = Vec::new();
        let mut best: Option<(String, i32)> = None;

        for &candidate in candidates {
            match self.first_edge_cost(source, candidate) {
                Some(cost) => {
                    let better = match &best {
                        Some((_, best_cost)) => cost < *best_cost,
                        None => true,
                    };
                    if better {
                        best = Some((candidate.to_string(), cost));
                    }
                }
                None => signals.push(format!("miss:{}", candidate)),
            }
        }

        match best {
            Some((target, cost)) => Pick {
                target,
                cost,
                signals,
            },
            None => {
                signals.push("unreachable".to_string());
                Pick {
                    target: "none".to_string(),
                    cost: 0,
                    signals,
                }
            }
        }
    }
}

impl Default for CostGraph {
    fn default() -> CostGraph {
        CostGraph::new()
    }
}
