use rust_ordo_ext::TokenBucket;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut rt = TokenBucket::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "capacity" => {
                let cap: i32 = parts[2].parse().unwrap_or(0);
                rt.set_capacity(parts[1], cap);
            }
            "request" => {
                let grant = rt.request(parts[1]);
                let signals = if grant.signals.is_empty() {
                    "none".to_string()
                } else {
                    grant.signals.join(",")
                };
                println!(
                    "node={} allowed={} remaining={} signals={}",
                    grant.node, grant.allowed, grant.remaining, signals
                );
            }
            _ => {}
        }
    }
}
