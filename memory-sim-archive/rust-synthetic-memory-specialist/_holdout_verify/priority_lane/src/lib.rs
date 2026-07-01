pub struct LaneScheduler {
    nodes: Vec<(String, u32)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanePlan {
    pub node: String,
    pub priority: u32,
    pub signals: Vec<String>,
}

impl LaneScheduler {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, name: &str, priority: u32) {
        self.nodes.push((name.to_string(), priority));
    }

    pub fn schedule(&self, candidates: &[&str]) -> Option<LanePlan> {
        let mut best: Option<(String, u32)> = None;
        for &candidate in candidates {
            let known = self
                .nodes
                .iter()
                .find(|(name, _)| name == candidate)
                .map(|(name, priority)| (name.clone(), *priority));
            if let Some((name, priority)) = known {
                let take = match &best {
                    Some((_, best_priority)) => priority > *best_priority,
                    None => true,
                };
                if take {
                    best = Some((name, priority));
                }
            }
        }
        best.map(|(node, priority)| {
            let signals = vec![format!("lane:{}", node)];
            LanePlan { node, priority, signals }
        })
    }
}
