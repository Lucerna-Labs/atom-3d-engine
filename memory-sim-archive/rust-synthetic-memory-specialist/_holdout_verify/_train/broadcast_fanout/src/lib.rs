#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fanout {
    pub deliveries: Vec<String>,
    pub count: i32,
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

    pub fn broadcast(&self, source: &str) -> Fanout {
        let mut reached: Vec<String> = Vec::new();
        let mut frontier: Vec<String> = vec![source.to_string()];
        while !frontier.is_empty() {
            let current = frontier.remove(0);
            for edge in self.edges.iter() {
                if edge.from == current
                    && edge.to != source
                    && !reached.contains(&edge.to)
                {
                    reached.push(edge.to.clone());
                    frontier.push(edge.to.clone());
                }
            }
        }
        let deliveries: Vec<String> = reached
            .iter()
            .map(|target| format!("{}>{}", source, target))
            .collect();
        let count = deliveries.len() as i32;
        Fanout { deliveries, count }
    }
}
