pub struct DeadletterRuntime {
    edges: Vec<(String, String)>,
    targets: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Delivery {
    pub path: Vec<String>,
    pub delivered: bool,
    pub signals: Vec<String>,
}

impl DeadletterRuntime {
    pub fn new() -> Self {
        Self { edges: Vec::new(), targets: Vec::new() }
    }

    pub fn connect(&mut self, from: &str, to: &str) {
        self.edges.push((from.to_string(), to.to_string()));
    }

    pub fn add_target(&mut self, node: &str) {
        self.targets.push(node.to_string());
    }

    pub fn route(&self, start: &str) -> Delivery {
        let mut path = vec![start.to_string()];
        let mut current = start.to_string();
        loop {
            let next = self
                .edges
                .iter()
                .find(|(from, _)| *from == current)
                .map(|(_, to)| to.clone());
            match next {
                Some(to) => {
                    if path.contains(&to) {
                        break;
                    }
                    path.push(to.clone());
                    current = to;
                }
                None => break,
            }
        }
        let terminal = path.last().cloned().unwrap_or_default();
        let delivered = self.targets.iter().any(|t| *t == terminal);
        let signals = if delivered {
            Vec::new()
        } else {
            vec![format!("deadletter:{}", terminal)]
        };
        Delivery { path, delivered, signals }
    }
}
