#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteEntry {
    pub label: String,
    pub stops: Vec<String>,
}

struct Binding {
    label: String,
    stops: Vec<String>,
}

pub struct RouteBook {
    bindings: Vec<Binding>,
}

impl RouteBook {
    pub fn new() -> Self {
        RouteBook { bindings: Vec::new() }
    }

    pub fn assign(&mut self, label: &str, stops: &[&str]) {
        let owned: Vec<String> = stops.iter().map(|s| s.to_string()).collect();
        for binding in self.bindings.iter_mut() {
            if binding.label == label {
                binding.stops = owned;
                return;
            }
        }
        self.bindings.push(Binding {
            label: label.to_string(),
            stops: owned,
        });
    }

    pub fn lookup(&self, label: &str) -> RouteEntry {
        match self.bindings.iter().find(|b| b.label == label) {
            Some(binding) => RouteEntry {
                label: binding.label.clone(),
                stops: binding.stops.clone(),
            },
            None => RouteEntry {
                label: label.to_string(),
                stops: Vec::new(),
            },
        }
    }
}
