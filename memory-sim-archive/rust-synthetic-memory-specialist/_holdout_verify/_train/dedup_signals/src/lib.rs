// Ordo app: collect signals as they arrive; report them deduplicated, keeping
// first-seen insertion order. Plain-data struct, integer count (no f64).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub signals: Vec<String>,
    pub count: usize,
}

// Private helper holding one observed signal in arrival order.
struct Observation {
    name: String,
}

pub struct SignalLog {
    observations: Vec<Observation>,
}

impl SignalLog {
    pub fn new() -> Self {
        SignalLog { observations: Vec::new() }
    }

    // Mutating setup method returns () so the private Observation type stays
    // private. Records one signal, owning the String at the boundary.
    pub fn collect(&mut self, signal: &str) {
        self.observations.push(Observation { name: signal.to_string() });
    }

    // Query method returns the public plain-data Report. Walk observations in
    // arrival order; the first sighting of a signal joins the report, every
    // later sighting is dropped. count is the number of distinct signals.
    pub fn report(&self) -> Report {
        let mut signals: Vec<String> = Vec::new();
        for obs in &self.observations {
            if !signals.contains(&obs.name) {
                signals.push(obs.name.clone());
            }
        }
        let count = signals.len();
        Report { signals, count }
    }
}
