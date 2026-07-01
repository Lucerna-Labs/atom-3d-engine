#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrainTrace {
    pub order: Vec<String>,
    pub signals: Vec<String>,
}

struct Item {
    name: String,
    priority: i64,
}

pub struct PriorityDrain {
    items: Vec<Item>,
}

impl PriorityDrain {
    pub fn new() -> Self {
        PriorityDrain { items: Vec::new() }
    }

    pub fn enqueue(&mut self, name: &str, priority: i64) {
        self.items.push(Item { name: name.to_string(), priority });
    }

    pub fn drain(&self) -> DrainTrace {
        let mut indexed: Vec<(usize, &Item)> = self.items.iter().enumerate().collect();
        // Highest priority first; ties keep insertion order (stable by original index).
        indexed.sort_by(|a, b| {
            b.1.priority
                .cmp(&a.1.priority)
                .then_with(|| a.0.cmp(&b.0))
        });
        let order: Vec<String> = indexed.iter().map(|(_, item)| item.name.clone()).collect();
        let mut signals: Vec<String> = Vec::new();
        match indexed.first() {
            Some((_, item)) => signals.push(format!("top:{}", item.name)),
            None => signals.push("empty".to_string()),
        }
        DrainTrace { order, signals }
    }
}
