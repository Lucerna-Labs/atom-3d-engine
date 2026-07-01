// Ordo app: nodes carry tags; query returns the node names holding a given tag,
// sorted alphabetically and deduplicated. Plain-data struct, integer count (no f64).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TagMatch {
    pub tag: String,
    pub nodes: Vec<String>,
    pub count: usize,
}

// Private helper holding one (node, tag) assignment in insertion order.
struct Assignment {
    node: String,
    tag: String,
}

pub struct Catalog {
    assignments: Vec<Assignment>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { assignments: Vec::new() }
    }

    // Mutating setup method returns () so the private Assignment type stays private.
    pub fn register(&mut self, node: &str, tag: &str) {
        self.assignments.push(Assignment { node: node.to_string(), tag: tag.to_string() });
    }

    // Query method returns the public plain-data TagMatch. Collect every node that
    // carries `tag`, dedup, then sort alphabetically for deterministic output.
    pub fn select(&self, tag: &str) -> TagMatch {
        let mut nodes: Vec<String> = Vec::new();
        for a in &self.assignments {
            if a.tag == tag && !nodes.contains(&a.node) {
                nodes.push(a.node.clone());
            }
        }
        nodes.sort();
        let count = nodes.len();
        TagMatch { tag: tag.to_string(), nodes, count }
    }
}
