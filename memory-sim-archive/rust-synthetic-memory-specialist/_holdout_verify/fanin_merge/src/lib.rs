//! fanin_merge: register flows (sequences of node names) that get emergent
//! thread ids starting at 1, and query which threads' routes include a node.

/// A merge trace for a single node: the node name plus the thread ids, in
/// thread order, whose route includes that node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MergeTrace {
    pub node: String,
    pub threads: Vec<usize>,
}

/// The fan-in / merge fabric. Each registered flow becomes a thread with an
/// emergent id (1, 2, 3, ...). Threads remember their route so merge queries
/// can report which threads pass through a given node.
#[derive(Clone, Debug, Default)]
pub struct MergeFabric {
    routes: Vec<Vec<String>>,
}

impl MergeFabric {
    /// Create an empty fabric with no threads.
    pub fn new() -> Self {
        MergeFabric { routes: Vec::new() }
    }

    /// Register a flow as a new thread and return its emergent id (1-based,
    /// in registration order).
    pub fn flow(&mut self, nodes: &[&str]) -> usize {
        let route: Vec<String> = nodes.iter().map(|&n| n.to_string()).collect();
        self.routes.push(route);
        self.routes.len()
    }

    /// Report, in thread order, the ids of the threads whose route includes
    /// the given node.
    pub fn merge(&self, node: &str) -> MergeTrace {
        let mut threads: Vec<usize> = Vec::new();
        for (idx, route) in self.routes.iter().enumerate() {
            if route.iter().any(|n| n == node) {
                threads.push(idx + 1);
            }
        }
        MergeTrace {
            node: node.to_string(),
            threads,
        }
    }
}
