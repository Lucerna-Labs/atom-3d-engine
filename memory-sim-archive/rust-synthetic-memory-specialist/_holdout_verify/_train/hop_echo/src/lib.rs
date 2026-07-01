#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Echo {
    pub terminal: String,
    pub path: Vec<String>,
}

struct Edge {
    from: String,
    to: String,
}

pub struct Walk {
    edges: Vec<Edge>,
}

impl Walk {
    pub fn new() -> Self {
        Walk { edges: Vec::new() }
    }

    pub fn link(&mut self, from: &str, to: &str) {
        self.edges.push(Edge {
            from: from.to_string(),
            to: to.to_string(),
        });
    }

    pub fn hop(&self, start: &str) -> Echo {
        let mut path = vec![start.to_string()];
        let mut current = start.to_string();
        loop {
            match self.edges.iter().find(|e| e.from == current) {
                Some(edge) => {
                    if path.contains(&edge.to) {
                        break;
                    }
                    path.push(edge.to.clone());
                    current = edge.to.clone();
                }
                None => break,
            }
        }
        Echo {
            terminal: current,
            path,
        }
    }
}
