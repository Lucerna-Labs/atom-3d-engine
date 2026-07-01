#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Degree {
    pub node: String,
    pub out_degree: u32,
    pub in_degree: u32,
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

    pub fn degree(&self, node: &str) -> Degree {
        let mut out_degree: u32 = 0;
        let mut in_degree: u32 = 0;
        for edge in self.edges.iter() {
            if edge.from == node {
                out_degree += 1;
            }
            if edge.to == node {
                in_degree += 1;
            }
        }
        Degree {
            node: node.to_string(),
            out_degree,
            in_degree,
        }
    }
}
