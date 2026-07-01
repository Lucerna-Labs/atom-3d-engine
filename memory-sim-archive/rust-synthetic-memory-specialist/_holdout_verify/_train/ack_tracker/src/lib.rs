pub struct AckTracker {
    sent: Vec<String>,
    acked: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingReport {
    pub pending: Vec<String>,
    pub count: u32,
}

impl AckTracker {
    pub fn new() -> Self {
        Self { sent: Vec::new(), acked: Vec::new() }
    }

    pub fn send(&mut self, id: &str) {
        self.sent.push(id.to_string());
    }

    pub fn ack(&mut self, id: &str) {
        self.acked.push(id.to_string());
    }

    pub fn pending(&self) -> PendingReport {
        let mut pending: Vec<String> = Vec::new();
        for id in self.sent.iter() {
            let is_acked = self.acked.iter().any(|a| a == id);
            if !is_acked && !pending.iter().any(|p| p == id) {
                pending.push(id.clone());
            }
        }
        let count = pending.len() as u32;
        PendingReport { pending, count }
    }
}
