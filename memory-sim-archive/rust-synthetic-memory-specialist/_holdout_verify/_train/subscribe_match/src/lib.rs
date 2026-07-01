#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    pub topic: String,
    pub subscribers: Vec<String>,
}

struct Subscription {
    node: String,
    topic: String,
}

pub struct Broker {
    subscriptions: Vec<Subscription>,
}

impl Broker {
    pub fn new() -> Self {
        Broker { subscriptions: Vec::new() }
    }

    pub fn subscribe(&mut self, node: &str, topic: &str) {
        self.subscriptions.push(Subscription {
            node: node.to_string(),
            topic: topic.to_string(),
        });
    }

    pub fn publish(&self, topic: &str) -> Match {
        let mut subscribers: Vec<String> = Vec::new();
        for sub in &self.subscriptions {
            if sub.topic == topic && !subscribers.contains(&sub.node) {
                subscribers.push(sub.node.clone());
            }
        }
        Match { topic: topic.to_string(), subscribers }
    }
}
