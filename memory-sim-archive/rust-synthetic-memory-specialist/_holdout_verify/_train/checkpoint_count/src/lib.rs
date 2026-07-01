#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SinceReport {
    pub count: i32,
    pub events: Vec<String>,
}

struct Event {
    id: i32,
    label: String,
}

pub struct CheckpointCounter {
    events: Vec<Event>,
    next_id: i32,
    last_checkpoint: i32,
}

impl CheckpointCounter {
    pub fn new() -> Self {
        CheckpointCounter { events: Vec::new(), next_id: 1, last_checkpoint: 0 }
    }

    pub fn record(&mut self, label: &str) {
        let id = self.next_id;
        self.next_id += 1;
        self.events.push(Event { id, label: label.to_string() });
    }

    pub fn checkpoint(&mut self) {
        self.last_checkpoint = self.next_id - 1;
    }

    pub fn since(&self) -> SinceReport {
        let mut events: Vec<String> = Vec::new();
        for event in self.events.iter() {
            if event.id > self.last_checkpoint {
                events.push(event.label.clone());
            }
        }
        let count = events.len() as i32;
        SinceReport { count, events }
    }
}
