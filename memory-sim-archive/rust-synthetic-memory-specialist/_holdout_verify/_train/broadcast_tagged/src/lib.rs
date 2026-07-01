// Ordo app: broadcast from a source through a directed mesh, delivering only to
// downstream nodes that carry a given tag. Traversal is BFS in insertion order;
// the source itself is never a delivery target. Plain-data struct, integer count
// (no f64) so the result derives Eq.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Delivery {
    pub source: String,
    pub tag: String,
    pub deliveries: Vec<String>,
    pub count: i32,
}

// Private helper holding one directed edge in insertion order.
struct Edge {
    from: String,
    to: String,
}

// Private helper holding one (node, tag) assignment in insertion order.
struct Tagging {
    node: String,
    tag: String,
}

pub struct Mesh {
    edges: Vec<Edge>,
    taggings: Vec<Tagging>,
}

impl Mesh {
    pub fn new() -> Self {
        Mesh { edges: Vec::new(), taggings: Vec::new() }
    }

    // Mutating setter returns () so the private Edge type stays private.
    pub fn link(&mut self, from: &str, to: &str) {
        self.edges.push(Edge { from: from.to_string(), to: to.to_string() });
    }

    // Mutating setter returns () so the private Tagging type stays private.
    pub fn tag(&mut self, node: &str, tag: &str) {
        self.taggings.push(Tagging { node: node.to_string(), tag: tag.to_string() });
    }

    // Private query: does `node` carry `tag`?
    fn carries(&self, node: &str, tag: &str) -> bool {
        self.taggings.iter().any(|t| t.node == node && t.tag == tag)
    }

    // Query method returns the public plain-data Delivery. Walk every node
    // reachable from `source` via BFS in insertion order (excluding the source),
    // and emit "source>node" for each reached node that carries `tag`. Reached
    // nodes are deduplicated; deliveries follow BFS reachability order.
    pub fn broadcast(&self, source: &str, tag: &str) -> Delivery {
        let mut reached: Vec<String> = Vec::new();
        let mut frontier: Vec<String> = vec![source.to_string()];
        while !frontier.is_empty() {
            let current = frontier.remove(0);
            for edge in self.edges.iter() {
                if edge.from == current && edge.to != source && !reached.contains(&edge.to) {
                    reached.push(edge.to.clone());
                    frontier.push(edge.to.clone());
                }
            }
        }
        let mut deliveries: Vec<String> = Vec::new();
        for node in reached.iter() {
            if self.carries(node, tag) {
                deliveries.push(format!("{}>{}", source, node));
            }
        }
        let count = deliveries.len() as i32;
        Delivery {
            source: source.to_string(),
            tag: tag.to_string(),
            deliveries,
            count,
        }
    }
}
