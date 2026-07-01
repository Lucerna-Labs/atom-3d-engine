#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AckReport {
    pub unacked: Vec<String>,
    pub count: u32,
}

struct Send {
    id: String,
    #[allow(dead_code)]
    time: i64,
}

struct Ack {
    id: String,
    time: i64,
}

pub struct AckWindow {
    sends: Vec<Send>,
    acks: Vec<Ack>,
}

impl AckWindow {
    pub fn new() -> Self {
        AckWindow { sends: Vec::new(), acks: Vec::new() }
    }

    pub fn send(&mut self, id: &str, time: i64) {
        self.sends.push(Send { id: id.to_string(), time });
    }

    pub fn ack(&mut self, id: &str, time: i64) {
        self.acks.push(Ack { id: id.to_string(), time });
    }

    pub fn report(&self, cutoff: i64) -> AckReport {
        let mut unacked: Vec<String> = Vec::new();
        for s in &self.sends {
            let acked_by_cutoff = self
                .acks
                .iter()
                .any(|a| a.id == s.id && a.time <= cutoff);
            if !acked_by_cutoff && !unacked.iter().any(|u| u == &s.id) {
                unacked.push(s.id.clone());
            }
        }
        let count = unacked.len() as u32;
        AckReport { unacked, count }
    }
}
