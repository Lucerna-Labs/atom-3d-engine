// Ordo app: entries are buffered with an integer ttl. At sweep time T the buffer
// reports the entries that are still alive (ttl strictly greater than T), in
// insertion order. Plain-data struct with String/i32 fields only (no f64).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sweep {
    pub now: i32,
    pub alive: Vec<String>,
    pub count: i32,
}

// Private helper holding one buffered entry in insertion order.
struct Entry {
    label: String,
    ttl: i32,
}

pub struct Buffer {
    entries: Vec<Entry>,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer { entries: Vec::new() }
    }

    // Mutating setup method returns () so the private Entry type stays private.
    pub fn add(&mut self, label: &str, ttl: i32) {
        self.entries.push(Entry { label: label.to_string(), ttl });
    }

    // Query method returns the public plain-data Sweep. An entry survives the
    // sweep at time `now` only when its ttl is strictly greater than `now`.
    // Survivors are collected in insertion order.
    pub fn alive(&self, now: i32) -> Sweep {
        let mut alive: Vec<String> = Vec::new();
        for entry in &self.entries {
            if entry.ttl > now {
                alive.push(entry.label.clone());
            }
        }
        let count = alive.len() as i32;
        Sweep { now, alive, count }
    }
}
