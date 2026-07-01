#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expiry {
    pub expired: Vec<String>,
    pub active: Vec<String>,
}

struct Lease {
    id: String,
    ttl: i64,
}

pub struct Ledger {
    leases: Vec<Lease>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger { leases: Vec::new() }
    }

    pub fn record(&mut self, id: &str, ttl: i64) {
        self.leases.push(Lease {
            id: id.to_string(),
            ttl,
        });
    }

    pub fn expire(&self, now: i64) -> Expiry {
        let mut expired: Vec<String> = Vec::new();
        let mut active: Vec<String> = Vec::new();
        for lease in &self.leases {
            if lease.ttl <= now {
                expired.push(lease.id.clone());
            } else {
                active.push(lease.id.clone());
            }
        }
        Expiry { expired, active }
    }
}
