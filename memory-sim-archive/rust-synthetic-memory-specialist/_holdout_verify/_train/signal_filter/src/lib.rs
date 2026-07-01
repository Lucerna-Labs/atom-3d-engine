// Ordo app: a bus collects signal strings in arrival order; a query filters them
// to those beginning with a given prefix, preserving insertion order. Plain-data
// result struct with an integer count (no f64), derives Clone/Debug/PartialEq/Eq.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Filtered {
    pub prefix: String,
    pub signals: Vec<String>,
    pub count: usize,
}

pub struct SignalBus {
    signals: Vec<String>,
}

impl SignalBus {
    pub fn new() -> Self {
        SignalBus { signals: Vec::new() }
    }

    // Mutating setter returns (); owns the String at the boundary.
    pub fn collect(&mut self, signal: &str) {
        self.signals.push(signal.to_string());
    }

    // Query method returns the public plain-data Filtered. Keep every signal whose
    // text starts with `prefix`, in original insertion order (no sort, no dedup).
    pub fn filter(&self, prefix: &str) -> Filtered {
        let mut signals: Vec<String> = Vec::new();
        for s in &self.signals {
            if s.starts_with(prefix) {
                signals.push(s.clone());
            }
        }
        let count = signals.len();
        Filtered { prefix: prefix.to_string(), signals, count }
    }
}
