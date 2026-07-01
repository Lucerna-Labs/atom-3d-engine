#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fanout {
    pub node: String,
    pub degree: i32,
}

struct Edge {
    from: String,
    to: String,
}

pub struct Fabric {
    edges: Vec<Edge>,
}

impl Fabric {
    pub fn new() -> Self {
        Fabric { edges: Vec::new() }
    }

    pub fn link(&mut self, from: &str, to: &str) {
        self.edges.push(Edge {
            from: from.to_string(),
            to: to.to_string(),
        });
    }

    pub fn fan(&self, node: &str) -> Fanout {
        let mut degree: i32 = 0;
        for edge in self.edges.iter() {
            if edge.from == node {
                degree += 1;
            }
        }
        Fanout {
            node: node.to_string(),
            degree,
        }
    }
}
