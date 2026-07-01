#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Adjacency {
    pub node: String,
    pub targets: Vec<String>,
    pub count: i32,
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

    pub fn link(&mut self, from: &str, to: &str) {
        self.edges.push(Edge {
            from: from.to_string(),
            to: to.to_string(),
        });
    }

    // After reversing every directed edge, the new out-neighbors of `node`
    // are exactly the original in-neighbors of `node` (the sources that
    // pointed at it). Targets are reported in edge insertion order, deduped.
    pub fn reversed_adjacency(&self, node: &str) -> Adjacency {
        let mut targets: Vec<String> = Vec::new();
        for edge in self.edges.iter() {
            if edge.to == node && !targets.contains(&edge.from) {
                targets.push(edge.from.clone());
            }
        }
        let count = targets.len() as i32;
        Adjacency {
            node: node.to_string(),
            targets,
            count,
        }
    }
}
