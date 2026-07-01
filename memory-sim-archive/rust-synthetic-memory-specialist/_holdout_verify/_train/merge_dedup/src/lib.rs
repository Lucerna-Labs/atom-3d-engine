// Ordo app: merge several flows into one ordered path, keeping first-seen node
// order and dropping duplicates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Path {
    pub nodes: Vec<String>,
    pub signals: Vec<String>,
}

// Private helper: one captured flow, in insertion order.
struct Flow {
    nodes: Vec<String>,
}

pub struct Merger {
    flows: Vec<Flow>,
}

impl Merger {
    pub fn new() -> Self {
        Merger { flows: Vec::new() }
    }

    // Mutating setup method returns () so the private Flow type stays private.
    // Records one flow as a sequence of nodes, owning Strings at the boundary.
    pub fn flow(&mut self, nodes: &[&str]) {
        let mut owned: Vec<String> = Vec::new();
        for &n in nodes {
            owned.push(n.to_string());
        }
        self.flows.push(Flow { nodes: owned });
    }

    // Query method returns the public plain-data Path. Walk every flow in
    // insertion order, then each node within a flow in order; the first time a
    // node is seen it joins the merged path, every later sighting is dropped and
    // records a dup:<node> signal in encounter order.
    pub fn merge(&self) -> Path {
        let mut nodes: Vec<String> = Vec::new();
        let mut signals: Vec<String> = Vec::new();
        for flow in &self.flows {
            for node in &flow.nodes {
                if nodes.contains(node) {
                    signals.push(format!("dup:{}", node));
                } else {
                    nodes.push(node.clone());
                }
            }
        }
        Path { nodes, signals }
    }
}
