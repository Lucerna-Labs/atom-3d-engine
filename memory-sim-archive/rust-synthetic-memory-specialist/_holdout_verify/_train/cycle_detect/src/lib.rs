//! cycle_detect: follow the first outgoing edge (in insertion order) from a
//! start node, one hop per edge, recording each visited node. If a node is
//! ever revisited, a cycle exists: stop, flag it, and name the repeated node.
//! Otherwise walk until no outgoing edge remains and report no cycle.

/// Result of walking from a start node: the path of visited nodes (the first
/// revisit is appended so the cycle is visible), whether a cycle was found, and
/// the repeated node ("none" has no meaning here; see the runtime field docs).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CycleTrace {
    pub path: Vec<String>,
    pub cycle: bool,
    pub repeated: String,
}

/// A directed edge from one node to another, stored in insertion order.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Edge {
    from: String,
    to: String,
}

/// Runtime that holds the directed edges and walks deterministic paths,
/// detecting the first revisited node along the way.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CycleRuntime {
    edges: Vec<Edge>,
}

impl CycleRuntime {
    pub fn new() -> CycleRuntime {
        CycleRuntime { edges: Vec::new() }
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
    /// edge. If a node is revisited the walk stops with `cycle = true` and
    /// `repeated` set to that node (the repeated node is appended to `path` so
    /// the loop is visible). If no outgoing edge remains the walk stops with
    /// `cycle = false` and `repeated = "none"`.
    pub fn walk(&self, start: &str) -> CycleTrace {
        let mut path: Vec<String> = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        let mut current = start.to_string();

        loop {
            if seen.iter().any(|n| *n == current) {
                // Revisited node: append it to make the loop visible, flag it.
                path.push(current.clone());
                return CycleTrace {
                    path,
                    cycle: true,
                    repeated: current,
                };
            }
            path.push(current.clone());
            seen.push(current.clone());

            match self.first_outgoing(&current) {
                Some(edge) => {
                    current = edge.to.clone();
                }
                None => {
                    return CycleTrace {
                        path,
                        cycle: false,
                        repeated: "none".to_string(),
                    };
                }
            }
        }
    }
}

impl Default for CycleRuntime {
    fn default() -> CycleRuntime {
        CycleRuntime::new()
    }
}
