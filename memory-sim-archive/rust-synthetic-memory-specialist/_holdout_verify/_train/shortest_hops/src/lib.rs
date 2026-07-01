//! shortest_hops: report the minimum number of hops along a directed graph from a
//! start node to a target node, found by breadth-first search. The query returns
//! the shortest path (ties broken by insertion order of edges) and its hop count.
//! When the target is unreachable, the path is empty and hops is -1.

/// Result of a shortest-path query: the minimum hop count and the path taken.
/// Plain-data struct with String/i32 fields only (no f64) so it derives Eq.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HopResult {
    pub reachable: String,
    pub path: Vec<String>,
    pub hops: i32,
}

/// A directed edge from one node to another, stored in insertion order.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Edge {
    from: String,
    to: String,
}

/// Runtime holding the directed edges; answers shortest-hop queries via BFS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HopGraph {
    edges: Vec<Edge>,
}

impl HopGraph {
    pub fn new() -> HopGraph {
        HopGraph { edges: Vec::new() }
    }

    /// Record a directed edge from `from` to `to`, preserving insertion order.
    /// Mutating setter returns () so the private Edge type stays private.
    pub fn link(&mut self, from: &str, to: &str) {
        self.edges.push(Edge {
            from: from.to_string(),
            to: to.to_string(),
        });
    }

    /// Has this node been seen already in the BFS frontier?
    fn seen(visited: &[String], node: &str) -> bool {
        visited.iter().any(|v| v == node)
    }

    /// Breadth-first search for the minimum number of hops from `start` to
    /// `target`. Neighbors are explored in edge insertion order, so among
    /// shortest paths the one whose branching edges were declared earliest wins.
    /// Returns the public plain-data HopResult. A node always reaches itself in
    /// zero hops. When unreachable, path is empty and hops is -1.
    pub fn shortest(&self, start: &str, target: &str) -> HopResult {
        if start == target {
            return HopResult {
                reachable: "yes".to_string(),
                path: vec![start.to_string()],
                hops: 0,
            };
        }

        // BFS over node states, each carrying the full path used to reach it.
        let mut visited: Vec<String> = vec![start.to_string()];
        let mut frontier: Vec<Vec<String>> = vec![vec![start.to_string()]];

        while !frontier.is_empty() {
            let mut next_frontier: Vec<Vec<String>> = Vec::new();
            for route in frontier.iter() {
                let current = route[route.len() - 1].clone();
                for edge in self.edges.iter() {
                    if edge.from == current && !HopGraph::seen(&visited, &edge.to) {
                        let mut extended = route.clone();
                        extended.push(edge.to.clone());
                        if edge.to == target {
                            let hops = (extended.len() as i32) - 1;
                            return HopResult {
                                reachable: "yes".to_string(),
                                path: extended,
                                hops,
                            };
                        }
                        visited.push(edge.to.clone());
                        next_frontier.push(extended);
                    }
                }
            }
            frontier = next_frontier;
        }

        HopResult {
            reachable: "no".to_string(),
            path: Vec::new(),
            hops: -1,
        }
    }
}

impl Default for HopGraph {
    fn default() -> HopGraph {
        HopGraph::new()
    }
}
