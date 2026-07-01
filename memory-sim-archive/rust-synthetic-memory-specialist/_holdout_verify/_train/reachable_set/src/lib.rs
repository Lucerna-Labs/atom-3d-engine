// Ordo app: report every node reachable from a start node over directed edges,
// including the start node itself, reported sorted alphabetically.
// Plain-data public struct with String fields and an integer count (no f64, derives Eq).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReachSet {
    pub nodes: Vec<String>,
    pub count: i32,
}

// Private helper: a single directed edge, kept in insertion order.
struct Edge {
    from: String,
    to: String,
}

pub struct Graph {
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Graph { edges: Vec::new() }
    }

    // Mutating setup method returns () so the private Edge type stays private.
    pub fn link(&mut self, from: &str, to: &str) {
        self.edges.push(Edge {
            from: from.to_string(),
            to: to.to_string(),
        });
    }

    // Query method returns the public plain-data ReachSet. Breadth-first walk over
    // outgoing edges in insertion order, owning Strings, cycle-safe via the visited
    // list. The start node is always included. The result is sorted alphabetically.
    pub fn reachable(&self, start: &str) -> ReachSet {
        let mut visited: Vec<String> = vec![start.to_string()];
        let mut frontier: Vec<String> = vec![start.to_string()];
        while !frontier.is_empty() {
            let current = frontier.remove(0);
            for edge in self.edges.iter() {
                if edge.from == current && !visited.iter().any(|v| v == &edge.to) {
                    visited.push(edge.to.clone());
                    frontier.push(edge.to.clone());
                }
            }
        }
        visited.sort();
        let count = visited.len() as i32;
        ReachSet {
            nodes: visited,
            count,
        }
    }
}
