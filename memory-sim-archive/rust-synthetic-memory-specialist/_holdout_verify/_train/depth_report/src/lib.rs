// Ordo app: report the longest path (in hops) reachable from a start node
// over the edge graph. Plain-data String/integer structs, std-only.

// Public plain-data report struct. `hops` is the number of edges on the
// longest simple path found from the start (0 if start has no outgoing edge).
// `deepest` is that longest path as a node sequence; `signals` records the
// terminal/leaf node where the deepest path ends, or a "cycle" note when the
// search was bounded by an already-visited node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DepthReport {
    pub start: String,
    pub hops: i64,
    pub deepest: Vec<String>,
    pub signals: Vec<String>,
}

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
        self.edges.push(Edge { from: from.to_string(), to: to.to_string() });
    }

    // Query method returns the public DepthReport. Depth-first over outgoing
    // edges in insertion order, tracking the path to avoid revisiting a node
    // on the current path (cycle protection). The longest simple path wins;
    // ties keep the first one found (insertion-order DFS). A "cycle:<node>"
    // signal is recorded if traversal was ever stopped by a back-edge, and a
    // "leaf:<node>" signal names the terminal node of the deepest path.
    pub fn depth(&self, start: &str) -> DepthReport {
        let mut best_path: Vec<String> = vec![start.to_string()];
        let mut signals: Vec<String> = Vec::new();
        let mut visited: Vec<String> = vec![start.to_string()];
        self.walk(start, &mut visited, &mut best_path, &mut signals);

        let leaf = best_path.last().cloned().unwrap_or_else(|| start.to_string());
        signals.push(format!("leaf:{}", leaf));

        DepthReport {
            start: start.to_string(),
            hops: (best_path.len() as i64) - 1,
            deepest: best_path,
            signals,
        }
    }

    // Recursive DFS. `path` is the current path (also serves as visited set for
    // cycle protection). Whenever the current path is longer than `best`, copy
    // it into `best`. Records a "cycle:<to>" signal once per back-edge hit.
    fn walk(
        &self,
        current: &str,
        path: &mut Vec<String>,
        best: &mut Vec<String>,
        signals: &mut Vec<String>,
    ) {
        for edge in self.edges.iter().filter(|e| e.from == current) {
            if path.contains(&edge.to) {
                let note = format!("cycle:{}", edge.to);
                if !signals.contains(&note) {
                    signals.push(note);
                }
                continue;
            }
            path.push(edge.to.clone());
            if path.len() > best.len() {
                *best = path.clone();
            }
            self.walk(&edge.to.clone(), path, best, signals);
            path.pop();
        }
    }
}
