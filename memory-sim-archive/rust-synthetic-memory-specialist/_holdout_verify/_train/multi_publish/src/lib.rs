#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Publication {
    pub thread: i32,
    pub payload: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublishLog {
    pub entries: Vec<Publication>,
    pub count: i32,
}

pub struct Broker {
    entries: Vec<Publication>,
    next_thread: i32,
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            entries: Vec::new(),
            next_thread: 1,
        }
    }

    pub fn publish(&mut self, payload: &str) {
        let thread = self.next_thread;
        self.next_thread += 1;
        self.entries.push(Publication {
            thread,
            payload: payload.to_string(),
        });
    }

    pub fn last(&self) -> Publication {
        self.entries[self.entries.len() - 1].clone()
    }

    pub fn log(&self) -> PublishLog {
        let entries = self.entries.clone();
        let count = entries.len() as i32;
        PublishLog { entries, count }
    }
}
