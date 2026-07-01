#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sinks {
    pub nodes: Vec<String>,
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

    fn has_outgoing(&self, node: &str) -> bool {
        for edge in self.edges.iter() {
            if edge.from == node {
                return true;
            }
        }
        false
    }

    pub fn sinks(&self) -> Sinks {
        let mut nodes: Vec<String> = Vec::new();
        for edge in self.edges.iter() {
            for node in [&edge.from, &edge.to] {
                if !nodes.contains(node) && !self.has_outgoing(node) {
                    nodes.push(node.clone());
                }
            }
        }
        nodes.sort();
        let count = nodes.len() as i32;
        Sinks { nodes, count }
    }
}
