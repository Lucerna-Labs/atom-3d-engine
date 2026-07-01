// Ordo app: rank nodes by out-degree. A node belongs to the graph if it shows
// up as either the source or the target of at least one link. The ranking lists
// every such node ordered by out-degree descending, with ties broken
// alphabetically by node name. Nodes that only ever appear as targets have an
// out-degree of zero and still take part in the ranking.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ranking {
    pub nodes: Vec<String>,
}

// Private helper: one directed edge, kept in insertion order.
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

    // Query method returns the public plain-data Ranking. Collect every node
    // that appears anywhere (first-seen order), then sort by out-degree
    // descending with an alphabetical tie-break, producing a deterministic
    // ranked list of names.
    pub fn rank(&self) -> Ranking {
        let mut all: Vec<String> = Vec::new();
        for edge in &self.edges {
            if !all.contains(&edge.from) {
                all.push(edge.from.clone());
            }
            if !all.contains(&edge.to) {
                all.push(edge.to.clone());
            }
        }
        all.sort_by(|a, b| {
            let da = self.out_degree(a);
            let db = self.out_degree(b);
            db.cmp(&da).then_with(|| a.cmp(b))
        });
        Ranking { nodes: all }
    }

    // Private helper: how many edges leave this node.
    fn out_degree(&self, node: &str) -> u32 {
        let mut count: u32 = 0;
        for edge in &self.edges {
            if edge.from == node {
                count += 1;
            }
        }
        count
    }
}
