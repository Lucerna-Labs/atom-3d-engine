#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThrottlePlan {
    pub node: String,
    pub count: u32,
    pub signals: Vec<String>,
}

struct Counter {
    node: String,
    count: u32,
}

pub struct ThrottleWindow {
    limit: u32,
    counters: Vec<Counter>,
}

impl ThrottleWindow {
    pub fn new() -> Self {
        ThrottleWindow { limit: 0, counters: Vec::new() }
    }

    pub fn set_limit(&mut self, limit: u32) {
        self.limit = limit;
    }

    pub fn event(&mut self, node: &str) -> ThrottlePlan {
        let count = match self.counters.iter_mut().find(|c| c.node == node) {
            Some(counter) => {
                counter.count += 1;
                counter.count
            }
            None => {
                self.counters.push(Counter { node: node.to_string(), count: 1 });
                1
            }
        };
        let signal = if count > self.limit {
            format!("throttle:{}", node)
        } else {
            format!("pass:{}", node)
        };
        ThrottlePlan { node: node.to_string(), count, signals: vec![signal] }
    }
}
