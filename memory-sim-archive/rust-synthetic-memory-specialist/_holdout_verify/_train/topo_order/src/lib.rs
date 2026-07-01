//! topo_order: a runtime that holds the directed edges of a graph and produces a
//! deterministic topological order using Kahn's algorithm. Nodes are tracked in
//! first-seen order (the first time a name appears as either endpoint of an
//! edge). Each round, among the nodes whose every incoming edge has already been
//! emitted, the one that appeared earliest is emitted next. If some nodes can
//! never be emitted a cycle exists: the partial order is reported and the nodes
//! still trapped in the cycle (in first-seen order) are listed as remaining.

/// Result of ordering the graph: the topological `order` produced so far, whether
/// a `cycle` blocked completion, and the `remaining` nodes that were never
/// emitted because they sit on or behind a cycle (in first-seen order). When
/// there is no cycle, `remaining` is empty and `order` lists every node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopoOrder {
    pub order: Vec<String>,
    pub cycle: bool,
    pub remaining: Vec<String>,
}

/// A directed edge from one node to another, stored in insertion order.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Edge {
    from: String,
    to: String,
}

/// Runtime that accumulates directed edges and computes a deterministic
/// topological order over the nodes it has seen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopoRuntime {
    edges: Vec<Edge>,
    nodes: Vec<String>,
}

impl TopoRuntime {
    pub fn new() -> TopoRuntime {
        TopoRuntime {
            edges: Vec::new(),
            nodes: Vec::new(),
        }
    }

    /// Record a directed edge from `from` to `to`, preserving insertion order and
    /// registering each endpoint in first-seen node order.
    pub fn edge(&mut self, from: &str, to: &str) {
        self.see(from);
        self.see(to);
        self.edges.push(Edge {
            from: from.to_string(),
            to: to.to_string(),
        });
    }

    /// Register a node in first-seen order if it is new.
    fn see(&mut self, node: &str) {
        if !self.nodes.iter().any(|n| n == node) {
            self.nodes.push(node.to_string());
        }
    }

    /// Count of edges pointing into `node` that come from a node not yet emitted.
    fn pending_indegree(&self, node: &str, emitted: &[String]) -> usize {
        self.edges
            .iter()
            .filter(|e| e.to == node)
            .filter(|e| !emitted.iter().any(|n| *n == e.from))
            .count()
    }

    /// Produce a deterministic topological order using Kahn's algorithm. Each
    /// round, the earliest first-seen node that is not yet emitted and whose
    /// pending indegree is zero is appended to the order. If a round finds no
    /// such node while some remain, those nodes are caught in a cycle: ordering
    /// stops with `cycle = true` and `remaining` set to the un-emitted nodes in
    /// first-seen order. Otherwise `cycle = false` and `remaining` is empty.
    pub fn order(&self) -> TopoOrder {
        let mut emitted: Vec<String> = Vec::new();

        loop {
            let next = self.nodes.iter().find(|n| {
                !emitted.iter().any(|e| e == *n) && self.pending_indegree(n, &emitted) == 0
            });

            match next {
                Some(node) => emitted.push(node.clone()),
                None => break,
            }
        }

        if emitted.len() == self.nodes.len() {
            TopoOrder {
                order: emitted,
                cycle: false,
                remaining: Vec::new(),
            }
        } else {
            let remaining: Vec<String> = self
                .nodes
                .iter()
                .filter(|n| !emitted.iter().any(|e| e == *n))
                .cloned()
                .collect();
            TopoOrder {
                order: emitted,
                cycle: true,
                remaining,
            }
        }
    }
}

impl Default for TopoRuntime {
    fn default() -> TopoRuntime {
        TopoRuntime::new()
    }
}
