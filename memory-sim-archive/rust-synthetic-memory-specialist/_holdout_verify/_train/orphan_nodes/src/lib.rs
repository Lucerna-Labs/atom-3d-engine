// Ordo app: report orphan nodes -- nodes that take part in the graph but have
// no incoming edges. A node belongs to the graph if it shows up as either the
// source or the target of at least one link. An orphan is a node that is never
// the target of any link. The report lists those orphan nodes sorted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrphanReport {
    pub nodes: Vec<String>,
}

// Private helper: one directed edge, in insertion order.
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

    // Mutating setter returns () so the private Edge type stays private. Owns
    // its Strings at the boundary.
    pub fn link(&mut self, from: &str, to: &str) {
        self.edges.push(Edge {
            from: from.to_string(),
            to: to.to_string(),
        });
    }

    // Query method returns the public plain-data OrphanReport. Collect every
    // node that appears anywhere (first-seen order), then keep only those that
    // are never a target, and finally sort the result for a deterministic
    // contract.
    pub fn orphans(&self) -> OrphanReport {
        let mut all: Vec<String> = Vec::new();
        for edge in &self.edges {
            if !all.contains(&edge.from) {
                all.push(edge.from.clone());
            }
            if !all.contains(&edge.to) {
                all.push(edge.to.clone());
            }
        }
        let mut nodes: Vec<String> = Vec::new();
        for node in &all {
            if !self.has_incoming(node) {
                nodes.push(node.clone());
            }
        }
        nodes.sort();
        OrphanReport { nodes }
    }

    // Private helper: true when some edge targets this node.
    fn has_incoming(&self, node: &str) -> bool {
        for edge in &self.edges {
            if edge.to == node {
                return true;
            }
        }
        false
    }
}
