// A complete, compiling generic Ordo app used as a SHAPE template (not any task's answer).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trace {
    pub path: Vec<String>,
    pub signals: Vec<String>,
}

struct Edge {
    from: String,
    to: String,
}

pub struct Relay {
    edges: Vec<Edge>,
}

impl Relay {
    pub fn new() -> Self {
        Relay { edges: Vec::new() }
    }

    // Mutating setup method returns () so the private Edge type stays private.
    pub fn connect(&mut self, from: &str, to: &str) {
        self.edges.push(Edge { from: from.to_string(), to: to.to_string() });
    }

    // Query method returns the public plain-data Trace. Walk the first outgoing
    // edge in insertion order, owning Strings; record a signal on a dead end.
    pub fn route(&self, start: &str) -> Trace {
        let mut path = vec![start.to_string()];
        let mut signals: Vec<String> = Vec::new();
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
                None => {
                    signals.push(format!("end:{}", current));
                    break;
                }
            }
        }
        Trace { path, signals }
    }
}
